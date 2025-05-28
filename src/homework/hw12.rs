use rand::Rng;

pub fn count_permutation(shipments: &Vec<u32>) -> isize {
    let n = shipments.len();
    if n == 0 {
        return 0;
    }

    let total: u32 = shipments.iter().sum();

    if total as usize % n != 0 {
        return -1;
    }

    let average = total / n as u32;

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
    let mut rng = rand::thread_rng();

    let average = rng.gen_range(1..=100);

    let mut shipments = vec![average; n];

    if n > 1 {
        let k = rng.gen_range(0..=average);
        shipments[0] += k;
        shipments[1] = shipments[1].saturating_sub(k);
    }

    shipments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_permutation() {
        assert_eq!(count_permutation(&vec![8, 2, 2, 4, 4]), 4);
        assert_eq!(count_permutation(&vec![9, 3, 7, 2, 9]), 7);
        assert_eq!(count_permutation(&vec![1, 2, 3]), -1);
        assert_eq!(count_permutation(&vec![4, 4, 4, 4]), 0);
    }

    #[test]
    fn test_gen_shipments() {
        for n in 1..20 {
            let shipments = gen_shipments(n);
            let result = count_permutation(&shipments);
            assert!(result >= 0, "Generated vector should be evenly distributable");
        }
    }
}
