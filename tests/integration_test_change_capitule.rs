mod common;

use rename_files::change_capitule::change_capitules;

use std::ptr::null;
use counter::Counter;
use tempfile::{tempdir};

use crate::common::{create_folder_to_test, get_folder_file_names};


use pretty_assertions::{assert_eq};

#[test]
fn test_change_capitule() {
    let temp_dir = tempdir().unwrap();
    let dirs = ["01 - tmp", "02 - tmp", "03 - tmp", "04 - tmp", "05 - tmp"];

    create_folder_to_test(&temp_dir, None, &dirs);

    change_capitules::change("03");

    let result = get_folder_file_names(&temp_dir.path().to_str().unwrap(), None);
    let result = result.iter().collect::<Counter<_>>();

    let expect: Vec<String> =
            vec![
            "01 - tmp",
            "1.00 - test.md",
            "02 - tmp",
            "2.00 - test.md",
            "04 - tmp",
            "4.1.00 - test.md",
            "05 - tmp",
            "5.00 - test.md",
            "06 - tmp",
            "6.00 - test.md",
            "Introdução.md"
            ].iter().map(|&s| s.to_string()).collect();

    let expect = expect.iter().collect::<Counter<_>>();

    assert_eq!(result, expect);

}