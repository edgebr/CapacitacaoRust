mod static_vars {
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

mod unsafe_mut_static_var {
    use std::thread::sleep;
    use std::time::Duration;

    static mut UNSAFE_COUNTER: u32 = 0;

    #[test]
    fn unsafe_mut_static() {
        unsafe {
            UNSAFE_COUNTER += 1;
            println!("Counter: {}", UNSAFE_COUNTER);
        }
    }
}

mod safe_mut_static_var {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Mutex;

    static SAFE_ARC_COUNTER: AtomicU32 = AtomicU32::new(0);

    #[test]
    fn atomic_mut_static() {
        SAFE_ARC_COUNTER.fetch_add(1, Ordering::AcqRel);
        println!("Counter: {}", SAFE_ARC_COUNTER.load(Ordering::Acquire));
    }

    static SAFE_MUTEX_COUNTER: Mutex<u32> = Mutex::new(0);

    #[test]
    fn mutex_mut_static() {
        let mut value = SAFE_MUTEX_COUNTER.lock().unwrap();
        *value += 1;
        println!("Counter: {}", value);
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

mod const_vs_static {
    /*
        1. Sem endereço fixo na memória.
        2. É copiado para cada local utilizado
        3. Rápido em tempo de execução, porém gera um binário maior
        4. Semelhante ao `#define` do C
    */
    const CONST: u32 = 0;
    /*
        1. Tem endereço fixo na memória.
        2. Semelhante ao `static const` do C.
     */
    static STATIC: u32 = 0;
}

mod move_string {
    fn digits_to_string(digit_list: &[u8]) -> String {
        /* É criada a variável 'string' no escopo da função 'digits_to_string'. */
        /* O buffer da String é alocado na heap, porém um "handler" é alocado na stack. */
        let mut string = String::new();

        for digit in digit_list {
            string.push(char::from_digit(*digit as u32, 10).unwrap());
        }

        /* A variável 'string' não é destruida aqui, mas sim movida para fora da função. Em C, a
        variável seria destruída e o seu valor copiado para a variável que recebe o retorno da
        função. */
        /* Como a variável é movida, o "handler" da String não sai do escopo e não é destruído.
        Desta forma, o buffer da String não é desalocado da heap. */
        string
    }

    #[test]
    fn convert_to_string() {
        let digits = [1, 2, 3, 4, 5];
        let digits_str = digits_to_string(&digits);
        /* A variável 'string' criada na função 'digits_to_string' foi movida para a
        variável 'digits_str'. Note que a variável não foi copiada para fora como C, mas sim
        movida. A operação de mover uma variável aumenta o desempenho (uso de CPU e de memória)
        porque não é necessário copia a variável. */

        println!("Digits: {}", digits_str);

        /* No fim do escopo da função a variável 'digits_str' é desalocada ("destruída") da stack
        e sua parte na heap é desalocada no mesmo momento. */
    }
}

mod move_my_string {
    #[derive(Debug)]
    struct MyString(String);

    fn digits_to_my_string(digit_list: &[u8]) -> MyString {
        let mut string = MyString(String::new());

        for digit in digit_list {
            string.0.push(char::from_digit(*digit as u32, 10).unwrap());
        }

        string
    }

    #[test]
    fn convert_to_my_string() {
        {
            let digits = [1, 2, 3, 4, 5];
            let digits_str = digits_to_my_string(&digits);

            println!("Digits: {:?}", digits_str);
        }
        println!("Depois do escopo");
    }
}

mod move_my_string_with_drop {
    #[derive(Debug)]
    struct MyString(String);

    impl Drop for MyString {
        fn drop(&mut self) {
            println!("Droping MyString");
        }
    }

    fn digits_to_my_string(digit_list: &[u8]) -> MyString {
        let mut string = MyString(String::new());

        for digit in digit_list {
            string.0.push(char::from_digit(*digit as u32, 10).unwrap());
        }

        string
    }

    #[test]
    fn convert_to_my_string() {
        {
            let digits = [1, 2, 3, 4, 5];
            let digits_str = digits_to_my_string(&digits);

            println!("Digits: {:?}", digits_str);
        }
        println!("Depois do escopo");
    }
}

mod borrow_checker {
    /*
        Regras do Borrow Checker (Verificador de Empréstimos)

        1. É possivel ter infinitas referências IMUTÁVEL (&) ao mesmo tempo.
        2. Se existir uma referência MUTÁVEL (&mut), não pode existir nenhum outro tipo de
           referência. Nem mutável (&mut), nem imutável (&).
    */

    use std::vec;
    use crate::borrow_checker::point::Point;

    #[test]
    fn one_imut() {
        let a = 10;
        let a_ref = &a;

        println!("{}: {}", a, a_ref);
    }

    #[test]
    fn many_imut() {
        let a = 10;
        let ref1 = &a;
        let ref2 = &a;
        let ref3 = ref2;

        println!("{}: {}, {}, {}", a, ref1, ref2, ref3);
    }

    #[test]
    fn one_mut() {
        let mut a = 10;
        let mut_ref = &mut a;

        println!("{}", mut_ref);
    }

    #[test]
    fn two_mut() {
        let mut a = 10;
        let ref1 = &mut a;
        // let ref2 = &mut a;

        println!("{}", ref1);
        // println!("{}", ref2);
    }

    #[test]
    fn one_mut_one_imut() {
        let mut a = 10;
        let mut_ref = &mut a;
        // let imut_ref = &a;

        // println!("{}", imut_ref);
        println!("{}", mut_ref);
    }

    mod point {
        #[derive(Clone)]
        pub struct Point {
            x: i32,
            y: i32,
        }

        impl Point {
            pub fn new(x: i32, y: i32) -> Point {
                Point { x, y }
            }

            pub fn set_x(&mut self, x: i32) {
                self.x = x;
            }

            pub fn get_x(&self) -> &i32 {
                &self.x
            }
        }
    }

    #[test]
    fn struct_mut_ref() {
        let mut p1 = point::Point::new(1, 2);

        let p1_x = p1.get_x();

        p1.set_x(3);

        // println!("{}", p1_x);
    }

    #[test]
    fn for_problem() {
        let mut points = vec![point::Point::new(1, 2)];

        for p in points.iter_mut() {
            // p.set_x(p.get_x() + 1);

            if *p.get_x() == 2 {
                // points.remove(0);
            }
        }
    }

    fn print_out(item: &Vec<u8>) {
        for i in item {
            println!("{}", i);
        }
    }

    #[test]
    fn test_print_out() {
        let item = vec![1, 2, 3];

        print_out(&item);
        print_out(&item);
    }

    #[test]
    fn print_last() {
        let mut item = vec![1, 2, 3];

        let last = item.last().unwrap();
        println!("Last: {}", last);

        item.pop();

        // println!("Last: {}", last);
    }

    fn reverse_and_print(list: &Vec<u8>) {
        for li in list.iter().rev() {
            println!("{}", li);
        }
    }

    fn reverse_and_print_mut(list: &mut Vec<u8>) {
        list.reverse();
        for li in list.iter() {
            println!("{}", li);
        }
    }

    #[test]
    fn test_reverse_and_print() {
        let list = vec![1, 2, 3];

        reverse_and_print(&list);
        reverse_and_print(&list);

        let mut list = vec![4, 5, 6];

        reverse_and_print_mut(&mut list);
        reverse_and_print_mut(&mut list);
    }
}

fn main() {}
