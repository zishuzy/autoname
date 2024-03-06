use clap::Parser;
use std::{collections::HashMap, fs, path};

/// Auto rename TV series files.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path of the tv file.
    #[arg(short, long)]
    path: path::PathBuf,

    /// Tv name
    #[arg(short, long, value_name = "TV_NAME")]
    name: String,

    /// The file extension to be renamed, if not set, the most used extension will be used.
    #[arg(short, long)]
    ext: Option<String>,

    /// The season of the tv
    #[arg(short, long, default_value_t = 1)]
    season: u8,

    /// The type of sort. 1: ascending, 2: descending
    #[arg(short = 't', long, default_value_t = 1)]
    sort: u8,

    /// Whether to rename files. 0: no, 1: yes
    #[arg(short, long, default_value_t = 0)]
    rename: u8,
}

fn read_dir(dst_path: &path::Path) -> Vec<fs::DirEntry> {
    match fs::read_dir(dst_path) {
        Ok(entries) => entries.filter_map(Result::ok).collect(),
        Err(_) => {
            eprintln!("Failed to read directory!");
            Vec::new()
        }
    }
}

fn detect_ext(entries: &[fs::DirEntry]) -> Option<String> {
    let mut ext_count: HashMap<String, usize> = HashMap::new();

    for entry in entries.iter().filter(|e| !e.path().is_dir()) {
        if let Some(ext) = entry.path().extension() {
            let count = ext_count
                .entry(ext.to_string_lossy().into_owned())
                .or_insert(0);
            *count += 1;
        }
    }

    ext_count
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(ext, _)| ext)
}

fn main() {
    let mut args = Args::parse();
    let dst_path = args.path.as_path();

    if !dst_path.exists() {
        eprintln!("The path does not exist, path[{}]", dst_path.display());
        return;
    }

    // 读取目录中的文件到 vec 中
    let mut vec_entries = read_dir(dst_path);

    if args.ext.is_none() {
        println!("No specified file extension, than detect the most used extension.");
        args.ext = detect_ext(&vec_entries);
        if let Some(ref ext) = args.ext {
            println!("Got the most used extension: {:?}", ext);
        }
    }

    // 移除不需要执行重命名的文件
    vec_entries.retain(|e| matches!(e.path().extension(), Some(ext) if ext == args.ext.as_deref().unwrap_or_default()));

    // 对待重命名的文件进行排序
    vec_entries.sort_by(|a, b| {
        if args.sort == 1 {
            a.path().cmp(&b.path())
        } else {
            b.path().cmp(&a.path())
        }
    });

    // 执行重命名逻辑
    for (count, entry) in vec_entries
        .iter()
        .enumerate()
        .filter(|(_, e)| !e.path().is_dir())
    {
        if let Some(ext) = entry.path().extension() {
            if ext.to_string_lossy() != args.ext.as_deref().unwrap_or_default() {
                continue;
            }

            let name = format!(
                "{}.S{:02}E{:02}.{}",
                args.name,
                args.season,
                count + 1,
                args.ext.as_deref().unwrap_or_default()
            );
            if args.rename == 0 {
                println!("{} => {}", entry.file_name().to_string_lossy(), name);
            } else {
                match fs::rename(&entry.path(), dst_path.join(&name)) {
                    Ok(_) => println!(
                        "Renamed: {} => {}",
                        entry.file_name().to_string_lossy(),
                        name
                    ),
                    Err(e) => eprintln!("Failed to rename: {}", e),
                }
            }
        } else {
            eprintln!("Failed to get extension: {}", entry.path().display());
        }
    }
}
