#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

impl User{
    fn get_username(&self) -> bool {
        self.active
    }

    fn get_user(username: String) -> User {
        User {
            username,
            email: String::from("someone@example.com"),
            sign_in_count: 1,
            active: false,
        }
    }
}

fn main(){
    let mut user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));

    user1.email = String::from("anotheremail@example.com");

    println!( "The user is {:#?} ", &user1.get_username());

    let sq = Rectangle::square(3);
    println!( "The user is {:#?} ", sq);

    let u = User::get_user(String::from("someusername123"));
    println!( "The user is {:#?} ", u);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}