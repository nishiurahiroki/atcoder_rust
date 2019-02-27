fn main() {
    let (first, second) = {
        let input = get_input_read_line();
        let numbers : Vec<i32> = input
                                    .split(" ")
                                    .map(|value| value.trim().parse().unwrap())
                                    .collect();
        (numbers[0], numbers[1])
    };

    match second % first {
        0 => println!("{:?}", first + second),
        _ => println!("{:?}", second - first)
    };
}

fn get_input_read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    input
}
