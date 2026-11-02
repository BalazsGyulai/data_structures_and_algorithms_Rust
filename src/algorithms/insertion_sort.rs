pub fn insertion_sort<T: PartialOrd>(array: &mut Vec<T>){

    for i in 1..array.len() {
        let mut j = i;

        while j > 0 && array[j - 1] > array[j] {
            array.swap(j - 1, j);
            j = j - 1;
        }
    }
}

pub fn insertion_sort_by_key<T>(array: &mut [T], key: fn (&T) -> usize) {
    for i in 1..array.len() {
        let mut j = i;
        while j > 0 && key(&array[j - 1]) > key(&array[j]){
            array.swap(j - 1, j);
            j -= 1;
        }
    }
}