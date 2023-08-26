/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use std::{
    cmp::{max, min},
    vec,
};

use crate::{workspace::Workspace, Workspaces};

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    MoveFocus {
        workspace_num: i32,
    },
    MoveContainer {
        workspace_num: i32,
    },
    RenameWorkspace {
        workspace_num: i32,
        new_workspace_num: i32,
    },
}

pub struct Workflow<W: Workspace> {
    workspaces: Workspaces<W>,
}

impl<W: Workspace> Workflow<W> {
    pub fn new(workspaces: Workspaces<W>) -> Self {
        Self { workspaces }
    }

    pub fn move_focus_to_next(&self) -> Vec<Action> {
        let next_workspace =
            self.find_next_workspace(self.workspaces.focused_workspace().contains_windows());

        if let Some((next_workspace_number, needs_shift)) = next_workspace {
            let mut actions = if needs_shift {
                self.shift_successors()
            } else {
                vec![]
            };

            actions.push(Action::MoveFocus {
                workspace_num: next_workspace_number,
            });

            actions
        } else {
            vec![]
        }
    }

    pub fn move_container_to_next(&self) -> Vec<Action> {
        if self.focused_workspace_is_empty() {
            return vec![];
        }

        let next_workspace = self.find_next_workspace(
            self.workspaces
                .focused_workspace()
                .contains_not_focused_container(),
        );

        if let Some((next_workspace_number, needs_shift)) = next_workspace {
            let mut actions = if needs_shift {
                self.shift_successors()
            } else {
                vec![]
            };

            actions.push(Action::MoveContainer {
                workspace_num: next_workspace_number,
            });
            actions.push(Action::MoveFocus {
                workspace_num: next_workspace_number,
            });

            actions
        } else {
            vec![]
        }
    }

    fn find_next_workspace(&self, extend_output: bool) -> Option<(i32, bool)> {
        let mut next_workspace = None;
        let mut next_none_existing_workspace =
            self.workspaces.max_workspace_number().unwrap_or(0) + 1;
        let mut next_workspace_on_same_output = None;

        for (workspace, expected_workspace_number) in self
            .workspaces
            .successors_of_focused()
            .zip(self.focused_workspace_number() + 1..)
        {
            next_workspace.get_or_insert(workspace);

            if workspace.output_name() == self.current_output() {
                next_workspace_on_same_output.get_or_insert(workspace);
            };

            if expected_workspace_number < workspace.workspace_number() {
                next_none_existing_workspace =
                    min(expected_workspace_number, next_none_existing_workspace);
            };
        }

        match next_workspace_on_same_output {
            None if extend_output => Some((
                self.focused_workspace_number() + 1,
                next_workspace.is_some(),
            )),
            Some(next_on_output) => Some((
                min(
                    next_none_existing_workspace,
                    next_on_output.workspace_number(),
                ),
                false,
            )),
            _ => None,
        }
    }

    pub fn move_focus_to_prev(&self) -> Vec<Action> {
        match self.find_previous_workspace() {
            Some(workspace_num) => vec![Action::MoveFocus { workspace_num }],
            None => vec![],
        }
    }

    pub fn move_container_to_prev(&self) -> Vec<Action> {
        if self.focused_workspace_is_empty() {
            return vec![];
        }

        match self.find_previous_workspace() {
            Some(workspace_num) => vec![
                Action::MoveContainer { workspace_num },
                Action::MoveFocus { workspace_num },
            ],
            None => vec![],
        }
    }

    fn find_previous_workspace(&self) -> Option<i32> {
        let prev_workspace_on_output = self
            .workspaces
            .predecessors_of_focused()
            .find(|w| self.current_output() == w.output_name());

        if let Some(prev_workspace) = prev_workspace_on_output {
            self.handle_more_predecessor_workspaces_on_output(prev_workspace)
        } else {
            self.handle_no_more_predecessor_workspaces_on_output()
        }
    }

    fn handle_more_predecessor_workspaces_on_output(&self, prev_workspace: W) -> Option<i32> {
        let expected_predecessor_number = self.focused_workspace_number() - 1;

        let last_missing_workspace = self
            .workspaces
            .predecessors_of_focused()
            .zip((0..=expected_predecessor_number).rev())
            .find_map(|(w, expected_num)| {
                if expected_num > w.workspace_number() {
                    Some(expected_num)
                } else {
                    None
                }
            });

        let prev_workspace_number = prev_workspace.workspace_number();
        let prev_workspace_number = max(
            last_missing_workspace.unwrap_or(prev_workspace_number),
            prev_workspace_number,
        );

        Some(prev_workspace_number)
    }

    fn handle_no_more_predecessor_workspaces_on_output(&self) -> Option<i32> {
        let expected_predecessor_number = self.focused_workspace_number() - 1;

        let predecessor_on_different_output = self.workspaces.predecessor_of_focused();

        let prev_workspace_number = max(
            expected_predecessor_number,
            predecessor_on_different_output
                .map(|w| w.workspace_number() + 1)
                .unwrap_or(1),
        );

        if prev_workspace_number < self.focused_workspace_number() {
            Some(prev_workspace_number)
        } else {
            None
        }
    }

    pub fn shift_successors(&self) -> Vec<Action> {
        let expected_number_of_successor = self.focused_workspace_number() + 1;

        let mut actions: Vec<_> = self
            .workspaces
            .successors_of_focused()
            .zip(expected_number_of_successor..)
            .take_while(|(workspace, expected_num)| *expected_num == workspace.workspace_number())
            .map(|(workspace, _)| Action::RenameWorkspace {
                workspace_num: workspace.workspace_number(),
                new_workspace_num: workspace.workspace_number() + 1,
            })
            .collect();

        actions.reverse();

        actions
    }

    fn focused_workspace_number(&self) -> i32 {
        self.workspaces.focused_workspace().workspace_number()
    }

    fn focused_workspace_is_empty(&self) -> bool {
        !self.workspaces.focused_workspace().contains_windows()
    }

    fn current_output(&self) -> W::OutputName {
        self.workspaces.focused_workspace().output_name()
    }
}
