#[derive(Debug)]
#[allow(dead_code)]
pub enum Node{
    // This node can have children, text, tag,
    Element{
        tag: String,
        children: Vec<Node>
    },
    Text(String)
}