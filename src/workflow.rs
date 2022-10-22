/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use std::cmp::{max, min};

use swayipc::Node;

use crate::{error::Error, workspace::Workspaces};

pub enum Action {
    MoveFocus { workspace_num: i32 },
    MoveContainer { workspace_num: i32 },
}

pub struct Workflow<'a> {
    workspaces: Workspaces<'a>,
}

impl<'a> Workflow<'a> {
    pub fn new(tree: &'a Node) -> Result<Self, Error> {
        Workspaces::new(tree).map(|workspaces| Self { workspaces })
    }

    pub fn move_focus_to_next(&self) -> Vec<Action> {
        let next_workspace = self.find_next_workspace();
        vec![Action::MoveFocus {
            workspace_num: next_workspace,
        }]
    }

    pub fn move_container_to_next(&self) -> Vec<Action> {
        let next_workspace = self.find_next_workspace();
        vec![
            Action::MoveContainer {
                workspace_num: next_workspace,
            },
            Action::MoveFocus {
                workspace_num: next_workspace,
            },
        ]
    }

    fn find_next_workspace(&self) -> i32 {
        let last_non_empty_workspace = self.workspaces.last_non_empty_workspace();
        let focused_workspace = self.workspaces.focused_workspace().workspace_number();
        min(
            focused_workspace + 1,
            last_non_empty_workspace
                .map(|w| w.workspace_number() + 1)
                .unwrap_or(1),
        )
    }

    pub fn move_focus_to_prev(&self) -> Vec<Action> {
        let prev_workspace = self.find_previous_workspace();
        vec![Action::MoveFocus {
            workspace_num: prev_workspace,
        }]
    }

    pub fn move_container_to_prev(&self) -> Vec<Action> {
        let prev_workspace = self.find_previous_workspace();
        vec![
            Action::MoveContainer {
                workspace_num: prev_workspace,
            },
            Action::MoveFocus {
                workspace_num: prev_workspace,
            },
        ]
    }

    fn find_previous_workspace(&self) -> i32 {
        let focused_workspace = self.workspaces.focused_workspace().workspace_number();
        max(focused_workspace - 1, 1)
    }
}
