use std::ops::Sub;

use anyhow::Result;
use fixedbitset::FixedBitSet;
use petgraph::{
    graph::{DiGraph, NodeIndex},
    visit::Dfs,
};
use ruline_context::Context;
use ruline_field::Field;

use crate::{error::ConditionError, Expression, LogicalOperator};

pub struct Evaluator<'a> {
    graph: &'a DiGraph<Expression, ()>,
    dfs: Dfs<NodeIndex, FixedBitSet>,
    stack: Vec<(NodeIndex, Vec<bool>)>,
}

impl<'a> Evaluator<'a> {
    pub fn new(graph: &'a DiGraph<Expression, ()>, root: NodeIndex) -> Self {
        let dfs = Dfs::new(graph, root);
        let stack = Vec::new();
        Self { graph, dfs, stack }
    }

    pub fn eval(&mut self, ctx: &'a Context) -> Result<bool> {
        while let Some(node) = self.dfs.next(&self.graph) {
            match &self.graph[node] {
                Expression::Comparison {
                    operator, operands, ..
                } => {
                    let operands = operands
                        .iter()
                        .map(Field::from)
                        .map(|field| field.process(ctx))
                        .collect::<Result<Vec<_>>>()?;

                    if let Some(res) = self.handle_result(operator.eval(&operands)?)? {
                        return Ok(res);
                    }
                }
                Expression::Logical { operator, .. } => match self.stack.last_mut() {
                    Some((last, _)) if last != &node => self.prepare_node(node),
                    Some((last, _)) if last == &node => {
                        if let Some(res) = self.evaluate_node(operator)? {
                            return Ok(res);
                        }
                    }
                    None => {
                        self.prepare_node(node);
                    }
                    _ => {}
                },
            }
        }

        Ok(true)
    }

    fn prepare_node(&mut self, node: NodeIndex) {
        self.stack.push((node, Vec::new()));
        let index = self.dfs.stack.len().sub(self.graph.neighbors(node).count());
        self.dfs.stack.insert(index, node);
        self.dfs.discovered.toggle(node.index());
    }

    fn evaluate_node(&mut self, operator: &LogicalOperator) -> Result<Option<bool>> {
        let (_, stack) = self.stack.pop().ok_or(ConditionError::ExpressionInvalid)?;

        let result = match operator {
            LogicalOperator::And => stack.iter().all(|&x| x),
            LogicalOperator::Or => stack.iter().any(|&x| x),
        };

        self.handle_result(result)
    }

    fn handle_result(&mut self, result: bool) -> Result<Option<bool>> {
        match self.stack.last_mut() {
            Some((_, parent_stack)) => {
                parent_stack.push(result);
                Ok(None)
            }
            None => Ok(Some(result)),
        }
    }
}
