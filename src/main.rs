use std::{thread, time::Duration};

use windows::Win32::UI::Input::KeyboardAndMouse::{
    keybd_event, GetAsyncKeyState, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VK_CAPITAL, VK_SPACE,
};

macro_rules! SpamKey {
    ($activator: ident,$key: ident,$timeout: ident) => {
        let button = u8::try_from($key.0).unwrap();
        unsafe {
            if (GetAsyncKeyState($activator.0.into())) != 0 {
                keybd_event(button, 0, KEYEVENTF_KEYDOWN, 0);
                thread::sleep($timeout);
                keybd_event(button, 0, KEYEVENTF_KEYUP, 0);
            }
        }
    };
}

fn main() {
    const KEYEVENTF_KEYDOWN: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(0);
    const TIMEOUT: Duration = Duration::from_millis(5);
    loop {
        SpamKey!(VK_CAPITAL, VK_SPACE, TIMEOUT);
        thread::sleep(TIMEOUT);
    }
}
