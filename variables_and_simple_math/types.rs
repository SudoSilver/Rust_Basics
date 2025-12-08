//Oh this is a long one
//so just like every compiled language
//you can and SHOULD
//declare variable types
fn main() {
    let x: i64; //64 bit intreger can be positive and negative 
    let y: u64; //64 bit intreger can only be positive but can be way double the size

    let f: f64; //64 bit float

    let c: char; //a singular unicode character
    let s: String; //a string which is dynamic sized kinda i will explain at the bottom
    let s1: &str = "hello"; //a string which is of static size

    let ar: [i64; 2] = [0;2]; //a static sized array which goes [type;amount] = [starting value; amount]
    let vec: Vec<i64> = vec![2,3,4,5]; //something like an array but with dynamic size goes Vec<type>
    
    let t_or_f: bool; //A standard boolient can be true or false 
}

//because this is just a tutorial i used uninitialized variables
//typically you cant do this unless you assign them later 
//important to use the vec! macro if creating a vector with some values

//STRINGS
//A normal string in rust is dynamically sized so it can grow and shrink
//it lives in the heap which doesn't matter in 90% of cases
//it is an owned value you will learn more about that in the borrow section
