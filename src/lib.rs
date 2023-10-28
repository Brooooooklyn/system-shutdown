#![deny(clippy::all)]

use napi::{Error, Result, Status};
use napi_derive::napi;

#[napi]
pub fn shutdown() -> Result<()> {
  system_shutdown::shutdown().map_err(|err| Error::new(Status::GenericFailure, err.to_string()))
}

#[napi]
pub fn force_shutdown() -> Result<()> {
  system_shutdown::force_shutdown()
    .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))
}

#[napi]
pub fn reboot() -> Result<()> {
  system_shutdown::reboot().map_err(|err| Error::new(Status::GenericFailure, err.to_string()))
}

#[napi]
pub fn force_reboot() -> Result<()> {
  system_shutdown::force_reboot().map_err(|err| Error::new(Status::GenericFailure, err.to_string()))
}

#[napi]
pub fn logout() -> Result<()> {
  system_shutdown::logout().map_err(|err| Error::new(Status::GenericFailure, err.to_string()))
}

#[napi]
pub fn force_logout() -> Result<()> {
  system_shutdown::force_logout().map_err(|err| Error::new(Status::GenericFailure, err.to_string()))
}

#[napi]
pub fn sleep() -> Result<()> {
  system_shutdown::sleep().map_err(|err| Error::new(Status::GenericFailure, err.to_string()))
}

#[napi]
pub fn hibernate() -> Result<()> {
  system_shutdown::hibernate().map_err(|err| Error::new(Status::GenericFailure, err.to_string()))
}
