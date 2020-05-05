//future additions, pick based on level of urgency (tuple with string and level of urgency)



use rand::Rng;

fn main() {
    let mut defcon0_items = ["resume","computer forensics", "internship", "research"];
    let mut defcon1_items = ["rubicks cube","coding problems", "reverse engineer logicola"];
    let mut defcon2_items = ["arduino","pokedex", "read","wireshark"];

    let mut list_of_shit = [defcon0_items,defcon1_items,defcon2_items];
    let mut random_number = rand::thread_rng().gen_range(1, list_of_shit.len());


    println!("get off your ass");


}
