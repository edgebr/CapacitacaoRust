/// # Regras do Borrow Checker
///
/// 1. É possivel ter inifinitas referências **IMUTÁVEIS** (`&`) ao mesmo tempo.
/// 2. Só é possivel ter **EXTATAMENTE 1** referência **MUTÁVEL** (`&mut`).
///
/// Observação: Quando é dito, na regra 2, que só pode haver uma referência mutável, estão incluidas
/// as referências imutáveis também.
/// Desta forma, não é possível de 2 referências mutáveis (`&mut`), assim como não é possível ter
/// 1 referência imutável (`&`), ao mesmo tempo, de 1 referência mutável (`&mut`).
#[allow(unused)]
mod borrow_checker {
    mod rule_1 {
        #[test]
        fn one_imut_ref() {
            let a = 7;
            let a_ref = &a;

            println!("{a}: {a_ref}");
        }

        #[test]
        fn many_imut_ref() {
            let a = 7;
            let ref1 = &a;
            let ref2 = &a;
            let ref3 = ref2;

            println!("{a}: {ref1}, {ref2}, {ref3}");
        }
    }

    mod rule_2 {
        #[test]
        fn one_mut_ref() {
            let mut a = 7;
            let mut_ref = &mut a;

            *mut_ref = 12;

            println!("{mut_ref}");
        }

        #[test]
        fn invalid_2mut() {
            // let mut a = 7;
            // let ref1 = &mut a;
            // let ref2 = &mut a;
            //
            // println!("{ref1}");
            // println!("{ref2}");
        }

        #[test]
        fn valid_2mut() {
            let mut a = 7;

            let ref1 = &mut a;
            println!("{ref1}");

            let ref2 = &mut a;
            println!("{ref2}");
        }

        #[test]
        fn invalid_1mut_1imut() {
            // let mut a = 7;
            // let mut_ref = &mut a;
            // let imut_ref = &a;
            //
            // println!("{mut_ref}");
            // println!("{imut_ref}");
        }

        #[test]
        fn valid_1mut_1imut() {
            let mut a = 7;
            let mut_ref = &mut a;
            println!("{mut_ref}");

            let imut_ref = &a;
            println!("{imut_ref}");
        }

        #[test]
        fn trick_borrow() {
            // let mut a = 7;
            // let mut_ref = &mut a;
            //
            // println!("{a}: {mut_ref}");
        }
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

    mod method_call {
        use crate::borrow_checker::person::Person;

        #[test]
        fn simple_call_imut() {
            let p = Person::new("Matheus");

            let name = p.get_name();
            let name = Person::get_name(&p);
        }

        #[test]
        fn simple_call_mut() {
            let mut p = Person::new("Matheus");

            p.set_name("Marcos");
            Person::set_name(&mut p, "Marcos");
            Person::set_name(&mut p, "Marcos");
        }

        #[test]
        fn simple_call_mut_ref() {
            let mut p = Person::new("Matheus");
            let p_mut = &mut p;

            p_mut.set_name("Marcos");
            Person::set_name(p_mut, "Marcos");
        }

        #[test]
        fn invalid_1mut_1imut() {
            // let mut p = Person::new("Matheus");
            // let p_mut = &mut p;
            //
            // let name = p.get_name();
            // p_mut.set_name("Marcos");
        }

        #[test]
        fn detailed_invalid_1mut_1imut() {
            // let mut p = Person::new("Matheus");
            // let p_mut = &mut p;
            //
            // let name = Person::get_name(&p);
            // Person::set_name(p_mut, "Marcos");
        }

        #[test]
        fn valid_1mut_1imut() {
            let mut p = Person::new("Matheus");

            let name = p.get_name();
            p.set_name("Marcos");
        }

        #[test]
        fn detailed_valid_1mut_1imut() {
            let mut p = Person::new("Matheus");

            let name = Person::get_name(&p);
            Person::set_name(&mut p, "Marcos");
        }

        #[test]
        fn invalid_func_mut_ref() {
            // fn set_to_marcos(p2: &mut Person, old_name: &str) {
            //     p2.set_name("Marcos");
            //     println!("Changing from {old_name} to {}", p2.get_name());
            // }
            //
            // let mut p = Person::new("Matheus");
            //
            // set_to_marcos(&mut p, p.get_name());
        }

