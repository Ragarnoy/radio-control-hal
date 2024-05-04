//! # Radio Control Hardware Abstraction Layer
//!
//! This crate defines traits for radio control hardware abstraction.
//!

#![cfg_attr(not(feature = "std"), no_std)]

use radio::{Channel, Radio, Receive};

/// Error handling for vehicle radio operations
#[derive(Debug, Clone, Copy)]
pub enum RadioControlError {
    CommunicationError,
    DataError,
    HardwareError,
}

pub trait RadioController: PollChannels + PollData + Radio {}

/// Trait to poll the number of channels available on the radio
pub trait PollChannels: Channel {
    type Error: core::fmt::Debug;
    fn number_of_channels(&self) -> Result<usize, <Self as PollChannels>::Error>;
}

/// Trait to poll raw and normalized data from a specific channel
pub trait PollData: Channel {
    type RawData;
    type NormalizedData;
    type Error: core::fmt::Debug;
    
    fn poll_raw_data(&self, channel: &Self::Channel) -> Result<Self::RawData, <Self as PollData>::Error>;
    fn poll_normalized_data(&self, channel: &Self::Channel) -> Result<Self::NormalizedData, <Self as PollData>::Error>;
}

/// Implement the traits on a radio device
struct VehicleRadio<R: Receive> {
    radio: R,
}

impl<R> PollChannels for VehicleRadio<R>
    where
        R: Receive,
{
    type Error = RadioControlError;

    fn number_of_channels(&self) -> Result<usize, RadioControlError> {
        // Assuming the number of channels can be derived or is fixed
        Ok(8) // Example fixed number of channels
    }
}

impl<R> PollData for VehicleRadio<R>
    where
        R: Receive,
{
    type RawData = u16;
    type NormalizedData = f32;
    type Error = RadioControlError;

    fn poll_raw_data(&self, _channel: &usize) -> Result<u16, RadioControlError> {
        // Simulate fetching raw data from the radio
        // Example: Fetching data directly from a hardware register or buffer
        Ok(1234) // Example raw data
    }

    fn poll_normalized_data(&self, channel: &usize) -> Result<f32, RadioControlError> {
        let raw_data = self.poll_raw_data(channel)?;
        Ok(raw_data as f32 / u16::MAX as f32) // Normalizing the raw data
    }
}

impl<R> Channel for VehicleRadio<R>
    where
        R: Receive,
{
    type Channel = usize;
    type Error = RadioControlError;

    fn set_channel(&mut self, _channel: &Self::Channel) -> Result<(), Self::Error> {
        // Set the channel on the radio
        Ok(())
    }
}