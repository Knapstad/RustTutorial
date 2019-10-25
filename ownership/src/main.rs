//dead code
fn some_func() {//s is not valid here, not declared yet

    let s="hello";//s is declared and now valid
    //do stuff with s  here

    }// s no longer valid, scope is ended end dead code

fn main(){
    
    let mut s = String::from("Heisann"); //create mutable string s now valid
    s.push_str(", verden!"); // appends to string
    println!("{}",s);
}// scope over and memory returned