// first linked list

use std::mem;

// keeps track of head
pub struct List {
    head: Link,
}

enum Link {
    Empty,
    Some(Box<Node>),
}

struct Node {
    val: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, val: i32) {
        let new_node = Box::new(Node {
            val,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::Some(new_node);
    }

    // removes head of list and returns it.
    // sets next element as new head
    pub fn pop(&mut self) -> Option<i32> {
        // replace self.head (node) with an empty link and return the head
        let head_link: Link = mem::replace(&mut self.head, Link::Empty);
        match head_link {
            Link::Empty => Option::None,
            Link::Some(box_node) => {
                self.head = box_node.next;
                Option::Some(box_node.val)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_list() {
        let mut list = List::new();

        // empty list reutrns none
        assert_eq!(list.pop(), Option::None);

        // add nodes to list
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);

        // check removal
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
