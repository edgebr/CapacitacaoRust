fn variable_imutable() {
    /* Criação de uma váriável imutável. Por padrão em Rust (e preferencialmente) todas as
       variáveis são imutáveis. */
    let a = 10;

    /* O código abaixo não compila, pois 'a' é uma váriável imutável. Ou seja, ela só pode
       ter seu valor alterado uma única vez.
       a = 20; */

    /* O código abaxo compila, pois a variável 'a' está sendo redefinida. Isso é chamado de
       shadowing. */
    let a = 20;
}

fn variable_mutable() {
    /* Criação de uma váriável mutável. A escolha de variáveis mutáveis devem ser a excessão. */
    let mut a = 20;

    /* O código abaixo compila, pois 'a' é uma variável mutável. Ou seja, ela pode ter o seu valor
     alterado várias vezes. */
    a = 10;

    /* Com a variável mutável ainda conseguimos fazer o shadowing e mudar o tipo da variável para
       imutável. Note que está sendo criada uma variável, porém estamos "reaproveitando" o mesmo
       nome: 'a' */
    let a = 30;
}

fn integers() {
    let unsigned_byte: u8;
    let unsigned_short: u16;
    let unsigned_word: u32;
    let unsigned_dword: u64;
    let unsigned_qword: u128;
    let unsigned_arch_dependent_size: usize;

    let signed_byte: i8;
    let signed_short: i16;
    let signed_word: i32;
    let signed_dword: i64;
    let signed_qword: i128;
    let signed_arch_dependent_size: isize;
}

fn float_points() {
    let single_precision: f32;
    let double_precision: f64;
}

fn type_inference() {
    let b: u8 = 21;
    let a = 21u8;
    let d = 23_u8;
    let c = 2_000;
    let hex: u32 = 0xff_ff_ff_ff;
    let octal = 0o237;
    let binary = 0b11;
}

fn chars() {
    let char_one_byte = '$';
    let char_two_bytes = '£';
    let char_three_bytes = '€';
}

#[test]
fn strings() {
    let literal_str: &str = "Hello, World!";
    let static_literal_str: &'static str = "Hello, World!";

    let str_one_byte = "$";
    let str_two_byte = "£";
    let str_three_byte = "€";

    println!("[{}] bytes: {}, letters: {}", str_one_byte, str_one_byte.len(), str_one_byte.chars().count());
    println!("[{}] bytes: {}, letters: {}", str_two_byte, str_two_byte.len(), str_two_byte.chars().count());
    println!("[{}] bytes: {}, letters: {}", str_three_byte, str_three_byte.len(), str_three_byte.chars().count());
}

fn byte_strings() {
    let literal_byte_str = b"Hello,\x20World!";
}

fn booleans() {
    let true_var = true;
    let false_var = false;
}

fn receive_array(a: [u8; 5]) { todo!() }

/// Arrays tem tamanho fixo e os tipos são iguais
fn arrays() {
    let array_type: [u8; 5];
    let array_init_with_values = [1, 2i16, 3, 4, 5];
    let array_init_with_same_value = [2u8; 5];

    let access = array_init_with_values[2];
    receive_array([3; 5]);
    receive_array([5, 4, 3, 2, 1]);
}

/// Tuplas tem tamanho fixo, mas podem ter tipos diferentes
fn tuples() {
    let tuple_same_type: (u8, u8);
    let mut tuple_mult_types: (i32, f32, char, bool);

    let number = (1);
    let tuple_one_value = (1, );
    let tuple_inference = (2, 3.2_f32, 'a', false);

    tuple_mult_types = tuple_inference;
    /* Erro de compilação */
    // tuple_mult_types = tuple_one_value;

    let access = tuple_inference.0;
}
