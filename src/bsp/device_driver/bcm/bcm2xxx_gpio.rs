/*
 * File: bcm2xxx_gpio.rs
 * Project: RpiOS
 * File Created: Thursday, 30th December 2021 11:47:40 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */

use crate::{
    bsp::device_driver::common::MMIODerefWrapper, driver, synchronization::interface::Mutex,
    synchronization::NullLock,
};

use tock_registers::{
    interfaces::{ReadWriteable, Writeable},
    register_bitfields, register_structs,
    registers::ReadWrite,
};

//----------------------------------------
// private stuff
//----------------------------------------

// GPIO registers.
//
// Descriptions taken from 
// - https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf -> this is great

register_bitfields! {
    u32, // 32 bits wide

    // GPIO Function select register 1 (controls pins 14&15 among others)
    GPFSEL1 [
        // pin 14 (offset 12)
        FSEL14 OFFSET(12) NUMBITS(3) [
            // possible values for the field
            input = 0b000,
            output = 0b001,
            altFunc0 = 0b100 // uart is in mode0
        ],

        FSEL15 OFFSET(15) NUMBITS(3) [
            // possible values for the field
            input = 0b000,
            output = 0b001,
            altFunc0 = 0b100 // uart is in mode0
        ],
    ],

    // GPIO Pull up/down register
    // used in conjunction with GPPUDCLK0/1/2
    GPPUD [
        // pin up/down
        PUD OFFSET(0) NUMBITS(2) [
            off = 0b00,
            pullDown = 0b01,
            pullUp = 0b10
        ]
    ],

    // GPIO Pull Up/down clock register 0, to control pins 14&15
    GPPUDCLK0 [
        // assert clock on pin n
        PUDCLK14 OFFSET(14) NUMBITS(1) [
            noEffect = 0b0,
            assertClock = 0b1
        ],

        PUDCLK15 OFFSET(15) NUMBITS(1) [
            noEffect = 0b0,
            assertClock = 0b1
        ]
    ],

    /// GPIO Pull-up / Pull-down Register 0
    ///
    /// BCM2711 only.
    GPIO_PUP_PDN_CNTRL_REG0 [
        /// Pin 15
        GPIO_PUP_PDN_CNTRL15 OFFSET(30) NUMBITS(2) [
            NoResistor = 0b00,
            PullUp = 0b01
        ],

        /// Pin 14
        GPIO_PUP_PDN_CNTRL14 OFFSET(28) NUMBITS(2) [
            NoResistor = 0b00,
            PullUp = 0b01
        ]
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    RegisterBlock {
        (0x00 => _reserved1),
        (0x04 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0x08 => _reserved2),
        (0x94 => GPPUD: ReadWrite<u32, GPPUD::Register>),
        (0x98 => GPPUDCLK0: ReadWrite<u32, GPPUDCLK0::Register>),
        (0x9C => _reserved3),
        (0xE4 => GPIO_PUP_PDN_CNTRL_REG0: ReadWrite<u32, GPIO_PUP_PDN_CNTRL_REG0::Register>),
        (0xE8 => @END),
    }
}

// abtracts the register calling
type Registers = MMIODerefWrapper<RegisterBlock>;

//----------------------------------------
// public stuff
//----------------------------------------

pub struct GPIOInner {
    registers: Registers,
}

// Export the inner part for panic to use.
// useful when panicing and we want to log something quickly without using the standart console interface
pub use GPIOInner as PanicGPIO;

/// Repersent the GPIO Hardware.
pub struct GPIO {
    // more than possible that two or more cores will try to access the gpio,
    // so it is only logical to put a lock on it
    inner: NullLock<GPIOInner>,
}

// GPIO inner implementations
impl GPIOInner {
    /// Create an GPIOInner instance
    /// # Safety
    /// - Ensure the validity of the MMIO start address (as provided in the cheep manual)
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            registers: Registers::new(mmio_start_addr),
        }
    }

    /// Disable pull-up/down on pins 14 and 15
    #[cfg(feature = "bsp_rpi3")]
    fn disable_pud_14_15_bcm2837(&mut self) {
        use crate::cpu;
        // 1. Write to GPPUD to set the required control signal (i.e. Pull-up or Pull-Down or neither
        // to remove the current Pull-up/down)
        // 2. Wait 150 cycles – this provides the required set-up time for the control signal
        // 3. Write to GPPUDCLK0/1 to clock the control signal into the GPIO pads you wish to
        // modify – NOTE only the pads which receive a clock will be modified, all others will
        // retain their previous state.
        // 4. Wait 150 cycles – this provides the required hold time for the control signal
        // 5. Write to GPPUD to remove the control signal
        // 6. Write to GPPUDCLK0/1 to remove the clock

        // Make an educated guess for a good delay value (Sequence described in the BCM2837
        // peripherals PDF).
        //
        // - According to Wikipedia, the fastest Pi3 clocks around 1.4 GHz.
        // - The Linux 2837 GPIO driver waits 1 µs between the steps.
        //
        // So lets try to be on the safe side and default to 2000 cycles, which would equal 1 µs
        // would the CPU be clocked at 2 GHz.
        const DELAY: usize = 2000;

        // turn off pull up/down
        self.registers.GPPUD.write(GPPUD::PUD::off);
        // wait
        cpu::spin_for_cycles(DELAY);
        // assert clock on pins 14 and 15
        self.registers
            .GPPUDCLK0
            .write(GPPUDCLK0::PUDCLK14::assertClock + GPPUDCLK0::PUDCLK15::assertClock);
        // wait
        cpu::spin_for_cycles(DELAY);
        // turn off pull up/down
        self.registers.GPPUD.write(GPPUD::PUD::off);
        // write to GPPUDCLK0 to remove clock
        self.registers.GPPUDCLK0.set(0);
    }
    /// Disable pull-up/down on pins 14 and 15. - copied, not tested
    #[cfg(feature = "bsp_rpi4")]
    fn disable_pud_14_15_bcm2711(&mut self) {
        self.registers.GPIO_PUP_PDN_CNTRL_REG0.write(
            GPIO_PUP_PDN_CNTRL_REG0::GPIO_PUP_PDN_CNTRL15::PullUp
                + GPIO_PUP_PDN_CNTRL_REG0::GPIO_PUP_PDN_CNTRL14::PullUp,
        );
    }

    /// Choose alt function 0 for pins 14, 15
    /// and disaple pull up/down
    /// TX - pin 15
    /// RX - pin 14
    pub fn init_pl011_uart_pins(&mut self) {
        self.registers
            .GPFSEL1
            .modify(GPFSEL1::FSEL14::altFunc0 + GPFSEL1::FSEL15::altFunc0);

        #[cfg(feature = "bsp_rpi3")]
        self.disable_pud_14_15_bcm2837();

        #[cfg(feature = "bsp_rpi4")]
        self.disable_pud_14_15_bcm2711();
    }
}

impl GPIO {
    /// Create an instance of GPIO device driver
    /// # Safety
    /// - User must ensure validity of the mmio start address
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            inner: NullLock::new(GPIOInner::new(mmio_start_addr)),
        }
    }

    /// init gpio uart pins
    /// same as the inner method, but with wrapping lock.
    pub fn init_gpio_uart_pins(&self) {
        self.inner.lock(|inner| inner.init_pl011_uart_pins());
    }
}

// Interface code for the device driver trait (as specified in driver.rs)

impl driver::interface::DeviceDriver for GPIO {
    /// Returns identity string of the driver
    fn compatible(&self) -> &'static str {
        "GPIO Device Driver"
    }
  //  fn init(&self) -> Result<(), &'static str> {
        //self.init_gpio_uart_pins();
        //Ok(())
    //}
}
