mod word_search;

use word_search::WordSearch;
use std::env;
use std::process;

fn main() {
    // Leer el nombre del archivo y la opción desde los argumentos de línea de comandos
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Uso: {} <archivo_entrada> <opción> [<palabra_objetivo>]", args[0]);
        eprintln!("Opciones disponibles:");
        eprintln!("1 - Buscar ocurrencias de una palabra.");
        eprintln!("2 - Buscar el patrón X-MAS.");
        eprintln!("3 - Buscar el patrón X-MAS en forma de X.");
        process::exit(1);
    }

    let input_file = &args[1];
    let option: usize = args[2].parse().unwrap_or(0);

    // Crear WordSearch desde el archivo
    let word_search = match WordSearch::from_file(input_file) {
        Ok(ws) => ws,
        Err(err) => {
            eprintln!("Error al leer el archivo '{}': {}", input_file, err);
            process::exit(1);
        }
    };

    match option {
        1 => {
            // Buscar una palabra específica
            if args.len() < 4 {
                eprintln!("Debe proporcionar una palabra objetivo para la opción 1.");
                process::exit(1);
            }
            let target_word = &args[3];
            let count = word_search.count_word(target_word);
            println!("El total de ocurrencias de '{}' es: {}", target_word, count);
        }
        2 => {
            // Buscar el patrón X-MAS
            let count = word_search.count_xmas_pattern();
            println!("El total de ocurrencias del patrón X-MAS es: {}", count);
        }
        3 => {
            // Buscar el patrón X-MAS en forma de X
            let count = word_search.count_x_mas();
            println!("El total de ocurrencias del patrón X-MAS en forma de X es: {}", count);
        }
        _ => {
            eprintln!("Opción no válida. Use 1, 2 o 3.");
            process::exit(1);
        }
    }
}
