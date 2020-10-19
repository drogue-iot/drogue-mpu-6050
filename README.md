[![crates.io](https://img.shields.io/crates/v/drogue-mpu-6050.svg)](https://crates.io/crates/drogue-mpu-6050)
[![docs.rs](https://docs.rs/drogue-mpu-6050/badge.svg)](https://docs.rs/drogue-mpu-6050)
[![Matrix](https://img.shields.io/matrix/drogue-iot:matrix.org)](https://matrix.to/#/#drogue-iot:matrix.org)

# `drogue-mpu-6050`

i<sup>2</sup>c Driver for the InvenSense [MPU-6050 motion-processor](https://invensense.tdk.com/products/motion-tracking/6-axis/mpu-6050/).

## Advanced DMP

This driver can load the appropriate firmware for quaternion-based DMP processing on-chip.

## Set up

### i<sup>2</sup>c

Initialize your i<sup>2</sup>c bus per usual:

```rust
let scl3 = gpioc.pc0.into_open_drain_output(&mut gpioc.moder, &mut gpioc.otyper).into_af4(&mut gpioc.moder, &mut gpioc.afrl);
let sda3 = gpioc.pc1.into_open_drain_output(&mut gpioc.moder, &mut gpioc.otyper).into_af4(&mut gpioc.moder, &mut gpioc.afrl);
let i2c = I2c::i2c3(device.I2C3, (scl3, sda3), KiloHertz(100), clocks, &mut rcc.apb1r1);
```

### MPU driver

The MPU driver requires an `embedded-time` capable clock.

```rust
let sensor = Mpu6050::new(i2c, Address::default(), &CLOCK).unwrap();
```

Once your clock is ticking, you may then manipulate the MPU.
For ease, the `initialize_dmp()` method is provided to set up reasonable configurations and load the DMP firmware into the processor.

```rust
let sensor: &mut Mpu6050<'_, _, _> = ctx.resources.sensor;
sensor.initialize_dmp().unwrap();
```

If using the advanced on-chip DMP logic, the FIFO will contain 28-byte packets of quaternion and other data.

The first 16 bytes are quaternions, which can be constructed using the `Quaternion` class.

```rust
let len = sensor.get_fifo_count().unwrap();
if len >= 28 {
    let buf = sensor.read_fifo(&mut buf).unwrap();
    let q = Quaternion::from_bytes(&buf[0..16]).unwrap().normalize();
    ....
}
```

A quaternion may also be converted into a `Euler` or `YawPitchRoll` measurement.
