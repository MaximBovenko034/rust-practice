fn draw_tree(levels: usize) {
    (1..=levels).for_each(|level| {
        (1..=level).for_each(|line| {
            let stars = 2 * line - 1;
            let padding = levels + 1 - line;
            let row = " ".repeat(padding) + &"*".repeat(stars);
            println!("{}", row);
        });
    });
}

fn main() {
    draw_tree(5); // змінюй це число, щоб малювати більше або менше рівнів
}
