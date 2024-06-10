
unit! {
    system: uom::si;
    quantity: uom::si::length;

    /// Solar radius is a unit of distance used to express the size of stars relative to the Sun.
    @sol_rad: 6.957e8; "R_sun", "Solar radius", "Solar radius";
}



#[cfg(test)]
mod tests {
    use uom::si::length::{astronomical_unit, parsec};
    use crate::units::iau::sol_rad;

    #[test]
    fn test() {
        let au = uom::si::f64::Length::new::<astronomical_unit>(1.);
        let ps = uom::si::f64::Length::new::<parsec>(1.);
        let s_r = uom::si::f64::Length::new::<sol_rad>(1.);
    }
}