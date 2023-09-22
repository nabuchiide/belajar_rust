
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);
struct MySturct;
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("admin@email.com"),
        username: String::from("admin user"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("adminayah@email.com"),
        ..user1
    };

    let mut user1 = build_user(String::from("emailia@email.com"), String::from("nana"));
    println!("Hello : {}", user2.username);
    println!("email mu : {}", user2.email);

    let black = Color(0, 0, 0);
    let center = Point(0, 0, 0);

}
