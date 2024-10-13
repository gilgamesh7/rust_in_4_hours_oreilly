// #![allow(dead_code)]  -- Use to suppress warmnings about dead code

fn main() {
    enum Colour {
        Red,
        Green,
        Blue,
    }

    let my_colour = Colour::Red;
    // println!("{}", my_colour);

    match my_colour {
        Colour::Red => {
            println!("coch");
        }
        Colour::Green => {
            println!("gwyrdd");
        }
        Colour::Blue => {
            println!("glas");
        }
    }

    enum HouseLocation {
        Number(i32),
        Name(String),
        Unknown,
    }

    let house_location: HouseLocation = HouseLocation::Number(123);

    match house_location {
        HouseLocation::Number(num) => println!("Number is {0}", num),
        HouseLocation::Name(name) => println!("Name is {0}", name),
        HouseLocation::Unknown => println!("Unknown"),
    }
    println!(
        "The size of the house is : {}",
        std::mem::size_of::<HouseLocation>()
    );

    let house_location: HouseLocation = HouseLocation::Name(String::from("Cahir"));
    match house_location {
        HouseLocation::Number(num) => println!("Number is {0}", num),
        HouseLocation::Name(name) => println!("Name is {0}", name),
        HouseLocation::Unknown => println!("Unknown"),
    }
    println!(
        "The size of the house is : {}",
        std::mem::size_of::<HouseLocation>()
    );

    let house_location: HouseLocation = HouseLocation::Unknown;
    match house_location {
        HouseLocation::Number(num) => println!("Number is {0}", num),
        HouseLocation::Name(name) => println!("Name is {0}", name),
        HouseLocation::Unknown => println!("Unknown"),
    }

    let mut fav_num: Option<i32>;
    fav_num = Some(123);
    match fav_num {
        Some(num) => println!("Favourite number is {0}", num),
        None => println!("No favourite number"),
    }

    fav_num = None;
    match fav_num {
        Some(num) => println!("Favourite number is {0}", num),
        None => println!("No favourite number"),
    }
    match fav_num {
        Some(num) => println!("Favourite number is {0}", num),
        None => println!(
            "Use this as your favourite number : {0}",
            fav_num.unwrap_or(42)
        ),
    }

    let _ = demo_result_enum(String::from("hello"));
    let _ = demo_result_enum(String::from("123"));

    let good_str = String::from("hello");
    println!("Value of good str is {}", good_str.parse::<i32>().unwrap_or(-1));

    let new_str = String::from("12345");
    println!("Value of new str is {}", new_str.parse::<i32>().unwrap_or(-1));

    let do_not_panic_str: String = String::from("1234");
    println!("Value of do_not_panic_str is {}", do_not_panic_str.parse::<i32>().expect("Not a number"));
    let panic_str = String::from("hello");
    println!("Value of new str is {}", panic_str.parse::<i32>().expect("Not a number"));
}



fn demo_result_enum(s: String) {
    match s.parse::<i32>() {
        Ok(num) => {
            println!("Value of parsed string is {0}", num);
        }
        Err(err) => {
            println!("Error is {0}", err);
        }
    }

}
