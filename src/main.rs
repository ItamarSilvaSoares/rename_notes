use std::env;

use dotenv::dotenv;

use rename_files::{messages::MsgErros, rename_files_and_folder};

fn main() {
    dotenv().ok();
    
    let path_rust_notes = env::var("PATH_RUST_NOTES")
        .expect(MsgErros::EnvError.msg());
    
    match rename_files_and_folder(&path_rust_notes, None, None) {
        Ok(_) => println!("Renomeação concluída com sucesso!"),
        Err(e) => eprintln!("Erro durante a renomeação: {:?}", e),
    };
}