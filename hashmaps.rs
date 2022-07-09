// Why use Hashtabes?
// They are very fast for insert,delete, search an average O(1) time or worst case O(n) if there are many collision. To optimized hash tables we use linked lists if there are collisions and caching to improve performance. A load factor of less than 0.7 is desirable. (number of times in table/total number of slots.)

use std::collections::HashMap;
 fn main(){
    let mut animals = HashMap::new();

    animals.insert(String::from("Dog"),1);
    animals.insert(String::from("Cat"),2);
    animals.insert(String::from("Horse"),3);

    for (name,location) in &animals{
        println!("Animal: {} -> Mapped {}", name, location);
    }
 }