use std::alloc::{alloc, dealloc, realloc, handle_alloc_error, Layout};
use std::ptr;

/// A dynamic array that grows automatically.
pub struct DynArray<T> {
    ptr: *mut T,      // Pointer to the allocated memory.
    len: usize,       // Number of elements currently stored.
    capacity: usize,  // Total capacity of the allocated memory.
}

impl<T> DynArray<T> {
    /// Creates a new, empty dynamic array.
    pub fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    /// Creates a dynamic array with an initial capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            Self::new()
        } else {
            let layout = Layout::array::<T>(capacity).expect("Layout creation failed");
            // SAFETY: We expect the allocation to succeed (or panic via handle_alloc_error).
            let ptr = unsafe { alloc(layout) } as *mut T;
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            Self {
                ptr,
                len: 0,
                capacity,
            }
        }
    }

    /// Pushes a new element onto the end of the dynamic array.
    /// Prints the current length and capacity after the push.
    pub fn push(&mut self, elem: T) {
        if self.len == self.capacity {
            self.grow();
        }
        // SAFETY: We have ensured there is enough capacity.
        unsafe {
            self.ptr.add(self.len).write(elem);
        }
        self.len += 1;
        println!("After push: length = {}, capacity = {}", self.len, self.capacity);
    }

    /// Grows the array by doubling its capacity.
    /// Prints a message showing the capacity change.
    fn grow(&mut self) {
        let old_capacity = self.capacity;
        let new_capacity = if self.capacity == 0 { 1 } else { self.capacity * 2 };
        println!("Growing capacity from {} to {}", old_capacity, new_capacity);

        let new_layout = Layout::array::<T>(new_capacity).expect("Layout creation failed");

        let new_ptr = unsafe {
            if self.capacity == 0 {
                alloc(new_layout)
            } else {
                let old_layout = Layout::array::<T>(self.capacity).expect("Layout creation failed");
                realloc(self.ptr as *mut u8, old_layout, new_layout.size())
            }
        } as *mut T;

        if new_ptr.is_null() {
            handle_alloc_error(new_layout);
        }

        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }

    /// Pops the last element from the dynamic array.
    /// Returns `Some(T)` if there was an element, or `None` if the array is empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // SAFETY: We have decremented the length so the element at self.len is "removed".
            unsafe { Some(self.ptr.add(self.len).read()) }
        }
    }

    /// Returns an immutable reference to the element at the given index.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            // SAFETY: The index is within bounds.
            unsafe { Some(&*self.ptr.add(index)) }
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the given index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            // SAFETY: The index is within bounds.
            unsafe { Some(&mut *self.ptr.add(index)) }
        } else {
            None
        }
    }

    /// Sets the value at the given index.
    /// Panics if the index is out of bounds.
    pub fn set(&mut self, index: usize, value: T) {
        if let Some(elem) = self.get_mut(index) {
            *elem = value;
        } else {
            panic!("Index {} out of bounds", index);
        }
    }

    /// Returns an iterator over immutable references to the elements.
    pub fn iter(&self) -> DynArrayIter<T> {
        DynArrayIter {
            array: self,
            index: 0,
        }
    }
}

/// An iterator for `DynArray` that yields immutable references to its elements.
pub struct DynArrayIter<'a, T> {
    array: &'a DynArray<T>,
    index: usize,
}

impl<'a, T> Iterator for DynArrayIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len {
            let item = self.array.get(self.index);
            self.index += 1;
            item
        } else {
            None
        }
    }
}

/// Clean up the allocated memory when the `DynArray` goes out of scope.
impl<T> Drop for DynArray<T> {
    fn drop(&mut self) {
        // Drop each element in the array.
        for i in 0..self.len {
            unsafe {
                ptr::drop_in_place(self.ptr.add(i));
            }
        }
        // Deallocate the memory if any was allocated.
        if self.capacity != 0 {
            let layout = Layout::array::<T>(self.capacity).expect("Layout creation failed");
            unsafe {
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

fn main() {
    let mut arr = DynArray::new();

    // Push integers into the dynamic array.
    for i in 0..10 {
        println!("Pushing element: {}", i);
        arr.push(i);
    }

    // Print the current elements.
    println!("\nElements in DynArray:");
    for (i, elem) in arr.iter().enumerate() {
        println!("Element at index {}: {}", i, elem);
    }

    // Modify the value at index 2 using the set method.
    println!("\nModifying element at index 2 to 42 using set()");
    arr.set(2, 42);

    // Print the elements after modification.
    println!("\nElements after modification:");
    for (i, elem) in arr.iter().enumerate() {
        println!("Element at index {}: {}", i, elem);
    }

    // Pop elements one by one.
    println!("\nPopping elements:");
    while let Some(value) = arr.pop() {
        println!("Popped: {}", value);
    }
}

