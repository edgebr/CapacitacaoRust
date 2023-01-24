fn basic_slice() {
    let a = [1, 2, 3, 4, 5, 6];
    println!("a: {:?}", a);

    let s = &a[..]; // a[2:4]
    println!("{:?}", s);

    let s = &a[2..4];
    println!("{:?}", s);
}

fn string_slice() {
    let a = String::from("Hello");

    let b = &a;
    let c: &str = &a;
    println!("b: {}, c: {}", b, c);

    let d = "World";
    let e: &String = d;

    let f = &d[..];
    let g = &d[1..3];
    println!("f: {}, g: {}", f, g);
}

fn range() {
    for n in 0..10 {
        println!("{}", i);
    }

    let r = 0..10;
    for n in r {
        println!("{}", i);
    }
}

fn inclusive_range() {
    for n in 0..=10 {
        println!("{}", i);
    }
}
