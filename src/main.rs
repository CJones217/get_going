//future additions, pick based on level of urgency (tuple with string and level of urgency)
//add feature to add or remove tasks
//use with database?
//gui
//app

use rand::Rng;

fn main() {
    let  defcon0_items = ["resume","computer forensics", "internship", "research"];
    let  defcon1_items = ["rubicks cube","coding problems", "reverse engineer logicola", "take a break!"];
    let  defcon2_items = ["arduino","pokedex", "read","wireshark"];

    let  list_of_shit = [defcon0_items,defcon1_items,defcon2_items];
    let  random_number = rand::thread_rng().gen_range(0, 4);

    println!("get off your ass");
    for n in 0.. list_of_shit.len() {
        println!("test {:?}: {:?}",n, list_of_shit[n][random_number]);
    }

}
