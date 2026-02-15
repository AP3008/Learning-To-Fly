fn main (){
    let v: Vec<i32> = Vec::new(); 
    let the_vec:Vec<i32> = vec![1,2,3];
    
    let mut vec:Vec<i32> = Vec::new(); 
    vec.push(5);
    vec.push(6); 
    vec.push(7);

    println!("{:?}",vec);

    //I want to take the element in the 3rd position
    let thrid: &i32 = &vec[2]; 
    //We make it a reference to i32 because we don't want to take owndership of the variable. 

    let first: Option<&i32> = v.get(0); 
    match first{
        Some(first:i32) => println!("The first element is {first}"),
        None => println!("that element doesn't exist"),
    }

    // UTF-8
    let s = "hello".to_string();
    let s = String::from("Hello");
    let mut s = String::from("foo");
    //To grow the string we use push_str not push, because push just pushes a string slice and the
    //result can't grow, but to add another string, we can use push_str
    s.push_str("bar");

    //HashMap<K, V>

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50); 
    let team_name: String = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); //Because get returns an option so we just want to
                                                //ignore it because we know we will have our value
    for (key: &String, value: &i32) in &scores{
        println!("{key}, {vale}");
    }
}
