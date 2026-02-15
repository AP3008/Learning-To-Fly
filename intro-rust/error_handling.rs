fn main(){
    let result = divide(10.0, 0.0); 
    match result{
        Some(x) => println!("result: {x}"),
        None => println!("Error :("),
    }
}
// enum Option<T>{ // Define the generic option type
//     Some(T), // represents a value
//     None // represenets no value
// }

// enum Result<T, E>{ // Define the generic result type
//     Ok(T) // Represents a value
//     Err(E) // Represents an error
// }

fn divide(numerator: f64, denominator: f64) ->Option<f64>{
    if denominator == 0.0{
        None
    } else {
        Some(numerator/denominator)
    }
}

    
