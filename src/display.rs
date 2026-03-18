use crate::dom::Node;

pub fn pretty_print(node: &Node, depth: usize){
    let indent = "  ".repeat(depth);

    match node{
        Node::Element{tag, children} => {
            println!("{}<{}>", indent, tag);

            for child in children{
                pretty_print(child, depth + 1);
            }

            println!("{}</{}>", indent, tag);
        }
        Node::Text(text) => {
            println!("{}\"{}\"", indent, text);
        }
    }
}