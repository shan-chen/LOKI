// use antlr_rust::common_token_stream::CommonTokenStream;
// use antlr_rust::input_stream::InputStream;
// use antlr_rust::token_factory::CommonTokenFactory;
// use antlr_rust::tree::Visitable;
// use loki_spec::loki_spec::loki_speclexer::*;
// use loki_spec::loki_spec::loki_specparser::*;
// use loki_spec::loki_spec::spec_visitor::*;
// use serde_json::Map;
// use serde_json::Value;
// // use antlr_rust::tree::{ParseTree, ParseTreeVisitor};

// use std::fs;
// use std::path::PathBuf;

// use core::global_definition::*;
// use core::loki_message::LokiMessage;

// fn main() {
//     let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//     config_path.push("testcase");
//     config_path.push("simple.spec");
//     println!("config file: {:?}", config_path);
//     let input_str: String = fs::read_to_string(config_path).unwrap();
//     let input = InputStream::new(&*input_str);
//     let tf = CommonTokenFactory::default();
//     let lexer = Loki_specLexer::new_with_token_factory(input, &tf);
//     let token_source = CommonTokenStream::new(lexer);
//     let mut parser = Loki_specParser::new(token_source);
//     let result = parser.document();

//     // println!("{}",result.is_ok());

//     let result_ctx = result.expect("input parsed unsuccessfully");

//     let mut visitor = Specvisitor::default();
//     result_ctx.accept(&mut visitor);
//     let msgs_list = visitor.get_spec_messages().clone();
//     set_message_list(msgs_list);
//     // set_spec_visitor(visitor);

//     let mut msg = LokiMessage::generate("p2ppacket".to_string());
//     let mut new_content = Map::new();
//     new_content.insert(
//         "message type".to_string(),
//         Value::String(String::from("testing")),
//     );
//     msg.set_content(new_content);
//     let res = msg.encode();
//     assert_eq!(
//         res.unwrap(),
//         vec!(10_u8, 07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67)
//     );
// }

fn main() {}
