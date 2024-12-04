use regex::Regex;
use std::fs;

fn extract_multiplications_from_line(line: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Regex inválido");
    re.captures_iter(line)
        .filter_map(|cap| {
            let num1 = cap[1].parse::<i32>().ok()?;
            let num2 = cap[2].parse::<i32>().ok()?;
            Some((num1, num2))
        })
        .collect()
}

fn sum_multiplications(filename: &str) -> i32 {
    let contents = fs::read_to_string(filename).expect("Error al leer el archivo");
    let mut total_sum = 0;

    for line in contents.lines() {
        let results = extract_multiplications_from_line(line);
        for (num1, num2) in results {
            total_sum += num1 * num2;
        }
    }

    total_sum
}

/// Calcula la suma total de las multiplicaciones válidas considerando `do()` y `don't()`.

fn sum_conditioned_multiplications(filename: &str) -> i32 {
    // Leer el contenido del archivo
    let contents = fs::read_to_string(filename).expect("Error al leer el archivo");

    // Expresiones regulares para capturar los patrones
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").expect("Regex inválido");
    let re_do = Regex::new(r"do\(\)").expect("Regex inválido");
    let re_dont = Regex::new(r"don't\(\)").expect("Regex inválido");

    let mut total_sum = 0;
    let mut is_enabled = true; // Las multiplicaciones están habilitadas por defecto.

    // Procesar cada línea del archivo
    for mut line in contents.lines() {
        // Mientras haya coincidencias de `mul()`, `do()` o `don't()`, procesamos paso por paso
        while !line.is_empty() {
            // Primero verificamos `do()`, `don't()`, y luego `mul()`
            if let Some(cap) = re_mul.captures(line) {
                // Encontramos "mul()", verificamos si está habilitado
                let num1 = cap[1].parse::<i32>().expect("Número inválido");
                let num2 = cap[2].parse::<i32>().expect("Número inválido");

                // Si las multiplicaciones están habilitadas, realizamos la multiplicación
                if is_enabled {
                    println!("Multiplicando {} * {}", num1, num2);
                    total_sum += num1 * num2;
                }

                // Eliminamos el patrón "mul()" procesado
                line = &line[cap.get(0).unwrap().end()..]; // Eliminamos "mul(X, Y)"
            }
            
            if let Some(mat) = re_do.find(line) {
                // Encontramos "do()", habilitar las multiplicaciones
                is_enabled = true;
                line = &line[mat.end()..]; // Eliminamos "do()"
                continue;
            }

            if let Some(mat) = re_dont.find(line) {
                // Encontramos "don't()", deshabilitar las multiplicaciones
                is_enabled = false;
                line = &line[mat.end()..]; // Eliminamos "don't()"
                continue;
            } else {
                // Si no hay más coincidencias de "mul()", "do()" o "don't()", detenemos el bucle
                break;
            }
        }
    }

    total_sum
}






fn main() {
    let filename = "input2.txt";
    let total_sum = sum_multiplications(filename);
    println!("La suma total de las multiplicaciones es: {}", total_sum);

        // Parte 2: Suma condicionada por do() y don't()
    let total_sum_part2 = sum_conditioned_multiplications(filename);
    println!(
        "Parte 2: La suma total de las multiplicaciones habilitadas es: {}",
        total_sum_part2
    );
}
