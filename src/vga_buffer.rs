use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    /// Global skrivare för VGA-textbufferten.
    ///
    /// Används av våra `print!`- och `println!`-makron.
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        kolumn: 0,
        färg: ColorCode::ny(Color::Yellow, Color::Black),
        buffert: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

/// Färgpaletten som används av VGA i textläge.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// Håller reda på förgrunds- och bakgrundsfärgen för ett tecken.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    /// Skapar en färgkombination för VGA-bufferten.
    fn ny(förgrund: Color, bakgrund: Color) -> ColorCode {
        ColorCode((bakgrund as u8) << 4 | (förgrund as u8))
    }
}

/// Ett tecken som kan skrivas direkt till VGA-bufferten.
///
/// Varje tecken består av ett ASCII-tecken och en färgkod.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_tecken: u8,
    färg: ColorCode,
}

/// Antalet rader i VGA-textbufferten.
const BUFFER_HEIGHT: usize = 25;

/// Antalet kolumner i VGA-textbufferten.
const BUFFER_WIDTH: usize = 80;

/// Själva VGA-textbufferten.
///
/// Varje position innehåller ett tecken som skrivs via `Volatile`,
/// eftersom hårdvaran ligger direkt i minnesmappat I/O-minne.
#[repr(transparent)]
struct Buffer {
    tecken: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Skriver text till VGA-bufferten.
///
/// Skrivaren håller koll på aktuell kolumn, radbryter vid behov och
/// hanterar även `\n`.
pub struct Writer {
    kolumn: usize,
    färg: ColorCode,
    buffert: &'static mut Buffer,
}

impl Writer {
    /// Skriver ett ASCII-tecken till skärmen.
    ///
    /// Om vi når slutet av raden flyttar vi ner en rad först.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.ny_rad(),

            byte => {
                if self.kolumn >= BUFFER_WIDTH {
                    self.ny_rad();
                }

                let rad = BUFFER_HEIGHT - 1;
                let kolumn = self.kolumn;

                self.buffert.tecken[rad][kolumn].write(ScreenChar {
                    ascii_tecken: byte,
                    färg: self.färg,
                });

                self.kolumn += 1;
            }
        }
    }

    /// Skriver en sträng till VGA-bufferten.
    ///
    /// VGA-textläge klarar bara utskrivbara ASCII-tecken.
    /// Allt annat ersätts därför med `0xfe`.
    fn write_string(&mut self, text: &str) {
        for byte in text.bytes() {
            match byte {
                // Utskrivbart ASCII eller radbrytning.
                0x20..=0x7e | b'\n' => self.write_byte(byte),

                // VGA kan inte visa detta tecken.
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// Flyttar alla rader ett steg uppåt och tömmer sista raden.
    fn ny_rad(&mut self) {
        for rad in 1..BUFFER_HEIGHT {
            for kolumn in 0..BUFFER_WIDTH {
                let tecken = self.buffert.tecken[rad][kolumn].read();
                self.buffert.tecken[rad - 1][kolumn].write(tecken);
            }
        }

        self.rensa_rad(BUFFER_HEIGHT - 1);
        self.kolumn = 0;
    }

    /// Tömmer en rad genom att fylla den med mellanslag.
    fn rensa_rad(&mut self, rad: usize) {
        let tomt_tecken = ScreenChar {
            ascii_tecken: b' ',
            färg: self.färg,
        };

        for kolumn in 0..BUFFER_WIDTH {
            self.buffert.tecken[rad][kolumn].write(tomt_tecken);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.write_string(text);
        Ok(())
    }
}

/// Skriver formaterad text till VGA-bufferten.
///
/// Fungerar ungefär som standardbibliotekets `print!`,
/// fast vår output går direkt till VGA.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

/// Skriver formaterad text följt av en radbrytning.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Intern funktion som skickar formatterad text till vår globala skrivare.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    WRITER.lock().write_fmt(args).unwrap();
}
