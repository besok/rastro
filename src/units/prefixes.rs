use crate::units::UnitError;
// si_prefixes = [
// (["Q"], ["quetta"], 1e30),
// (["R"], ["ronna"], 1e27),
// (["Y"], ["yotta"], 1e24),
// (["Z"], ["zetta"], 1e21),
// (["E"], ["exa"], 1e18),
// (["P"], ["peta"], 1e15),
// (["T"], ["tera"], 1e12),
// (["G"], ["giga"], 1e9),
// (["M"], ["mega"], 1e6),
// (["k"], ["kilo"], 1e3),
// (["h"], ["hecto"], 1e2),
// (["da"], ["deka", "deca"], 1e1),
// (["d"], ["deci"], 1e-1),
// (["c"], ["centi"], 1e-2),
// (["m"], ["milli"], 1e-3),
// (["u"], ["micro"], 1e-6),
// (["n"], ["nano"], 1e-9),
// (["p"], ["pico"], 1e-12),
// (["f"], ["femto"], 1e-15),
// (["a"], ["atto"], 1e-18),
// (["z"], ["zepto"], 1e-21),
// (["y"], ["yocto"], 1e-24),
// (["r"], ["ronto"], 1e-27),
// (["q"], ["quecto"], 1e-30),
// ]
//
//
// binary_prefixes = [
// (["Ki"], ["kibi"], 2**10),
// (["Mi"], ["mebi"], 2**20),
// (["Gi"], ["gibi"], 2**30),
// (["Ti"], ["tebi"], 2**40),
// (["Pi"], ["pebi"], 2**50),
// (["Ei"], ["exbi"], 2**60),
// ]



#[macro_export]
macro_rules! prefixes {
    ($head_name:ident; $($alias:ident, $name:ident, $value:expr),+ $(,)*) => {

        enum $head_name {
            $($name,)+
        }

        impl $head_name {
            pub fn value(&self) -> f64 {
                match self {
                        $($head_name::$name => $value,)+
                }
            }

            pub fn alias(&self) -> &'static str {
                match self {
                    $($head_name::$name => stringify!($alias),)+
                }
            }

            pub fn name(&self) -> &'static str {
                match self {
                    $($head_name::$name => stringify!($name),)+
                }
            }
        }

        impl TryFrom<&str> for $head_name {
            type Error = UnitError;
            fn try_from(value: &str) -> Result<Self, UnitError> {
                match value {
                    $(stringify!($alias) | stringify!($name)=> Ok($head_name::$name),)+
                    _ => Err(UnitError(format!("Invalid prefix: {}", value)))
                }
            }
        }

    };

}

crate::prefixes!(
    SiPrefixes;
    Q, quetta, 1e30,
    R, ronna, 1e27,
    Y, yotta, 1e24,
    Z, zetta, 1e21,
    E, exa, 1e18,
    P, peta, 1e15,
    T, tera, 1e12,
    G, giga, 1e9,
    M, mega, 1e6,
    k, kilo, 1e3,
    h, hecto, 1e2,
    da, deka, 1e1,
    d, deci, 1e-1,
    c, centi, 1e-2,
    m, milli, 1e-3,
    u, micro, 1e-6,
    n, nano, 1e-9,
    p, pico, 1e-12,
    f, femto, 1e-15,
    a, atto, 1e-18,
    z, zepto, 1e-21,
    y, yocto, 1e-24,
    r, ronto, 1e-27,
    q, quecto, 1e-30,
);

crate::prefixes!(
    BinaryPrefixes;
    Ki, kibi, 2f64.powf(10.),
    Mi, mebi, 2f64.powf(20.),
    Gi, gibi, 2f64.powf(30.),
    Ti, tebi, 2f64.powf(40.),
    Pi, pebi, 2f64.powf(50.),
    Ei, exbi, 2f64.powf(60.),
);

#[cfg(test)]
mod tests{
    use crate::units::prefixes::{BinaryPrefixes, SiPrefixes};

    #[test]
    fn smoke() {
        assert_eq!(SiPrefixes::try_from("k").unwrap().value(), 1e3);
        assert_eq!(BinaryPrefixes::try_from("Gi").unwrap().value(), 2f64.powf(30.));
        assert_eq!(SiPrefixes::exa.value(), 1e18);
    }

}