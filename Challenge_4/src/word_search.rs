use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct WordSearch {
    puzzle: Vec<String>,
}

impl WordSearch {
    // Crear WordSearch desde un archivo
    pub fn from_file(filename: &str) -> Result<Self, io::Error> {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
        let puzzle: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
        Ok(WordSearch { puzzle })
    }

    // Contar las ocurrencias de un patrón específico "X-MAS"
    pub fn count_word(&self, word: &str) -> usize {
        let mut count = 0;
        let rows = self.puzzle.len();
        let cols = self.puzzle[0].len();

        // Buscar la palabra en todas direcciones
        for row in 0..rows {
            for col in 0..cols {
                if self.check_word(row, col, word) {
                    count += 1;
                }
            }
        }
        count
    }

    // Verificar si una palabra está en la posición dada
    fn check_word(&self, row: usize, col: usize, word: &str) -> bool {
        // Aquí deberías implementar la lógica para buscar la palabra en las direcciones posibles
        // Ejemplo: horizontal, vertical, diagonal...
        false
    }

    // Contar el patrón X-MAS
    pub fn count_xmas_pattern(&self) -> usize {
        let mut count = 0;
        let rows = self.puzzle.len();
        let cols = self.puzzle[0].len();

        for row in 0..rows {
            for col in 0..cols {
                if self.puzzle[row].chars().nth(col) == Some('X') {
                    if row + 3 < rows && col + 3 < cols {
                        if self.puzzle[row+1].chars().nth(col+1) == Some('M')
                            && self.puzzle[row+2].chars().nth(col+2) == Some('A')
                            && self.puzzle[row+3].chars().nth(col+3) == Some('S') {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    // Buscar el patrón X-MAS en forma de X
    pub fn count_x_mas(&self) -> usize {
        let mut count = 0;
        let rows = self.puzzle.len();
        let cols = self.puzzle[0].len();

        // Los patrones codificados para formar "X-MAS"
        let patterns = [
            ['M', 'S', 'M', 'S'],
            ['S', 'M', 'S', 'M'],
            ['S', 'S', 'M', 'M'],
            ['M', 'M', 'S', 'S'],
        ];

        // Recorrer todas las posiciones válidas para el centro de la X
        for row in 1..rows - 1 {
            for col in 1..cols - 1 {
                if self.puzzle[row].chars().nth(col) == Some('A') {
                    for pattern in &patterns {
                        if self.puzzle[row - 1].chars().nth(col - 1) == Some(pattern[0])
                            && self.puzzle[row - 1].chars().nth(col + 1) == Some(pattern[1])
                            && self.puzzle[row + 1].chars().nth(col - 1) == Some(pattern[2])
                            && self.puzzle[row + 1].chars().nth(col + 1) == Some(pattern[3])
                        {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }
}
