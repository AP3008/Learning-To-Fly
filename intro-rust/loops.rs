fn main(){
    let mut counter = 0; 
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; 
        }
    }

    println!("The result is {result}"); // Returns 20

    // Loop Labels

    // While loops
    
    let mut number = 3; 
    while number != 0 {
        println!("{number}");
        number -= 1;
        break
    }
    println!("Exited while loop");

    // For loops

    let a = [1,2,3,4,5,6];

    for element in a {
        println!("{element}");
    }

    
}
