//! The command line: `staticfs HOST PORT`.

pub const USAGE: &str = "staticfs HOST PORT\n\
This program serves the files in the current directory as an HTTP server.\n\
Files and folders whose name starts with a dot are not served, and neither is anything a symbolic link leads to outside of the directory.";

#[derive(Debug, PartialEq)]
pub enum Command {
    Serve { host: String, port: u16 },
    Help,
}

/// `args` are the arguments after the program name.
pub fn parse(args: &[String]) -> Result<Command, String> {
    match args {
        [flag] if flag == "-h" || flag == "--help" => Ok(Command::Help),
        [host, port] if !host.is_empty() => {
            let port = port.parse::<u16>().map_err(|e| format!("PORT: {e}"))?;
            Ok(Command::Serve {
                host: host.clone(),
                port,
            })
        }
        _ => Err("expected HOST and PORT".to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn host_and_port() {
        assert_eq!(
            parse(&args(&["127.0.0.1", "8080"])),
            Ok(Command::Serve {
                host: "127.0.0.1".into(),
                port: 8080
            })
        );
    }

    #[test]
    fn help() {
        assert_eq!(parse(&args(&["--help"])), Ok(Command::Help));
        assert_eq!(parse(&args(&["-h"])), Ok(Command::Help));
    }

    #[test]
    fn mistakes_are_reported() {
        assert!(parse(&args(&[])).is_err());
        assert!(parse(&args(&["localhost"])).is_err());
        assert!(parse(&args(&["localhost", "http"])).is_err());
        assert!(parse(&args(&["localhost", "70000"])).is_err());
        assert!(parse(&args(&["", "80"])).is_err());
        assert!(parse(&args(&["a", "1", "2"])).is_err());
    }
}
