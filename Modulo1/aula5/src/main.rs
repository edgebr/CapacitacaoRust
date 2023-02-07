#![feature(exclusive_range_pattern)]
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn while_c_like() {
        let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let mut i = 0;

        print!("Items: [");
        while i < arr.len() {
            print!("{}, ", arr[i]);
            i += 1;
        }
        println!("]");
    }

    #[test]
    fn while_simple() {
        let mut slice: &[u8] = &[6, 7, 8, 9, 5, 4, 3, 2, 1];
        print!("Items: [");
        while slice.len() > 0 {
            print!("{}, ", slice[0]);
            slice = &slice[1..];
        }
        println!("]");
    }

    #[test]
    fn while_let() {
        let slice: &[u8] = &[6, 7, 8, 9, 5, 4, 3, 2, 1];
        let mut range = 0..slice.len();

        print!("Items: [");
        while let Some(i) = range.next() {
            print!("{}, ", slice[i]);
        }
        println!("]");
    }

    // enum Option {
    //     None,
    //     Some(i32),
    // }

    #[test]
    fn while_let_iter() {
        let slice: &[u8] = &[6, 7, 8, 9, 5, 4, 3, 2, 1];
        let mut iter = slice.into_iter();

        print!("Items: [");
        while let Some(value) = iter.next() {
            print!("{}, ", value);
        }
        println!("]");
    }

    #[test]
    fn for_simple() {
        print!("Items: [");
        for i in 0..10 {
            print!("{i}, ");
        }
        println!("]");

        print!("Items: [");
        for i in String::from("hello world").chars() {
            print!("{:#?}, ", i);
        }
        println!("]");

        print!("Items: [");
        for i in [10, 20, 30, 40] {
            print!("{i}, ");
        }
        println!("]");

        let v: Vec<i64> = vec![6, 7, 8, 9, 5, 4, 3, 2, 1];
        print!("Items: [");
        for i in v {
            print!("{i}, ");
        }
        println!("]");
    }

    #[test]
    fn loop_simple() {
        let mut i = 0;

        print!("Items: [");

        loop {
            print!("{i}, ");

            if i == 10 {
                break;
            } else {
                i += 1;
            }
        }
        println!("]");
    }

    #[test]
    fn loop_complex() {
        let mut i = 0;

        print!("Items: [");

        'outer_loop: loop {
            loop {
                print!("{i}, ");

                if i == 10 {
                    break 'outer_loop;
                } else {
                    i += 1;
                }
            }
        }
        println!("]");
    }

    /// Switch like function in C
    fn u8_to_text_pt_br(val: u8) -> String {
        let pt_br_text;

        match val {
            // like switch
            1 => pt_br_text = String::from("um"),
            2 => pt_br_text = String::from("dois"),
            3 => pt_br_text = String::from("três"),
            _ => pt_br_text = String::from("outro"),
        };

        return pt_br_text;
    }

    /// Match function in Rust
    fn u8_to_text_pt_br_rs(val: u8) -> String {
        match val {
            1 => String::from("um"),
            2 => String::from("dois"),
            3..7 => String::from("entre três e seis"),
            7..=9 => String::from("entre sete e nove"),
            10 | 100 => String::from("Dez ou cem"),
            _ => String::from("outro"),
        }
    }
    #[test]
    fn match_simple() {
        assert_eq!("um", u8_to_text_pt_br(1));
        assert_eq!("três", u8_to_text_pt_br(3));
        assert_eq!("outro", u8_to_text_pt_br(8));
        assert_eq!("outro", u8_to_text_pt_br(0));
        assert_eq!("um", u8_to_text_pt_br_rs(1));
        assert_eq!("entre três e seis", u8_to_text_pt_br_rs(3));
        assert_eq!("entre sete e nove", u8_to_text_pt_br_rs(8));
        assert_eq!("Dez ou cem", u8_to_text_pt_br_rs(100));
    }

    #[test]
    fn match_tuple() {
        let jack = ("Jack", '🤠');
        let john = ("John", '🙂');
        let mary = ("Mary", '👧');

        fn check_user_hat(user: (&str, char)) {
            match user {
                (user_name, '🤠') => println!("{user_name} has a hat!"),
                user_has_no_hat => println!("No hats with {}", user_has_no_hat.0),
            };
        }

        check_user_hat(jack);
        check_user_hat(john);
        check_user_hat(mary);
    }

    #[test]
    fn match_guards() {
        let x = 25;

        let result = match x {
            m if m > 30 => "maior",
            n if 20 <= n && n <= 30 => "entre",
            _ => "menor",
        };

        assert_eq!("entre", result);
    }

    #[test]
    fn match_binding() {
        let age = 75;

        let result = match age {
            a @ 0..=19 => format!("Jovem de {a}"),
            a @ 20..60 => format!("Adulto de {a}"),
            a @ 75 => format!("Aposentado de {a}"),
            a => format!("Idoso de {a}"),
        };

        println!("{result} anos é destaque na matéria.");
    }

    #[test]
    fn match_arrays() {
        let list: &[u8] = &[1, 2, 3];

        match list {
            [first, second, ..] => println!("First: {first}, second: {second}"),
            [first] => println!("Only first: {first}"),
            [] => println!("No items"),
        }
    }

    #[test]
    fn match_arrays_functional() {
        let mut list: &[u8] = &[1, 2, 3, 4, 5, 6, 7];

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
                _ => {
                    unreachable!()
                }
            }
        }
    }

    #[test]
    fn match_arrays_functional_sum() {
        let mut list: &[u8] = &[1, 2, 3, 3, 4, 6, 6];

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

    struct BankAccount {
        owner: String,
        balance: f64,
    }

    fn show_account_status(account: BankAccount) {
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
            } if o == String::from("") => {
                println!("The account is invalid")
            }
            BankAccount { owner, balance } if balance == 0.0 => {
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
        show_account_status(acc);
        let acc = BankAccount {
            owner: String::from("Warren Buffett"),
            balance: 116_000_000_000.0,
        };
        show_account_status(acc);
        let acc = BankAccount {
            owner: String::from("Jack Hat"),
            balance: 1050.0,
        };
        show_account_status(acc);
        let acc = BankAccount {
            owner: String::from("John Shoe"),
            balance: 0.0,
        };
        show_account_status(acc);
        let acc = BankAccount {
            owner: String::default(),
            balance: 100.0,
        };
        show_account_status(acc);
    }
}
