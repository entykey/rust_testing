use std::collections::HashMap;
use std::collections::HashSet;


// check if a hashmap has duplication
fn has_duplicate_values<K, V, S>(map: &HashMap<K, V, S>) -> bool
where
    V: Eq + std::hash::Hash,
{
    let values: Vec<_> = map.values().collect();
    let len_before = values.len();
    let len_after = values.into_iter().collect::<HashSet<_>>().len();
    len_before != len_after
}

fn insert_with_duplicates(map: &mut HashMap<String, i32>, incoming_string: String) {
    let mut count = 0;
    let mut modified_string = incoming_string.clone();

    while map.contains_key(&modified_string) {
        count += 1;
        modified_string = format!("{}{}", incoming_string, count);
    }

    map.insert(modified_string, count);
}

fn main() {
    // Create a new empty HashMap to store the grocery items and their quantities
    let mut grocery_list: HashMap<String, i32> = HashMap::new();

    // Add items to the grocery list along with their quantities
    grocery_list.insert(String::from("Apples"), 5);
    grocery_list.insert(String::from("Bananas"), 3);
    grocery_list.insert(String::from("Milk"), 2);
    grocery_list.insert(String::from("Bread"), 1);

    // Check if an item is in the grocery list and get its quantity
    let item = String::from("Apples");
    if let Some(quantity) = grocery_list.get(&item) {
        println!("We need {} {}.", quantity, item);
    } else {
        println!("{} is not on the grocery list.", item);
    }

    // Create a new empty HashSet to store the grocery items
    let mut grocery_set: HashSet<String> = HashSet::new();

    // Add items to the grocery set
    grocery_set.insert(String::from("Apples"));
    grocery_set.insert(String::from("Bananas"));
    grocery_set.insert(String::from("Milk"));
    grocery_set.insert(String::from("Bread"));

    // Check if an item is in the grocery set
    let item = String::from("Apples");
    if grocery_set.contains(&item) {
        println!("{} is on the grocery list.", item);
    } else {
        println!("{} is not on the grocery list.", item);
    }



    // duplication check:
    let mut map1: HashMap<i32, i32> = HashMap::new();
    map1.insert(1, 10);
    map1.insert(2, 20);
    map1.insert(3, 30);
    map1.insert(4, 20); // Duplicate value

    let mut map2: HashMap<i32, i32> = HashMap::new();
    map2.insert(1, 10);
    map2.insert(2, 20);
    map2.insert(3, 30);

    println!("map1 has duplicate values: {}", has_duplicate_values(&map1)); // Output: map1 has duplicate values: true
    println!("map2 has duplicate values: {}", has_duplicate_values(&map2)); // Output: map2 has duplicate values: false

    

    println!();
    // insert with duplicate:
    let mut my_map: HashMap<String, i32> = HashMap::new();

    insert_with_duplicates(&mut my_map, "apple".to_string());
    insert_with_duplicates(&mut my_map, "banana".to_string());
    insert_with_duplicates(&mut my_map, "apple".to_string());
    insert_with_duplicates(&mut my_map, "banana".to_string());
    insert_with_duplicates(&mut my_map, "apple".to_string());
    insert_with_duplicates(&mut my_map, "pinapple".to_string());
    insert_with_duplicates(&mut my_map, "banana".to_string());

    for (key, value) in my_map.iter() {
        println!("{}: {}", key, value);
    }
    println!();
    for (key, _) in my_map.iter() {
        println!("{}", key);
    }
    println!();


    // hashmap just insert with no order, this approach will print out the sorted map by key
    let mut keys: Vec<&String> = my_map.keys().collect();
    keys.sort();

    for key in keys {
        println!("{}", key);
    }
    
}