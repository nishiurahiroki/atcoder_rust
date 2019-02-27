fn main () {
    let input = get_input_read_line();
    let operation_count : usize = input.trim().parse().unwrap();

    let input = get_input_read_line();
    let mut numbers : Vec<i32> = input
                                    .split(" ")
                                    .map(|value| value.trim().parse::<i32>().unwrap())
                                    .collect();

    let mut division_limit : usize = 0;
    loop {
        let can_division_all = operation_count == numbers.iter().filter(|number| *number % 2 == 0).count();
        if can_division_all {
            division_limit += 1;
            numbers = numbers
                        .iter()
                        .map(|number| number / 2)
                        .collect();
        } else {
            break;
        }
    }

    println!("{:?}", division_limit)
}

fn get_input_read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    input
}
