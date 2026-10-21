
// unsafe {
//     // room for 4 isizes (4 * 8 bytes, aligned to 8)
//     let layout = Layout::array::<isize>(4).unwrap();
//     let ptr = alloc(layout) as *mut isize;          // alloc gives *mut u8, so cast it
//     if ptr.is_null() { handle_alloc_error(layout); }

//     ptr.add(2).write(20);                            // add() counts in ELEMENTS
//     let x = ptr.add(2).read();

//     // grow to 8 elements: realloc wants the new size in BYTES
//     let new_layout = Layout::array::<isize>(8).unwrap();
//     let ptr = realloc(ptr as *mut u8, layout, new_layout.size()) as *mut isize;
//     if ptr.is_null() { handle_alloc_error(new_layout); }

//     // shift 4 elements one slot to the right (the ranges overlap, so use copy, not copy_nonoverlapping)
//     std::ptr::copy(ptr, ptr.add(1), 4);

//     dealloc(ptr as *mut u8, new_layout);             // must be the layout it CURRENTLY has
// }