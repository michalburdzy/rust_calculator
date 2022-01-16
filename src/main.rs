use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap();
    let second = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let arithmetic_operator = operator.chars().next().unwrap();

    let result = calculate(arithmetic_operator, first_number, second_number);

    println!("RESULT: {}", result);
}

fn calculate(operator: char, num_a: f32, num_b: f32) -> f32 {
    match operator {
        '+' => num_a + num_b,
        '-' => num_a - num_b,
        '/' => num_a / num_b,
        '%' => num_a % num_b,
        'x' | 'X' | '*' => num_a * num_b,
        _ => panic!("Invalid arithmetic operator used {}", operator),
    }
}
