use std::cell::RefCell;
use std::rc::Rc;

mod node_space {
    pub struct Node {
        pub value: String,
        pub prev: Option<Rc<RefCell<Node>>>,
        pub next: Option<Rc<RefCell<Node>>>,
    }

    impl Node {
        pub fn new(value: String) -> Node {
            Node {
                value,
                prev: None,
                next: None,
            }
        }

        pub fn append(&mut self, mut new_node: Node) {
            let mut tail = self;

            while let Some(ref mut next) = tail.next {
                tail = next;
            }

            let new_node = Rc::new(RefCell::new(new_node));
            new_node.borrow_mut().prev = Some(Rc::clone(&Rc::new(RefCell::new(*tail))));
            tail.next = Some(Rc::clone(&new_node));
        }
    }
}

fn main() {
    use node_space::Node;

    let mut head = Node::new("Hello".to_string());
    let mut tail = &mut head;
    for i in 0..10 {
        let content = format!("Hello {}", i + 1);
        let node = Node::new(content);
        tail.next = Some(Box::new(node));
        tail = tail.next.as_mut().unwrap();
    }

    let mut current = &head;
    while let Some(node) = current.next.as_ref() {
        println!("{}", node.value);
        current = node;
    }
}
