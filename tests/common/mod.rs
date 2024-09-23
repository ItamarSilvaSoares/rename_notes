use std::fs;
use std::fs::File;
use std::path::Path;
use tempfile::TempDir;

pub fn get_folder_file_names(path: &str, flag: Option<u8>) -> Vec<String> {
    let flag = flag.unwrap_or(1);

    let dirs = fs::read_dir(&path).unwrap();

    let mut results = Vec::new();

    for dir in dirs {
        let file_name = dir.unwrap().file_name();
        let file_name = file_name.to_str().unwrap().to_string();
        let new_path = format!("{}/{}", path, &file_name);

        results.push(file_name);

        if Path::new(&new_path).is_dir() && flag <= 2 {
            let vec_string = get_folder_file_names(&new_path, Some(flag + 1));
            results.extend(vec_string);
        }
    }

    results
}

pub fn create_folder_to_test(dir_t: &TempDir, resource: Option<&str>, dirs: [&str; 3]) {
    for (index, dir) in dirs.iter().enumerate() {
        let tmp = &dir_t.path().to_str().unwrap();

        let folder = format!("{}/{}", &tmp, dir);
        fs::create_dir(&folder).expect("falha ao criar as pastas");

        let mut file = format!("{}/00 - test.md", &folder);

        if index == 2 {
            file = format!("{}/3.1.00 - test.md", &folder)
        }

        File::create(file).expect("falha ao criar o arquivo");

        if resource.is_some() {
            let folder = format!("{}/{}", folder, resource.unwrap());
            fs::create_dir_all(folder).unwrap();
        }


        if index == 0 {
            let file = format!("{}/Introdução.md", &tmp);
            File::create(file).unwrap();

            if resource.is_some() {
                let aux = format!("{}/{}", &tmp, resource.unwrap());
                fs::create_dir_all(aux).unwrap();
            }
        }
    }
}
