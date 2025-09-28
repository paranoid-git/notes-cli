use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;
#[derive(Serialize, Clone, Eq, PartialEq, Deserialize)]
struct Contentstruct {
    id: usize,
    content: String,
}
fn import_notes(filename: &str) -> Result<Vec<Contentstruct>, std::io::Error> {
    let file = File::open(filename)?;
    let notes: Vec<Contentstruct> = serde_json::from_reader(file)?;
    println!("Notes imported from {}", filename);
    Ok(notes)
}

fn export_notes(items: &Vec<Contentstruct>, filename: &str) -> Result<(), std::io::Error> {
    let json = serde_json::to_string(items)?;
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;
    println!("Notes exported to {}", filename);
    Ok(())
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
            "import" => {
                if let Ok(notes) = import_notes(rest[1]) {
                    items = notes;
                }
            }
            "export" => {
                if let Err(err) = export_notes(&items, rest[1]) {
                    println!("Error exporting notes: {}", err);
                }
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_import_notes() {
        let filename = "asd.json";
        let contents = r#"[{"id":1,"content":"Hello, World!"}]"#;
        let mut file = File::create(filename).unwrap();
        file.write_all(contents.as_bytes()).unwrap();

        let notes = import_notes(filename).unwrap();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].id, 1);
        assert_eq!(notes[0].content, "Hello, World!");

        // Clean up the test file
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_export_notes() {
        let items = vec![
            Contentstruct {
                id: 1,
                content: "Hello, World!".to_string(),
            },
            Contentstruct {
                id: 2,
                content: "Goodbye, World!".to_string(),
            },
        ];

        let filename = "asd.json";
        export_notes(&items, filename).unwrap();

        let contents = std::fs::read_to_string(filename).unwrap();
        assert_eq!(
            contents,
            r#"[{"id":1,"content":"Hello, World!"},{"id":2,"content":"Goodbye, World!"}]"#
        );

        // Clean up the test file
        std::fs::remove_file(filename).unwrap();
    }
    #[test]
    fn test_create_with_empty_items_vector() {
        let mut items: Vec<Contentstruct> = Vec::new();
        create(&["hello"], &mut items);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, 1);
        assert_eq!(items[0].content, "hello");
    }

    #[test]
    fn test_create_with_non_empty_items_vector() {
        let mut items: Vec<Contentstruct> = vec![Contentstruct {
            id: 1,
            content: "world".to_string(),
        }];
        create(&["hello"], &mut items);
        assert_eq!(items.len(), 2);
        assert_eq!(items[1].id, 2);
        assert_eq!(items[1].content, "hello");
    }

    #[test]
    fn test_delete() {
        let mut items: Vec<Contentstruct> = vec![
            Contentstruct {
                id: 1,
                content: "world".to_string(),
            },
            Contentstruct {
                id: 2,
                content: "hello".to_string(),
            },
        ];
        delete("2".to_string(), &mut items);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, 1);
        assert_eq!(items[0].content, "world");
    }
}
