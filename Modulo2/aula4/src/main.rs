/// !!! RECAP !!!
///
/// ![Move-Copy-Borrow](https://rufflewind.com/img/rust-move-copy-borrow.png)
///
/// 1. Inifinitas referências IMUTÁVEIS (&) ao mesmo tempo.
/// 2. EXTATAMENTE 1 referência MUTÁVEL (&mut) ao mesmo tempo.
#[allow(unused)]
mod borrow_checker {
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

    mod iterator_invalidation {
        use crate::borrow_checker::person::Person;
        use std::time::Duration;

        #[test]
        fn invalid_for_loop() {
            // let mut people = vec![
            //     Person::new("Matheus"),
            //     Person::new("Marcos"),
            //     Person::new("Lucas"),
            //     Person::new("João"),
            // ];
            //
            // fn remove_lucas(people: &mut Vec<Person>) {
            //     let mut counter = 0;
            //
            //     for person in people.iter() {
            //         if person.get_name() == "Lucas" {
            //             people.remove(counter);
            //         }
            //
            //         counter += 1;
            //     }
            // }
            //
            // remove_lucas(&mut people);
        }

        #[test]
        fn valid_for_loop() {
            let mut people = vec![
                Person::new("Matheus"),
                Person::new("Marcos"),
                Person::new("Lucas"),
                Person::new("João"),
            ];
            let mut people2 = people.clone();
            let people3 = people.clone();

            fn remove_lucas_1(people: &mut Vec<Person>) {
                let mut counter = 0;
                let mut index_to_del = 0xFF_FF_FF_FF;

                for person in people.iter() {
                    if person.get_name() == "Lucas" {
                        index_to_del = counter;
                        break;
                    }

                    counter += 1;
                }

                if index_to_del != 0xFF_FF_FF_FF {
                    people.remove(index_to_del);
                }
            }

            fn remove_lucas_2(people: &mut Vec<Person>) {
                if let Some(lucas_pos) = people.iter().position(|x| x.get_name() == "Lucas") {
                    people.remove(lucas_pos);
                }
            }

            fn remove_lucas_3(people: Vec<Person>) -> Vec<Person> {
                people
                    .into_iter()
                    .filter(|x| x.get_name() != "Lucas")
                    .collect()
            }

            remove_lucas_1(&mut people);
            println!("remove_lucas_1: {:?}", people);

            remove_lucas_2(&mut people2);
            println!("remove_lucas_2: {:?}", people2);

            println!("remove_lucas_3: {:?}", remove_lucas_3(people3));
        }

        #[test]
        fn valid_while() {
            let mut people = vec![
                Person::new("Matheus"),
                Person::new("Marcos"),
                Person::new("Lucas"),
                Person::new("João"),
            ];

            fn remove_lucas(people: &mut Vec<Person>) {
                let mut counter = 0;
                let mut cursor = &people[..];

                while !cursor.is_empty() {
                    let person = &cursor[0];

                    if person.get_name() == "Lucas" {
                        people.remove(counter);
                        break;
                    }

                    counter += 1;
                    cursor = &cursor[1..];
                }
            }

            remove_lucas(&mut people);
        }

        #[test]
        fn invalid_if_let() {
            // enum PersonAgeGroup {
            //     Child { age: u8 },
            //     Young { free_time: Duration },
            //     Adult { money: f32 },
            // }
            //
            // let mut people = vec![
            //     PersonAgeGroup::Child { age: 7 },
            //     PersonAgeGroup::Young {
            //         free_time: Duration::from_secs(60 * 60 * 8),
            //     },
            //     PersonAgeGroup::Adult { money: 12_000.0 },
            // ];
            //
            // if let PersonAgeGroup::Child { age } = people.last_mut().unwrap() {
            //     *age += 1;
            //     people.push(PersonAgeGroup::Child { age: *age });
            // }
        }

        #[test]
        fn valid_if_let() {
            enum PersonAgeGroup {
                Child { age: u8 },
                Young { free_time: Duration },
                Adult { money: f32 },
            }

            let mut people = vec![
                PersonAgeGroup::Child { age: 7 },
                PersonAgeGroup::Young {
                    free_time: Duration::from_secs(60 * 60 * 8),
                },
                PersonAgeGroup::Adult { money: 12_000.0 },
            ];

            /* 1st Solution */
            if let PersonAgeGroup::Child { age } = people.last_mut().unwrap() {
                *age += 1;
            }
            if let PersonAgeGroup::Child { age } = people.last().unwrap() {
                people.push(PersonAgeGroup::Child { age: *age });
            }

            /* 2nd Solution */
            let (is_child, age) = if let PersonAgeGroup::Child { age } = people.last_mut().unwrap()
            {
                *age += 1;
                (true, *age)
            } else {
                (false, 0)
            };

            if is_child {
                people.push(PersonAgeGroup::Child { age });
            }
        }
    }
}

