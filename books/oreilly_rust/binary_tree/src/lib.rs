
#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}


#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_single_node() {
        let tree = BinaryTree::NonEmpty(
            Box::new(TreeNode {
                element: "Jupiter",
                left: BinaryTree::Empty,
                right: BinaryTree::Empty,
            })
        );

        assert_eq!(
            format!("{tree:?}"),
            "NonEmpty(TreeNode { element: \"Jupiter\", left: Empty, right: Empty })",
        );
    }

    #[test]
    fn test_small_tree() {
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

        println!("{mars:#?}");
        let expected = indoc! {r#"
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
        assert_eq!(format!("{mars:#?}"), expected);
    }
}
