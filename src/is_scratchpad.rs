/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use crate::node_traits::NamedNode;

pub trait IsScratchpad {
    fn is_scratchpad_workspace(&self) -> bool;
}

impl<T> IsScratchpad for T
where
    T: NamedNode,
{
    fn is_scratchpad_workspace(&self) -> bool {
        self.get_name()
            .as_ref()
            .map(|n| n == "__i3_scratch")
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_without_name_is_not_recognized_as_scratchpad() {
        let node = given_node_without_name();

        let is_scratchpad = node.is_scratchpad_workspace();

        assert!(!is_scratchpad);
    }

    #[test]
    fn node_with_a_random_name_is_not_recognized_as_scratchpad() {
        let node = given_node_with_name("some name");

        let is_scratchpad = node.is_scratchpad_workspace();

        assert!(!is_scratchpad);
    }

    #[test]
    fn node_with_the_right_name_is_recognized_as_scratchpad() {
        let node = given_node_with_name("__i3_scratch");

        let is_scratchpad = node.is_scratchpad_workspace();

        assert!(is_scratchpad);
    }

    fn given_node_without_name() -> TestNode {
        TestNode { name: None }
    }

    fn given_node_with_name(name: &str) -> TestNode {
        TestNode {
            name: Some(name.to_owned()),
        }
    }

    struct TestNode {
        name: Option<String>,
    }

    impl NamedNode for TestNode {
        fn get_name(&self) -> &Option<String> {
            &self.name
        }
    }
}
