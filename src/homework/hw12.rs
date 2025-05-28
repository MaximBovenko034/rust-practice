pub fn count_permutation(shipments: &Vec<u32>) -> isize {
    let n = shipments.len();
    if n == 0 {
        return 0;
    }

    let total: u32 = shipments.iter().sum();

    // Перевірка, чи можливо рівномірно розподілити вантаж
    if total as usize % n != 0 {
        return -1;
    }

    let average = total / n as u32;

    // Підрахунок мінімальної кількості пересувань
    // Сума "надлишків" (товару, який потрібно перенести з кораблів, що мають більше ніж average)
    let mut moves = 0;

    for &w in shipments.iter() {
        if w > average {
            moves += (w - average) as usize;
        }
    }

    moves as isize
}

/// Генерує вектор довжини n, який можна рівномірно розподілити
pub fn gen_shipments(n: usize) -> Vec<u32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Спершу обираємо середнє значення (наприклад від 1 до 100)
    let average = rng.gen_range(1..=100);

    // Створимо вектор із усіма значеннями average
    let mut shipments = vec![average; n];

    // Трохи випадково змінимо пару значень, але так, щоб сума лишилась незмінною
    if n > 1 {
        // Наприклад, збільшимо перший на k, а другий зменшимо на k
        let k = rng.gen_range(0..average.min(average)); // це для прикладу, можна будь-яке k <= average

        shipments[0] += k;
        shipments[1] -= k;
    }

    shipments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_permutation() {
        let data = vec![8, 2, 2, 4, 4];
        assert_eq!(count_permutation(&data), 4);

        let data2 = vec![9, 3, 7, 2, 9];
        assert_eq!(count_permutation(&data2), 7);

        let data3 = vec![1, 2, 3];
        assert_eq!(count_permutation(&data3), -1); // неможливо

        let data4 = vec![4, 4, 4, 4];
        assert_eq!(count_permutation(&data4), 0); // вже рівномірно
    }

    #[test]
    fn test_gen_shipments() {
        for n in 1..10 {
            let shipments = gen_shipments(n);
            let result = count_permutation(&shipments);
            assert!(result >= 0, "Generated vector should be evenly distributable");
        }
    }
}

