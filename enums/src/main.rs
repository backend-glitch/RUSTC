enum Shape {
    Circle(u32),
    Square(u32),
    Triangle { base: u32, height: u32 },

}

    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }


   
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct


    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

   


  

fn main() {
    let s = Shape::Circle(10);

    match s {
        Shape::Circle(r) => println!("R = {}", r),
        Shape::Square(x) => println!("Side = {}", x),
       
    }

     let s2 = Shape::Triangle { base: 5, height: 8 };

       let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

     let m = Message::Write(String::from("hello"));
    m.call();

}

