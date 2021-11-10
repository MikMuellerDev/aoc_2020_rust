extern crate regex;
use regex::Regex;

pub fn run() {
    println!("<--- DAY 02 --->");
    let raw_input = include_str!("resources/day2.txt");

    let re = Regex::new(r"^(\d{1,2})-(\d{1,2}) ([a-z]): ([a-z]+)$").unwrap();

    let input: Vec<(u8, u8, char, &str)> = raw_input
        .split('\n')
        .map(|policy| {
            let caps = re.captures(policy).unwrap();
            (
                caps.get(1).unwrap().as_str().parse::<u8>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<u8>().unwrap(),
                caps.get(3).unwrap().as_str().parse::<char>().unwrap(),
                caps.get(4).unwrap().as_str(),
            )
        })
        .collect::<Vec<(u8, u8, char, &str)>>();

    part1(&input);
    part2(&input);
}

// fn part1(input: &Vec<(u8, u8, char, &str)>) {
//     println!(
//         "{}",
//         input
//             .iter()
//             .filter(|policy| {
//                 (policy.0..=policy.1)
//                     .contains(&(policy.3.chars().filter(|char| char == &policy.2).count() as u8))
//             })
//             .count()
//     )
// }

fn part1(input: &Vec<(u8, u8, char, &str)>) {
    let mut valid_password_count: u32 = 0;
    for tupl in input.iter() {
        // println!("{:?}", tupl);
        let char_to_search = tupl.2;
        let minimum_occurence = tupl.0;
        let maximum_occurence = tupl.1;
        let password = tupl.3;
        let occurences: u8 = password
            .chars()
            .filter(|char| char == &char_to_search)
            .count() as u8;

        if (minimum_occurence..=maximum_occurence).contains(&occurences) {
            valid_password_count += 1;
        }
    }
    println!("Valid Passwords Part 1: {}", valid_password_count)
}

// fn part2(input: &Vec<(u8, u8, char, &str)>) {
//     println!(
//         "{}",
//         input
//             .iter()
//             .filter(|policy| {
//                 (policy.3.chars().nth((policy.0 - 1) as usize).unwrap() == policy.2)
//                     ^ (policy.3.chars().nth((policy.1 - 1) as usize).unwrap() == policy.2)
//             })
//             .count()
//     )
// }

fn part2(input: &Vec<(u8, u8, char, &str)>) {
    let mut valid_password_count: u32 = 0;
    for tupl in input.iter() {
        let char = tupl.2;
        let index1 = tupl.0;
        let index2 = tupl.1;
        let password = tupl.3;

        if (password.chars().nth((index1 - 1) as usize).unwrap() == char)
            ^ (password.chars().nth((index2 - 1) as usize).unwrap() == char)
        {
            valid_password_count += 1;
        }
    }
    println!("Valid Passwords Part 2: {}", valid_password_count)
}
