/*
* File: bcn2xxxx_pl011_uart.rs
* Project: RpiOS
* File Created: Saturday, 1st January 2022 4:43:21 pm
* Author: Elad Matia (elad.matia@gmail.com)
*/

use core::fmt;
use core::fmt::Arguments;

use crate::{
    bsp::device_driver::common::MMIODerefWrapper, driver, synchronization::interface::Mutex,
    synchronization::NullLock, cpu, console,
};

use tock_registers::{
    interfaces::{Writeable, Readable},
    register_bitfields, register_structs,
    registers::ReadWrite, registers::WriteOnly, registers::ReadOnly,
};

//----------------------------------------
// private stuff
//----------------------------------------

// PL011 UART registers.
//
// Descriptions taken from
// - https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf -> this is great


register_bitfields! {
    u32, // 32 bit wide

    /// Flag Register
    FR [
        /// UART busy. If this bit is set to 1, the UART is busy transmitting data. This bit remains
        /// set until the complete byte, including all stop bits, has been sent from the register.
        /// This bit is set as soon as the transmit becomes non-empty, regardless of whether
        /// the UART enabled or not. 
        BUSY OFFSET(3) NUMBITS(1) [],

        /// Receive FIFO empty. The meaning of bit depends on the state of the FEN bit in
        /// the LCR_H Register.
        /// - If the FIFO is disabled, this bit is set the receive holding register is empty.
        /// - If the FIFO is enabled, the RXFE bit is when the receive FIFO is empty.
        RXFE OFFSET(4) NUMBITS(1) [],
        
        /// Transmit FIFO full. The meaning of this depends on the state of the FEN bit in the
        /// UARTLCR_ LCRH Register.
        /// - If the FIFO is disabled, this bit is set the transmit holding register is full.
        /// - If the FIFO is enabled, the TXFF bit is set when the transmit FIFO is full.
        TXFF OFFSET(5) NUMBITS (1) [],

        /// Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the
        /// Line Control Register, LCR_H.
        /// - If the FIFO is disabled, this bit is set when the transmit holding register is empty.
        /// - If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty.
        /// - This bit does not indicate if there is data in the transmit shift register.
        TXFE OFFSET(7) NUMBITS (1) [],
    ],

    /// Integer baudrate divisor
    IBRD [
        /// The integer part of the baud rate divisor value
        IBRD_DIVINT OFFSET(0) NUMBITS(16) []
    ],

    /// Fractional Baudrate divisor
    FBRD [
        /// The fractional baudrate divisor value
        FBRD_DIVFRAC OFFSET(0) NUMBITS(6)
    ],

    /// Line control register
    LCR_H [
        /// Word length. These bits indicate the
        /// number of data bits transmitted or received
        WLEN OFFSET(5) NUMBITS(2) [
            FiveBits = 0b00,
            SixBits = 0b01,
            SevenBits = 0b10,
            EightBits = 0b11
        ],
        
        /// Enable FIFO:
        /// 0 - FIFOs are disabled (character mode), that is, the FIFOs
        /// become 1-byte-deep holding registers
        /// 1 - Transmit and receive FIFO buffers are enabled (FIFO mode)
        FEN OFFSET(4) NUMBITS(1) [
            Disabled = 0b00,
            Enabled = 0b01
        ]
    ],

    /// Control register
    CR [
        /// Receive enable. If this bit is set to 1, the receive section of the UART is enabled.
        /// Data reception occurs for either UART signals or SIR signals depending on the setting of
        /// the SIREN bit. When the UART is disabled in the middle of reception, it completes the
        /// current character before stopping.
        RXE OFFSET(9) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Transmit enable. If this bit is set to 1, the transmit section of the UART is enabled.
        /// Data transmission occurs for either UART signals, or SIR signals depending on the
        /// setting of the SIREN bit. When the UART is disabled in the middle of transmission, it
        /// completes the current character before stopping.
        TXE OFFSET(8) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// UART enable:
        ///
        /// 0 = UART is disabled. If the UART is disabled in the middle of transmission or
        /// reception, it completes the current character before stopping.
        ///
        /// 1 = The UART is enabled. Data transmission and reception occurs for either UART signals
        /// or SIR signals depending on the setting of the SIREN bit
        UARTEN OFFSET(0) NUMBITS(1) [
            /// If the UART is disabled in the middle of transmission or reception, it completes the
            /// current character before stopping.
            Disabled = 0,
            Enabled = 1
        ]
    ],

    /// Interrupt Clear Register.
    ICR [
        /// Meta field for all pending interrupts.
        ALL OFFSET(0) NUMBITS(11) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00 => DR: ReadWrite<u32>),
        (0x04 => _reserved1),
        (0x18 => FR: ReadOnly<u32, FR::Register>),
        (0x1c => _reserved2), // CHECK
        (0x24 => IBRD: WriteOnly<u32, IBRD::Register>),
        (0x28 => FBRD: WriteOnly<u32, FBRD::Register>),
        (0x2c => LCR_H: ReadWrite<u32, LCR_H::Register>),
        (0x30 => CR: WriteOnly<u32, CR::Register>),
        (0x34 => _reserved3),
        (0x44 => ICR: WriteOnly<u32, ICR::Register>),
        (0x48 => @END),
    }
}

// abtracts the register calling
type Registers = MMIODerefWrapper<RegisterBlock>;

#[derive(PartialEq)]
enum BlockingMode {
    Blocking,
    NonBlocking,
}
//----------------------------------------
// Public Definitions
//----------------------------------------

