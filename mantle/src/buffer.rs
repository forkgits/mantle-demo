use ffi;
use error;

use device::Device;
use MantleObject;

use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::slice;
use std::mem;

pub struct Buffer {
    memory: ffi::GR_GPU_MEMORY,
    size: usize,
}

pub struct Mapping<'a, T> {
    buffer: &'a Buffer,
    pointer: *mut T,
    size: usize,
}

impl Buffer {
    pub fn empty(device: &Arc<Device>, size: usize) -> Arc<Buffer> {
        let heap = &device.get_heaps()[0];

        let infos = ffi::GR_MEMORY_ALLOC_INFO {
            size: (heap.page_size * (1 + (size - 1) / heap.page_size)) as ffi::GR_GPU_SIZE,
            alignment: 0,
            flags: 0,
            heapCount: 1,
            heaps: [heap.id, 0, 0, 0, 0, 0, 0, 0],
            memPriority: ffi::GR_MEMORY_PRIORITY_NORMAL,
        };

        let mem = unsafe {
            let mut mem = mem::uninitialized();
            error::check_result(ffi::grAllocMemory(*device.get_id(), &infos, &mut mem)).unwrap();
            mem
        };

        Arc::new(Buffer {
            memory: mem,
            size: size,
        })
    }

    pub fn map<T>(&self) -> Mapping<T> {
        let data = unsafe {
            let mut data = mem::uninitialized();
            error::check_result(ffi::grMapMemory(self.memory, 0, &mut data)).unwrap();
            data
        };

        Mapping {
            buffer: self,
            pointer: data as *mut _,
            size: self.size,
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            error::check_result(ffi::grFreeMemory(self.memory)).unwrap();
        }
    }
}

impl<'a, T> Deref for Mapping<'a, T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts_mut(self.pointer, self.size)
        }
    }
}

impl<'a, T> DerefMut for Mapping<'a, T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.pointer, self.size)
        }
    }
}

impl<'a, T> Drop for Mapping<'a, T> {
    fn drop(&mut self) {
        unsafe {
            error::check_result(ffi::grUnmapMemory(self.buffer.memory)).unwrap();
        }
    }
}
