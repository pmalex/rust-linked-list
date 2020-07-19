pub mod one_linked_list {
    use std::fmt;

    #[derive(Debug)]
    struct Node<T> {
        data: T,
        next: Option<Box<Node<T>>>,
    }

    impl<T: fmt::Display> fmt::Display for Node<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{{{}}}", self.data)
        }
    }

    #[derive(Debug)]
    pub struct List<T> {
        head: Option<Box<Node<T>>>,
    }

    impl<T> fmt::Display for List<T>
    where
        T: fmt::Display,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut node = &self.head;

            while let Some(tmp) = node {
                write!(f, "{} -> ", tmp)?;
                node = &tmp.next;
            }

            write!(f, "None")
        }
    }

    impl<T> std::default::Default for List<T> {
        fn default() -> Self {
            List { head: None }
        }
    }

    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            let mut cur_link = std::mem::replace(&mut self.head, None);
            // `while let` == "do this thing until this pattern doesn't match"
            while let Some(mut boxed_node) = cur_link {
                cur_link = std::mem::replace(&mut boxed_node.next, None);
                // boxed_node goes out of scope and gets dropped here;
                // but its Node's `next` field has been set to None
                // so no unbounded recursion occurs.
            }
        }
    }

    impl<T> List<T> {
        pub fn new() -> List<T> {
            List { head: None }
        }

        pub fn push_front(&mut self, data: T) {
            let new_node = Box::new(Node {
                data: data,
                next: self.head.take(),
            });

            self.head = Some(new_node);
        }

        pub fn pop_front(&mut self) -> Option<T> {
            self.head.take().map(|node| {
                self.head = node.next;
                node.data
            })
        }

        pub fn peek_front(&self) -> Option<&T> {
            self.head.as_ref().map(|node| {
                &node.data
            })
        }

        pub fn peek_front_mut(&mut self) -> Option<&mut T> {
            self.head.as_mut().map(|node| {
                &mut node.data
            })
        }
    }
}
