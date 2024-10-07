/*
 * Copyright 2022 l1npengtul <l1npengtul@protonmail.com> / The Nokhwa Contributors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use nokhwa::{
    nokhwa_initialize,
    pixel_format::RgbFormat,
    query,
    utils::{ApiBackend, RequestedFormat, RequestedFormatType},
    CallbackCamera,
};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

fn main() {
    // only needs to be run on OSX
    nokhwa_initialize(|granted| {
        println!("User said {}", granted);
    });
    let cameras = query(ApiBackend::Auto).unwrap();
    cameras.iter().for_each(|cam| println!("{:?}", cam));

    let format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    let first_camera = cameras.first().unwrap();

    let frame_count = Arc::new(Mutex::new(0));
    let last_time = Arc::new(Mutex::new(Instant::now()));

    let frame_count_clone = Arc::clone(&frame_count);
    let last_time_clone = Arc::clone(&last_time);

    let mut threaded = CallbackCamera::new(first_camera.index().clone(), format, move |_| {
        let mut count = frame_count_clone.lock().unwrap();
        *count += 1;
        let now = Instant::now();
        let mut last = last_time_clone.lock().unwrap();
        if now.duration_since(*last) >= Duration::from_secs(1) {
            println!("FPS: {}", *count);
            *count = 0;
            *last = now;
        }
    })
    .unwrap();
    threaded.open_stream().unwrap();
    loop {
        threaded.poll_frame().unwrap();
    }
}
