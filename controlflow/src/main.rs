fn main() {
    let number = 7;

    if number < 5{
        println!("condition was {}",number<5);
    } else {
        println!("condition was {}", number<5);
    }
    let mut num = 1;
    loop{
        if num == 4{
            break
        }
        println!("counting to 3... {}", num);
        num+=1;
    }

    //return value from loop
    let mut count = 0;
    let result = loop{
        count +=1;
        if count == 10 {
            break count *2;
        }
    };
    println!("result is: {}", result);

}

