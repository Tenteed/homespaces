use enigo::{Enigo, Key, KeyboardControllable};

pub fn create_desktop() {
    let mut enigo = Enigo::new();

    enigo.key_down(Key::Meta);
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('d'));
    enigo.key_up(Key::Meta);
    enigo.key_up(Key::Control);
}

