fn main() {
    println!("Function test:");
    another_function(5);
    yet_another_function("hello");
    multi_param_function(3, "Bendik");

}
fn yet_another_function(x: &str) {
    println!("The value passed is {}", x);
}

fn another_function(x: i32) {
    println!("The value passed is {}", x);
}

fn multi_param_function(x: i32, y: &str){
    println!("The int passed is {}, the string is {}", x, y);
}
