#[allow(dead_code)]
mod dom;
mod display;

use display::pretty_print;
use dom::Node;

fn main() {
    let root = Node::Element {
            tag: "div".to_string(),
            children: vec![
                Node::Element{
                    tag: "h1".to_string(),
                    children: vec![
                        Node::Text("Hello".to_string()),
                    ],
                },
                Node::Element{
                    tag: "p".to_string(),
                    children: vec![
                        Node::Text("World".to_string()),
                    ],
                },
            ]
        };

        pretty_print(&root, 0);
}
