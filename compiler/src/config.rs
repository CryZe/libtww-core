use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    pub src: Src,
    pub out: Out,
    pub link: Link,
}

#[derive(Deserialize)]
pub struct Src {
    pub src: PathBuf,
    pub dol: PathBuf,
    pub link: Vec<PathBuf>,
    pub patch: PathBuf,
}

#[derive(Deserialize)]
pub struct Out {
    pub dol: PathBuf,
    pub map: PathBuf,
    pub iso: Option<PathBuf>,
}

#[derive(Deserialize)]
pub struct Link {
    pub entries: Vec<String>,
    pub base: String,
}

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(short = "i", long = "iso")]
    pub iso: bool,
}
