#![feature(never_type)]

mod static_vars {
    mod imutable {
        static NAME: &'static str = "Matheus";
        static AGE: u8 = 27;

        #[test]
        fn static_name_n_age() {
            println!("Name: {}, age: {}", NAME, AGE);
        }

        #[test]
        fn static_inside_fn() {
            static AGE: u8 = 16;

            println!("Age: {}", AGE);
        }
    }

    mod unsafe_mut {
        static mut UNSAFE_COUNTER: u32 = 0;

        #[test]
        fn unsafe_mut_static() {
            unsafe {
                UNSAFE_COUNTER += 1;
                println!("Counter: {}", UNSAFE_COUNTER);
            }
        }
    }

    mod safe_mut {
        use std::sync::atomic::{AtomicU32, Ordering};
        use std::sync::Mutex;

        static SAFE_ARC_COUNTER: AtomicU32 = AtomicU32::new(0);

        #[test]
        fn atomic_mut_static() {
            SAFE_ARC_COUNTER.fetch_add(1, Ordering::AcqRel);
            println!("Counter: {}", SAFE_ARC_COUNTER.load(Ordering::Acquire));
        }

        static SAFE_MUTEX_COUNTER: Mutex<String> = Mutex::new(String::new());

        #[test]
        fn mutex_mut_static() {
            let mut value = SAFE_MUTEX_COUNTER.lock().unwrap();
            *value += "Hello";
            println!("Counter: {}", value);
        }
    }
}

mod const_vars {
    const PI: f32 = 3.14;

    #[test]
    fn inside_fn() {
        const MATH_PI: f32 = 3.14159265359;

        println!("π: {}, math π: {}", PI, MATH_PI);
    }

    struct Engineer;

    impl Engineer {
        const PI: u8 = 3;
    }

    #[test]
    fn engineer() {
        println!("engineer π: {}", Engineer::PI);
    }
}

/// Diferenças de `const` e `static`
///
/// |                       | const | static |
/// |:----------------------|:-----:|:------:|
/// | Tipos Explicito       | Sempre              | Sempre |
/// | Mutabilidade          | Nunca               | `unsafe` |
/// | Endereço fixo         | ❌                  | ✅ |
/// | É copiado em cada uso | ✅                  | ❌ |
/// | Tempo de execução     | Rápido              | Lento |
/// | Tamanho do binário    | Grande              | Pequeno |
/// | Semelhança com C      | `#define VAR 0`     | `const uint8_t var = 0;` |
/// | Inicialização         | Sempre na definição | Sempre na definição |
mod const_vs_static {
    #[allow(unused)]
    const CONST: [u8; 1_000_000] = [0u8; 1_000_000];
    #[allow(unused)]
    static STATIC: [u8; 1_000_000] = [0u8; 1_000_000];
}

mod memory_ownership {
    #[test]
    fn one_owners() {
        /* 's1' é dona do slot de memória que guarda uma String. */
        let s1 = String::from("Hello, World!");
        /* Agora 's2' possui a "propriedade" do slot de memória que pertencia a 's2' */
        let s2 = s1;

        /* Não é possível usar a variável 'd1' aqui, pois ela não é mais dona de nenhum slot de
           memória. */
        // println!("s1: {}", s1);
        println!("s2: {}", s2);

        /* O conceito de passar a "propriedade (sobre a memória)" de uma variável para outra,
           recebe um verbo especial: Mover. */
    }

    #[test]
    fn two_owner() {
        /* 'd1' é dona de um slot que contém o valor 10 */
        let d1 = 7;
        /* Neste ponto, 'd2' não assume a "propriedade" do slot de 'd1', ao invés disso, o valor é
           copiado. Isso ocorre com todos os tipos primitivos (e referências também).
           Ou seja, os tipos primitivos (e referências) são copiados. */
        let d2 = d1;

        println!("d1: {}", d1);
        println!("d2: {}", d2);
    }

