/// Variáveis imutáveis auxiliam o compilador em otimizações no código, pois, essas variáveis não
/// vão alterar o seu valor depois da primeira atribuição. Isso também torna o código mais seguro.
fn variable_imutable() {
    /* Criação de uma váriável imutável. Por padrão em Rust (e preferencialmente) todas as
       variáveis são imutáveis. */
    let a = 10;

    /* O código abaixo não compila, pois 'a' é uma váriável imutável. Ou seja, ela só pode
       ter seu valor alterado uma única vez. */
    // a = 20;

    /* O código abaxo compila, pois a variável 'a' está sendo redefinida. Isso é chamado de
       shadowing. */
    let a = "20";
}

/// Em alguns momentos é necessário ter variáveis que podem mudar seu valor após a primeira
/// atribuição. Isso torna as otimizações do compilador mais difíceis, porém em alguns momentos
/// é necessário.
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

/// Em rust, não existe inteiro no formato 'int', como em c. Para usar inteiros, devemos
/// especificar quantos bits queremos utilizar.
fn integers() {
    /* Inteiros sem sinal */
    let unsigned_byte: u8;      // uint8_t
    let unsigned_short: u16;    // uint16_t
    let unsigned_word: u32;     // uint32_t
    let unsigned_dword: u64;    // uint64_t
    let unsigned_qword: u128;   // Sem equivalente em C90, no C23 uint128_t
    let unsigned_arch_dependent_size: usize; // size_t

    /* Inteiros com sinal */
    let signed_byte: i8;        // int8_t
    let signed_short: i16;      // int16_t
    let signed_word: i32;       // int32_t
    let signed_dword: i64;      // int64_t
    let signed_qword: i128;     // Sem equivalente em C90, no C23 uint128_t
    let signed_arch_dependent_size: isize;  // size_t
}

/// Assim como em c, rust disponibiliza dois tipos de pontos flutuantes: f32 e f64.
fn float_points() {
    /* Ponto flutuante de 32 bits, ou, precisão simples. */
    let single_precision: f32; // float

    /* Ponto flutuante de 64 bits, ou, precisão dupla. */
    let double_precision: f64; // double
}

/// Em rust não é necessário (na maioria das vezes) dizer o tipo da variável.
/// O compilador identifica os tipos das variáveis em tempo de compilação.
fn type_inference() {
    /* Inferência do tipo a partir do valor */
    let a = 21;
    /* Definição explicita do tipo */
    let b: u8 = 21;
    /* Valor contém o tipo explicitamente */
    let a = 21u8;
    /* Utilização do caracter de separação entre o número e o tipo */
    let d = 23_u8;
    /* Utilização do caracter se separação entre as centenas do número */
    let c = 2_000;

    /* Representação em hexadecimal. Inicia com 0x */
    let hex: u32 = 0xff_ff_ff_ff;
    /* Representação em octal. Inicia com 0o */
    let octal = 0o237;
    /* Representação em binário. Inicia com 0b */
    let binary = 0b11u32;
}

/// Caracteres são sempre representados por aspas simples ''.
/// Só pode haver um caracter nas aspas simples, mesmo que essa letra ocupe mais de um byte na
/// tabela do unicode.
/// Os caracteres ocupam 4 bytes
fn chars() {
    let unicode_one_byte = '$';
    let unicode_two_bytes = '£';
    let unicode_three_bytes = '€';
    let unicode_four_bytes = '😊';
}

/// Strings são limitadas por aspas duplas "".
fn strings() {
    /* Tipo da string literal é &str */
    let literal_str: &str = "Hello, World!";

    /* String com 1 byte e 1 caracter */
    let str_one_byte = "$";
    /* String com 2 bytes e 1 caracter */
    let str_two_byte = "£";
    /* String com 3 bytes e 1 caracter */
    let str_three_byte = "€";
    /* String com 3 bytes e 1 caracter */
    let str_three_byte = "😊";
}

/// Bytestring são limitadas por aspas duplas, iniciada pela letra b (b"").
fn byte_strings() {
    /* O tipo de uma bytestring é uma referencia de um array de u8 (&[u8; N]) */
    let literal_byte_str = b"Hello,\x20World!";
}

/// Booleanos só podem assumir dois valores true ou false
fn booleans() {
    let true_var = true;
    let false_var = false;
}

/// O tipo unit é um tipo especial que possui apenas um valor: ().
/// A representação do tipo é a mesma do seu único valor: ().
/// Desta forma, quando encontramos () no código pode ser tanto o valor quanto o tipo.
/// O que vai diferenciar o tipo do valor é o contexto.
/// Esse tipo é semelhante ao tipo 'void' do c, entretanto, diferentemente do c, esse tipo possui um valor.
fn unit_type() {
    let unit = ();
}

/// Rust não permite a conversão direta de inteiros.
/// É necessário que o desenvolvedor explicite a conversão usando a sintaxe 'as'.
fn casts_integers() {
    /* Conversão de i32 para u8 */
    let a: u8 = 10i32 as u8;

    /* Erro de compilação. Não é possível fazer a conversão direta de um inteiro com sinal, para um
       inteiro sem sinal. O sentido contrário também é válido. */
    let a: u32 = 10i32;
    // let a: i32 = 10u32;
    let a: u32 = 10i32 as u32;
}

/// Com a mesma sintaxe do 'as' é possivel converter inteiros em ponto flutuantes e o inverso.
fn cast_between_float_and_int() {
    /* O valor em ponto flutuante será truncado quando convertido para um inteiro. */
    let a: u8 = 10.5_f32 as u8;

    /* A conversão de inteiro para ponto flutuante é direta. */
    let a: f32 = 10u8 as f32;
}

/// A conversão booleano para inteiro é direta usando a sintaxe 'as'.
fn cast_bool_to_int() {
    let a = true as u8;
}

/// Não é possível converter um inteiro para um booleano usando a sintaxe 'as'.
/// É necessário verificar se o inteiro difere de zero.
fn cast_int_to_bool() {
    /* Variável inteira criada */
    let a = 1u8;

    /* Erro de compilação */
    let a = a as bool;

    /* Forma correta de fazer o cast de inteiro para booleano */
    let a = a != 0 as bool;
}

/// A conversão de char para inteiro é realizada diretamente usando a sintaxe 'as'.
fn cast_char_to_int() {
    /* O valor de 'a' será 65. */
    let a = 'A' as u8;
    /* O valor de 'a' será 172. O valor é truncado, pois, o caracter '€' possui mais de um byte */
    let a = '€' as u8;
    /* O valor de 'a' será 8364. O valor possui todos os bytes do caracter '€' */
    let a = '€' as u16;
}

/// A conversão de inteiro para char é direta, usando a sintaxe 'as', se o inteiro for do tipo u8.
fn cast_int_to_char() {
    /* O valor de 'a' será 'A' */
    let a = 65u8 as char;

    /* Erro de compilação. Não é possível converter um inteiro diferente de u8 para char. */
    // let a = 65_i32 as char;
}

/// É possível obter o dígito no formato char, usando o método 'from_digits'.
/// Note que esse método é utilizado apenas para obter um único dígito, já que o char só suporta
/// apenas um único caracter.
fn digit_to_char() {
    /* Obtendo o dígito referente ao número 2 na base 10 ('2') */
    let a = char::from_digit(2, 10).unwrap();

    /* Obtendo o dígito referente ao númeor 10 na base 16 ('a') */
    let a = char::from_digit(10, 16).unwrap();

    /* Erro em runtime. Pois não é possível converter 10 em apenas um caracter. */
    let a = char::from_digit(10, 10).unwrap();
}
