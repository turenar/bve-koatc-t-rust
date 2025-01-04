use crate::bve::unit::{Time, Velocity};
use num_traits::{cast, AsPrimitive, Num, NumCast};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Acceleration(/*meter per second per second*/ pub(super) f64);
impl Acceleration {
    pub fn kilometer_per_second_per_second<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Self(value.into() * 1000.)
    }
    pub fn kmps2<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Self::kilometer_per_second_per_second(value)
    }
    pub fn kilometer_per_second_per_hour<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Self(value.into() * (1000. / 3600.))
    }
    pub fn kmpsh<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Self::kilometer_per_second_per_hour(value)
    }
    pub fn as_kmps2(&self) -> f64 {
        self.0 / 1000.
    }
    pub fn as_kmpsh(&self) -> f64 {
        self.0 * (3600. / 1000.)
    }
}

impl Debug for Acceleration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}km/h/s", self.as_kmps2())
    }
}
impl Add for Acceleration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub for Acceleration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl<T> Mul<Time<T>> for Acceleration
where
    T: 'static + Num + Copy + NumCast + PartialOrd,
{
    type Output = Velocity;

    fn mul(self, rhs: Time<T>) -> Self::Output {
        Velocity(self.0 * cast::<T, f64>(rhs.0).unwrap() / 1000.)
    }
}
impl<T> Mul<Acceleration> for Time<T>
where
    T: 'static + Num + Copy + NumCast + PartialOrd,
{
    type Output = Velocity;
    fn mul(self, rhs: Acceleration) -> Self::Output {
        rhs * self
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construct() {
        assert_eq!(
            0.0,
            Acceleration::kilometer_per_second_per_hour(0.0).as_kmpsh()
        );
        assert_eq!(
            1.,
            Acceleration::kilometer_per_second_per_hour(1.0).as_kmpsh()
        );
        assert_eq!(0.5, Acceleration::kmps2(0.5).as_kmps2());
        assert_eq!(0.5, Acceleration::kmpsh(1800.).as_kmps2());
    }
    #[test]
    fn unit() {
        assert_eq!(Acceleration::kmpsh(1800.), Acceleration::kmps2(0.5));
        assert_eq!(Acceleration::kmpsh(3600.), Acceleration::kmps2(1.));
        assert_eq!(Acceleration::kmps2(1.), Acceleration::kmpsh(3600.));
    }
    #[test]
    fn add() {
        assert_eq!(
            Acceleration::kmps2(4),
            Acceleration::kmps2(3) + Acceleration::kmps2(1)
        );
        assert_eq!(
            Acceleration::kmpsh(1.5),
            Acceleration::kmpsh(1.) + Acceleration::kmpsh(0.5)
        );
    }
    #[test]
    fn sub() {
        assert_eq!(
            Acceleration::kmps2(2),
            Acceleration::kmps2(3) - Acceleration::kmps2(1)
        );
        assert_eq!(
            Acceleration::kmpsh(0.5),
            Acceleration::kmpsh(1.) - Acceleration::kmpsh(0.5)
        );
    }
    #[test]
    fn mul() {
        assert_eq!(Velocity::kmps(1), Acceleration::kmpsh(5) * Time::hours(0.2));
        assert_eq!(
            Velocity::kmps(1),
            Time::seconds(0.2) * Acceleration::kmps2(5)
        );
    }
}
