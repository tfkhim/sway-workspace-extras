/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

mod node_builder;

use node_builder::{single_output, two_outputs, Node};
use sway_workspace_extras::{Action, Workflow};

#[test]
fn single_empty_workspace() {
    let tree = single_output(|output| {
        output.workspace(1).focused();
    });

    let actions = when_shift_successors(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn single_workspace_with_one_window() {
    let tree = single_output(|output| {
        output.workspace(1).add_focused_window();
    });

    let actions = when_shift_successors(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn current_workspace_is_empty() {
    let tree = single_output(|output| {
        output.workspace(1).focused();
        output.workspace(2).add_window();
    });

    let actions = when_shift_successors(tree);

    assert_eq!(
        actions,
        &[Action::RenameWorkspace {
            workspace_num: 2,
            new_workspace_num: 3
        }]
    );
}

#[test]
fn current_workspace_not_empty() {
    let tree = single_output(|output| {
        output.workspace(1).add_focused_window();
        output.workspace(2).add_window();
    });

    let actions = when_shift_successors(tree);

    assert_eq!(
        actions,
        &[Action::RenameWorkspace {
            workspace_num: 2,
            new_workspace_num: 3
        }]
    );
}

#[test]
fn next_workspace_is_empty() {
    let tree = single_output(|output| {
        output.workspace(1).add_focused_window();
        output.workspace(3).add_window();
    });

    let actions = when_shift_successors(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn gap_between_successors() {
    let tree = single_output(|output| {
        output.workspace(1).add_focused_window();
        output.workspace(2).add_window();
        output.workspace(4).add_window();
    });

    let actions = when_shift_successors(tree);

    assert_eq!(
        actions,
        &[Action::RenameWorkspace {
            workspace_num: 2,
            new_workspace_num: 3
        }]
    );
}

#[test]
fn multiple_successors() {
    let tree = single_output(|output| {
        output.workspace(1).add_focused_window();
        output.workspace(2).add_window();
        output.workspace(3).add_window();
    });

    let actions = when_shift_successors(tree);

    assert_eq!(
        actions,
        &[
            Action::RenameWorkspace {
                workspace_num: 3,
                new_workspace_num: 4
            },
            Action::RenameWorkspace {
                workspace_num: 2,
                new_workspace_num: 3
            }
        ]
    );
}

#[test]
fn two_empty_outputs() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).focused();
        },
        |output_2| {
            output_2.workspace(2);
        },
    );

    let actions = when_shift_successors(tree);

    assert_eq!(
        actions,
        &[Action::RenameWorkspace {
            workspace_num: 2,
            new_workspace_num: 3
        }]
    );
}

#[test]
fn on_second_output_without_successors() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1);
        },
        |output_2| {
            output_2.workspace(2).focused();
        },
    );

    let actions = when_shift_successors(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn gap_between_successors_on_different_outputs() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).focused();
            output_1.workspace(2).add_window();
        },
        |output_2| {
            output_2.workspace(4);
        },
    );

    let actions = when_shift_successors(tree);

    assert_eq!(
        actions,
        &[Action::RenameWorkspace {
            workspace_num: 2,
            new_workspace_num: 3
        }]
    );
}

#[test]
fn successors_on_different_outputs() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).focused();
            output_1.workspace(2).add_window();
        },
        |output_2| {
            output_2.workspace(3);
        },
    );

    let actions = when_shift_successors(tree);

    assert_eq!(
        actions,
        &[
            Action::RenameWorkspace {
                workspace_num: 3,
                new_workspace_num: 4
            },
            Action::RenameWorkspace {
                workspace_num: 2,
                new_workspace_num: 3
            }
        ]
    );
}

#[test]
fn two_workspace_with_gap_between() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).add_focused_window();
        },
        |output_2| {
            output_2.workspace(3);
        },
    );

    let actions = when_shift_successors(tree);

    assert_eq!(actions, &[]);
}

fn when_shift_successors(tree: Node) -> Vec<Action> {
    let workflow = Workflow::new(&tree).unwrap();
    workflow.shift_successors()
}
