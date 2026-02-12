fn main() {
    test();
    add_numbers(1,2,4.32);
    println!("{}", multiply_numbers(1,4));
}

fn test() {
    println!("Test has been called!");
}

fn add_numbers(x: i32, y: i32, z: f32){
    let a = {
        x+y+(z as i32) //If you add a semicolon on the last statement, you don't return anything
                       //and then a will be unable to be formatted. 
    };
    println!("The sum is: {}", a);
}

fn multiply_numbers(x:i32, y:i32) -> i32{
    x * y //This is equal to return x * y;
}
