// extern crate protoc_rust;

use protoc_rust::Customize;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use std::{env, fs};

// use protobuf_codegen::Codegen;

use loki_proto::loki_proto::proto_message::*;
use loki_proto::loki_proto::proto_visitor::*;
use loki_spec::loki_spec::message_loader::*;

use std::io::Write;

use walkdir::WalkDir;

fn main() {
    //0. clear buildup
    clear_buildup();

    //1. parse and generate .rs files from .proto files
    convert_proto_files();
    // convert_proto_files_serde();

    //2. load proto messages and convert to loki's proto_message
    let mut msg_loader = MsgLoader::default();

    let mut proto_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    proto_path.push("protos");
    // println!("testcases file: {:?}", proto_path);
    msg_loader.load_proto_messages_with_path(proto_path);

    // for msg in msg_loader.get_proto_messages(){
    //     println!("here: {},{}", msg.get_name(), msg.get_path());
    //     if msg.is_same_name(){
    //         println!("{},{}", msg.get_name(), msg.get_path());
    //     }
    // }

    //2.3 fix import errors
    fix_import_errors(msg_loader.get_visitors());

    //3. generate encode.rs & decode.rs
    // generate_encode_file(msg_loader.get_proto_messages());
    // generate_decode_file(msg_loader.get_proto_messages());

    generate_encode_hashmap_file(msg_loader.get_proto_messages());
    generate_decode_hashmap_file(msg_loader.get_proto_messages());
}

fn clear_buildup() {
    let mut proto_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    proto_path.push("src");
    proto_path.push("protos");

    fs::remove_dir_all(proto_path).unwrap_or_else(|why| {
        println!("Error! {:?}", why.kind());
    });
}

#[allow(deprecated)]
fn convert_proto_files() {
    let mut proto_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let pre_index = proto_path.display().to_string().len();
    proto_path.push("protos");

    for entry in WalkDir::new(proto_path.as_os_str())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let f_path = entry.path().display().to_string();
        if f_name.ends_with(".proto") {
            // println!("{} : {}", f_name, (&f_path[pre_index+1..f_path.len()]).to_string());

            let mut dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            dir_path.push("src");
            dir_path.push(&f_path[pre_index + 1..f_path.len() - (f_name.len() + 1)]);

            let d_path = dir_path.display().to_string();

            // println!("output dir is: {}",(&d_path[pre_index+1..d_path.len()]).to_string());

            match fs::create_dir_all(dir_path) {
                Err(why) => println!("Error! {:?}", why.kind()),
                Ok(_) => {}
            }

            // println!("out_dir {}, input_path {}", &d_path[pre_index+1..d_path.len()], &f_path[pre_index+1..f_path.len()]);

            // generate .rs files by Codegen
            protobuf_codegen::Codegen::new()
                // Use `protoc` parser, optional.
                .protoc()
                .includes(&["protos"])
                // Inputs must reside in some of include paths.
                .input(&f_path[pre_index + 1..f_path.len()])
                // Specify output directory relative to Cargo output directory.
                .cargo_out_dir(&d_path[pre_index + 1..d_path.len()])
                .run_from_script();
        }
    }

    //2.1 remove mod.rs in each dir
    remove_mod_rs_files();
    //2.2 generate mod.rs in each dir
    generate_mod_rs_files();
}

#[allow(deprecated)]
fn convert_proto_files_old() {
    let mut proto_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let pre_index = proto_path.display().to_string().len();
    proto_path.push("protos");

    for entry in WalkDir::new(proto_path.as_os_str())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let f_path = entry.path().display().to_string();
        if f_name.ends_with(".proto") {
            // println!("{} : {}", f_name, (&f_path[pre_index+1..f_path.len()]).to_string());

            let mut dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            dir_path.push("src");
            dir_path.push(&f_path[pre_index + 1..f_path.len() - (f_name.len() + 1)]);

            let d_path = dir_path.display().to_string();

            // println!("output dir is: {}",(&d_path[pre_index+1..d_path.len()]).to_string());

            match fs::create_dir_all(dir_path) {
                Err(why) => println!("Error! {:?}", why.kind()),
                Ok(_) => {}
            }

            // generate .rs files by protoc
            protoc_rust::run(protoc_rust::Args {
                out_dir: (&d_path[pre_index + 1..d_path.len()]),
                input: &[(&f_path[pre_index + 1..f_path.len()])],
                includes: &["protos"],
                customize: Customize {
                    ..Default::default()
                },
            })
            .expect("protoc");
        }
    }

    //2.1 generate mod.rs in each dir
    generate_mod_rs_files();
}

