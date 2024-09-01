#[derive(Debug)]
pub enum Editor {
    VSCode, // default
    Neovim,
    Vim,
    Emacs,
}

impl Editor {
    // instance a new blank editor
    pub fn new() -> Editor {
        Editor::VSCode
    }
    fn from_str(s: &str) -> Result<Editor, String> {
        match s {
            "VSCode" => Ok(Editor::VSCode),
            "Neovim" => Ok(Editor::Neovim),
            "Vim" => Ok(Editor::Vim),
            "Emacs" => Ok(Editor::Emacs),
            _ => Err("Invalid editor".to_string()),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Editor::VSCode => "VSCode",
            Editor::Neovim => "Neovim",
            Editor::Vim => "Vim",
            Editor::Emacs => "Emacs",
        }
    }
}

// impl Editor {
//     pub fn editor_string(&self) -> String {
//         match self {
//             Editor::VSCode => "VSCode".to_string(),
//             Editor::Neovim => "Neovim".to_string(),
//             Editor::Vim => "Vim".to_string(),
//             Editor::Emacs => "Emacs".to_string(),
//         }
//     }
// }
