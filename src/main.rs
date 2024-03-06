use clap::Parser;
use std::{collections::HashMap, fs, path::PathBuf};

/// Auto rename TV series files.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path of the tv file.
    #[arg(short, long)]
    path: PathBuf,

    /// Tv name
    #[arg(short, long, value_name = "TV_NAME")]
    name: String,

    /// The file extension to be renamed, if not set, the most used extension will be used.
    #[arg(short = 'x', long)]
    suffix: Option<String>,

    /// The season of the tv
    #[arg(short, long, default_value_t = 1)]
    season: u8,

    /// The type of sort. 1: ascending, 2: descending
    #[arg(short = 't', long, default_value_t = 1)]
    sort: u8,
}

fn main() {
    let mut args = Args::parse();
    let mut vec_entries: Vec<fs::DirEntry> = Vec::new();

    println!("args: {:#?}", args);

    let dst_path = args.path.as_path();
    if !dst_path.exists() {
        eprintln!("The path does not exist, {}", dst_path.display());
        return;
    }

    if let Ok(entries) = fs::read_dir(dst_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                vec_entries.push(entry);
            }
        }
    } else {
        println!("Failed to read directory.");
    }

    // 排序 vec_entries
    vec_entries.sort_by(|a, b| {
        if args.sort == 1 {
            a.path().cmp(&b.path())
        } else {
            b.path().cmp(&a.path())
        }
    });

    if args.suffix == None {
        let mut ext_count: HashMap<String, usize> = HashMap::new();
        let mut max_count = 0;
        let mut max_ext = String::new();
        println!("No set suffix, than detect the most used extension.");
        for entry in vec_entries.iter() {
            let path = entry.path();
            // println!("{}", path.display());
            if path.is_dir() {
                continue;
            }
            if let Some(ext) = path.extension() {
                let count = ext_count
                    .entry(ext.to_str().unwrap().to_string())
                    .or_insert(0);
                *count += 1;
                if *count > max_count {
                    max_count = *count;
                    max_ext = ext.to_str().unwrap().to_string();
                }
            }
        }
        println!("Got the most used extension: {:?}", max_ext);
        args.suffix = Some(max_ext);
    }

    println!("Value for name: {:?}", args.name);

    let mut count = 1;
    for entry in vec_entries {
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        if let Some(ext) = path.extension() {
            if ext.to_str().unwrap() != args.suffix.as_deref().unwrap() {
                continue;
            }
            let name = format!(
                "{}.S{:02}E{:02}.{}",
                args.name,
                args.season,
                count,
                args.suffix.as_deref().unwrap()
            );
            count += 1;
            match fs::rename(
                &path,
                PathBuf::from(format!("{}/{}", dst_path.display(), name)),
            ) {
                Ok(_) => println!(
                    "Renamed: {} => {}",
                    // path.file_name().unwrap().to_str().unwrap().display(),
                    path.file_name().unwrap().to_str().unwrap(),
                    name
                ),
                Err(e) => println!("Failed to rename: {}", e),
            }
        }
    }
}
