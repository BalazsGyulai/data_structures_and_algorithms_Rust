use crate::person::Person;
use crate::algorithms::insertion_sort::insertion_sort_by_key;

pub fn radix_sort(array: &mut [Person]) {
    insertion_sort_by_key(array, |p| p.day);
    insertion_sort_by_key(array, |p| p.month);
    insertion_sort_by_key(array, |p| p.year);
}