#[allow(unused)]
mod generics {
    mod without_generics {
        fn add_valor_i32(array: &mut [i32], pos: usize, valor: i32) -> bool {
            if pos >= array.len() {
                return false;
            }

            array[pos] = valor;

            true
        }

        fn add_valor_char(array: &mut [char], pos: usize, valor: char) -> bool {
            if pos >= array.len() {
                return false;
            }

            array[pos] = valor;

            true
        }

        #[test]
        fn test_add_valor() {
            let mut number_list = [0; 5];
            let result = add_valor_i32(&mut number_list, 0, 10);
            println!("add_valor_i32 result: {result}, number_list: {number_list:?}");

            let mut char_list = ['\0'; 5];
            let result = add_valor_char(&mut char_list, 0, 'M');
            println!("add_valor_char result: {result}, char_list: {char_list:?}");
        }
    }

    mod with_generics {
        use std::fmt::Debug;

        fn add_valor<T: Copy>(array: &mut [T], pos: usize, valor: T) -> T {
            if pos >= array.len() {
                return valor;
            }

            array[pos] = valor;

            array[0]
        }

        #[test]
        fn test_add_valor() {
            let mut number_list = [0; 5];
            let result = add_valor(&mut number_list, 0, 10);
            println!("add_valor_i32 result: {result}, number_list: {number_list:?}");

            let mut char_list = ['\0'; 5];
            let result = add_valor(&mut char_list, 10, 'M');
            println!("add_valor_char result: {result}, char_list: {char_list:?}");
        }
    }

    mod generics_in_struct {
        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }

        #[test]
        fn generic_struct_usage() {
            let p1: Point<i32> = Point { x: 5, y: 5 };
            let p2: Point<f32> = Point { x: 1.0f32, y: 4.0 };
            let p3: Point<char> = Point { x: 'a', y: 'b' };

            println!("p1: {p1:?}");
            println!("p2: {p2:?}");
            println!("p3: {p3:?}");
        }

        #[test]
        fn invalid_generic_struct_two_types() {
            let p1 = Point {
                x: 5,
                y: 5.0 as i32,
            };
            println!("p1: {p1:?}");
        }

        #[test]
        fn valid_generic_struct_two_types() {
            #[derive(Debug)]
            struct Point2<T, U> {
                x: T,
                y: U,
            }

            let p1: Point2<i32, f64> = Point2 { x: 5, y: 5.0 };
            println!("p1: {p1:?}");
        }

