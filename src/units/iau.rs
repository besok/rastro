#[macro_use]
mod length {
    quantity! {
            /// Length (base unit parsec, pc).
            quantity: Length; "length";
            /// Length dimension, pc.
            dimension: Q<
                P1 /*length*/,
                Z0 /*mass*/,
                Z0 /*time*/,
                Z0 /*energy*/,
                Z0 /*light*/
            >;
            units {
                @meter: 1.0E0; "m", "meter", "meters";
                @parsec: 3.08567758149E16; "pc", "parsec", "parsecs";
                @light_year: 9.461E15; "ly", "light-year", "light-years";
                @light_second: 2.998E8; "ls", "light-second", "light-seconds";
                @astronomical_unit: 1.496E11; "AU", "astronomical unit", "astronomical units";
                @sol_rad: 6.957e8; "R_sun", "Solar radius", "Solar radius";
                @jupiter_rad: 7.1492e7; "R_jup", "Jupiter radius", "Jupiter radius";
                @earth_rad: 6.371e6; "R_earth", "Earth radius", "Earth radius";
            }
        }
}

#[macro_use]
mod light {
    quantity! {
            /// Length (base unit parsec, pc).
            quantity: Light; "light";
            /// Length dimension, pc.
            dimension: Q<
                Z0 /*length*/,
                P1 /*light*/,
                Z0 /*time*/,
                Z0 /*energy*/,
                Z0 /*mass*/
            >;
            units {
                @watt: 1.0; "W", "watt", "watts";
                @sol_lum: 3.828e26; "L_sun", "Solar luminosity", "Solar luminosity";
                @photon: 6.626e-34; "photon", "photon", "photon";
                @jansky: 1.0e-26; "Jy", "Jansky", "Jansky";
                @rayleigh: 1.0e-26; "R", "Rayleigh", "Rayleigh";
            }
        }
}

#[macro_use]
mod energy {
    #[deny(overflowing_literals)]
    quantity! {
        /// Energy (base unit joule, J).
        quantity: Energy; "energy";
        /// Energy dimension, J.
        dimension: Q<
            Z0 /*length*/,
            Z0 /*light*/,
            P1 /*energy*/,
            Z0 /*time*/,
            Z0 /*mass*/
        >;
        units {
            @joule: 1.0; "J", "joule", "joules";
            @rydberg: 2.17987e-18; "Ry", "Rydberg", "Rydbergs";
            @foe: 1.0e44; "foe", "Foe", "Foe";
            @bethe: 1.0e44; "B", "Bethe", "Bethe";
        }
    }
}

#[macro_use]
mod mass {
    quantity! {
        /// Mass (base unit kilogram, kg).
        quantity: Mass; "mass";
        /// Mass dimension, kg.
        dimension: Q<
            Z0 /*length*/,
            Z0 /*light*/,
            Z0 /*energy*/,
            P1 /*mass*/,
            Z0 /*time*/
        >;
        units {
            @kilogram: 1.0; "kg", "kilogram", "kilograms";
            @sol_mass: 1.989e30; "M_sun", "Solar mass", "Solar mass";
            @jupiter_mass: 1.898e27; "M_jup", "Jupiter mass", "Jupiter mass";
            @earth_mass: 5.972e24; "M_earth", "Earth mass", "Earth mass";
        }
    }
}

#[macro_use]
mod time {
    quantity! {
            quantity: Time; "time";
            dimension: Q<
                Z0 /*length*/,
                Z0 /*light*/,
                Z0 /*energy*/,
                Z0 /*mass*/,
                P1 /*time*/
            >;
            units {
                @second: 1.0; "s", "second", "seconds";
                @julian_year: 3.15576E7; "yr", "Julian year", "Julian years";
            }
        }
}

system! {
    quantities: Q {
        length: meter, L;
        mass: kilogram, M;
        time: second, T;
        energy: joule, J;
        light: watt, W;
    }

    units: U {
        mod length::Length,
        mod light::Light,
        mod energy::Energy,
        mod mass::Mass,
        mod time::Time,
    }
}
pub mod f64 {
    mod mks {
        pub use super::super::*;
    }

    Q!(self::mks, f64);
}

#[cfg(test)]
mod tests {
    use crate::units::iau;
    use crate::units::iau::length::{astronomical_unit, parsec, sol_rad};
    use crate::units::iau::time::julian_year;

    #[test]
    fn test() {
        let au = iau::f64::Length::new::<astronomical_unit>(1.);
        let au2 = iau::f64::Length::new::<astronomical_unit>(1.);
        println!("{:?} + {:?} = {:?}", au, au2, au + au2);
        let ps = iau::f64::Length::new::<parsec>(1.);
        let s_r = iau::f64::Length::new::<sol_rad>(1.);
        let y_y = iau::f64::Time::new::<julian_year>(1.);
    }
}