use std::collections::HashMap;

use anyhow::Result;
use comparison::ComparisonOperator;
pub use error::ConditionError;
use evaluate::Evaluator;
use petgraph::{
    graph::{DiGraph, NodeIndex},
    visit::Dfs,
};
use ruline_context::Context;
use ruline_field::{Field, FieldDefinition};
use serde::{Deserialize, Serialize};
use serde_json::Value;

mod comparison;
mod error;
mod evaluate;
mod test;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ConditionDefinition {
    Binary {
        expression: Expression,
        fallbacks: Vec<String>,
        results: Vec<String>,
    },
    Decision {
        expressions: Vec<Expression>,
        fallbacks: Vec<String>,
        results: HashMap<String, Vec<String>>,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Expression {
    Comparison {
        id: String,
        operator: ComparisonOperator,
        operands: Vec<FieldDefinition>,
    },
    Logical {
        id: String,
        operator: LogicalOperator,
        expressions: Vec<Self>,
    },
}

impl Expression {
    fn get_id(&self) -> String {
        match self {
            Expression::Comparison { id, .. } | Expression::Logical { id, .. } => id.to_owned(),
        }
    }

    fn setup_graph(&self, graph: &mut DiGraph<Expression, ()>, parent: Option<NodeIndex>) {
        match self {
            Expression::Comparison { .. } => {
                let idx = graph.add_node(self.to_owned());
                if let Some(parent_id) = parent {
                    graph.add_edge(parent_id, idx, ());
                }
            }
            Expression::Logical { expressions, .. } => {
                let idx = graph.add_node(self.to_owned());
                if let Some(parent_id) = parent {
                    graph.add_edge(parent_id, idx, ());
                }
                for expressions in expressions {
                    expressions.setup_graph(graph, Some(idx));
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Condition {
    definition: ConditionDefinition,
    graph: DiGraph<Expression, ()>,
    dependencies: Vec<String>,
    dependants: Vec<String>,
}

impl Condition {
    pub fn evaluate(&self, ctx: &Context) -> Result<Vec<String>> {
        match &self.definition {
            ConditionDefinition::Binary {
                fallbacks, results, ..
            } => {
                let root = self.graph.node_indices().next().unwrap();
                let passed = Evaluator::new(&self.graph, root).eval(ctx)?;
                match passed {
                    true => Ok(results.to_vec()),
                    false => Ok(fallbacks.to_vec()),
                }
            }
            ConditionDefinition::Decision {
                expressions,
                fallbacks,
                results,
                ..
            } => {
                let mut indices = self.graph.node_indices().collect::<Vec<_>>();
                let mut indices = indices.iter_mut();
                let mut next_calls = Vec::new();

                for _ in expressions {
                    let root = indices.next().unwrap();
                    if Evaluator::new(&self.graph, *root).eval(ctx)? {
                        next_calls.extend(
                            results
                                .get(&self.graph[*root].get_id().to_string())
                                .ok_or(ConditionError::ExpressionInvalid)?
                                .to_vec(),
                        );
                    }
                }

                if next_calls.is_empty() {
                    next_calls.extend(fallbacks.to_owned());
                }

                Ok(next_calls)
            }
        }
    }

    pub fn validate(&self) -> Result<()> {
        match &self.definition {
            ConditionDefinition::Binary { .. } => {
                let root = self
                    .graph
                    .node_indices()
                    .next()
                    .ok_or(ConditionError::ExpressionInvalid)?;

                self.validate_conditions(root)
            }
            ConditionDefinition::Decision { expressions, .. } => {
                if expressions.is_empty() {
                    return Err(ConditionError::ExpressionInvalid.into());
                }

                for _ in expressions {
                    let root = self
                        .graph
                        .node_indices()
                        .next()
                        .ok_or(ConditionError::ExpressionInvalid)?;

                    self.validate_conditions(root)?;
                }

                Ok(())
            }
        }
    }

    pub fn dependencies(&self) -> Vec<String> {
        self.dependencies.to_owned()
    }

    pub fn dependants(&self) -> Vec<String> {
        self.dependants.to_owned()
    }

    fn validate_conditions(&self, node: NodeIndex) -> Result<()> {
        let mut dfs = Dfs::new(&self.graph, node);

        while let Some(node) = dfs.next(&self.graph) {
            let childrens_count = self.graph.neighbors(node).count();
            match &self.graph[node] {
                Expression::Logical { id, .. } if childrens_count < 2 => {
                    return Err(ConditionError::LogicalChildrenCountInvalid {
                        id: id.to_owned(),
                        childrens_count,
                    }
                    .into());
                }
                _ => {}
            }
        }

        Ok(())
    }
}

impl TryFrom<Value> for Condition {
    type Error = ConditionError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let definition: ConditionDefinition =
            serde_json::from_value(value).map_err(ConditionError::Serde)?;

        Self::try_from(definition)
    }
}

impl TryFrom<ConditionDefinition> for Condition {
    type Error = ConditionError;

    fn try_from(definition: ConditionDefinition) -> Result<Self, Self::Error> {
        let mut graph = DiGraph::new();

        match &definition {
            ConditionDefinition::Binary {
                expression: expressions,
                ..
            } => {
                expressions.setup_graph(&mut graph, None);
            }
            ConditionDefinition::Decision { expressions, .. } => {
                for expressions in expressions {
                    expressions.setup_graph(&mut graph, None);
                }
            }
        }

        let mut dependencies = graph
            .node_indices()
            .filter_map(|node| match &graph[node] {
                Expression::Comparison { operands, .. } => Some(operands),
                _ => None,
            })
            .flatten()
            .flat_map(|operand| Field::from(operand).dependencies())
            .collect::<Vec<_>>();

        dependencies.sort();
        dependencies.dedup();

        let mut dependants = match &definition {
            ConditionDefinition::Binary {
                results, fallbacks, ..
            } => {
                let mut dependants = results.to_vec();
                dependants.extend(fallbacks.to_owned());
                dependants
            }
            ConditionDefinition::Decision {
                results, fallbacks, ..
            } => {
                let mut dependants = results.values().flatten().cloned().collect::<Vec<_>>();
                dependants.extend(fallbacks.to_owned());
                dependants
            }
        };

        dependants.sort();
        dependants.dedup();

        Ok(Self {
            definition,
            dependencies,
            dependants,
            graph,
        })
    }
}
