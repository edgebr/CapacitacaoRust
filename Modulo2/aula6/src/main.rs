// Aula 5.2
fn main() {
    println!("Hello, world!");
}

mod vector {
    #[test]
    fn vec_101() {
        let mut a = Vec::<i32>::new();
        dbg!(a.capacity());
        a.push(10);
        dbg!(a.capacity());
        dbg!(&a);

        let mut a = Vec::<i32>::with_capacity(200);
        a.shrink_to(2);

        a.insert(0, 10);
        a.push(20);
        dbg!(&a);
        a.pop();
        a.remove(0);
        dbg!(&a);
    }

    #[test]
    fn vec_macro() {
        let b = vec![1, 2, 3, 4];
        dbg!(b);
        let b = vec![1_u64, 2, 3, 4];
        dbg!(b);
        let b = vec![true, false, false, true];
        dbg!(b);
    }

    #[test]
    fn vec_slice() {
        let b = vec![1, 2, 3, 4];
        let x = &b[..];
        dbg!(x);
        dbg!(x[1]);
        dbg!(&b[..2]);
        dbg!(&b[2..=3]);
    }

    #[test]
    fn vec_access() {
        let mut b = vec![1, 2, 3, 4];

        let mut l = 0;
        println!("--- By square brackets while");
        while l < b.len() {
            println!("{}", b[l]);
            l += 1;
        }

        println!("--- By square brackets for");
        for i in 0..b.len() {
            println!("{}", b[i]);
        }

        let mut l = 0;
        println!("--- By getting while");
        while let Some(n) = b.get(l) {
            println!("{}", n);
            l += 1;
        }

        println!("--- By iterator");
        for n in b.iter() {
            println!("{n}");
        }

        println!("--- By enumerate");
        for (index, n) in b.iter().enumerate() {
            println!("[{index}]:{n}");
        }

        println!("--- By iter mut");
        for n in b.iter_mut() {
            *n += 1
        }
        dbg!(&b);
    }

    #[test]
    fn vec_fifo() {
        let mut a = Vec::<i32>::new();
        a.insert(0, 10);
        a.insert(0, 20);
        a.insert(0, 30);
        dbg!(&a);
        dbg!(a.pop());
        dbg!(a.pop());
        dbg!(a.pop());
        dbg!(&a);
    }

    #[test]
    fn vec_lifo() {
        let mut a = Vec::<i32>::new();
        a.push(10);
        a.push(20);
        a.push(30);
        dbg!(&a);
        dbg!(a.pop());
        dbg!(a.pop());
        dbg!(a.pop());
        dbg!(&a);
    }

    #[test]
    fn vec_rev_sort() {
        let mut b = vec![1, 2, 3, 4];
        dbg!(&b);
        b.reverse();
        dbg!(&b);
        b.sort(); //timsort
        dbg!(&b);
    }
}

mod string {
    #[test]
    fn string_101() {
        let _s = String::from("Hello World!");
        let _s = "Hello World!".to_string();
        let _s = String::from_iter(vec![
            'H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd', '!',
        ]);
        let v = vec![
            b'H', b'e', b'l', b'l', b'o', b' ', b'W', b'o', b'r', b'l', b'd', b'!',
        ];
        let _s = String::from_utf8_lossy(&v[..]);
        let s: String = "Hello World!".to_owned();
        dbg!(s);
    }

    #[test]
    fn string_usage() {
        let mut s = String::from("Hello World!");
        dbg!(&s.capacity());
        dbg!(&s.starts_with("He"));
        s.push_str(" Welcome!");
        dbg!(&s);
        let pieces: Vec<&str> = s.split("!").collect(); // from str
        dbg!(pieces);
        let mut a: String = "Hello ".to_string();
        a += "World";
        dbg!(a);
    }
}

// Aula 6.1

mod closure {
    #[test]
    fn closure_101() {
        // Introdução a closures
        let f = |x| x * 2;
        dbg!(f(10));

        let g = |x, y: i32| {
            if x % 2 == 0 {
                x * y
            } else {
                x + y
            }
        };
        dbg!(g(10, 5));
        dbg!(g(5, 2));
    }
}

