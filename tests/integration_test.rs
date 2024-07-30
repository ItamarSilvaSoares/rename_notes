
use std::fs::{self, File};
use std::path::Path;

use counter::Counter;
use tempfile::{tempdir, TempDir};

use rename_files::rename_files_and_folder;

#[test]
fn integration_test() {
    let dir_t = tempdir().unwrap();

    create_file_folder_to_test(&dir_t);
    
    
    match rename_files_and_folder(&dir_t.path().to_str().unwrap(), None, None) {
    Ok(_) => {}
    Err(e) => panic!("Algo de errado ocorreu: {:?}", e),
    };
    
    let results = get_folder_file_names(&dir_t.path().to_str().unwrap(), None);
    let results = results.iter().collect::<Counter<_>>();
    
    let expect: Vec<String> = 
    vec![
    "01 - tmp",
    "1.00 - test.md",
    "_resources",
    "1.01 - res",
    "Introdução.md",
    "_resources",
    "01 - res",
    "02 - tmp",
    "2.00 - test.md", 
    "_resources",
    "07.1 - tmp",
    "7.1.00 - test.md",
    "_resources",
    "01 - res",
    "2.01 - res"].iter().map(|&s| s.to_string()).collect();
    
    let expect = expect.iter().collect::<Counter<_>>();
    
    assert_eq!(expect, results);

}

fn create_file_folder_to_test(dir_t: &TempDir) {
    let resource = "_resources/01 - res";
    let dirs = ["01 - tmp", "02 - tmp", "07.1 - tmp"];
    
    for (index, dir) in dirs.iter().enumerate() {
        let tmp = &dir_t.path().to_str().unwrap();
        
        let folder = format!("{}/{}", &tmp, dir);
        fs::create_dir(&folder).expect("falha ao criar as pastas");

        let mut file = format!("{}/00 - test.md", &folder);

        if index == 2 {
        file = format!("{}/7.1.00 - test.md", &folder)
        }

        File::create(file).expect("falha ao criar o arquivo");

        let folder = format!("{}/{}", folder, resource);

        fs::create_dir_all(folder).unwrap();

        if index == 0 {
        let file = format!("{}/Introdução.md", &tmp);
        File::create(file).unwrap();

        let aux = format!("{}/{}", &tmp, resource);
        fs::create_dir_all(aux).unwrap();
        }
    }
}

fn get_folder_file_names(path: &str, flag: Option<u8>) -> Vec<String> {
    let flag = flag.unwrap_or(1);
    
    let dirs = fs::read_dir(&path).unwrap();
    
    let mut  results = Vec::new();

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


