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
    T: PartialOrd + Hash + Eq,
{
    heap_size: usize,
    heap: Vec<T>,
    position_map: HashMap<T, BTreeSet<usize>>,
}

impl<T> PriorityQueue<T>
where
    T: PartialOrd + Hash + Eq,
{
    fn new(size: usize) -> Self {
        Self {
            heap: Vec::new(),
            heap_size: size,
            position_map: HashMap::new(),
        }
    }

    fn map_add(&mut self, elem: T, index: usize) {
        if let Some(set) = self.position_map.get_mut(&elem) {
            set.insert(index);
        } else {
            let mut set = BTreeSet::new();
            set.insert(index);
            self.position_map.insert(elem, set);
        }
    }

    fn sink(index: usize) {}

    fn swim(size: usize) {}

    fn less(i: usize, j: usize) {}

    // simple
    fn is_empty() {}

    fn clear() {}

    fn size() {}

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
