git add hw03.rs
git commit -m "Додав код для малювання конверта"
git push

// hw04.rs

fn main() {
    const WIDTH: usize = 9;  // ширина ромба
    const HEIGHT: usize = 9; // висота ромба

    // Верхня частина ромба
    for i in 0..HEIGHT / 2 {
        for _ in 0..(HEIGHT / 2 - i) {
            print!(" "); // Пробіли перед зірочками
        }
        for _ in 0..(2 * i + 1) {
            print!("*"); // Малюємо зірочки
        }
        println!(); // Переходимо на новий рядок
    }

    // Нижня частина ромба
    for i in (0..HEIGHT / 2).rev() {
        for _ in 0..(HEIGHT / 2 - i) {
            print!(" "); // Пробіли перед зірочками
        }
        for _ in 0..(2 * i + 1) {
            print!("*"); // Малюємо зірочки
        }
        println!(); // Переходимо на новий рядок
    }
}
