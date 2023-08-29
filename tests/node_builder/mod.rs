/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

mod node;

pub use self::node::Node;

pub fn single_output<F>(setup: F) -> Node
where
    F: FnOnce(&mut OutputBuilder),
{
    build(|root| {
        root.output("out-1", setup);
    })
}

pub fn two_outputs<F1, F2>(setup_output_1: F1, setup_output_2: F2) -> Node
where
    F1: FnOnce(&mut OutputBuilder),
    F2: FnOnce(&mut OutputBuilder),
{
    build(|root| {
        root.output("out-1", setup_output_1);
        root.output("out-2", setup_output_2);
    })
}

pub fn build<F>(setup: F) -> Node
where
    F: FnOnce(&mut TreeBuilder),
{
    let mut id = IdGenerator::new();
    let mut tree = Node::create_named_node(id.next(), "root");
    let mut builder = TreeBuilder {
        id: &mut id,
        tree: &mut tree,
    };
    builder.output("__i3", |out| {
        out.scratch_workspace();
    });
    setup(&mut builder);
    tree
}

struct IdGenerator {
    id: i64,
}

impl IdGenerator {
    fn new() -> Self {
        Self { id: 0 }
    }
    fn next(&mut self) -> i64 {
        self.id += 1;
        self.id
    }
}

pub struct TreeBuilder<'a> {
    id: &'a mut IdGenerator,
    tree: &'a mut Node,
}

impl<'a> TreeBuilder<'a> {
    pub fn output<F>(&mut self, name: &str, setup: F)
    where
        F: FnOnce(&mut OutputBuilder),
    {
        let mut output = Node::create_output(self.id.next(), name);
        let mut builder = OutputBuilder {
            id: self.id,
            output: &mut output,
        };
        setup(&mut builder);
        self.tree.nodes.push(output);
    }
}

pub struct OutputBuilder<'a> {
    id: &'a mut IdGenerator,
    output: &'a mut Node,
}

impl<'a> OutputBuilder<'a> {
    pub fn workspace(&mut self, num: i32) -> WorkspaceBuilder {
        self.add_workspace(Some(num))
    }

    fn scratch_workspace(&mut self) {
        let builder = self.add_workspace(None);
        builder.name("__i3_scratch");
    }

    fn add_workspace(&mut self, num: Option<i32>) -> WorkspaceBuilder {
        self.output
            .nodes
            .push(Node::create_workspace(self.id.next(), num));

        WorkspaceBuilder {
            id: self.id,
            workspace: self.output.nodes.last_mut().unwrap(),
        }
    }
}

pub struct WorkspaceBuilder<'a> {
    id: &'a mut IdGenerator,
    workspace: &'a mut Node,
}

impl<'a> WorkspaceBuilder<'a> {
    pub fn focused(self) -> Self {
        self.workspace.is_focused = true;
        self
    }

    pub fn name(self, name: &str) -> Self {
        self.workspace.name = Some(name.to_owned());
        self
    }

    pub fn add_focused_window(self) -> Self {
        let mut node = Node::create_named_node(self.id.next(), "Window");
        node.is_focused = true;
        self.workspace.nodes.push(node);
        self
    }

    pub fn add_window(self) -> Self {
        let node = Node::create_named_node(self.id.next(), "Window");
        self.workspace.nodes.push(node);
        self
    }
}
