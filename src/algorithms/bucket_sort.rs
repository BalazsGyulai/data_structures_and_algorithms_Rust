use crate::algorithms::insertion_sort::insertion_sort;

pub fn bucket_sort(input: &Vec<f64>, k: usize)->Vec<f64> {
    let mut buckets: Vec<Vec<f64>> = vec![Vec::new(); k];
    
    for elem in input {
        let index_to_push: usize = (elem * k as f64) as usize;
        buckets[index_to_push].push(*elem);
    }

    let mut out: Vec<f64> = Vec::new();
    
    for mut bucket in buckets {
        insertion_sort(&mut bucket);
        while let Some(elem) = bucket.pop() {
            out.push(elem);
        }
    }
    
    return out;
}