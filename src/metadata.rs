use std::path::PathBuf;

pub struct Metadata<'a> {
    name: &'a str,
    entry_type: &'a str,
}

impl<'a> Metadata<'a> {
    pub fn new(file_path: &'a PathBuf) -> Self {
        let result = Metadata {
            name: file_path.file_name().unwrap().to_str().unwrap(),
            entry_type: if file_path.is_file() {
                file_path.extension().unwrap_or_default().to_str().unwrap()
            } else {
                "directory"
            },
        };

        result
    }

    pub fn display(&self) -> String {
        format!("Name: {}\nType: {}", self.name, self.entry_type)
    }
}
