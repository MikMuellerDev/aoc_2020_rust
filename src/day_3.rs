pub fn run() {
    println!("<--- DAY 03 --->");
    let raw_input = include_str!("resources/day3.txt");

    let input: Vec<&str> = raw_input.split('\n').collect();

    let slope1 = Slope { right: 3, down: 1 };
    println!("Tree count part1: {}", part1(&input, slope1));

    part2(&input);
}

/* fn part1(input: &Vec<&str>) {
    let slope_y: u8 = 1;
    let slope_x: u8 = 3;

    let mut iterator_x: u32 = 0;
    let mut iterator_y: u32 = 0;
    let mut tree_count = 0;
    let line_length: u8 = input[0].chars().count() as u8;
    loop {
        let line_str = input[iterator_y as usize];
        println!("line: {}", line_str);
        let array_index_column: u8 = iterator_x as u8 % line_length;
        let current_char = line_str.chars().nth(array_index_column as usize).unwrap();

        println!("current char: {}", current_char);

        if current_char == '#' {
            tree_count += 1;
        }

        iterator_y += slope_y as u32;
        iterator_x += slope_x as u32;

        if iterator_y >= input.len() as u32 {
            break;
        }
    }
    println!("Trees: {}", tree_count)
} */

struct Slope {
    right: u8,
    down: u8,
}

fn part1(input: &Vec<&str>, slope: Slope) -> u16 {
    let mut tree_count: u16 = 0;

    let mut position_x: u16 = 0;
    let mut position_y: u16 = 0;

    let line_width: u8 = input[0].chars().count() as u8;
    loop {
        let current_char = input[position_y as usize]
            .chars()
            .nth(position_x as usize % line_width as usize)
            .unwrap();

        if current_char == '#' {
            tree_count += 1;
        }

        position_x += slope.right as u16;
        position_y += slope.down as u16;

        if position_y as usize >= input.len() {
            break;
        }
    }
    return tree_count;
}

fn part2(input: &Vec<&str>) {
    let slopes: [Slope; 5] = [
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];
    let mut prod: u32 = 1;
    for sl in slopes {
        prod = prod * part1(input, sl) as u32;
    }
    println!("Trees Part2: {}", prod)
}