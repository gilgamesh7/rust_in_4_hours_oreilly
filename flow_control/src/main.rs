fn main() {
    demo_if();
    demo_match();
}

fn demo_if(){
    let age : i8 = 59;

    if age > 50{
        println!("Yo, at {0} , are old !", age);
    }

    if age < 50{
        println!("you are young");
    }
    else{
        println!("This is an else");
    }

    let message = if age > 50 {"You are old"} else {"you are young"};
    println!("{}",message);

    // let height = 200;
    let height = 180;
    if height < 160 || height > 190{
        println!("you are not tall enough");
    }
    if height > 160 && height < 190{
        println!("you are tall enough");
    }
}

fn demo_match(){
    // let num = 100;
    // let num = 200;

    let num = 300;

    match num {
        100 => {
            println!("Hundred");
            println!("Cant");
            println!("Hundre");
        },
        200 => println!("Two hundred"),
        _ => println!("Not a hundred"),
    }
}