fn remove_mod_rs_files() {
    let mut proto_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    proto_path.push("src");
    proto_path.push("protos");

    for entry in WalkDir::new(proto_path.as_os_str())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let f_path = entry.path().display().to_string();

        // println!("{} : {}", f_name, f_path);
        if f_name.ends_with("mod.rs") {
            fs::remove_file(f_path).unwrap_or_else(|why| {
                println!("Error! {:?}", why.kind());
            });
        }
    }
}

fn generate_mod_rs_files() {
    let mut proto_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    proto_path.push("src");
    proto_path.push("protos");

    for entry in WalkDir::new(proto_path.as_os_str())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let f_path = entry.path().display().to_string();

        // println!("{} : {}", f_name, f_path);
        if f_name.ends_with(".rs") {
            if f_name.ends_with("mod.rs") {
                // println!("{} : {}", f_name, f_path);
            } else {
                // println!("{} : {}", f_name, f_path);
                let content = (&f_name[0..f_name.len() - 3]).to_string();
                let mut path_buf = entry.into_path();
                path_buf.pop();
                path_buf.push("mod.rs");
                // println!("path is: {}", path_buf.display());
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(path_buf)
                    .expect("cannot open file");

                file.write_all(format!("pub mod {};\n", content).as_bytes())
                    .expect("write failed");
            }
        } else {
            let content = f_name.to_string();
            // print!("name is {}", f_name);

            let mut entry_copy = entry.clone();

            let mut path_buf = entry.into_path();
            path_buf.push("mod.rs");
            let mut file = std::fs::File::create(path_buf).expect("create failed");

            let mut tmp = entry_copy.into_path();
            tmp.pop();
            tmp.push("mod.rs");
            // println!(" path is: {}", tmp.display());
            if tmp.exists() {
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(tmp)
                    .expect("cannot open file");
                file.write_all(format!("pub mod {};\n", content).as_bytes())
                    .expect("write failed");
            }
        }
    }
}

