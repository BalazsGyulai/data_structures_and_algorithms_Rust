use exercises::{algorithms::radix_sort::radix_sort, person::Person};
fn main() {
    let mut array: Vec<Person> = vec![
        Person {name: "a", year: 1948, month: 10, day: 31},
        Person {name: "b", year: 2901, month: 1, day: 2},
        Person {name: "c", year: 3124, month: 13, day: 19},
        Person {name: "d", year: 2012, month: 3, day: 14},
        Person {name: "e", year: 2181, month: 6, day: 2},
        Person {name: "f", year: 1294, month: 2, day: 12},
        Person {name: "g", year: 1245, month: 4, day: 8},
        Person {name: "h", year: 4871, month: 11, day: 12},
        Person {name: "i", year: 2342, month: 12, day: 31},
    ];

    radix_sort(&mut array);

    for person in array {
        println!("{:?}", person);
    }

}