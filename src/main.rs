use std::{thread, time::Duration};

use windows::Win32::UI::Input::KeyboardAndMouse::{
    keybd_event, GetAsyncKeyState, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VK_CAPITAL, VK_SPACE,
};

fn main() {
    const KEYEVENTF_KEYDOWN: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(0);
    const TIMEOUT: Duration = Duration::from_millis(5);
    let button = u8::try_from(VK_SPACE.0).unwrap();
    loop {
        // VK_CAPITAL - CAPSLOCK
        unsafe {
            if (GetAsyncKeyState(VK_CAPITAL.0.into())) != 0 {
                keybd_event(button, 0, KEYEVENTF_KEYDOWN, 0);
                thread::sleep(TIMEOUT);
                keybd_event(button, 0, KEYEVENTF_KEYUP, 0);
            }
        }
        thread::sleep(TIMEOUT);
    }
}
