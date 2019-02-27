const BTC_RATE : f64 = 380000.0;

fn main () {
    let input = get_read_line();
    let gift_presenter_count = input
                                .trim()
                                .parse::<i32>()
                                .unwrap();

    let mut money_list : Vec<f64> = Vec::new();

    for _ in 0..gift_presenter_count {
        let input = get_read_line();
        let mut result : Vec<String> = input
                                    .split(" ")
                                    .map(|value| value.trim().to_string())
                                    .collect();
        let price = &result[0];
        let currency = &result[1];

        match currency.as_str() {
            "JPY" => money_list.push(price.parse::<f64>().unwrap()),
            "BTC" => money_list.push(price.parse::<f64>().unwrap() * BTC_RATE),
            _ => panic!("error.")
        }
    }

    let total : f64 = money_list
                        .iter()
                        .fold(0.0, |sum, i| sum + i);

    println!("{:?}", total);
}

fn get_read_line() -> String {
    let mut result = String::new();
    std::io::stdin().read_line(&mut result).unwrap();
    result
}
