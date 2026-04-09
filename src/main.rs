#[derive(Debug)]
enum EntryType {
    Task,
    Secret,
}

#[derive(Debug)]
struct GuardianEntry {
    id: u32,
    title: String,
    content: String,
    entry_type: EntryType,
    priority: u8,
}

fn main() {
    // Create the first task
    let my_task = GuardianEntry {
        id: 1,
        title: String::from("Learn Rust Structs"),
        content: String::from("Understand how to groupd data together."),
        entry_type: EntryType::Task,
        priority: 1,
    };

    // create a Secret

    let my_secret = GuardianEntry {
        id: 2,
        title: String::from("My Password"),
        content: String::from("123456"),
        entry_type: EntryType::Secret,
        priority: 5,
    };

    println!(
        "My task ID: {:?}, My Task Title: {:?}, My Task Content: {:?}, My Task Entry Type {:?}, My Task Priority {:?}",
        my_task.id, my_task.title, my_task.content, my_task.entry_type, my_task.priority
    );
    println!(
        "My secret ID: {:?}, My Secret Title: {:?}, My Secret Content: {:?}, My Secret Entry Type {:?}, My Secret Priority {:?}",
        my_secret.id, my_secret.title, my_secret.content, my_secret.entry_type, my_secret.priority
    );
}
