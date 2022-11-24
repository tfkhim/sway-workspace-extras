/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use sway_workspace_extras::{NamedNode, NodeWithChildren, SwayNode};

pub struct Node {
    id: i64,
    is_workspace: bool,
    num: Option<i32>,
    pub(super) name: Option<String>,
    pub(super) is_focused: bool,
    pub(super) nodes: Vec<Node>,
    floating_nodes: Vec<Node>,
}

impl Node {
    pub fn create_non_workspace_node(id: i64, name: &str) -> Self {
        Node {
            id,
            is_workspace: false,
            num: None,
            name: Some(name.to_owned()),
            is_focused: false,
            nodes: vec![],
            floating_nodes: vec![],
        }
    }

    pub fn create_workspace(id: i64, num: Option<i32>) -> Self {
        Node {
            id,
            is_workspace: true,
            num,
            name: None,
            is_focused: false,
            nodes: vec![],
            floating_nodes: vec![],
        }
    }
}

impl NamedNode for Node {
    fn get_name(&self) -> &Option<String> {
        &self.name
    }
}

impl NodeWithChildren for Node {
    fn get_nodes(&self) -> &Vec<Self> {
        &self.nodes
    }

    fn get_floating_nodes(&self) -> &Vec<Self> {
        &self.floating_nodes
    }
}

impl SwayNode for Node {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_num(&self) -> Option<i32> {
        self.num
    }

    fn is_workspace(&self) -> bool {
        self.is_workspace
    }

    fn is_focused(&self) -> bool {
        self.is_focused
    }

    fn find_as_ref<F>(&self, predicate: F) -> Option<&Self>
    where
        F: Copy + Fn(&Self) -> bool,
    {
        if predicate(self) {
            return Some(self);
        }
        self.nodes
            .iter()
            .find_map(|n| n.find_as_ref(predicate))
            .or_else(|| {
                self.floating_nodes
                    .iter()
                    .find_map(|n| n.find_as_ref(predicate))
            })
    }
}
