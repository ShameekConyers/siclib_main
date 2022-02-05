use ::std;
// use std::borrow::{Borrow, BorrowMut};
use std::assert_eq;
use std::cell::{Ref, RefCell};
use std::clone::Clone;
use std::ops::Drop;
use std::option::Option;
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        return List {
            head: Option::None,
            tail: Option::None,
        };
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Option::Some(old_head) => {
                old_head.borrow_mut().prev = Option::Some(new_head.clone());
                new_head.borrow_mut().next = Option::Some(old_head);
                self.head = Option::Some(new_head)
            }

            Option::None => {
                // empty list
                self.head = Option::Some(new_head.clone());
                self.tail = Option::Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Option::Some(new_head) => {
                    // reconcile differences
                    new_head.borrow_mut().prev.take(); // make null
                    self.head = Option::Some(new_head);
                }
                Option::None => {
                    // List is empty
                    self.tail.take();
                }
            }
            return Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem;
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        return self
            .head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| return &node.elem));
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Option::Some(_) = self.pop_front() {}
    }
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        return Rc::new(RefCell::new(Node {
            elem: elem,
            prev: Option::None,
            next: Option::None,
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_and_node_init() {
        let node = Node::<i32>::new(1);
        let list = List::<i32>::new();
    }

    #[test]
    fn list_push_pop_test() {
        let mut list = List::new();

        list.push_front(1);
        list.push_front(3);

        assert_eq!(list.pop_front(), Option::Some(3));
        assert_eq!(list.pop_front(), Option::Some(1));
        assert_eq!(list.pop_front(), Option::None);

        list.push_front(5);
        assert_eq!(list.pop_front(), Option::Some(5));
    }
}