#[allow(dead_code)]
fn fix_import_errors(visitors: &Vec<Protovisitor>) {
    let mut proto_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    proto_path.push("src");
    proto_path.push("protos");

    let pre_index = proto_path.display().to_string().len();

    for entry in WalkDir::new(proto_path.as_os_str())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let f_path = entry.path().display().to_string();

        // println!("{} : {}", f_name, f_path);
        if f_name.ends_with(".rs") {
            if f_name.ends_with("mod.rs") {
            } else {
                let cur_name = &f_path[pre_index + 1..f_path.len() - 3];
                // println!("here!!! {}", cur_name.to_string());
                for visitor in visitors {
                    let vis_name = &visitor.get_name()[0..visitor.get_name().len() - 6];
                    // println!("here!!! {}", visitor.get_name().to_string());
                    if vis_name.to_string() == cur_name.to_string() {
                        for import in visitor.get_imports() {
                            // println!("import is {}", import);

                            let import_str = import[0..import.len() - 6].to_string();

                            let values: Vec<&str> = import_str.split('/').collect();

                            let mut import_path = "crate::protos::".to_string();
                            let tmp = import_str.replace("/", "::");
                            import_path += &tmp;
                            import_path += "::";

                            let mut error_path_1 = "super::super::".to_string();
                            error_path_1 += values[values.len() - 1];
                            error_path_1 += "::";

                            let mut error_path_2 = "super::".to_string();
                            error_path_2 += values[values.len() - 1];
                            error_path_2 += "::";

                            //TODO more super::*

                            // println!("error path {} ; import path {}", error_path, import_path);

                            // let tmp_path = f_path.clone();

                            let mut error_file = File::open(&f_path).expect("open file failed");
                            let mut error_data = String::new();
                            error_file
                                .read_to_string(&mut error_data)
                                .expect("read file data failed");
                            drop(error_file);
                            // if error_path ==
                            let mut new_data = error_data.replace(&error_path_1, &import_path);
                            new_data = new_data.replace(&error_path_2, &import_path);

                            let mut new_file = File::create(&f_path).expect("open file failed");
                            new_file
                                .write(new_data.as_bytes())
                                .expect("write file failed");
                        }
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
fn generate_encode_file(msgs: &Vec<ProtoMessage>) {
    let mut proto_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    proto_path.push("src");
    proto_path.push("encode.rs");

    let mut file = std::fs::File::create(proto_path).expect("create failed");

    // 1. add import
    file.write_all("use protobuf::Message;\n".as_bytes())
        .expect("write failed");

    let mut proto_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    proto_path.push("src");
    proto_path.push("protos");
    let pre_index = proto_path.display().to_string().len();

    for entry in WalkDir::new(proto_path.as_os_str())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let f_path = entry.path().display().to_string();

        if f_name.ends_with(".rs") {
            if f_name.ends_with("mod.rs") {
            } else {
                // println!("{} : {}", f_name, (&f_path[pre_index+1..f_path.len()-3]));
                let mut tmp_str = &f_path[pre_index + 1..f_path.len() - 3];
                let new_data = tmp_str.replace("/", "::");
                // println!("{}", new_data);
                file.write_all(format!("use crate::protos::{}::*;\n", new_data).as_bytes())
                    .expect("write failed");
            }
        }
    }

    file.write_all("\n".as_bytes()).expect("write failed");

    // 2. add fn encode
    file.write_all(
        "pub fn encode(msg: loki_spec::loki_spec::message::Message) -> Vec<u8> {\n".as_bytes(),
    )
    .expect("write failed");
    file.write_all("\tlet msg_name = msg.get_name();\n\n".as_bytes())
        .expect("write failed");
    file.write_all("\tmatch msg_name.as_str() {\n".as_bytes())
        .expect("write failed");

    // 3. generate swith case
    for msg in msgs {
        let mut tmp_name = msg.get_name();
        let values: Vec<&str> = tmp_name.split('.').collect();
        if values.len() > 1 {
            tmp_name = values[1].to_string();
            // println!("{}",tmp_name);
        } else {
            //3.2 handle duplicative name
            if msg.is_same_name() {
                let mut name = msg.get_path().replace("/", ".");
                name += ".";
                name += &msg.get_name();

                let mut path = "crate::protos::".to_string();
                let s = msg.get_path().replace("/", "::");
                path += &s;
                path += "::";
                path += &msg.get_name();

                file.write_all(format!("\t\t\"{}\" => {} \n", name, "{").as_bytes())
                    .expect("write failed");

                file.write_all(format!("\t\t\tlet tmp = {}::default(); \n", path).as_bytes())
                    .expect("write failed");
                file.write_all(format!("\t\t\tlet bytes = tmp.write_to_bytes().expect(\"{} write to bytes error!\"); \n",msg.get_name()).as_bytes()).expect("write failed");
                file.write_all("\t\t\treturn bytes;\n".as_bytes())
                    .expect("write failed");

                file.write_all("\t\t}\n".as_bytes()).expect("write failed");
            } else {
                file.write_all(format!("\t\t\"{}\" => {} \n", msg.get_name(), "{").as_bytes())
                    .expect("write failed");

                file.write_all(
                    format!("\t\t\tlet tmp = {}::default(); \n", msg.get_name()).as_bytes(),
                )
                .expect("write failed");
                file.write_all(format!("\t\t\tlet bytes = tmp.write_to_bytes().expect(\"{} write to bytes error!\"); \n",msg.get_name()).as_bytes()).expect("write failed");
                file.write_all("\t\t\treturn bytes;\n".as_bytes())
                    .expect("write failed");

                file.write_all("\t\t}\n".as_bytes()).expect("write failed");
            }
        }
    }

    file.write_all("\t\t_ => println!()\n".as_bytes())
        .expect("write failed");
    file.write_all("\t}\n".as_bytes()).expect("write failed");
    file.write_all("\treturn [].to_vec()\n".as_bytes())
        .expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");
}

#[allow(dead_code)]
fn generate_decode_file(msgs: &Vec<ProtoMessage>) {
    let mut proto_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    proto_path.push("src");
    proto_path.push("decode.rs");

    let mut file = std::fs::File::create(proto_path).expect("create failed");

    file.write_all("use protobuf::Message;\n".as_bytes())
        .expect("write failed");

    let mut proto_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    proto_path.push("src");
    proto_path.push("protos");
    let pre_index = proto_path.display().to_string().len();

    for entry in WalkDir::new(proto_path.as_os_str())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let f_path = entry.path().display().to_string();

        if f_name.ends_with(".rs") {
            if f_name.ends_with("mod.rs") {
            } else {
                // println!("{} : {}", f_name, (&f_path[pre_index+1..f_path.len()-3]));
                let mut tmp_str = &f_path[pre_index + 1..f_path.len() - 3];
                let new_data = tmp_str.replace("/", "::");
                // println!("{}", new_data);
                file.write_all(format!("use crate::protos::{}::*;\n", new_data).as_bytes())
                    .expect("write failed");
            }
        }
    }

    file.write_all("\n".as_bytes()).expect("write failed");

    file.write_all("pub fn decode(bytes: &[u8], msg_name:String) -> loki_spec::loki_spec::message::Message {\n".as_bytes()).expect("write failed");
    file.write_all("\tmatch msg_name.as_str() {\n\n".as_bytes())
        .expect("write failed");

    for msg in msgs {
        let mut tmp_name = msg.get_name();
        let values: Vec<&str> = tmp_name.split('.').collect();
        if values.len() > 1 {
            tmp_name = values[1].to_string();
            // println!("{}",tmp_name);
        } else {
            //3.2 handle duplicative name
            if msg.is_same_name() {
                let mut name = msg.get_path().replace("/", ".");
                name += ".";
                name += &msg.get_name();

                let mut path = "crate::protos::".to_string();
                let s = msg.get_path().replace("/", "::");
                path += &s;
                path += "::";
                path += &msg.get_name();

                file.write_all(format!("\t\t\"{}\" => {} \n", name, "{").as_bytes())
                    .expect("write failed");

                file.write_all(format!("\t\t\tlet _tmp = {}::parse_from_bytes(bytes).expect(\"{} parse from bytes error!\"); \n",path, msg.get_name()).as_bytes()).expect("write failed");
                file.write_all(
                    format!(
                        "\t\t\tlet msg  = loki_spec::loki_spec::message::Message::default(); \n"
                    )
                    .as_bytes(),
                )
                .expect("write failed");
                file.write_all("\t\t\t//convert tmp => msg\n".as_bytes())
                    .expect("write failed");
                file.write_all("\t\t\treturn msg;\n".as_bytes())
                    .expect("write failed");

                file.write_all("\t\t}\n".as_bytes()).expect("write failed");
            } else {
                file.write_all(format!("\t\t\"{}\" => {} \n", msg.get_name(), "{").as_bytes())
                    .expect("write failed");

                // file.write_all(format!("\t\t\tlet mut tmp = {}::parse_from_bytes(bytes).expect(\"{} parse from bytes error!\"); \n",msg.get_name(),msg.get_name()).as_bytes()).expect("write failed");
                // file.write_all(format!("\t\t\tlet mut msg  = loki_spec::loki_spec::message::Message::default(); \n").as_bytes()).expect("write failed");
                file.write_all(format!("\t\t\tlet _tmp = {}::parse_from_bytes(bytes).expect(\"{} parse from bytes error!\"); \n",msg.get_name(),msg.get_name()).as_bytes()).expect("write failed");
                file.write_all(
                    format!(
                        "\t\t\tlet msg  = loki_spec::loki_spec::message::Message::default(); \n"
                    )
                    .as_bytes(),
                )
                .expect("write failed");
                file.write_all("\t\t\t//convert tmp => msg\n".as_bytes())
                    .expect("write failed");
                file.write_all("\t\t\treturn msg;\n".as_bytes())
                    .expect("write failed");

                file.write_all("\t\t}\n".as_bytes()).expect("write failed");
            }
        }
    }

    file.write_all("\t\t_ => println!()\n".as_bytes())
        .expect("write failed");
    file.write_all("\t}\n".as_bytes()).expect("write failed");
    file.write_all("\tlet msg  = loki_spec::loki_spec::message::Message::default();\n".as_bytes())
        .expect("write failed");
    file.write_all("\treturn msg;\n".as_bytes())
        .expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");
}

#[allow(dead_code)]
fn generate_encode_hashmap_file_old(msgs: &Vec<ProtoMessage>) {
    let mut proto_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    proto_path.push("src");
    proto_path.push("encode_map_to_buffer.rs");

    let mut file = std::fs::File::create(proto_path).expect("create failed");

    // 1. add import
    file.write_all("use protobuf::Message;\n".as_bytes())
        .expect("write failed");
    file.write_all("use serde_json::{Map, Value};\n".as_bytes())
        .expect("write failed");

    let mut proto_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    proto_path.push("src");
    proto_path.push("protos");
    let pre_index = proto_path.display().to_string().len();

    for entry in WalkDir::new(proto_path.as_os_str())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let f_path = entry.path().display().to_string();

        if f_name.ends_with(".rs") {
            if f_name.ends_with("mod.rs") {
            } else {
                // println!("{} : {}", f_name, (&f_path[pre_index+1..f_path.len()-3]));
                let mut tmp_str = &f_path[pre_index + 1..f_path.len() - 3];
                let new_data = tmp_str.replace("/", "::");
                // println!("{}", new_data);
                file.write_all(format!("use crate::protos::{}::*;\n", new_data).as_bytes())
                    .expect("write failed");
            }
        }
    }

    file.write_all("\n".as_bytes()).expect("write failed");

    // 2. add fn encode
    file.write_all("pub fn encode(msg: loki_spec::loki_spec::message::Message, content:Map<String, Value>) -> Vec<u8>{\n".as_bytes()).expect("write failed");
    file.write_all("\tlet msg_name = msg.get_name();\n".as_bytes())
        .expect("write failed");
    file.write_all("\tlet msg_attrs = msg.get_attrs();\n\n".as_bytes())
        .expect("write failed");
    file.write_all("\tmatch msg_name.as_str() {\n".as_bytes())
        .expect("write failed");

    // 3. generate swith case
    for msg in msgs {
        let mut tmp_name = msg.get_name();
        let values: Vec<&str> = tmp_name.split('.').collect();
        if values.len() > 1 {
            tmp_name = values[1].to_string();
            // println!("{}",tmp_name);
        } else {
            //3.2 handle duplicative name
            if msg.is_same_name() {
                let mut name = msg.get_path().replace("/", ".");
                name += ".";
                name += &msg.get_name();

                let mut path = "crate::protos::".to_string();
                let s = msg.get_path().replace("/", "::");
                path += &s;
                path += "::";
                path += &msg.get_name();

                file.write_all(format!("\t\t\"{}\" => {} \n", name, "{").as_bytes())
                    .expect("write failed");

                file.write_all(format!("\t\t\tlet tmp = {}::default(); \n", path).as_bytes())
                    .expect("write failed");

                let msg_attrs = msg.get_attrs();

                for attr in msg_attrs {
                    let attr_name = attr.get_attr_name();
                    let attr_type = attr.get_attr_type();

                    file.write_all(
                        format!(
                            "\t\t\tlet tmp_value = content.get(\"{}\").unwrap(); \n",
                            attr_name
                        )
                        .as_bytes(),
                    )
                    .expect("write failed");

                    match attr_type.as_str() {
                        "string" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_str().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                        }
                        "bool" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_bool().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                        }
                        "float" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_f64().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                        }
                        "double" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_f64().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                        }
                        "int32" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_i64().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                        }
                        "int64" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_i64().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                        }
                        "uint32" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_u64().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                        }
                        "uint64" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_u64().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                        }
                        "Array" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_array().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                            //TODO{}
                        }
                        "Struct" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_object().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                            //TODO{}
                        }
                        "bytes" => {
                            file.write_all(format!("\t\t\tlet value = tmp_value.as_str().unwrap().as_bytes().to_vec(); \n").as_bytes()).expect("write failed");
                        }
                        "map" => {
                            file.write_all(format!("\t\t\tlet value = tmp_value; \n").as_bytes())
                                .expect("write failed");
                            println!("here!!! name is {}, type is {}", attr_name, attr_type);
                            //TODO{}
                        }
                        _ => {
                            file.write_all(format!("\t\t\tlet value = tmp_value; \n").as_bytes())
                                .expect("write failed");
                        }
                    }
                    file.write_all(format!("\t\t\ttmp.set_{}(value); \n", attr_name).as_bytes())
                        .expect("write failed");
                    file.write_all("\n".as_bytes()).expect("write failed");
                }

                file.write_all("\n".as_bytes()).expect("write failed");

                file.write_all(format!("\t\t\tlet bytes = tmp.write_to_bytes().expect(\"{} write to bytes error!\"); \n",msg.get_name()).as_bytes()).expect("write failed");
                file.write_all("\t\t\treturn bytes;\n".as_bytes())
                    .expect("write failed");

                file.write_all("\t\t}\n".as_bytes()).expect("write failed");
            } else {
                file.write_all(format!("\t\t\"{}\" => {} \n", msg.get_name(), "{").as_bytes())
                    .expect("write failed");

                file.write_all(
                    format!("\t\t\tlet tmp = {}::default(); \n", msg.get_name()).as_bytes(),
                )
                .expect("write failed");

                let msg_attrs = msg.get_attrs();

                for attr in msg_attrs {
                    let attr_name = attr.get_attr_name();
                    let attr_type = attr.get_attr_type();

                    file.write_all(
                        format!(
                            "\t\t\tlet tmp_value = content.get(\"{}\").unwrap(); \n",
                            attr_name
                        )
                        .as_bytes(),
                    )
                    .expect("write failed");

                    match attr_type.as_str() {
                        "string" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_str().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                        }
                        "bool" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_bool().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                        }
                        "float" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_f64().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                        }
                        "double" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_f64().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                        }
                        "int32" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_i64().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                        }
                        "int64" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_i64().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                        }
                        "uint32" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_u64().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                        }
                        "uint64" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_u64().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                        }
                        "Array" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_array().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                            let ele_type = attr.get_attr_ele_type();
                            // TODO{}
                        }
                        "Struct" => {
                            file.write_all(
                                format!("\t\t\tlet value = tmp_value.as_object().unwrap(); \n")
                                    .as_bytes(),
                            )
                            .expect("write failed");
                            let attr_ref = attr.get_attr_reff();
                            //TODO{}
                        }
                        "bytes" => {
                            file.write_all(format!("\t\t\tlet value = tmp_value.as_str().unwrap().as_bytes().to_vec(); \n").as_bytes()).expect("write failed");
                        }
                        "map" => {
                            file.write_all(format!("\t\t\tlet value = tmp_value; \n").as_bytes())
                                .expect("write failed");
                            println!("here!!! name is {}, type is {}", attr_name, attr_type);
                            //TODO{}
                        }
                        _ => {
                            file.write_all(format!("\t\t\tlet value = tmp_value; \n").as_bytes())
                                .expect("write failed");
                            println!("here!!! name is {}, type is {}", attr_name, attr_type);
                        }
                    }

                    file.write_all(format!("\t\t\ttmp.set_{}(value); \n", attr_name).as_bytes())
                        .expect("write failed");
                    file.write_all("\n".as_bytes()).expect("write failed");
                }

                file.write_all("\n".as_bytes()).expect("write failed");

                file.write_all(format!("\t\t\tlet bytes = tmp.write_to_bytes().expect(\"{} write to bytes error!\"); \n",msg.get_name()).as_bytes()).expect("write failed");
                file.write_all("\t\t\treturn bytes;\n".as_bytes())
                    .expect("write failed");

                file.write_all("\t\t}\n".as_bytes()).expect("write failed");
            }
        }
    }

    file.write_all("\t\t_ => println!()\n".as_bytes())
        .expect("write failed");
    file.write_all("\t}\n".as_bytes()).expect("write failed");
    file.write_all("\treturn [].to_vec()\n".as_bytes())
        .expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");
}

