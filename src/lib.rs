#![no_std]

pub mod sensor;
pub mod address;
pub mod error;
pub mod registers;
pub mod accel;
pub mod gyro;
pub mod fifo;
pub mod clock_source;
pub mod config;
mod dmp_firmware;
mod firmware_loader;
pub mod quaternion;
pub mod gravity;
pub mod yaw_pitch_roll;
pub mod euler;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
