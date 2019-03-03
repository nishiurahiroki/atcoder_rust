fn main() {
    let (first, second, exceed) = {
        let input = get_read_line();
        let inputs : Vec<usize> = input
                                    .split(" ")
                                    .map(|value| value.trim().parse::<usize>().unwrap())
                                    .collect();
        (inputs[0], inputs[1], inputs[2])
    };

    let biggest_number = {
        if first >= second {
            first
        } else if first == second {
            second
        } else {
            second
        }
    };

    let mut common_divisors : Vec<usize> = Vec::new();
    for index in 0..biggest_number {
        if first % (index + 1) == 0 && second % (index + 1) == 0 {
            common_divisors.push(index + 1);
        }
    }

    println!("{:?}", common_divisors[exceed - 1]);
}

fn get_read_line() -> String {
    let mut result = String::new();
    std::io::stdin().read_line(&mut result).unwrap();
    result
}
