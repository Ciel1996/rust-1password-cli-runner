use std::process::Command;

/// This struct defines a secret from the 1Password CLI.
pub struct OnePasswordSecret {
    connection_string: String,
}

impl OnePasswordSecret {
    /// Creates a new instance of OnePasswordSecret that is connected to the
    /// given connection_string.
    ///
    /// This does not load anything. To access the 1Password CLI tool, you MUST
    /// use the .load method.
    pub fn new(connection_string : String) -> OnePasswordSecret {
        OnePasswordSecret {
            connection_string
        }
    }

    /// Loads the secret behind the connection_string of this OnePasswordSecret.
    /// It interacts with the 1Password CLI, so that must be installed!
    ///
    /// panics:
    /// if 1Password CLI is not installed!
    /// if OS is not supported!
    pub fn load(&self) -> Option<String> {
        let is_unix = cfg!(target_os = "linux") || cfg!(target_os = "macos");

       if is_unix {
            let error = Command::new("op")
                .arg("--version")
                .output()
                .expect("1Password CLI is not installed!")
                .stderr;

            if !error.is_empty() {
                panic!("{}", String::from_utf8(error).unwrap().trim());
            }

            let secret = Command::new("op")
                .arg("read")
                .arg(&self.connection_string)
                .output();

            if let Ok(secret) = secret {
                Some(String::from_utf8(secret.stdout).unwrap().trim().to_string())
            } else {
                None
            }
        } else {
            panic!("Unsupported OS!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_dummy_secret() {
        let connection_string = String::from(
            "op://DEV-Automated-Testing/rust-1password-cli-runner-test-1234/credential");
        let secret = OnePasswordSecret::new(connection_string);
        let secret = secret.load();

        assert_ne!(secret, None);

        let secret = secret.unwrap();

        assert_eq!(secret, String::from("1234"));
    }

    #[test]
    fn can_load_empty_secret() {
        let connection_string = String::from(
            "op://DEV-Automated-Testing/rust-1password-cli-runner-test-empty/credential");
        let secret = OnePasswordSecret::new(connection_string);
        let secret = secret.load();

        assert_ne!(secret, None);

        let secret = secret.unwrap();

        assert_eq!(secret, String::from(""));
    }
}
