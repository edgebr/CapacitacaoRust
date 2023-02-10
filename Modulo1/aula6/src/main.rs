/* Conteúdo desta aula:
    - Structs [Ok]
    - Tuple Structs [Ok]
    - Field Shorthand Syntax [Ok]
    - Enums [Ok]
    - Variant Payloads [Ok]
    - Enum Sizes [Ok]
    - Methods
    - Method Receiver
*/

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn match_arrays() {
        let list: &[u8] = &[1, 2, 3, 4, 5, 6];
        match list {
            [first, second, ..] => println!("First: {first}, second: {second}"),
            [first] => println!("Only first: {first}"),
            [] => println!("No items"),
        }
    }

    #[test]
    fn match_arrays_functional() {
        let mut list: &[u8] = &[1, 2, 3, 4];

        loop {
            match list {
                [head, tail @ ..] if tail.len() > 0 => {
                    println!("pop: {head}");
                    list = tail;
                }
                [head] => {
                    println!("pop tail: {}", head);
                    break;
                }
                [..] => {
                    unreachable!()
                }
            }
        }
    }

    #[test]
    fn match_arrays_functional_sum() {
        let mut list: &[u8] = &[3, 31, 2, 3, 4, 5, 6, 7];

        let mut total = 0;
        loop {
            match list {
                [first, middle @ .., last] => {
                    total += first + last;
                    println!("first + last = {}", first + last);
                    list = middle;
                }
                [first] => {
                    total += first;
                    println!("first = {}", first);
                    break;
                }
                _ => {
                    break;
                }
            }
        }
        println!("Total {total}");
    }

    #[derive(Debug)]
    struct Person {
        first_name: String,
        last_name: String,
        cpf: String,
        age: u8,
    }

    #[test]
    fn structs_101() {
        let jhon = Person {
            first_name: "Jhon".to_string(),
            last_name: "Foo".to_string(),
            cpf: "040.305.033-90".to_string(),
            age: 30,
        };
        dbg!(&jhon);

        println!(
            "\nName: {} {}\nCPF: {}\nAge: {}\n",
            jhon.first_name, jhon.last_name, jhon.cpf, jhon.age
        );
        let p = &jhon;
        println!("\nName: {:?}", p);
        println!(
            "\nName: {} {}\nCPF: {}\nAge: {}\n",
            &p.first_name, p.last_name, p.cpf, p.age

        );
    }

    #[derive(Debug)]
    struct MyConsts; //() unit struct

    impl MyConsts {
        pub fn pi() -> f64 {
            std::f64::consts::PI
            // 3.1415
        }
        pub fn e() -> f64 {
            std::f64::consts::E
            // 2.7182

        }
    }

    #[test]
    fn structs_unit() {
        dbg!(MyConsts);
        println!("π = {}", MyConsts::pi());
        println!("e = {}", MyConsts::e());
    }

    #[derive(Debug)]
    struct PairI32(i32, f32);

    #[test]
    fn structs_tuple() {
        let p = PairI32(10, 20.0);
        dbg!(&p);
        println!("result = {}", p.0 + p.1 as i32)
    }

    #[derive(Debug, Default)]
    pub struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }
        pub fn origin() -> Self {
            Point { x: 0, y: 0 }
        }
        pub fn x(&self) -> i32 {
            self.x
        }
        pub fn y(&self) -> i32 {
            self.y
        }
        pub fn set_x(&mut self, x: i32) {
            self.x = x;
        }
        pub fn set_y(&mut self, y: i32) {
            self.y = y;
        }
    }

    #[derive(Debug, Default)]
    pub struct Rectangle {

        top_left: Point,
        bottom_right: Point,
    }

    impl Rectangle {
        fn area(&self) -> i32 {
            (self.bottom_right.x() - self.top_left.x())
                * (self.bottom_right.y() - self.top_left.y())
        }
    }

    #[test]
    fn structs_composition() {
        let rec = Rectangle {
            // top_left: Point { x: 0, y: 0 },
            top_left: Point::origin(),

            bottom_right: Point::new(10, 10),
        };

        dbg!(&rec);
        println!("Area rec = {}m²", rec.area());

        let top_left = Point::new(100, 50);

        let rec2 = Rectangle {
            top_left, //shorthand syntax
            bottom_right: Point::new(101, 100),
        };
        dbg!(&rec2);
        println!("Area rec2 = {}m²", rec2.area());

        let rec3 = Rectangle {
            bottom_right: Point::new(1000, 1000),
            ..rec2 //all the other fields of rec2
        };

        // dbg!(&rec2.bottom_right);
        dbg!(&rec3);
        println!("Area rec3 = {}m²", rec3.area());

        let rec4 = Rectangle {
            bottom_right: Point::new(100, 200),
            ..Default::default()
        };

        dbg!(&rec4);
        println!("Area rec4 = {}m²", rec4.area());

        let mut rec5: Rectangle = Default::default();
        dbg!(&rec5);
        rec5.top_left = Point::new(0, 30);
        rec5.bottom_right.set_x(10);
        rec5.bottom_right.set_y(50);

        dbg!(&rec5);
        println!("Area rec5 = {}m²", rec5.area());

        let Rectangle { top_left, .. } = rec5;
        dbg!(top_left);

    }

    // #[derive(Debug)]
    // enum CLikeEnum {
    //     First,
    //     Second,
    //     Third,
    //     Fourth,
    // }
    //
    // #[derive(Debug)]
    // enum CLikeEnum2 {
    //     Small = 100,
    //     Medium,
    //     Large,
    //     ExtraLarge,
    // }
    //
    // #[derive(Debug)]
    // enum Variant {
    //     Nil,
    //     Tuple(i32, bool),
    //     Struct { a: String, b: f32, c: [i32; 5] },
    // }
    //
    // impl Variant {
    //     fn who_am_i(&self) {
    //         match self {
    //             Variant::Nil => println!("I am Nil"),
    //             Variant::Tuple(a, b) => println!("I am a the variant tuple ({a},{b})"),
    //             Variant::Struct { a, b, c } => {
    //                 println!("I am a the variant struct {{ {a}, {b}, {:?}}} ", c)
    //             }
    //         }
    //     }
    // }
    //
    // #[test]
    // fn enum_simple() {
    //     let c_like = CLikeEnum::Second;
    //     dbg!(c_like as i32);
    //     let c_like = CLikeEnum::Fourth;
    //     dbg!(c_like as i32);
    //     println!("{:?}, {:?}", CLikeEnum::First, CLikeEnum::Third);
    //     println!("CLikeEnum size is {}", std::mem::size_of::<CLikeEnum>());
    //
    //     let tshirt = CLikeEnum2::Small;
    //     dbg!(tshirt as i32);
    //     let tshirt = CLikeEnum2::ExtraLarge;
    //     dbg!(tshirt as i32);
    //     println!("{:?}, {:?}", CLikeEnum2::Large, CLikeEnum2::Medium);
    //     println!("CLikeEnum2 size is {}", std::mem::size_of::<CLikeEnum>());
    //
    //     let v = Variant::Nil;
    //     dbg!(&v);
    //     v.who_am_i();
    //     println!("Variant::Nil size is {}", std::mem::size_of_val(&v));
    //
    //     let v = Variant::Tuple(10, true);
    //     dbg!(&v);
    //     v.who_am_i();
    //     println!("Variant::Tuple size is {}", std::mem::size_of_val(&v));
    //
    //     let v = Variant::Struct {
    //         a: "Hello!".to_string(),
    //         b: 10.0,
    //         c: [0, 1, 2, 3, 4],
    //     };
    //     dbg!(&v);
    //     v.who_am_i();
    //     println!("Variant::Struct size is {}", std::mem::size_of_val(&v));
    //
    //     if let Variant::Struct { a, .. } = v {
    //         println!("Only a = {a}");
    //     }
    // }
    //
    // enum State {
    //     Open,
    //     Closing,
    //     Closed,
    //     Opening,
    // }
    //
    // #[test]
    // fn match_enum_simple() {
    //     fn stringfy_state(state: State) {
    //         match state {
    //             State::Open => {
    //                 println!("Open")
    //             }
    //             State::Closing => {
    //                 println!("Closing")
    //             }
    //             State::Closed => {
    //                 println!("Closed")
    //             }
    //             State::Opening => {
    //                 println!("Opening")
    //             }
    //         }
    //     }
    //
    //     stringfy_state(State::Open);
    //     stringfy_state(State::Closing);
    //     stringfy_state(State::Closed);
    //     stringfy_state(State::Opening);
    // }
    //
    // #[derive(Debug)]
    // enum Color {
    //     Black,
    //     White,
    //     RGB(u8, u8, u8),
    //     ARGB(u8, u8, u8, u8),
    //     CMYK(u8, u8, u8, u8),
    // }
    //
    // fn stringfy_color(color: Color) {
    //     match color {
    //         Color::Black
    //         | Color::RGB(0, 0, 0)
    //         | Color::ARGB(_, 0, 0, 0)
    //         | Color::CMYK(0, 0, 0, 100) => println!("Black(□)"),
    //         Color::White
    //         | Color::RGB(255, 255, 255)
    //         | Color::ARGB(_, 255, 255, 255)
    //         | Color::CMYK(0, 0, 0, 0) => println!("White(■)"),
    //         Color::RGB(255, g, b) => println!("Red is full! RGB(r=255,g={g},b={b})"),
    //         Color::RGB(r, g @ 0, b) => println!("No green at all! RGB(r={r},g={g},b={b})"),
    //         Color::RGB(r, g, b) if b > 0 => println!("It has some blue! RGB(r={r},g={g},b={b})"),
    //         Color::RGB(r, g, b) => println!("RGB(r={r},g={g},b={b})"),
    //         Color::ARGB(a, r, g, b) => println!("ARGB(a={a},r={r},g={g},b={b})"),
    //         Color::CMYK(c, m, y, k) => println!("CYMK(c={c},y={m},y={y},k={k})"),
    //     }
    // }
    //
    // #[test]
    // fn match_enum_complex() {
    //     let color = Color::Black;
    //     stringfy_color(color);
    //     let color = Color::White;
    //     stringfy_color(color);
    //     let color = Color::RGB(0, 0, 0);
    //     stringfy_color(color);
    //     let color = Color::RGB(255, 255, 255);
    //     stringfy_color(color);
    //     let color = Color::RGB(25, 1, 255);
    //     stringfy_color(color);
    //     let color = Color::RGB(254, 0, 255); // Compare com o próximo
    //     stringfy_color(color);
    //     let color = Color::RGB(255, 0, 255);
    //     stringfy_color(color);
    //     let color = Color::RGB(10, 10, 0);
    //     stringfy_color(color);
    //     let color = Color::ARGB(0xFF, 10, 10, 0);
    //     stringfy_color(color);
    //     let color = Color::CMYK(0, 0, 0, 0);
    //     stringfy_color(color);
    //     let color = Color::CMYK(0, 0, 0, 100);
    //     stringfy_color(color);
    //     let color = Color::CMYK(1, 2, 3, 4);
    //     stringfy_color(color);
    // }
    //
    // struct BankAccount {
    //     owner: String,
    //     balance: f64,
    // }
    //
    // fn show_account_status(account: BankAccount) {
    //     match account {
    //         BankAccount { owner, balance } if owner.starts_with("Jack") => {
    //             println!(
    //                 "This is the Jack's account. The balance is USD${:.02}",
    //                 balance
    //             )
    //         }
    //         BankAccount {
    //             owner: o,
    //             balance: _,
    //         } if o == String::from("") => {
    //             println!("The account is invalid")
    //         }
    //         BankAccount { owner, balance } if balance == 0.0 => {
    //             println!("The {}'s account has no balance", owner)
    //         }
    //         BankAccount { owner, balance: b } => {
    //             println!("The {}'s account balance is USD${:.02}", owner, b)
    //         }
    //     };
    // }
    //
    // #[test]
    // fn match_struct() {
    //     let acc = BankAccount {
    //         owner: String::from("Elon Musk"),
    //         balance: 228_000_000_000.0,
    //     };
    //     show_account_status(acc);
    //     let acc = BankAccount {
    //         owner: String::from("Warren Buffett"),
    //         balance: 116_000_000_000.0,
    //     };
    //     show_account_status(acc);
    //     let acc = BankAccount {
    //         owner: String::from("Jack Hat"),
    //         balance: 1050.0,
    //     };
    //     show_account_status(acc);
    //     let acc = BankAccount {
    //         owner: String::from("John Shoe"),
    //         balance: 0.0,
    //     };
    //     show_account_status(acc);
    //     let acc = BankAccount {
    //         owner: String::default(),
    //         balance: 100.0,
    //     };
    //     show_account_status(acc);
    // }
}
