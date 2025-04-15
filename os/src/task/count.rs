//! Implementation of [`SyscallCount`]

#[derive(Copy, Clone,Debug)]
#[repr(C)]
/// task SyscallCount
pub struct SyscallCount {
    /// count for syscall_id, syscall_id ranges 0..999
    count: [u8; 1000],
}

impl SyscallCount {
    /// Create a new empty counter
    pub fn init() -> Self{
        Self {
            count: [0; 1000],
        }
    }

    /// update counter of syscall specified by syscall_id
    pub fn update(&mut self, syscall_id: usize) {
        self.count[syscall_id] += 1;
    }

    /// get counter for syscall specified by syscall_id
    pub fn getcount(&self, syscall_id: usize) -> u8 {
        self.count[syscall_id]
    }
    
}
