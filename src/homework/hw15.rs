use std::collections::HashSet;

pub fn solve_puzzle() -> Vec<(u8, u8, u8, u8, u8, u8, u8, u8)> {
    let mut solutions = Vec::new();

    for m in 1..=9 {
        for u in 0..=9 {
            if u == m { continue; }
            for x in 0..=9 {
                if x == m || x == u { continue; }
                for a in 1..=9 {
                    if a == m || a == u || a == x { continue; }
                    for s in 1..=9 {
                        if s == m || s == u || s == x || s == a { continue; }
                        for l in 0..=9 {
                            if l == m || l == u || l == x || l == a || l == s { continue; }
                            for o in 0..=9 {
                                if o == m || o == u || o == x || o == a || o == s || o == l { continue; }
                                for n in 0..=9 {
                                    if n == m || n == u || n == x || n == a || n == s || n == l || n == o { continue; }

                                    let muxa = m as u32 * 1000 + u as u32 * 100 + x as u32 * 10 + a as u32;
                                    let a_digit = a as u32;
                                    let slon = s as u32 * 1000 + l as u32 * 100 + o as u32 * 10 + n as u32;

                                    if muxa * a_digit == slon {
                                        solutions.push((m, u, x, a, s, l, o, n));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    solutions
}

pub fn print_solution(solution: &(u8, u8, u8, u8, u8, u8, u8, u8)) {
    let (m,u,x,a,s,l,o,n) = *solution;
    println!("  {}{}{}{}", m, u, x, a);
    println!("       x   {}", a);
    println!("  ------");
    println!("   {}{}{}{}", s, l, o, n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        let sols = solve_puzzle();
        assert!(!sols.is_empty(), "Should find at least one solution");
        println!("Found {} solutions.", sols.len());
        for sol in &sols {
            print_solution(sol);
        }
    }
}

