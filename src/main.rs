/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

mod application_error;

use application_error::CommandErrorConversion;
use application_error::Error;
use clap::{Parser, Subcommand};
use std::process::ExitCode;
use std::process::Termination;
use sway_workspace_extras::{Action, Workflow};
use swayipc::Connection;

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
    Shift,
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
    let tree = connection.get_tree()?;

    let workflow = Workflow::new(&tree)?;

    let actions = match cli.command {
        Commands::Next => workflow.move_focus_to_next(),
        Commands::Prev => workflow.move_focus_to_prev(),
        Commands::MoveNext => workflow.move_container_to_next(),
        Commands::MovePrev => workflow.move_container_to_prev(),
        Commands::Shift => workflow.shift_successors(),
    };

    execute_actions(&mut connection, &actions)
}

fn execute_actions(connection: &mut Connection, actions: &[Action]) -> Result<(), Error> {
    if actions.is_empty() {
        return Ok(());
    }

    let commands: Vec<_> = actions.iter().map(action_to_sway_command).collect();
    let command_string = commands.join(";");
    connection.run_command(command_string).convert_errors()
}

fn action_to_sway_command(action: &Action) -> String {
    match action {
        Action::MoveFocus { workspace_num } => format!("workspace {}", workspace_num),
        Action::MoveContainer { workspace_num } => {
            format!("move container to workspace {}", workspace_num)
        }
        Action::RenameWorkspace {
            workspace_num,
            new_workspace_num,
        } => format!("rename workspace {workspace_num} to  {new_workspace_num}"),
    }
}
