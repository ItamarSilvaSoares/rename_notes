use std::env;

use dotenv::dotenv;
use rename_files::{rename_files_and_folder, messages::MsgErros};

fn main() {
    dotenv().ok();
    
    let path_rust_notes = env::var("PATH_RUST_NOTES")
        .expect(MsgErros::EnvError.msg());
    
    rename_files_and_folder(&path_rust_notes, None, None);
}