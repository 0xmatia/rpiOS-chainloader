/*
 * File: driver.rs
 * Project: RpiOS
 * File Created: Thursday, 30th December 2021 2:49:44 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */

use crate::driver::interface;

/// Driver manager for the raspberry Pi
struct BSPDriverManager {
   device_drivers: [&'static (dyn interface::DeviceDriver + Sync); 2] 
}


/// GLOBAL, static driver manager
static BSP_DRIVER_MANAGER: BSPDriverManager = BSPDriverManager {
    device_drivers: [&super::GPIO, &super::PL011_UART]
};

pub fn driver_manager() -> &'static impl interface::DeviceManager {
    &BSP_DRIVER_MANAGER
}

// Interface code for the device manager of the raspberry pi
impl interface::DeviceManager for BSPDriverManager {
    fn all_device_drivers(&self) -> &[&'static (dyn interface::DeviceDriver + Sync)] {
        &self.device_drivers
    }

    fn post_device_driver_init(&self) {
       super::GPIO.init_gpio_uart_pins();
    }
}