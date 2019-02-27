fn main() {
    let input = get_input_read_line();
    let monster_count : usize = input.trim().parse::<usize>().unwrap();

    let input = get_input_read_line();
    let mut monsters : Vec<i32> = input
                                .split(" ")
                                .map(|value| value.trim().parse::<i32>().unwrap())
                                .collect();

    let mut is_all_same = false;
    let mut can_division10_all = false;
    let mut is_even_only = false;
    let mut exists_odd = false;

    let mut same_count = 0;
    let mut division10_count = 0;
    let mut even_count = 0;
    let mut odd_count = 0;

    for monster in &monsters {
        if monster == &monsters[0] {
            same_count += 1;
        }

        if monster % 10 == 0 {
            division10_count += 1;
        }

        if monster % 2 == 0 {
            even_count += 1;
        }

        if monster % 2 != 0 {
            odd_count += 1;
        }
    }

    let total_monster_count = monsters.iter().count();

    is_all_same = same_count == total_monster_count;
    can_division10_all = division10_count == total_monster_count;
    is_even_only = even_count == total_monster_count;
    exists_odd = odd_count > 0;

    if is_all_same {
        println!("{:?}", monsters[0]);
    } else if can_division10_all {
        let monster_list = monsters.sort();
        println!("{:?}", monsters[0]);
    } else if is_even_only {
        println!("{:?}", 2);
    } else if exists_odd {
        println!("{:?}", 1);
    }
}

fn get_input_read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    input
}
