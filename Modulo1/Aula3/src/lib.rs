#[test]
fn immutable_ref() {
    let a = 10;

    let b = &a;
    let c: &i32 = &a;
}

#[test]
fn mutable_ref() {
    let mut a = 10;

    let b = &mut a;
    let c: &mut i32 = &mut a;

    let d = &a;
}

#[test]
fn deref() {
    let a = 10;
    let b = &a;

    println!("a: {}", *b);
}

#[test]
fn deref_mut() {
    let mut a = 10;
    let b = &mut a;

    *b = 20;

    println!("a: {}", a);
}

#[test]
fn illegal_ref() {
    let a = 10;
    // let b = &mut a;
}

#[test]
fn illegal_deref() {
    let a = 10;
    let b = &a;

    // *b = 20;

    println!("a: {}", a);
}

#[test]
fn dangling_ref() {
    // let a: &i32;
    // {
    //     let b = 10;
    //     a = &b;
    // }
    // println!("a: {}", a);
}

#[test]
fn basic_slice() {
    let a = [1, 2, 3, 4, 5, 6];
    println!("a: {:?}", a);

    let s = &a[..]; // a[:]
    println!("{:?}", s);

    let a = &a[..2];
    println!("{:?}", s);

    let s = &a[2..4]; // a[2:4]
    println!("{:?}", s);
}

#[test]
fn string_slice() {
    let a = String::from("Hello");
    let a = "Hello".to_string();

    let b = &a;
    let c: &str = &a;
    println!("b: {}, c: {}", b, c);

    let d = "World";
    // let e: &String = d;

    let f = &d[..];
    let g = &d[1..3];
    println!("f: {}, g: {}", f, g);
}

#[test]
fn range() {
    for n in 0..10 {
        println!("{}", n);
    }

    let r = 0..10u8;
    for n in r {
        println!("{}", n);
    }
}

#[test]
fn inclusive_range() {
    for n in 0..=10 {
        println!("{}", n);
    }
}

fn basic() {
    println!("Hello");
}

fn many_argument(a1: i32, a2: u8, a3: bool) {
    basic();
}

fn many_argument_without_warning(a1: i32, a2: u8, a3: bool, _c: char) {
    many_argument(a1, a2, a3);
}

fn ref_argument(n: &i32) {}

fn mut_ref_argument(n: &mut i32) {}

#[test]
fn call_ref_arguments() {
    let a = 4;
    let mut b = 3;
    let mut c = 6;
    let d = &a;

    ref_argument(&a);
    mut_ref_argument(&mut b);
    ref_argument(&c);
    ref_argument(d);
}

fn move_argument(a: Vec<i32>) {}

fn dont_move_argument(a: &Vec<i32>) {}

#[test]
fn call_move_argument() {
    let a = vec![0, 0, 0, 0, 0];
    let b = vec![1, 1, 1, 1, 1];

    move_argument(a);
    // println!("{:?}", a);
    dont_move_argument(&b);
    println!("{:?}", b);
}

fn no_name_argument(_: Vec<i32>) {}

fn mut_argument(mut v: Vec<i32>) {
    v.push(1);
    println!("v {:?}", v);
}

#[test]
fn call_mut_argument() {
    let v = vec![];

    mut_argument(v);
    no_name_argument(vec![]);

    let u = vec![];
    let mut w = u;

    w.push(1);
    println!("w {:?}", w);
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn hello() {
    println!("Hello");
}

fn unit_return() -> () {}

fn empty() {
    return ();
}

fn with_result() -> i32 {
    return 0;
}

fn with_result_no_semicolon() -> i32 {
    let a = 20;
    
    a
}

fn return_ref(a: &i32) -> &i32 {
    a
}

// fn invalid_return_ref(a: &i32) -> &i32 {
    // let b = 10;
    // &b
// }

// fn invalid_default_value(a: i32 = 0) {}

static FORTY_TWO: u8 = const_function();

const fn const_function() -> u8 {
    42
}

fn bar() -> i32 {
    1
}

fn receive_func(callback: fn() -> i32) -> i32 {
    callback()
}

#[test]
fn foo() {
    let mut a = 10;
    {
        let b = &mut a;

        let recv = receive_func(bar);
        println!("recv: {}", recv);

        println!("{}", b);
    }

    let c = &a;
    println!("{}", c);
}
