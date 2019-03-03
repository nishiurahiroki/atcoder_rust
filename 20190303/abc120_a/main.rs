fn main() {
    let (price, haved_money, satisfaction) = {
        let input_value = get_read_line();
        let values : Vec<i32> = input_value
                                    .split(" ")
                                    .map(|value| value.trim().parse::<i32>().unwrap())
                                    .collect();
        (values[0], values[1], values[2])
    };

    let can_buy_count = haved_money / price;
    if can_buy_count >= satisfaction {
        println!("{:?}", satisfaction);
    } else {
        println!("{:?}", can_buy_count);
    }
}

fn get_read_line() -> String {
    let mut result = String::new();
    std::io::stdin().read_line(&mut result).unwrap();
    result
}
