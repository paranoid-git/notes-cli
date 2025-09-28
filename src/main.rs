use std::io::stdin;

struct Contentstruct {
    id: usize,
    content: String,
}

fn main() {
    let mut items: Vec<Contentstruct> = Vec::new();

    loop {
        println!("Please type a command: ");
        let mut command = String::new();
        stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        let second = command.trim().split_whitespace();
        let rest: Vec<&str> = second.collect();
        //println!("rest is: {:?}", rest);
        match rest[0] {
            "create" => create(&rest[1..], &mut items),
            "list" => list(&items),
            "delete" => delete(String::from(rest[1]), &mut items),
            _ => println!("Invalid command {}", rest[0]),
        }
    }
}

fn create(rest: &[&str], items: &mut Vec<Contentstruct>) {
    println!("Creating a new item...");
    let contents = rest.join(" ");

    let mut next_id = 1;
    if !items.is_empty() {
        next_id = items.iter().map(|item| item.id).max().unwrap() + 1;
    }

    let toput = Contentstruct {
        id: next_id,
        content: contents,
    };
    items.push(toput);
    println!("Created item with id: {}", next_id);
}

fn list(items: &Vec<Contentstruct>) {
    println!("Listing all items...");
    for n in items {
        println!("Id: {}, Contents: {}", n.id, n.content)
    }
}

fn delete(second: String, items: &mut Vec<Contentstruct>) {
    println!("Deleting an item...");
    let secondnumver: usize = second.parse::<usize>().unwrap();
    if secondnumver <= items.len() {
        items.remove(secondnumver - 1);
        println!("Deleted item with id: {}", secondnumver);
    } else {
        println!("Invalid id: {}", secondnumver);
    }
}
