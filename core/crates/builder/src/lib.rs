use anyhow::{Context, Result};
use serde::Deserialize;

mod build_function_artifact;
mod helpers;
mod run_artifact;
mod run_generated;
mod write_generated_project;

pub use build_function_artifact::build_function_artifact;
pub use run_artifact::run_artifact;
pub use run_generated::run_generated;

pub(crate) const ENTRY_FUNCTION: &str = "__ts2rust_entry";
pub(crate) const GENERATED_EDITION: &str = "2021";

#[derive(Debug, Deserialize)]
pub struct FunctionSignature {
    pub params: Vec<FunctionParam>,
}

impl FunctionSignature {
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).context("failed to parse function signature")
    }
}

#[derive(Debug, Deserialize)]
pub struct FunctionParam {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}
