struct Reactangle {
    width: u32,
    height: u32,
}

// implementation of structure
impl Reactangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn can_hold(&self, other: &Reactangle) -> bool {
        return self.width >= other.width && self.height >= other.height;
    }
}

// enum IP
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]  // allow println! macro with {:?} trait 
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let rect = Reactangle {
        width: 12,
        height: 32,
    };
    let rect1 = Reactangle {
        width: 11,
        height: 30,
    };
    let rect2 = Reactangle {
        width: 12,
        height: 38,
    };
    println!("area of rect: {}", rect.area());
    println!("rect can hold rect1? {}", rect.can_hold(&rect1));
    println!("rect can hold rect2? {}", rect.can_hold(&rect2));

    // enum implementation
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.1.0.1"),
    };
    let office = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("1"),
    };
    println!("{:?}",home);
}
