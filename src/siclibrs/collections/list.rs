use ::std;
use std::assert_eq;
use std::boxed::Box;
use std::mem;
use std::ops;
use std::option;
use std::option::Option;
use std::unimplemented;
pub struct List<T> {
    head: Link<T>,
}

// enum Link {
//     Empty,
//     Data(Box<Node>),
// }

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        return List { head: Link::None };
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Link::Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        return self.head.take().map(|node| {
            self.head = node.next;
            return node.elem;
        });
    }

    pub fn peek(&self) -> Option<&T> {
        return self.head.as_ref().map(|node| {
            return &node.elem;
        });
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        return self.head.as_mut().map(|node| {
            return &mut node.elem;
        });
    }
}

impl<T> ops::Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        // while true let ...
        while let Link::Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_init() {
        let _list = List::<i32>::new();
    }

    #[test]
    fn list_push_pop_test() {
        let mut list = List::new();

        list.push(1);
        list.push(3);

        assert_eq!(list.pop(), Option::Some(3));
        assert_eq!(list.pop(), Option::Some(1));
        assert_eq!(list.pop(), Option::None);

        list.push(5);
        assert_eq!(list.pop(), Option::Some(5));
    }

    #[test]
    fn list_peek_test() {
        let mut list = List::new();
        assert_eq!(list.peek(), Option::None);
        assert_eq!(list.peek_mut(), Option::None);

        list.push(1);
        list.push(3);

        assert_eq!(list.peek(), Option::Some(&3));

        // let _ = list.peek_mut().replace(&mut 5);
        // assert_eq!(list.peek(), Option::Some(&5));

        list.peek_mut().map(|value| {
            *value = 10;
        });
        assert_eq!(list.peek(), Option::Some(&10));
    }
}
