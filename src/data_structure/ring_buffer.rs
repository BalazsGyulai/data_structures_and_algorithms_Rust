use std::{
    alloc::{Layout, System, alloc, dealloc, handle_alloc_error, realloc}, ffi::os_str::Display, fmt::write, fs::copy, io::ErrorKind::Other, ptr,
};

pub struct RingBuffer {
    data: *mut isize,
    start: usize,
    size: usize,
    capacity: usize,
}

impl RingBuffer {
    pub fn new() -> Self {
        Self {
            data: ptr::null_mut(),
            start: 0,
            size: 0,
            capacity: 0,
        }
    }

    pub fn get(&self, index: usize) -> Option<isize> {
        if index >= self.size {
            return None;
        }
        unsafe {
            Some(self.slot(index).read())
        }
    }

    pub fn push_front(&mut self, data: isize) {
        if self.size >= self.capacity {
            self.re_size();
        }

        self.start = (self.start + self.capacity - 1) % self.capacity;

        unsafe {
            self.data.write(data);
        }

        self.size += 1;
    }

    pub fn push_back(&mut self, data: isize) {
        if self.size >= self.capacity {
            self.re_size();
        }

        unsafe {
            self.slot(self.start).write(data);
        }

        self.size += 1;
    }

    pub fn pop_front(&mut self)->Option<isize> {
        if self.size == 0 {
            return None;
        }

        unsafe {
            let first_value = self.data.read();
            self.start += 1;
            self.size -= 1;
            return Some(first_value);
        }
    }

    pub fn pop_back(&mut self)->Option<isize> {
        if self.size == 0 {
            return None;
        }

        unsafe {
            let last_value = self.data.add(self.size).read();
            
            self.size -= 1;

            return Some(last_value);
        }
    }

    pub fn insert(&mut self, index: usize, data: isize) {
        if index > self.size {
            panic!("index {} is out of bound!", index);
        }

        if self.size >= self.capacity {
            self.re_size();
        }

        unsafe {
            ptr::copy(self.data.add(index), self.data.add(index + 1), self.size - index);
            self.data.add(index).write(data);
        }
        self.size += 1;
    }

    pub fn contains(&self, data: isize)->bool {
        let mut offset: usize = 0;
        while offset < self.size && self.get(offset).unwrap() != data {
            offset += 1;
        }

        offset < self.size
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
            let new_data = alloc(new_layout) as *mut isize;

            if new_data.is_null() {
                handle_alloc_error(new_layout);
            }

            if self.capacity > 0 {
                ptr::copy_nonoverlapping(
                    self.data.add(self.start),
                    new_data,
                    self.capacity - self.start,
                );
                ptr::copy_nonoverlapping(
                    self.data,
                    new_data.add(self.start),
                    self.capacity - (self.capacity - self.size),
                );

                let old_layout = Layout::array::<isize>(self.capacity).unwrap();
                dealloc(self.data as *mut u8, old_layout);
            }

            self.data = new_data;
            self.start = 0;
            self.capacity = new_capacity;
        }
    }

    fn slot(&self, index: usize) -> *mut isize {
        unsafe { self.data.add((self.start + index) % self.capacity) }
    }
}

impl Drop for RingBuffer {
    fn drop(&mut self) {
        if self.capacity > 0 {
            let layout = Layout::array::<isize>(self.capacity).unwrap();
            unsafe {
                dealloc(self.data as *mut u8, layout);
            }
        }
    }
}

impl std::fmt::Display for RingBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for i in 0 .. self.size{
            if i > 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}", self.get(i).unwrap())?;
        }
        
        write!(f, "]")
    }
}

impl std::fmt::Debug for RingBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RingBuffer [")?;

        for i in 0..self.capacity{
            if i > 0 {
                write!(f, ", ")?;
            }

            unsafe {
                let value = self.get(i);
                write!(f, "{:?}", value)?;
            }
        }
        
        write!(f, "]")
    }
}