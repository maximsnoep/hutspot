use itertools::Itertools;

/// Calculates the average of a list of elements.
#[must_use]
#[inline]
pub fn calculate_average_f32<T>(list: impl Iterator<Item = T>) -> T
where
    T: Default + std::ops::Add<Output = T> + std::ops::Div<f32, Output = T> + std::iter::Sum<T>,
{
    let (sum, count) = list.fold((T::default(), 0.0), |(sum, count), elem| (sum + elem, count + 1.0));
    sum / count
}

/// Calculates the average of a list of elements.
#[must_use]
#[inline]
pub fn calculate_average_f64<T>(list: impl Iterator<Item = T>) -> T
where
    T: Default + std::ops::Add<Output = T> + std::ops::Div<f64, Output = T> + std::iter::Sum<T>,
{
    let (sum, count) = list.fold((T::default(), 0.0), |(sum, count), elem| (sum + elem, count + 1.0));
    sum / count
}

#[must_use]
pub fn intersection_in_sequence(elem_a: usize, elem_b: usize, sequence: &[usize]) -> bool {
    let mut sequence_copy = sequence.to_owned();
    sequence_copy.retain(|&elem| elem == elem_a || elem == elem_b);
    debug_assert!(sequence_copy.len() == 4, "{sequence_copy:?}");
    sequence_copy.dedup();
    sequence_copy.len() >= 4
}

#[must_use]
pub fn set_intersection<T: std::cmp::PartialEq + Clone>(collection_a: &[T], collection_b: &[T]) -> Vec<T> {
    let mut intesection = collection_b.to_owned();
    intesection.retain(|edge_id| collection_a.contains(edge_id));
    intesection
}

#[must_use]
pub fn wrap_pairs<T: Copy>(sequence: &[T]) -> Vec<(T, T)> {
    sequence.iter().cycle().copied().take(sequence.len() + 1).tuple_windows().collect()
}