mod option {
    #[test]
    fn option() {
        let mut x: Option<f64> = None;
        dbg!(x.is_none());
        // dbg!(x.unwrap());

        let y: Option<f64> = Some(5.6);
        dbg!(y.is_none());
        if dbg!(y.is_some()) {
            dbg!(y.unwrap());
        }

        let a = 10.0;
        dbg!(x.unwrap_or_else(|| a + 0.1));
        dbg!(x.unwrap_or_default());

        dbg!(y.and(x)); // -> None
        dbg!(y.or(x)); // -> Some
        dbg!(y.xor(x)); // -> Some

        let f = |x| Some(x + 10.0);
        dbg!(x.and_then(f).or(Some(1.5)));
        dbg!(y.and_then(|x| Some(x + 10.0)));
        dbg!(x.insert(6.6));
        dbg!(x.is_some());

        #[derive(Debug)]
        struct User {
            name: String,
        }
        impl User {
            pub fn new(name: &str) -> Self {
                Self {
                    name: name.to_string(),
                }
            }
        }

        let mut users = [User::new("Jack Noname"), User::new("Mary Bridge")];
        let mut index = 0;

        // for user in users.iter_mut() {
        //     if user.name.starts_with("Jack") {
        //         user.name = "Jack Shoe".to_string();
        //     }
        // }
        users.iter_mut().for_each(|user| {
            if user.name.starts_with("Jack") {
                user.name = "Jack Shoe".to_string();
            }
        });

        // loop {
        //     match users.get_mut(index) {
        //         Some(User { name }) if name.starts_with("Jack") => {
        //             *name = "Jack Shoe".to_string();
        //         }
        //         Some(_) => {}
        //         None => {
        //             break;
        //         }
        //     }
        //     index += 1;
        // }
        dbg!(users);

        // Jeito funcional de fazer
        let mut users = [
            User::new("Jack Jack"),
            User::new("Mary Bridge"),
            User::new("Joe Blank"),
        ];
        users.iter_mut().for_each(|user| {
            if user.name.starts_with("Jack") {
                user.name = String::from("Jack Shoe");
            }
        });
        dbg!(&users);
        if users
            .iter()
            .position(|user| user.name.starts_with("Jack"))
            .and_then(|index| Some(users[index].name = "Jack Shoes".to_owned()))
            .is_some()
        {
            dbg!(&users);
        }
    }
}

#[allow(unused_imports)]
mod hashmap {
    use std::collections::HashMap;

    #[test]
    fn hashmap_101() {
        let mut h = HashMap::<&str, Vec<u8>>::new();
        h.insert("Jack", vec![1, 2, 3]);
        dbg!(&h);
        h.insert("Joe", vec![10, 20, 30, 100]);
        dbg!(&h);
        h.insert("Mary", vec![100, 200, 210, 220]);
        dbg!(&h);

        dbg!(h.contains_key("Mary"));

        dbg!(h.get_key_value("Mary"));

        dbg!(h.entry("Mary"));

        h.entry("Alexa")
            .and_modify(|v| v.push(80))
            .or_insert(vec![50, 60, 70]);

        match h.get_mut("Joe") {
            Some(x) => x.push(200),
            None => println!("No Joe found"),
        }

        dbg!(h.remove("Jack"));
        dbg!(h.remove("Jack"));
    }
}

#[allow(unused_variables)]
#[allow(dead_code)]
mod box_ {

    // int *p = malloc(32 * size_of(int));
    //
    // free..

    #[test]
    fn box_101() {
        fn inc_first(slice: &mut [u32]) {
            slice[0] += 1
        }

        let mut array = [10u32; 3]; // Pilha
        array[0] = 7;
        dbg!(&array);
        inc_first(&mut array);
        dbg!(&array);

        let mut heap_array = Box::new([10u32; 3]);
        heap_array[0] = 7;
        dbg!(&heap_array);
        inc_first(&mut *heap_array);
        inc_first(heap_array.as_mut());
        dbg!(&heap_array);

        let mut heap_array2 = Box::new(Box::new(Box::new(Box::new([10; 3]))));
        heap_array2[0] = 7;
        dbg!(&heap_array2);
        dbg!(heap_array2.len());
        inc_first(&mut ****heap_array2);
        inc_first(heap_array2.as_mut().as_mut().as_mut().as_mut());
        dbg!(&heap_array2);

        enum Variants {
            Type0,
            Type1(u8),
            Type2(f32),
            Type3([u32; 1000]),
        }
        dbg!(std::mem::size_of::<Variants>());

        enum Variants2 {
            Type1(u8),
            Type2(f32),
            Type3(Box<[u32; 1000]>),
        }
        dbg!(std::mem::size_of::<Variants2>());
    }

    #[test]
    fn box_recursive_types() {
        #[derive(Debug, Default)]
        struct Node<V> {
            value: V,
            child: Option<Box<Node<V>>>,
        }

        impl<V> Node<V> {
            pub fn new(value: V, child: Option<Box<Self>>) -> Self {
                Self { value, child }
            }
        }

        impl<V> FromIterator<V> for Node<V> {
            // fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
            //     let mut node_list = Vec::<Node<V>>::new();
            //     for value in iter.into_iter() {
            //         node_list.push(Node::<V>::new(value, None));
            //     }
            //     node_list.reverse();
            //     let mut child = Some(Box::from(node_list.remove(0)));
            //     let mut node = node_list.remove(0);
            //     loop {
            //         node.child = child;
            //         if node_list.len() == 0 {
            //             break;
            //         }
            //         child = Some(Box::from(node));
            //         node = node_list.remove(0);
            //     }
            //     node
            // }
            fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
                iter.into_iter()
                    .map(|v| Node::<V>::new(v, None))
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .reduce(|child, mut node| {
                        node.child = Some(Box::from(child));
                        node
                    })
                    .unwrap()
            }
        }

        let n = Node::new(10, None);
        dbg!(&n);
        // dbg!(Node::from_iter([1, 2, 3, 4, 5]));
        dbg!(Node::from_iter(vec![1, 2, 3, 4, 5]));
        // dbg!(Node::from_iter("Hello world!".chars()));
    }
}
