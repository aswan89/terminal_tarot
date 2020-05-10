use serde::Deserialize;
use crate::stored_element::StoredElement;

#[derive(Deserialize, PartialEq, Debug)]
struct CardGraphic {
    graph_string: String,
}

#[derive(Deserialize, PartialEq, Debug)]
struct Meaning {
    light: Vec<String>,
    shadow: Vec<String>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Card {
    rank: u8,
    suit: String,
    name: String,
    meanings: Meaning,
    keywords: Vec<String>,
    fortune_telling: Vec<String>,
}

impl Card {
    pub fn print(&self, seed: u64, mut writer: impl std::io::Write) {
        use rand::SeedableRng;
        use rand::rngs;
        use rand::seq::SliceRandom;

        let mut rng = rngs::StdRng::seed_from_u64(seed);
        writeln!(writer, "{}", self.name).unwrap();
        writeln!(writer, "{}", std::iter::repeat("-").take(self.name.len()).collect::<String>()).unwrap();
        writeln!(writer, "{}", self.fortune_telling.choose(&mut rng).unwrap_or(&String::from("No Fortune"))).unwrap();
        writeln!(writer, "Light: {}", self.meanings.light.choose(&mut rng).unwrap_or(&String::from("No Light meaning"))).unwrap();
        writeln!(writer, "Shadow: {}", self.meanings.shadow.choose(&mut rng).unwrap_or(&String::from("No Shadow meaning"))).unwrap();
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Deck {
    cards: Vec<Card>,
    pub name: String
}

use std::fmt;
impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl StoredElement for Deck {
    fn new_from_json(json: &str) -> Deck {
        serde_json::from_str(json).unwrap()
    } 
}
impl Deck {
    pub fn shuffle_deck(&mut self, seed: u64) {
        use rand::SeedableRng;
        use rand::rngs;
        use rand::seq::SliceRandom;

        let mut rng = rngs::StdRng::seed_from_u64(seed);
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&self, count: usize) -> &[Card] {
        if self.cards.len() < count {
            panic!("Attempted to draw more cards than are in deck.");
        }
        &self.cards[0..count]
    }
}

pub mod test_utils;
#[cfg(test)]
mod tests {
    use crate::stored_element::StoredElement;
    use crate::deck::test_utils::utils::{
        return_test_deck,
        return_test_card
    };
    use crate::deck::{
        Deck
    };

    #[test]
    fn display_card() {
        let mut test_result = Vec::new();
        let test_card = return_test_card();
        let target_output = r#"test_name
---------
test_fortune1
Light: light_meaning2
Shadow: shadow_meaning
"#;
        test_card.print(1, &mut test_result);
        let test_output = String::from_utf8(test_result).unwrap();

        assert_eq!(
            test_output,
            target_output
            )
    }

    #[test]
    fn deck_deserial() {
        let mut test_deck = Deck::new_from_json(&return_test_deck());
        test_deck.cards.truncate(1);

        let manual_deck = Deck {
            name: "test deck".to_string(),
            cards: vec![ return_test_card() ],
        };

        assert_eq!(manual_deck, test_deck);
    }

    #[test]
    fn deck_draw() {
        let test_deck = Deck::new_from_json(&return_test_deck());
        assert_eq!(test_deck.draw(1).len(), 1);
        assert_eq!(test_deck.draw(2).len(), 2);
    }

    #[test]
    #[should_panic]
    fn deck_draw_too_many() {
        let test_deck = Deck::new_from_json(&return_test_deck());
        test_deck.draw(100);
    }

    #[test]
    fn deck_repeatable_draw() {
        let mut test_deck = Deck::new_from_json(&return_test_deck());
        let mut test_deck2 = Deck::new_from_json(&return_test_deck());
        test_deck.shuffle_deck(1);
        test_deck2.shuffle_deck(1);
        assert_eq!(test_deck.draw(2), test_deck2.draw(2));
    }

    #[test]
    fn deck_shuffled_draw() {
        let mut test_deck = Deck::new_from_json(&return_test_deck());
        test_deck.shuffle_deck(1);
        let mut test_deck2 = Deck::new_from_json(&return_test_deck());
        test_deck2.shuffle_deck(2);
        let first_draw = test_deck.draw(3);
        let second_draw = test_deck2.draw(3);
        assert_ne!(first_draw, second_draw);
    }


}



