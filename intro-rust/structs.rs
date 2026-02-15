fn main(){
    //tuple
    let tup: (i32,i32) = (200,500);

    // Struct 
    struct Book{
            title: String,
        author: String, 
        pages: u32,
        available: bool,
    }
    struct User{
        active: bool, 
        username: String, 
        email: String, 
        sign_in_count: u64,
    }
    
    let mut user1: User = User{
        active: true, 
        username: "John".to_string(),
        email: String::from("john@gmail.com"),
        sign_in_count: 21,
    };
    
    let user2: User = User{
        active: false,
        ..user1
    };

    user1.email = String::from("anotheremail@gmail.com"); 
}
