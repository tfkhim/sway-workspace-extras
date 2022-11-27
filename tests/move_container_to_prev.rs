/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

mod node_builder;

use node_builder::{single_output, Node};
use sway_workspace_extras::{Action, Workflow};

#[test]
fn single_empty_workspace() {
    let tree = single_output(|output| {
        output.workspace(1).focused();
    });

    let actions = when_move_container_to_prev(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn single_workspace_with_one_window() {
    let tree = single_output(|output| {
        output.workspace(1).add_focused_window();
    });

    let actions = when_move_container_to_prev(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn single_workspace_with_two_windows() {
    let tree = single_output(|output| {
        output.workspace(1).add_focused_window().add_window();
    });

    let actions = when_move_container_to_prev(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn first_workspace_without_windows() {
    let tree = single_output(|output| {
        output.workspace(1).focused();
        output.workspace(2).add_window();
    });

    let actions = when_move_container_to_prev(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn first_workspace_with_window() {
    let tree = single_output(|output| {
        output.workspace(1).add_focused_window();
        output.workspace(2).add_window();
    });

    let actions = when_move_container_to_prev(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn trailing_empty_workspace() {
    let tree = single_output(|output| {
        output.workspace(1).add_window();
        output.workspace(2).focused();
    });

    let actions = when_move_container_to_prev(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn on_last_workspace_with_one_window() {
    let tree = single_output(|output| {
        output.workspace(1).add_window();
        output.workspace(2).add_focused_window();
    });

    let actions = when_move_container_to_prev(tree);

    assert_eq!(
        actions,
        &[
            Action::MoveContainer { workspace_num: 1 },
            Action::MoveFocus { workspace_num: 1 }
        ]
    );
}

#[test]
fn creates_intermediate_workspaces() {
    let tree = single_output(|output| {
        output.workspace(1).add_window();
        output.workspace(3).add_focused_window();
    });

    let actions = when_move_container_to_prev(tree);

    assert_eq!(
        actions,
        &[
            Action::MoveContainer { workspace_num: 2 },
            Action::MoveFocus { workspace_num: 2 }
        ]
    );
}

#[test]
fn on_empty_intermediate_workspace() {
    let tree = single_output(|output| {
        output.workspace(1).add_window();
        output.workspace(2).focused();
        output.workspace(3).add_window();
    });

    let actions = when_move_container_to_prev(tree);

    assert_eq!(actions, &[]);
}

fn when_move_container_to_prev(tree: Node) -> Vec<Action> {
    let workflow = Workflow::new(&tree).unwrap();
    workflow.move_container_to_prev()
}
