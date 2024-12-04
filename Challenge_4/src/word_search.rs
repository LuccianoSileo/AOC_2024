use std::fs;
use std::io::{self, BufRead};

pub struct WordSearch {
    puzzle: Vec<String>,
    directions: Vec<(isize, isize)>,
}

impl WordSearch {
    /// Constructor para inicializar desde un vector de líneas
    pub fn new(puzzle: Vec<String>) -> Self {
        Self {
            puzzle,
            directions: vec![
                (0, 1),   // derecha
                (1, 0),   // abajo
                (0, -1),  // izquierda
                (-1, 0),  // arriba
                (1, 1),   // diagonal abajo-derecha
                (1, -1),  // diagonal abajo-izquierda
                (-1, 1),  // diagonal arriba-derecha
                (-1, -1), // diagonal arriba-izquierda
            ],
        }
    }

    /// Constructor para inicializar desde un archivo
    pub fn from_file(file_path: &str) -> Result<Self, io::Error> {
        let file = fs::File::open(file_path)?;
        let reader = io::BufReader::new(file);

        let puzzle = reader
            .lines()
            .filter_map(|line| line.ok()) // Ignorar líneas con errores
            .collect();

        Ok(Self::new(puzzle))
    }

    /// Contar las ocurrencias de una palabra
    pub fn count_word(&self, word: &str) -> usize {
        let rows = self.puzzle.len();
        let cols = self.puzzle[0].len();
        let mut count = 0;

        for row in 0..rows {
            for col in 0..cols {
                for &(dx, dy) in &self.directions {
                    if self.check_word(word, row as isize, col as isize, dx, dy) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    /// Verificar si una palabra está en una dirección específica
    fn check_word(
        &self,
        word: &str,
        start_row: isize,
        start_col: isize,
        dx: isize,
        dy: isize,
    ) -> bool {
        let rows = self.puzzle.len() as isize;
        let cols = self.puzzle[0].len() as isize;
        let chars: Vec<char> = word.chars().collect();

        for (i, &c) in chars.iter().enumerate() {
            let new_row = start_row + i as isize * dx;
            let new_col = start_col + i as isize * dy;

            if new_row < 0 || new_col < 0 || new_row >= rows || new_col >= cols {
                return false;
            }

            if self.puzzle[new_row as usize].chars().nth(new_col as usize) != Some(c) {
                return false;
            }
        }

        true
    }
}
