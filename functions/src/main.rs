fn main() {
    println!("Function test:");
    another_function(5);
    yet_another_function("hello");
    multi_param_function(3, "Bendik");
    expression_statement();

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

fn expression_statement(){
    let x = 5; // statement

    let y= { // expression
        let x = x-2;
        x+1 // no semicolon to make it a expression
    };
    println!("The value of y is {}", y)
}