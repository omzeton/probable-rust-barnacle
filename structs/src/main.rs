struct Color(i32, i32, i32);
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    favorite_color: Color,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


fn main() {
    let mut user1 = User {
        active: true,
        email: String::from("test@test.com"),
        username: String::from("johnny"),
        sign_in_count: 1,
        favorite_color: Color(0, 0, 0),
    };

    user1.email = String::from("test2@test.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
        favorite_color: Color(255, 255, 255),
    };

    let mut user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    user3.favorite_color = Color(0, 255, 0);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
    println!("rect1 is {:?}", &rect1);
    println!("rect1 is {} and {}", &rect1.width, &rect1.height);
}
