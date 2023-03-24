fn main() {
    println!("Hello, world!");
}
mod tests {
    mod cell {
        use std::cell::Cell;

        #[test]
        fn cell() {
            #[derive(Debug)]
            struct BankAccount {
                name: Cell<i32>,
                balance: Cell<f64>,
            }
            impl BankAccount {
                pub fn new(name: i32, balance: f64) -> Self {
                    Self {
                        name: Cell::new(name),
                        balance: Cell::new(balance),
                    }
                }
                pub fn inc_balance(&self) {
                    let balance = self.balance.get();
                    //
                    self.balance.set(balance + 1.0);
                }
            }
            let ba = BankAccount::new(30, 100.0);
            ba.balance.set(200.0);
            ba.inc_balance();
            dbg!(ba.name.get());
            dbg!(ba);
        }
    }

    mod refcell {
        use std::cell::{Cell, RefCell};
        #[test]
        fn refcell() {
            #[derive(Debug)]
            struct BankAccount {
                name: RefCell<String>,
                balance: Cell<f64>,
            }
            impl BankAccount {
                pub fn new(name: &str, balance: f64) -> Self {
                    Self {
                        name: RefCell::new(name.to_owned()),
                        balance: Cell::new(balance),
                    }
                }

                pub fn set_balance(&self, new_balance: f64) -> f64 {
                    // let old_balance = self.balance.get();
                    // self.balance.set(new_balance);
                    // old_balance
                    self.balance.replace(new_balance)
                }

                pub fn set_name(&self, new_name: &str) -> String {
                    // let old_name = self.name.borrow();
                    // *self.name.borrow_mut() = new_name.to_string();
                    self.name.replace(new_name.to_owned())
                    // old_name.to_owned()
                }
            }
            let ba = BankAccount::new("Jack", 100.0);
            dbg!(&ba);
            ba.set_balance(150.0);
            ba.set_name("Joe");
            dbg!(ba);
        }
    }

    mod rc {
        use std::cell::RefCell;
        use std::rc::Rc;

        #[test]
        fn rc_101() {
            let a;
            {
                let b = Box::new(10);
                println!("raw Box(b) {:p}", b);
                a = b.clone();
                println!("raw Box(a) {:p}", a);
            }
            println!("a is {}", a);

            let a;
            {
                let b = Rc::new(10);
                println!("raw Rc(b) {:p}", b);
                a = b.clone(); // Rc::clone(&b);
                println!("raw Rc(a) {:p}", a);
                dbg!(Rc::strong_count(&a));
            }
            dbg!(Rc::strong_count(&a));
            println!("a as Rc<i32>: {}", a);
            // *a = 5;

            let a;
            {
                let b = Rc::new(RefCell::new(10));
                println!("raw Rc<RefCell<(b)> {:p}", b);
                a = b.clone();
                println!("raw Rc<RefCell<(a)> {:p}", a);
                dbg!(Rc::strong_count(&a));
            }
            dbg!(Rc::strong_count(&a));
            println!("a as Rc<RefCell<i32>>: {}", a.borrow());
            // *a = 5;
            *a.borrow_mut() = 6;
            a.replace(5);
            println!("a as Rc<RefCell<i32>> after borrow_mut: {}", a.borrow());

            #[derive(Debug)]
            struct Dog {
                name: String,
            }
            impl Dog {
                pub fn new_rc_refcell_empty_dog_vec() -> Rc<RefCell<Vec<Self>>> {
                    Rc::new(RefCell::new(Vec::<Dog>::new()))
                }
            }

            let dogs = Rc::new(RefCell::new(Vec::<Dog>::with_capacity(4)));
            dogs.borrow_mut().push(Dog {
                name: "Rex".to_string(),
            });
            dogs.borrow_mut().push(Dog {
                name: "Toby".to_string(),
            });
            dogs.borrow_mut().push(Dog {
                name: "Bob".to_string(),
            });
            dogs.borrow_mut().push(Dog {
                name: "Lessy".to_string(),
            });

            dogs.borrow().first().and_then(|dog| {
                println!("First dog's name: {}", dog.name);
                Some(dog)
            });

            dbg!(dogs.borrow());

            dbg!(dogs);

            let dogs = Dog::new_rc_refcell_empty_dog_vec();
            dogs.borrow_mut().push(Dog {
                name: "Bradoc".to_string(),
            });
            dbg!(dogs.borrow());
        }
    }
}
