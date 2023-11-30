fn main(){
    println!("Number: {}{}",2,1); // Single Placeholder
    println!("{} is a {}","saif","boy"); // Multiple Placeholders
    println!("{1} and {0}","saif",10 ); // Positional Arguments
    println!("{Name} is a {Occupation}.", Name = "Saif", Occupation = "student");// Named Arguments
    println!("{:b} {:x} {:o}",10,10,10); // Placeholder Traits
    println!("{} + {} = {}",10,10,10+10);// Basic Maths
}