/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use swayipc::Node;

pub trait NodeWithChildren: Sized {
    fn get_nodes(&self) -> &Vec<Self>;
    fn get_floating_nodes(&self) -> &Vec<Self>;
}

impl NodeWithChildren for Node {
    fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    fn get_floating_nodes(&self) -> &Vec<Node> {
        &self.floating_nodes
    }
}

pub trait NamedNode {
    fn get_name(&self) -> &Option<String>;
}

impl NamedNode for Node {
    fn get_name(&self) -> &Option<String> {
        &self.name
    }
}
