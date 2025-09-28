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
    let toput = Contentstruct {
        id: items.len(),
        content: contents,
    };
    items.push(toput);
    println!("Created item with id: {}", items.len() - 1);
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
    let _removed = items.remove(secondnumver);
    println!("Deleted item with id: {}", secondnumver);
}
