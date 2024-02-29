use clap::Parser;
use std::fs;
use std::path::PathBuf;

/// Auto rename TV series files.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path of the tv
    #[arg(short, long)]
    path: PathBuf,

    /// The name of the tv
    #[arg(short, long, value_name = "TV_NAME")]
    name: Option<String>,

    /// The suffix of file
    #[arg(short = 'x', long)]
    suffix: Option<String>,

    /// The season of the tv
    #[arg(short, long, default_value_t = 1)]
    season: u8,

    /// The type of sort
    #[arg(short = 't', long, default_value_t = 1)]
    sort: u8,
}

fn main() {
    let args = Args::parse();
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

    for entry in vec_entries {
        let path = entry.path();

        // 打印文件或子目录的路径
        println!("{}", path.display());

        // 如果是目录，可以递归遍历
        if path.is_dir() {
            // 递归遍历子目录
            // traverse_directory(&path);
        }
    }

    if args.suffix == None {
        println!("No set suffix!");

    }

    if let Some(name) = args.name.as_deref() {
        println!("Value for name: {:?}", name);
    }
}
