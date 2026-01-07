use std::cell::RefCell;
use std::rc::Rc;

pub type Scope<T> = Box<T>;
pub type Ref<T> = Rc<RefCell<T>>;

pub fn new_ref<T>(value: T) -> Ref<T> {
    Rc::new(RefCell::new(value))
}

pub fn new_scope<T>(value: T) -> Scope<T> {
    Box::new(value)
}


/// Memory Test
#[cfg(windows)]
pub fn print_mem(tag: &str) {
    use windows_sys::Win32::System::ProcessStatus::*;
    use windows_sys::Win32::System::Threading::GetCurrentProcess;

    unsafe {
        let mut pmc = PROCESS_MEMORY_COUNTERS::default();
        GetProcessMemoryInfo(
            GetCurrentProcess(),
            &mut pmc,
            std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        );

        println!(
            "[MEM][{}] Private={} MB, WorkingSet={} MB",
            tag,
            pmc.PagefileUsage / 1024 / 1024,
            pmc.WorkingSetSize / 1024 / 1024
        );
    }
}