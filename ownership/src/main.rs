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
    let s2 = s1; // s1 moved to s2, s1 nolonger valid
    println!("{}",s2); //s2 will work s1 not so mutch

    // copy
    let s1 = String::from("Hello");
    let s2 = s1.clone();// made s2 a clone of s1 
    println!("{} {}",s2, s1); 

    let x = 5;
    let y = x; //simple datatype, known size and are only stored on the stack, therfore this works
    println!("{} are the 'same' {}", x, y);

}// scope over and memory returned

fn some_func() {//s is not valid here, not declared yet

    let s="hello";//s is declared and now valid
    //do stuff with s  here

    }// s no longer valid, scope is ended end dead code

fn ownership_transferal(){

    let s1 = gives_ownership();//gives ownership moves its returnvalue
                               //to s1

    let s2 = String::from("Hello");// s2 enters scope

    let s3 = takes_and_gives_back(s2); // s2 is moved in to takes_and_gives_back
                                      // move return in to s3
} // s3 goes out of scope as does s1 and both are dropped

fn gives_ownership() -> String{
    let some_string = String::from("Hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String{ //a_string comes in to scope

    a_string  //a_string is returned
}