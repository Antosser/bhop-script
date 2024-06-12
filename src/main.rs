use std::{thread, time::Duration};

fn main() -> anyhow::Result<()> {
    let check_interval =
        inquire::CustomType::<u64>::new("Interval between checks (ms):").prompt()?;

    let hold_period = inquire::CustomType::<u64>::new("Hold time (ms):").prompt()?;

    loop {
        thread::sleep(Duration::from_millis(check_interval));

        if inputbot::KeybdKey::SpaceKey.is_pressed() {
            inputbot::KeybdKey::SemicolonKey.press();

            thread::sleep(Duration::from_millis(hold_period));

            inputbot::KeybdKey::SemicolonKey.release();
        }
    }
}
