use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn tree_node_wrap(node: TreeNode) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(node)))
    }

    pub fn new_left_right(val: i32, left: i32, right: i32) -> Self {
        TreeNode {
            val,
            left: Self::tree_node_wrap(Self::new(left)),
            right: Self::tree_node_wrap(Self::new(right)),
        }
    }

    pub fn new_left(val: i32, left: i32) -> Self {
        TreeNode {
            val,
            left: Self::tree_node_wrap(Self::new(left)),
            right: None,
        }
    }

    pub fn new_right(val: i32, right: i32) -> Self {
        let right = Self::new(right);
        TreeNode {
            val,
            left: Some(Rc::new(RefCell::new(right))),
            right: None,
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn tree_test_fixture() -> Option<Rc<RefCell<TreeNode>>> {
        let root = TreeNode::tree_node_wrap(TreeNode::new(4));

        match root {
            Some(ref node) => {
                let mut tree = node.borrow_mut();
                tree.left = TreeNode::tree_node_wrap(TreeNode::new_left_right(2, 1, 3));
                tree.right = TreeNode::tree_node_wrap(TreeNode::new(7));
            }
            None => (),
        };

        root
    }

    /*
      - the spec is dictated by Leetcode
        and it will take ownership of
        the root which makes looping
        difficult
      - introduce the "next" to focus on
        visiting one node at a time
      - let search_bst handle the looping
    */

    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut visit = Solution::next(&root, val);
        loop {
            if let Some(ref refcell) = visit {
                if refcell.borrow().val == val {
                    break;
                }
                visit = Solution::next(&visit, val)
            } else {
                break;
            }
        }
        visit
    }

    pub fn next(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut result = None;
        if let Some(ref refcell) = root {
            match refcell.borrow().val.cmp(&val) {
                Ordering::Equal => {
                    result = Some(Rc::clone(refcell));
                }
                /*
                  - visit the right child if the parent is less than the val
                */
                Ordering::Less => {
                    if let Some(right_node_refcell) = &refcell.borrow().right {
                        result = Some(Rc::clone(right_node_refcell));
                    }
                }
                Ordering::Greater => {
                    if let Some(left_node_refcell) = &refcell.borrow().left {
                        result = Some(Rc::clone(left_node_refcell));
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found() {
        let target = TreeNode::tree_node_wrap(TreeNode::new_left_right(2, 1, 3));

        let result = Solution::search_bst(Solution::tree_test_fixture(), 2);

        assert_eq!(result, target);
    }

    #[test]
    fn not_found() {
        let result = Solution::search_bst(Solution::tree_test_fixture(), 5);
        assert_eq!(result, None);
    }

    #[test]
    fn partial_eq() {
        let node = TreeNode::new_left_right(4, 2, 7);
        let node2 = TreeNode::new_left_right(4, 2, 7);
        assert_eq!(node, node2);
    }
}
