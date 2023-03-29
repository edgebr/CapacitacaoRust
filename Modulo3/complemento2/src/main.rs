#![feature(associated_type_defaults)]

fn main() {
    println!("Hello, world!");
}

mod result {
    use std::error::Error;
    use std::fmt::{Display, Formatter};
    use std::num::ParseFloatError;

    #[test]
    fn result_101() {
        fn my_checked_div(a: f32, b: f32) -> Result<f32, String> {
            if b == 0.0 {
                Err(format!("b has a prohibited value: {b:.02}"))
            } else if a == 0.0 {
                Err(format!("Ineffective operation. a has the value {a:.02}."))
            } else {
                Ok(a / b)
            }
        }

        let (x, y) = (5.0, 2.5);
        if let Ok(result) = my_checked_div(x, y) {
            println!("{x:.02} / {y:.02} = {result:.02}")
        }

        for (a, b) in [(0.0, 0.0), (0.0, -10.1), (100.1, 0.0), (-230.1, 40.7)] {
            match my_checked_div(a, b) {
                Ok(result) => {
                    println!("{a:.02} / {b:.02} = {result:.02}")
                }
                Err(err_msg) => println!("{err_msg}"),
            }
        }

        let a: Result<i32, i32> = Ok(1);
        let b: Result<i32, i32> = Ok(2);
        dbg!(a.and(b).unwrap());
        dbg!(a.or(b).unwrap());
        let a: Result<i32, i32> = Err(1);
        let b: Result<i32, i32> = Err(2);
        let _ = dbg!(a.and(b));
        let _ = dbg!(a.or(b));

        let a: Result<Option<i32>, i32> = Ok(Some(1));
        dbg!(&a);
        dbg!(a.transpose());
    }
    #[test]
    fn result_match() {
        fn str_multiply(a: &str, b: &str) -> Result<f32, ParseFloatError> {
            let fa = match a.parse::<f32>() {
                Ok(x) => x,
                Err(e) => return Err(e),
            };

            let fb = match b.parse::<f32>() {
                Ok(x) => x,
                Err(e) => return Err(e),
            };

            Ok(fa * fb)
        }
        let _ = dbg!(str_multiply("5.6", "17.9"));
        let _ = dbg!(str_multiply("56", "17.9"));
        let _ = dbg!(str_multiply("A5.6", "17.9"));
        let _ = dbg!(str_multiply("5.6", ""));
        let _ = dbg!(str_multiply("56", "17,9"));
    }

    #[test]
    fn result_question_mark() {
        fn str_multiply_qm(a: &str, b: &str) -> Result<f32, ParseFloatError> {
            // let fa = match a.parse::<f32>() {
            //     Ok(x) => x,
            //     Err(e) => return Err(e),
            // };
            let fa = a.parse::<f32>()?;

            let fb = b.parse::<f32>()?;

            Ok(fa * fb)
        }
        let _ = dbg!(str_multiply_qm("5.6", "17.9"));
        let _ = dbg!(str_multiply_qm("56", "17.9"));
        let _ = dbg!(str_multiply_qm("A5.6", "17.9"));
        let _ = dbg!(str_multiply_qm("5.6", ""));
        let _ = dbg!(str_multiply_qm("56", "17,9"));
    }

