/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

mod node_ext;
mod workspace;

use clap::{Parser, Subcommand};
use core::cmp::{max, min};
use swayipc::Connection;
use swayipc::Error;
use swayipc::Fallible;
use swayipc::Node;
use workspace::Workspaces;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Next,
    Prev,
    MoveNext,
    MovePrev,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let mut connection = Connection::new()?;

    match cli.command {
        Commands::Next => execute_next(&mut connection),
        Commands::Prev => execute_previous(&mut connection),
        Commands::MoveNext => execute_move_next(&mut connection),
        Commands::MovePrev => execute_move_prev(&mut connection),
    }
}

fn execute_next(connection: &mut Connection) -> Result<(), Error> {
    let tree = connection.get_tree()?;
    let next_workspace = find_next_workspace(&tree);
    move_focus_to_workspace(connection, next_workspace)
}

fn execute_move_next(connection: &mut Connection) -> Result<(), Error> {
    let tree = connection.get_tree()?;
    let next_workspace = find_next_workspace(&tree);
    move_container_to_workspace(connection, next_workspace)
}

fn find_next_workspace(tree: &Node) -> i32 {
    let workspaces = Workspaces::new(tree);
    let last_workspace = workspaces.last_non_empty_workspace();
    let next_workspace = workspaces.successor_of_focused();
    min(next_workspace, last_workspace.unwrap_or(0) + 1)
}

fn execute_previous(connection: &mut Connection) -> Result<(), Error> {
    let tree = connection.get_tree()?;
    let prev_workspace = find_previous_workspace(&tree);
    move_focus_to_workspace(connection, prev_workspace)
}

fn execute_move_prev(connection: &mut Connection) -> Result<(), Error> {
    let tree = connection.get_tree()?;
    let prev_workspace = find_previous_workspace(&tree);
    move_container_to_workspace(connection, prev_workspace)
}

fn find_previous_workspace(tree: &Node) -> i32 {
    let workspaces = Workspaces::new(tree);
    max(workspaces.predecessor_of_focused(), 1)
}

fn move_focus_to_workspace(connection: &mut Connection, workspace_num: i32) -> Result<(), Error> {
    let command = format!("workspace {}", workspace_num);
    connection.run_command(command).and_then(collect_errors)
}

fn move_container_to_workspace(
    connection: &mut Connection,
    workspace_num: i32,
) -> Result<(), Error> {
    let command = format!("move container to workspace {}", workspace_num);
    connection.run_command(command).and_then(collect_errors)
}

fn collect_errors(results: Vec<Fallible<()>>) -> Result<(), Error> {
    let mut errors = results.into_iter().filter_map(Result::err);
    errors.next().map(Err).unwrap_or(Ok(()))
}
