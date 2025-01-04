use crate::bve::unit::acceleration::Acceleration;
use crate::bve::unit::{Length, Time};
use num_traits::{cast, AsPrimitive, Float, Num, NumCast};
use std::ops::{Add, Div, Mul, Sub};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Velocity(/* meter per second */ pub(super) f64);
impl Velocity {
    pub fn kilometer_per_second<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Self(value.into() * 1000f64)
    }
    pub fn kmps<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Self::kilometer_per_second(value)
    }
    pub fn meter_per_second<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Self(value.into())
    }
    pub fn mps<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Self::meter_per_second(value)
    }
    pub fn as_mps(&self) -> f64 {
        self.0
    }
    pub fn as_kmps(&self) -> f64 {
        self.0 / 1000f64
    }
}

impl Add for Velocity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub for Velocity {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl<T> Mul<Time<T>> for Velocity
where
    T: 'static + Num + Copy + NumCast + PartialOrd,
{
    type Output = Length<T>;

    fn mul(self, rhs: Time<T>) -> Self::Output {
        Length(cast::<f64, T>(self.0).unwrap() * rhs.0)
    }
}
impl<T> Mul<Velocity> for Time<T>
where
    T: 'static + Num + Copy + NumCast + PartialOrd,
{
    type Output = Length<T>;

    fn mul(self, rhs: Velocity) -> Self::Output {
        rhs * self
    }
}
impl<T> Div<Time<T>> for Velocity
where
    T: 'static + Num + Copy + NumCast + PartialOrd,
{
    type Output = Acceleration;
    fn div(self, rhs: Time<T>) -> Self::Output {
        Acceleration(self.0 / cast::<T, f64>(rhs.as_seconds()).unwrap())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::bve::unit::neareq::nearly_equal;

    #[test]
    fn mps() {
        assert_eq!(10., Velocity::mps(10.).as_mps());
        assert!(nearly_equal(10. / 1000., Velocity::mps(10.).as_kmps()));
    }
    #[test]
    fn kmps() {
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

        assert_eq!(Length::meters(14), Time::seconds(2) * Velocity::mps(7),);
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
