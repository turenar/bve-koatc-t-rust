use num_traits::Float;

pub(crate) fn nearly_equal<F: Float>(a: F, b: F) -> bool {
    (a - b).abs() < F::epsilon()
}
