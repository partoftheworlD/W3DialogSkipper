use std::{thread, time::Duration};

use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, SendInput, INPUT, INPUT_KEYBOARD, INPUT_MOUSE, KEYBD_EVENT_FLAGS,
    KEYEVENTF_KEYUP, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, VIRTUAL_KEY, VK_LCONTROL, VK_SPACE,
};

fn send(inputs: &[INPUT], vkey: VIRTUAL_KEY, dur: Duration) {
    const SIZE: i32 = size_of::<INPUT>() as i32;
    unsafe {
        if (GetAsyncKeyState(vkey.0.into())) != 0 {
            SendInput(inputs, SIZE);
            thread::sleep(dur);
        }
    }
}

fn clicker(inputs: &mut [INPUT]) {
    inputs[0].r#type = INPUT_MOUSE;
    inputs[0].Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTDOWN;

    inputs[1].r#type = INPUT_MOUSE;
    inputs[1].Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTUP;
}

fn spam_key(button: VIRTUAL_KEY, inputs: &mut [INPUT]) {
    const KEYEVENTF_KEYDOWN: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(0);

    inputs[0].r#type = INPUT_KEYBOARD;

    inputs[0].Anonymous.ki.dwFlags = KEYEVENTF_KEYDOWN;
    inputs[0].Anonymous.ki.wVk = button;
    inputs[0].Anonymous.ki.wScan = button.0;

    inputs[1] = inputs[0];
    inputs[1].Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;
}

fn main() {
    const TIMEOUT: Duration = Duration::from_millis(5);
    let mut inputs: [INPUT; 2] = { Default::default() };

    clicker(&mut inputs);

    loop {
        send(&inputs, VK_LCONTROL, TIMEOUT);
        thread::sleep(TIMEOUT);
    }
}
