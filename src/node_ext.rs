/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use swayipc::Node;

pub trait NodeExt {
    fn find_all_nodes_by<F>(&self, predicate: F) -> Vec<&Node>
    where
        F: Copy + Fn(&Node) -> bool;

    fn has_child_nodes(&self) -> bool;

    fn has_focused_child(&self) -> bool;
}

impl NodeExt for Node {
    fn find_all_nodes_by<F>(&self, predicate: F) -> Vec<&Node>
    where
        F: Copy + Fn(&Node) -> bool,
    {
        fn find_all_nodes_by_rec<'a, F>(node: &'a Node, predicate: F, nodes: &mut Vec<&'a Node>)
        where
            F: Copy + Fn(&Node) -> bool,
        {
            if predicate(node) {
                nodes.push(node)
            }
            node.nodes
                .iter()
                .for_each(|node| find_all_nodes_by_rec(node, predicate, nodes));
        }

        let mut result_nodes = Vec::<&Node>::new();
        find_all_nodes_by_rec(self, predicate, &mut result_nodes);
        result_nodes
    }

    fn has_child_nodes(&self) -> bool {
        !self.nodes.is_empty() || !self.floating_nodes.is_empty()
    }

    fn has_focused_child(&self) -> bool {
        self.find_as_ref(|n| n.focused).is_some()
    }
}
