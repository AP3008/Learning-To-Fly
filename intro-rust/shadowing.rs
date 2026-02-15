fn main(){
    // Shadowing && Not the same as mut
    let x = 5; // x_1 = 5
    // Here x_1 is being shadowed by x_2, we are using the value from x_1 and incrementing it to
    // make x_2, now the compiler will only use x_2 when refering to x
    let x = x + 1; // x_2 = 5 + 1 = 6

    {
        // This is shadowed from the inside, 
        let x = x * 2; // x_3 = 6 * 2 = 12
        println!("The value of x in the inner scope is: {x}");
    }

    let spaces = "   "; 
    let spaces = spaces.len(); // Changed types using shadowing
}
