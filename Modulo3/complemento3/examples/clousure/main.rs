#![allow(unused)]

mod function_pointer {
    fn say_hello(name: String) -> String {
        format!("Hello, {name}")
    }

    #[test]
    fn test_fn_ptr() {
        let greetings = say_hello("Matheus".to_owned());
        println!("{greetings}");

        let fn_ptr = say_hello;
        println!("{}", fn_ptr("Matheus".to_owned()));
    }

    #[test]
    fn test_clousure() {
        let full_clousure = |name: String| -> String {
            let res = format!("Hello, {name}");
            res
        };

        let no_hints_return = |name: String| {
            let res = format!("Hello, {name}");
            res
        };

        let one_line_clousure = |name: String| format!("Hello, {name}");

        let no_hints = |name| format!("Hello, {name}");
        println!("{}", no_hints("Matheus".to_owned()));
    }

    fn print_callback_result(arg: String, callback: fn(String) -> String) {
        println!("{}", callback(arg));
    }

    #[test]
    fn test_fn_ptr_as_argument() {
        let clousure = |name| format!("Hello, {name}");

        print_callback_result("Matheus".to_owned(), clousure);
    }
}

mod imut_clousure {
    #[test]
    fn test_imut_clousure() {
        let greeting = String::from("Goodbye");

        let say_goodbye = |name: &str| format!("{}, {}", &greeting, name);

        println!("{}", say_goodbye("Matheus"));
    }

    fn print_callback_result<F>(arg: String, callback: F)
    where
        F: Fn(String) -> String,
    {
        println!("{}", callback(arg));
    }

    fn print_callback_result_fn(arg: String, callback: fn(String) -> String) {
        println!("{}", callback(arg));
    }

    #[test]
    fn test_imut_clousure_as_argument() {
        let greeting = String::from("Goodbye");
        let imut_clousure = |name| format!("{}, {}", greeting, name);
        let fn_clousure = |name| format!("Goodbye, {}", name);

        /* Fn() <- fn(), Fn() */
        print_callback_result("Matheus".to_owned(), imut_clousure);
        print_callback_result("Matheus".to_owned(), fn_clousure);
    }

    #[test]
    fn test_invalid_clousure() {
        let greeting = String::from("Goodbye");
        let imut_clousure = |name: String| format!("{}, {}", greeting, name);
        let fn_clousure = |name| format!("Hello, {}", name);

        /* fn() <- fn() */
        // print_callback_result_fn("Matheus".to_owned(), imut_clousure);
        print_callback_result_fn("Matheus".to_owned(), fn_clousure);
    }
}

mod mut_clousure {
    #[test]
    fn test_mut_clousure() {
        let mut greeting = String::from("Goodbye");

        let mut say_goodbye = |name: &str| {
            greeting.remove(0);
            format!("{}, {}", &greeting, name)
        };

        println!("{}", say_goodbye("Matheus"));
    }

    fn print_callback_result<F>(arg: String, mut callback: F)
    where
        F: FnMut(String) -> String,
    {
        println!("{}", callback(arg));
    }

    fn print_callback_result_fn_imut<F>(arg: String, callback: F)
    where
        F: Fn(String) -> String,
    {
        println!("{}", callback(arg));
    }

    fn print_callback_result_fn(arg: String, callback: fn(String) -> String) {
        println!("{}", callback(arg));
    }

    #[test]
    fn test_mut_clousure_as_argument() {
        let mut greeting = String::from("Goodbye");
        let greeting2 = String::from("Hello");

        let mut mut_clousure = |name| {
            greeting.remove(0);
            format!("{}, {}", greeting, name)
        };
        let imut_clousure = |name| format!("{}, {}", greeting2, name);
        let fn_clousure = |name| format!("Goodbye, {}", name);

        /* FnMut() <- FnMut(), Fn(), fn()  */
        print_callback_result("Matheus".to_owned(), mut_clousure);
        print_callback_result("Matheus".to_owned(), imut_clousure);
        print_callback_result("Matheus".to_owned(), fn_clousure);
    }

    #[test]
    fn test_invalid_clousure() {
        let mut greeting = String::from("Goodbye");
        let greeting2 = String::from("Hello");

        let mut mut_clousure = |name: String| {
            greeting.remove(0);
            format!("{}, {}", greeting, name)
        };
        let imut_clousure = |name| format!("{}, {}", greeting2, name);
        let fn_clousure = |name| format!("Goodbye, {}", name);

        /* Fn() <- Fn(), fn()  */
        // print_callback_result_fn_imut("Matheus".to_owned(), mut_clousure);
        print_callback_result_fn_imut("Matheus".to_owned(), imut_clousure);
        print_callback_result_fn_imut("Matheus".to_owned(), fn_clousure);

        /* fn() <- fn() */
        // print_callback_result_fn("Matheus".to_owned(), mut_clousure);
        // print_callback_result_fn("Matheus".to_owned(), imut_clousure);
        print_callback_result_fn("Matheus".to_owned(), fn_clousure);
    }
}

