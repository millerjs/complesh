use std::io;

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

        /// General error
        Error(err: String) {
            display("{}", err)
            from(err: String) -> (err)
            from(err: &'static str) -> (err.to_string())
        }
    }
}
