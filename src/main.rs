fn main() {
    let mut input1 = String::new();
    println!("First number:");
    std::io::stdin().read_line(&mut input1).unwrap();
    let number1: i32 = input1.trim().parse().unwrap();

    let mut input2 = String::new();
    println!("Second number:");
    std::io::stdin().read_line(&mut input2).unwrap();
    let number2: i32 = input2.trim().parse().unwrap();

    let mut input3 = String::new();
    println!("Operator:");
    std::io::stdin().read_line(&mut input3).unwrap();
    let operator: &str = input3.trim();

    match operator {
        "+" => println!("{}", number1 + number2),
        "-" => println!("{}", number1 - number2),
        "*" => println!("{}", number1 * number2),
        "/" => println!("{}", number1 / number2),
        &_ => panic!("Error"),
    }
}
