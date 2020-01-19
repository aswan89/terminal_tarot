use serde::Deserialize;
use crate::deck::{Card, Deck};
use crate::stored_element::StoredElement;

#[derive(Deserialize, PartialEq, Debug)]
enum PosOrientation {
    Horizontal,
    Vertical,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Spread {
    positions: Vec<Position>,
    position_x_size: u8,
    position_y_size: u8,
    name: String 
}

use std::fmt;
impl fmt::Display for Spread {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl StoredElement for Spread {
    fn new_from_json(json: &str) -> Spread {
        serde_json::from_str(json).unwrap()
    }
}
    

#[derive(PartialEq, Debug)]
pub struct FilledSpread<'a> {
    spread: Spread,
    cards: &'a [Card],
}

impl<'a> FilledSpread<'a> {

    pub fn new(spread: Spread, deck: &mut Deck, seed: u64) -> FilledSpread {
        deck.shuffle_deck(seed);
        let pos_count = spread.positions.len();
        FilledSpread {
            spread: spread,
            cards: deck.draw(pos_count),
        }
    }

    pub fn print(&self, iflag: bool, seed: u64, mut writer: impl std::io::Write) {
        let filled_pos = self.spread.positions.iter().zip(self.cards.iter());
        for pos in filled_pos {
            pos.0.print(&mut writer);
            writeln!(&mut writer, "").unwrap();
            pos.1.print(seed, &mut writer);
            writeln!(&mut writer, "").unwrap();
            writeln!(&mut writer, "{}", std::iter::repeat("=").take(30).collect::<String>()).unwrap();
            writeln!(&mut writer, "").unwrap();
            if iflag {
                writeln!(&mut writer, "Press ENTER to draw next card").unwrap();
                let mut press = String::new();
                std::io::stdin().read_line(&mut press).unwrap();
            }
        }
    }

}

#[derive(Deserialize, PartialEq, Debug)]
struct Position {
    order: u8,
    name: String,
    meaning: String,
    orientation: PosOrientation,
    x_pos: u8,
    y_pos: u8,
}

impl Position {
    fn print(&self, mut writer: impl std::io::Write) {
        writeln!(writer, "{}", self.name).unwrap();
        writeln!(writer, "{}", std::iter::repeat("-").take(self.name.len()).collect::<String>()).unwrap();
        writeln!(writer, "{}", self.meaning).unwrap();
    }
}

mod test_utils;
#[cfg(test)]
mod tests {
    use crate::spread::test_utils::{
        gen_test_spread,
        gen_test_spread_json
    };
    use crate::spread::{
        FilledSpread,
        Spread
    };
    use crate::deck::Deck;
    use crate::deck::test_utils::{
        return_test_deck
    };
    use crate::stored_element::StoredElement;
    #[test]
    fn position_print() {
        let mut test_result = Vec::new();
        let test_spread = gen_test_spread();

        let target_output = r#"test position 1
---------------
test meaning 1
"#;
        test_spread.positions[0].print(&mut test_result);
        let test_output = String::from_utf8(test_result).unwrap();
        assert_eq!(test_output, target_output);

    }

    #[test]
    fn filled_spread_print() {
        let mut test_result = Vec::new();
        let test_spread = gen_test_spread();
        let mut test_deck = Deck::new_from_json(&return_test_deck());
        let test_filled_spread = FilledSpread::new(
            test_spread,
            &mut test_deck,
            1
        );
        let target_output = 
r#"test position 1
---------------
test meaning 1

test_name1
----------
test_fortune1
Light: light_meaning1
Shadow: shadow_meaning1

==============================

test position 2
---------------
test meaning 2

test_name2
----------
test_fortune2
Light: light_meaning2
Shadow: shadow_meaning2

==============================

"#;

        test_filled_spread.print(false, 1, &mut test_result);
        let test_output = String::from_utf8(test_result).unwrap();

        assert_eq!(test_output, target_output);

    }

    #[test]
    fn spread_deserial() {
        let test_spread: Spread = serde_json::from_str(&gen_test_spread_json()).unwrap();
        assert_eq!(gen_test_spread(), test_spread);
    }

    #[test]
    fn construct_filled_spread() {
        let test_spread = gen_test_spread();
        let mut test_deck = Deck::new_from_json(&return_test_deck());
        let mut ref_deck = Deck::new_from_json(&return_test_deck());
        ref_deck.shuffle_deck(1);
        
        let manual_filled_spread: FilledSpread = FilledSpread {
            spread: gen_test_spread(),
            cards: ref_deck.draw(2),
        };

        assert_eq!(
            manual_filled_spread,
            FilledSpread::new(
                test_spread,
                &mut test_deck,
                1
            )
       );
    }

}
