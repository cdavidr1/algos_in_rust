// only support comparable elements
// poll: remove highest priority
// the highest priority is the smallest number
// add: adds element to pqueue
// heap is used
// heap: tree satisfying heap invariant
// heap invariant:
//  if A parent of B,
//  then A is ordered wrt B
//  for all A, B in the tree
// leads to max and min heaps for example
use std::collections::{BTreeSet, HashMap};
use std::hash::Hash;

struct PriorityQueue<T>
where
    T: PartialOrd + Hash + Eq + Clone,
{
    heap_size: usize,
    heap: Vec<T>,
    position_map: HashMap<T, BTreeSet<usize>>,
}

impl<T> PriorityQueue<T>
where
    T: PartialOrd + Hash + Eq + Clone,
{
    fn new(size: usize) -> Self {
        Self {
            heap: Vec::new(),
            heap_size: size,
            position_map: HashMap::new(),
        }
    }

    fn make_p_q(elems: &[T]) -> Self {
        let mut p_q = PriorityQueue::new(elems.len());
        for (index, elem) in elems.iter().enumerate() {
            p_q.heap.push(elem.clone());
            p_q.map_add(elem.clone(), index);
        }
        let mut i = 0.max((p_q.heap_size / 2) as isize - 1);
        while i >= 0 {
            p_q.sink(i as usize);
            i -= 1;
        }
        p_q
    }

    fn map_add(&mut self, elem: T, index: usize) {
        self.position_map
            .entry(elem)
            .or_insert_with(BTreeSet::new)
            .insert(index);
    }

    fn sink(&mut self, mut index: usize) {
        loop {
            let left = 2 * index + 1;
            let right = 2 * index + 2;
            let mut smallest = left;

            if right < self.size() && self.less(right, left) {
                smallest = right;
            }

            if left >= self.size() || self.less(index, smallest) {
                break;
            }

            self.swap(smallest, index);
            index = smallest;
        }
    }

    fn swim(size: usize) {}

    fn less(&self, i: usize, j: usize) -> bool {
        let node_a = self.heap.get(i).unwrap();
        let node_b = self.heap.get(j).unwrap();
        node_a <= node_b
    }

    fn is_empty() {}

    fn clear() {}

    fn size(&self) -> usize {
        self.heap.len()
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
    }

    fn peek() {}

    fn poll() {}

    fn contains(elem: T) {}

    fn add(elem: T) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_works() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_create() {
        let p_q: PriorityQueue<i32> = PriorityQueue::new(10);
        assert_eq!(p_q.heap_size, 10);
    }
}
