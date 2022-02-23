// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use self::{queue::*, stack::*};
use std::sync::{
    atomic::{AtomicPtr, Ordering},
    Arc,
};

pub trait Queue {
    /// The inner item type
    type Item;

    /// Puts an item into the queue at the end
    fn put(&self, value: Self::Item);

    /// Removes the first item. Returns [`None`], if no item is present
    fn poll(&self) -> Option<&Self::Item>;
}

pub trait Stack {
    /// The inner item type
    type Item;

    /// Pushes an item to the top of the stack
    fn push(&self, value: Self::Item);

    /// Pops an item from the top of the stack. Returns [`None`],
    /// if no item is present.
    fn pop(&self) -> Option<&Self::Item>;
}

struct Node<T> {
    value: Option<T>,
    next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value: Some(value),
            next: AtomicPtr::new(std::ptr::null_mut()),
        }
    }

    fn empty() -> Self {
        Self {
            value: None,
            next: AtomicPtr::new(std::ptr::null_mut()),
        }
    }
}

mod stack {
    use super::*;

    /// A non blocking stack using the Michael & Scott implementation
    pub struct NonBlockingStack<T> {
        head: Arc<AtomicPtr<Node<T>>>,
    }

    impl<T> Clone for NonBlockingStack<T> {
        fn clone(&self) -> Self {
            Self {
                head: self.head.clone(),
            }
        }
    }

    impl<T> Default for NonBlockingStack<T> {
        fn default() -> Self {
            Self {
                head: Arc::new(AtomicPtr::new(std::ptr::null_mut())),
            }
        }
    }

    impl<T> Stack for NonBlockingStack<T>
    where
        T: Clone,
    {
        type Item = T;

        fn push(&self, value: Self::Item) {
            match self.head.load(Ordering::Acquire) {
                node_ptr if node_ptr.is_null() => {
                    let new = Node::new(value);
                    self.head.store(Box::into_raw(Box::new(new)), Ordering::Release)
                }
                node_ptr if !node_ptr.is_null() => loop {
                    let new = Node::new(value.clone());
                    let old = unsafe { &mut *node_ptr };
                    new.next.store(old, Ordering::Release);

                    if self
                        .head
                        .compare_exchange(old, Box::into_raw(Box::new(new)), Ordering::Release, Ordering::Relaxed)
                        .is_ok()
                    {
                        break;
                    }
                },

                _ => {}
            }
        }

        fn pop(&self) -> Option<&Self::Item> {
            match self.head.load(Ordering::Acquire) {
                node_ptr if node_ptr.is_null() => None,
                node_ptr if !node_ptr.is_null() => loop {
                    let old = unsafe { &mut *node_ptr };
                    match old.next.load(Ordering::Acquire) {
                        next_ptr if next_ptr.is_null() => {
                            self.head.store(std::ptr::null_mut(), Ordering::Release);
                            return old.value.as_ref();
                        }
                        next_ptr if !next_ptr.is_null() => {
                            if self
                                .head
                                .compare_exchange(old, next_ptr, Ordering::Release, Ordering::Relaxed)
                                .is_ok()
                            {
                                return old.value.as_ref();
                            }
                        }
                        _ => return None,
                    };
                },
                _ => None,
            }
        }
    }
}

mod queue {

    use super::*;

    pub struct NonBlockingQueue<T> {
        head: Arc<AtomicPtr<Node<T>>>,
        tail: Arc<AtomicPtr<Node<T>>>,
    }

    impl<T> Default for NonBlockingQueue<T> {
        fn default() -> Self {
            let inner = Box::into_raw(Box::new(Node::<T>::empty()));

            Self {
                head: Arc::new(AtomicPtr::new(inner)),
                tail: Arc::new(AtomicPtr::new(inner)),
            }
        }
    }

    impl<T> Queue for NonBlockingQueue<T> {
        type Item = T;

        fn put(&self, value: Self::Item) {
            let new = Node::new(value);
            let new_ptr = Box::into_raw(Box::new(new));

            let tail_ptr = self.tail.load(Ordering::Acquire);

            match tail_ptr.is_null() {
                true => self.tail.store(new_ptr, Ordering::Release),
                false => loop {
                    let tail = unsafe { &*tail_ptr };

                    if tail
                        .next
                        .compare_exchange(std::ptr::null_mut(), new_ptr, Ordering::Release, Ordering::Relaxed)
                        .is_ok()
                    {
                        self.tail.store(tail.next.load(Ordering::Acquire), Ordering::Release);
                        break;
                    }
                },
            }
        }

        fn poll(&self) -> Option<&Self::Item> {
            loop {
                let head_ptr = self.head.load(Ordering::Acquire);
                let tail_ptr = self.tail.load(Ordering::Acquire);

                let next_ptr = match head_ptr.is_null() {
                    true => return None,
                    false => {
                        let head = unsafe { &*head_ptr };

                        head.next.load(Ordering::Acquire)
                    }
                };

                if head_ptr.eq(&self.head.load(Ordering::Acquire)) {
                    if head_ptr == tail_ptr {
                        if next_ptr.is_null() {
                            return None;
                        }

                        self.tail
                            .compare_exchange(tail_ptr, next_ptr, Ordering::Release, Ordering::Relaxed)
                            .expect("swapping tail ptr failed");
                    } else {
                        let result = &unsafe { &*next_ptr }.value;

                        if self
                            .head
                            .compare_exchange(head_ptr, next_ptr, Ordering::Release, Ordering::Relaxed)
                            .is_ok()
                        {
                            return result.as_ref();
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use rand_utils::random::usize;

    #[inline(always)]
    fn rand_usize() -> usize {
        usize(usize::MAX)
    }

    #[test]
    fn test_stack() {
        let stack = NonBlockingStack::default();
        let end = 10000;

        (0..=end).for_each(|n| stack.push(n));
        (0..=end).rev().for_each(|n| assert_eq!(Some(&n), stack.pop()));

        assert_eq!(None, stack.pop());
    }

    #[test]
    fn test_queue() {
        let queue = NonBlockingQueue::default();
        let end = 2;

        (0..=end).for_each(|n| queue.put(n));
        (0..=end).for_each(|n| assert_eq!(Some(&n), queue.poll()));

        assert_eq!(None, queue.poll());
    }
}
