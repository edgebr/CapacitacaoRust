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

    let a = 10.5;

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

    "hello".len();

    println!("bytes: {}, len: {}", str_one_byte.len(), str_one_byte.chars().count());
    println!("bytes: {}, len: {}", str_two_byte.len(), str_two_byte.chars().count());
    println!("bytes: {}, len: {}", str_three_byte.len(), str_three_byte.chars().count());
    println!("bytes: {}, len: {}", str_four_byte.len(), str_four_byte.chars().count());
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
    let unit: () = ();
}

#[test]
fn casts_integers() {
    let a: u8 = 10i32 as u8;

    let a: u16 = (1u8 as u16) + 1u16;

    a.ilog2();

    // let a: u32 = 10i32;
    // let a: i32 = 10u32;
    let a: u32 = 10i32 as u32;
}

#[test]
fn cast_between_float_and_int() {
    let a: u8 = 10.5_f32.round() as u8;

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
    let matrix: [[u8; 5]; 5];


    // let a = array_type[0];
}

#[test]
fn init_array_with_values() {
    let array_init_with_values = [1, 2i16, 3, 4, 5];

    println!("10: {:?}", array_init_with_values);
}

#[test]
fn init_array_with_same_value() {
    let array_init_with_same_value = [2u8; 5];
    let array = [false; 1000];

    println!("{:?}", array_init_with_same_value);
}

#[test]
fn acess_array_element() {
    let array = [2u8; 5];
    let index = 6;

    println!("{}", array[1]);
    // println!("{:?}", array.get(6));
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
    let mut tuple = (0i32, 1.0_f32, 'a', false);

    tuple.0 = 1;

    println!("{}", tuple.0);
    // println!("{}", tuple.4);
}

#[test]
fn array_advantages() {
    let a = [0u32; 1000];
    let i = 3;

    println!("{}", a[i]);
}

#[test]
fn create_string() {
    let hello = "Hello, World!";
    println!("{}", hello);

    let hello = String::from("Hello, World!");
    println!("{}", hello);
    let world = hello + "!!!!";
    assert_eq!(world, "Hello, World!!!!!");

    println!("{world}");

    let hello = "Hello, World!".to_string();
    println!("{}", hello);
}

#[test]
fn tuple_advantages() {
    let a = (0i32, 0f32, false, 'b', [5u8; 100]);

    println!("{}", a.3);
}
