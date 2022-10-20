/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

mod error;
mod node_ext;
mod workspace;

use clap::{Parser, Subcommand};
use core::cmp::{max, min};
use error::CommandErrorConversion;
use error::Error;
use std::process::ExitCode;
use std::process::Termination;
use swayipc::Connection;
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

fn main() -> ExitCode {
    match run_program() {
        Ok(success) => success.report(),
        Err(error) => error.report(),
    }
}

fn run_program() -> Result<(), Error> {
    let cli = Cli::try_parse()?;

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
    let workspaces = Workspaces::new(&tree)?;
    let next_workspace = find_next_workspace(&workspaces);
    move_focus_to_workspace(connection, next_workspace)
}

fn execute_move_next(connection: &mut Connection) -> Result<(), Error> {
    let tree = connection.get_tree()?;
    let workspaces = Workspaces::new(&tree)?;
    let next_workspace = find_next_workspace(&workspaces);
    move_container_to_workspace(connection, next_workspace)
}

fn find_next_workspace(workspaces: &Workspaces) -> i32 {
    let last_workspace = workspaces.last_non_empty_workspace();
    let next_workspace = workspaces.successor_of_focused();
    min(next_workspace, last_workspace.unwrap_or(0) + 1)
}

fn execute_previous(connection: &mut Connection) -> Result<(), Error> {
    let tree = connection.get_tree()?;
    let workspaces = Workspaces::new(&tree)?;
    let prev_workspace = find_previous_workspace(&workspaces);
    move_focus_to_workspace(connection, prev_workspace)
}

fn execute_move_prev(connection: &mut Connection) -> Result<(), Error> {
    let tree = connection.get_tree()?;
    let workspaces = Workspaces::new(&tree)?;
    let prev_workspace = find_previous_workspace(&workspaces);
    move_container_to_workspace(connection, prev_workspace)
}

fn find_previous_workspace(workspaces: &Workspaces) -> i32 {
    max(workspaces.predecessor_of_focused(), 1)
}

fn move_focus_to_workspace(connection: &mut Connection, workspace_num: i32) -> Result<(), Error> {
    let command = format!("workspace {}", workspace_num);
    connection.run_command(command).convert_errors()
}

fn move_container_to_workspace(
    connection: &mut Connection,
    workspace_num: i32,
) -> Result<(), Error> {
    let command = format!("move container to workspace {}", workspace_num);
    connection.run_command(command).convert_errors()
}
