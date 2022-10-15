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
    workspaces: Vec<Workspace<'a>>,
}

impl<'a> Workspaces<'a> {
    pub fn new(tree: &'a Node) -> Self {
        let workspaces = tree
            .find_all_nodes_by(|node| node.node_type == NodeType::Workspace)
            .into_iter()
            .filter_map(Workspace::new)
            .collect();
        Self { workspaces }
    }

    pub fn predecessor_of_focused(&self) -> i32 {
        self.find_focused_workspace().unwrap_or(0) - 1
    }

    pub fn successor_of_focused(&self) -> i32 {
        self.find_focused_workspace().unwrap_or(0) + 1
    }

    fn find_focused_workspace(&self) -> Option<i32> {
        self.workspaces
            .iter()
            .find(|w| w.is_focused())
            .map(|w| w.num)
    }

    pub fn last_non_empty_workspace(&self) -> Option<i32> {
        self.workspaces
            .iter()
            .filter(|w| w.contains_windows())
            .map(|w| w.num)
            .max()
    }
}

struct Workspace<'a> {
    node: &'a Node,
    num: i32,
}

impl<'a> Workspace<'a> {
    fn new(node: &'a Node) -> Option<Self> {
        node.num.map(|num| Self { num, node })
    }

    fn contains_windows(&self) -> bool {
        !self.node.nodes.is_empty() || !self.node.floating_nodes.is_empty()
    }

    fn is_focused(&self) -> bool {
        self.node.find_as_ref(|n| n.focused).is_some()
    }
}