mod once_clousure {
    #[test]
    fn test_once_clousure() {
        let greeting = String::from("Goodbye");

        let say_goodbye = |name: &str| {
            let mut hello = greeting;
            hello.remove(0);
            format!("{}, {}", &hello, name)
        };

        println!("{}", say_goodbye("Matheus"));
    }

    #[test]
    fn test_once_closure_twice() {
        let greeting = String::from("Goodbye");

        let say_goodbye = |name: &str| {
            let mut hello = greeting;
            hello.remove(0);
            format!("{}, {}", &hello, name)
        };

        println!("{}", say_goodbye("Matheus"));
        // println!("{}", say_goodbye("Matheus"));
    }

    #[test]
    fn test_move_syntax() {
        let greeting = String::from("Goodbye");
        let greeting2 = String::from("Hello");

        let say_goodbye = |name: String| format!("{}, {}", greeting, name);
        let say_hello = move |name: String| format!("{}, {}", greeting2, name);
        println!("{}", greeting);
        // println!("{}", greeting2);

        println!("{}", say_goodbye("Matheus".to_owned()));
        println!("{}", say_hello("Matheus".to_owned()));
        // println!("{}", say_hello("Matheus".to_owned()));
    }

    fn print_callback_result<F>(arg: String, mut callback: F)
    where
        F: FnOnce(String) -> String,
    {
        println!("{}", callback(arg));
    }

    fn print_callback_result_fn_mut<F>(arg: String, mut callback: F)
    where
        F: FnMut(String) -> String,
    {
        println!("{}", callback(arg));
    }

    fn print_callback_result_fn_imut<F>(arg: String, callback: F)
    where
        F: Fn(String) -> String,
    {
        println!("{}", callback(arg));
    }

    fn print_callback_result_fn(arg: String, callback: fn(String) -> String) {
        println!("{}", callback(arg));
    }

    #[test]
    fn test_once_clousure_as_argument() {
        let mut greeting = String::from("Goodbye");
        let greeting2 = String::from("Hello");
        let greeting3 = String::from("HHi");
        let greeting4 = String::from("Hello Again");

        let mut mut_clousure = |name: String| {
            greeting.remove(0);
            format!("{}, {}", greeting, name)
        };
        let imut_clousure = |name| format!("{}, {}", greeting2, name);
        let fn_clousure = |name| format!("Goodbye, {}", name);
        let once_clousure = |name| {
            let mut hi = greeting3;
            hi.remove(0);
            format!("{}, {}", hi, name)
        };
        let move_clousure = move |name| format!("{}, {}", greeting4, name);

        /* FnOnce() <- FnOnce(), FnMut(), Fn(), fn()  */
        print_callback_result("Matheus".to_owned(), move_clousure);
        print_callback_result("Matheus".to_owned(), once_clousure);
        print_callback_result("Matheus".to_owned(), mut_clousure);
        print_callback_result("Matheus".to_owned(), imut_clousure);
        print_callback_result("Matheus".to_owned(), fn_clousure);
    }

    #[test]
    fn test_invalid_clousure() {
        let mut greeting = String::from("Goodbye");
        let greeting2 = String::from("Hello");
        let greeting3 = String::from("HHi");
        let greeting4 = String::from("Hello Again");

        let mut mut_clousure = |name: String| {
            greeting.remove(0);
            format!("{}, {}", greeting, name)
        };
        let imut_clousure = |name| format!("{}, {}", greeting2, name);
        let fn_clousure = |name| format!("Goodbye, {}", name);
        let once_clousure = |name: String| {
            let mut hi = greeting3;
            hi.remove(0);
            format!("{}, {}", hi, name)
        };
        let move_clousure = move |name: String| format!("{}, {}", greeting4, name);

        /* FnMut() <- FnMut(), Fn(), fn()  */
        print_callback_result_fn_mut("Matheus".to_owned(), move_clousure);
        // print_callback_result_fn_mut("Matheus".to_owned(), once_clousure);
        print_callback_result_fn_mut("Matheus".to_owned(), mut_clousure);
        print_callback_result_fn_mut("Matheus".to_owned(), imut_clousure);
        print_callback_result_fn_mut("Matheus".to_owned(), fn_clousure);

        /* Fn() <- Fn(), fn()  */
        // print_callback_result_fn_imut("Matheus".to_owned(), move_clousure);
        // print_callback_result_fn_imut("Matheus".to_owned(), once_clousure);
        // print_callback_result_fn_imut("Matheus".to_owned(), mut_clousure);
        print_callback_result_fn_imut("Matheus".to_owned(), imut_clousure);
        print_callback_result_fn_imut("Matheus".to_owned(), fn_clousure);

        /* fn() <- fn() */
        // print_callback_result_fn("Matheus".to_owned(), move_clousure);
        // print_callback_result_fn("Matheus".to_owned(), once_clousure);
        // print_callback_result_fn("Matheus".to_owned(), mut_clozusure);
        // print_callback_result_fn("Matheus".to_owned(), imut_clousure);
        print_callback_result_fn("Matheus".to_owned(), fn_clousure);
    }
}

fn main() {}
