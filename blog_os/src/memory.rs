#[repr(align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    blog_os::init();

    #[cfg(not(test))]
    {
        let ptr = 0x2031b2 as *mut u8;

        // read from a code page
        unsafe {
            let _x = *ptr;
        }
        println!("read worked");

        // write to a code page
        unsafe {
            *ptr = 42;
        }
        println!("write worked");
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    blog_os::hlt_loop();
}