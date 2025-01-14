struct Node {
    value: String,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: String) -> Node {
        Node { value, next: None }
    }
}

fn main() {
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
