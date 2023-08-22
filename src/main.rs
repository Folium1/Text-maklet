use arboard;
use image;
use leptess;
use std::fs;

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

fn main() {
    let event_loop: winit::event_loop::EventLoop<()> = EventLoopBuilder::new().build();

    let hotkeys_manager: GlobalHotKeyManager = GlobalHotKeyManager::new().unwrap();

    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyD);

    hotkeys_manager.register(hotkey).unwrap();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    event_loop.run(
        move |_event: winit::event::Event<'_, ()>, _, control_flow: &mut ControlFlow| {
            *control_flow = ControlFlow::Poll;
            if let Ok(event) = global_hotkey_channel.try_recv() {
                if hotkey.id() == event.id {
                    decode();
                }
            }
        },
    )
}

fn decode() {
    let path: String = get_image();
    if path == "".to_string() {
        return;
    }
    let text: String = get_text(&path);

    let res: Result<(), std::io::Error> = fs::remove_file(path);
    if res.is_err() {
        panic!("")
    }

    set_to_clipboard(&text);
}

fn get_image() -> String {
    let mut clipboard: arboard::Clipboard = arboard::Clipboard::new().unwrap();

    let image: arboard::ImageData<'_> = match clipboard.get_image() {
        Ok(img) => img,
        Err(_) => {
            // If there is no image in the clipboard. Do nothing.
            return "".to_string();
        }
    };

    let image: image::RgbaImage = image::ImageBuffer::from_raw(
        image.width.try_into().unwrap(),
        image.height.try_into().unwrap(),
        image.bytes.into_owned(),
    )
    .unwrap();

    image.save("z.png").unwrap();
    return "z.png".to_string();
}

fn get_text(path: &str) -> String {
    let mut lt: leptess::LepTess = leptess::LepTess::new(None, "ukr+eng").unwrap();

    lt.set_variable(
        leptess::Variable::TesseditCharWhitelist,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzXVI0123456789 []{}.?!/@%#$%^&*()_-=+|\\,<>±§`~\"\'\t",
    )
    .expect("");

    let res: Result<(), leptess::leptonica::PixError> = lt.set_image(path);
    if res.is_err() {
        panic!("{}", res.err().unwrap())
    }

    return lt.get_utf8_text().unwrap();
}

fn set_to_clipboard(text: &str) {
    let mut clipboard: arboard::Clipboard = arboard::Clipboard::new().unwrap();

    let res: Result<(), arboard::Error> = clipboard.set().text(text);
    if res.is_err() {
        fs::write("logs.log", res.err().unwrap().to_string()).unwrap();
    }
}
