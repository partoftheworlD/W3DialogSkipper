use std::{mem::zeroed, thread, time::Duration};

use windows::Win32::UI::Input::KeyboardAndMouse::{
    keybd_event, mouse_event, GetAsyncKeyState, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD,
    INPUT_MOUSE, INPUT_TYPE, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, MOUSEEVENTF_LEFTDOWN,
    MOUSEEVENTF_LEFTUP, MOUSEINPUT, VIRTUAL_KEY, VK_CAPITAL, VK_LCONTROL, VK_SPACE,
};

fn safe_click(vkey: VIRTUAL_KEY, dur: Duration) {
    unsafe {
        let mut inputs: [INPUT; 2] = zeroed();

        inputs[0].r#type = INPUT_MOUSE;
        inputs[0].Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTDOWN;

        inputs[1].r#type = INPUT_MOUSE;
        inputs[1].Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTUP;

        if (GetAsyncKeyState(vkey.0.into())) != 0 {
            SendInput(&inputs, size_of::<INPUT>() as i32);
            thread::sleep(dur);
        }
    }
}

fn safe_spam(vkey: VIRTUAL_KEY, dur: Duration, button: VIRTUAL_KEY) {
    const KEYEVENTF_KEYDOWN: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(0);
    unsafe {
        let mut inputs: [INPUT; 2] = zeroed();

        inputs[0].r#type = INPUT_KEYBOARD;
        inputs[0].Anonymous.ki.dwFlags = KEYEVENTF_KEYDOWN;
        inputs[0].Anonymous.ki.wVk = button;
        inputs[0].Anonymous.ki.wScan = button.0;

        inputs[1].r#type = INPUT_KEYBOARD;
        inputs[1].Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;
        inputs[1].Anonymous.ki.wVk = button;
        inputs[1].Anonymous.ki.wScan = button.0;

        if (GetAsyncKeyState(vkey.0.into())) != 0 {
            SendInput(&inputs, size_of::<INPUT>() as i32);
            thread::sleep(dur);
        }
    }
}

fn main() {
    const TIMEOUT: Duration = Duration::from_millis(5);
    loop {
        safe_spam(VK_LCONTROL, TIMEOUT, VK_SPACE);
        thread::sleep(TIMEOUT);
    }
}
