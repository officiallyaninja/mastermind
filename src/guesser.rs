use crate::{
    game::Game,
    pattern::{self, Color, Pattern},
};

#[derive(Default)]
pub struct Feedback {
    pub exact: u8,
    pub misplaced: u8,
}
impl Feedback {
    pub fn new() -> Self {
        Self::default()
    }
}

pub trait Guesser {
    fn guess(&self) -> Pattern;
}

struct Matrix<T> {
    rows: usize,
    columns: usize,
    elems: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn get(&self, i: usize, j: usize) -> Option<&T> {
        if i >= self.rows || j >= self.columns {
            return None;
        }
        self.elems.get(self.columns * i + j)
    }

    pub fn from_vec(rows: usize, columns: usize, init: Vec<T>) -> Self {
        Self {
            rows,
            columns,
            elems: init,
        }
    }
}

fn all_patterns(num_colors: u8, len: usize) -> Vec<Pattern> {
    let mut pattern = vec![0; len];
    let mut patterns = Vec::new();
    let patterns = 'main: loop {
        patterns.push(pattern.clone());
        for color in &mut pattern {
            *color += 1;
            if *color < num_colors {
                continue 'main;
            }
            *color = 0;
        }
        break patterns;
    };
    patterns
        .into_iter()
        .map(|p| p.into_iter().map(Color).collect())
        .collect()
}

fn feedback_matrix(patterns: &[Pattern], num_colors: u8, len: usize) -> Matrix<Feedback> {
    let mut feedback_vec: Vec<Feedback> = Vec::new();
    for correct_pattern in patterns {
        let game = Game::new(num_colors, correct_pattern.clone());
        for guessed_pattern in patterns {
            let feedback = game.evaluate_guess(guessed_pattern);
            feedback_vec.push(feedback);
        }
    }
    Matrix::from_vec(len, len, feedback_vec)
}
struct Unoptimized {
    num_colors: u8,
    pattern_len: usize,
    feedback_matrix: Matrix<Feedback>,
    possible_patterns: Vec<Pattern>,
}

impl Unoptimized {
    pub fn new(num_colors: u8, pattern_len: usize) -> Self {
        let possible_patterns = all_patterns(num_colors, pattern_len);
        let feedback_matrix = feedback_matrix(&possible_patterns, num_colors, pattern_len);
        Self {
            num_colors,
            pattern_len,
            possible_patterns,
            feedback_matrix,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::guesser::{all_patterns, feedback_matrix};

    #[test]
    fn test_all_patterns() {
        for c in 1..=6 {
            for n in 1..=4 {
                let patterns_len = all_patterns(c, n).len();
                let expected_len = (c as usize).pow(n as u32);
                assert_eq!(patterns_len, expected_len);
            }
        }
        println!(
            "{:?}",
            all_patterns(3, 2)
                .into_iter()
                .map(|p| p.into_iter().map(|c| *c).collect())
                .collect::<Vec<Vec<u8>>>()
        );
    }
    #[test]
    fn test_small_feedback_matrix() {
        // 3x2 Feedback table
        let feedback_table: Vec<(u8, u8)> = vec![
            //     RR      RG      GR      GG
            vec![(0, 2), (0, 1), (0, 1), (0, 0)], // RR
            vec![(0, 1), (0, 2), (2, 0), (0, 1)], // RG
            vec![(0, 1), (2, 0), (0, 2), (0, 1)], // GR
            vec![(0, 0), (0, 1), (0, 1), (0, 2)], // GG
        ]
        .into_iter()
        .flatten()
        .collect();
        let matrix: Vec<(u8, u8)> = feedback_matrix(&all_patterns(2, 2), 2, 2)
            .elems
            .into_iter()
            .map(|f| (f.misplaced, f.exact))
            .collect();
        assert_eq!(feedback_table.len(), matrix.len());
        for (i, (a, b)) in feedback_table.into_iter().zip(matrix).enumerate() {
            assert_eq!(a, b, "{i}")
        }
    }
}
