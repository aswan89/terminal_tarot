use std::path::Path;

pub trait StoredElement  {

    fn new_from_json(json: &str) -> Self;

    fn new_from_file(path: &Path) -> Self 
        where Self: std::marker::Sized {
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        
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
