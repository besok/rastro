use crate::time::Time;

/// The attribute is an enum that contains the different types of attributes
/// that can be used in the coordinates' system.
enum Attribute{
    Time(Time),
    Quantity(),
    EarthLocation(),
    Coordinate(),
    CartesianRepr(),
    DifferentialRepr(),
}
