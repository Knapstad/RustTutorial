fn some_func() {//s is not valid here, not declared yet

    let s="hello";//s is declared and now valid
    //do stuff with s  here

    }// s no longer valid, scope is ended

fn main(){

    let mut s = String::from("Heisann"); //create mutable string
    s.push_str(", verden!"); // appends to string
    println!("{}",s);
}