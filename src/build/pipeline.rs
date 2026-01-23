use crate::build::action::ConfigBoundAction;
use std::rc::Rc;
use std::sync::Arc;

pub enum PipelineAction {
    Pipeline(Vec<Pipeline>),
    Act(Rc<ConfigBoundAction>),
}

pub struct Pipeline {
    pub name: Option<String>,
    pub parent: Option<Arc<Pipeline>>,
    pub actions: Vec<PipelineAction>,
}
