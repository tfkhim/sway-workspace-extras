/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use crate::node_ext::NodeExt;
use swayipc::Node;
use swayipc::NodeType;

pub struct Workspaces<'a> {
    nodes: Vec<&'a Node>,
}

impl<'a> Workspaces<'a> {
    pub fn new(tree: &'a Node) -> Self {
        Workspaces {
            nodes: tree.find_all_nodes_by(|node| node.node_type == NodeType::Workspace),
        }
    }

    pub fn predecessor_of_focused(&self) -> i32 {
        self.find_focused_workspace().unwrap_or(0) - 1
    }

    pub fn successor_of_focused(&self) -> i32 {
        self.find_focused_workspace().unwrap_or(0) + 1
    }

    fn find_focused_workspace(&self) -> Option<i32> {
        self.nodes
            .iter()
            .find(|w| w.has_focused_child())
            .and_then(|w| w.num)
    }

    pub fn last_non_empty_workspace(&self) -> Option<i32> {
        self.nodes
            .iter()
            .filter(|w| w.has_child_nodes())
            .filter_map(|w| w.num)
            .max()
    }
}
