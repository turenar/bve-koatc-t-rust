use crate::bve::unit::velocity::Velocity;
use crate::bve::unit::Time;
use num_traits::{cast, Num, NumCast};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Sub};

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Length<T>(pub(super) T)
where
    T: Num + Copy + NumCast + PartialOrd;
impl<T> Length<T>
where
    T: Num + Copy + NumCast + PartialOrd,
{
    pub(super) fn raw_to_meter() -> T {
        Self::thousand()
    }
    fn thousand() -> T {
        cast(1000).unwrap()
    }
    pub fn kilometers(value: T) -> Length<T> {
        Length(value * Self::thousand() * Self::thousand())
    }
    pub fn meters(value: T) -> Length<T> {
        Length(value * Self::thousand())
    }
    pub fn millimeters(value: T) -> Length<T> {
        Length(value)
    }
    pub fn as_kilometers(&self) -> T {
        self.0 / Self::thousand() / Self::thousand()
    }
    pub fn as_meters(&self) -> T {
        self.0 / Self::thousand()
    }
    pub fn as_millimeters(&self) -> T {
        self.0
    }
}
impl<T> Debug for Length<T>
where
    T: Num + Copy + NumCast + PartialOrd + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}m", self.as_meters())
    }
}
impl<'a, T, U> From<&'a Length<T>> for Length<U>
where
    T: Num + Copy + NumCast + PartialOrd,
    U: Num + Copy + NumCast + PartialOrd + TryFrom<T>,
    <U as TryFrom<T>>::Error: Debug,
{
    fn from(value: &'a Length<T>) -> Self {
        Self(value.0.try_into().unwrap())
    }
}

impl<T> Add for Length<T>
where
    T: Num + Copy + NumCast + PartialOrd,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl<T> Sub for Length<T>
where
    T: Num + Copy + NumCast + PartialOrd,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl<T> Div<Time<T>> for Length<T>
where
    T: Num + Copy + NumCast + PartialOrd,
{
    type Output = Velocity<T>;

    fn div(self, rhs: Time<T>) -> Self::Output {
        Velocity(self.0 / rhs.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construct() {
        assert_eq!(10_000, Length::meters(10).as_millimeters());
        assert_eq!(10, Length::meters(10).as_meters());
        assert_eq!(0, Length::meters(10).as_kilometers());

        assert_eq!(10_000_000, Length::kilometers(10).as_millimeters());
        assert_eq!(10_000, Length::kilometers(10).as_meters());
        assert_eq!(10, Length::kilometers(10).as_kilometers());

        assert_eq!(10_000., Length::meters(10.).as_millimeters());
        assert_eq!(10., Length::meters(10.).as_meters());
        assert_eq!(0.01, Length::meters(10.).as_kilometers());

        assert_eq!(10_000_000., Length::kilometers(10.).as_millimeters());
        assert_eq!(10_000., Length::kilometers(10.).as_meters());
        assert_eq!(10., Length::kilometers(10.).as_kilometers());

        assert_eq!(10, Length::millimeters(10).as_millimeters());
        assert_eq!(0, Length::millimeters(10).as_meters());
        assert_eq!(0, Length::millimeters(10).as_kilometers());

        assert_eq!(10., Length::millimeters(10.).as_millimeters());
        assert_eq!(0.01, Length::millimeters(10.).as_meters());
        assert_eq!(0.000_01, Length::millimeters(10.).as_kilometers());
    }

    #[test]
    fn unit() {
        assert_eq!(Length::kilometers(1), Length::millimeters(1_000_000));
        assert_eq!(Length::kilometers(1), Length::meters(1_000));
        assert_ne!(Length::kilometers(1), Length::meters(999));
        assert_eq!(Length::meters(1), Length::millimeters(1_000));
    }

    #[test]
    fn add() {
        assert_eq!(
            Length::millimeters(9),
            Length::millimeters(5) + Length::millimeters(4)
        );
        assert_eq!(
            Length::meters(1.234),
            Length::meters(1.) + Length::millimeters(234.)
        );
        assert_eq!(
            Length::meters(9_876),
            Length::meters(4876) + Length::kilometers(5)
        );
    }
    #[test]
    fn sub() {
        assert_eq!(
            Length::millimeters(1),
            Length::millimeters(5) - Length::millimeters(4)
        );
        assert_eq!(
            Length::meters(0.766),
            Length::meters(1.) - Length::millimeters(234.)
        );
        assert_eq!(
            Length::meters(-124),
            Length::meters(4876) - Length::kilometers(5)
        );
    }
}
