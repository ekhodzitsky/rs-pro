pub enum FileSystemComponent {
    File {
        name: String,
        size: u64,
    },
    Directory {
        name: String,
        components: Vec<FileSystemComponent>,
    },
}

impl FileSystemComponent {
    pub fn get_size(&self) -> u64 {
        match self {
            FileSystemComponent::File { size, .. } => *size,
            FileSystemComponent::Directory { components, .. } => components
                .iter()
                .map(|component| component.get_size())
                .sum(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_size_is_correct() {
        let file = FileSystemComponent::File {
            name: "file.txt".to_string(),
            size: 100,
        };
        assert_eq!(file.get_size(), 100);
    }

    #[test]
    fn empty_directory_size_is_zero() {
        let directory = FileSystemComponent::Directory {
            name: "empty_dir".to_string(),
            components: Vec::new(),
        };
        assert_eq!(directory.get_size(), 0);
    }

    #[test]
    fn directory_size_is_sum_of_its_contents() {
        let file1 = FileSystemComponent::File {
            name: "file1.txt".to_string(),
            size: 100,
        };
        let file2 = FileSystemComponent::File {
            name: "file2.txt".to_string(),
            size: 200,
        };
        let directory = FileSystemComponent::Directory {
            name: "dir".to_string(),
            components: vec![file1, file2],
        };
        assert_eq!(directory.get_size(), 300);
    }

    #[test]
    fn nested_directory_size_is_correct() {
        let file1 = FileSystemComponent::File {
            name: "file1.txt".to_string(),
            size: 100,
        };
        let file2 = FileSystemComponent::File {
            name: "file2.txt".to_string(),
            size: 200,
        };
        let subdir = FileSystemComponent::Directory {
            name: "subdir".to_string(),
            components: vec![file1],
        };
        let root_dir = FileSystemComponent::Directory {
            name: "root".to_string(),
            components: vec![subdir, file2],
        };
        assert_eq!(root_dir.get_size(), 300);
    }
}
