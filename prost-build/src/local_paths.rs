use std::collections::{BTreeMap, BTreeSet};

use crate::util;
use prost_types::{DescriptorProto, EnumDescriptorProto, FileDescriptorProto};

#[derive(Default)]
pub struct LocalPaths {
    // pb message path -> relative file path
    message_file_paths: BTreeMap<String, Vec<String>>,
    // pb package path -> relative file path
    file_paths: BTreeMap<String, BTreeSet<Vec<String>>>,
    // relative file path -> pb package path
    pb_paths: BTreeMap<Vec<String>, String>,
}

impl LocalPaths {
    pub fn new(files: &[FileDescriptorProto]) -> Result<Self, String> {
        let mut msg_graph = Self::default();

        for file in files {
            let file_path: Vec<String> = util::get_file_path(&file);
            let pb_path = format!(
                "{}{}",
                if file.package.is_some() { "." } else { "" },
                file.package.as_ref().map(String::as_str).unwrap_or("")
            );
            for msg in &file.message_type {
                msg_graph.add_message(file_path.clone(), &pb_path, msg);
            }
            for enum_ in &file.enum_type {
                msg_graph.add_enum(file_path.clone(), &pb_path, enum_);
            }
            msg_graph
                .pb_paths
                .insert(file_path.clone(), pb_path.clone());
            msg_graph
                .file_paths
                .entry(pb_path)
                .or_insert_with(|| Default::default())
                .insert(file_path);
        }

        Ok(msg_graph)
    }

    pub fn message_file_path(&self, pb_path: &str) -> Option<&Vec<String>> {
        self.message_file_paths.get(pb_path)
    }

    pub fn file_path(&self, pb_path: &str) -> Option<&BTreeSet<Vec<String>>> {
        self.file_paths.get(pb_path)
    }

    pub fn pb_path(&self, file_path: &Vec<String>) -> Option<&String> {
        self.pb_paths.get(file_path)
    }

    fn add_message(&mut self, file_path: Vec<String>, pb_path: &str, msg: &DescriptorProto) {
        let msg_name = msg.name.as_ref().unwrap();
        let pb_path = format!("{}.{}", pb_path, msg_name);
        for msg in &msg.nested_type {
            self.add_message(file_path.clone(), &pb_path, msg);
        }
        self.message_file_paths.insert(pb_path, file_path);
    }

    fn add_enum(&mut self, file_path: Vec<String>, pb_path: &str, enum_: &EnumDescriptorProto) {
        let enum_name = enum_.name.as_ref().unwrap();
        let pb_path = format!("{}.{}", pb_path, enum_name);
        self.message_file_paths.insert(pb_path, file_path);
    }
}
