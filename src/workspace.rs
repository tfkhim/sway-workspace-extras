/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use crate::find_all_nodes::FindAllNodes;
use crate::is_scratchpad::IsScratchpad;
use crate::node_traits::SwayNode;
use crate::tree_error::TreeError;

pub struct Workspaces<Node> {
    workspaces: Vec<Workspace<Node>>,
    focused_workspace: Workspace<Node>,
}

impl<'a, Node: SwayNode> Workspaces<&'a Node> {
    pub fn new(tree: &'a Node) -> Result<Self, TreeError> {
        let mut workspaces = Self::collect_regular_workspaces(tree)?;
        workspaces.sort_by_key(|w| w.num);

        let focused_workspace = Self::find_focused_workspace(&workspaces)?;

        Ok(Self {
            workspaces,
            focused_workspace,
        })
    }

    fn collect_regular_workspaces(tree: &'a Node) -> Result<Vec<Workspace<&'a Node>>, TreeError> {
        tree.find_all_nodes_by(SwayNode::is_workspace)
            .into_iter()
            .filter(|w| !w.is_scratchpad_workspace())
            .map(Workspace::new)
            .collect::<Result<Vec<_>, _>>()
    }

    fn find_focused_workspace(
        workspaces: &[Workspace<&'a Node>],
    ) -> Result<Workspace<&'a Node>, TreeError> {
        workspaces
            .iter()
            .find(|w| w.is_focused())
            .copied()
            .ok_or(TreeError::NoFocusedWorkspace())
    }

    pub fn focused_workspace(&self) -> Workspace<&'a Node> {
        self.focused_workspace
    }

    pub fn last_non_empty_workspace(&self) -> Option<Workspace<&'a Node>> {
        self.workspaces
            .iter()
            .filter(|w| w.contains_windows())
            .max_by_key(|w| w.num)
            .cloned()
    }

    pub fn successors_of_focused(&self) -> Vec<Workspace<&'a Node>> {
        let focused_num = self.focused_workspace().workspace_number();
        self.workspaces
            .iter()
            .filter(|w| w.num > focused_num)
            .cloned()
            .collect()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Workspace<Node> {
    node: Node,
    num: i32,
}

impl<'a, Node: SwayNode> Workspace<&'a Node> {
    fn new(node: &'a Node) -> Result<Self, TreeError> {
        node.get_num()
            .ok_or_else(|| TreeError::MissingWorkspaceNumber(node.get_id()))
            .map(|num| Self { num, node })
    }

    pub fn workspace_number(&self) -> i32 {
        self.num
    }

    pub fn contains_windows(&self) -> bool {
        !self.node.get_nodes().is_empty() || !self.node.get_floating_nodes().is_empty()
    }

    pub fn is_focused(&self) -> bool {
        self.node.find_as_ref(|n| n.is_focused()).is_some()
    }

    pub fn contains_not_focused_container(&self) -> bool {
        self.node
            .get_nodes()
            .iter()
            .chain(self.node.get_floating_nodes().iter())
            .any(|node| !node.is_focused())
    }
}
