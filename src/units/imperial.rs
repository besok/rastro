//! Imperial units

pub use uom::si::*;

ISQ!(uom::si, f64, (foot, pound, second, ampere, kelvin, mole, candela));
pub mod temperature {
    unit! {
        system: crate::units::imperial;
        quantity: crate::units::imperial::thermodynamic_temperature;

        @rankine: 5.0 / 9.0; "°R", "rankine", "rankines";
    }
}

