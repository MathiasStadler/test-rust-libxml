use libxml::parser::Parser;
use libxml::xpath::Context;

fn main() {
  let parser: Parser = Parser::default();
  let doc: libxml::tree::Document = parser.parse_file("tests/resources/file01.xml").unwrap();
  let context: Context = Context::new(&doc).unwrap();
  let result: libxml::xpath::Object = context.evaluate("//child/text()").unwrap();

  for node in &result.get_nodes_as_vec() {
    println!("Found: {}", node.get_content());
  }
}
