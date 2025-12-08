fn main() {
    let x: bool = true;
    let y: bool = false;
    let z: bool = true;

    if x == true || y == true { //this executes if either x or y is true we achieve that through the or operator ||
        println!("Something");
    }

    if x == true && y == true {//with the and operator && both conditions have to be true 
        println!("Something");
    }

    if (x == true && y == false) || z == true {//the stuff in the () get checked first instead of going with the order they are written
        println!("Something");
    } 

    if y != true { //the ! is the not operator for when you want a condition to not be true
        println!("Something");
    }
}

//So the rundown version is 
//use || if either of the conditions being true is fine
//use && if you need both conditions to be true
//use ! if you want your condition to not execute when it returns true and execute only if it returns false
//group checks with () so they execute first
//boolient algebra really helps here go reaserch it if you want 
//even if you dont this is fine 