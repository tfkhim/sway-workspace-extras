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

use crate::{
    node_traits::SwayNode,
    tree_error::TreeError,
    workspace::{Workspace, Workspaces},
};

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

pub struct Workflow<OutName, Node> {
    workspaces: Workspaces<OutName, Node>,
}

impl<'a, Node: SwayNode> Workflow<&'a str, &'a Node> {
    pub fn new(tree: &'a Node) -> Result<Self, TreeError> {
        Workspaces::new(tree).map(|workspaces| Self { workspaces })
    }

    pub fn move_focus_to_next(&self) -> Vec<Action> {
        let extend_output =
            |last_workspace: &Workspace<&'a str, &'a Node>| last_workspace.contains_windows();

        let next_workspace = self.find_next_workspace(extend_output);

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

        let extend_output = |last_workspace: &Workspace<&'a str, &'a Node>| {
            last_workspace.contains_not_focused_container()
        };

        let next_workspace = self.find_next_workspace(extend_output);

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

    fn find_next_workspace<F>(&self, extend_output: F) -> Option<(i32, bool)>
    where
        F: Fn(&Workspace<&'a str, &'a Node>) -> bool,
    {
        let successor_on_same_output = self
            .workspaces
            .successors_of_focused()
            .find(|w| self.current_output() == w.output_name());

        if let Some(next_on_output) = successor_on_same_output {
            self.handle_more_workspaces_on_output(&next_on_output)
        } else {
            self.handle_no_more_workspaces_on_output(extend_output)
        }
    }

    fn handle_more_workspaces_on_output(
        &self,
        next_on_output: &'a Workspace<&'a str, &'a Node>,
    ) -> Option<(i32, bool)> {
        let expected_successor_number = self.focused_workspace_number() + 1;

        let next_missing_workspace = self
            .workspaces
            .successors_of_focused()
            .zip(expected_successor_number..)
            .find_map(|(w, expected_num)| {
                if expected_num < w.workspace_number() {
                    Some(expected_num)
                } else {
                    None
                }
            });

        let next_existing_num = next_on_output.workspace_number();
        let next_workspace = min(
            next_missing_workspace.unwrap_or(next_existing_num),
            next_existing_num,
        );

        Some((next_workspace, false))
    }

    fn handle_no_more_workspaces_on_output<F>(&self, extend_output: F) -> Option<(i32, bool)>
    where
        F: Fn(&Workspace<&'a str, &'a Node>) -> bool,
    {
        if !extend_output(&self.workspaces.focused_workspace()) {
            return None;
        };

        let expected_successor_number = self.focused_workspace_number() + 1;

        let needs_shift = self
            .workspaces
            .successor_of_focused()
            .map(|w| w.workspace_number() == expected_successor_number)
            .unwrap_or(false);

        Some((expected_successor_number, needs_shift))
    }

    pub fn move_focus_to_prev(&self) -> Vec<Action> {
        let prev_workspace = self.find_previous_workspace();
        if prev_workspace == self.focused_workspace_number() {
            vec![]
        } else {
            vec![Action::MoveFocus {
                workspace_num: prev_workspace,
            }]
        }
    }

    pub fn move_container_to_prev(&self) -> Vec<Action> {
        if self.focused_workspace_is_empty() {
            return vec![];
        }

        let prev_workspace = self.find_previous_workspace();

        if prev_workspace == self.focused_workspace_number() {
            vec![]
        } else {
            vec![
                Action::MoveContainer {
                    workspace_num: prev_workspace,
                },
                Action::MoveFocus {
                    workspace_num: prev_workspace,
                },
            ]
        }
    }

    fn find_previous_workspace(&self) -> i32 {
        let focused_workspace = self.focused_workspace_number();
        max(focused_workspace - 1, 1)
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

    fn current_output(&self) -> &'a str {
        self.workspaces.focused_workspace().output_name()
    }
}
