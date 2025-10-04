mod dsu;
mod fenwick_tree;
mod graph;
mod segment_tree;
mod stack;

use segment_tree::segment_tree::function::Function;
use segment_tree::segment_tree::segment_tree::SegmentTree;

fn main() {
    let example_input = vec![4, 2, 8, 1, 3, 9, 12];
    let seg_tree = SegmentTree::construct_and_build(&example_input, Function::Min);
    println!("{:?}", seg_tree.get_segment(1, 3));
}
