//This is a special loop
//it is the idiomatic loop in rust
//an equivelant to do while in c++ and other languages kind of
fn main() {
    let i: i64 = 0;

    //just like do while in other programming languages it executes always a minimum of one time unlike other loops who can execute 0 times
    loop { //this loop is by default infinite
        if i == 10 { 
            break; //to stop it we add the break; keword inside typically in a condition
        }
        i+=1;//again increasing the counter
        println!("{}",i); //as with all loops the code in the {} executes every time until the condition breaks it
    }

    loop {} //loop is also used when we need an infinite loop for whatever reason 
}