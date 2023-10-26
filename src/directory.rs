use std::fs;
use std::io;
use std::path;

pub fn size_to_hr(size: u64) -> String {
    if size < 1024 {
        return format!("{:.2} b", size);
    } else if size < 1024 * 1024 {
        return format!("{:.2} Kb", size as f64 / 1024.0);
    } else if size < 1024 * 1024 * 1024 {
        return format!("{:.2} Mb", size as f64 / 1024.0 / 1024.0);
    } else if size < 1024 * 1024 * 1024 * 1024 {
        return format!("{:.2} Gb", size as f64 / 1024.0 / 1024.0 / 1024.0);
    } else {
        return format!("{:.2} Tb", size as f64 / 1024.0 / 1024.0 / 1024.0 / 1024.0);
    }
}

pub fn get_directory_size(path: &str) -> u64 {
    let path: &path::Path = path::Path::new(path);

    if path.is_dir() {
        let mut total_size: u64 = 0;

        for entry in fs::read_dir(path).unwrap() {
            let entry: fs::DirEntry = entry.unwrap();
            let file_type: fs::FileType = entry.file_type().unwrap();

            if file_type.is_dir() {
                total_size += get_directory_size(entry.path().to_str().unwrap());
            } else if file_type.is_file() {
                total_size += entry.metadata().unwrap().len();
            }
        }

        total_size
    } else {
        0
    }
}

pub fn get_folders(path: &str) -> Vec<String> {
    let mut folders: Vec<String> = Vec::<String>::new();
    fs::read_dir(path)
        .unwrap()
        .into_iter()
        .for_each(|file: Result<fs::DirEntry, io::Error>| {
            let path: path::PathBuf = file.unwrap().path();
            let full_path: String = path.display().to_string();
            let metadata: fs::Metadata = fs::metadata(&full_path).unwrap();

            if metadata.is_dir() {
                folders.push(full_path);
            }
        });

    folders
}

pub fn get_files(path: &str) -> Vec<String> {
    let mut folders: Vec<String> = Vec::<String>::new();
    fs::read_dir(path)
        .unwrap()
        .into_iter()
        .for_each(|file: Result<fs::DirEntry, io::Error>| {
            let path: path::PathBuf = file.unwrap().path();
            let full_path: String = path.display().to_string();
            let metadata: fs::Metadata = fs::metadata(&full_path).unwrap();

            if metadata.is_file() {
                folders.push(full_path);
            }
        });

    folders
}

pub fn get_all_files(path: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::<String>::new();
    fs::read_dir(path)
        .unwrap()
        .into_iter()
        .for_each(|file: Result<fs::DirEntry, io::Error>| {
            let path: path::PathBuf = file.unwrap().path();
            if path.exists() {
                let full_path: String = path.display().to_string();
                let metadata: fs::Metadata = fs::metadata(&full_path).unwrap();

                let link: Result<path::PathBuf, io::Error> = fs::read_link(&full_path);
                if !link.is_ok() && metadata.is_dir() {
                    let mut sub_files: Vec<String> = get_all_files(&full_path);
                    files.append(&mut sub_files);
                }
                if metadata.is_file() {
                    files.push(full_path);
                }
            }
        });

    files
}
