pub trait PageMeta {
    fn title(&self) -> &'static str;
    fn description(&self) -> &'static str;
}
