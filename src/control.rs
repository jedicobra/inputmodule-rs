use rp2040_hal::rom_data::reset_to_usb_boot;

use crate::{patterns::*, State};

pub enum _CommandVals {
    _Brightness = 0x00,
    _Pattern = 0x01,
    _BootloaderReset = 0x02,
    _Sleep = 0x03,
    _Animate = 0x04,
    _Panic = 0x05,
    _Draw = 0x06,
}

pub enum PatternVals {
    _Percentage = 0x00,
    Gradient,
    DoubleGradient,
    DisplayLotus,
    ZigZag,
    FullBrightness,
    DisplayPanic,
    DisplayLotus2,
}

pub enum Command {
    Brightness(u8),
    Pattern(PatternVals),
    BootloaderReset,
    Percentage(u8),
    Sleep(bool),
    Animate(bool),
    Panic,
    Draw([u8; 39]),
    _Unknown,
}

pub fn parse_command(count: usize, buf: &[u8]) -> Option<Command> {
    if count >= 4 && buf[0] == 0x32 && buf[1] == 0xAC {
        let command = buf[2];
        let arg = buf[3];

        //let mut text: String<64> = String::new();
        //writeln!(&mut text, "Command: {command}, arg: {arg}").unwrap();
        //let _ = serial.write(text.as_bytes());

        match command {
            0x00 => Some(Command::Brightness(arg)),
            0x01 => match arg {
                0x00 => {
                    if count >= 5 {
                        Some(Command::Percentage(buf[4]))
                    } else {
                        None
                    }
                }
                0x01 => Some(Command::Pattern(PatternVals::Gradient)),
                0x02 => Some(Command::Pattern(PatternVals::DoubleGradient)),
                0x03 => Some(Command::Pattern(PatternVals::DisplayLotus)),
                0x04 => Some(Command::Pattern(PatternVals::ZigZag)),
                0x05 => Some(Command::Pattern(PatternVals::FullBrightness)),
                0x06 => Some(Command::Pattern(PatternVals::DisplayPanic)),
                0x07 => Some(Command::Pattern(PatternVals::DisplayLotus2)),
                _ => None,
            },
            0x02 => Some(Command::BootloaderReset),
            0x03 => Some(Command::Sleep(arg == 1)),
            0x04 => Some(Command::Animate(arg == 1)),
            0x05 => Some(Command::Panic),
            0x06 => {
                if count >= 3 + 39 {
                    let mut bytes = [0; 39];
                    bytes.clone_from_slice(&buf[3..3 + 39]);
                    Some(Command::Draw(bytes))
                } else {
                    None
                }
            }
            _ => None, //Some(Command::Unknown),
        }
    } else {
        None
    }
}

pub fn handle_command(command: &Command, state: &mut State, matrix: &mut Foo) {
    match command {
        Command::Brightness(br) => {
            //let _ = serial.write("Brightness".as_bytes());
            state.brightness = *br;
            matrix.set_scaling(*br).expect("failed to set scaling");
        }
        Command::Percentage(p) => {
            //let p = if count >= 5 { buf[4] } else { 100 };
            state.grid = percentage(*p as u16);
        }
        Command::Pattern(pattern) => {
            //let _ = serial.write("Pattern".as_bytes());
            match pattern {
                PatternVals::Gradient => state.grid = gradient(),
                PatternVals::DoubleGradient => state.grid = double_gradient(),
                PatternVals::DisplayLotus => state.grid = display_lotus(),
                PatternVals::ZigZag => state.grid = zigzag(),
                PatternVals::FullBrightness => full_brightness(matrix),
                PatternVals::DisplayPanic => state.grid = display_panic(),
                PatternVals::DisplayLotus2 => state.grid = display_lotus2(),
                _ => {}
            }
        }
        Command::BootloaderReset => {
            //let _ = serial.write("Bootloader Reset".as_bytes());
            reset_to_usb_boot(0, 0);
        }
        Command::Sleep(_go_sleeping) => {
            //sleep(go_sleeping, state, matrix);
        }
        Command::Animate(a) => state.animate = *a,
        Command::Panic => panic!("Ahhh"),
        Command::Draw(vals) => state.grid = draw(&vals),
        _ => {}
    }
}