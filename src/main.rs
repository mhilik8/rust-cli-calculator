fn main() {
    println!("Hello to Rust CLI Calculator");
    let input_str = "5+6";
    let parsed = parse_math(input_str);
    let results = parsed.0 + parsed.1;
    println!("results: {}", results);
}

fn parse_math(to_parse: &str) -> (i32, i32){
    let parts: Vec<&str> = to_parse.split("+").collect();
    let left: i32 = parts[0].parse().unwrap();
    let right: i32 = parts[1].parse().unwrap();
    println!("left: {}", left);
    println!("right: {}", right);
    (left, right)
}
