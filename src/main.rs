#[allow(dead_code)]
mod parser;
mod dom;
mod display;

use display::pretty_print;
use parser::Parser;


fn main() {
    let html = "<div><p>Hello</p></div>".to_string();

    let mut parser = Parser::new(html);
    let dom = parser.parse();

    println!("{:#?}", dom);
    println!("\n");
    pretty_print(&dom, 0);
}
