use std::fs::{self, rename};
use std::io;
use std::num::ParseIntError;
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

pub fn rename_files_and_folder(
    path_folder: &str,
    chapter_number: Option<&u32>,
    flag: Option<u8>) -> Result<(), AppErrors> {
    let flag = get_flag(flag);
    let paths = fs::read_dir(&path_folder)
        .expect(MsgErros::ReadDirError.msg());


    for path in paths {
        let folder_name = path?.file_name();
        let folder_name = folder_name.to_str()
            .ok_or(AppErrors::IoError(
                io::Error::new(
                    io::ErrorKind::Other,
                    MsgErros::FolderNameError.msg())))?;

        let old_name = format!("{}/{}", &path_folder, &folder_name);
        let is_file_bool = Path::new(&old_name).is_file();

        if (!is_valid_to_rename(&folder_name) && folder_name != RESOURCES)
            || (!is_file_bool && dir_start_with_chapter(&folder_name)) ||
            (is_file_bool && is_file(&folder_name, &chapter_number)?) {
            continue;
        };

        let chapter_number: u32 = get_chapter(&folder_name, chapter_number)?;

        if chapter_number == 0 {
            continue;
        }


        if (!dir_start_with_chapter(&folder_name) && flag == 1
            && Path::new(&old_name).is_dir())
            || (folder_name == RESOURCES) {
            rename_files_and_folder(&old_name, Some(&chapter_number), Some(flag + 1))?;
            continue;
        }


        let new_name = format!("{}/{}.{}", &path_folder, chapter_number, folder_name);


        rename(old_name, new_name)?;
    }
    Ok(())
}

#[derive(Debug)]
pub enum AppErrors {
    ParseIntError(ParseIntError),
    RegexError,
    IoError(io::Error),
}

impl From<ParseIntError> for AppErrors {
    fn from(err: ParseIntError) -> Self {
        AppErrors::ParseIntError(err)
    }
}
impl From<io::Error> for AppErrors {
    fn from(err: io::Error) -> Self {
        AppErrors::IoError(err)
    }
}

fn get_chapter(folder_name: &str, chapter_number: Option<&u32>) -> Result<u32, AppErrors> {
    if folder_name == RESOURCES && chapter_number.is_none() {
        return Ok(0);
    }

    match chapter_number {
        Some(&i) => Ok(i),
        None => {
            let mat = RE.find(folder_name).ok_or(AppErrors::RegexError)?;
            let chapter = mat.as_str().parse::<u32>()?;
            Ok(chapter)
        }
    }
}

fn is_valid_to_rename(folder_name: &str) -> bool {
    RE.is_match(&folder_name)
}

fn dir_start_with_chapter(path: &str) -> bool {
    RE_NUMBER.is_match(&path)
}

fn is_file(path: &str, chapter_number: &Option<&u32>) -> Result<bool, AppErrors> {
    let number_in_file = get_chapter(&path, None)?;

    Ok(number_in_file == *chapter_number.unwrap())
}

#[cfg(test)]
mod tests {
    use super::AppErrors;
    use super::*;

    macro_rules! assert_err {
        ($expression:expr, $($pattern:tt)+) => {
            match $expression {
                $($pattern)+ => (),
                ref e => panic!("expected `{}` but got `{:?}`", stringify!($($pattern)+), e),
            }
        }
    }

    #[test]
    fn get_chapter_in_folder_name() {
        let folder_name = "1 - tests";
        let chapter_number = get_chapter(folder_name, None).unwrap();
        assert_eq!(chapter_number, 1);
    }

    #[test]
    fn get_chapter_in_variable_some() {
        let folder_name = "1 - test";
        let number: Option<&u32> = Some(&2);
        let chapter_number = get_chapter(folder_name, number).unwrap();

        assert_eq!(chapter_number, 2);
        assert_ne!(chapter_number, 1);
    }

    #[test]
    fn get_chapter_error_chapter_no_found() {
        let folder_name = "test";

        assert_err!(get_chapter(folder_name, None), Err(AppErrors::RegexError))
    }

    #[test]
    fn get_chapter_folder_name_igual_resource() {
        let folder_name = "_resources";

        let chapter_number = get_chapter(&folder_name, None).unwrap();

        assert_eq!(chapter_number, 0);
    }

    #[test]
    fn start_with_chapter_false() {
        let folder_name = "test";
        let result = dir_start_with_chapter(folder_name);
        assert_ne!(result, true)
    }

    #[test]
    fn start_with_chapter_true() {
        let folder_name = "3.00 test";
        let result = dir_start_with_chapter(folder_name);
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
