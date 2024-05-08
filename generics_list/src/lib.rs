#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let mut node: Node<T> = Node { value, next: None };
        if let Some(head) = self.head.take() {
            node.next = Some(Box::new(head));
            self.head = Some(node);
        } else {
            self.head = Some(node)
        }

        println!("")
    }

    pub fn pop(&mut self) {
        if let Some(head) = self.head.take() {
            self.head = head.next.map(|boxed_node| *boxed_node);
        }
    }

    pub fn len(&self) -> usize {
        let mut count = 0 as usize;
        if let Some(head) = &self.head {
            let mut current = Some(head);
            while let Some(node) = current {
                count += 1;
                current = node.next.as_ref().map(|boxed_node| &**boxed_node);
                if node.next.is_none() {
                    break;
                }
            }
        };
        count
    }
}
