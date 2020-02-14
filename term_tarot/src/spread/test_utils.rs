#[cfg(test)]
pub mod utils {
use crate::spread::{
    Position,
    Spread,
    PosOrientation
};
pub fn gen_test_spread_json() -> String {
    String::from(r#"
      {
          "name": "test spread",
          "position_x_size": 5,
          "position_y_size": 7,
          "positions": [
              {
                  "order": 1,
                  "name": "test position 1",
                  "meaning": "test meaning 1",
                  "orientation": "Horizontal",
                  "x_pos": 0,
                  "y_pos": 0
              },
              {
                  "order": 2,
                  "name": "test position 2",
                  "meaning": "test meaning 2",
                  "orientation": "Vertical",
                  "x_pos": 5,
                  "y_pos": 5 
              }
           ]
       }
    "#)
}

pub fn gen_test_spread() -> Spread {
    let pos1 = Position {
        order: 1,
        name: "test position 1".to_string(),
        meaning: "test meaning 1".to_string(),
        orientation: PosOrientation::Horizontal,
        x_pos: 0,
        y_pos: 0,
    };
    let pos2 = Position {
        order: 2,
        name: "test position 2".to_string(),
        meaning: "test meaning 2".to_string(),
        orientation: PosOrientation::Vertical,
        x_pos: 5,
        y_pos: 5,
    };
    Spread {
        positions: vec![pos1, pos2],
        position_x_size: 5,
        position_y_size: 7,
        name: "test spread".to_string()
    }
}
}
