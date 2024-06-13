// Copyright (c) 2024 Contributors to the Eclipse Foundation
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

use core::time::Duration;
use iceoryx2::prelude::*;

const CYCLE_TIME: Duration = Duration::from_secs(1);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service_name = ServiceName::new("Service/With/Properties")?;

    let service = zero_copy::Service::new(&service_name)
        .publish_subscribe::<u64>()
        .open_with_attributes(
            // define attributes that the service requires
            // if no attributes are defined the service accepts any attribute
            &AttributeVerifier::new()
                .require("camera_resolution", "1920x1080")
                .require_key("dds_service_mapping"),
        )?;

    let subscriber = service.subscriber().create()?;

    println!("defined service attributes: {:?}", service.attributes());

    while let Iox2Event::Tick = Iox2::wait(CYCLE_TIME) {
        while let Some(sample) = subscriber.receive()? {
            println!("received: {:?}", *sample);
        }
    }

    println!("exit");

    Ok(())
}
