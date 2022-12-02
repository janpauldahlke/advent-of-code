use std::fs;

fn main() {
    let inp = read_input();
    //let out = answer_one(inp);
    let out = answer_two(inp);
    println!("{out}")
}
fn read_input() -> String {
    let res = fs::read_to_string("./inputs/day_1.txt").unwrap();
    res
}

// region --_Day1
fn answer_one(input: String) -> String {
    let one_result = input
        .split("\n\n")
        .map(|weight| {
            weight
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    one_result.to_string()
}

fn answer_two(input: String) -> String {
    let mut two_result = input
        .split("\n\n")
        .map(|weight| {
            weight
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    two_result.sort_by(|a, b| b.cmp(a));
    let sum: u32 = two_result.iter().take(3).sum();

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let test_input = read_input();

        assert_eq!(answer_one(test_input), "24000")
    }
}

// endregion -- Day1
