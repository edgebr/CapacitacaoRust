fn array_type() {
    let array_type: [u8; 5]; // u8 var_name[5]
}

fn init_array_with_values() {
    let array_init_with_values = [1, 2i16, 3, 4, 5];

    println!("{}", array_init_with_values);
}

fn init_array_with_same_value() {
    let array_init_with_same_value = [2u8; 5];

    println!("{:?}", array_init_with_values);
}

fn acess_array_element() {
    let array = [2u8; 5];

    println!("{}", array[1]);
    println!("{:?}", array.get(6));
    println!("{}", array[6]);
}

fn receive_array(a: [u8; 5]) {}

fn passing_array_to_func() {
    receive_array([3; 5]);
    receive_array([3; 4]);
}

fn tuples_same_type() {
    let tuple_same_type: (u8, u8);
    let tuple_same_type = (2u8, 1u8);
}

fn tuples_mult_type() {
    let tuple_mult_types: (i32, f32, char, bool);
    let tuple_mult_types = (0i32, 1.0_f32, 'a', false);
}

fn tuple_one_position() {
    let number = (1);
    let tuple_one_position = (1, );
}

fn invalid_tuple_assing() {
    let mut tuple1 = ("1", 3);
    let tuple2 = (2, 5);

    tuple1 = tuple2;
}

fn tuple_access() {
    let tuple = (0i32, 1.0_f32, 'a', false);

    println!("{}", tuple.2);
    println!("{}", tuple.4);
}

fn array_advantages() {
    let a = [0u32; 1000];
    let i = 3;

    println!("{}", a[i]);
}

fn tuple_advantages() {
    let a = (0i32, 0f32, false, 'b', [5u8; 100]);

    println!("{}", a.3);
}
