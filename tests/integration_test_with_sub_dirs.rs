
mod common;
use counter::Counter;
use tempfile::{tempdir};

use rename_files::rename_files_and_folder;
use crate::common::{create_folder_to_test, get_folder_file_names};

use pretty_assertions::{assert_eq};

#[test]
fn integration_test_with_sub_dirs() {
    let dir_t = tempdir().unwrap();
    let dirs = ["01 - tmp", "02 - tmp", "03.1 - tmp"];

    create_folder_to_test(&dir_t, Some("_resources/01 - res"), dirs);
    
    
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
    "03.1 - tmp",
    "3.1.00 - test.md",
    "_resources",
    "01 - res",
    "2.01 - res"].iter().map(|&s| s.to_string()).collect();
    
    let expect = expect.iter().collect::<Counter<_>>();
    
    assert_eq!(expect, results);

}
