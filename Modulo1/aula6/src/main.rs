fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    /*
    Structs
    Tuple Structs
    Field Shorthand Syntax
    Enums
    Variant Payloads
    Enum Sizes
    Methods
    Method Receiver
         */

    #[derive(Debug)]
    struct Person {
        first_name: String,
        last_name: String,
        cpf: String,
        age: u8,
    }

    #[test]
    fn structs_101() {
        let fulano = Person {
            first_name: "Fulano".to_string(),
            last_name: "de Tal".to_string(),
            cpf: "040.305.033-90".to_string(),
            age: 30,
        };

        dbg!(&fulano);

        println!(
            "\nName: {} {}\nCPF: {}\nAge: {}\n",
            fulano.first_name, fulano.last_name, fulano.cpf, fulano.age
        );
    }

    #[derive(Debug)]
    struct EngPI;

    impl EngPI {
        fn value() -> i32 {
            3
        }
    }

    #[test]
    fn structs_unit() {
        dbg!(EngPI {});
        println!("π = {}", EngPI::value());
    }

    #[derive(Debug, Default)]
    struct Point(i32, i32);

    #[test]
    fn structs_tuple() {
        let p = Point(10, 20);
        dbg!(&p);
        println!("result = {}", p.0 + p.1)
    }

    #[derive(Debug, Default)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    impl Rectangle {
        fn area(&self) -> i32 {
            (self.bottom_right.0 - self.top_left.0) * (self.bottom_right.1 - self.top_left.1)
        }
    }

    #[test]
    fn structs_composition() {
        let rec = Rectangle {
            top_left: Point(0, 0),
            bottom_right: Point(10, 10),
        };

        dbg!(&rec);
        println!("Area rec = {}m²", rec.area());

        let top_left = Point(100, 50);

        let rec2 = Rectangle {
            top_left: top_left, //shorthand syntax
            bottom_right: Point(101, 100),
        };
        dbg!(&rec2);
        println!("Area rec2 = {}m²", rec2.area());

        let rec3 = Rectangle {
            bottom_right: Point(1000, 1000),
            ..rec2 //all the other fields of rec2
        };

        dbg!(&rec3);
        println!("Area rec3 = {}m²", rec3.area());

        let rec4 = Rectangle {
            bottom_right: Point(100, 200),
            ..Default::default() //all the other fields of rec2
        };

        dbg!(&rec4);
        println!("Area rec4 = {}m²", rec4.area());

        let mut rec5: Rectangle = Default::default();
        rec5.top_left = Point(0, 30);
        rec5.bottom_right.0 = 10;
        rec5.bottom_right.1 = 50;

        dbg!(&rec5);
        println!("Area rec5 = {}m²", rec5.area());
    }
}
