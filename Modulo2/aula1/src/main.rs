mod case1 {
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

mod case2 {
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

mod case3 {
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

fn main() {}
