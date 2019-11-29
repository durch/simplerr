// Creates and error for the current crate, aliases Result to use the new error and defines a from! macro to use for converting to
// the local error type.
#[macro_export]
macro_rules! err {
    ($i: ident) => {
        #[derive(Debug)]
        pub struct $i {
            pub description: Option<String>,
            pub data: Option<String>
        }

        impl From<&str> for $i {
            fn from(str: &str) -> Self {
                $i {
                    description: Some(str.to_string()),
                    data: None
                }
            }
        }

        impl core::fmt::Display for $i {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.description.as_ref() {
                    Some(err) => write!(f, "{}", err),
                    None => write!(f, "An unknown error has occurred!"),
                }
            }
        }
        pub type Result<T, E = $i> = std::result::Result<T, E>;

        macro_rules! from {
            ($t: ty) => {
                impl From<$t> for $i {
                    fn from(e: $t) -> $i {
                        $i {
                            description: Some(String::from(format!("{}", e))),
                            data: None
                        }
                    }
                }
            };
        }
    };
}