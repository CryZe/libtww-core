use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    pub src: Src,
    pub build: Build,
    pub link: Link,
}

#[derive(Deserialize)]
pub struct Src {
    pub src: PathBuf,
    pub iso: PathBuf,
    pub link: Vec<PathBuf>,
    pub patch: PathBuf,
}

#[derive(Deserialize)]
pub struct Build {
    pub map: PathBuf,
    pub iso: PathBuf,
}

#[derive(Deserialize)]
pub struct Link {
    pub entries: Vec<String>,
    pub base: String,
}
