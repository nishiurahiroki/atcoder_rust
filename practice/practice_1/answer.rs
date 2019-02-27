fn main() {
    let first = get_input_read_line().trim().parse::<i32>().unwrap();

    let (second, third) = {
        let line = get_input_read_line();
        let seconds : Vec<&str> = line.split(" ").collect();
        (seconds[0].trim().parse::<i32>().unwrap(), seconds[1].trim().parse::<i32>().unwrap())
    };

    let message = get_input_read_line();

    println!("{:?} {}", first + second + third, message.trim());
}

fn get_input_read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    input.to_string()
}