        #[test]
        fn valid_func_mut_ref() {
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

        #[test]
        fn detailed_invalid_func_mut_ref() {
            // fn set_to_marcos(p2: &mut Person, old_name: &str) {
            //     p2.set_name("Marcos");
            //     println!("Changing from {old_name} to {}", p2.get_name());
            // }
            //
            // let mut p = Person::new("Matheus");
            //
            // /* 1st Solution */
            // let name = p.get_name().to_string();
            // let name_ref = Person::get_name(&p);
            // let name = name_ref.to_string();
            // set_to_marcos(&mut p, &name);
            //
            // /* 2nd Solution */
            // // set_to_marcos(&mut p, &p.get_name().to_string());
            // //                        ------------
            // //                        |
            // //                        Person::get_name(&p) -> &str
            // //                                                ----
            // //                                                |
            // //                                                (&str).to_string() -> String
            // //                                                                      ------
            // //                                                                      |
            // //                                                                      &String -> &str
        }

        #[test]
        fn invalid_move_after_imut_borrow() {
            // let p = Person::new("Matheus");
            // let p2 = &p;
            //
            // let p3 = p;
            //
            // let name = p2.get_name();
        }

        #[test]
        fn valid_move_after_imut_borrow() {
            let p = Person::new("Matheus");

            let name = p.get_name();
            let name = Person::get_name(&p);

            let p3 = p;

            let name = p3.get_name();
            let name = Person::get_name(&p3);
        }
    }

    // !!! DICA !!!
    // Sempre procure utilizar variáveis e referências imutável.
    // A preferência por referências imutáveis, facilita a escrita do código, porque podemos ter
    // várias referências imutáveis sem conflitos entre elas.

    mod why_borrow_checker {
        fn reverse_and_print_1(list: &[i32]) {
            for li in list.iter().rev() {
                print!("{li}, ");
            }
            println!();
        }

        fn reverse_and_print_2(list: &mut [i32]) {
            list.reverse();
            for li in list.iter() {
                print!("{li}, ");
            }
            println!();
        }

        #[test]
        fn meaningful_signatures() {
            let mut list = [1, 2, 3];

            reverse_and_print_1(&list);
            println!("Original list: {list:?}");

            reverse_and_print_2(&mut list);
            println!("Original list: {list:?}");
        }

        #[test]
        fn twice_call_imut() {
            let mut list = [1, 2, 3];

            reverse_and_print_1(&list);
            reverse_and_print_1(&list);
        }

        #[test]
        fn twice_call_mut() {
            let mut list = [1, 2, 3];

            reverse_and_print_2(&mut list);
            reverse_and_print_2(&mut list);
        }

        fn add_1_count_even_imut(list: &[i32]) -> usize {
            list.iter().filter(|x| ((*x + 1) % 2) == 0).count()
        }

        fn add_1_count_even_mut(list: &mut [i32]) -> usize {
            for li in list.iter_mut() {
                *li += 1;
            }
            list.iter().filter(|x| (*x % 2) == 0).count()
        }

        #[test]
        fn result_twice_imut() {
            let list = [1, 2, 3, 4, 6];

            if add_1_count_even_imut(&list) <= 2 {
                println!("Few evens: {}", add_1_count_even_imut(&list));
            } else {
                println!("Many evens: {}", add_1_count_even_imut(&list));
            }
        }

        #[test]
        fn invalid_result_twice_mut() {
            let mut list = [1, 2, 3, 4, 6];

            if add_1_count_even_mut(&mut list) <= 2 {
                println!("Few evens: {}", add_1_count_even_mut(&mut list));
            } else {
                println!("Many evens: {}", add_1_count_even_mut(&mut list));
            }
        }

        #[test]
        fn valid_result_twice_mut() {
            let mut list = [1, 2, 3, 4, 6];

            let count = add_1_count_even_mut(&mut list);
            if count <= 2 {
                println!("Few evens: {}", count);
            } else {
                println!("Many evens: {}", count);
            }
        }
    }

    /// !!! RECAP !!!
    ///
    /// ![Move-Copy-Borrow](https://rufflewind.com/img/rust-move-copy-borrow.png)
    ///
    /// 1. Inifinitas referências IMUTÁVEIS (&) ao mesmo tempo.
    /// 2. EXTATAMENTE 1 referência MUTÁVEL (&mut) ao mesmo tempo.
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
            let mut people3 = people.clone();

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

fn main() {}
