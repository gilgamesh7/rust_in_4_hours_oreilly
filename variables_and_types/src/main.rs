use std::mem;

fn main() {
    demo_integers();

    demo_floats();

    demo_simple_datatypes();

    demo_techniques();
}

fn demo_integers() {
    let example_i32_int: i32 = -12345;
    let example_isize_int: isize = -12346;

    let example_u32_int: u32 = 12347;
    let example_usize_int: usize = 12348;

    println!(
        "\n Numbers are : {} , {}, {}, {} \n",
        example_i32_int, example_isize_int, example_u32_int, example_usize_int
    );
    println!(
        "\n Numbers in reverse are : {3} , {2} , {1} , {0}",
        example_i32_int, example_isize_int, example_u32_int, example_usize_int
    );
    println!(
        "\n Size of example_i32_int : {}",
        mem::size_of_val(&example_i32_int)
    );
    println!(
        "\n Size of example_isize_int : {}",
        mem::size_of_val(&example_isize_int)
    );
    println!(
        "\n Size of example_usize_int : {}",
        mem::size_of_val(&example_usize_int)
    );
    println!("usize is {} bytes on my machine", mem::size_of::<isize>());
}

fn demo_floats() {
    let example_f32 = -1.6e19;

    println!("\nValue of example_f32 is {0}\n", example_f32);
    println!("Size of f64 floatt is {0}", mem::size_of::<usize>());
    println!(
        "Size allocated to example_f32 is {0}",
        mem::size_of_val(&example_f32)
    );
}

fn demo_simple_datatypes() {
    let is_cat: bool = true;

    println!("Is it true that Maya is a cat ? {0}", is_cat);
}

fn demo_techniques() {
    let mut my_char: char = 'X';
    println!("Value of mychar is {0}", my_char);
    my_char = 'Y';
    println!("Value of mychar is {0}", my_char);

    let _my_float: f32 = 3.14;

    let g: i8 = 123;
    let mut h: f32 = 3.14;
    println!("Size of g is {0} and size of h is {1}", mem::size_of_val(&g), mem::size_of_val(&h));

    h = g as f32;
    println!("value of g is {0} and h is {1}",g,h);
    println!("Size of g is {0} and size of h is {1}", mem::size_of_val(&g), mem::size_of_val(&h));

    const SECONDS_IN_HOUR : i32 = 3_600;
    const HOURS_IN_DAY: i32 = 24;
    println!("Seconds in a day : {0}", SECONDS_IN_HOUR*HOURS_IN_DAY);

}
