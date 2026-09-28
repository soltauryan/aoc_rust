use aoc_rust::read_input;
use std::str::FromStr;

#[derive(Debug)]
struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl FromStr for Present {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("x");

        let length = parts.next()
            .ok_or("Missing Length")?
            .trim()
            .parse::<u32>()
            .map_err(|_| "Invalid number")?;

        let width = parts.next()
            .ok_or("Missing Width")?
            .trim()
            .parse::<u32>()
            .map_err(|_| "Invalid number")?;

        let height = parts.next()
            .ok_or("Missing Height")?
            .trim()
            .parse::<u32>()
            .map_err(|_| "Invalid number")?;

        Ok(Present { length, width, height })
    }
}

fn main() {
    let input = read_input(2015, 2);
    let mut sum = 0;

    let presents: Result<Vec<Present>, String> = input
        .lines()
        .map(Present::from_str)
        .collect();

    match presents {
        Ok(present_list) => {
            for p in &present_list {
                sum += wrapping_required(p);
            }
            println!("Total wrapping paper: {sum}")
        }
        Err(e) => eprintln!("Error parsing input: {e}"),
    }
}

fn wrapping_required(present:&Present) -> u32 {
    let a = present.length * present.width;
    let b = present.width * present.height;
    let c = present.height * present.length;
    let smallest_side = a.min(b).min(c);
    
    return {
        a * 2 +
        b * 2 +
        c * 2 + 
        smallest_side
    }
}