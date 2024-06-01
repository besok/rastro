use toml::{Value};
#[macro_export]
macro_rules! var {
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
                let tpe_str = stringify!($tpe);
                let name_str = stringify!($name);
                let value: $tpe =
                toml_value
                    .try_into()
                    .map_err(|_| ConfigurationError(
                        format!("\
                        the conversion error for {name_str} occured. \
                        The type = {tpe_str} \
                        ")
                ))?;
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
#[macro_export]
macro_rules! namespace {
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
        }


    };
}
