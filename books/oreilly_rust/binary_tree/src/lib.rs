
#[derive(Debug)]
pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}


#[derive(Debug)]
pub struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}


impl<T: Ord> BinaryTree<T> {
    pub fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            },
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SINGLE_NODE_EXPECTED: &str = concat!(
        "NonEmpty(TreeNode { element: \"Jupiter\", ",
        "left: Empty, right: Empty })"
    );

    const SMALL_TREE_EXPECTED: &str = indoc! {r#"
        NonEmpty(
            TreeNode {
                element: "Mars",
                left: NonEmpty(
                    TreeNode {
                        element: "Jupiter",
                        left: Empty,
                        right: Empty,
                    },
                ),
                right: NonEmpty(
                    TreeNode {
                        element: "Mercury",
                        left: Empty,
                        right: Empty,
                    },
                ),
            },
        )"#
    };

    #[test]
    fn test_single_node_manually() {
        let tree = BinaryTree::NonEmpty(
            Box::new(TreeNode {
                element: "Jupiter",
                left: BinaryTree::Empty,
                right: BinaryTree::Empty,
            })
        );

        assert_eq!(format!("{tree:?}"), SINGLE_NODE_EXPECTED);
    }

    #[test]
    fn test_single_node_add() {
        let mut tree = BinaryTree::Empty;
        tree.add("Jupiter");
        assert_eq!(format!("{tree:?}"), SINGLE_NODE_EXPECTED);
    }

    #[test]
    fn test_small_tree_add() {
        let mut tree = BinaryTree::Empty;
        tree.add("Mars");
        tree.add("Jupiter");
        tree.add("Mercury");
        assert_eq!(format!("{tree:#?}"), SMALL_TREE_EXPECTED);
    }

    #[test]
    fn test_small_tree_manually() {
        let jupiter = BinaryTree::NonEmpty(
            Box::new(TreeNode {
                element: "Jupiter",
                left: BinaryTree::Empty,
                right: BinaryTree::Empty,
            })
        );

        let mercury = BinaryTree::NonEmpty(
            Box::new(TreeNode {
                element: "Mercury",
                left: BinaryTree::Empty,
                right: BinaryTree::Empty,
            })
        );

        // Mars' tree takes ownership of jupitor and mercury
        let mars = BinaryTree::NonEmpty(
            Box::new(TreeNode {
                element: "Mars",
                left: jupiter,
                right: mercury,
            })
        );
        assert_eq!(format!("{mars:#?}"), SMALL_TREE_EXPECTED);
    }
}
