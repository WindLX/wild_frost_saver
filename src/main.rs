use std::fs;
use std::io::{self, ErrorKind, Write};
use std::path::PathBuf;
use std::process;
use whoami;

struct Command {
    old_path: PathBuf,
    new_path: PathBuf,
}

impl Command {
    fn from_paths(old_path: &PathBuf, new_path: &PathBuf) -> Command {
        Command {
            old_path: old_path.clone(),
            new_path: new_path.clone(),
        }
    }

    fn execute(&self, input: &mut io::Stdin, output: &mut dyn Write) -> io::Result<()> {
        writeln!(output, "\nPlease select a command:")?;
        writeln!(output, "1. Save files from old to new")?;
        writeln!(output, "2. Load files from new to old")?;
        writeln!(output, "q. Quit")?;
        loop {
            let mut command = String::new();
            input.read_line(&mut command)?;

            match command.trim() {
                "1" => match self.save_files() {
                    Ok(n) => writeln!(output, "Save {} files successfully", n)?,
                    Err(e) => writeln!(output, "Save failed: {}", e)?,
                },
                "2" => match self.load_files() {
                    Ok(n) => writeln!(output, "Load {} files successfully", n)?,
                    Err(e) => writeln!(output, "Load failed: {}", e)?,
                },
                "q" => process::exit(0),
                _ => writeln!(output, "Invalid command: {}", command.trim())?,
            };
        }
    }

    fn save_files(&self) -> io::Result<usize> {
        copy_archive(&self.old_path, &self.new_path)
    }

    fn load_files(&self) -> io::Result<usize> {
        copy_archive(&self.new_path, &self.old_path)
    }
}

fn copy_archive(old_path: &PathBuf, new_path: &PathBuf) -> io::Result<usize> {
    if !old_path.exists() {
        fs::create_dir(old_path)?;
        return Err(io::Error::new(
            ErrorKind::NotFound,
            "Old path does not exist",
        ));
    }

    let mut count = 0;
    if !new_path.exists() {
        fs::create_dir(new_path)?;
    }

    for file in fs::read_dir(old_path)? {
        let file = file?;
        let new_file = new_path.join(file.file_name());
        fs::copy(file.path(), new_file)?;
        count += 1;
    }

    Ok(count)
}

fn main() -> io::Result<()> {
    let username = whoami::username();
    let old_path = PathBuf::from(format!(
        "C:/Users/{}/AppData/LocalLow/Deadpan Games/Wildfrost/Profiles/Default",
        username
    ));
    let new_path = PathBuf::from("Default");

    let command = Command::from_paths(&old_path, &new_path);
    command.execute(&mut io::stdin(), &mut io::stdout())
}
