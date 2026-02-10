fn main(){

    // Scalar Data types

    let x: i32 = 5; // Assigning 32 bit int to x
    //Other type of integers
    let y: i8 = 2;
    let z: i16 = 14;
    let a: i64 = 128;
    let b: i128 = 2048; //Represents the number of bits
    //for i (integer) you can make it negative as it is signed, used two's complement so
    //for i8 we have -(2^7) -> 2^7 - 1 as the range 
    let x: u32 = 15;
    // u (unsigned) is the same as signed except you cannot use negative numbers as no bit is being
    // ex. range u8 0 -> 2^8

    //For floating point numbers we only have 2
    let x: f32 = 12.53;
    //^ single precision 
    let x: f64 = 1334.21245; 
    //^ double precision
    //If type not specified, f64 is default
    
    // Booleans

    let a: bool = true; //true = 1 and false = 0

    let b: char = 'b'; // use single quotation mark 

    // Compound Types
    
    // Tuple
    let tup (i32, bool, char) = (1, true, 'c');
    let mut tup2 (i8, bool, char) = (1, true, 'c'); //These are two seperate tuples and I could not
                                                //sign one another to each other. 
    println!("{}", tup.0); //This accesses the first element of the tuple

    // tuple has to be mutable to change values as indicies but it has to be of the same type

    // Arrays
    
    let arr:[i32; 5] = [1,2,3,4,5]; // Has to be of the same type
    // Array definition has to be the [type; num elements]

}
