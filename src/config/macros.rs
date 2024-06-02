
/// This macro is used to define a configuration variable.
/// It creates a struct with a single field, the value of the variable.
/// The struct has methods to get and set the value.
/// The struct implements the Default trait.
/// The struct has a method to convert a TOML Value to the struct.
///
/// See usages in the values.rs file.
///
#[macro_export]
macro_rules! cfg_var {
    (
        name: $name:ident,
        type: $tpe:ty,
        dsc: $dsc:expr,
        default: $default:expr
    ) => {
        #[derive(Debug)]
        pub struct $name {
            value:$tpe,
        }
        impl $name {
            pub fn new(value: $tpe) -> Self {
                $name {
                    value: value
                }
            }
            pub fn set(&mut self, value: $tpe) {
                self.value = value;
            }
            pub fn get(&self) -> &$tpe {
                &self.value
            }
            pub fn dsc() -> String {
                $dsc.to_string()
            }

            pub fn from_toml_value(toml_value: &Value) -> Result<$name, ConfigurationError> {
                let value: $tpe = <$tpe>::from_toml_value(toml_value)?;
                Ok($name::new(value))
            }
        }
        impl Default for $name {
            fn default() -> Self {
                $name {
                    value: $default
                }
            }
        }
    };
}

/// This macro is used to define a configuration namespace.
/// It creates a struct with fields for each variable in the namespace.
/// It creates a method to get the name of the namespace.
/// It creates a method to convert the namespace to a TOML section.
/// It creates a method to convert a TOML Table to the namespace.
///
/// See usages in the values.rs file.
#[macro_export]
macro_rules! cfg_namespace {
    (
        name: $name:expr,
        tpe: $tpe:ident,
        body: $($v:ident : $tp:tt),+ $(,)*
    ) => {
        #[derive(Debug,Default)]
        pub struct $tpe {
           $(pub $v: $tp,)*
        }
        impl $tpe {
            pub fn new(
                $($v: $tp),*
            ) -> Self {
                $tpe {
                    $($v: $v,)*
                }
            }
            pub fn name() -> String {
                $name.to_string()
            }

            pub fn to_toml_section(&self) -> String {
                let mut s = String::new();
                s.push_str(&format!("[{}]\n", $name.to_string()));
                $(
                    s.push_str(&format!("{}\n", $tp::dsc()));
                    s.push_str(&format!("{} = {:?}\n", stringify!($v), self.$v.get()));
                )*
                s
            }
            pub fn from_toml_section(toml_table:&Table) -> Result<$tpe, ConfigurationError> {
                let mut res = $tpe::default();
                if let Some(section) = toml_table.get($name) {
                    if let Value::Table(section) = section {
                        $(
                            let toml_value = section.get(stringify!($v));
                            if let Some(toml_value) = toml_value {
                                res.$v = $tp::from_toml_value(toml_value)?;
                            }
                        )*
                        Ok(res)
                    }else {
                        Err(ConfigurationError(
                            format!("Invalid TOML file. The section {} has wrong format. \
                            Expected a table, got {:?}", $name, section)))
                    }

                } else {
                    Ok(res)
                }
            }
        }


    };
}
