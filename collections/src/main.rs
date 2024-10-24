use std::collections::HashMap;

fn main() {
    // Vector
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("v1 is {:?}, of length {}, firts element is {}", v1, v1.len(), v1[0]); // println!("v1 is {:?", v1);

    let mut v2 = vec![1, 2, 3, 4, 5];
    v2.insert(0, 0);
    println!("v2 is {:?}, of length {}, firts element is {}", v2, v2.len(), v2[0]);

    vector_bounds_checking(3);

    let mut m1: HashMap<String, i32> = HashMap::new();
    m1.insert(String::from("UK"), 44);
    m1.insert(String::from("USA"), 1);
    m1.insert(String::from("Germany"), 49);
    println!("{:?}, length is {}", m1, m1.len());

    let value = m1.entry(String::from("UK")).or_insert(44);
    println!("Value for UK is {:?}", value);

    let _sa_value = m1.entry(String::from("South Africa")).or_insert(27);
    println!("{:?}", m1);

    let opt = m1.get(&String::from("UK"));
    match opt {
        Some(x) => println!("Value for UK is {:?}", x),
        None => println!("UK does not exist"),
    }
}

fn vector_bounds_checking(i: usize) {
    let v = vec![1, 2, 3, 4, 5];

    let opt = v.get(i);
    match opt {
        Some(x) => println!("v[{}] = {}", i, x),
        None => println!("v[{}] does not exist", i),
    }
    println!("v[{}] = {}", i, v[i]);
}


