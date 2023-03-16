pub trait AsUrl {
    #[allow(clippy::wrong_self_convention)]
    fn as_url_path(self) -> String;
}
