use std::env;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;

struct FileGuard {
    #[allow(dead_code)]
    file: Rc<File>,
}

impl FileGuard {
    fn new(file: Rc<File>) -> FileGuard {
        FileGuard { file }
    }
}

impl Drop for FileGuard {
    fn drop(&mut self) {
        println!("The file will be closed automatically.");
    }
}

pub fn demo() {
    println!("RAII Guard");

    let file_path = Path::new(&env::current_dir().unwrap()).join("src/main.rs");

    match File::open(file_path) {
        Ok(file) => {
            let shared_file = Rc::new(file);
            let _file_guard = FileGuard::new(shared_file);
            println!("File opened successfully.");
        }
        Err(e) => {
            println!("Failed to open the file: {}", e);
        }
    }

    // Здесь _file_guard выходит из области видимости и будет автоматически освобожден.
    // Это приведет к вызову Drop::drop и закрытию файла.
    //
    // Можно продолжать выполнение кода:

    println!();
}
