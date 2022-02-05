use ::std;
use std::assert_eq;
use std::clone::Clone;
use std::iter::Iterator;
use std::mem;
use std::ops;
use std::option::Option;
use std::rc::Rc;
use std::result::Result;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        return List { head: Link::None };
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        return List {
            head: Link::Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        };
    }

    pub fn tail(&self) -> List<T> {
        return List {
            head: self.head.as_ref().and_then(|node| {
                return node.next.clone();
            }),
        };
    }

    pub fn head(&self) -> Option<&T> {
        return self.head.as_ref().map(|node| {
            return &node.elem;
        });
    }

    pub fn iter(&self) -> Iter<'_, T> {
        return Iter {
            next: self.head.as_ref().map(|node| &**node),
        };
    }
}

impl<T> ops::Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Option::Some(node) = head {
            // panics if there is a reference
            if let Result::Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        return self.next.map(|node| {
            // self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
            self.next = node.next.as_ref().map(|node| &**node);
            return &node.elem;
        });
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

        list = list.prepend(1);
        list = list.prepend(3);

        assert_eq!(list.head(), Option::Some(&3));
        list = list.tail();
        assert_eq!(list.head(), Option::Some(&1));
        list = list.tail();
        assert_eq!(list.head(), Option::None);

        list = list.prepend(5);
        assert_eq!(list.head(), Option::Some(&5));
    }

    #[test]
    fn list_peek_test() {
        let mut list = List::new();
        assert_eq!(list.head(), Option::None);

        list = list.prepend(1);
        list = list.prepend(3);

        assert_eq!(list.head(), Option::Some(&3));
    }

    #[test]
    fn list_iter_test() {
        let mut list = List::new();
        for i in 0..10 {
            list = list.prepend(i);
        }

        let mut iter = list.iter();
        for i in (0..10).rev() {
            assert_eq!(Option::Some(&i), iter.next());
        }

        assert_eq!(iter.next(), Option::None);

        let mut iter = list.iter();
        for i in (0..10).rev() {
            assert_eq!(Option::Some(&i), iter.next());
        }

        assert_eq!(iter.next(), Option::None);

        mem::drop(list);
    }
}
