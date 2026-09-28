pub fn read_input(year: u32, day: u32) -> String {
    let path = format!("input/{year}/d{day:02}.txt");
    std::fs::read_to_string(path).unwrap()
}