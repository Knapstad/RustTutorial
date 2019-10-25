fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    let int = 42; //integer 32bit signed by default
    let float = 4.2;// float default to 64bit


    println!("{} {}",int, float);

    let sum = 5 + 10;

    let difference = 95.5 - 3.1;

    let product = 4 * 30;

    let quotient = 56.7/32.1;

    let remainder = 43 % 5;
    println!("sum: {}\ndiff: {}\nprod: {}\nquot: {}\nremain: {}", sum, difference, product, quotient, remainder)
}
