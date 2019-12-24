#![warn(clippy::all)]

#[macro_use] extern crate lazy_static_include;
#[macro_use] extern crate lazy_static;

lazy_static_include_str!(TEST, "data/input.txt");

/// https://adventofcode.com/2019/day/1
fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

/// Fuel required to launch a given module is based on its mass. 
/// Specifically, to find the fuel required for a module,
/// take its mass, divide by three, round down, and subtract 2.
fn fuel_required(mass: i64) -> i64 {
    (mass / 3) - 2
}

fn fuel_required_2(mass: i64) -> i64 {
    let mut result = (mass / 3) - 2;
    if result > 0 {
        result += fuel_required_2(result);
        result
    } else {
        0
    }
}

fn parse_data() -> Vec<i64> {
    TEST.lines().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn part_1() -> i64 {
    let data = parse_data();
    data.into_iter().fold(0, |acc, x| acc + fuel_required(x))
}

fn part_2() -> i64 {
    let data = parse_data();
    data.into_iter().fold(0, |acc, x| acc + fuel_required_2(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = parse_data();
        assert_eq!(data[0], 141_923);
        assert_eq!(data.len(), 100);
    }

    #[test]
    fn test_fuel_required() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100_756), 33_583);
    }

    #[test]
    fn test_fuel_required_2() {
        assert_eq!(fuel_required_2(14), 2);
        assert_eq!(fuel_required_2(1969), 966);
        assert_eq!(fuel_required_2(100_756), 50346);
    }
}
