use genie::scx::StringKey;
use genie::{HotkeyInfo};
use genie::lang::{LangFileType};
use std::fs::File;

fn main() {
    println!("Hello, world!");
    
    // let langfile = LangFile::new();
    let lf = File::open("data/key-values-en-utf8.txt").expect("failed to open langfile");
    let lft = LangFileType::KeyValue;
    let langfile = lft.read_from(lf).expect("failed to parse langfile");

    let mut f = File::open("data/qwwerty_bewick_test.hki").expect("failed to open hki");
    let info = HotkeyInfo::from(&mut f).expect("failed to parse hki");
    println!("{:?}", info.version());

    for group in info.iter() {
        println!("-----");
        for hotkey in group.iter() {
            if hotkey.string_id < 0 {
                continue;
            }
            let sk = match StringKey::try_from(hotkey.string_id) {
                Ok(sk) => sk,
                Err(err) => {
                    println!("{:}", err);
                    continue;
                },
            };
            let action_string = match langfile.get(&sk) {
                Some(a) => a,
                None => {
                    println!("No corresponding action name for {:}", sk.to_string());
                    continue;
                }
            };
            // TODO: use crate keycode instead of this
            let key_id = match hotkey.key {
                -1 => u32::try_from(168).unwrap(),
                0 => continue,
                _ => u32::try_from(hotkey.key).unwrap(),
            };
            let key_name = match char::from_u32(key_id) {
                Some(k) => String::try_from(k).unwrap(),
                None => "Unknown".to_string(),
            };
            println!("{:} - {:} ({:})", action_string, key_name, hotkey.key);
        }
    }
}
