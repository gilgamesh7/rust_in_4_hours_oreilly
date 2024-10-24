fn main() {
    demo_passing_values();

    demo_passing_by_value_or_borrow();

    demo_mutable_reference();
}

fn demo_passing_values() {
    let mut x: isize = 10;
    let y: String = String::from("hello");
    
    some_func(x, y);
    println!("Value of x is {}", x);
    // println!("Value of y is {}", y);
}

fn some_func(mut x: isize, y: String) {
    println!("Value of x is {}", x);
    println!("Value of y is {}", y);

    x = 20;
}

fn demo_passing_by_value_or_borrow() {
    let mut x: isize = 10;
    let y: String = String::from("hello");
    
    some_func_2(&x, &y);

    println!("Value of x is {}", x);
    println!("Value of y is {}", y);
}

fn some_func_2(x: &isize, y: &String) {
    println!("Value of x is {}", x);
    println!("Value of y is {}", y);

    println!("Value of x is {}", *x);
    println!("Value of y is {}", *y);

    println!("Value of x is {:p}", x); 
    println!("Value of y is {:p}", y);

}

fn demo_mutable_reference() {
    let mut x: isize = 10;
    let mut y: String = String::from("hello");

    some_func_3(&mut x, &mut y);

    println!("Value of x is {}", x);
    println!("Value of y is {}", y);
}

fn some_func_3(x: &mut isize, y: &mut String) {
    *x = 20;
    (*y).push_str(" world");
}
