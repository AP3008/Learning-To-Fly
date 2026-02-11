use std::io; 
fn main(){
    //let x = 127_000 as i64;
    let y = 10 as i8;

    let n = (i32::MAX as i64) + 1; 
    let z = n + (y as i64);
    println!("{}", z);

    let int = 10 as f32; 
    println!("{}", int+3.245);
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Readline error");
    let num_input:i64 = input.trim().parse().unwrap();
    println!("{}",num_input + 2);
}
