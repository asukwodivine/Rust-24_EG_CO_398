use std::fs::{self, File};
use std::io::{self, BufRead, Write};

fn write_log(path: &str, entries: &[&str]) -> io::Result<()> {
    let mut file = File::create(path)?;
    for entry in entries {
        writeln!(file, "{}", entry)?;
    }
    Ok(())
}

fn count_lines(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().count())
}

pub fn run() -> io::Result<()> {
    let path = "output.log";
    let entries = vec![
        "INFO Server started",
        "WARN High memory usage",
        "INFO Backup complete",
        "ERROR Disk full",
    ];

    write_log(path, &entries)?;

    let n = count_lines(path)?;
    println!("Wrote {} lines to {}", n, path);

    let content = fs::read_to_string(path)?;
    let errors: Vec<&str> = content
        .lines()
        .filter(|line| line.starts_with("ERROR"))
        .collect();
    println!("Error lines: {:?}", errors);

    fs::remove_file(path)?;

    Ok(())
}
