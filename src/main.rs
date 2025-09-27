use std::{env, io::stdin};

struct Contentstruct {
    id: usize,
    content: String,
}

fn main() {
    let mut items: Vec<Contentstruct> = Vec::new();

    loop {
        let mut command = String::new();
        stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        let second = command.trim().split_whitespace();
        let rest: Vec<&str> = second.collect();
        println!("rest is: {}", rest[0]);

        match rest[0] {
            "create" => create(String::from(rest[1]), &mut items),
            "list" => list(&items),
            "delete" => delete(String::from(rest[1]), &mut items),
            _ => println!("Invalid command {}", rest[0]),
        }
    }
}

fn create(contents: String, items: &mut Vec<Contentstruct>) {
    println!("Creating a new item...");
    let toput = Contentstruct {
        id: items.len(),
        content: contents,
    };
    items.push(toput)
    // Add code here to handle the creation of a new item
    // You can prompt the user for input and store it in a data structure
    // You can also create a file or store the item in a database
}

fn list(items: &Vec<Contentstruct>) {
    println!("Listing all items...");
    for n in items {
        println!("id: {}, contents: {}", n.id, n.content)
        // Add code here to retrieve and display all items
        // You can read from a file or query a database
    }
}

fn delete(second: String, items: &mut Vec<Contentstruct>) {
    println!("Deleting an item...");
    let secondnumver: usize = second.parse::<usize>().unwrap();
    let _removed = items.remove(secondnumver);
    // Add code here to handle the deletion of an item
    // You can prompt the user for input and remove the corresponding item from a data structure
    // You can also delete a file or remove the item from a database
}