#[allow(dead_code)]
fn generate_encode_hashmap_file(msgs: &Vec<ProtoMessage>) {
    let mut proto_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    proto_path.push("src");
    proto_path.push("encode_map_to_buffer.rs");

    let mut file = std::fs::File::create(proto_path).expect("create failed");

    // 1. add import
    file.write_all("use protobuf::Message;\n".as_bytes())
        .expect("write failed");
    file.write_all("use protobuf_json_mapping::*;\n".as_bytes())
        .expect("write failed");
    file.write_all("use serde_json::{Map, Value, json};\n".as_bytes())
        .expect("write failed");

    let mut proto_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    proto_path.push("src");
    proto_path.push("protos");
    let pre_index = proto_path.display().to_string().len();

    for entry in WalkDir::new(proto_path.as_os_str())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let f_path = entry.path().display().to_string();

        if f_name.ends_with(".rs") {
            if f_name.ends_with("mod.rs") {
            } else {
                // println!("{} : {}", f_name, (&f_path[pre_index+1..f_path.len()-3]));
                let mut tmp_str = &f_path[pre_index + 1..f_path.len() - 3];
                let new_data = tmp_str.replace("/", "::");
                // println!("{}", new_data);
                file.write_all(format!("use crate::protos::{}::*;\n", new_data).as_bytes())
                    .expect("write failed");
            }
        }
    }

    file.write_all("\n".as_bytes()).expect("write failed");

    // 2. add fn encode
    file.write_all("pub fn encode(msg: loki_spec::loki_spec::message::Message, content:Map<String, Value>) -> Vec<u8>{\n".as_bytes()).expect("write failed");
    file.write_all("\tlet msg_name = msg.get_name();\n".as_bytes())
        .expect("write failed");
    // file.write_all("\tlet msg_attrs = msg.get_attrs();\n\n".as_bytes()).expect("write failed");
    file.write_all("\tmatch msg_name.as_str() {\n".as_bytes())
        .expect("write failed");

    // 3. generate swith case
    for msg in msgs {
        let mut tmp_name = msg.get_name();
        let values: Vec<&str> = tmp_name.split('.').collect();
        if values.len() > 1 {
            tmp_name = values[1].to_string();
            // println!("{}",tmp_name);
        } else {
            //3.2 handle duplicative name
            if msg.is_same_name() {
                let mut name = msg.get_path().replace("/", ".");
                name += ".";
                name += &msg.get_name();

                let mut path = "crate::protos::".to_string();
                let s = msg.get_path().replace("/", "::");
                path += &s;
                path += "::";
                path += &msg.get_name();

                file.write_all(format!("\t\t\"{}\" => {} \n", name, "{").as_bytes())
                    .expect("write failed");

                file.write_all(
                    format!("\t\t\tlet json_str = json!(content).to_string(); \n").as_bytes(),
                )
                .expect("write failed");

                file.write_all(format!("\t\t\tlet mut tmp = protobuf_json_mapping::parse_from_str::<{}>(&json_str).expect(\"{} parse from map_str error!\"); \n",path, msg.get_name()).as_bytes()).expect("write failed");

                file.write_all(format!("\t\t\tlet bytes = tmp.write_to_bytes().expect(\"{} write to bytes error!\"); \n",msg.get_name()).as_bytes()).expect("write failed");

                file.write_all("\t\t\treturn bytes;\n".as_bytes())
                    .expect("write failed");

                file.write_all("\t\t}\n".as_bytes()).expect("write failed");
            } else {
                file.write_all(format!("\t\t\"{}\" => {} \n", msg.get_name(), "{").as_bytes())
                    .expect("write failed");

                file.write_all(
                    format!("\t\t\tlet json_str = json!(content).to_string(); \n").as_bytes(),
                )
                .expect("write failed");

                file.write_all(format!("\t\t\tlet mut tmp = protobuf_json_mapping::parse_from_str::<{}>(&json_str).expect(\"{} parse from map_str error!\"); \n",msg.get_name(), msg.get_name()).as_bytes()).expect("write failed");

                file.write_all(format!("\t\t\tlet bytes = tmp.write_to_bytes().expect(\"{} write to bytes error!\"); \n",msg.get_name()).as_bytes()).expect("write failed");

                file.write_all("\t\t\treturn bytes;\n".as_bytes())
                    .expect("write failed");

                file.write_all("\t\t}\n".as_bytes()).expect("write failed");
            }
        }
    }

    file.write_all("\t\t_ => println!()\n".as_bytes())
        .expect("write failed");
    file.write_all("\t}\n".as_bytes()).expect("write failed");
    file.write_all("\treturn [].to_vec()\n".as_bytes())
        .expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");
}

