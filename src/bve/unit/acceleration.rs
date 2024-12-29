use crate::bve::unit::{Length, Time, Velocity};
use num_traits::{cast, Num, NumCast};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Acceleration<T>(pub(super) T)
where
    T: Num + Copy + NumCast + PartialOrd;
impl<T> Acceleration<T>
where
    T: Num + Copy + NumCast + PartialOrd,
{
    pub fn kilometer_per_second_per_second(value: T) -> Self {
        Velocity::kilometer_per_second(value) / Time::seconds(T::one())
    }
    pub fn kmps2(value: T) -> Self {
        Self::kilometer_per_second_per_second(value)
    }
    pub fn kilometer_per_second_per_hour(value: T) -> Self {
        Velocity::kilometer_per_second(value) / Time::hours(T::one())
    }
    pub fn kmpsh(value: T) -> Self {
        Self::kilometer_per_second_per_hour(value)
    }
    pub fn as_kmps2(&self) -> T {
        self.0 * Time::raw_to_second() * Time::raw_to_second()
            / Length::raw_to_meter()
            / cast(1000).unwrap()
    }
    pub fn as_kmpsh(&self) -> T {
        self.0 * cast(3600).unwrap() * Time::raw_to_second() * Time::raw_to_second()
            / Length::raw_to_meter()
            / cast(1000).unwrap()
    }
}

impl<T> Debug for Acceleration<T>
where
    T: Num + Copy + NumCast + PartialOrd + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}km/h/s", self.as_kmps2())
    }
}
impl<T> Add for Acceleration<T>
where
    T: Num + Copy + NumCast + PartialOrd,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl<T> Sub for Acceleration<T>
where
    T: Num + Copy + NumCast + PartialOrd,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl<T> Mul<Time<T>> for Acceleration<T>
where
    T: Num + Copy + NumCast + PartialOrd,
{
    type Output = Velocity<T>;

    fn mul(self, rhs: Time<T>) -> Self::Output {
        Velocity(self.0 * rhs.0)
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
}
