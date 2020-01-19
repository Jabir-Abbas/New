use std::io;
fn main() {
    let mut height = String::new();    
    let mut weight = String::new();    

    println!("Enter Height in Cm:  ");
    io::stdin().read_line(&mut height).expect("Failed to read line");
    let height : u32 = height.trim().parse().expect("Please type a number!");

    println!("Enter Weight in Kg:  ");
    io::stdin().read_line(&mut weight).expect("Failed to read line");
    let weight : u32 = weight.trim().parse().expect("Please type a number!");

    let mut PMB =weight/((height/100)^2);
    println!("pmb is: {}",PMB);
    println!()

}