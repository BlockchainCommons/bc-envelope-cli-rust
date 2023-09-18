pub trait Exec {
    fn exec(&self) -> anyhow::Result<String>;
}
