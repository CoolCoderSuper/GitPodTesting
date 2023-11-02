fn main() {
    println!("Notes App in Rust");
    let mut notes: Vec<Note> = Vec::new();
    loop {
        let mut command = String::new();
        match std::io::stdin().read_line(&mut command) {
            Ok(_s) => {
                command = command.trim().to_string();
                if command == "exit"{
                    break;
                }else if command.starts_with("add:"){
                    command.remove_adv(0, 4);
                    notes.push(Note {id: match notes.iter().max_by(|x, y| x.id.cmp(&y.id)) {
                        Some(n) => n.id + 1,
                        None => 1
                    }, value: command});
                }else if command.starts_with("remove:") {
                    command.remove_adv(0, 7);
                    let id: i32 = command.parse().unwrap();
                    let index = notes.iter().position(|x| x.id == id);
                    match index {
                        Some(n) => {notes.remove(n);},
                        None => println!("Could not find note with id: {}", id),
                    }
                }else if command == "list" {
                    for note in notes.iter() {
                        println!("{}: {}", note.id, note.value);
                    }
                }else{
                    println!("Invalid Command")
                }
            }
            Err(e) => println!("{e}")
        }
    }
}

struct Note{
    id: i32,
    value: String
}

impl RemoveAdv for String {
    fn remove_adv(&mut self, start: usize, length: usize) {
        for _i in start..length {
            self.remove(start);
        }
    }
}

trait RemoveAdv {
    fn remove_adv(&mut self, start: usize, length: usize);
}