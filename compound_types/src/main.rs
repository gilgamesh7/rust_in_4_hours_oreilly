fn main() {
    let arr_1 = [2, 3, 4, 5];

    println!("arr_1 : {:?} of length {1}", arr_1, arr_1.len());

    let mut arr_2 = [1, 2, 3, 4, 5];
    arr_2[0] = 10;
    println!("arr_2 : {:?} of length {1}", arr_2, arr_2.len());

    let arr_3 : [String; 3];
    arr_3 = [String::from("hello"), String::from("world"), String::from("rust")];

    println!("arr_3 : {:?} of length {1}", arr_3, arr_3.len());

    // let arr_4: [&str; 3] = ["Hello", 1, 3.14];
    let arr_4: [i32; 3] ;
    arr_4 = [1, 2, 3];
    println!("First elemnt of {:?} is {}", arr_4, arr_4[0]);

    let tuple_1 = (1, 2, 3);
    println!("First elemnt of {:?} is {}", tuple_1, tuple_1.0);
    let mut tuple_2: (i32, &str, f64) ;
    tuple_2 = (1, "Hello", 3.14);
    tuple_2.0 = 10;
    println!("Second elemnt of {:?} is {}", tuple_2, tuple_2.1);
}
