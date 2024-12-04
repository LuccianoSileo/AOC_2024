mod parser;
mod calculator;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

fn main() -> io::Result<()> {
    let filename = "input3.txt";
    let lines = read_lines(filename)?;

    let parser = parser::MemoryParser::new();
    let mut total_sum = 0;

    for line in lines {
        println!("Procesando línea: {}", line);
        let instructions = parser.parse(&line);
        println!("Instrucciones parseadas: {:?}", instructions);
        let line_sum = calculator::calculate_sum(&instructions);
        println!("Suma de la línea: {}", line_sum);
        total_sum += line_sum;
    }

    println!("La suma total de las multiplicaciones válidas es: {}", total_sum);

    Ok(())
}