    #[test]
    fn ref_owner() {
        /* Aqui nós temos a variável 'v1' dona sobre um slot de memória. */
        let v1 = String::from("Hello, World!");
        /* Agora estamos obtendo uma referência de 'v1'.
           'r1' será uma variável que contém uma referência para 'v1'.
           Porém, como qualquer outra variável, ela é dona de um slot de memória.
           A diferença é o tipo: uma referência para outra variável. */
        let r1 = &v1;
        /* Aqui, como dito anteriormente, a referência é copiada e a "propriedade" de r1 não é perdida.
           Note que a referência de 'r1' é copiada e não o valor de 'v1'. */
        let r2 = r1;

        println!("r1: {r1}");
        println!("r2: {r2}");


        let d1 = 7;
        /* Para ficar mais claro, o valor de 'd1' está sendo copiado para 'd2', assim como o valor
        de 'r1' está sendo copiado para 'r2'.
           A única diferença de `let d2 = d1;` para `let r2 = r1;` são os tipos das variáveis,
           sendo 'd1' e 'd2' do tipo 'i32' e 'r1' e 'r2' do tipo '&String' */
        let d2 = d1;

        println!("d1: {d1}");
        println!("d2: {d2}");
    }

    #[test]
    fn slice_are_also_refs() {
        let v1 = [7; 10];

        let r1 = &v1;
        let r2 = r1;

        println!("r1: {r1:?}");
        println!("r2: {r2:?}");
    }

    mod move_syntax {
        #[test]
        fn copy_types() {
            let integers = 7;
            let i2 = integers;
            println!("intergers: {integers}, i2: {i2}");

            let floats = 7.0;
            let f2 = floats;
            println!("floats: {floats}, f2: {f2}");

            let booleans = true;
            let b2 = booleans;
            println!("booleans: {booleans}, b2: {b2}");

            let refs = &floats;
            let r2 = refs;
            println!("refs: {refs}, r2: {r2}");

            let tuples = (1, 2.0, false);
            let t2 = tuples;
            println!("tuples: {tuples:?}, t2: {t2:?}");

            let arrays = [7; 10];
            let a2 = arrays;
            println!("arrays: {arrays:?}, a2: {a2:?}");

            let functions = |x| println!("Greetings: {x:?}");
            let f2 = functions;
            functions("Matheus");
            f2("Tenório");
        }

        #[derive(Debug)]
        struct MoveStruct {}

        #[derive(Debug)]
        enum MoveEnum {
            Var1,
            Var2,
        }

        #[test]
        fn struct_moves() {
            let move_struct = MoveStruct {};
            let ms2 = move_struct;
            // println!("move_struct: {move_struct:?}");
            println!("ms2: {ms2:?}");
        }

        #[test]
        fn enum_moves() {
            let move_enum = MoveEnum::Var1;
            let me2 = move_enum;
            // println!("move_enum: {move_enum:?}");
            println!("ms2: {me2:?}");
        }

        #[test]
        fn array_of_structs() {
            let move_array = [MoveStruct {}, MoveStruct {}, MoveStruct {}];
            let ma2 = move_array;
            // println!("move_array: {move_array:?}");
            println!("ma2: {ma2:?}");
        }

        #[test]
        fn array_of_enums() {
            let move_array = [MoveEnum::Var1, MoveEnum::Var2, MoveEnum::Var1];
            let ma2 = move_array;
            // println!("move_array: {move_array:?}");
            println!("ma2: {ma2:?}");
        }

        #[test]
        fn fast_init_array() {
            // let a1 = [MoveStruct {}; 5];
            // let a2 = [MoveEnum::Var1; 5];
        }

        #[test]
        fn user_types_tuple() {
            let move_tuple = (MoveEnum::Var2, MoveStruct {}, MoveEnum::Var1);
            let mt2 = move_tuple;
            // println!("move_tuple: {move_tuple:?}");
            println!("mt2: {mt2:?}");
        }

        // !!!RECAP!!!
        // Tipos compostos só podem ser copiados se seus tipos puderem ser copiados. Se pelo menos
        // um dos tipos (no caso do array o tipo principal) não puder ser movido, então a tupla
        // (ou o array) não poderá ser copiado, será movido.

        #[test]
        fn functions_args() {
            fn greetings(p: Person) {
                println!("Hello \"{}\"!", p.name);
            }

            let vip = Person::new("Matheus");
            greetings(vip);
            // println!("Vip name: {}", vip.name);
        }

