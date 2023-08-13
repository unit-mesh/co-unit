use crate::model::{CodeDataStruct, ContainerService};

pub mod api_analyser;


trait ApiAnalyser {
    fn analysis_by_node(&mut self, node: &CodeDataStruct, workspace: String);
    fn to_container_services(&self) -> Vec<ContainerService>;
    fn new() -> Self;
}
