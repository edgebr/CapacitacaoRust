#![feature(never_type)]

#[allow(unused)]
mod generics {
    mod const_generics {
        #[derive(Debug)]
        struct PersonRef<'a> {
            name: String,
            friends: &'a [String],
        }

        impl<'a> PersonRef<'a> {
            pub fn new(name: &str, friends: &'a [String]) -> Self {
                Self {
                    name: name.to_string(),
                    friends,
                }
            }
        }

        #[test]
        fn test_person_ref() {
            let friends = ["Marcos".to_string(), "Lucas".to_string()];
            let person = PersonRef::new("Matheus", &friends);

            dbg!(person);
        }

        #[derive(Debug)]
        struct Person<const N: usize> {
            name: String,
            friends: [String; N],
        }

        impl<const N: usize> Person<N> {
            pub fn new(name: &str, friends: [String; N]) -> Self {
                Self {
                    name: name.to_string(),
                    friends,
                }
            }
        }

        #[test]
        fn test_person_const_generic() {
            let person = Person::new("Matheus", ["Marcos".to_string(), "Lucas".to_string()]);
            dbg!(person);

            let person = Person::new(
                "Matheus",
                [
                    "Marcos".to_string(),
                    "Lucas".to_string(),
                    "João".to_string(),
                    "Pedro".to_string(),
                    "Paulo".to_string(),
                ],
            );
            dbg!(person);
        }

        impl Person<0> {
            pub fn print_friends(&self) {
                println!("No friends at all");
            }
        }

        #[test]
        fn test_no_friends() {
            let person = Person::new("Matheus", []);

            person.print_friends();
            dbg!(person);
        }
    }

    mod generic_and_lifetimes {
        use std::fmt::Debug;

        #[derive(Debug)]
        struct PersonRef<'a> {
            name: String,
            friends: &'a [String],
        }

        impl<'a> PersonRef<'a> {
            pub fn new(name: &str, friends: &'a [String]) -> Self {
                Self {
                    name: name.to_string(),
                    friends,
                }
            }
        }

        impl PersonRef<'static> {
            pub fn static_friends(&self) -> &'static [String] {
                &self.friends
            }
        }

        fn print_slices<'a, T: Debug + PartialEq>(first: &'a [T], second: &'a [T]) {
            print!("[");
            for el in first {
                print!("{el:?},");
            }
            for el in second {
                print!(
                    "{el:?}{}",
                    if el == second.last().unwrap() {
                        ""
                    } else {
                        ","
                    }
                );
            }
            println!("]");
        }

        // fn invalid_print_slices<T: Debug, 'a>(first: &'a [T], second: &'a [T]) {
        //     println!("[");
        //     for el in first {
        //         print!("{el:?},");
        //     }
        //     for el in second {
        //         print!("{el:?},");
        //     }
        //     println!("]");
        // }

        #[test]
        fn test_print_slices() {
            let l1 = [1, 2, 3];
            let l2 = [2, 3];

            print_slices(&l1, &l2);
        }

        fn cat_slices<'a, const N: usize>(first: &'a [i32], second: &'a [i32]) -> Vec<i32> {
            let mut first_vec = first.to_vec();
            first_vec.extend(second.iter());
            first_vec
        }

        // fn invalid_cat_slices<const N: usize, 'a>(first: &'a [i32], second: &'a [i32]) -> Vec<i32> {
        //     let mut first_vec = first.to_vec();
        //     first_vec.extend(second.iter());
        //     first_vec
        // }

        #[test]
        fn test_cat_slices() {
            let l1 = [1, 2, 3];
            let l2 = [2, 3];

            let l3 = cat_slices::<5>(&l1, &l2);
            dbg!(l3);

            // let l3 = cat_slices::<l1.len() + l2.len()>(&l1, &l2);

            // const SIZE: usize = <[i32]>::len(&l1);
            // let l3 = cat_slices::<SIZE>(&l1, &l2);
            // dbg!(l3);
        }
    }

    mod phantom_types {
        use std::marker::PhantomData;

        struct Low;

        struct Medium;

        struct High;

        struct Heater<S> {
            _marker: std::marker::PhantomData<S>,
        }

        impl Heater<Low> {
            pub fn add_water(self) -> Heater<Medium> {
                Heater {
                    _marker: PhantomData,
                }
            }
        }

        impl Heater<Medium> {
            pub fn drain(self) -> Heater<Low> {
                Heater {
                    _marker: PhantomData,
                }
            }

            pub fn fill(self) -> Heater<High> {
                Heater {
                    _marker: PhantomData,
                }
            }
        }

        impl Heater<High> {
            pub fn remove_water(self) -> Heater<Medium> {
                Heater {
                    _marker: PhantomData,
                }
            }
        }

        fn run_heater<S>(h: &mut Heater<S>) {
            todo!()
        }

        fn refill_heater(h: &mut Heater<Low>) {
            todo!()
        }

        /// TODO: Após Traits
        mod driver {}
    }

    /// TODO: Após Traits
    mod bounds {}

    /// TODO: Após Traits
    mod multiple_bounds {}

    /// TODO: Após Traits
    mod where_clause {}
}

mod trivia {
    use std::time::Duration;

    #[test]
    #[allow(unreachable_code)]
    #[allow(unused_variables)]
    fn never_type() {
        let a: ! = loop {};
        let p: ! = panic!();
    }

    #[allow(unused)]
    fn blink() -> ! {
        let val = false;
        loop {
            let val = !val;

            set_led(val);
            delay(500);
        }
    }

    fn set_led(state: bool) {
        println!("Led {}", if state { "On" } else { "Off" });
    }

    #[inline(always)]
    fn delay(ms: u64) {
        std::thread::sleep(Duration::from_millis(ms));
    }

    #[test]
    #[allow(unused)]
    fn enum_zero_variant() {
        enum ZeroVariant {}

        // let no_variant = ???;
    }
}

fn main() {
    println!("Hello, world!");
}
