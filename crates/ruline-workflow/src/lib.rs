mod error;

use std::collections::HashMap;

use anyhow::Result;
use dashmap::DashMap;
use error::WorkflowError;
use petgraph::{csr::IndexType, graph::DiGraph, visit::Bfs, Direction};
use ruline_action::{Action, ActionDefinition};
use ruline_condition::{Condition, ConditionDefinition};
use ruline_context::Context;
use ruline_output::{Output, OutputDefinition};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ComponentDefinition {
    Condition {
        name: String,
        definition: ConditionDefinition,
    },
    Action {
        name: String,
        definition: ActionDefinition,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowDefinition {
    pub components: HashMap<i64, ComponentDefinition>,
    pub variables: DashMap<String, Value>,
    pub output: OutputDefinition,
}

#[derive(Debug)]
pub enum Component {
    Condition(Condition),
    Action(Action),
}
#[derive(Debug)]
pub struct Workflow {
    components: HashMap<i64, Component>,
    variables: DashMap<String, Value>,
    output: Output,
    graph: DiGraph<i64, ()>,
}

impl Workflow {
    pub fn process(&self, data: Value) -> Result<Value> {
        let context = Context::new(data, self.variables.clone());
        let mut bfs = Bfs::new(&self.graph, self.graph.node_indices().next().unwrap());

        while let Some(node) = bfs.next(&self.graph) {
            if self.graph[node] == 0 {
                continue;
            }

            match self.components.get(&self.graph[node]).unwrap() {
                Component::Condition(condition) => {
                    let result = condition.evaluate(&context)?;
                    let dependants_skipped = condition
                        .dependants()
                        .iter()
                        .filter_map(|d| self.graph.node_indices().find(|n| self.graph[*n] == *d))
                        .filter(|&n| !result.contains(&self.graph[n]))
                        .collect::<Vec<_>>();

                    for skipped in dependants_skipped {
                        bfs.stack.retain(|n| n.index() != skipped.index());
                    }
                }
                Component::Action(a) => {
                    a.process(&context)?;
                }
            }
        }

        self.output.process(&context)
    }

    pub fn validate(&self) -> Result<()> {
        let cycle = petgraph::algo::is_cyclic_directed(&self.graph);
        if cycle {
            return Err(WorkflowError::CycleDetected.into());
        }

        for node in self.graph.node_indices() {
            if let Some(Component::Condition(condition)) = self.components.get(&self.graph[node]) {
                condition.validate()?;
            }
        }

        Ok(())
    }
}

impl TryFrom<Value> for Workflow {
    type Error = WorkflowError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let definition: WorkflowDefinition =
            serde_json::from_value(value).map_err(WorkflowError::Serde)?;
        let mut components = HashMap::new();
        let mut nodes = HashMap::new();
        let mut graph = DiGraph::new();

        let parent_node = graph.add_node(0);

        for (id, component) in definition.components {
            match component {
                ComponentDefinition::Condition { definition, .. } => {
                    let condition =
                        Condition::try_from(definition).map_err(WorkflowError::Condition)?; // (1
                    components.insert(id, Component::Condition(condition));
                    nodes.insert(id, graph.add_node(id));
                }
                ComponentDefinition::Action { definition, .. } => {
                    let action = Action::try_from(definition).map_err(WorkflowError::Action)?;
                    components.insert(id, Component::Action(action));
                    nodes.insert(id, graph.add_node(id));
                }
            }
        }

        for (&component_id, component) in components.iter() {
            match component {
                Component::Condition(condition) => {
                    let node = nodes.get(&component_id).unwrap();
                    for &dependency in &condition.dependencies() {
                        let dependency_node = nodes.get(&dependency).ok_or_else(|| {
                            WorkflowError::DependencyNotFound {
                                component_id,
                                dependency,
                            }
                        })?;
                        graph.add_edge(*dependency_node, *node, ());
                    }

                    for &dependant in &condition.dependants() {
                        let dependant_node = nodes.get(&dependant).ok_or_else(|| {
                            WorkflowError::DependantNotFound {
                                component_id,
                                dependant,
                            }
                        })?;
                        graph.add_edge(*node, *dependant_node, ());
                    }
                }
                Component::Action(action) => {
                    let node = nodes.get(&component_id).unwrap();
                    for &dependency in &action.dependencies() {
                        let dependency_node = nodes.get(&dependency).ok_or_else(|| {
                            WorkflowError::DependencyNotFound {
                                component_id,
                                dependency,
                            }
                        })?;
                        graph.add_edge(*dependency_node, *node, ());
                    }
                }
            }
        }

        for node in graph.node_indices() {
            if graph.edges_directed(node, Direction::Incoming).count() == 0 && node != parent_node {
                graph.add_edge(parent_node, node, ());
            }
        }

        let output = Output::try_from(definition.output).map_err(WorkflowError::Output)?;

        Ok(Self {
            components,
            graph,
            variables: definition.variables,
            output,
        })
    }
}
