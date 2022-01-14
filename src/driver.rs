/*
* File: driver.rs
* Project: RpiOS
* File Created: Sunday, 26th December 2021 4:16:20 pm
* Author: Elad Matia (elad.matia@gmail.com)
*/

/// Driver-related traits (DeviceDriver, Driver manager)
pub mod interface {
    /// Device driver trait - each driver has to implement this 
    pub trait DeviceDriver {
        /// Return a string identifying the driver
        fn compatible(&self) -> &'static str;

        /// Called by kernel on startup to initialize the driver.
        /// Devices can only be used after their driver has been initialized
        fn init(&self) -> Result<(), &'static str> {
            Ok(())
        }
    }

    /// Each BSP should implement its own device manager.
    /// Only one global instance should exist
    pub trait DeviceManager {
        /// Return a slice of references to all initialized drivers.
        fn all_device_drivers(&self) -> &[&'static (dyn DeviceDriver + Sync)];

        /// post-init driver work. useful when drivers depends on other driver to work
        fn post_device_driver_init(&self);
    }
}