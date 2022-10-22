/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use crate::error::Error;
use crate::node_ext::NodeExt;
use swayipc::Node;
use swayipc::NodeType;

pub struct Workspaces<'a> {
    workspaces: Vec<Workspace<'a>>,
    focused_workspace: Workspace<'a>,
}

impl<'a> Workspaces<'a> {
    pub fn new(tree: &'a Node) -> Result<Self, Error> {
        let workspaces = Self::collect_regular_workspaces(tree)?;
        let focused_workspace = Self::find_focused_workspace(&workspaces)?;

        Ok(Self {
            workspaces,
            focused_workspace,
        })
    }

    fn collect_regular_workspaces(tree: &'a Node) -> Result<Vec<Workspace<'a>>, Error> {
        tree.find_all_nodes_by(|node| node.node_type == NodeType::Workspace)
            .into_iter()
            .filter(|w| !w.is_scratchpad_workspace())
            .map(Workspace::new)
            .collect::<Result<Vec<_>, _>>()
    }

    fn find_focused_workspace(workspaces: &[Workspace<'a>]) -> Result<Workspace<'a>, Error> {
        workspaces
            .iter()
            .find(|w| w.is_focused())
            .cloned()
            .ok_or_else(|| {
                Error::Validation("Could not find a workspace which has focus".to_owned())
            })
    }

    pub fn focused_workspace(&self) -> Workspace<'a> {
        self.focused_workspace.clone()
    }

    pub fn last_non_empty_workspace(&self) -> Option<Workspace<'a>> {
        self.workspaces
            .iter()
            .filter(|w| w.contains_windows())
            .max_by_key(|w| w.num)
            .cloned()
    }
}

#[derive(Debug, Clone)]
pub struct Workspace<'a> {
    node: &'a Node,
    num: i32,
}

impl<'a> Workspace<'a> {
    fn new(node: &'a Node) -> Result<Self, Error> {
        node.num
            .ok_or_else(|| {
                let msg = format!("The num property of workspace with id {} is None", node.id);
                Error::Validation(msg)
            })
            .map(|num| Self { num, node })
    }

    pub fn workspace_number(&self) -> i32 {
        self.num
    }

    pub fn contains_windows(&self) -> bool {
        !self.node.nodes.is_empty() || !self.node.floating_nodes.is_empty()
    }

    pub fn is_focused(&self) -> bool {
        self.node.find_as_ref(|n| n.focused).is_some()
    }
}
