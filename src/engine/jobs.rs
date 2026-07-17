use std::path::PathBuf;
use std::rc::Rc;
use crate::engine::context::BuildContext;

pub struct IntermediateFile {
    pub path: PathBuf,
    pub kind: String
}

pub trait Job {
    fn run(&self) -> IntermediateFile;
}

pub struct JobList {
    pub jobs: Vec<Box<dyn Job>>,
    pub name: String,
    parallel: bool
}

impl JobList {
    pub fn new(name: String) -> JobList {
        JobList {
            jobs: vec![],
            name,
            parallel: true
        }
    }

    pub fn add_job(&mut self, job: Box<dyn Job>) {
        self.jobs.push(job);
    }

    pub fn append(&mut self, other: JobList) {
        self.jobs.extend(other.jobs);
    }

    pub fn disable_parallel(mut self) -> JobList {
        self.parallel = false;

        return self;
    }
}

pub enum Program {
    Toolchain(String),
    Path(String)
}

pub struct ExternalJob {
    program: Program,
    args: Vec<String>,
}

pub struct RustJob {
    code: fn() -> IntermediateFile
}
impl RustJob {
    pub fn new(code: fn() -> IntermediateFile) -> RustJob {
        RustJob { code }
    }
}

impl Job for RustJob {
    fn run(&self) -> IntermediateFile {
        (self.code)()
    }
}

pub trait JobFactory {
    fn build(&self, context: Rc<BuildContext>) -> JobList;
}