use scrap::{Capturer, Display};

use crate::communication::messages::{DisplayScreenshot, GetScreenshotResponse};
use log::debug;
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;

const FRAMES_PER_SECOND: u32 = 60;

pub fn get_screenshot_request() -> GetScreenshotResponse {
    let mut screenshots: Vec<DisplayScreenshot> = Vec::new();
    debug!("Getting screenshot");

    let one_frame = Duration::from_secs(1) / FRAMES_PER_SECOND;
    let displays = Display::all().expect("Could not get all displays");

    for display in displays {
        println!("Display id ");
        let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
        let (w, h) = (capturer.width(), capturer.height());

        loop {
            match capturer.frame() {
                Ok(frame) => screenshots.push(DisplayScreenshot {
                    buffer: frame.to_vec(),
                    width: w,
                    height: h,
                }),
                Err(error) => {
                    // TODO don't block forever
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

    GetScreenshotResponse {
        displays_screenshots: screenshots,
        error_info: None,
    }
}
