/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum TreeError {
    #[error("The num property of workspace with id {0} is None")]
    MissingWorkspaceNumber(i64),
    #[error("The output with id {0} doesn't have a name")]
    MissingOutputName(i64),
    #[error("Could not find a workspace which has focus")]
    NoFocusedWorkspace(),
}
