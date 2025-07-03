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

    let actions = when_move_container_to_next(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn single_workspace_with_one_window() {
    let tree = single_output(|output| {
        output.workspace(1).add_focused_window();
    });

    let actions = when_move_container_to_next(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn single_workspace_with_two_windows() {
    let tree = single_output(|output| {
        output.workspace(1).add_focused_window().add_window();
    });

    let actions = when_move_container_to_next(tree);

    assert_eq!(
        actions,
        &[
            Action::MoveContainer { workspace_num: 2 },
            Action::MoveFocus { workspace_num: 2 }
        ]
    );
}

#[test]
fn trailing_empty_workspace() {
    let tree = single_output(|output| {
        output.workspace(1).add_window();
        output.workspace(2).focused();
    });

    let actions = when_move_container_to_next(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn on_last_workspace_with_one_window() {
    let tree = single_output(|output| {
        output.workspace(1).add_window();
        output.workspace(2).add_focused_window();
    });

    let actions = when_move_container_to_next(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn on_last_workspace_with_two_windows() {
    let tree = single_output(|output| {
        output.workspace(1).add_window();
        output.workspace(2).add_focused_window().add_window();
    });

    let actions = when_move_container_to_next(tree);

    assert_eq!(
        actions,
        &[
            Action::MoveContainer { workspace_num: 3 },
            Action::MoveFocus { workspace_num: 3 }
        ]
    );
}

#[test]
fn on_intermediate_workspace() {
    let tree = single_output(|output| {
        output.workspace(1).add_focused_window();
        output.workspace(2).add_window();
    });

    let actions = when_move_container_to_next(tree);

    assert_eq!(
        actions,
        &[
            Action::MoveContainer { workspace_num: 2 },
            Action::MoveFocus { workspace_num: 2 }
        ]
    );
}

#[test]
fn creates_intermediate_workspaces() {
    let tree = single_output(|output| {
        output.workspace(1).add_focused_window();
        output.workspace(3).add_window();
    });

    let actions = when_move_container_to_next(tree);

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

    let actions = when_move_container_to_next(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn initial_state_with_two_outputs() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).focused();
        },
        |output_2| {
            output_2.workspace(2);
        },
    );

    let actions = when_move_container_to_next(tree);

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

    let actions = when_move_container_to_next(tree);

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

    let actions = when_move_container_to_next(tree);

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

    let actions = when_move_container_to_next(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn two_windows_on_first_output_with_empty_second_output() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).add_focused_window().add_window();
        },
        |output_2| {
            output_2.workspace(2);
        },
    );

    let actions = when_move_container_to_next(tree);

    assert_eq!(
        actions,
        &[
            Action::RenameWorkspace {
                workspace_name: "2".to_owned(),
                new_workspace_name: "3".to_owned(),
            },
            Action::MoveContainer { workspace_num: 2 },
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

    let actions = when_move_container_to_next(tree);

    assert_eq!(actions, &[]);
}

#[test]
fn two_windows_on_first_output_with_non_empty_second_output() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).add_focused_window().add_window();
        },
        |output_2| {
            output_2.workspace(2).add_window();
        },
    );

    let actions = when_move_container_to_next(tree);

    assert_eq!(
        actions,
        &[
            Action::RenameWorkspace {
                workspace_name: "2".to_owned(),
                new_workspace_name: "3".to_owned(),
            },
            Action::MoveContainer { workspace_num: 2 },
            Action::MoveFocus { workspace_num: 2 }
        ]
    );
}

#[test]
fn creates_empty_intermediate_workspaces_on_same_output() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).add_focused_window().add_window();
        },
        |output_2| {
            output_2.workspace(3);
        },
    );

    let actions = when_move_container_to_next(tree);

    assert_eq!(
        actions,
        &[
            Action::MoveContainer { workspace_num: 2 },
            Action::MoveFocus { workspace_num: 2 }
        ]
    );
}

#[test]
fn workspaces_separated_by_different_output() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).add_focused_window();
            output_1.workspace(3).add_window();
        },
        |output_2| {
            output_2.workspace(2);
        },
    );

    let actions = when_move_container_to_next(tree);

    assert_eq!(
        actions,
        &[
            Action::MoveContainer { workspace_num: 3 },
            Action::MoveFocus { workspace_num: 3 }
        ]
    );
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

    let actions = when_move_container_to_next(tree);

    assert_eq!(
        actions,
        &[
            Action::MoveContainer { workspace_num: 3 },
            Action::MoveFocus { workspace_num: 3 }
        ]
    );
}

#[test]
fn on_last_workspace_of_first_output_with_second_output_in_between() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).add_window();
            output_1.workspace(3).add_focused_window().add_window();
        },
        |output_2| {
            output_2.workspace(2);
        },
    );

    let actions = when_move_container_to_next(tree);

    assert_eq!(
        actions,
        &[
            Action::MoveContainer { workspace_num: 4 },
            Action::MoveFocus { workspace_num: 4 }
        ]
    );
}

#[test]
fn on_last_workspace_of_output_with_gap_on_second_output() {
    let tree = two_outputs(
        |output_1| {
            output_1.workspace(1).add_window();
            output_1.workspace(2).add_focused_window().add_window();
        },
        |output_2| {
            output_2.workspace(3).add_window();
            output_2.workspace(5).add_window();
        },
    );

    let actions = when_move_container_to_next(tree);

    assert_eq!(
        actions,
        &[
            Action::RenameWorkspace {
                workspace_name: "3".to_owned(),
                new_workspace_name: "4".to_owned(),
            },
            Action::MoveContainer { workspace_num: 3 },
            Action::MoveFocus { workspace_num: 3 }
        ]
    );
}

#[test]
fn supports_named_workspaces() {
    let tree = two_outputs(
        |output_1| {
            output_1
                .named_workspace(1, "ws-1")
                .add_focused_window()
                .add_window();
        },
        |output_2| {
            output_2.named_workspace(2, "ws-2");
        },
    );

    let actions = when_move_container_to_next(tree);

    assert_eq!(
        actions[0],
        Action::RenameWorkspace {
            workspace_name: "2: ws-2".to_owned(),
            new_workspace_name: "3: ws-2".to_owned(),
        },
    );
}

fn when_move_container_to_next(tree: Node) -> Vec<Action> {
    let workflow = get_workspaces_of(&tree).map(Workflow::new).unwrap();
    workflow.move_container_to_next()
}
