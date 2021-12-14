use std::io;
use std::path::PathBuf;

use crate::hqr_ress::load_hqr;

#[derive(Debug)]
pub struct Message {
    root: PathBuf,
    last_file_init: Option<usize>,
    language: usize,

    max_text: usize,
    flag_speak: bool,

    buffer_text: [u8; 25000],
    buffer_order: [u8; 1024],
}

// const LIST_LANGUAGE: [&str; 5] = ["EN_", "FR_", "DE_", "SP_", "IT_"];

// const LIST_FILE_TEXT: [&str; 15] = [
//     "sys", "cre", "gam", "000", "001", "002", "003", "004", "005", "006", "007", "008", "009",
//     "010", "011",
// ];

const NAME_HQR_TEXT: &str = "text.hqr";

const MAX_TEXT_LANG: usize = 14;

impl Message {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            last_file_init: None,
            language: 1, // English
            max_text: 0,
            flag_speak: false,
            buffer_text: [0; 25000],
            buffer_order: [0; 1024],
        }
    }

    pub fn init_dial(&mut self, file_index: usize) -> io::Result<()> {
        let last_file_index = self.last_file_init.replace(file_index);
        if last_file_index == Some(file_index) {
            return Ok(());
        }

        let path = self.root.join(NAME_HQR_TEXT);

        let index = (self.language * MAX_TEXT_LANG * 2 + file_index * 2) / 2;
        let max_text = load_hqr(&path, &mut self.buffer_order, index)?;
        self.max_text = max_text;

        let index = self.language * MAX_TEXT_LANG * 2 + file_index + 1;
        load_hqr(path, &mut self.buffer_text, index)?;

        if self.flag_speak {
            // self.init_speak(file_index)?;
        }

        Ok(())
    }
}
