use rust_search_binary_tree_700::Solution;

fn main() {
    let root = Solution::tree_test_fixture();
    //println!("root is {:?}", root);

    let found = Solution::search_bst(root, 5);

    println!("found is {:?}", found);
}
