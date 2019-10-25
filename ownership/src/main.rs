//dead code
fn some_func() {//s is not valid here, not declared yet

    let s="hello";//s is declared and now valid
    //do stuff with s  here

    }// s no longer valid, scope is ended end dead code

fn main(){
    
    let mut s = String::from("Heisann"); //create mutable string s now valid
    s.push_str(", verden!"); // appends to string
    println!("{}",s);

    //ints
    let x = 3; //bind 3 to x
    let y = x; // copy 3 to y 
    // ints are simple values with known fixed size, pushed to stack

    //strings
    let s1 = String::from("Hello"); // poiter pushed to stack values on heap
    let s2= s1; // s1 moved to s2, s1 nolonger valid
    println!("{}",s2); //s2 will work s1 not so mutch

    // copy

}// scope over and memory returned