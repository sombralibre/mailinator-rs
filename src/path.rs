pub(super) trait AsUrlPath {
    #[allow(clippy::wrong_self_convention)]
    fn as_url_path(self) -> String;
}
