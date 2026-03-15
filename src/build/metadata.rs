use std::{fmt::Display, fs::File, io::{BufRead, BufReader}, path::PathBuf};

use anyhow::anyhow;
use viator_utils::{array::ArrayUnwrapMany, lua::hashbrown::HashbrownMap, string::StringLifeHacks};

use crate::build::{lua::ViatorFileLua};

#[derive(Clone, Debug)]
pub enum WorkspaceType {
    Master, Sub
}

///
/// Defines the metadata 
///
/// Example:
/// ```
/// -- @v:workspaces = master
/// -- @v:minVersion = 1
/// -- @v:extern = true
/// -- 
/// -- @for_some_plugins:field = value
/// ```
///
#[derive(Clone, Debug)]
pub struct Metadata {
    pub workspaces: Option<WorkspaceType>, 
    pub min_version: Option<u32>,
    pub ext: Option<bool>,

    pub extra_meta: HashbrownMap<String, String>
}

impl Metadata {

    pub fn parse(path: PathBuf) -> anyhow::Result<Metadata> {
        if !path.exists() {
            return Err(anyhow!("Given path {:?} does not exist", path))
        }

        let mut md = Metadata {
            workspaces: None,
            min_version: None,
            ext: None,
            extra_meta: HashbrownMap::new()
        };

        let mut reader = BufReader::new(File::open(&path)?);
        loop {
            let mut line = String::new();
            let len = reader.read_line(&mut line);

            if len.is_err() {
                break;
            } 

            if len.unwrap() == 0 {
                break;
            }
            
            // If pop-ed string is not '--', we are done with metadata
            if !line.trimpeek("--") {
                break;
            }
        
            // Not a metadata but just a comment
            if !line.trimpeek("@") {
                continue; 
            }

            let [key, value] = (line.jsplit::<2, &str>("=")).unwrap_many()?;
            let [domain, field] = (key.jsplit::<2, &str>(":")).unwrap_many()?;
            
            if domain == "v" {
                match field {
                    "workspaces" => md.workspaces = match value {
                        "master" => Some(WorkspaceType::Master),
                        "sub" => Some(WorkspaceType::Sub),
                        _ => return Err(anyhow!("Unexpected value for workspaces in metadata! Expected 'master' or 'sub' but got '{}'", value))
                    },

                    "minVersion" => md.min_version = Some(value.parse()?),
                    "extern" => md.ext = Some(value.parse()?),
                    
                    _ => { }
                };
            }
            
            md.extra_meta.insert(key.to_string(), value.to_string());

        }
        
        return Ok(md)
    }
}

impl Display for Metadata {

}

pub struct ViatorFile {
    pub metadata: Metadata,
    pub viator_file: Option<ViatorFileLua>
}

