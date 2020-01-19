use crate::deck::{
    Card,
    Meaning
};
pub fn return_test_card() -> Card {
    let manual_meanings = Meaning {
        light: vec![ 
            String::from("light_meaning"),
            String::from("light_meaning2") 
        ],
        shadow: vec![ 
            String::from("shadow_meaning"),
            String::from("shadow_meaning2")
        ],
    };

    Card {
        rank: 0,
        suit: String::from("test_suit"),
        name: String::from("test_name"),
        meanings: manual_meanings,
        keywords: vec![
            String::from("test_keyword"),
            String::from("test_keyword1")
        ],
        fortune_telling: vec![
            String::from("test_fortune"),
            String::from("test_fortune1")
        ],
    }
}

pub fn return_test_deck() -> String {
    String::from(r#"
    {
        "name": "test deck",
        "cards": 
        [
          {
            "rank": 0,
            "suit": "test_suit",
            "name": "test_name",
            "meanings": {
              "light": [
                "light_meaning",
                "light_meaning2"
              ],
              "shadow": [
                "shadow_meaning",
                "shadow_meaning2"
              ]
            },
            "keywords": [
              "test_keyword",
              "test_keyword1"
            ],
            "fortune_telling": [
              "test_fortune",
              "test_fortune1"
            ]
          },
          {
            "rank": 1,
            "suit": "test_suit1",
            "name": "test_name1",
            "meanings": {
              "light": [
                "light_meaning1"
              ],
              "shadow": [
                "shadow_meaning1"
              ]
            },
            "keywords": [
              "test_keyword1"
            ],
            "fortune_telling": [
              "test_fortune1"
            ]
          }, 
          {
            "rank": 2,
            "suit": "test_suit2",
            "name": "test_name2",
            "meanings": {
              "light": [
                "light_meaning2"
              ],
              "shadow": [
                "shadow_meaning2"
              ]
            },
            "keywords": [
              "test_keyword2"
            ],
            "fortune_telling": [
              "test_fortune2"
            ]
          } 
        ]
    }
    "#)
}
