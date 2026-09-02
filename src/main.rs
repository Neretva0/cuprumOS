#![no_std]  // Jag använder inte Rusts standardbibliotek.
#![no_main] // Jag använder min egen entry point.

// Vi behöver PanicInfo för att kunna hantera panics
// även utan standardbiblioteket.
use core::panic::PanicInfo;

/// Min egen entry point.
///
/// Eftersom jag kör utan Rusts standardbibliotek behöver jag
/// själv ange var programmet ska börja köras.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Programmet har startat.
    // Just nu har jag inget mer att göra, så jag håller
    // processorn kvar här.
    loop {}
}

/// Hanterar panics.
///
/// Utan standardbiblioteket finns ingen färdig panic-handler,
/// så jag definierar min egen.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Om något går fel stannar programmet här.
    loop {}
}
