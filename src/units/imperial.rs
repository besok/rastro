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
pub mod mass {
    unit! {
        system: crate::units::imperial;
        quantity: crate::units::imperial::mass;

        @stone: 6.35029318E0; "st", "stone", "stones";
    }
}
pub mod length {
    unit! {
        system: crate::units::imperial;
        quantity: crate::units::imperial::length;

        @furlong: 2.01168E2; "fur", "furlong", "furlongs";
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn smoke() {
        let v = crate::units::imperial::;
        println!("{:?}", v)
    }
}