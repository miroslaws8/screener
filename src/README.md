# Screener

## Description
This Rust project appears to be a web page screener. It uses the headless_chrome crate to control a headless Chrome browser. The main function, capture, takes in arguments from the Args struct, which likely includes parameters such as the URL of the webpage to capture.

The function creates a new browser tab with the specified URL and dimensions. It then waits for a specific element on the page to load, as specified by the wait_for field in the Args struct.

Once the element is loaded, it captures a screenshot of the page in PNG format. The screenshot data is then written to a file named "screenshot.png". The function returns a Result type, which allows for error handling if any of these steps fail.

## Installation
`cargo build`

## Usage
`screener <url> <component>`
