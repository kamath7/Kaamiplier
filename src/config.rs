use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProjectConfig {
    pub name: String,
    #[allow(dead_code)]
    pub language: String,
    pub source: String,
    pub output: String,
    pub main_class: String,
}