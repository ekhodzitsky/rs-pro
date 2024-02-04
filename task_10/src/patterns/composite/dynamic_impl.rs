pub trait FileSystemComponent {
    fn get_size(&self) -> u64;
    fn get_name(&self) -> &str;
}

pub struct File {
    pub name: String,
    pub size: u64,
}

impl FileSystemComponent for File {
    fn get_size(&self) -> u64 {
        self.size
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

pub struct Directory {
    pub name: String,
    components: Vec<Box<dyn FileSystemComponent>>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            components: Vec::new(),
        }
    }

    pub fn add_component(&mut self, component: Box<dyn FileSystemComponent>) {
        self.components.push(component);
    }
}

impl FileSystemComponent for Directory {
    fn get_size(&self) -> u64 {
        self.components
            .iter()
            .map(|component| component.get_size())
            .sum()
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_name_and_size_are_correct() {
        let file = File {
            name: "test_file.txt".to_string(),
            size: 150,
        };
        assert_eq!(file.get_name(), "test_file.txt");
        assert_eq!(file.get_size(), 150);
    }

    #[test]
    fn directory_name_and_size_with_single_file() {
        let file = Box::new(File {
            name: "test_file.txt".to_string(),
            size: 150,
        });
        let mut directory = Directory::new("test_directory");
        directory.add_component(file);

        assert_eq!(directory.get_name(), "test_directory");
        assert_eq!(directory.get_size(), 150);
    }

    #[test]
    fn directory_name_and_size_with_multiple_components() {
        let file1 = Box::new(File {
            name: "file1.txt".to_string(),
            size: 100,
        });
        let file2 = Box::new(File {
            name: "file2.txt".to_string(),
            size: 200,
        });

        let mut subdirectory = Directory::new("subdirectory");
        subdirectory.add_component(file1);

        let mut directory = Directory::new("directory");
        directory.add_component(Box::new(subdirectory));
        directory.add_component(file2);

        assert_eq!(directory.get_name(), "directory");
        assert_eq!(directory.get_size(), 300);
    }

    #[test]
    fn empty_directory_has_zero_size() {
        let directory = Directory::new("empty_directory");
        assert_eq!(directory.get_name(), "empty_directory");
        assert_eq!(directory.get_size(), 0);
    }
}
