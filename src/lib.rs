use std::fs::{self, rename};
use std::path::Path;

use regex::Regex;

use crate::messages::{MsgErros, RESOURCES};

pub mod messages;

lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d{1,2}").unwrap();
        static ref RE_NUMBER: Regex = Regex::new(r"^\d{1,2}\.").unwrap();
    }

fn get_flag(flag: Option<u8>) -> u8 {
    flag.unwrap_or(1)
}

pub fn rename_files_and_folder(path_folder: &str, chapter_number: Option<&u32>, flag: Option<u8>) {
    let flag = get_flag(flag);
    let paths = fs::read_dir(&path_folder)
        .expect(MsgErros::ReadDirError.msg());


    for path in paths {
        let path = path
            .expect(MsgErros::DirEntryError.msg())
            .file_name();
        let folder_name = path.to_str()
            .expect(MsgErros::FolderNameError.msg());


        if (!is_valid_to_rename(&folder_name) && folder_name != RESOURCES)
            | start_with_chapter(&folder_name) {
            continue;
        };

        let chapter_number: u32 = get_chapter(folder_name, chapter_number);
        let old_name = format!("{}/{}", &path_folder, &folder_name);

        if (!start_with_chapter(&folder_name) && flag == 1
            && Path::new(&old_name).is_dir())
            | (folder_name == RESOURCES) {
            rename_files_and_folder(&old_name, Some(&chapter_number), Some(flag + 1));
            continue;
        }

        let new_name = format!("{}/{}.{}", &path_folder, chapter_number, folder_name);

        rename(old_name, new_name)
            .expect(MsgErros::RenameError.msg());
    }
}

fn get_chapter(folder_name: &str, chapter_number: Option<&u32>) -> u32 {
    match chapter_number {
        Some(&i) => i,
        None => {
            RE.find(folder_name).map(|mat| mat.as_str().parse::<u32>()
                .unwrap_or_else(|err| panic!("{} {:?}", MsgErros::ParseIntError.msg(), err)))
                .unwrap_or_else(|| panic!("{}", MsgErros::RegexError.msg()))
        }
    }
}

fn is_valid_to_rename(folder_name: &str) -> bool {
    RE.is_match(&folder_name)
}

fn start_with_chapter(path: &str) -> bool {
    RE_NUMBER.is_match(&path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_chapter_in_folder_name() {
        let folder_name = "1 - tests";
        let chapter_number = get_chapter(folder_name, None);
        assert_eq!(chapter_number, 1);
    }

    #[test]
    fn get_chapter_in_variable_some() {
        let folder_name = "1 - test";
        let number: u32 = 2;
        let chapter_number = get_chapter(folder_name, Some(&number));

        assert_eq!(chapter_number, 2);
        assert_ne!(chapter_number, 1);
    }

    #[test]
    #[should_panic(expected = "Arquivo ou pasta não possui o capítulo no nome")]
    fn get_chapter_error_chapter_no_found() {
        let folder_name = "test";
        get_chapter(folder_name, None);
    }

    #[test]
    fn start_with_chapter_false() {
        let folder_name = "test";
        let result = start_with_chapter(folder_name);
        assert_ne!(result, true)
    }

    #[test]
    fn start_with_chapter_true() {
        let folder_name = "3.00 test";
        let result = start_with_chapter(folder_name);
        assert_eq!(result, true)
    }

    #[test]
    fn is_valid_to_rename_true() {
        let folder_name = "3 test";
        let result = is_valid_to_rename(folder_name);
        assert_eq!(result, true)
    }

    #[test]
    fn is_valid_to_rename_false() {
        let folder_name = "test";
        let result = is_valid_to_rename(folder_name);
        assert_ne!(result, true)
    }
}
