use alloc::alloc::*;

#[derive(Default)]
pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		alloc(layout) as *mut u8
	}
	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		dealloc(ptr, layout);
	}
}

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

// #[panic_handler]
// fn panic(_info: &core::panic::PanicInfo) -> ! {
// 	loop {
// 	}
// }
