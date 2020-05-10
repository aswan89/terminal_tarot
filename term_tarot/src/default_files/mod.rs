extern crate directories;
use std::path::PathBuf;
use directories::ProjectDirs;
use std::io::{Error, ErrorKind};

struct DefaultFile {
    filename: String,
    file_contents: &'static str,
}

pub enum ElementType {
    Spread,
    Deck,
}

fn return_default_files_dir() -> std::io::Result<PathBuf> {
    match ProjectDirs::from("", "terminal_tarot", "terminal_tarot") {
        Some(path) => Ok(path.data_dir().to_path_buf()),
        None => Err(Error::new(ErrorKind::Other, "couldn't determine data directory")),
    }
}

fn construct_dir_path(new_dir_name: &str) -> std::io::Result<PathBuf>{
    let mut data_dir = return_default_files_dir()?;
    data_dir.push(new_dir_name);
    Ok(data_dir)
}

pub fn write_default_files(element: ElementType, overwrite: bool) -> std::io::Result<PathBuf> {
    let standard_deck = DefaultFile {
        filename: "standard_deck.json".to_string(),
        file_contents: include_str!("included_decks/default_deck.json"),
    };
    
    let debug_spread = DefaultFile {
        filename: "debug_spread.json".to_string(),
        file_contents: include_str!("included_spreads/debug_spread.json"),
    };
    
    let celtic_cross = DefaultFile {
        filename: "celtic_cross.json".to_string(),
        file_contents: include_str!("included_spreads/celtic_cross.json"),
    };

    match element {
        ElementType::Deck => {
            let mut deck_path = construct_dir_path("default_decks")?;
            check_or_write_dir(&mut deck_path, vec![standard_deck], &overwrite)?;
            Ok(deck_path)
        },
        ElementType::Spread => {
            let mut spread_path = construct_dir_path("default_spreads")?;
            check_or_write_dir(&mut spread_path, vec![debug_spread, celtic_cross], &overwrite)?;
            Ok(spread_path)
        },
    }
}

fn check_or_write_dir(target_dir: &mut PathBuf, files_to_write: Vec<DefaultFile>, overwrite: &bool) ->  std::io::Result<()>{
    if target_dir.exists() == false {
        std::fs::create_dir_all(&target_dir)?;
    }
    write_files(target_dir, files_to_write, overwrite)?;
    Ok(())
}

fn write_files(target_dir: &mut PathBuf, files_to_write: Vec<DefaultFile>, overwrite: &bool) -> std::io::Result<()>{
    for file in files_to_write.iter() {
        let mut file_path = target_dir.clone();
        file_path.push(&file.filename);
        if !file_path.exists() | overwrite {
            std::fs::write(file_path, file.file_contents)?;
        }
    }
    Ok(())
}
