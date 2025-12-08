fn main() {
    let x = 10; //this is an immutable variable
    let mut y = 10; //with the mut keyword it turns mutable

    //so because x is immutable i can't do x = x + y;
    y = y + x; //this is totally fine as y is mutable 

    println!("{}", y); //and we use {} inside the quotes as a placeholder for our value
                       //and then and the variable after a , to get print the value
    
    //so now we got some math stuff
    let a = 10 + 2; //addition 
    let b = 10 - 2; //subtraction
    let c = 10 / 2; //division
    let d = 10 * 2; //multiplication
    let e = 10 % 2; //the remainder of the division

    println!("{} {} {} {} {}",a,b,c,d,e); //so little quiz for ya what is this gonna print
}

//So to sum up
//Mutable variables have the key mut and can change
//Immutable variables dont have the key word mut and can't be changed
//{} are a placeholder for a value
//then ,variable to print the variable
//you can also daisy chain the variables 