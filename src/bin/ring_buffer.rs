use exercises::data_structure::ring_buffer::RingBuffer;


fn main() {
    let mut ring_buffer = RingBuffer::new();

    ring_buffer.push_back(1);
    ring_buffer.push_back(2);
    ring_buffer.push_back(3);
    ring_buffer.push_back(4);
    ring_buffer.push_back(5);
    ring_buffer.push_front(6);
    println!("{}", ring_buffer);
    println!("{:?}", ring_buffer);
    
    println!("pop front {:?}", ring_buffer.pop_front());
    println!("{}", ring_buffer);
    println!("{:?}", ring_buffer);

    println!("pop back{:?}", ring_buffer.pop_back());
    println!("{}", ring_buffer);
    println!("{:?}", ring_buffer);

    println!("insert at 3 value 7");
    ring_buffer.insert(1, 7);
    println!("{}", ring_buffer);
    println!("{:?}", ring_buffer);

    println!("contains 3: {}", ring_buffer.contains(3));
}