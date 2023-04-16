use std::fs;
use std::io::stdin;
use std::path::PathBuf;
use std::process;
use whoami;

fn main() {
    println!("Saver and Loader for Wild Frost");
    println!("Press [1] to save archive");
    println!("Press [2] to load archive");
    println!("Press [q] to exit");

    let username = whoami::username();
    let old_path = PathBuf::from(format!(
        "C:/Users/{}/AppData/LocalLow/Deadpan Games/Wildfrost/Profiles/Default",
        username
    ));

    let new_path = PathBuf::from("Default");

    let mut command = String::new();
    loop {
        stdin().read_line(&mut command).unwrap_or_else(|e| {
            println!("Failed to read command: {}", e);
            0
        });
        command = command.trim().trim_end().into();

        match command.as_str() {
            "1" => match copy_archive(&old_path, &new_path) {
                Some(n) => println!("Save {} files successfully", n),
                None => print!("Save failed"),
            },
            "2" => match copy_archive(&new_path, &old_path) {
                Some(n) => println!("Load {} files successfully", n),
                None => println!("Load failed"),
            },
            "q" => process::exit(1),
            _ => println!("Unvaild command: {}", command),
        };
        command.clear();
    }
}

fn copy_archive(old_path: &PathBuf, new_path: &PathBuf) -> Option<usize> {
    if !old_path.exists() {
        fs::create_dir(old_path).expect("Failed to create archive directory");
        return None;
    }

    let mut count = 0;
    if !new_path.exists() {
        fs::create_dir(new_path).expect("Failed to create directory");
    }

    for file in fs::read_dir(old_path).expect("Failed to read archive directory") {
        let new_file = new_path.join(file.as_ref().expect("Failed to get file name").file_name());
        if let Err(_) = fs::copy(file.unwrap().path(), new_file) {
            println!("Failed to copy file");
            return None;
        }
        count += 1;
    }

    Some(count)
}
