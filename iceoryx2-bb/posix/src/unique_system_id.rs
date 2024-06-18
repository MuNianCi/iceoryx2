// Copyright (c) 2023 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache Software License 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
// which is available at https://opensource.org/licenses/MIT.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Generates a [`UniqueSystemId`] which is in all processes on the current system. The guarantee
//! is that no other process with the same id exists.
//! But it is possible that a process with a specific id terminates and a new process generates
//! the same id.
//!
//! # Example
//!
//! ```
//! use iceoryx2_bb_posix::unique_system_id::*;
//!
//! struct MyThing {
//!     unique_system_id: UniqueSystemId,
//! }
//!
//! impl MyThing {
//!     fn new() -> Self {
//!         Self {
//!             unique_system_id: UniqueSystemId::new().expect("Failed to create UniqueSystemId")
//!         }
//!     }
//!
//!     fn id(&self) -> u128 {
//!         self.unique_system_id.value()
//!     }
//! }
//! ```

use iceoryx2_bb_elementary::enum_gen;
use iceoryx2_bb_log::fail;
use iceoryx2_pal_concurrency_sync::iox_atomic::IoxAtomicU32;
use iceoryx2_pal_posix::posix;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::atomic::Ordering};

use crate::{
    clock::Time,
    process::{Process, ProcessId},
    semaphore::ClockType,
};

enum_gen! { UniqueSystemIdCreationError
  entry:
    FailedToAcquireTime
}

/// Creates a system wide unique id. There does not exist another process which has generated the
/// same id. There will never be another process on the same system with the same id.
/// The [`UniqueSystemId`] is generated by the processes current process id and the current system
/// time using the [`ClockType::Monotonic`].
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[repr(C)]
pub struct UniqueSystemId {
    pid: u32,
    seconds: u32,
    nanoseconds: u32,
    counter: u32,
}

impl Display for UniqueSystemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl From<u128> for UniqueSystemId {
    fn from(value: u128) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl UniqueSystemId {
    /// Creates a new system wide unique id
    pub fn new() -> Result<Self, UniqueSystemIdCreationError> {
        static COUNTER: IoxAtomicU32 = IoxAtomicU32::new(0);
        let msg = "Failed to create UniqueSystemId";
        let pid = Process::from_self().id().value() as _;
        let now = fail!(from "UniqueSystemId::new()",
                        when Time::now_with_clock(ClockType::default()),
                        with UniqueSystemIdCreationError::FailedToAcquireTime,
                        "{} since the current time could not be acquired.", msg);

        Ok(UniqueSystemId {
            pid,
            seconds: now.seconds() as u32,
            nanoseconds: now.nanoseconds(),
            counter: COUNTER.fetch_add(1, Ordering::Relaxed),
        })
    }

    /// Returns the underlying value of the new system wide unique id
    pub fn value(&self) -> u128 {
        unsafe { core::mem::transmute(*self) }
    }

    /// Returns the [`ProcessId`] which was used to create the [`UniqueSystemId`]
    pub fn pid(&self) -> ProcessId {
        ProcessId::new(self.pid as posix::pid_t)
    }

    /// Returns the [`Time`] when the [`UniqueSystemId`] was created
    pub fn creation_time(&self) -> Time {
        Time {
            clock_type: ClockType::default(),
            seconds: self.seconds as u64,
            nanoseconds: self.nanoseconds,
        }
    }
}
