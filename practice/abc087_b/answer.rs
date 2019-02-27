fn main () {
    
}

fn get_input_read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    input
}
