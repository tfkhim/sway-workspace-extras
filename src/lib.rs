/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

mod find_all_nodes;
mod is_scratchpad;
mod node_traits;
mod tree_error;
mod workflow;
mod workspace;

pub use crate::node_traits::{NamedNode, NodeWithChildren, SwayNode};
pub use crate::tree_error::TreeError;
pub use crate::workflow::{Action, Workflow};
pub use crate::workspace::Workspaces;
