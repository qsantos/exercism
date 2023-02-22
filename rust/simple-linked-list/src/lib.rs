use std::iter::FromIterator;

pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut anchor = &self.head;
        while let Some(node) = anchor {
            count += 1;
            anchor = &node.next;
        }
        count
    }

    pub fn push(&mut self, value: T) {
        let next = self.head.take();
        self.head = Some(Box::new(Node { value, next }))
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.value)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(node) = &self.head {
            Some(&node.value)
        } else {
            None
        }
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut rev = SimpleLinkedList::new();
        while let Some(value) = self.pop() {
            rev.push(value);
        }
        rev
    }
}

impl<T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for value in iter {
            list.push(value)
        }
        list
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = Vec::new();
        let mut maybe_node = linked_list.head.take();
        while let Some(node) = maybe_node {
            vec.push(node.value);
            maybe_node = node.next;
        }
        vec.reverse();
        vec
    }
}
