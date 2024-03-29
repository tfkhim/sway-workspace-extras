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

impl<W: Workspace> Workspaces<W> {
    pub fn new(mut workspaces: Vec<W>) -> Result<Self, TreeError> {
        workspaces.sort_by_key(W::workspace_number);

        let focused_workspace = Self::find_focused_workspace(&workspaces)?;

        Ok(Self {
            workspaces,
            focused_workspace,
        })
    }

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
    output_name: OutName,
    workspace: Node,
    num: i32,
}

pub fn get_workspaces_of<'a, Node: SwayNode>(
    tree: &'a Node,
) -> Result<Workspaces<SwayWorkspace<&'a str, &'a Node>>, TreeError> {
    let output_to_workspaces = |output: &'a Node| {
        output
            .find_all_nodes_by(SwayNode::is_workspace)
            .into_iter()
            .filter(|w| !w.is_scratchpad_workspace())
            .map(|w| SwayWorkspace::new_from_output_and_workspace_nodes(output, w))
    };

    tree.find_all_nodes_by(SwayNode::is_output)
        .into_iter()
        .flat_map(output_to_workspaces)
        .collect::<Result<Vec<_>, _>>()
        .and_then(Workspaces::new)
}

impl<'a, Node: SwayNode> SwayWorkspace<&'a str, &'a Node> {
    fn new_from_output_and_workspace_nodes(
        output: &'a Node,
        workspace: &'a Node,
    ) -> Result<Self, TreeError> {
        let output_name = output
            .get_name()
            .as_ref()
            .map(String::as_str)
            .ok_or_else(|| TreeError::MissingOutputName(output.get_id()))?;

        let num = workspace
            .get_num()
            .ok_or_else(|| TreeError::MissingWorkspaceNumber(workspace.get_id()))?;

        Ok(Self {
            output_name,
            num,
            workspace,
        })
    }
}

impl<'a, Node: SwayNode> Workspace for SwayWorkspace<&'a str, &'a Node> {
    type OutputName = &'a str;

    fn workspace_number(&self) -> i32 {
        self.num
    }

    fn output_name(&self) -> &'a str {
        self.output_name
    }

    fn contains_windows(&self) -> bool {
        !self.workspace.get_nodes().is_empty() || !self.workspace.get_floating_nodes().is_empty()
    }

    fn is_focused(&self) -> bool {
        self.workspace.find_as_ref(|n| n.is_focused()).is_some()
    }

    fn contains_not_focused_container(&self) -> bool {
        self.workspace
            .get_nodes()
            .iter()
            .chain(self.workspace.get_floating_nodes().iter())
            .any(|node| !node.is_focused())
    }
}
