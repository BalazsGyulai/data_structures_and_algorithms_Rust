use exercises::algorithms::bucket_sort::bucket_sort;

fn main() {
    let input: Vec<f64> = vec![0.78, 0.17, 0.39, 0.26, 0.72, 0.94, 0.21, 0.12, 0.23, 0.68];

    println!("{:?}", bucket_sort(&input, 10));
}