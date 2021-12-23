use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct SearchQueue<T> {
    queue: BinaryHeap<Reverse<WithCost<T>>>,
}

impl<T> SearchQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
        }
    }
    pub fn push(&mut self, cost: usize, node: T) {
        self.queue.push(Reverse(WithCost(cost, node)));
    }

    pub fn pop(&mut self) -> Option<(usize, T)> {
        if let Some(Reverse(WithCost(cost, node))) = self.queue.pop() {
            Some((cost, node))
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct WithCost<T>(usize, T);

impl<T: Copy> Copy for WithCost<T> {}

impl<T> PartialEq for WithCost<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T> Eq for WithCost<T> {}

impl<T> PartialOrd for WithCost<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> Ord for WithCost<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
