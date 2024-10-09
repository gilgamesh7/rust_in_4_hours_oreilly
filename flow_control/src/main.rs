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
}

fn demo_match(){

}