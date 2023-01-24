static A: i32 = 0;
const B: i32 = 34;

#[test]
fn variable_imutable() {
    let a = 10;
    let a: i32 = 10;

    // a = 20;

    let a = "20";
}

#[test]
fn variable_mutable() {
    let mut a = 20;
    // int32_t a = 20;

    a = 10;

    let a = 30;

    let mut a = 10;
}

#[test]
fn integers() {
    let unsigned_byte: u8;      // uint8_t
    let unsigned_short: u16;    // uint16_t
    let unsigned_word: u32;     // uint32_t
    let unsigned_dword: u64;    // uint64_t
    let unsigned_qword: u128;   // Sem equivalente em C90, no C23 uint128_t
    let unsigned_arch_dependent_size: usize; // size_t

    let signed_byte: i8;        // int8_t
    let signed_short: i16;      // int16_t
    let signed_word: i32;       // int32_t
    let signed_dword: i64;      // int64_t
    let signed_qword: i128;     // Sem equivalente em C90, no C23 uint128_t
    let signed_arch_dependent_size: isize;  // size_t
}

#[test]
fn float_points() {
    let single_precision: f32; // float

    let double_precision: f64; // double
}

#[test]
fn type_inference() {
    let a = 21;
    let b: u8 = 21;

    let a = 21u8;
    let d = 23_u8;

    let c = 2_000;
    let hex: u32 = 0xff_ff_ff_ff;
    let octal = 0o237;
    let binary = 0b11u32;
}

#[test]
fn chars() {
    let unicode_one_byte = '$';
    let unicode_two_bytes = '£';
    let unicode_three_bytes = '€';
    let unicode_four_bytes = '😊';
}

#[test]
fn strings() {
    let literal_str: &str = "Hello, World!";

    let str_one_byte = "$";
    let str_two_byte = "£";
    let str_three_byte = "€";
    let str_four_byte = "😊";

    println!("byetes: {}, len: {}", str_one_byte.len(), str_one_byte.chars().count());
    println!("byetes: {}, len: {}", str_two_byte.len(), str_two_byte.chars().count());
    println!("byetes: {}, len: {}", str_three_byte.len(), str_three_byte.chars().count());
    println!("byetes: {}, len: {}", str_four_byte.len(), str_four_byte.chars().count());
}

#[test]
fn byte_strings() {
    let literal_byte_str = b"Hello,\x20World!";
}

#[test]
fn booleans() {
    let true_var = true;
    let false_var = false;
}

#[test]
fn unit_type() {
    let unit = ();
}

#[test]
fn casts_integers() {
    let a: u8 = 10i32 as u8;

    // let a: u32 = 10i32;
    // let a: i32 = 10u32;
    let a: u32 = 10i32 as u32;
}

#[test]
fn cast_between_float_and_int() {
    let a: u8 = 10.5_f32 as u8;

    let a: f32 = 10u8 as f32;
}

#[test]
fn cast_bool_to_int() {
    let a = true as u8;
}

#[test]
fn cast_int_to_bool() {
    let a = 1u8;

    // let a = a as bool;

    let a = a != 0;
}

#[test]
fn cast_char_to_int() {
    let a = 'A' as u8;
    let a = '€' as u8;
    let a = '€' as u16;
}

#[test]
fn cast_int_to_char() {
    let a = 65u8 as char;

    // let a = 65_i32 as char;
}

#[test]
fn digit_to_char() {
    let a = char::from_digit(2, 10).unwrap();

    let a = char::from_digit(10, 16).unwrap();

    let a = char::from_digit(10, 10).unwrap();
}

#[test]
fn array_type() {
    let array_type: [u8; 5]; // u8 var_name[5]
}

#[test]
fn init_array_with_values() {
    let array_init_with_values = [1, 2i16, 3, 4, 5];

    // println!("{}", array_init_with_values);
}

#[test]
fn init_array_with_same_value() {
    let array_init_with_same_value = [2u8; 5];

    println!("{:?}", array_init_with_same_value);
}

#[test]
fn acess_array_element() {
    let array = [2u8; 5];
    let index = 6;

    println!("{}", array[1]);
    println!("{:?}", array.get(6));
    // println!("{}", array[6]);
    println!("{}", array[index]);
}

fn receive_array(a: [u8; 5]) {}

#[test]
fn passing_array_to_func() {
    receive_array([3; 5]);
    // receive_array([3; 4]);
}

#[test]
fn tuples_same_type() {
    let tuple_same_type: (u8, u8);
    let tuple_same_type = (2u8, 1u8);
}

#[test]
fn tuples_mult_type() {
    let tuple_mult_types: (i32, f32, char, bool);
    let tuple_mult_types = (0i32, 1.0_f32, 'a', false);
}

#[test]
fn tuple_one_position() {
    let number = (1);
    let tuple_one_position = (1, );
}

#[test]
fn invalid_tuple_assing() {
    let mut tuple1 = ("1", 3);
    let tuple2 = (2, 5);

    // tuple1 = tuple2;
}

#[test]
fn tuple_access() {
    let tuple = (0i32, 1.0_f32, 'a', false);

    println!("{}", tuple.2);
    // println!("{}", tuple.4);
}

#[test]
fn array_advantages() {
    let a = [0u32; 1000];
    let i = 3;

    println!("{}", a[i]);
}

#[test]
fn tuple_advantages() {
    let a = (0i32, 0f32, false, 'b', [5u8; 100]);

    println!("{}", a.3);
}

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

    let s = &a[..]; // a[2:4]
    println!("{:?}", s);

    let s = &a[2..4];
    println!("{:?}", s);
}

#[test]
fn string_slice() {
    let a = String::from("Hello");

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

    let r = 0..10;
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

fn move_argument(a: [i32; 5]) {}

fn dont_move_argument(a: &[i32]) {}

#[test]
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

// fn invalid_return_ref(a: &i32) -> &i32 {
//     let b = 10;
//     &b
// }

// fn invalid_default_value(a: i32 = 0) {}
