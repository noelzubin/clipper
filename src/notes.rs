use crate::action::Action;
use crate::error::ClipperError;
use crate::utils::get_string_from_conf;
use crate::Config;
use std::fs::OpenOptions;
use std::io::Write;
use std::process;

const FILENAME_CONFIG: &str = "notes_file";

#[derive(PartialEq, Debug)]
pub struct NotesConfig {
    filename: String,
}

impl NotesConfig {
    pub fn try_from_conf(conf: &Config) -> Result<Self, ClipperError> {
        let conf = conf.get_config();
        if !conf.contains_key(FILENAME_CONFIG) {
            return Err(ClipperError::MissingConfigKeys(vec![String::from(
                FILENAME_CONFIG,
            )]));
        }
        Ok(NotesConfig {
            filename: get_string_from_conf(conf, FILENAME_CONFIG),
        })
    }
}

#[derive(Debug)]
pub struct Notes {
    conf: NotesConfig,
}

impl Notes {
    pub fn new(conf: Config) -> Self {
        match NotesConfig::try_from_conf(&conf) {
            Err(e) => {
                eprintln!("{}", e.to_string());
                process::exit(0);
            }
            Ok(conf) => Notes { conf },
        }
    }

    pub fn save_in_notes(&self, selection: impl AsRef<str>) {
        let file = OpenOptions::new().append(true).open(&self.conf.filename);
        match file {
            Ok(mut file) => {
                writeln!(file, "{}", selection.as_ref()).unwrap();
                // file.write_all(url.as_ref().as_bytes()).unwrap();
            }
            Err(e) => {
                eprintln!("{}", e.to_string());
                process::exit(0);
            }
        };
    }
}

impl Action for Notes {
    fn perform(&self, selection: &str) {
        self.save_in_notes(selection);
    }
}
