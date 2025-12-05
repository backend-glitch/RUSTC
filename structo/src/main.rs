struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {

    //struct
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

  
    user1.email = String::from("anotheremail@example.com");

      //instance of user1.
       let user2 = User {
        email: String::from("another@example.com"),
        ..user1 //using slices
    };
}

//struct functions
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

//area 
fn areaofrect(height: u32, width: u32) -> u32{

    widht*height
}