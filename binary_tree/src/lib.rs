
/// `BinaryTree`s are either empty or contain a boxed `TreeNode`
#[derive(Debug)]
pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}


/// Every `TreeNode` contain data, and possibly child `BinaryTree`s.
#[derive(Debug)]
pub struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}


/// Iterator over `BinaryTree`.
/// Structure hold's the current state of this iteration.
struct TreeIter<'a, T> {
    // Stack of references to tree nodes.
    unvisited: Vec<&'a TreeNode<T>>
}


impl<'a, T: 'a> TreeIter<'a, T> {
    /// Walk the left edge of the tree, pushing every node seen onto the stack.
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let BinaryTree::NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}


impl<T: Ord> BinaryTree<T> {
    /// Create new, empty BinaryTree.
    pub fn empty() -> Self {
        BinaryTree::Empty
    }

    /// Create new BinaryTree with the given root value.
    pub fn new(value: T) -> Self {
        BinaryTree::<T>::NonEmpty(Box::new(TreeNode {
            element: value,
            left: BinaryTree::Empty,
            right: BinaryTree::Empty,
        }))
    }

    /// Add new value to the tree.
    /// Note that nodes, including the root, are not moved during operation.
    pub fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::new(value)
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
    fn test_empty() {
        let tree: BinaryTree<String> = BinaryTree::empty();
        assert_eq!(format!("{tree:?}"), "Empty");
    }

    #[test]
    fn test_new() {
        let tree = BinaryTree::new("Jupiter");
        assert_eq!(format!("{tree:?}"), SINGLE_NODE_EXPECTED);
    }

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
