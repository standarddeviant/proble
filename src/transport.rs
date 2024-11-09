// use crate::transport::dfu_uuids::*;
// use crate::transport::DfuTransport;

use async_trait::async_trait;
use btleplug::api::{
    Central, CentralEvent, Characteristic, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::Adapter;
use btleplug::platform::Peripheral;
use futures::stream::StreamExt;
use futures::TryFutureExt;
use std::error::Error;

pub async fn unwrap_adapter() -> Adapter {
    let manager = btleplug::platform::Manager::new().await.unwrap();
    let adapters = manager.adapters().await.unwrap();
    adapters.into_iter().next().unwrap()
}

pub async fn scan_for_peripherals(
    adapter: &Adapter,
    // _filter_string: &str,
) -> Result<Vec<Peripheral>, Box<dyn Error>> {
    let _ = adapter.start_scan(ScanFilter::default()).await?;
    // central.start_scan(ScanFilter::default()).await?

    // TODO: clean this up...
    return Ok(adapter.peripherals().await.unwrap());
}
