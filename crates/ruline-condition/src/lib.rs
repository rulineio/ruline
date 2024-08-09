use std::collections::HashMap;

use anyhow::Result;
use comparison::ComparisonOperator;
use error::ConditionError;
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

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ConditionDefinition {
    Binary {
        id: i64,
        name: String,
        expression: Expression,
        fallbacks: Vec<i64>,
        results: Vec<i64>,
    },
    Decision {
        id: i64,
        name: String,
        expressions: Vec<Expression>,
        fallbacks: Vec<i64>,
        results: HashMap<String, Vec<i64>>,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Expression {
    Comparison {
        id: i64,
        operator: ComparisonOperator,
        operands: Vec<FieldDefinition>,
    },
    Logical {
        id: i64,
        operator: LogicalOperator,
        expressions: Vec<Self>,
    },
}

impl Expression {
    fn get_id(&self) -> i64 {
        match self {
            Expression::Comparison { id, .. } | Expression::Logical { id, .. } => *id,
        }
    }

    fn setup_graph(&self, graph: &mut DiGraph<Expression, ()>, parent: Option<NodeIndex>) {
        match self {
            Expression::Comparison { .. } => {
                let idx = graph.add_node(self.clone());
                if let Some(parent_id) = parent {
                    graph.add_edge(parent_id, idx, ());
                }
            }
            Expression::Logical { expressions, .. } => {
                let idx = graph.add_node(self.clone());
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
    pub definition: ConditionDefinition,
    graph: DiGraph<Expression, ()>,
    dependencies: Vec<i64>,
}

impl Condition {
    pub fn evaluate(&self, ctx: &Context) -> Result<Vec<i64>> {
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
                    next_calls.extend(fallbacks);
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

    pub fn dependencies(&self) -> Vec<i64> {
        self.dependencies.clone()
    }

    fn validate_conditions(&self, node: NodeIndex) -> Result<()> {
        let mut dfs = Dfs::new(&self.graph, node);

        while let Some(node) = dfs.next(&self.graph) {
            let childrens_count = self.graph.neighbors(node).count();
            match &self.graph[node] {
                Expression::Logical { id, .. } if childrens_count < 2 => {
                    return Err(ConditionError::LogicalChildrenCountInvalid {
                        id: *id,
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
            .map(|operand| Field::from(operand).dependency())
            .collect::<Vec<_>>();

        dependencies.sort();
        dependencies.dedup();

        Ok(Self {
            definition,
            dependencies,
            graph,
        })
    }
}
