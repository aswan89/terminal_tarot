extern crate clap;
extern crate pager;
extern crate shellexpand;
use clap::{App, Arg};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use pager::Pager;

extern crate terminal_tarot;
use terminal_tarot::deck::Deck;
use terminal_tarot::spread::{Spread, FilledSpread};
use terminal_tarot::stored_element::StoredElement;
use terminal_tarot::default_files::{write_default_files, ElementType};

fn main() {
    let now = std::time::SystemTime::now();

    let matches = App::new("Terminal Tarot")
        .arg(Arg::with_name("interactive")
             .short("i")
             .long("interactive")
             .help("Set to view tarot cards/positions in interactive manner")
             )
        .arg(Arg::with_name("detailed")
             .short("d")
             .long("detailed")
             .help("Should multiple interpretations for cards/positions be displayed?")
             )
        .arg(Arg::with_name("seed")
             .short("s")
             .long("seed")
             .takes_value(true)
             .help("Value used to draw cards and select interpretations")
             )
        .arg(Arg::with_name("overwrite_default_files")
             .short("o")
             .long("overwrite")
             .help("Write packaged spread/deck files to default directory ($HOME/.local/share/terminal_tarot)")
             )
        .arg(Arg::with_name("spread_path")
             .long("spread_path")
             .takes_value(true)
             .help("Path that holds desired spread files. Can be a single file or a directory")
             )
        .arg(Arg::with_name("deck_path")
             .long("deck_path")
             .takes_value(true)
             .help("Path that holds desired deck files. Can be a single file or a directory")
             )
        .get_matches();

    #[derive(Hash)]
    struct InputSeed {
        value: String, 
    }

    let seed = InputSeed {
        value: matches.value_of("seed").unwrap_or(
             &now.duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string()
            ).to_string(),
    };

    fn calc_paths(arguments: &clap::ArgMatches, tar_element: ElementType) -> std::path::PathBuf {
        let tar_arg = match tar_element {
            ElementType::Spread => "spread_path",
            ElementType::Deck => "deck_path",
        };
        match arguments.is_present(tar_arg) {
            true => std::path::PathBuf::from(
                shellexpand::tilde(arguments.value_of(tar_arg).unwrap()).to_string()
            ),
            false => {
                write_default_files(tar_element, arguments.is_present("overwrite_default_files")).expect("couldn't write default files")
            },
        }
    }

    let spread_path = calc_paths(&matches, ElementType::Spread);
    println!("the spread_path is {:?}", spread_path);
    let deck_path = calc_paths(&matches, ElementType::Deck);
    println!("the deck_path is {:?}", deck_path);

    fn calc_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    let mut deck = Deck::new_from_path(deck_path.as_path());
    let spread = Spread::new_from_path(spread_path.as_path());
    let filled_spread = FilledSpread::new(spread, &mut deck, calc_hash(&seed));

    if !matches.is_present("interactive") {
      Pager::new().setup();
    }

    filled_spread.print(
        matches.is_present("interactive"),
        calc_hash(&seed), 
        &mut std::io::stdout()
        );
}