        #[test]
        fn generic_struct_two_types_four_fields() {
            #[derive(Debug)]
            struct Point2<T, U> {
                x: T,
                y: U,
                z: T,
                w: U,
            }

            let p1 = Point2 {
                x: 5,
                y: 'A',
                z: 10,
                w: 'F',
            };
            println!("p1: {p1:?}");
        }
    }

    /// ::<>
    mod turbo_fish {
        #[test]
        fn explicity_in_function() {
            fn add_valor<T>(array: &mut [T], pos: usize, valor: T) -> bool {
                if pos >= array.len() {
                    return false;
                }

                array[pos] = valor;

                true
            }

            let mut number_list = [0; 5];
            let result = add_valor::<i32>(&mut number_list, 0, 10);
            println!("add_valor_i32 result: {result}, number_list: {number_list:?}");
        }

        #[test]
        fn explicity_struct() {
            #[derive(Debug)]
            struct Point<T, U> {
                x: T,
                y: U,
            }

            let p1 = Point::<i32, f32> { x: 5, y: 5.0 };
            let p1: Point<i32, f32> = Point { x: 5, y: 5.0 };
            println!("p1: {p1:?}");
        }

        #[test]
        fn real_case() {
            struct Hello {
                a: i32,
                b: u16,
            }

            let size_of_char = std::mem::size_of::<Hello>();
            let align = std::mem::align_of::<Hello>();
            dbg!(size_of_char);
            dbg!(align);

            let parse_u32 = "2023".parse::<u32>().unwrap();
            dbg!(parse_u32);

            let list = [5i32; 5];

            let list_sum = list.iter().sum::<i32>();
            dbg!(list_sum);

            let list_product: i32 = list.iter().product();
            dbg!(list_product);

            let one_to_100 = (1..=100).collect::<Vec<u8>>();
            let one_to_100 = (1..=100).collect::<Vec<i16>>();
        }
    }

    mod generics_in_enum {
        use crate::generics::generics_in_enum::RichOutput::{Fail, Success};

        #[derive(Debug)]
        enum Nullable<T> {
            NonNull(T),
            Null,
        }

        #[test]
        fn generic_enum_def() {
            let mut nullable_i32: Nullable<i32> = Nullable::NonNull(7);
            println!("nullable_i32: {nullable_i32:?}");
            nullable_i32 = Nullable::Null;
            println!("nullable_i32: {nullable_i32:?}");

            let mut nullable_string = Nullable::Null;
            println!("nullable_string: {nullable_string:?}");
            nullable_string = Nullable::NonNull("Hello".to_string());
            println!("nullable_string: {nullable_string:?}");

            let mut nullable_string = Nullable::<String>::Null;
            println!("nullable_string: {nullable_string:?}");
            nullable_string = Nullable::<String>::NonNull("Hello".to_string());
            println!("nullable_string: {nullable_string:?}");
        }

        #[test]
        fn invalid_change_generic_type() {
            let mut nullable_f32: Nullable<f32> = Nullable::NonNull(3.0);
            // nullable_f32 = Nullable::NonNull('m');
            // nullable_f32 = Nullable::<char>::Null;
        }

        #[test]
        fn generic_enum_usage() {
            let nullable_f32 = Nullable::NonNull(3.0);
            // let nullable_f32: Nullable<f32> = Nullable::Null;

            if let Nullable::NonNull(v) = &nullable_f32 {
                println!("nullable_f32 as a value: {v}");
            } else {
                println!("nullable_f32 is null!");
            }

            match nullable_f32 {
                Nullable::NonNull(v) => println!("nullable_f32 as a value: {v}"),
                Nullable::Null => println!("nullable_f32 is null!"),
            }
        }

        #[derive(Debug)]
        enum RichOutput<S, F> {
            Success(S),
            Fail(F),
        }

        #[test]
        fn generic_enum_two_types() {
            fn safe_divide(a: f32, b: f32) -> RichOutput<f32, String> {
                if b != 0.0 {
                    Success(a / b)
                } else {
                    Fail("b cannot be zero!!!".to_string())
                }
            }

            let res = safe_divide(1.0, 2.0);
            if let Success(s) = &res {
                println!("Success with value: {s}");
            } else if let Fail(f) = &res {
                println!("Fail with message: {f}");
            }

            match safe_divide(10.0, 0.0) {
                Success(s) => println!("Success with value: {s}"),
                Fail(f) => println!("Fail with message: {f}"),
            }
        }
    }

    mod performance {
        #[derive(Debug)]
        enum Nullable<T> {
            NonNull(T),
            Null,
        }

        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        enum Nullable_i32 {
            NonNull(i32),
            Null,
        }

        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        enum Nullable_String {
            NonNull(String),
            Null,
        }

        /*----------------------------------------------*/

        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }

        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        struct Point_i32 {
            x: i32,
            y: i32,
        }

        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        struct Point_f32 {
            x: f32,
            y: f32,
        }
    }

    mod methods_fixed_type {
        struct Point<T> {
            x: T,
            y: T,
        }

        impl Point<i32> {
            pub fn new(x: i32, y: i32) -> Self {
                Self { x, y }
            }

            pub fn dot(&self, rhs: &Point<f32>) -> i32 {
                self.x * rhs.x as i32 + self.y * rhs.y as i32
            }
        }

        #[test]
        fn test_point_i32() {
            let p1 = Point::<i32>::new(1, 2);
            let p2 = Point::new_f32(2.0, 3.0);

            let dot = p1.dot(&p2);
            dbg!(dot);
        }

        impl Point<f32> {
            pub fn new_f32(x: f32, y: f32) -> Self {
                Self { x, y }
            }

            // TODO Porque deu certo?
            pub fn dot(&self, rhs: &Point<f32>) -> f32 {
                self.x * rhs.x + self.y * rhs.y
            }

            pub fn magnitude(&self) -> f32 {
                self.dot(self).sqrt()
            }
        }

        #[test]
        fn test_point_f32() {
            let p1 = Point::new_f32(1.0, 2.0);
            // let p1 = Point::<i32>::new(1, 2);

            let magnitude = p1.magnitude();
            dbg!(magnitude);
        }
    }

    mod methods_generic_type {
        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            pub fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }

        impl Point<f32> {
            pub fn magnitude(&self) -> f32 {
                (self.x * self.x + self.y * self.y).sqrt()
            }
        }

        #[test]
        fn test_point_i32() {
            let p1 = Point::new(1, 2);
            let p2 = Point::new(2, 3);

            dbg!(p1);
            dbg!(p2);
        }

        #[test]
        fn test_point_f32() {
            let p1 = Point::new(1.0, 2.0);
            // let p1 = Point::new(1, 2);

            let magnitude = p1.magnitude();
            dbg!(magnitude);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
