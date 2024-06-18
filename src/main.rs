use std::{thread, time::Duration};

use windows::Win32::UI::Input::KeyboardAndMouse::{
    keybd_event, GetAsyncKeyState, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VIRTUAL_KEY, VK_CAPITAL, VK_SPACE
};

fn safe_spam(vkey: VIRTUAL_KEY, dur: Duration, button: VIRTUAL_KEY) {
    const KEYEVENTF_KEYDOWN: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(0);
    unsafe {
            if (GetAsyncKeyState(vkey.0.into())) != 0 {
                println!("vkey={} button={}", vkey.0, button.0);
                keybd_event(button.0.try_into().unwrap(), 0, KEYEVENTF_KEYDOWN, 0);
                thread::sleep(dur);
                keybd_event(button.0.try_into().unwrap(), 0, KEYEVENTF_KEYUP, 0);
            }
        }
}
fn main() {
    const TIMEOUT: Duration = Duration::from_millis(100);
    loop {
        safe_spam(VK_CAPITAL,TIMEOUT, VK_SPACE);
        thread::sleep(TIMEOUT);
    }   
}
