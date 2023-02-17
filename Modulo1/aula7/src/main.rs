mod bank;

fn main() {
    println!("Aula 07!");
}

#[derive(Debug)]
enum CLikeEnum {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Debug)]
enum CLikeEnum2 {
    Small = 1000,
    Medium,
    Large,
    ExtraLarge,
}

/// Variant demonstra um Enum com tamanho variado. Sendo esse um tipo unitário, uma tupla e uma
/// estrutura.
///
/// Para verificar o layout do `enum Variant` é necessário executar um commando especial
/// [Performance book](https://nnethercote.github.io/perf-book/type-sizes.html).
/// O commando `rustc +nightly -Zprint-type-sizes ./src/main.rs` mostra com detalhes o layout dos
/// tipos definidos no arquivo `main.rs`.
///
/// # Execução
///
/// ```console
/// prompt> rustc +nightly -Zprint-type-sizes ./src/main.rs
/// print-type-size type: `Variant`: 56 bytes, alignment: 8 bytes
/// print-type-size     discriminant: 1 bytes
/// print-type-size     variant `Struct`: 55 bytes
/// print-type-size         padding: 3 bytes
/// print-type-size         field `.b`: 4 bytes, alignment: 4 bytes
/// print-type-size         field `.c`: 20 bytes
/// print-type-size         padding: 4 bytes
/// print-type-size         field `.a`: 24 bytes, alignment: 8 bytes
/// print-type-size     variant `Tuple`: 7 bytes
/// print-type-size         field `.1`: 1 bytes
/// print-type-size         padding: 2 bytes
/// print-type-size         field `.0`: 4 bytes, alignment: 4 bytes
/// print-type-size     variant `Nil`: 0 bytes
/// ...
/// ```
/// ### Análise
///
/// | Nome    | Alinhamento | Composição                                                                                              | Total |
/// |---------|-------------|---------------------------------------------------------------------------------------------------------|-------|
/// | Struct  | 8           | { 1 (discriminant), 3 (padding) , 4 } + { 4, 4 } + { 4, 4 } + {4, 4 (padding)} + { 8 } + { 8 } + { 8 }  | 56    |
/// | Tuple   | 4           | { 1 (discriminant), 1, 2 (padding) } + { 4 }                                                            | 8     |
/// | Nil     | 1           | { 1 (discriminant) }                                                                                    | 1     |
///
#[derive(Debug)]
enum Variant {
    /// Unit struct que indica vazio.
    Nil,
    /// Tuple struct que indica um par inteiro de 32 bist e um booleano.
    Tuple(i32, bool),
    /// Struct convencional.
    Struct {
        a: String,
        b: f32,
        c: Box<[i32; 50]>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bank::BankAccount;

    impl Variant {
        fn who_am_i(&self) {
            match self {
                Variant::Nil => println!("I am Nil"),
                Variant::Tuple(a, b) => println!("I am a the variant tuple ({a},{b})"),
                Variant::Struct { a, b, c } => {
                    println!("I am a the variant struct {{ {a}, {b}, {:?}}} ", c)
                }
            }
        }
    }

    #[test]
    fn enum_simple() {
        let c_like = CLikeEnum::Second;
        dbg!(c_like as u8);
        let c_like = CLikeEnum::Fourth;
        dbg!(c_like as i32);
        println!("{:?}, {:?}", CLikeEnum::First, CLikeEnum::Third);
        println!("CLikeEnum size is {}", std::mem::size_of::<CLikeEnum>());

        let tshirt = CLikeEnum2::Small;
        dbg!(tshirt as i32);
        let tshirt = CLikeEnum2::ExtraLarge;
        dbg!(tshirt as i32);
        println!("{:?}, {:?}", CLikeEnum2::Large, CLikeEnum2::Medium);
        println!("CLikeEnum2 size is {}", std::mem::size_of::<CLikeEnum2>());

        let v = Variant::Nil;
        dbg!(&v);
        v.who_am_i();
        println!("Variant::Nil size is {}", std::mem::size_of_val(&v));

        let v = Variant::Tuple(10, true);
        dbg!(&v);
        v.who_am_i();
        println!("Variant::Tuple size is {}", std::mem::size_of_val(&v));

        let v = Variant::Struct {
            a: "Hello!".to_string(), // String::from("Hello!") // "Hello!".into()
            b: 10.0,
            c: Box::new([0; 50]),
        };
        dbg!(&v);
        v.who_am_i();
        println!("Variant::Struct size is {}", std::mem::size_of_val(&v));

        if let Variant::Struct { a, .. } = v {
            println!("Only a = {a}");
        }
    }

    enum State {
        Open,
        Closing,
        Closed,
        Opening,
    }

    #[test]
    fn match_enum_simple() {
        fn stringfy_state(state: State) {
            match state {
                State::Open => {
                    println!("Open")
                }
                State::Closing => {
                    println!("Closing")
                }
                State::Closed => {
                    println!("Closed")
                }
                State::Opening => {
                    println!("Opening")
                }
            }
        }

        stringfy_state(State::Open);
        stringfy_state(State::Closing);
        stringfy_state(State::Closed);
        stringfy_state(State::Opening);
    }

    #[derive(Debug)]
    enum Color {
        Black,
        White,
        RGB(u8, u8, u8),
        ARGB(u8, u8, u8, u8),
        CMYK(u8, u8, u8, u8),
    }

    fn stringfy_color(color: Color) {
        match color {
            Color::Black
            | Color::RGB(0, 0, 0)
            | Color::ARGB(_, 0, 0, 0)
            | Color::CMYK(0, 0, 0, 100) => println!("Black(□)"),
            Color::White
            | Color::RGB(255, 255, 255)
            | Color::ARGB(_, 255, 255, 255)
            | Color::CMYK(0, 0, 0, 0) => println!("White(■)"),
            Color::RGB(255, g, b) => println!("Red is full! RGB(r=255,g={g},b={b})"),
            Color::RGB(r, g @ 0, b) => println!("No green at all! RGB(r={r},g={g},b={b})"),
            Color::RGB(r, g, b) if b > 0 => println!("It has some blue! RGB(r={r},g={g},b={b})"),
            Color::RGB(r, g, b) => println!("RGB(r={r},g={g},b={b})"),
            Color::ARGB(a, r, g, b) => println!("ARGB(a={a},r={r},g={g},b={b})"),
            Color::CMYK(c, m, y, k) => println!("CYMK(c={c},y={m},y={y},k={k})"),
        }
    }

    #[test]
    fn match_enum_complex() {
        let color = Color::Black;
        stringfy_color(color);
        let color = Color::White;
        stringfy_color(color);
        let color = Color::RGB(0, 0, 0);
        stringfy_color(color);
        let color = Color::RGB(255, 255, 255);
        stringfy_color(color);
        let color = Color::RGB(25, 1, 255);
        stringfy_color(color);
        let color = Color::RGB(254, 0, 255); // Compare com o próximo
        stringfy_color(color);
        let color = Color::RGB(255, 0, 255);
        stringfy_color(color);
        let color = Color::RGB(10, 10, 0);
        stringfy_color(color);
        let color = Color::ARGB(0xFF, 10, 10, 0);
        stringfy_color(color);
        let color = Color::CMYK(0, 0, 0, 0);
        stringfy_color(color);
        let color = Color::CMYK(0, 0, 0, 100);
        stringfy_color(color);
        let color = Color::CMYK(1, 2, 3, 4);
        stringfy_color(color);
    }

    fn show_account_status(account: &BankAccount) {
        match account {
            BankAccount { owner, balance } if owner.starts_with("Jack") => {
                println!(
                    "This is the Jack's account. The balance is USD${:.02}",
                    balance
                )
            }
            BankAccount {
                owner: o,
                balance: _,
            } if *o == String::from("") => {
                println!("The account is invalid")
            }
            BankAccount { owner, balance } if *balance == 0.0 => {
                println!("The {}'s account has no balance", owner)
            }
            BankAccount { owner, balance: b } => {
                println!("The {}'s account balance is USD${:.02}", owner, b)
            }
        };
    }

    #[test]
    fn match_struct() {
        let acc = BankAccount {
            owner: String::from("Elon Musk"),
            balance: 228_000_000_000.0,
        };
        show_account_status(&acc);
        dbg!(&acc);

        let acc = BankAccount::builder()
            .owner("Warren Buffett")
            .balance(116_000_000_000.0)
            .build();
        show_account_status(&acc);

        let acc = BankAccount {
            owner: String::from("Jack Hat"),
            balance: 1050.0,
        };
        show_account_status(&acc);

        let acc = BankAccount {
            owner: String::from("John Shoe"),
            balance: 0.0,
        };
        show_account_status(&acc);

        let acc = BankAccount {
            owner: String::default(),
            balance: 100.0,
        };
        show_account_status(&acc);
    }

    #[test]
    fn builder_example() {
        let a = BankAccount::builder()
            .balance(35000.0)
            .owner("James")
            .build();
        dbg!(&a);
    }
}
