use std::fs;

fn main() {
    //answer_one()
}
fn read_input() {
    fs::read_to_string("./inputs/day_1.txt").unwrap();
}

// region --_Day1
fn answer_one(input: &str) -> String {
    println!("{}", input);
    "24000".to_string()
}

fn answer_two(input: &str) -> String {
    println!("{}", input);
    "24000".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let test_input = "1000
        2000
        3000
        ";

        assert_eq!(answer_one(test_input), "24000")
    }
}

// endregion -- Day1
