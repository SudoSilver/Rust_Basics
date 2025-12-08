fn main() {
    let x: i64 = 10; 

    if x > 0 {//curly brackets are mandatory () are not and they actually make the compiler give a warning non fatal
        println!("{} is greater than 0", x); //executes if the condition is true
    }

    if x > 0 {
        println!("{} is greater than 0", x);
    }else{ //the else executes only if the if doesn't it can not have a condition
        println!("{} is not greater than 0", x);
    }

    if x < 0 {
        println!("{} is not greater than 0", x);
    }else if x > 0 { //this will only attempt to check if the if does not execute
        println!("{} is greater than 0", x);
    }else{ //in case non of the previous conditions trigger this does
        println!("x is equal to 0");
    }
}

//This is pretty much it for simple if statments 
//They are not really complex YET 
//go to the next file you will understand me