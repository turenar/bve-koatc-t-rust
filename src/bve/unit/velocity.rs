use crate::bve::unit::acceleration::Acceleration;
use crate::bve::unit::{Length, Time};
use num_traits::{cast, Num, NumCast};
use std::ops::{Add, Div, Mul, Sub};
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Velocity<T>(pub(super) T)
where
    T: Num + Copy + NumCast + PartialOrd;
impl<T> Velocity<T>
where
    T: Num + Copy + NumCast + PartialOrd,
{
    pub(super) fn raw_to_mps(v: T) -> T {
        v * Time::<T>::raw_to_second() / Length::<T>::raw_to_meter()
    }
    pub fn kilometer_per_second(value: T) -> Self {
        Length::kilometers(value) / Time::seconds(T::one())
    }
    pub fn kmps(value: T) -> Self {
        Self::kilometer_per_second(value)
    }
    pub fn meter_per_second(value: T) -> Self {
        Length::meters(value) / Time::seconds(T::one())
    }
    pub fn mps(value: T) -> Self {
        Self::meter_per_second(value)
    }
    pub fn as_mps(&self) -> T {
        Self::raw_to_mps(self.0)
    }
    pub fn as_kmps(&self) -> T {
        self.as_mps() / cast(1000).unwrap()
    }
}

impl<T> Add for Velocity<T>
where
    T: Num + Copy + NumCast + PartialOrd,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl<T> Sub for Velocity<T>
where
    T: Num + Copy + NumCast + PartialOrd,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl<T> Mul<Time<T>> for Velocity<T>
where
    T: Num + Copy + NumCast + PartialOrd,
{
    type Output = Length<T>;

    fn mul(self, rhs: Time<T>) -> Self::Output {
        Length(self.0 * rhs.0)
    }
}
impl<T> Div<Time<T>> for Velocity<T>
where
    T: Num + Copy + NumCast + PartialOrd,
{
    type Output = Acceleration<T>;
    fn div(self, rhs: Time<T>) -> Self::Output {
        Acceleration(self.0 / rhs.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::bve::unit::neareq::nearly_equal;

    #[test]
    fn mps() {
        assert_eq!(10, Velocity::mps(10).as_mps());
        assert_eq!(0, Velocity::mps(10).as_kmps());

        assert_eq!(10., Velocity::mps(10.).as_mps());
        assert!(nearly_equal(10. / 1000., Velocity::mps(10.).as_kmps()));
    }
    #[test]
    fn kmps() {
        assert_eq!(10000, Velocity::kmps(10).as_mps());
        assert_eq!(10, Velocity::kmps(10).as_kmps());

        assert_eq!(10000., Velocity::kmps(10.).as_mps());
        assert_eq!(10., Velocity::kmps(10.).as_kmps());
    }
    #[test]
    fn add() {
        assert_eq!(Velocity::mps(12), Velocity::mps(5) + Velocity::mps(7));
        assert_eq!(Velocity::mps(8.), Velocity::mps(5.) + Velocity::mps(3.));
    }
    #[test]
    fn sub() {
        assert_eq!(Velocity::mps(2), Velocity::mps(5) - Velocity::mps(3));
        assert_eq!(Velocity::mps(-2.), Velocity::mps(5.) - Velocity::mps(7.));
    }
    #[test]
    fn multiply() {
        assert_eq!(Length::meters(14), Velocity::mps(7) * Time::seconds(2));
        assert_eq!(Length::meters(15.), Velocity::mps(7.5) * Time::seconds(2.));
        assert_eq!(
            Length::meters(15),
            Velocity::mps(5_000) * Time::milliseconds(3)
        );
    }
    #[test]
    fn divide() {
        assert_eq!(Velocity::mps(2), Length::meters(6) / Time::seconds(3));
        assert_eq!(Velocity::mps(2.), Length::meters(6.) / Time::seconds(3.));
        assert_eq!(
            Velocity::mps(2),
            Length::millimeters(6) / Time::milliseconds(3)
        );
    }
}
