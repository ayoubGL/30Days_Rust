#[derive(Debug)]
struct GuardianEntry {
    id: u32,
    title: String,
}

fn main() {
    let mut storage: Vec<GuardianEntry> = Vec::new();

    // Create an entry
    let entry = GuardianEntry {
        id: 1,
        title: String::from("Day 5 Task"),
    };
    storage.push(entry);

    // 1. Immutable Borrow: We pass a reference (&) so the function can "read"
    display_entry(&storage[0]);

    // 2. Mutable Borrow: We pass a mutable reference (&mut) to "change" it
    update_title(&mut storage[0], String::from("Mastering Borrowing"));

    match find_by_id(&storage, 1) {
        Some(found_entry) => {
            println!("Found: {:?}", found_entry);
        }
        None => {
            println!("Entry not found.");
        }
    }
}

// Function that BORROWS to read
fn display_entry(entry: &GuardianEntry) {
    println!("Viewing Entry: {}", entry.title);
} // The borrow ends here, but 'storage' still owns the data.

// Function that BORROWS to change
fn update_title(entry: &mut GuardianEntry, new_title: String) {
    entry.title = new_title;
}

fn find_by_id(entries: &[GuardianEntry], id: u32) -> Option<&GuardianEntry> {
    for entry in entries {
        if entry.id == id {
            return Some(entry);
        }
    }

    None
}

fn deleted_by_id(entries: &mut Vec<GuardianEntry>, id: u32) {
    // position() returns the index of the element to be removed
    if let Some(index) = entries.iter().position(|e| e.id == id) {
        entries.remove(index);
        println!("Entry with ID {} removed.", id);
    } else {
        println!("Entry with ID {} not found.", id);
    }
}
