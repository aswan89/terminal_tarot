use std::path::Path;
use dialoguer::Select;
use walkdir::WalkDir;

pub trait StoredElement  {

    fn new_from_json(json: &str) -> Self
    where 
        Self: std::marker::Sized;

    fn new_from_path(path: &Path) -> Self 
    where 
        Self: std::marker::Sized + std::fmt::Display
    {
        let mut found_items = Vec::new(); 
        for e in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if e.metadata().unwrap().is_file() {
                found_items.push(Self::new_from_file(e.path()));
            }
        }

        match found_items.len() {
            1 => {
                return found_items.remove(0)
            }
            _ => {
                let mut menu = Select::new();
                let selection = menu.items(&found_items[..])
                    .with_prompt("Make a selection:")
                    .interact();
                return found_items.remove(
                    selection.expect("Error at menu select")
                    );
            }
        }
    }

    fn new_from_file(path: &Path) -> Self 
    where Self: std::marker::Sized {
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::open(path).expect("Error when opening file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Error when reading file contents");
        
        match path.extension() {
            None => panic!("Need file extension to determine deserialization method!"),
            Some(os_str) => {
                match os_str.to_str() {
                    Some("json") => Self::new_from_json(&contents),
                    _ => panic!("Don't know how to deserialize file type!"),
                }
            }
        }
    }
}
