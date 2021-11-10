pub fn run() {
    println!("<--- DAY 01 --->");
    let raw_input = include_str!("resources/day1.txt");

    let input: Vec<u32> = raw_input
        .split('\n')
        .map(|it| it.parse::<u32>().unwrap())
        .collect();

    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<u32>) {
    // Uses index in addition. Otherwise, execution times will be longer and results will appear twice
    for (index, outer) in input.iter().enumerate() {
        for inner in &input[index..] {
            // &input[index..] <slices> the input and [index..] defines a range from <index> to the <end> of the array
            if outer + inner == 2020 {
                println!(
                    "Answer Part1 => {}",
                    inner * outer
                );
            }
        }
    }
}

fn part2(input: &Vec<u32>) {
    for (index1, num1) in input.iter().enumerate() {
        for (index2, num2) in &mut input[index1..].iter().enumerate() {
            for num3 in &input[(index1 + index2)..] {
                if num1 + num2 + num3 == 2020 {
                    println!("Answer Part2 => {}", num1 * num2 * num3);
                }
            }
        }
    }
}
