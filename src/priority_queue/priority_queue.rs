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
    heap: Vec<T>,
    position_map: HashMap<T, BTreeSet<usize>>,
}

impl<T> PriorityQueue<T>
where
    T: PartialOrd + Hash + Eq + Clone,
{
    fn new() -> Self {
        Self {
            heap: Vec::new(),
            position_map: HashMap::new(),
        }
    }

    fn make_p_q(elems: &[T]) -> Self {
        let mut p_q = PriorityQueue::new();
        for (index, elem) in elems.iter().enumerate() {
            p_q.heap.push(elem.clone());
            p_q.map_add(elem.clone(), index);
        }
        let mut i = 0.max((p_q.size() / 2) as isize - 1);
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

    // fn map_remove(&mut self, elem: &T, index: usize) {}

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
        while index > 0 {
            let parent = (index - 1) / 2;
            if !self.less(index, parent) {
                break;
            }
            self.swap(parent, index);
            index = parent;
        }
    }

    fn less(&self, i: usize, j: usize) -> bool {
        let node_a = self.heap.get(i).unwrap();
        let node_b = self.heap.get(j).unwrap();
        node_a < node_b
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
        if i == j {
            return;
        }
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

    fn poll(&mut self) -> Option<T> {
        self.remove_at(0)
    }

    fn remove_at(&mut self, index: usize) -> Option<T> {
        if !self.is_empty() {
            let last_index = self.size() - 1;
            let elem = self.heap.get(index).unwrap().clone();

            self.swap(index, last_index);
            self.heap.remove(last_index);
            // self.map_remove(&elem, index);

            if index == last_index {
                return Some(elem);
            }

            let reorder_elem = self.heap.get(index).unwrap().clone();
            self.sink(index);
            if self.heap.get(index).unwrap().eq(&reorder_elem) {
                self.swim(index);
            }
            return Some(elem);
        }
        return None;
    }

    fn contains(&self, elem: T) -> bool {
        self.position_map.contains_key(&elem)
    }

    fn add(&mut self, elem: T) {
        self.heap.push(elem.clone());
        let index = self.size() - 1;
        self.map_add(elem.clone(), index);
        self.swim(index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call() {
        let example_input = vec![4, 2, 8];
        PriorityQueue::make_p_q(&example_input);
    }

    #[test]
    fn test_clear() {
        let mut p_q = _create();
        p_q.clear();
        assert_eq!(p_q.size(), 0);
    }

    #[test]
    fn test_works() {
        assert_eq!(1, 1);
    }

    fn _create() -> PriorityQueue<i32> {
        let p_q: PriorityQueue<i32> = PriorityQueue::new();
        p_q
    }
}
