fn main() {
    let i: i64 = 0; //it is typical to use i as a counter for loops 
    
    while i <= 20 { //just like ifs we do not need () around the condition
        println!("{}",i); //the code inside the {} is going to be executed as many times as the condition allows
        i+=1; //adjust the counter so we dont have an infinite loop
    }
}