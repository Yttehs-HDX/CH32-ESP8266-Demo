use ch32_hal::println;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    println!("Panicked: {}", info);
    loop {}
}
