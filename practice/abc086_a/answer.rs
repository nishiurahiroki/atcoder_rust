fn main() {
    let input = get_input_read_line();

    let squares : Vec<&str> = input
                                .trim()
                                .split("")
                                .filter(|value| "1" == value.to_string())
                                .collect();

    println!("{:?}", squares.iter().count());
}

fn get_input_read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    input
}
