use std::{fs::{self, File}, io::Write, path::Path};

pub struct Storage {}

impl Storage {
    const CACHE: &str = "cache/";
    pub fn new() -> Self {
        Self {}
    }
    pub fn get(&self, name: &str) -> Option<String> {
        let mut path = Storage::CACHE.to_owned();
        path.push_str(name.to_ascii_lowercase().as_str());
        if !Path::new(&path).exists() {
            return None;
        }
        path.push_str("/res.txt");
        let contents = fs::read_to_string(path).expect("Should have been able to read the file");
        return Some(contents);
    }
    pub fn insert(&self, hash: &str, kanzii: &Vec<u8>) {
        let mut path = Storage::CACHE.to_owned();
        path.push_str(hash.to_ascii_lowercase().as_str());
        let _ = fs::create_dir_all(&path);

        let mut file = File::create(path + "/res.txt").unwrap();
        let _ = file.write_all(&kanzii);
    }

    pub fn insert_with_copy(&self, hash: &str, input_path: &str, kanzii: &Vec<u8>) {
        self.insert(hash, kanzii);

        let mut path = Storage::CACHE.to_owned();
        path.push_str(hash.to_ascii_lowercase().as_str());
        path.push_str("/image.png");
        let _ = fs::copy(input_path, path);
    }
}