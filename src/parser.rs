use crate::dom::Node;

pub struct Parser{
    input: Vec<char>,
    pos: usize,
}

impl Parser{
    pub fn new(input: String) -> Self{
        Self{
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn next_char(&mut self) -> Option<char>{
        let ch = self.peek();
        self.pos += 1;
        ch
    }

    fn starts_with(&self, s: &str) -> bool{
        self.input[self.pos..]
            .iter()
            .collect::<String>()
            .starts_with(s)
    }

    fn parse_text(&mut self) -> Node{
        let mut text = String::new();

        while let Some(c) = self.peek(){
            if c == '<' {
                break;
            }

            text.push(c);
            self.next_char();
        }

        Node::Text(text.trim().to_string())
    }

    fn parse_tag_name(&mut self) -> String {
        let mut name = String::new();

        while let Some(c) = self.peek(){
            if c.is_alphanumeric() {
                name.push(c);
                self.next_char();
            }else{
                break;
            }
        }

        name
    }

    fn parse_element(&mut self) -> Node{
        self.next_char();

        let tag = self.parse_tag_name();

        self.next_char();

        let mut children = Vec::new();

        while !self.starts_with(&format!("</{}", tag)){
            children.push(self.parse_node());
        }

        self.next_char();
        self.next_char();
        self.parse_tag_name();
        self.next_char();

        Node::Element{ tag, children }
    }

    fn parse_node(&mut self) -> Node{
        match self.peek(){
            Some('<') => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    pub fn parse(&mut self) -> Node{
        self.parse_node()
    }
}