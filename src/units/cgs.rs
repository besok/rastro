/// The CGS system of units is a system of units
/// of physical quantities based on the centimeter, gram, and second.
pub use uom::si::*;
ISQ!(uom::si, f64, (centimeter, gram, second, ampere, kelvin, mole, candela));
mod pressure_q {
    unit! {
        system: crate::units::cgs;
        quantity: crate::units::cgs::pressure;

        @barye: 1.0E-1; "Ba", "barye", "baryes";
    }
}

mod length_q {
    unit! {
        system: crate::units::cgs;
        quantity: crate::units::cgs::length;

        @kayser: 1.0E-1; "K", "kayser", "kaysers";
    }
}


#[cfg(test)]
mod tests {
    use uom::si::diffusion_coefficient::stokes;
    use uom::si::electric_charge::{abcoulomb, franklin};
    use uom::si::electric_current::{abampere, statampere};
    use crate::units::cgs::electric_dipole_moment::debye;
    use crate::units::cgs::energy::erg;
    use crate::units::cgs::dynamic_viscosity::poise;
    use crate::units::cgs::force::dyne;
    use crate::units::cgs::acceleration::galileo;
    use crate::units;
    use crate::units::cgs::length_q::kayser;
    use crate::units::cgs::pressure_q::barye;

    #[test]
    fn smoke() {
        let v = units::cgs::Acceleration::new::<galileo>(1.0);
        let v = units::cgs::Energy::new::<erg>(1.0);
        let v = units::cgs::Force::new::<dyne>(1.0);
        let v = units::cgs::Pressure::new::<barye>(1.0);
        let v = units::cgs::DynamicViscosity::new::<poise>(1.0);
        let v = units::cgs::DiffusionCoefficient::new::<stokes>(1.0);
        let v = units::cgs::ElectricDipoleMoment::new::<debye>(1.0);
        let v = units::cgs::ElectricCharge::new::<franklin>(1.0);
        let v = units::cgs::ElectricCharge::new::<abcoulomb>(1.0);
        let v = units::cgs::ElectricCurrent::new::<statampere>(1.0);
        let v = units::cgs::ElectricCurrent::new::<abampere>(1.0);
        let v = units::cgs::Length::new::<kayser>(1.0);
        println!("{:?}", v)
    }
}