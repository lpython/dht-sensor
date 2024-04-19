use core::convert::Infallible;

use embedded_hal_async::delay::DelayNs;
use embedded_hal::digital::{Error, ErrorKind, ErrorType, InputPin, OutputPin, StatefulOutputPin};

const TIMEOUT_US: u8 = 100;

#[derive(Debug)]
pub enum DhtError<E: Error> {
    PinError(E),
    ChecksumMismatch,
    Timeout,
}


impl<E, P, F> From<P> for DhtError<F>
where
    E: Error,
    P: ErrorType<Error = E>,
    F: Error
{
    fn from(error:  P) -> Self {
        // DhtError::PinError(error)
        DhtError::Timeout
    }
}

// impl<E> Error for DhtError<E> {
//     fn kind(&self) -> ErrorKind { ErrorKind::Other }
// }

// impl<E> ErrorType for DhtError<E > {
//     type Error = core::convert::Infallible;
// }

// impl<E: Error> From<Infallible> for DhtError<E> {
//     fn from(_: Infallible) -> DhtError<E> {
//         unreachable!()
//     }
// }

// impl<E, I, F> From<I::Error> for DhtError<E>
// where E: Error,
//       I: InputPin<Error = F> 
// {
//     fn from(error: I::Error) -> DhtError<E> {
//         DhtError::PinError(error.into())
//     }
// }

// impl<E: Error > From<E> for DhtError<E> 
// {
//     fn from(error: E) -> DhtError<E> {
//         DhtError::PinError(error)
//     }
// }

// impl<E, F> From<F> for DhtError<E> 
// where F: embedded_hal::digital::ErrorType,
//       F::Error: Error
     
// {
//     fn from(error_type: F) -> DhtError<E> {
//         DhtError::PinError(())
//     }
// }

// impl<E> From<DhtError<E>> for DhtError<E> where E: embedded_hal::digital::ErrorType {
//     fn from(error: DhtError<E>) -> Self {
//         // Your conversion logic here
//     }
// }


pub trait Delay: DelayNs  {}
impl<T> Delay for T where T: DelayNs  {}

pub trait InputOutputPin : InputPin  + OutputPin  {}
impl<T, > InputOutputPin  for T where T: InputPin  + OutputPin {}


async fn read_bit<E: Error>(
    delay: &mut impl Delay,
    pin: &mut impl InputPin<Error = E>,
) -> Result<bool, DhtError<E>> {
    wait_until_timeout(delay, || pin.is_high()).await?;
    delay.delay_us(35).await;
    let high = pin.is_high()?;
    wait_until_timeout(delay, || pin.is_low()).await?;
    Ok(high)
}

async fn read_byte<E: Error>(delay: &mut impl Delay, pin: &mut impl InputPin<Error = E>) -> Result<u8, DhtError<E>> {
    let mut byte: u8 = 0;
    for i in 0..8 {
        let bit_mask = 1 << (7 - (i % 8));
        if read_bit(delay, pin).await? {
            byte |= bit_mask;
        }
    }
    Ok(byte)
}

pub async fn read_raw<E: Error>(
    delay: &mut impl Delay,
    pin: &mut impl InputOutputPin,
) -> Result<[u8; 4], DhtError<E>> {
    pin.set_high().ok();
    delay.delay_us(48).await;

    wait_until_timeout(delay, || pin.is_high()).await?;
    wait_until_timeout(delay, || pin.is_low()).await?;

    let mut data = [0; 4];
    for b in data.iter_mut() {
        *b = read_byte(delay, pin).await?;
    }
    let checksum = read_byte(delay, pin).await?;
    if data.iter().fold(0u8, |sum, v| sum.wrapping_add(*v)) != checksum {
        Err(DhtError::ChecksumMismatch)
    } else {
        Ok(data)
    }
}

/// Wait until the given function returns true or the timeout is reached.
async fn wait_until_timeout<E, F>(delay: &mut impl Delay, mut func: F) -> Result<(), DhtError<E>>
where
    F: FnMut() -> Result<bool, Infallible>,
    E: Error
{
    for _ in 0..TIMEOUT_US {
        if func()? {
            return Ok(());
        }
        delay.delay_us(1).await;
    }
    Err(DhtError::Timeout)
}
