/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use clap::{Parser, Subcommand};
use core::cmp::{max, min};
use swayipc::Connection;
use swayipc::Error;
use swayipc::Fallible;
use swayipc::Node;
use swayipc::NodeType;

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

struct Workspaces<'a> {
    nodes: Vec<&'a Node>,
}

impl<'a> Workspaces<'a> {
    fn new(tree: &'a Node) -> Self {
        Workspaces {
            nodes: tree.find_all_nodes_by(|node| node.node_type == NodeType::Workspace),
        }
    }

    fn predecessor_of_focused(&self) -> i32 {
        self.find_focused_workspace().unwrap_or(0) - 1
    }

    fn successor_of_focused(&self) -> i32 {
        self.find_focused_workspace().unwrap_or(0) + 1
    }

    fn find_focused_workspace(&self) -> Option<i32> {
        self.nodes
            .iter()
            .find(|w| w.has_focused_child())
            .and_then(|w| w.num)
    }

    fn last_non_empty_workspace(&self) -> Option<i32> {
        self.nodes
            .iter()
            .filter(|w| w.has_child_nodes())
            .filter_map(|w| w.num)
            .max()
    }
}

trait NodeExt {
    fn find_all_nodes_by<F>(&self, predicate: F) -> Vec<&Node>
    where
        F: Copy + Fn(&Node) -> bool;

    fn has_child_nodes(&self) -> bool;

    fn has_focused_child(&self) -> bool;
}

impl NodeExt for Node {
    fn find_all_nodes_by<F>(&self, predicate: F) -> Vec<&Node>
    where
        F: Copy + Fn(&Node) -> bool,
    {
        fn find_all_nodes_by_rec<'a, F>(node: &'a Node, predicate: F, nodes: &mut Vec<&'a Node>)
        where
            F: Copy + Fn(&Node) -> bool,
        {
            if predicate(node) {
                nodes.push(node)
            }
            node.nodes
                .iter()
                .for_each(|node| find_all_nodes_by_rec(node, predicate, nodes));
        }

        let mut result_nodes = Vec::<&Node>::new();
        find_all_nodes_by_rec(self, predicate, &mut result_nodes);
        result_nodes
    }

    fn has_child_nodes(&self) -> bool {
        !self.nodes.is_empty() || !self.floating_nodes.is_empty()
    }

    fn has_focused_child(&self) -> bool {
        self.find_as_ref(|n| n.focused).is_some()
    }
}
