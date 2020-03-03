use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Error {
    pub(crate) kind: Kind,
    pub(crate) cause: String,
}

#[derive(PartialEq, Debug)]
pub(crate) enum Kind {
    HTTP,
    HTML,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            Kind::HTTP => f.write_str("HTTP error")?,
            Kind::HTML => f.write_str("HTML error")?,
        }
        write!(f, "{}", self.cause)?;
        Ok(())
    }
}
