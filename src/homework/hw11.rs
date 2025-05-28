use rand::Rng;

pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

/// Повертає кортеж: (індекс першого елемента пари, сума пари)
pub fn min_adjacent_sum(data: &[i32]) -> Option<(usize, i32)> {
    if data.len() < 2 {
        return None;
    }
    let mut min_index = 0;
    let mut min_sum = data[0] + data[1];

    for i in 1..data.len() - 1 {
        let s = data[i] + data[i + 1];
        if s < min_sum {
            min_sum = s;
            min_index = i;
        }
    }

    Some((min_index, min_sum))
}

pub fn print_vector_with_min_pair(data: &[i32]) {
    let len = data.len();
    if len == 0 {
        println!("Empty vector");
        return;
    }

    // Друкуємо індекси
    print!("indexes:");
    for i in 0..len {
        print!(" {:2}. ", i);
    }
    println!();

    // Друкуємо дані
    print!("data:   ");
    for &v in data {
        print!("[{:2}], ", v);
    }
    println!();

    if let Some((min_idx, min_sum)) = min_adjacent_sum(data) {
        // Виводимо лінії під індексами
        print!("indexes: ");
        for i in 0..len {
            if i == min_idx {
                print!(" \\__ ");
            } else if i == min_idx + 1 {
                print!(" __/ ");
            } else {
                print!("     ");
            }
        }
        println!();

        println!(
            "min adjacent sum={}+{}={} at indexes:{},{}",
            data[min_idx], data[min_idx + 1], min_sum, min_idx, min_idx + 1
        );
    } else {
        println!("Not enough elements for adjacent sum");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_adjacent_sum() {
        let data = vec![45, 87, 49, 64, 50, 37, 45, 72, 55, 64, 90, 86, 60, 54, 78, 72, 83, 44, 89, 22];
        let res = min_adjacent_sum(&data);
        assert_eq!(res, Some((5, 82)));
    }
}

