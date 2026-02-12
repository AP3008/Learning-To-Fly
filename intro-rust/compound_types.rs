fn main(){
    //Arrays
    let arr:[i32; 5] = [1,2,3,4,5];
    println!("Numbers array: {:?}", arr);

    //Tuples
    let person:(&str, i32, bool)= ("Alice", 30, true);
    println!("{:?}", person);

}
