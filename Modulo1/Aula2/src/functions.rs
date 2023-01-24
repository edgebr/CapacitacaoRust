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

fn move_argument(a: [i32; 5]) {}

fn dont_move_argument(a: &[i32]) {}

fn call_move_argument() {
    let a = [0i32; 5];
    let b = [1i32; 5];

    move_argument(a);
    println!("{:?}", a);
    dont_move_argument(&b);
    println!("{:?}", b);
}

// TODO Estudar
fn mut_argument(mut n: i32) {}

fn unit_return() -> () {}

fn empty() {
    return ();
}

fn with_result() -> i32 {
    return 0;
}

fn with_result_no_semicolon() -> i32 {
    0
}

fn return_ref(a: &i32) -> &i32 {
    a
}

fn invalid_return_ref(a: &i32) -> &i32 {
    let b = 10;
    &b
}

fn invalid_default_value(a: i32 = 0) {}
