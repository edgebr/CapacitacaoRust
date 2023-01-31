#![feature(exclusive_range_pattern)]

extern crate core;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn blocks() {
        let a = {
            let x = 10;
            x + 100
        };
        dbg!(a);

        fn inc(val: i32) -> i32 {
            let x = val + 1;
            x
        }
        dbg!(inc(10));
    }
    #[test]
    fn blocks_simple() {
        let i: u8 = 150;
        let j: u8 = 130;
        let result = {
            let (res, ov) = i.overflowing_add(j);
            if ov {
                i
            } else {
                res
            }
        };
        dbg!(result);
    }

    #[test]
    fn labeled_break() {
        let i: u8 = 150;
        let j: u8 = 254;
        let result = 'calculation: {
            let (mut res, mut ov) = i.overflowing_add(j);
            if ov {
                (res, ov) = i.overflowing_add(j >> 1);
                dbg!(&res);
                if ov {
                    (res, ov) = i.overflowing_add(j >> 2);
                    dbg!(&res);
                    if ov {
                        break 'calculation i;
                    }
                }
            }
            res
        };
        dbg!(result);
    }

    #[test]
    fn if_literal() {
        if "" == "a" {
            println!("if");
        } else {
            println!("else");
        }
    }

    #[test]
    fn if_else() {
        let x = 10;

        let result;
        if x > 0 {
            result = "MAIOR";
        } else {
            result = "menor";
        }

        dbg!(result);
    }

    #[test]
    fn if_else_block() {
        let x = 100;

        let result = if x > 0 { "MAIOR" } else { "menor" };

        dbg!(result);
    }

    #[test]
    fn if_elseif() {
        let x = 25;

        let result;
        if x > 50 {
            result = "maior";
        } else if x > 30 {
            result = "entre";
        } else {
            result = "menor";
        }

        assert_eq!("menor", result);
    }

    #[test]
    fn if_elseif_block() {
        let x = 25;

        let result;
        if {
            let a = 50;
            x > a
        } {
            result = "maior";
        } else if x > 30 {
            result = "entre";
        } else {
            result = "menor";
        }

        assert_eq!("menor", result);
    }

    #[test]
    fn if_else_complex_block() {
        let x = 25;

        let a = 'block: {
            if x < 100 {
                if x > 80 {
                    break 'block 20;
                }
                break 'block 10;
            } else {
                5
            }
        };
        println!("a = {a}");
    }

    #[test]
    fn if_elseif_one_line() {
        let x = 25;

        let result = if x > 50 {
            "maior"
        } else if x > 30 {
            "dentro"
        } else {
            "menor"
        };

        assert_eq!("menor", result);
    }

    #[test]
    fn if_let() {
        let jack = ("Jack", '🤠');
        let john = ("John", '🙂');
        let mary = ("Mary", '👧');

        fn check_user_hat(user: (&str, char)) {
            if let (user_name, '🤠') = user {
                println!("{user_name} has a hat!");
            } else {
                println!("No hats with {}", user.0);
            }
        }
        check_user_hat(jack);
        check_user_hat(john);
        check_user_hat(mary);
    }

    #[test]
    fn if_let_simple() {
        let i: u8 = 150;
        let j: u8 = 50;
        let result = if let (res, false) = i.overflowing_add(j) {
            res
        } else {
            i
        };
        dbg!(result);
    }

    #[test]
    fn let_else() {
        let jack = ("Jack", '🤠');
        let john = ("John", '🙂');
        let mary = ("Mary", '👧');

        fn check_user_hat(user: (&str, char)) {
            let (user_name, '🤠') = user  else {
                println!("No hats with {}", user.0);
                return;
            };
            println!("{user_name} has a hat!");
        }
        check_user_hat(jack);
        check_user_hat(john);
        check_user_hat(mary);
    }

    #[test]
    fn let_else_blocks_simple() {
        let i: u8 = 150;
        let j: u8 = 130;
        let result = 'calc: {
            let (res, false) = i.overflowing_add(j) else { break 'calc i; };
            res
        };
        dbg!(result);
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
    fn while_let() {
        let mut range = 0..10;

        print!("Items: [");
        while let Some(value) = range.next() {
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
    fn match_plus_ifelse() {
        let x = 25;

        let result = match x {
            m if m > 50 => "maior",
            n if 0 <= n && n <= 30 => "entre",
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
