fn main(){
    let mut a: u8 = 5; 
    println!("{}", a);
    a = 16; 
    println!("{}", a); 
    
    //constants
    const Y: u8 = 10; 
    println!("{}", Y);
    // Y cannot be mutable because it is constant
    println!("{}", PI);  
}

//Can be used globally
const PI: f64 = 3.141592653;
