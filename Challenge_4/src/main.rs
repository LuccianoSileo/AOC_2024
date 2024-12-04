mod word_search;

use word_search::WordSearch;
use std::env;
use std::process;

fn main() {
    // Leer el nombre del archivo desde los argumentos de línea de comandos
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Uso: {} <archivo_entrada> <palabra_objetivo>", args[0]);
        process::exit(1);
    }

    let input_file = &args[1];
    let target_word = &args[2];

    // Crear WordSearch desde el archivo
    let word_search = match WordSearch::from_file(input_file) {
        Ok(ws) => ws,
        Err(err) => {
            eprintln!("Error al leer el archivo '{}': {}", input_file, err);
            process::exit(1);
        }
    };

    // Contar las ocurrencias de la palabra objetivo
    let count = word_search.count_word(target_word);
    println!("El total de ocurrencias de '{}' es: {}", target_word, count);
}
