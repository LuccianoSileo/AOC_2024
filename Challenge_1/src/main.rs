use std::fs::File;
use std::io::{self, BufRead};
use rayon::prelude::*;
use std::collections::HashMap;

// use std::path::Path;

fn read_create_vector(route: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let mut vector1 = Vec::new();
    let mut vector2 = Vec::new();

    // Abrir archivo en modo lectura
    let file = File::open(route)?;
    let reader = io::BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let elements: Vec<&str> = line.split_whitespace().collect();

        if elements.len() == 2 {
            // Intentar parsear los dos valores como i32
            match (elements[0].parse::<i32>(), elements[1].parse::<i32>()) {
                (Ok(valor1), Ok(valor2)) => {
                    vector1.push(valor1);
                    vector2.push(valor2);
                }
                _ => {
                    eprintln!("Error al parsear números en la línea {}: {}", index + 1, line);
                }
            }
        } else {
            eprintln!(
                "Formato incorrecto en la línea {}: '{}'. Se esperaban dos columnas.",
                index + 1,
                line
            );
        }
    }

    Ok((vector1, vector2))
}

fn total_distance(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
    left.sort();
    right.sort();
    left.par_iter()
        .zip(right.par_iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn similarity_score(left: Vec<i32>, right: Vec<i32>) -> i32 {
    // Crear un HashMap para contar las ocurrencias en la lista derecha
    let mut count_map = HashMap::new();
    for &num in &right {
        *count_map.entry(num).or_insert(0) += 1;
    }

    // Calcular el puntaje de similitud
    let mut score = 0;
    for &num in &left {
        if let Some(&count) = count_map.get(&num) {
            score += num * count;
        }
    }

    score
}

fn main() {
    let route_file = "input.txt";

    let (mut vector1, mut vector2) = match read_create_vector(route_file) {
        Ok(vectors) => vectors,
        Err(error) => {
            eprintln!("Error al leer el archivo: {}", error);
            return;
        }
    };

    // println!("Antes de ordenar:");
    // println!("Primer vector: {:?}", vector1);
    // println!("Segundo vector: {:?}", vector2);

    // vector1.sort();
    // vector2.sort();

    // println!("Después de ordenar:");
    // println!("Primer vector: {:?}", vector1);
    // println!("Segundo vector: {:?}", vector2);

    let result = total_distance(&mut vector1, &mut vector2);
    
    println!("Resultado: {:?}", result);

    let score = similarity_score(vector1, vector2);
    println!("Puntaje de similitud: {}", score);


}

