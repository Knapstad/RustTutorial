fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    let int = 42; //integer 32bit signed by default
    let float = 4.2;// float default to 64bit


    println!("{} {}",int, float);
// Numeric opperations
    let sum = 5 + 10;

    let difference = 95.5 - 3.1;

    let product = 4 * 30;

    let quotient = 56.7/32.1;

    let remainder = 43 % 5;
    println!("sum: {}\ndiff: {}\nprod: {}\nquot: {}\nremain: {}", sum, difference, product, quotient, remainder);

//Boolean

    let t = true;

    let f: bool = false;

// Char types

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

// Compundtypes
    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x is {}", x);

    //array
    let a = [1, 2, 3, 4, 5];
    let b: [i32;5] = [1, 2, 3, 4, 5];
    let c = [3; 5];
    println!("{:?}\n{:?}\n{:?}", a, b, c);
}
