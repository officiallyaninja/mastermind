use std::collections::HashMap;

use crate::{
    guesser::Feedback,
    pattern::{Color, Pattern},
};
pub struct Game {
    num_colors: u8,
    correct_pattern: Pattern,
}

impl Game {
    pub fn new_random(num_colors: u8, len: u8) -> Self {
        todo!()
    }
    pub fn new(num_colors: u8, pattern: Pattern) -> Self {
        if num_colors == 0 {
            panic!("can't have 0 colors")
        }
        for color in &pattern {
            assert!(**color < num_colors);
        }
        Self {
            num_colors,
            correct_pattern: pattern,
        }
    }
    pub fn evaluate_guess(&self, guess: &Pattern) -> Feedback {
        // idk how to make this less hard coded
        let mut feedback = Feedback::new();
        let mut color_counts = {
            let mut map: HashMap<Color, u8> = HashMap::new();
            for color in &self.correct_pattern {
                *map.entry(*color).or_default() += 1;
            }
            map
        };
        assert_eq!(guess.len(), self.correct_pattern.len());
        for (guessed_color, correct_color) in guess.iter().zip(&self.correct_pattern) {
            // count_left is the count of the guessed_color left in the pattern
            let Some(count_left) = color_counts.get_mut(guessed_color) else {
                // guessed_color is not present in correct pattern at all
                continue;
            };
            if guessed_color == correct_color {
                feedback.exact += 1;
                *count_left -= 1;
            }
        }
        for (guessed_color, correct_color) in guess.iter().zip(&self.correct_pattern) {
            let Some(count_left) = color_counts.get_mut(guessed_color) else {
                // guessed_color is not present in correct pattern at all
                continue;
            };

            if guessed_color == correct_color {
                // Already given feedback about this
                continue;
            }
            if *count_left == 0 {
                // There are no more of this color to give information about
                continue;
            }
            feedback.misplaced += 1;
            *count_left -= 1;
        }
        feedback
    }
}
