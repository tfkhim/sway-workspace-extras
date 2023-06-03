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

pub trait Workspace: Copy {
    type OutputName: Eq;

    fn workspace_number(&self) -> i32;
    fn output_name(&self) -> Self::OutputName;
    fn contains_windows(&self) -> bool;
    fn is_focused(&self) -> bool;
    fn contains_not_focused_container(&self) -> bool;
}

pub struct Workspaces<W: Workspace> {
    workspaces: Vec<W>,
    focused_workspace: W,
}

impl<'a, Node: SwayNode> Workspaces<SwayWorkspace<&'a str, &'a Node>> {
    pub fn new(tree: &'a Node) -> Result<Self, TreeError> {
        let mut workspaces = Self::collect_regular_workspaces(tree)?;
        workspaces.sort_by_key(|w| w.num);

        let focused_workspace = Self::find_focused_workspace(&workspaces)?;

        Ok(Self {
            workspaces,
            focused_workspace,
        })
    }

    fn collect_regular_workspaces(
        tree: &'a Node,
    ) -> Result<Vec<SwayWorkspace<&'a str, &'a Node>>, TreeError> {
        tree.find_all_nodes_by(SwayNode::is_output)
            .into_iter()
            .flat_map(|o| {
                o.find_all_nodes_by(SwayNode::is_workspace)
                    .into_iter()
                    .filter(|w| !w.is_scratchpad_workspace())
                    .map(|w| SwayWorkspace::new(o.get_name(), w))
            })
            .collect::<Result<Vec<_>, _>>()
    }
}

impl<W: Workspace> Workspaces<W> {
    fn find_focused_workspace(workspaces: &[W]) -> Result<W, TreeError> {
        workspaces
            .iter()
            .find(|w| w.is_focused())
            .copied()
            .ok_or(TreeError::NoFocusedWorkspace())
    }

    pub fn focused_workspace(&self) -> W {
        self.focused_workspace
    }

    pub fn successor_of_focused(&self) -> Option<W> {
        self.successors_of_focused().next()
    }

    pub fn successors_of_focused(&self) -> impl Iterator<Item = W> + '_ {
        let focused_num = self.focused_workspace().workspace_number();
        self.workspaces
            .iter()
            .filter(move |w| w.workspace_number() > focused_num)
            .copied()
    }

    pub fn predecessor_of_focused(&self) -> Option<W> {
        self.predecessors_of_focused().next()
    }

    pub fn predecessors_of_focused(&self) -> impl Iterator<Item = W> + '_ {
        let focused_num = self.focused_workspace().workspace_number();
        self.workspaces
            .iter()
            .filter(move |w| w.workspace_number() < focused_num)
            .rev()
            .copied()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SwayWorkspace<OutName, Node> {
    output: OutName,
    node: Node,
    num: i32,
}

impl<'a, Node: SwayNode> SwayWorkspace<&'a str, &'a Node> {
    fn new(output: &'a Option<String>, node: &'a Node) -> Result<Self, TreeError> {
        let output = output
            .as_ref()
            .map(String::as_str)
            .ok_or_else(|| TreeError::MissingOutputParent(node.get_id()))?;

        let num = node
            .get_num()
            .ok_or_else(|| TreeError::MissingWorkspaceNumber(node.get_id()))?;

        Ok(Self { output, num, node })
    }
}

impl<'a, Node: SwayNode> Workspace for SwayWorkspace<&'a str, &'a Node> {
    type OutputName = &'a str;

    fn workspace_number(&self) -> i32 {
        self.num
    }

    fn output_name(&self) -> &'a str {
        self.output
    }

    fn contains_windows(&self) -> bool {
        !self.node.get_nodes().is_empty() || !self.node.get_floating_nodes().is_empty()
    }

    fn is_focused(&self) -> bool {
        self.node.find_as_ref(|n| n.is_focused()).is_some()
    }

    fn contains_not_focused_container(&self) -> bool {
        self.node
            .get_nodes()
            .iter()
            .chain(self.node.get_floating_nodes().iter())
            .any(|node| !node.is_focused())
    }
}
