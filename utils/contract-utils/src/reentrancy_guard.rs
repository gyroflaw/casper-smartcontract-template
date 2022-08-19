#![allow(dead_code)]

use core::convert::TryFrom;

use casper_contract::contract_api::runtime;
use casper_types::{
    bytesrepr::{self, FromBytes, ToBytes},
    ApiError, CLType, CLTyped,
};

use crate::{get_key, set_key, ContractContext, ContractStorage};

pub const REENTRANCY_GUARD_KEY_NAME: &str = "reentrancy_guard";

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Reentrancy {
    NotEntered = 0,
    Entered = 1,
}

impl CLTyped for Reentrancy {
    fn cl_type() -> casper_types::CLType {
        CLType::U8
    }
}

impl ToBytes for Reentrancy {
    fn to_bytes(&self) -> Result<alloc::vec::Vec<u8>, casper_types::bytesrepr::Error> {
        let mut buffer = bytesrepr::allocate_buffer(self)?;
        buffer.extend((*self as u8).to_bytes()?);
        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        (*self as u8).serialized_length()
    }

    fn into_bytes(self) -> Result<alloc::vec::Vec<u8>, bytesrepr::Error>
    where
        Self: Sized,
    {
        self.to_bytes()
    }
}

impl FromBytes for Reentrancy {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (result, bytes) = u8::from_bytes(bytes).unwrap();
        Ok((Reentrancy::try_from(result).unwrap(), bytes))
    }
}

impl TryFrom<u8> for Reentrancy {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Reentrancy::NotEntered),
            1 => Ok(Reentrancy::Entered),
            _ => Err(()),
        }
    }
}

impl Default for Reentrancy {
    fn default() -> Self {
        Reentrancy::NotEntered
    }
}

pub trait ReentrancyGuard<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self) {
        set_key(REENTRANCY_GUARD_KEY_NAME, Reentrancy::default());
    }

    fn set_reentrancy(&mut self) {
        self.assert_reentrancy();
        set_key(REENTRANCY_GUARD_KEY_NAME, Reentrancy::Entered);
    }

    fn clear_reentrancy(&mut self) {
        set_key(REENTRANCY_GUARD_KEY_NAME, Reentrancy::NotEntered);
    }

    fn assert_reentrancy(&self) {
        let reentrancy: Reentrancy = get_key(REENTRANCY_GUARD_KEY_NAME).unwrap();
        if !reentrancy.eq(&Reentrancy::NotEntered) {
            runtime::revert(ApiError::PermissionDenied);
        }
    }
}
