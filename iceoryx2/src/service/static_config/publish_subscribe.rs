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

//! # Example
//!
//! ```
//! use iceoryx2::prelude::*;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let service_name = ServiceName::new("My/Funk/ServiceName")?;
//! let pubsub = zero_copy::Service::new(&service_name)
//!     .publish_subscribe()
//!     .typed::<u64>()
//!     .open_or_create()?;
//!
//! println!("type details:                     {:?}", pubsub.static_config().type_details());
//! println!("max publishers:                   {:?}", pubsub.static_config().max_supported_publishers());
//! println!("max subscribers:                  {:?}", pubsub.static_config().max_supported_subscribers());
//! println!("subscriber buffer size:           {:?}", pubsub.static_config().subscriber_max_buffer_size());
//! println!("history size:                     {:?}", pubsub.static_config().history_size());
//! println!("subscriber max borrowed samples:  {:?}", pubsub.static_config().subscriber_max_borrowed_samples());
//! println!("safe overflow:                    {:?}", pubsub.static_config().has_safe_overflow());
//!
//! # Ok(())
//! # }
//! ```

use super::type_details::{TypeDetails, TypeVariant};
use crate::config;
use serde::{Deserialize, Serialize};

/// The static configuration of an
/// [`MessagingPattern::PublishSubscribe`](crate::service::messaging_pattern::MessagingPattern::PublishSubscribe)
/// based service. Contains all parameters that do not change during the lifetime of a
/// [`Service`](crate::service::Service).
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct StaticConfig {
    pub(crate) max_subscribers: usize,
    pub(crate) max_publishers: usize,
    pub(crate) history_size: usize,
    pub(crate) subscriber_max_buffer_size: usize,
    pub(crate) subscriber_max_borrowed_samples: usize,
    pub(crate) enable_safe_overflow: bool,
    pub(crate) type_details: TypeDetails,
}

impl StaticConfig {
    pub(crate) fn new(config: &config::Config) -> Self {
        Self {
            max_subscribers: config.defaults.publish_subscribe.max_subscribers,
            max_publishers: config.defaults.publish_subscribe.max_publishers,
            history_size: config.defaults.publish_subscribe.publisher_history_size,
            subscriber_max_buffer_size: config
                .defaults
                .publish_subscribe
                .subscriber_max_buffer_size,
            subscriber_max_borrowed_samples: config
                .defaults
                .publish_subscribe
                .subscriber_max_borrowed_samples,
            enable_safe_overflow: config.defaults.publish_subscribe.enable_safe_overflow,
            type_details: TypeDetails {
                variant: TypeVariant::FixedSize,
                header_type_name: String::new(),
                header_size: 0,
                header_alignment: 0,
                payload_type_name: String::new(),
                payload_size: 0,
                payload_alignment: 0,
            },
        }
    }

    /// Returns the maximum supported amount of [`crate::port::publisher::Publisher`] ports
    pub fn max_supported_publishers(&self) -> usize {
        self.max_publishers
    }

    /// Returns the maximum supported amount of [`crate::port::subscriber::Subscriber`] ports
    pub fn max_supported_subscribers(&self) -> usize {
        self.max_subscribers
    }

    /// Returns the maximum history size that can be requested on connect.
    pub fn history_size(&self) -> usize {
        self.history_size
    }

    /// Returns the maximum supported buffer size for [`crate::port::subscriber::Subscriber`] port
    pub fn subscriber_max_buffer_size(&self) -> usize {
        self.subscriber_max_buffer_size
    }

    /// Returns how many [`crate::sample::Sample`] a [`crate::port::subscriber::Subscriber`] port
    /// can borrow in parallel at most.
    pub fn subscriber_max_borrowed_samples(&self) -> usize {
        self.subscriber_max_borrowed_samples
    }

    /// Returns true if the [`crate::service::Service`] safely overflows, otherwise false. Safe
    /// overflow means that the [`crate::port::publisher::Publisher`] will recycle the oldest
    /// [`crate::sample::Sample`] from the [`crate::port::subscriber::Subscriber`] when its buffer
    /// is full.
    pub fn has_safe_overflow(&self) -> bool {
        self.enable_safe_overflow
    }

    /// Returns the type details of the [`crate::service::Service`].
    pub fn type_details(&self) -> &TypeDetails {
        &self.type_details
    }
}
