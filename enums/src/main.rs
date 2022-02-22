enum IpAddrKind {
    V4,
    V6,
}

struct IpAddrOld {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrNew {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    println!("Hello, world!");
    let home = IpAddrOld {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddrOld {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let home = IpAddrNew::V4(127, 0, 0, 1);
    let loopback = IpAddrNew::V6(String::from("::1"));

    let m = Message::Write(String::from("Liminal haze"));
    m.call();
}
