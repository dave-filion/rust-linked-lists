// first linked list

// keeps track of head
// make generic
pub struct List<T> {
    head: Link<T>,
}

// type alias
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: Link::None }
    }

    pub fn push(&mut self, val: T) {
        let new_node = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Link::Some(new_node);
    }

    // removes head of list and returns it.
    // sets next element as new head
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

    // Returns head of list without popping
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.val)
    }

    // returns mutable version of head
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|n| &mut n.val)
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

        let mut list2 = List::new();

        list2.push("str1");
        list2.push("str2");

        assert_eq!(list2.pop(), Some("str2"));

        let peek_result = list2.peek();
        println!("peek=>{:?}", peek_result);
        assert_eq!(peek_result, Some(&"str1"));

        let mut mut_peek = list2.peek_mut();
        assert_eq!(mut_peek, Some(&mut "str1"));
    }
}
