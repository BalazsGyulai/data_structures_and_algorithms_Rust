use exercises::data_structure::dynamic_array::{DynamicArray};

fn main() {
    let mut dynamic_array = DynamicArray::new();
    println!("empty: {}", dynamic_array);
    dynamic_array.push_back(1);
    dynamic_array.push_back(2);
    dynamic_array.push_back(3);
    dynamic_array.push_front(0);
    println!("display: {}", dynamic_array);
    println!("debug: {:?}", dynamic_array);
    println!("get 0: {:?}", dynamic_array.get(0));
    let pop_front = dynamic_array.pop_front();
    println!("pop front: {:?}", pop_front);
    println!("get 0: {:?}", dynamic_array.get(0));
    dynamic_array.insert(1, 4);
    println!("insert idx(1) val4): {}", dynamic_array);
}
