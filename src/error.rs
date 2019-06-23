use failure::Fail;

#[derive(Debug, Fail)]
pub enum ClipperError {
    #[fail(display = "missing config values: {:?}", _0)]
    MissingConfigKeys(Vec<String>),
}
