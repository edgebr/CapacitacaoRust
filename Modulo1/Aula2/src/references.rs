fn immutable_ref() {
    let a = 10;

    let b = &a;
    let c: &i32 = &a;
}

fn mutable_ref() {
    let mut a = 10;

    let b = &mut a;
    let c: &mut i32 = &mut a;

    let d = &a;
}

fn deref() {
    let a = 10;
    let b = &a;

    println!("a: {}", *b);
}

fn deref_mut() {
    let mut a = 10;
    let b = &mut a;

    *b = 20;

    println!("a: {}", a);
}

fn illegal_ref() {
    let a = 10;
    let b = &mut a;
}

fn illegal_deref() {
    let a = 10;
    let b = &a;

    *b = 20;

    println!("a: {}", a);
}

fn dangling_ref() {
    let a: &i32;
    {
        let b = 10;
        a = &b;
    }
    println!("a: {}", a);
}
