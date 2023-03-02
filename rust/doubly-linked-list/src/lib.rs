use std::{marker::PhantomData, ptr::NonNull};

// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Link<T>,
}

pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _phantom: PhantomData<T>,
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    current: Link<T>,
    _phantom: PhantomData<&'a mut T>,
}

pub struct Iter<'a, T> {
    front: Link<T>,
    _phantom: PhantomData<&'a T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            front: None,
            back: None,
            len: 0,
            _phantom: PhantomData,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            current: self.front,
            list: self,
            _phantom: PhantomData,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            current: self.back,
            list: self,
            _phantom: PhantomData,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            front: self.front,
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.current.map(|cur| &mut (*cur.as_ptr()).value) }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            self.current = self.current.and_then(|cur| (*cur.as_ptr()).next);
            self.current.map(|cur| &mut (*cur.as_ptr()).value)
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe {
            self.current = self.current.and_then(|cur| (*cur.as_ptr()).prev);
            self.current.map(|cur| &mut (*cur.as_ptr()).value)
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        if let Some(cur) = self.current {
            unsafe {
                let cur = Box::from_raw(cur.as_ptr());
                if let Some(prev) = cur.prev {
                    (*prev.as_ptr()).next = cur.next;
                } else {
                    self.list.front = cur.next;
                }
                if let Some(next) = cur.next {
                    (*next.as_ptr()).prev = cur.prev;
                } else {
                    self.list.back = cur.prev;
                }
                self.current = cur.next.or(cur.prev);
                self.list.len -= 1;
                Some(cur.value)
            }
        } else {
            None
        }
    }

    pub fn insert_after(&mut self, value: T) {
        unsafe {
            let node = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                value,
                prev: None,
                next: None,
            })));
            if let Some(cur) = self.current {
                (*node.as_ptr()).prev = Some(cur);
                (*node.as_ptr()).next = (*cur.as_ptr()).next;
                if let Some(next) = (*cur.as_ptr()).next {
                    (*next.as_ptr()).prev = Some(node);
                } else {
                    self.list.back = Some(node);
                }
                (*cur.as_ptr()).next = Some(node);
            } else {
                self.list.front = Some(node);
                self.list.back = Some(node);
                self.current = Some(node);
            }
        }
        self.list.len += 1;
    }

    pub fn insert_before(&mut self, value: T) {
        unsafe {
            let node = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                value,
                prev: None,
                next: None,
            })));
            if let Some(cur) = self.current {
                (*node.as_ptr()).next = Some(cur);
                (*node.as_ptr()).prev = (*cur.as_ptr()).prev;
                if let Some(prev) = (*cur.as_ptr()).prev {
                    (*prev.as_ptr()).next = Some(node);
                } else {
                    self.list.front = Some(node);
                }
                (*cur.as_ptr()).prev = Some(node);
            } else {
                self.list.front = Some(node);
                self.list.back = Some(node);
                self.current = Some(node);
            }
        }
        self.list.len += 1;
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(front) = self.front {
            unsafe {
                self.front = (*front.as_ptr()).next;
                Some(&(*front.as_ptr()).value)
            }
        } else {
            None
        }
    }
}
