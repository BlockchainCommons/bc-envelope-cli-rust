pub trait Exec {
    fn exec(&self) -> Result<String, anyhow::Error>;
}
