use ::std;
use std::assert_eq;
use std::boxed::Box;
use std::mem;
use std::ops;
use std::option;
use std::option::Option;
use std::unimplemented;
pub struct List {
    head: Link,
}

// enum Link {
//     Empty,
//     Data(Box<Node>),
// }

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        return List { head: Link::None };
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::None),
        });

        self.head = Link::Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let result;
        match mem::replace(&mut self.head, Link::None) {
            Link::None => {
                result = Option::None;
            }
            Link::Some(node) => {
                result = Option::Some(node.elem);
                self.head = node.next;
            }
        }

        return result;
    }
}

impl ops::Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::None);

        // while true let ...
        while let Link::Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_init() {
        let _list = List::new();
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
}