#[allow(dead_code)]
fn generate_decode_hashmap_file(msgs: &Vec<ProtoMessage>) {
    let mut proto_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    proto_path.push("src");
    proto_path.push("decode_map_to_buffer.rs");

    let mut file = std::fs::File::create(proto_path).expect("create failed");

    // 1. add import
    file.write_all("use protobuf::Message;\n".as_bytes())
        .expect("write failed");
    file.write_all("use protobuf_json_mapping::*;\n".as_bytes())
        .expect("write failed");
    file.write_all("use serde_json::{Map, Value, json};\n".as_bytes())
        .expect("write failed");

    let mut proto_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    proto_path.push("src");
    proto_path.push("protos");
    let pre_index = proto_path.display().to_string().len();

    for entry in WalkDir::new(proto_path.as_os_str())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let f_path = entry.path().display().to_string();

        if f_name.ends_with(".rs") {
            if f_name.ends_with("mod.rs") {
            } else {
                // println!("{} : {}", f_name, (&f_path[pre_index+1..f_path.len()-3]));
                let mut tmp_str = &f_path[pre_index + 1..f_path.len() - 3];
                let new_data = tmp_str.replace("/", "::");
                // println!("{}", new_data);
                file.write_all(format!("use crate::protos::{}::*;\n", new_data).as_bytes())
                    .expect("write failed");
            }
        }
    }

    file.write_all("\n".as_bytes()).expect("write failed");

    // 2. add fn encode
    file.write_all("pub fn decode(msg: loki_spec::loki_spec::message::Message, stream: &[u8]) -> Map<String, Value>{\n".as_bytes()).expect("write failed");
    file.write_all("\tlet msg_name = msg.get_name();\n".as_bytes())
        .expect("write failed");
    // file.write_all("\tlet msg_attrs = msg.get_attrs();\n\n".as_bytes()).expect("write failed");
    file.write_all("\tmatch msg_name.as_str() {\n".as_bytes())
        .expect("write failed");

    // 3. generate swith case
    for msg in msgs {
        let mut tmp_name = msg.get_name();
        let values: Vec<&str> = tmp_name.split('.').collect();
        if values.len() > 1 {
            tmp_name = values[1].to_string();
            // println!("{}",tmp_name);
        } else {
            //3.2 handle duplicative name
            if msg.is_same_name() {
                let mut name = msg.get_path().replace("/", ".");
                name += ".";
                name += &msg.get_name();

                let mut path = "crate::protos::".to_string();
                let s = msg.get_path().replace("/", "::");
                path += &s;
                path += "::";
                path += &msg.get_name();

                file.write_all(format!("\t\t\"{}\" => {} \n", name, "{").as_bytes())
                    .expect("write failed");

                file.write_all(format!("\t\t\tlet tmp = {}::parse_from_bytes(stream).expect(\"{} parse from bytes error!\"); \n",path, msg.get_name()).as_bytes()).expect("write failed");
                file.write_all("\t\t\t//convert tmp => map\n".as_bytes())
                    .expect("write failed");

                file.write_all(format!("\t\t\tlet json_str = protobuf_json_mapping::print_to_string(&tmp).expect(\"{} parse to map_str error!\");\n",msg.get_name()).as_bytes()).expect("write failed");

                file.write_all(format!("\t\t\tlet v: Map<String,Value> = serde_json::from_str(&json_str).expect(\"serde_json::from_str error!\");\n").as_bytes()).expect("write failed");

                file.write_all("\t\t\treturn v;\n".as_bytes())
                    .expect("write failed");

                file.write_all("\t\t}\n".as_bytes()).expect("write failed");
            } else {
                file.write_all(format!("\t\t\"{}\" => {} \n", msg.get_name(), "{").as_bytes())
                    .expect("write failed");

                file.write_all(format!("\t\t\tlet tmp = {}::parse_from_bytes(stream).expect(\"{} parse from bytes error!\"); \n",msg.get_name(),msg.get_name()).as_bytes()).expect("write failed");

                file.write_all("\t\t\t//convert tmp => map\n".as_bytes())
                    .expect("write failed");

                file.write_all(format!("\t\t\tlet json_str = protobuf_json_mapping::print_to_string(&tmp).expect(\"{} parse to map_str error!\");\n",msg.get_name()).as_bytes()).expect("write failed");

                file.write_all(format!("\t\t\tlet v: Map<String,Value> = serde_json::from_str(&json_str).expect(\"serde_json::from_str error!\");\n").as_bytes()).expect("write failed");

                file.write_all("\t\t\treturn v;\n".as_bytes())
                    .expect("write failed");

                file.write_all("\t\t}\n".as_bytes()).expect("write failed");
            }
        }
    }

    file.write_all("\t\t_ => println!()\n".as_bytes())
        .expect("write failed");
    file.write_all("\t}\n".as_bytes()).expect("write failed");
    file.write_all("\treturn serde_json::Map::new();\n".as_bytes())
        .expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");
}
