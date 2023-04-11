#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

use rtt_target::{rprintln, rtt_init_print};

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("Hello, world!");

    // empty!();
    let res = double!(3);
    rprintln!("{}", res);

    macros::separators::test_separators();

    macros::patterns::test_greetings();
    macros::patterns::test_greetings_v2();

    macros::multiple_args::test_multiple_args();
    macros::multiple_args::test_mult_types();

    serde_examples::test_serde();

    loop {}
}

#[allow(unused)]
#[macro_use]
mod macros {
    use rtt_target::rprintln;

    #[macro_use]
    pub mod basic {
        macro_rules! empty {
            () => {};
        }

        #[macro_export]
        macro_rules! double {
            // $<name>:<designator>
            ($value:expr) => {
                $value * 2
            };
        }

        macro_rules! say_hello {
            () => {
                rprintln!("Hello");
            };
        }
    }

    pub mod separators {
        macro_rules! power {
            ($value:expr => $exp:expr) => {
                $value.pow($exp)
            };
        }

        macro_rules! power_v2 {
            ($value:expr, $exp:expr) => {
                $value.pow($exp)
            };
        }

        macro_rules! power_v3 {
            ($value:expr; $exp:expr) => {
                $value.pow($exp)
            };
        }

        pub fn test_separators() {
            power!(2i32 => 2);
            power_v2!(2i32, 2);
            power_v3!(2i32; 2);
        }
    }

    pub mod patterns {
        use super::*;

        macro_rules! greetings {
            (title: $title:expr, name: $name:expr) => {
                rprintln!("Hello {}. {}", $title, $name);
            };
        }

        pub fn test_greetings() {
            greetings! {
                title: "Mr.",
                name: "Matheus"
            }

            greetings![title: "Mrs.", name: "Marta"];
        }

        macro_rules! greetings_v2 {
            (Mr, $name:expr) => {
                rprintln!("Hello Mr. {}", $name);
            };
            (Mrs, $name:expr) => {
                rprintln!("Hello Mrs. {}", $name);
            };
        }

        pub fn test_greetings_v2() {
            greetings_v2![Mr, "Matheus"];
            greetings_v2!(Mrs, "Marta");
        }
    }

    pub mod multiple_args {
        use super::*;

        macro_rules! sum {
            ($first:expr, $($args:expr),+) => {};
        }

        macro_rules! sum_v2 {
            ($first:expr, $second:expr) => {
                $first + $second
            };
            ($first:expr, $($args:expr),+) => {
                $first + sum_v2!($($args),+)
            };
        }

        pub fn test_multiple_args() {
            rprintln!("{}", sum_v2!(1, 2));
            rprintln!("{}", sum_v2!(1, 2, 3));
            rprintln!("{}", sum_v2!(1, 2, 3, 4));
        }

        macro_rules! zero_or_more {
            ($($value:expr),*) => {};
        }

        macro_rules! one_or_more {
            ($($value:expr),+) => {};
        }

        macro_rules! zero_or_one {
            ($($value:expr)?) => {};
        }

        pub fn test_mult_types() {
            zero_or_more!(1, 2, 3);
            zero_or_more!(1, 2);
            zero_or_more!(1);
            zero_or_more!();

            one_or_more!(1, 2, 3);
            one_or_more!(1, 2);
            one_or_more!(1);

            zero_or_one!(1);
            zero_or_one!();
        }
    }

    mod multiple_statement {
        macro_rules! sum {
            ($first:expr, $second:expr) => {
                $first + $second
            };
            ($first:expr, $($args:expr),+) => {
                {
                    let res = sum_v2!($($args),+);
                    $first + res
                }
            };
        }
    }

    pub mod designators {
        macro_rules! expression {
            ($a:expr) => {};
        }

        fn sum(a: i32, b: i32) -> i32 {
            a + b
        }

        fn use_expression() {
            expression!("Hello");
            expression!(1 + 1);
            expression!(sum(3, 4));
        }

        macro_rules! literal {
            ($a:literal) => {};
        }

        fn use_literal() {
            literal!("123");
            literal!(123);
            literal!(true);
            literal!('a');

            // literal!(1 + 2);
        }

        macro_rules! path {
            ($a:path) => {};
        }

        fn use_path() {
            path!(hello);
            path!(this::file);
        }

        macro_rules! statement {
            ($a:stmt) => {};
        }

        fn use_statement() {
            statement! {
                let a = 1
            }

            statement!(
                struct Hello;
            );
        }

        macro_rules! block {
            ($a:block) => {};
        }

        fn use_block() {
            block! {{
                let a = 1;
                if (a > 2) {
                    35.0
                } else {
                    0.0
                }
            }}
        }

        macro_rules! types {
            ($a:ty) => {};
        }

        struct Hello;

        fn use_types() {
            types!(i32);
            types!(bool);
            types!(Hello);
            types!(());
        }

        macro_rules! identifier {
            ($a:ident) => {};
        }

        macro_rules! visibility {
            ($a:vis) => {};
            ($a:vis $name:ident) => {};
        }

        fn use_visibility() {
            visibility!(pub);
            visibility!(pub(crate));

            visibility!(pub hello);
            visibility!(hello);
        }

        macro_rules! lifetime {
            ($a:lifetime) => {};
        }

        fn use_lifetime() {
            lifetime!('a);
            lifetime!('static);
        }
    }

    macro_rules! my_struct {
        (
            $v:vis $name:ident => {
                $(
                    $field:ident<$field_type:ty>
                ),* $(,)?
            }
        )
        =>
        {
            $v struct $name {
                $($field: $field_type),*
            }
        };
    }

    my_struct! {
        pub Hello => {
            a<i32>,
            b<i32>,
        }
    }
}

mod serde_examples {
    use rtt_target::rprintln;
    use serde::{Deserialize, Serialize};

    macro_rules! println {
        ($fmt:literal) => {
            rprintln!($fmt);
        };
        ($fmt:literal, $($args:expr),+) => {
            rprintln!($fmt, $($args),+);
        };
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Packet {
        id: usize,
        name: &'static str,
        value: f32,
    }

    pub fn test_serde() {
        let pkt = Packet {
            id: 0,
            name: "Temperature",
            value: 0.3,
        };

        let pkt_json: heapless::String<30> = serde_json_core::to_string(&pkt).unwrap();
        println!("pkt: {}", pkt_json);

        let (pkt_from_str, _) =
            serde_json_core::from_str::<Packet>("{\"id\":1,\"name\":\"Acc\",\"value\":9.8}")
                .unwrap();
        println!("pkt: {:?}", pkt_from_str);
    }
}
