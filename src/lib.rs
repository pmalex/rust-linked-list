pub mod one_linked_list {
    use std::fmt;

    #[derive(Debug)]
    struct Node<T> {
        data: T,
        next: Option<Box<Node<T>>>,
    }

    impl<T: fmt::Display> fmt::Display for Node<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{{ {} }}", self.data)
        }
    }

    #[derive(Debug)]
    pub struct List<T> {
        head: Option<Box<Node<T>>>,
    }

    impl<T> fmt::Display for List<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "List:")
        }
    }

    impl<T> std::default::Default for List<T> {
        fn default() -> Self { List {head: None} }
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
    }
}
