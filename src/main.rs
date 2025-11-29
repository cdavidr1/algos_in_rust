mod dsu;
mod fenwick_tree;
mod graph;
mod priority_queue;
mod queue;
mod segment_tree;
mod stack;

use segment_tree::segment_tree::function::Function;
use segment_tree::segment_tree::segment_tree::SegmentTree;

use crate::dsu::dsu::Dsu;
use crate::fenwick_tree::fenwick_tree::FenwickTree;

fn main() {
    let example_input = vec![4, 2, 8, 1, 3, 9, 12];
    let seg_tree = SegmentTree::construct_and_build(&example_input, Function::Min);
    println!("{:?}", seg_tree.get_segment(1, 3));

    // Warnings
    let mut new_dsu: Dsu = Dsu::new(5);
    new_dsu.make_set(5);
    new_dsu.find_set(5);

    let mut new_ft: FenwickTree = FenwickTree::construct_and_build(&[1, 2, 3]);
    new_ft.add(1, 1);
    new_ft.range_sum(1, 2);
    new_ft.range_sum_from_zero(1);
}
