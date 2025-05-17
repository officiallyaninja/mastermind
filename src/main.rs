use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Color(u8);

impl Deref for Color {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

type Pattern = Vec<Color>;

#[derive(Default)]
pub struct Feedback {
    pub exact: u8,
    pub misplaced: u8,
}
impl Feedback {
    fn new() -> Self {
        Self::default()
    }
}

pub struct Game {
    num_colors: u8,
    correct_pattern: Pattern,
}

impl Game {
    pub fn new_random(num_colors: u8, len: u8) -> Self {
        todo!()
    }
    pub fn new(num_colors: u8, pattern: Pattern) -> Self {
        for color in &pattern {
            assert!(**color < num_colors);
        }
        Self {
            num_colors,
            correct_pattern: pattern,
        }
    }
    pub fn evaluate_guess(&self, guess: Pattern) -> Feedback {
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

pub trait Guesser {
    fn guess(&self) -> Pattern;
}

fn main() {
    let game = Game::new(
        3,
        vec![0, 1, 2, 1, 2, 2]
            .into_iter()
            .map(|num: u8| Color(num))
            .collect(),
    );
}

mod test {

    #[rustfmt::skip]
    // 3x2 Feedback table
    const feedback_table: [[(u8, u8); 9]; 9]= [
        [(0, 2), (0, 1), (0, 1), (0, 1), (0, 0), (0, 0), (0, 1), (0, 0), (0, 0)], // RR
        [(0, 1), (0, 2), (0, 1), (2, 0), (0, 1), (1, 0), (1, 0), (0, 1), (0, 0)], // RG
        [(0, 1), (0, 1), (0, 2), (1, 0), (0, 0), (0, 1), (2, 0), (1, 0), (0, 1)], // RB
        [(0, 1), (2, 0), (1, 0), (0, 2), (0, 1), (0, 1), (0, 1), (1, 0), (0, 0)], // GR
        [(0, 0), (0, 1), (0, 0), (0, 1), (0, 2), (0, 1), (0, 0), (0, 1), (0, 0)], // GG
        [(0, 0), (1, 0), (0, 1), (0, 1), (0, 1), (0, 2), (0, 0), (1, 0), (0, 1)], // GB
        [(0, 1), (1, 0), (2, 0), (0, 1), (0, 0), (0, 0), (0, 2), (0, 1), (0, 1)], // BR
        [(0, 0), (0, 1), (1, 0), (1, 0), (0, 1), (1, 0), (0, 1), (0, 2), (0, 1)], // BG
        [(0, 0), (0, 0), (0, 1), (0, 0), (0, 0), (0, 1), (0, 1), (0, 1), (0, 2)], // BB
    ];
}
