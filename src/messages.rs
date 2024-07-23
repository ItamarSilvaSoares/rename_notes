pub static RESOURCES: &str = "_resources";

pub enum MsgErros {
    ReadDirError,
    DirEntryError,
    FolderNameError,
    RenameError,
    ParseIntError,
    RegexError,
    EnvError,
}

impl MsgErros {
    pub fn msg(self) -> &'static str {
        match self {
            MsgErros::ReadDirError => "Não foi possível ler o diretório",
            MsgErros::DirEntryError => "Não foi possível obter o caminho",
            MsgErros::FolderNameError => "Nome de arquivo inválido",
            MsgErros::RenameError => "Erro ao renomear o arquivo ou pasta",
            MsgErros::ParseIntError => "Não foi possível converter o número do capitulo:",
            MsgErros::RegexError =>  "Arquivo ou pasta não possui o capítulo no nome",
            MsgErros::EnvError => "A variável PATH_RUST_NOTES deve ser configurada",
        }
    }
}