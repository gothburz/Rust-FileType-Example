use std::fs;

fn main() {
    // DECLARE CONSTANTS
    const file: &str = "./test/example.txt";
    const dir: &str = "./test";

    // FILE EXAMPLE
    match fs::metadata(file) {
        Ok(file_metadata_result) => {
            let file_type_a = file_metadata_result.file_type();
            println!("{:?}", file_type_a);
            println!("Is Dir?: {:?}", file_type_a.is_dir());
            println!("Is File?: {:?}", file_type_a.is_file());
            println!("Is Symlink?: {:?}", file_type_a.is_symlink());
        }
        _ => {}
    }

    // DIRECTORY SAMPLE
    match fs::metadata(dir) {
        Ok(dir_metadata_result) => {
            let mut file_type_b = dir_metadata_result.file_type();
            println!("FILETYPE Struct: {:?}", file_type_b);
            println!("Is Dir?: {:?}", file_type_b.is_dir());
            println!("Is File?: {:?}", file_type_b.is_file());
            println!("Is Symlink?: {:?}", file_type_b.is_symlink());
            // CLONING EXAMPLE
            let filetype_clone_b = file_type_b.clone();
            println!("CLONED FILETYPE: {:?}", filetype_clone_b);
            let metadata = fs::metadata(file).unwrap();
            let file_type_to_clone = metadata.file_type();
            // CLONE FROM
            file_type_b.clone_from(&file_type_to_clone);
            println!("Is File now?: {:?}", file_type_b.is_file());
        }
        _ => {}
    }
}