    #[test]
    fn result_error_propagation() {
        fn str_div_qm(a: &str, b: &str) -> Result<f32, ParseFloatError> {
            let fa = a.parse::<f32>()?;
            let fb = b.parse::<f32>()?;
            Ok(fa / fb)
        }
        let _ = dbg!(str_div_qm("5.6", "1.79"));
        let _ = dbg!(str_div_qm("56", "17.9"));
        let _ = dbg!(str_div_qm("A5.6", "17.9"));
        let _ = dbg!(str_div_qm("5.6", ""));
        let _ = dbg!(str_div_qm("56", "17,9"));
        let _ = dbg!(str_div_qm("56", "0"));

        #[derive(Debug, Default)]
        struct MyZeroDivisionError {}

        impl Display for MyZeroDivisionError {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "MyZeroDivisionError, you cannot divide by zero")
            }
        }
        impl Error for MyZeroDivisionError {}

        fn str_checked_div_qm(a: &str, b: &str) -> Result<f32, Box<dyn Error>> {
            let fa = a.parse::<f32>()?;
            let fb = b.parse::<f32>()?;
            if fb == 0.0 {
                return Err(Box::new(MyZeroDivisionError::default()));
            }
            Ok(fa / fb)
        }
        match str_checked_div_qm("56", "0") {
            Ok(res) => println!("result = {res}"),
            Err(e) => {
                let error_description: String;
                if let Some(e) = e.downcast_ref::<ParseFloatError>() {
                    error_description = e.to_string();
                } else if let Some(e) = e.downcast_ref::<MyZeroDivisionError>() {
                    error_description = e.to_string();
                } else {
                    error_description = "".to_string();
                }
                println!("Error happened: {error_description}");
            }
        }
        match str_checked_div_qm("56", "x") {
            Ok(res) => println!("result = {res}"),
            Err(e) => {
                let error_description: String;
                if let Some(e) = e.downcast_ref::<ParseFloatError>() {
                    error_description = e.to_string();
                } else if let Some(e) = e.downcast_ref::<MyZeroDivisionError>() {
                    error_description = e.to_string();
                } else {
                    error_description = "".to_string();
                }
                println!("Error happened: {error_description}");
            }
        }
    }
    #[test]
    fn result_error_propagation_improved() {
        #[derive(Debug)]
        enum MyCheckedDivErrorKind {
            Parse(MyCheckedDivOperand),
            ZeroDivision,
        }
        #[derive(Debug)]
        enum MyCheckedDivOperand {
            First,
            Second,
        }
        impl Display for MyCheckedDivOperand {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    match self {
                        MyCheckedDivOperand::First => "first",
                        MyCheckedDivOperand::Second => "second",
                    }
                )
            }
        }

        #[derive(Debug)]
        struct MyCheckedDivError {
            kind: MyCheckedDivErrorKind,
        }
        impl MyCheckedDivError {
            pub fn kind(&self) -> &MyCheckedDivErrorKind {
                &self.kind
            }
            pub fn parse_error(operand: MyCheckedDivOperand) -> Self {
                Self {
                    kind: MyCheckedDivErrorKind::Parse(operand),
                }
            }
            pub fn zero_division_error() -> Self {
                Self {
                    kind: MyCheckedDivErrorKind::ZeroDivision,
                }
            }
        }

        impl Display for MyCheckedDivError {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self.kind {
                    MyCheckedDivErrorKind::Parse(ref op) => {
                        write!(f, "MyCheckedDivError, the {op} operand is invalid",)
                    }
                    MyCheckedDivErrorKind::ZeroDivision => {
                        write!(f, "MyCheckedDivError, you cannot divide by zero")
                    }
                }
            }
        }

        impl Error for MyCheckedDivError {}

        fn str_checked_div_ue(a: &str, b: &str) -> Result<f32, MyCheckedDivError> {
            let Ok(fa) = a.parse::<f32>() else {
                return Err(MyCheckedDivError::parse_error(MyCheckedDivOperand::First))
            };
            let Ok(fb) = b.parse::<f32>() else {
                return Err(MyCheckedDivError::parse_error(MyCheckedDivOperand::Second))
            };
            if fb == 0.0 {
                return Err(MyCheckedDivError::zero_division_error());
            }
            Ok(fa / fb)
        }

        let check_error_closure = |err: &MyCheckedDivError| match err.kind() {
            MyCheckedDivErrorKind::Parse(ref op) => {
                println!(
                    "Try check the {op} operand value. That is invalid! Error message: {}",
                    err.to_string()
                )
            }
            MyCheckedDivErrorKind::ZeroDivision => println!(
                "You cannot divide by zero! Error message: {}",
                err.to_string()
            ),
        };

        match str_checked_div_ue("56", "0") {
            Ok(res) => println!("result = {res}"),
            Err(e) => check_error_closure(&e),
        }
        match str_checked_div_ue("56", "X0") {
            Ok(res) => println!("result = {res}"),
            Err(e) => check_error_closure(&e),
        }
    }

    #[test]
    fn panics() {
        // panic!("I wanted to panic here!");
        // unreachable!("This code could not execute!");
        // unimplemented!("Some day we will implement that");
        todo!("We are going to implement that in this commit");
    }
}
