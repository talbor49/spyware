use scrap::{Capturer, Display};

use std::io::ErrorKind::WouldBlock;
use crate::communication::messages::GetScreenshotResponse;
use log::{debug};
use std::time::Duration;
use std::thread;

pub fn get_screenshot_request() -> GetScreenshotResponse {
    debug!("Getting screenshot");

    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    loop {
        match capturer.frame() {
            Ok(frame) => {
                return GetScreenshotResponse {
                    buffer: frame.to_vec(),
                    width: w,
                    height: h,
                    error_info: None
                }
            },
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };
    }

}