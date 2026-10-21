use std::{
    alloc::{Layout, alloc, dealloc, handle_alloc_error, realloc},
    ptr,
};

pub struct DynamicArray {
    data: *mut isize,
    size: usize,
    capacity: usize,
}

impl DynamicArray {
    pub fn new() -> Self {
        Self {
            data: ptr::null_mut(),
            size: 0,
            capacity: 0,
        }
    }

    pub fn get(&self, index: usize) -> Option<isize> {
        if index >= self.size {
            return None;
        }

        unsafe {
            return Some(self.data.add(index).read());
        }
    }

    pub fn push_back(&mut self, data: isize) {
        if self.size >= self.capacity {
            self.re_size();
        }

        unsafe {
            self.data.add(self.size).write(data);
        }

        self.size += 1;
    }

    pub fn push_front(&mut self, data: isize) {
        if self.size >= self.capacity {
            self.re_size();
        }

        unsafe {
            ptr::copy(self.data, self.data.add(1), self.size);
            self.data.write(data); // writes to the 0th index
        }

        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<isize> {
        if self.is_empty() {
            return None;
        }

        unsafe {
            let first_item = self.get(0).unwrap();
            ptr::copy(self.data.add(1), self.data, self.size - 1);
            self.size -= 1;
            return Some(first_item);
        }
    }

    pub fn pop_back(&mut self) -> Option<isize> {
        if self.is_empty() {
            return None;
        }

        let last_element = self.get(self.size - 1).unwrap();
        self.size -= 1;
        return Some(last_element);
    }

    pub fn insert(&mut self, index: usize, data: isize) {
        if index > self.size {
            panic!("index {} is out of bound!", index);
        }

        if self.size >= self.capacity {
            self.re_size();
        }

        unsafe {
            ptr::copy(
                self.data.add(index),
                self.data.add(index + 1),
                self.size - index,
            );
            self.data.add(index).write(data);
        }
        self.size += 1;
    }

    pub fn contains(&self, data: isize) -> bool {
        let mut index: usize = 0;

        while index < self.size && self.get(index).unwrap() != data {
            index += 1;
        }

        index < self.size
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn re_size(&mut self) {
        let new_capacity = if self.capacity == 0 {
            1
        } else {
            self.capacity * 2
        };
        let new_layout = Layout::array::<isize>(new_capacity).unwrap();

        unsafe {
            let ptr = if self.capacity == 0 {
                alloc(new_layout) as *mut isize
            } else {
                let old_layout = Layout::array::<isize>(self.capacity).unwrap();
                realloc(self.data as *mut u8, old_layout, new_layout.size()) as *mut isize
            };

            if ptr.is_null() {
                handle_alloc_error(new_layout);
            }

            self.data = ptr;
            self.capacity = new_capacity;
        }
    }
}

impl Drop for DynamicArray {
    fn drop(&mut self) {
        if self.capacity > 0 {
            let layout = Layout::array::<isize>(self.capacity).unwrap();
            unsafe {
                dealloc(self.data as *mut u8, layout);
            }
        }
    }
}

impl std::fmt::Debug for DynamicArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DynamicArray[")?;

        for i in 0..self.size {
            if i > 0 {
                write!(f, ", ")?;
            }

            write!(f, "{:?}", self.get(i))?;
        }

        write!(f, "]")
    }
}

impl std::fmt::Display for DynamicArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for i in 0..self.size {
            if i > 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}", self.get(i).unwrap())?;
        }

        write!(f, "]")
    }
}
