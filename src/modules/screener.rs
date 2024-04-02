use std::error::Error;

use headless_chrome::Browser;
use headless_chrome::protocol::cdp::Target::CreateTarget;
use headless_chrome::protocol::cdp::Page;
use crate::modules::args_parser::Args;

pub fn capture(args: Args) -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;

    let tab = browser.new_tab_with_options(CreateTarget {
        url: args.url.to_string(),
        width: Some(1980),
        height: Some(1024),
        browser_context_id: None,
        enable_begin_frame_control: None,
        new_window: None,
        background: None,
    })?;

    // Wait
    tab.wait_for_element(&args.wait_for)?;

    let _jpeg_data = tab.capture_screenshot(
        Page::CaptureScreenshotFormatOption::Png,
        None,
        None,
        true)?;

    std::fs::write("screenshot.png", &_jpeg_data)?;

    Ok(())
}