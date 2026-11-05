use exercises::data_structure::simple_linked_list::SimpleLinkedList;

fn main() {
    let mut linked_list = SimpleLinkedList::new();
    println!("1. {:?}", linked_list);

    linked_list.push_front(1);
    println!("push_front: {:?}", linked_list);
    println!("{}", linked_list);

    println!("2. {:?}", linked_list.get_first());

    linked_list.push_front(2);
    println!("3. {:?}", linked_list);
    println!("{}", linked_list);

    linked_list.push_back(3);
    println!("4. {:?}", linked_list);
    println!("{}", linked_list);

    println!("pop_back: {:?}", linked_list.pop_back());
    println!("{}", linked_list);

    println!("pop_front: {:?}", linked_list.pop_front());
    println!("{}", linked_list);
}
