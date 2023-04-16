use std::fs;
use std::io::stdin;
use std::path::Path;
use std::process;

fn main() {
    println!("Saver and Loader for Wild Frost");
    println!("Press [1] to save archive");
    println!("Press [2] to load archive");
    println!("Press [q] to exit");

    let old_path =
        Path::new("C:/Users/wind/AppData/LocalLow/Deadpan Games/Wildfrost/Profiles/Default");
    let new_path = Path::new("Default");

    let mut command = String::new();
    loop {
        stdin().read_line(&mut command).unwrap_or_else(|e| {
            println!("Failed to read command: {}", e);
            0
        });
        command = command.trim().trim_end().into();
        match command.as_str() {
            "1" => match copy_archive(old_path, new_path) {
                Some(n) => println!("Save {} files successfully", n),
                None => print!("Save failed"),
            },
            "2" => match copy_archive(new_path, old_path) {
                Some(n) => println!("Load {} files successfully", n),
                None => println!("Load failed"),
            },
            "q" => process::exit(1),
            _ => println!("Unvaild command: {}", command),
        };
        command.clear();
    }
}

fn copy_archive(old_path: &Path, new_path: &Path) -> Option<i32> {
    match old_path.exists() {
        true => {
            let mut count = 0;
            if !new_path.exists() {
                fs::create_dir(new_path)
                    .unwrap_or_else(|e| println!("Create directory failed: {}", e));
            }
            for file in fs::read_dir(old_path).unwrap() {
                let new_file =
                    new_path.join(file.as_ref().expect("Create archive failed").file_name());
                let res = fs::copy(file.as_ref().unwrap().path(), new_file);
                match res {
                    Ok(_) => {
                        count += 1;
                    }
                    Err(_) => {
                        print!("Copy file failed");
                        return None;
                    }
                }
            }
            Some(count)
        }
        false => {
            fs::create_dir(old_path).expect("Create archive failed");
            None
        }
    }
}
