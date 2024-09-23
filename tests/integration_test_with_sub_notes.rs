use counter::Counter;
use tempfile::tempdir;
use crate::common::get_folder_file_names;

use pretty_assertions::{assert_eq};
use rename_files::rename_files_and_folder;

mod common;

#[test]
fn integration_test_with_sub_notes() {
    let dir_t = tempdir().unwrap();
    let dirs = ["01 - tmp", "02 - tmp", "04 - tmp"];
    common::create_folder_to_test(&dir_t, None, dirs);
    
    match rename_files_and_folder(&dir_t.path().to_str().unwrap(), None, None) {
        Ok(_) => {}
        Err(e) => panic!("Algo de errado ocorreu: {:?}", e),
        };


    let result = get_folder_file_names(&dir_t.path().to_str().unwrap(), None);
    let result = result.iter().collect::<Counter<_>>();
    
    let expect: Vec<String> = 
        vec![
        "01 - tmp",
        "1.00 - test.md",
        "02 - tmp",
        "2.00 - test.md",
        "04 - tmp",
        "4.3.1.00 - test.md"
        ].iter().map(|&s| s.to_string()).collect();
        
    let expect = expect.iter().collect::<Counter<_>>();

    assert_eq!(expect, result);
}