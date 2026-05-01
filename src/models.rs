use serde::Deserialize;

#[derive(Deserialize)]
pub struct GraphQlResponse {
    pub data: Option<Data>,
}

#[derive(Deserialize)]
pub struct Data {
    pub project: Option<Project>,
}

#[derive(Deserialize)]
pub struct Project {
    #[serde(rename = "terraformStates")]
    pub terraform_states: Option<TerraformStates>,
}

#[derive(Deserialize)]
pub struct TerraformStates {
    pub nodes: Vec<Node>,
}

#[derive(Deserialize)]
pub struct Node {
    pub name: String,
}
