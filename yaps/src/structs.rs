use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct YBuild {
    pub id: String,
    pub version: String,
    pub release: String,
    pub runtime: Option<Vec<String>>,
    pub source: Option<Vec<String>>,
    pub build: String,
    pub skippack: Option<String>,
    pub preremove: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YInstall {
    pub postinstall: String,
}


pub struct YInfo {
    pub name: String,
    pub version: String,
    pub release: String,
    pub description: Option<String>,
    pub depends: Option<String>,
}
