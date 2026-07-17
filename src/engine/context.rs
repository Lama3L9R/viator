use std::path::PathBuf;
use anyhow::Error;
use hashbrown::HashMap;
use viator_utils::path::PathBufLifeHack;
use crate::engine::jobs::IntermediateFile;

///
/// Contains intermediate and product files generate during pipeline execution
/// These files are supposed to be readonly
///
#[derive(Default)]
pub struct BuildContext {
    intermediates: HashMap<String, Vec<IntermediateFile>>,
    artifacts: HashMap<String, Vec<IntermediateFile>>,
    intermediate_dir: PathBuf,
    result_dir: PathBuf
}

impl BuildContext {
    pub fn new(intermediate_dir: PathBuf, result_dir: PathBuf) -> Result<BuildContext, Error> {
        let intermediate_dir = intermediate_dir.ensure_dir()?.mkdir()?;
        let result_dir = result_dir.ensure_dir()?.mkdir()?;

        let ctx = BuildContext {
            intermediates: Default::default(),
            artifacts: Default::default(),
            intermediate_dir,
            result_dir
        };

        return Ok(ctx);
    }

    pub fn push_intermediates(&mut self, kind: String, mut intermediates: Vec<IntermediateFile>) {
        if self.intermediates.contains_key(&kind) {
            self.intermediates.get_mut(&kind).unwrap().append(&mut intermediates);
        } else {
            self.intermediates.insert(kind, intermediates);
        }
    }

    pub fn push_intermediate(&mut self, kind: String, intermediate: IntermediateFile) {
        if self.intermediates.contains_key(&kind) {
            self.intermediates.get_mut(&kind).unwrap().push(intermediate);
        } else {
            self.intermediates.insert(kind, vec![intermediate]);
        }
    }

    pub fn push_artifact(&mut self, kind: String, artifact: IntermediateFile) {
        if self.artifacts.contains_key(&kind) {
            self.artifacts.get_mut(&kind).unwrap().push(artifact);
        } else {
            self.artifacts.insert(kind, vec![artifact]);
        }
    }

    pub fn push_artifacts(&mut self, kind: String, mut artifacts: Vec<IntermediateFile>) {
        if self.artifacts.contains_key(&kind) {
            self.artifacts.get_mut(&kind).unwrap().append(&mut artifacts);
        }
    }

    pub fn get_intermediates(&self, kind: String) -> Option<&Vec<IntermediateFile>> {
        self.intermediates.get(&kind)
    }

    pub fn get_artifacts(&self, kind: String) -> Option<&Vec<IntermediateFile>> {
        self.artifacts.get(&kind)
    }

    pub fn prepare_intermediates(&self, kind: String) -> Result<PathBuf, Error> {
        Ok(self.intermediate_dir.join(kind).ensure_dir()?.mkdir()?)
    }
}