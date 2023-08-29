/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use crate::node_traits::NodeWithChildren;

pub trait FindAllNodes {
    fn find_all_nodes_by<F>(&self, predicate: F) -> Vec<&Self>
    where
        F: Copy + Fn(&Self) -> bool;
}

impl<T> FindAllNodes for T
where
    T: NodeWithChildren,
{
    fn find_all_nodes_by<F>(&self, predicate: F) -> Vec<&Self>
    where
        F: Copy + Fn(&Self) -> bool,
    {
        let mut result_nodes = Vec::<&Self>::new();
        find_all_nodes_by_rec(self, predicate, &mut result_nodes);
        result_nodes
    }
}

fn find_all_nodes_by_rec<'a, N, F>(node: &'a N, predicate: F, nodes: &mut Vec<&'a N>)
where
    N: NodeWithChildren,
    F: Copy + Fn(&N) -> bool,
{
    if predicate(node) {
        nodes.push(node)
    }
    node.get_nodes()
        .iter()
        .chain(node.get_floating_nodes().iter())
        .for_each(|node| find_all_nodes_by_rec(node, predicate, nodes));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    #[test]
    fn should_not_collect_any_nodes_when_predicate_never_matches() {
        let root = given_tree();

        let nodes = root.find_all_nodes_by(|_| false);

        assert_eq!(nodes, Vec::<&str>::new());
    }

    #[test]
    fn should_collect_all_nodes_when_predicate_always_matches() {
        let root = given_tree();

        let nodes = root.find_all_nodes_by(|_| true);

        assert_eq!(nodes, vec!["root", "n1", "n2", "f1", "n3", "f2"]);
    }

    #[test]
    fn should_only_collect_nodes_for_which_the_predicate_matches() {
        let root = given_tree();

        let nodes = root.find_all_nodes_by(|n| n == "n1" || n == "f2");

        assert_eq!(nodes, vec!["n1", "f2"]);
    }

    fn given_tree() -> TestNode {
        TestNode {
            id: "root",
            nodes: vec![
                TestNode {
                    id: "n1",
                    nodes: vec![TestNode::new("n2")],
                    floating_nodes: vec![TestNode::new("f1")],
                },
                TestNode {
                    id: "n3",
                    nodes: vec![],
                    floating_nodes: vec![TestNode::new("f2")],
                },
            ],
            floating_nodes: vec![],
        }
    }

    #[derive(Debug)]
    struct TestNode {
        id: &'static str,
        nodes: Vec<TestNode>,
        floating_nodes: Vec<TestNode>,
    }

    impl TestNode {
        fn new(id: &'static str) -> Self {
            Self {
                id,
                nodes: vec![],
                floating_nodes: vec![],
            }
        }
    }

    impl NodeWithChildren for TestNode {
        fn get_nodes(&self) -> &Vec<TestNode> {
            &self.nodes
        }

        fn get_floating_nodes(&self) -> &Vec<TestNode> {
            &self.floating_nodes
        }
    }

    impl PartialEq<str> for TestNode {
        fn eq(&self, other: &str) -> bool {
            self.id == other
        }
    }
}