        #[test]
        fn functions_return() {
            fn add_mister(p: Person) -> String {
                let res = format!("Mr. {}", p.name);

                return res;
            }

            let vip = Person::new("Marcos");
            let vip_name = add_mister(vip);
            println!("Hello again \"{vip_name}\"!")
        }

        #[test]
        fn chalenge1() {
            fn greetings(p: Person) {
                println!("Hello \"{}\"!", p.name);
            }

            fn add_mister(p: Person) -> String {
                format!("Mr. {}", p.name)
            }

            fn add_mister2(mut p: Person) -> Person {
                todo!()
            }

            let p1 = Person::new("Lucas");
            greetings(add_mister2(p1));
        }

        use person::Person;

        mod person {
            pub struct Person {
                pub name: String,
            }

            impl Person {
                pub fn new(name: &str) -> Person {
                    Person { name: name.to_string() }
                }
            }
        }

        #[test]
        fn other_move_cases() {
            let v1 = [MoveEnum::Var1, MoveEnum::Var2];

            for v in v1 {
                println!("v: {v:?}");
            }
            // println!("v1: {v1:?}");

            let opt = Some(MoveEnum::Var1);
            match opt {
                Some(MoveEnum::Var1) => println!("Var1"),
                Some(v) => println!("Other variant: {:?}", v),
                None => println!("No value"),
            }

            // if let Some(v) = opt {
            //     println!("Other variant: {:?}", v);
            // }

            // if let Some(MoveEnum::Var2) = opt {
            //     println!("It's Var2");
            // }
        }
    }
}

/// TODO
mod borrow_checker {}

mod trivia {
    use std::time::Duration;

    #[test]
    fn never_type() {
        let a: ! = loop {};
        let p: ! = panic!();
    }

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
    fn enum_zero_variant() {
        enum ZeroVariant {}

        // let no_variant = ???;
    }
}

