mod iterator_trait {}

mod from_and_into_traits {}

mod read_and_write_traits {}

mod operator_overload {}

mod default_trait {}

mod drop_trait {}

mod dynamic_dispatch2 {
    // TODO Pesquisar!!
    mod object_safety {}
}

mod exercicio {
    #[derive(Debug, PartialEq, Eq)]
    #[allow(unused)]
    pub enum Comparison {
        Equal,
        Sublist,
        Superlist,
        Unequal,
    }

    #[allow(unused)]
    pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
        todo!()
    }

    #[test]
    fn test_exercicio() {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
