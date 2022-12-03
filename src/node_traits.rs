/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use swayipc::{Node, NodeType};

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

pub trait SwayNode: NodeWithChildren + NamedNode {
    fn get_id(&self) -> i64;
    fn get_num(&self) -> Option<i32>;
    fn is_workspace(&self) -> bool;
    fn is_output(&self) -> bool;
    fn is_focused(&self) -> bool;
    fn find_as_ref<F>(&self, predicate: F) -> Option<&Self>
    where
        F: Copy + Fn(&Self) -> bool;
}

impl SwayNode for Node {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_num(&self) -> Option<i32> {
        self.num
    }

    fn is_workspace(&self) -> bool {
        self.node_type == NodeType::Workspace
    }

    fn is_output(&self) -> bool {
        self.node_type == NodeType::Output
    }

    fn is_focused(&self) -> bool {
        self.focused
    }

    fn find_as_ref<F>(&self, predicate: F) -> Option<&Self>
    where
        F: Copy + Fn(&Self) -> bool,
    {
        self.find_as_ref(predicate)
    }
}
