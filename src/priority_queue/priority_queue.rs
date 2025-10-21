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
struct PriorityQueue<T: PartialOrd> {
    heap_size: usize,
    heap: Vec<T>,
}

impl<T: PartialOrd> PriorityQueue<T> {
    fn new(size: usize) -> Self {
        Self {
            heap: Vec::new(),
            heap_size: size,
        }
    }
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
        let p_q = PriorityQueue::new(10);
        assert_eq!(p_q.heap_size, 10);
    }
}
