use std::fs;

// Estructura que representa un informe
struct Report {
    levels: Vec<i32>,
}

impl Report {
    // Verificar si los niveles son monótonos
    fn is_monotonic(&self) -> bool {
        let increasing = self.levels.windows(2).all(|w| w[1] >= w[0]);
        let decreasing = self.levels.windows(2).all(|w| w[1] <= w[0]);
        increasing || decreasing
    }

    // Verificar si las diferencias entre niveles están en el rango permitido
    fn differences_valid(&self) -> bool {
        self.levels.windows(2).all(|w| {
            let diff = (w[1] - w[0]).abs();
            diff >= 1 && diff <= 3
        })
    }

    // Comprobar si el informe es seguro
    fn is_safe(&self) -> bool {
        self.is_monotonic() && self.differences_valid()
    }
    
    // Comprobar si el informe puede ser seguro eliminando un nivel
    fn is_safe_with_one_removal(&self) -> bool {
        for i in 0..self.levels.len() {
            let mut modified_levels = self.levels.clone();
            modified_levels.remove(i); // Eliminar el nivel en la posición `i`
            let modified_report = Report {
                levels: modified_levels,
            };
            if modified_report.is_safe() {
                return true; // Si alguna eliminación resulta en un informe seguro
            }
        }
        false // Ninguna eliminación lo hizo seguro
    }
}

fn main() {
    // Ruta al archivo de entrada
    let file_path = "input.txt";

    // Leer el contenido del archivo
    let input = fs::read_to_string(file_path)
        .expect("No se pudo leer el archivo");

    // Parsear los datos de entrada
    let reports: Vec<Report> = input
        .lines()
        .filter(|line| !line.trim().is_empty()) // Ignorar líneas vacías
        .map(|line| {
            let levels = line.split_whitespace() // Dividir por espacios
                .map(|num| num.parse::<i32>().unwrap()) // Convertir a enteros
                .collect();
            Report { levels }
        })
        .collect();

    // Contar informes seguros
    let safe_count = reports.iter().filter(|report| report.is_safe()).count();

    println!("Número de informes seguros (Primera Parte) : {}", safe_count);
    
    // Contar informes seguros
    let safe_count_one_remove = reports
        .iter()
        .filter(|report| report.is_safe() || report.is_safe_with_one_removal())
        .count();

    println!("Número de informes seguros con el Amortiguador de Problemas: {}", safe_count_one_remove);
}
