#[derive(StructOpt, Debug)]
pub enum Opt {
    /// Builds the Rom Hack
    #[structopt(name = "build")]
    Build {},
    /// Creates a new Rom Hack with the given name
    #[structopt(name = "new")]
    New { name: String },
}
