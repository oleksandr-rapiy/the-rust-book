use crate::rectangle::rectangle::Rectangle;

mod rectangle;

// NOTE: struct - allow you to group related data
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user = User {
        email: String::from("some@mail.com"),
        username: String::from("some_userName"),
        active: true,
        sign_in_count: 1,
    };

    let name = user.username;

    user.username = String::from("another@mail.com");

    let user2 = build_user(
        String::from("new_user@mail.com"),
        String::from("someUserName"),
    );

    // NOTE: create instance of the user from another one instance

    let new_one_user = User {
        email: String::from("new_one_user@mail.com"),
        username: String::from("another_user_name"),
        ..user2 // NOTE: here other not assigned data will be get from user2
    };

    let other_user = User {
        ..new_one_user
    };

    // NOTE: tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rec = Rectangle {
        width: 30,
        height: 50,
    };

    let rec2 = Rectangle {
        width: 100,
        height: 213,
    };

    println!("rec: {:#?}", rec);
    println!("The area of the rectangle is {} square pixels.", rec.area());

    println!("Can rec1 hold rec2 - {}", rec.can_hold(&rec2));

    println!("Can rec2 hold rec1 - {}", rec2.can_hold(&rec));

    let rec3 = Rectangle::square(21);
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 0,
        active: true,
    }
}
