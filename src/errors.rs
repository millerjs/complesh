use std::{io, result};
use nix;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        /// Simple io::Error wrapper
        IoErr(err: io::Error) { from() cause(err) }

        /// Wrapper for io::Error with path context
        IoErrContext(err: io::Error, path: String) {
            display("{}: {}", path, err)
            context(path: &'a str, err: io::Error) -> (err, path.to_owned())
            cause(err)
        }

        /// Wrapper for nix::Error without context
        NixError(err: nix::Error) { from() cause(err) }

        /// General error
        Error(err: String) {
            display("{}", err)
            from(err: String) -> (err)
            from(err: &'static str) -> (err.to_string())
        }
    }
}


pub type Result<T> = result::Result<T, Error>;