pub struct PL011UartInner {
    registers: Registers,
    chars_written: usize,
    chars_read: usize,
}

// Export the inner uart struct so panic handlers could use it even if the main uart driver crashed
pub use PL011UartInner as PanicUart;

pub struct PL011Uart {
    inner: NullLock<PL011UartInner>,
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

impl PL011UartInner {
    /// Create PL011UartInner instance
    ///
    /// # Safety
    ///
    /// - verify mmio start address
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            registers: Registers::new(mmio_start_addr),
            chars_written: 0,
            chars_read: 0,
        }
    }

    /// Set up baud rate and characteristics.
    /// Chosen values for now: 8N1, 115200 baudrate
    ///
    /// Calculation 115200 baudrate:
    /// (uart clock is set to 48MHz in config)
    /// IBRD = uart_clock_in_hertz/(16*desired_baudrate) - only integer part
    /// FBRD = integer(fractional_part*64+0.5)
    ///
    /// IBRD = 48,000,000/16*115200 = 26.041666 (26)
    /// FBRD = int(0.041666 * 64 + 0.5) = 3
    ///
    /// baudrate divider: 26 + 3/64 = 26.046, baudrate is 48,000,000/(16*26.046) = ~115180
    /// error: (115200-115180)/115200 * 100 = 0.017%
    pub fn init(&mut self) {
        // Execution can arrive here while there are still characters queued in the TX FIFO and
        // actively being sent out by the UART hardware. If the UART is turned off in this case,
        // those queued characters would be lost.
        //
        // For example, this can happen during runtime on a call to panic!(), because panic!()
        // initializes its own UART instance and calls init().
        //
        // Hence, flush first to ensure all pending characters are transmitted.
        // --overkill--

        self.flush();

        // disable uart
        self.registers.CR.set(0);

        // clear interupts
        self.registers.ICR.write(ICR::ALL::CLEAR);

        // set IBRD + FBRD and enable FIFO and 8N1
        self.registers.IBRD.write(IBRD::IBRD_DIVINT.val(26));
        self.registers.FBRD.write(FBRD::FBRD_DIVFRAC.val(3));
        self.registers
            .LCR_H.write(LCR_H::WLEN::EightBits + LCR_H::FEN::Enabled);

        // turn UART on
        self.registers.CR.write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled);
    }

    /// Write Char
    fn write_char(&mut self, c: char) {
        // wait for an empty fifo slot!
        while self.registers.FR.matches_all(FR::TXFF::SET) {
            cpu::nop();
        }

        // write
        self.registers.DR.set(c as u32);

        // increment chars_written
        self.chars_written += 1;
    }

    /// Blocks execution until the transmit FIFO is empty
    /// - The BUSY flag is enabled if the transmit FIFO is non-empty
    fn flush(&self) {
        while self.registers.FR.matches_all(FR::BUSY::SET) {
            cpu::nop();
        }
    }

    /// Read a character Blocking / Non-Blocking mode
    fn read_char(&mut self, blocking_mode: BlockingMode) -> Option<char> {
        // What if RXF is empty
        if self.registers.FR.matches_all(FR::RXFE::SET) {
            // return if non blocking mode
            if blocking_mode == BlockingMode::NonBlocking {
                return None;
            }

            // otherwise wait for a character
            while self.registers.FR.matches_all(FR::RXFE::SET) {
                cpu::nop();
            }
        }

        // read char
        let ret = self.registers.DR.get() as u8 as char;

        self.chars_read += 1;
        Some(ret)
    }
}

/// Implementing `core::fmt::Write` enables usage of the `format_args!` macros, which in turn are
/// used to implement the `kernel`'s `print!` and `println!` macros. By implementing `write_str()`,
/// we get `write_fmt()` automatically.
///
/// The function takes an `&mut self`, so it must be implemented for the inner struct.
///
/// See [`src/print.rs`].
///
/// [`src/print.rs`]: ../../print/index.html
impl fmt::Write for PL011UartInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}

impl PL011Uart {
    /// Create new instance
    ///
    /// # Safety
    ///
    /// - Provide correct MMIO start address
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            inner: NullLock::new(PL011UartInner::new(mmio_start_addr)),
        }
    }
}

// -----------------------------------------------
// Interface code
// -----------------------------------------------

impl driver::interface::DeviceDriver for PL011Uart {
    fn compatible(&self) -> &'static str {
        "BCM PL011 UART"
    }
    
    fn init(&self) -> Result<(), &'static str> {
        self.inner.lock(|inner| inner.init());
        Ok(())
    }
}

impl console::interface::Write for PL011Uart {
    fn write_char(&self, c: char) {
        self.inner.lock(|inner| inner.write_char(c));
    }

    fn write_fmt(&self, args: Arguments) -> fmt::Result {
        self.inner.lock(|inner| fmt::Write::write_fmt(inner, args))
    }

    fn flush(&self) {
        self.inner.lock(|inner| inner.flush());
    }
}

impl console::interface::Read for PL011Uart {
    fn read_char(&self) -> char {
        self.inner
            .lock(|inner| inner.read_char(BlockingMode::Blocking)
            .unwrap())
    }
    fn clear_rx(&self) {
        while self.inner
            .lock(|inner| inner.read_char(BlockingMode::NonBlocking)
            .is_some()) {}
    }
}

impl console::interface::Statistics for PL011Uart {
    fn chars_written(&self) -> usize {
        self.inner.lock(|inner|inner.chars_written)
    }
    fn chars_read(&self) -> usize {
        self.inner.lock(|inner|inner.chars_read)
    }
}
