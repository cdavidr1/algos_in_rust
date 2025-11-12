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

    fn map_update(&mut self, elem: &T, old: usize, new: usize) {
        if let Some(pos) = self.position_map.get_mut(elem) {
            pos.remove(&old);
            pos.insert(new);
        }
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

    fn swim(&mut self, mut index: usize) {
        let mut parent = (index - 1) / 2;
        while index > 0 && self.less(index, parent) {
            self.swap(parent, index);
            index = parent;
            parent = (index - 1) / 2;
        }
    }

    fn less(&self, i: usize, j: usize) -> bool {
        let node_a = self.heap.get(i).unwrap();
        let node_b = self.heap.get(j).unwrap();
        node_a <= node_b
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn clear(&mut self) {
        self.heap.clear();
        self.position_map.clear();
    }

    fn size(&self) -> usize {
        self.heap.len()
    }

    fn swap(&mut self, i: usize, j: usize) {
        let elem_i = self.heap[i].clone();
        let elem_j = self.heap[j].clone();
        self.heap.swap(i, j);
        self.map_update(&elem_i, i, j);
        self.map_update(&elem_j, j, i);
    }

    fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.heap.get(0)
        }
    }

    fn poll(&mut self) {}

    fn removeAt(i: usize) {}

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
