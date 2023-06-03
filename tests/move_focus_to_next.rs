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
use sway_workspace_extras::{get_workspaces_of, Action, Workflow};

#[test]
fn single_empty_workspace() {
    let tree = single_output(|output| {
        output.workspace(1).focused();
    });

    let actions = when_move_focus_to_next(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn trailing_empty_workspace() {
    let tree = single_output(|output| {
        output.workspace(1).add_window();
        output.workspace(2).focused();
    });

    let actions = when_move_focus_to_next(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn on_last_non_empty_workspace() {
    let tree = single_output(|output| {
        output.workspace(1).add_window();
        output.workspace(2).add_focused_window();
    });

    let actions = when_move_focus_to_next(tree);

    assert_eq!(actions, &[Action::MoveFocus { workspace_num: 3 }]);
}

#[test]
fn on_intermediate_workspace() {
    let tree = single_output(|output| {
        output.workspace(1).add_focused_window();
        output.workspace(2).add_window();
    });

    let actions = when_move_focus_to_next(tree);

    assert_eq!(actions, &[Action::MoveFocus { workspace_num: 2 }]);
}

#[test]
fn creates_empty_intermediate_workspaces() {
    let tree = single_output(|output| {
        output.workspace(1).add_focused_window();
        output.workspace(3).add_window();
    });

    let actions = when_move_focus_to_next(tree);

    assert_eq!(actions, &[Action::MoveFocus { workspace_num: 2 }]);
}

#[test]
fn on_empty_intermediate_workspace() {
    let tree = single_output(|output| {
        output.workspace(1).add_window();
        output.workspace(2).focused();
        output.workspace(3).add_window();
    });

    let actions = when_move_focus_to_next(tree);

    assert_eq!(actions, &[Action::MoveFocus { workspace_num: 3 }]);
}

#[test]
fn initial_state_with_empty_workspaces() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).focused();
        },
        |output_2| {
            output_2.workspace(2);
        },
    );

    let actions = when_move_focus_to_next(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn on_first_output_with_non_empty_second_output() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).focused();
        },
        |output_2| {
            output_2.workspace(2).add_window();
        },
    );

    let actions = when_move_focus_to_next(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn on_empty_first_output_with_gap_between_outputs() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).focused();
        },
        |output_2| {
            output_2.workspace(3);
        },
    );

    let actions = when_move_focus_to_next(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn on_non_empty_first_output_with_empty_second_output() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).add_focused_window();
        },
        |output_2| {
            output_2.workspace(2);
        },
    );

    let actions = when_move_focus_to_next(tree);

    assert_eq!(
        actions,
        &[
            Action::RenameWorkspace {
                workspace_num: 2,
                new_workspace_num: 3
            },
            Action::MoveFocus { workspace_num: 2 }
        ]
    );
}

#[test]
fn on_non_empty_first_output_with_non_empty_second_output() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).add_focused_window();
        },
        |output_2| {
            output_2.workspace(2).add_window();
        },
    );

    let actions = when_move_focus_to_next(tree);

    assert_eq!(
        actions,
        &[
            Action::RenameWorkspace {
                workspace_num: 2,
                new_workspace_num: 3
            },
            Action::MoveFocus { workspace_num: 2 }
        ]
    );
}

#[test]
fn creates_empty_intermediate_workspaces_on_same_output() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).add_focused_window();
        },
        |output_2| {
            output_2.workspace(3);
        },
    );

    let actions = when_move_focus_to_next(tree);

    assert_eq!(actions, &[Action::MoveFocus { workspace_num: 2 }]);
}

#[test]
fn on_last_empty_workspace_of_first_output() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).add_window();
            output_1.workspace(2).focused();
        },
        |output_2| {
            output_2.workspace(3).add_window();
        },
    );

    let actions = when_move_focus_to_next(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn workspaces_separated_by_different_output() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).focused();
            output_1.workspace(3).add_window();
        },
        |output_2| {
            output_2.workspace(2);
        },
    );

    let actions = when_move_focus_to_next(tree);

    assert_eq!(actions, &[Action::MoveFocus { workspace_num: 3 }]);
}

#[test]
fn non_empty_workspaces_separated_by_different_output() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).add_focused_window();
            output_1.workspace(3).add_window();
        },
        |output_2| {
            output_2.workspace(2);
        },
    );

    let actions = when_move_focus_to_next(tree);

    assert_eq!(actions, &[Action::MoveFocus { workspace_num: 3 }]);
}

#[test]
fn workspaces_separated_by_different_output_with_gap_after_second_output() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).add_focused_window();
            output_1.workspace(4).add_window();
        },
        |output_2| {
            output_2.workspace(2);
        },
    );

    let actions = when_move_focus_to_next(tree);

    assert_eq!(actions, &[Action::MoveFocus { workspace_num: 3 }]);
}

#[test]
fn on_last_workspace_of_output_with_gap_on_second_output() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).add_window();
            output_1.workspace(2).add_focused_window();
        },
        |output_2| {
            output_2.workspace(3).add_window();
            output_2.workspace(5).add_window();
        },
    );

    let actions = when_move_focus_to_next(tree);

    assert_eq!(
        actions,
        &[
            Action::RenameWorkspace {
                workspace_num: 3,
                new_workspace_num: 4
            },
            Action::MoveFocus { workspace_num: 3 }
        ]
    );
}

fn when_move_focus_to_next(tree: Node) -> Vec<Action> {
    let workflow = get_workspaces_of(&tree).map(Workflow::new).unwrap();
    workflow.move_focus_to_next()
}