// mod move_string {
//     fn digits_to_string(digit_list: &[u8]) -> String {
//         /* É criada a variável 'string' no escopo da função 'digits_to_string'. */
//         /* O buffer da String é alocado na heap, porém um "handler" é alocado na stack. */
//         let mut string = String::new();
//
//         for digit in digit_list {
//             string.push(char::from_digit(*digit as u32, 10).unwrap());
//         }
//
//         /* A variável 'string' não é destruida aqui, mas sim movida para fora da função. Em C, a
//         variável seria destruída e o seu valor copiado para a variável que recebe o retorno da
//         função. */
//         /* Como a variável é movida, o "handler" da String não sai do escopo e não é destruído.
//         Desta forma, o buffer da String não é desalocado da heap. */
//         string
//     }
//
//     #[test]
//     fn convert_to_string() {
//         let digits = [1, 2, 3, 4, 5];
//         let digits_str = digits_to_string(&digits);
//         /* A variável 'string' criada na função 'digits_to_string' foi movida para a
//         variável 'digits_str'. Note que a variável não foi copiada para fora como C, mas sim
//         movida. A operação de mover uma variável aumenta o desempenho (uso de CPU e de memória)
//         porque não é necessário copia a variável. */
//
//         println!("Digits: {}", digits_str);
//
//         /* No fim do escopo da função a variável 'digits_str' é desalocada ("destruída") da stack
//         e sua parte na heap é desalocada no mesmo momento. */
//     }
// }
//
// mod move_my_string {
//     #[derive(Debug)]
//     struct MyString(String);
//
//     fn digits_to_my_string(digit_list: &[u8]) -> MyString {
//         let mut string = MyString(String::new());
//
//         for digit in digit_list {
//             string.0.push(char::from_digit(*digit as u32, 10).unwrap());
//         }
//
//         string
//     }
//
//     #[test]
//     fn convert_to_my_string() {
//         {
//             let digits = [1, 2, 3, 4, 5];
//             let digits_str = digits_to_my_string(&digits);
//
//             println!("Digits: {:?}", digits_str);
//         }
//         println!("Depois do escopo");
//     }
// }
//
// mod move_my_string_with_drop {
//     #[derive(Debug)]
//     struct MyString(String);
//
//     impl Drop for MyString {
//         fn drop(&mut self) {
//             println!("Droping MyString");
//         }
//     }
//
//     fn digits_to_my_string(digit_list: &[u8]) -> MyString {
//         let mut string = MyString(String::new());
//
//         for digit in digit_list {
//             string.0.push(char::from_digit(*digit as u32, 10).unwrap());
//         }
//
//         string
//     }
//
//     #[test]
//     fn convert_to_my_string() {
//         {
//             let digits = [1, 2, 3, 4, 5];
//             let digits_str = digits_to_my_string(&digits);
//
//             println!("Digits: {:?}", digits_str);
//         }
//         println!("Depois do escopo");
//     }
// }
//
// mod borrow_checker {
//     /*
//         Regras do Borrow Checker (Verificador de Empréstimos)
//
//         1. É possivel ter infinitas referências IMUTÁVEL (&) ao mesmo tempo.
//         2. Se existir uma referência MUTÁVEL (&mut), não pode existir nenhum outro tipo de
//            referência. Nem mutável (&mut), nem imutável (&).
//     */
//
//     use std::vec;
//
//     #[test]
//     fn one_imut() {
//         let a = 10;
//         let a_ref = &a;
//
//         println!("{}: {}", a, a_ref);
//     }
//
//     #[test]
//     fn many_imut() {
//         let a = 10;
//         let ref1 = &a;
//         let ref2 = &a;
//         let ref3 = ref2;
//
//         println!("{}: {}, {}, {}", a, ref1, ref2, ref3);
//     }
//
//     #[test]
//     fn one_mut() {
//         let mut a = 10;
//         let mut_ref = &mut a;
//
//         println!("{}", mut_ref);
//     }
//
//     #[test]
//     fn two_mut() {
//         let mut a = 10;
//         let ref1 = &mut a;
//         // let ref2 = &mut a;
//
//         println!("{}", ref1);
//         // println!("{}", ref2);
//     }
//
//     #[test]
//     fn one_mut_one_imut() {
//         let mut a = 10;
//         let mut_ref = &mut a;
//         // let imut_ref = &a;
//
//         // println!("{}", imut_ref);
//         println!("{}", mut_ref);
//     }
//
//     mod point {
//         #[derive(Clone)]
//         pub struct Point {
//             x: i32,
//             #[allow(unused)]
//             y: i32,
//         }
//
//         impl Point {
//             pub fn new(x: i32, y: i32) -> Point {
//                 Point { x, y }
//             }
//
//             pub fn set_x(&mut self, x: i32) {
//                 self.x = x;
//             }
//
//             pub fn get_x(&self) -> &i32 {
//                 &self.x
//             }
//         }
//     }
//
//     #[test]
//     #[allow(unused_variables)]
//     fn struct_mut_ref() {
//         let mut p1 = point::Point::new(1, 2);
//
//         let p1_x = p1.get_x();
//
//         p1.set_x(3);
//
//         // println!("{}", p1_x);
//     }
//
//     #[test]
//     fn for_problem() {
//         let mut points = vec![point::Point::new(1, 2)];
//
//         for p in points.iter_mut() {
//             // p.set_x(p.get_x() + 1);
//
//             if *p.get_x() == 2 {
//                 // points.remove(0);
//             }
//         }
//     }
//
//     fn print_out(item: &Vec<u8>) {
//         for i in item {
//             println!("{}", i);
//         }
//     }
//
//     #[test]
//     fn test_print_out() {
//         let item = vec![1, 2, 3];
//
//         print_out(&item);
//         print_out(&item);
//     }
//
//     #[test]
//     fn print_last() {
//         let mut item = vec![1, 2, 3];
//
//         let last = item.last().unwrap();
//         println!("Last: {}", last);
//
//         item.pop();
//
//         // println!("Last: {}", last);
//     }
//
//     fn reverse_and_print(list: &Vec<u8>) {
//         for li in list.iter().rev() {
//             println!("{}", li);
//         }
//     }
//
//     fn reverse_and_print_mut(list: &mut Vec<u8>) {
//         list.reverse();
//         for li in list.iter() {
//             println!("{}", li);
//         }
//     }
//
//     #[test]
//     fn test_reverse_and_print() {
//         let list = vec![1, 2, 3];
//
//         reverse_and_print(&list);
//         reverse_and_print(&list);
//
//         let mut list = vec![4, 5, 6];
//
//         reverse_and_print_mut(&mut list);
//         reverse_and_print_mut(&mut list);
//     }
// }

fn main() {}
