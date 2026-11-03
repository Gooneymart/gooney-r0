use std::os::raw::c_void;

extern "C" {
    pub fn sim_init(base_addr: u64) -> *mut c_void;
    pub fn sim_step(handle: *mut c_void, cycles: u64);
    pub fn sim_read_mem(handle: *mut c_void, addr: u32) -> u32;
    pub fn sim_write_mem(handle: *mut c_void, addr: u32, data: u32);
    pub fn sim_destroy(handle: *mut c_void);
}
