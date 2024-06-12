pub use uom::si::f64::*;

#[cfg(test)]
mod tests{
    use uom::si::length::meter;
    use crate::units;

    #[test]
    fn smoke(){
        let l1 = units::si::Length::new::<meter>(15.0);
        println!("{}", l1.into_format_args(meter, uom::fmt::DisplayStyle::Abbreviation));
    }
}