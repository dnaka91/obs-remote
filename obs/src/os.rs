use crate::cstr_ptr;

pub fn cpu_usage() -> f64 {
    unsafe {
        let info = libobs_sys::os_cpu_usage_info_start();
        let usage = libobs_sys::os_cpu_usage_info_query(info);
        libobs_sys::os_cpu_usage_info_destroy(info);

        usage
    }
}

pub fn memory_usage() -> u64 {
    unsafe { libobs_sys::os_get_proc_resident_size() }
}

pub fn free_disk_space(path: &str) -> u64 {
    unsafe { libobs_sys::os_get_free_disk_space(cstr_ptr!(path)) }
}
