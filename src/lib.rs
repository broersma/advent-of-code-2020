pub mod aoc01;
pub mod aoc02;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc1() {
        aoc01::main();
    }

    #[test]
    fn aoc2() {
        aoc02::main();
    }
}
