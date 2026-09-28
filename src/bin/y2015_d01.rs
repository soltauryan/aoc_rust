use aoc_rust::read_input;

fn main() {
    let input = read_input(2015, 1);
    p1(&input);
    p2(&input);
}

fn p1(input: &String) {
    let mut floor = 0;

    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
    }

    println!("Part 1 answer:");
    println!("{}", floor);
}


fn p2(input: &String) {
    let mut floor = 0;
    let mut position = 1;

    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
        if floor == -1 {
            println!("P2 Answer:");
            println!("{}", position);
            break;
        }
        position += 1;

    }
}