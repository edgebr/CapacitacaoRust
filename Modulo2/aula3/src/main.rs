#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::fmt::Formatter;

    #[test]
    ///     Lifetimes are named regions of code that a reference must be valid for.
    fn lifetime_101() {
        //id <- m -> val
        let mut x = 10;
        let y = &mut x; // char const * const y = &x;
        *y += 1; // lifetime y ended
        let z = &mut x;
        *z += 5;

        dbg!(&x);

        // *y += 1;
    }

    #[test]
    fn lifetime_102() {
        fn inc(x: &mut i32) {
            *x += 1
        }

        let mut x = 1;
        inc(&mut x);
        inc(&mut x);
        inc(&mut x);
        inc(&mut x);
        let y = &x;

        dbg!(y);
    }

    ///
    /// The compiler uses three rules to figure out the lifetimes:
    ///
    /// 1. The compiler assigns a lifetime parameter to each parameter that’s a reference.
    /// 1. If there is exactly one input lifetime parameter, that lifetime is assigned to all output
    /// lifetime parameters.
    /// 1. If there are multiple input lifetime parameters, but one of them is &self or &mut self
    /// because this is a method, the lifetime of self is assigned to all output lifetime parameters.
    ///
    #[test]
    fn lifetime_elision_rules() {
        // input lifetimes ──────────┬──────────────┬──────────────╮
        fn rule1(arg1: &str, arg2: &str, arg3: &i32) {}

        //   input lifetimes ───────────╮        all output lifetime ──┬────────╮
        fn rule2(arg1: i32, arg2: &i32, arg3: bool) -> Result<&str, &str> {
            Ok("Hi")
        }

        println!("{:?}", rule2(10, &10, false));

        struct Rule3;
        impl Rule3 {
            //input lifetimes ─────┬───────────────┬──────────────╮all op lifetime ──┬────────╮
            fn rule3(&self, arg1: &str, arg2: &i32) -> Result<&str, &i32> {
                Err(&10)
            }
        }
        let a = Rule3;
        println!("{:?}", a.rule3("Hi", &10));
    }

    #[test]
    fn lifetime_longest() {
        // //     input lifetimes ──┬────────────╮           ╭───── output lifetime
        fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
            if s1.len() < s2.len() {
                s2
            } else {
                s1 // greater or equal
            }
        }
        dbg!(longest("Cebola", "Alho"));
    }

    #[test]
    fn lifetime_static() {
        static ME: &str = "test";
        let me: &'static str = "Test"; // const char * name = "Test";
        fn name() -> &'static str {
            "Jack"
        }
        let a = name();
        dbg!(a);
    }

    #[test]
    fn lifetime_struct() {
        #[derive(Debug)]
        pub struct FullName<'b> {

            first_name: &'static str, // (&'static)"Hello"
            last_name: &'b str, //&String:: "Hello" + ...
        }
        impl<'b> FullName<'b> {
            pub fn new(first_name: &'static str, last_name: &'b str) -> FullName<'b> {
                FullName { first_name, last_name }
            }
        }

        #[derive(Debug)]
        struct BankAccount<'a> {
            full_name: &'a FullName<'a>,
            balance: f64,
        }
        impl<'a> BankAccount<'a> {
            pub fn new<'b>(greetings: &'b str, full_name: &'a FullName<'a>, balance: f64) -> BankAccount<'a> {
                println!("{greetings} {} {}", full_name.first_name, full_name.last_name);
                Self { full_name, balance }
            }
            pub fn full_name(&'a self) -> &'a FullName {
                self.full_name
            }
            pub fn balance(&self) -> f64 {
                self.balance
            }
            pub fn set_full_name<'b: 'a>(&mut self, full_name: &'b FullName<'b>) {
                self.full_name = full_name;
            }
            pub fn set_balance(&mut self, balance: f64) {
                self.balance = balance;
            }
        }
        let mary = FullName { first_name: "Mary", last_name: "Good" };
        let mut mary_account = BankAccount {
            full_name: &mary,
            balance: 100.0,
        };
        dbg!(&mary_account);

        let joe = FullName::new("Joe", "Bridge");

        let joe_account = BankAccount::new(&String::from("Welcome"), &joe, 100.0);
        dbg!(&joe_account);

        let married_mary = FullName::new("Mary", "Good-bridge");
        {
            mary_account.set_full_name(&married_mary);
        }
        dbg!(mary_account);
    }

    #[test]
    fn lifetime_enum() {
        #[derive(Debug)]
        struct Message<'a>(&'a str);

        impl<'a> std::fmt::Display for Message<'a> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        #[derive(Debug)]
        enum Result<'b> {
            Fail { reason: &'b Message<'b> },
            Ok { message: &'b Message<'b> },
        }

        impl<'b> Result<'b> {
            pub fn new_ok<'a, 'c: 'a>(message: &'c Message) -> Result<'a> {
                Result::Ok { message }
            }
            pub fn new_fail<'a, 'c: 'a>(reason: &'c Message) -> Result<'a> {
                Result::Fail { reason }
            }
            pub fn show(&self) {
                match self {
                    Result::Ok { message } => println!("{message}"),
                    Result::Fail { reason } => println!("{reason}")
                }
            }
        }

        static SUCCESS_MESSAGE: Message = Message("That's great! Success!");
        static FAIL_MESSAGE: Message = Message("Failed! Try again...");

        fn first_is_bigger_than_second(first: i32, second: i32) -> Result<'static> {
            if first > second {
                Result::new_ok(&SUCCESS_MESSAGE)
            } else {
                Result::new_fail(&FAIL_MESSAGE)
            }
        }

        first_is_bigger_than_second(3, 5).show();
        first_is_bigger_than_second(2, 0).show();

        fn first_is_smaller_than_second(first: i32, second: i32) -> Result<'static> {
            if first < second {
                Result::new_ok(&SUCCESS_MESSAGE)
            } else {
                Result::new_fail(&FAIL_MESSAGE)
            }
        }

        first_is_smaller_than_second(3, 5).show();
        first_is_smaller_than_second(2, 0).show();
    }

    mod person {
        #[derive(Clone, Debug)]
        pub struct Person {
            name: String,
        }

        impl Person {
            pub fn new(name: &str) -> Self {
                Self {
                    name: name.to_string(),
                }
            }

            pub fn get_name(&self) -> &str {
                &self.name
            }

            pub fn set_name(&mut self, name: &str) {
                self.name = name.to_string();
            }

            pub fn say_goodbye(self) {
                println!("Goodbye {}", self.name);
            }

            pub fn add_mister(mut self) -> Self {
                self.name = format!("Mr. {}", self.name);
                self
            }
        }
    }

    #[test]
    fn valid_func_mut_ref() {
        use person::Person;

        fn set_to_marcos(p2: &mut Person, old_name: &str) {
            p2.set_name("Marcos");
            println!("Changing from {old_name} to {}", p2.get_name());
        }

        let mut p = Person::new("Matheus");

        /* 1st Solution */
        let name = p.get_name().to_string();
        set_to_marcos(&mut p, &name);

        /* 2nd Solution */
        // set_to_marcos(&mut p, &p.get_name().to_string());
    }
}
