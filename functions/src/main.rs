fn main() {
    println!("Function test:");
    another_function(5);
    yet_another_function("hello");

}
fn yet_another_function(x: &str) {
    println!("The value passed is {}", x);
}

fn another_function(x: i32) {
    println!("The value passed is {}", x);
}
