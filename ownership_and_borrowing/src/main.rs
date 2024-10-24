use std::mem;

// Ownership and borrowing
fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world of warcraft");
    let s3 = String::from("Stabat mater dolorósa uxta Crucem lacrimósa, dum pendébat Fílius.");

    println!("Size of {0} is {1}", s1, mem::size_of_val(&s1));
    println!("Size of {0} is {1}", s2, mem::size_of_val(&s2));
    println!("Size of {0} is {1}", s3, mem::size_of_val(&s3));

    let mut s4 = String::from("Bono malum");
    println!("s4 is {}", s4);
    s4.push_str("  superat");
    println!("s4 is {}", s4);

    let x: isize = 10;
    let y: isize = x;
    println!("x is {}, y is {}", x, y);

    let s5: String = s4;
    println!("s5 is {}", s5);
    // println!("s4 is {}", s4); WONT COMPILE

    let s6: String = String::from("nimis comedis");
    let s7: &String = &s6;
    let s8: &String = &s6;

    println!("s6 is {}", s6);
    println!("s7 is {}", s7);
    println!("s8 is {}", s8);

    let mut s9: String = String::from("Amantis");
    let r9: &mut String = &mut s9;
    r9.push_str(" Amentis");


    println!("r9 is {}", r9);
    println!("s9 is {}", s9);
 
}
