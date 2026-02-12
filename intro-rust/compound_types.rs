fn main(){
    //Arrays
    let arr:[i32; 5] = [1,2,3,4,5];
    println!("Numbers array: {:?}", arr);

    //Tuples
    let person:(&str, i32, bool)= ("Alice", 30, true);
    println!("{:?}", person);
    
    //Slices
    let num_slice: &[i32] = &[1,2,3,4,5];

    //Strings
    let mut say: String = String::from("Hell, ");
    println!("I say: {}", say); 
    say.push_str("Yeah!");
    println!("I really said: {}", say);
}
