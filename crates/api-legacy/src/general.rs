use log::info;
use obs::hotkeys::{Hotkey, InteractionFlags, Key, KeyCombination};
use tonic::{Request, Response, Status};

use self::{general_server::General, trigger_hotkey_by_sequence_request::KeyCode};
use crate::precondition;

tonic::include_proto!("obs_remote.legacy.general");

impl From<KeyCode> for Option<obs::hotkeys::Key> {
    fn from(value: KeyCode) -> Self {
        Some(match value {
            KeyCode::Unspecified => return None,
            KeyCode::Return => obs::hotkeys::Key::Return,
            KeyCode::Enter => obs::hotkeys::Key::Enter,
            KeyCode::Escape => obs::hotkeys::Key::Escape,
            KeyCode::Tab => obs::hotkeys::Key::Tab,
            KeyCode::Backtab => obs::hotkeys::Key::Backtab,
            KeyCode::Backspace => obs::hotkeys::Key::Backspace,
            KeyCode::Insert => obs::hotkeys::Key::Insert,
            KeyCode::Delete => obs::hotkeys::Key::Delete,
            KeyCode::Pause => obs::hotkeys::Key::Pause,
            KeyCode::Print => obs::hotkeys::Key::Print,
            KeyCode::Sysreq => obs::hotkeys::Key::Sysreq,
            KeyCode::Clear => obs::hotkeys::Key::Clear,
            KeyCode::Home => obs::hotkeys::Key::Home,
            KeyCode::End => obs::hotkeys::Key::End,
            KeyCode::Left => obs::hotkeys::Key::Left,
            KeyCode::Up => obs::hotkeys::Key::Up,
            KeyCode::Right => obs::hotkeys::Key::Right,
            KeyCode::Down => obs::hotkeys::Key::Down,
            KeyCode::Pageup => obs::hotkeys::Key::Pageup,
            KeyCode::Pagedown => obs::hotkeys::Key::Pagedown,
            KeyCode::Shift => obs::hotkeys::Key::Shift,
            KeyCode::Control => obs::hotkeys::Key::Control,
            KeyCode::Meta => obs::hotkeys::Key::Meta,
            KeyCode::Alt => obs::hotkeys::Key::Alt,
            KeyCode::Altgr => obs::hotkeys::Key::Altgr,
            KeyCode::Capslock => obs::hotkeys::Key::Capslock,
            KeyCode::Numlock => obs::hotkeys::Key::Numlock,
            KeyCode::Scrolllock => obs::hotkeys::Key::Scrolllock,
            KeyCode::F1 => obs::hotkeys::Key::F1,
            KeyCode::F2 => obs::hotkeys::Key::F2,
            KeyCode::F3 => obs::hotkeys::Key::F3,
            KeyCode::F4 => obs::hotkeys::Key::F4,
            KeyCode::F5 => obs::hotkeys::Key::F5,
            KeyCode::F6 => obs::hotkeys::Key::F6,
            KeyCode::F7 => obs::hotkeys::Key::F7,
            KeyCode::F8 => obs::hotkeys::Key::F8,
            KeyCode::F9 => obs::hotkeys::Key::F9,
            KeyCode::F10 => obs::hotkeys::Key::F10,
            KeyCode::F11 => obs::hotkeys::Key::F11,
            KeyCode::F12 => obs::hotkeys::Key::F12,
            KeyCode::F13 => obs::hotkeys::Key::F13,
            KeyCode::F14 => obs::hotkeys::Key::F14,
            KeyCode::F15 => obs::hotkeys::Key::F15,
            KeyCode::F16 => obs::hotkeys::Key::F16,
            KeyCode::F17 => obs::hotkeys::Key::F17,
            KeyCode::F18 => obs::hotkeys::Key::F18,
            KeyCode::F19 => obs::hotkeys::Key::F19,
            KeyCode::F20 => obs::hotkeys::Key::F20,
            KeyCode::F21 => obs::hotkeys::Key::F21,
            KeyCode::F22 => obs::hotkeys::Key::F22,
            KeyCode::F23 => obs::hotkeys::Key::F23,
            KeyCode::F24 => obs::hotkeys::Key::F24,
            KeyCode::F25 => obs::hotkeys::Key::F25,
            KeyCode::F26 => obs::hotkeys::Key::F26,
            KeyCode::F27 => obs::hotkeys::Key::F27,
            KeyCode::F28 => obs::hotkeys::Key::F28,
            KeyCode::F29 => obs::hotkeys::Key::F29,
            KeyCode::F30 => obs::hotkeys::Key::F30,
            KeyCode::F31 => obs::hotkeys::Key::F31,
            KeyCode::F32 => obs::hotkeys::Key::F32,
            KeyCode::F33 => obs::hotkeys::Key::F33,
            KeyCode::F34 => obs::hotkeys::Key::F34,
            KeyCode::F35 => obs::hotkeys::Key::F35,
            KeyCode::Menu => obs::hotkeys::Key::Menu,
            KeyCode::HyperL => obs::hotkeys::Key::HyperL,
            KeyCode::HyperR => obs::hotkeys::Key::HyperR,
            KeyCode::Help => obs::hotkeys::Key::Help,
            KeyCode::DirectionL => obs::hotkeys::Key::DirectionL,
            KeyCode::DirectionR => obs::hotkeys::Key::DirectionR,
            KeyCode::Space => obs::hotkeys::Key::Space,
            KeyCode::Exclam => obs::hotkeys::Key::Exclam,
            KeyCode::Quotedbl => obs::hotkeys::Key::Quotedbl,
            KeyCode::Numbersign => obs::hotkeys::Key::Numbersign,
            KeyCode::Dollar => obs::hotkeys::Key::Dollar,
            KeyCode::Percent => obs::hotkeys::Key::Percent,
            KeyCode::Ampersand => obs::hotkeys::Key::Ampersand,
            KeyCode::Apostrophe => obs::hotkeys::Key::Apostrophe,
            KeyCode::Parenleft => obs::hotkeys::Key::Parenleft,
            KeyCode::Parenright => obs::hotkeys::Key::Parenright,
            KeyCode::Asterisk => obs::hotkeys::Key::Asterisk,
            KeyCode::Plus => obs::hotkeys::Key::Plus,
            KeyCode::Comma => obs::hotkeys::Key::Comma,
            KeyCode::Minus => obs::hotkeys::Key::Minus,
            KeyCode::Period => obs::hotkeys::Key::Period,
            KeyCode::Slash => obs::hotkeys::Key::Slash,
            KeyCode::KeyCode0 => obs::hotkeys::Key::Key0,
            KeyCode::KeyCode1 => obs::hotkeys::Key::Key1,
            KeyCode::KeyCode2 => obs::hotkeys::Key::Key2,
            KeyCode::KeyCode3 => obs::hotkeys::Key::Key3,
            KeyCode::KeyCode4 => obs::hotkeys::Key::Key4,
            KeyCode::KeyCode5 => obs::hotkeys::Key::Key5,
            KeyCode::KeyCode6 => obs::hotkeys::Key::Key6,
            KeyCode::KeyCode7 => obs::hotkeys::Key::Key7,
            KeyCode::KeyCode8 => obs::hotkeys::Key::Key8,
            KeyCode::KeyCode9 => obs::hotkeys::Key::Key9,
            KeyCode::Numequal => obs::hotkeys::Key::Numequal,
            KeyCode::Numasterisk => obs::hotkeys::Key::Numasterisk,
            KeyCode::Numplus => obs::hotkeys::Key::Numplus,
            KeyCode::Numcomma => obs::hotkeys::Key::Numcomma,
            KeyCode::Numminus => obs::hotkeys::Key::Numminus,
            KeyCode::Numperiod => obs::hotkeys::Key::Numperiod,
            KeyCode::Numslash => obs::hotkeys::Key::Numslash,
            KeyCode::Num0 => obs::hotkeys::Key::Num0,
            KeyCode::Num1 => obs::hotkeys::Key::Num1,
            KeyCode::Num2 => obs::hotkeys::Key::Num2,
            KeyCode::Num3 => obs::hotkeys::Key::Num3,
            KeyCode::Num4 => obs::hotkeys::Key::Num4,
            KeyCode::Num5 => obs::hotkeys::Key::Num5,
            KeyCode::Num6 => obs::hotkeys::Key::Num6,
            KeyCode::Num7 => obs::hotkeys::Key::Num7,
            KeyCode::Num8 => obs::hotkeys::Key::Num8,
            KeyCode::Num9 => obs::hotkeys::Key::Num9,
            KeyCode::Colon => obs::hotkeys::Key::Colon,
            KeyCode::Semicolon => obs::hotkeys::Key::Semicolon,
            KeyCode::Quote => obs::hotkeys::Key::Quote,
            KeyCode::Less => obs::hotkeys::Key::Less,
            KeyCode::Equal => obs::hotkeys::Key::Equal,
            KeyCode::Greater => obs::hotkeys::Key::Greater,
            KeyCode::Question => obs::hotkeys::Key::Question,
            KeyCode::At => obs::hotkeys::Key::At,
            KeyCode::A => obs::hotkeys::Key::A,
            KeyCode::B => obs::hotkeys::Key::B,
            KeyCode::C => obs::hotkeys::Key::C,
            KeyCode::D => obs::hotkeys::Key::D,
            KeyCode::E => obs::hotkeys::Key::E,
            KeyCode::F => obs::hotkeys::Key::F,
            KeyCode::G => obs::hotkeys::Key::G,
            KeyCode::H => obs::hotkeys::Key::H,
            KeyCode::I => obs::hotkeys::Key::I,
            KeyCode::J => obs::hotkeys::Key::J,
            KeyCode::K => obs::hotkeys::Key::K,
            KeyCode::L => obs::hotkeys::Key::L,
            KeyCode::M => obs::hotkeys::Key::M,
            KeyCode::N => obs::hotkeys::Key::N,
            KeyCode::O => obs::hotkeys::Key::O,
            KeyCode::P => obs::hotkeys::Key::P,
            KeyCode::Q => obs::hotkeys::Key::Q,
            KeyCode::R => obs::hotkeys::Key::R,
            KeyCode::S => obs::hotkeys::Key::S,
            KeyCode::T => obs::hotkeys::Key::T,
            KeyCode::U => obs::hotkeys::Key::U,
            KeyCode::V => obs::hotkeys::Key::V,
            KeyCode::W => obs::hotkeys::Key::W,
            KeyCode::X => obs::hotkeys::Key::X,
            KeyCode::Y => obs::hotkeys::Key::Y,
            KeyCode::Z => obs::hotkeys::Key::Z,
            KeyCode::Bracketleft => obs::hotkeys::Key::Bracketleft,
            KeyCode::Backslash => obs::hotkeys::Key::Backslash,
            KeyCode::Bracketright => obs::hotkeys::Key::Bracketright,
            KeyCode::Asciicircum => obs::hotkeys::Key::Asciicircum,
            KeyCode::Underscore => obs::hotkeys::Key::Underscore,
            KeyCode::Quoteleft => obs::hotkeys::Key::Quoteleft,
            KeyCode::Braceleft => obs::hotkeys::Key::Braceleft,
            KeyCode::Bar => obs::hotkeys::Key::Bar,
            KeyCode::Braceright => obs::hotkeys::Key::Braceright,
            KeyCode::Asciitilde => obs::hotkeys::Key::Asciitilde,
            KeyCode::Nobreakspace => obs::hotkeys::Key::Nobreakspace,
            KeyCode::Exclamdown => obs::hotkeys::Key::Exclamdown,
            KeyCode::Cent => obs::hotkeys::Key::Cent,
            KeyCode::Sterling => obs::hotkeys::Key::Sterling,
            KeyCode::Currency => obs::hotkeys::Key::Currency,
            KeyCode::Yen => obs::hotkeys::Key::Yen,
            KeyCode::Brokenbar => obs::hotkeys::Key::Brokenbar,
            KeyCode::Section => obs::hotkeys::Key::Section,
            KeyCode::Diaeresis => obs::hotkeys::Key::Diaeresis,
            KeyCode::Copyright => obs::hotkeys::Key::Copyright,
            KeyCode::Ordfeminine => obs::hotkeys::Key::Ordfeminine,
            KeyCode::Guillemotleft => obs::hotkeys::Key::Guillemotleft,
            KeyCode::Notsign => obs::hotkeys::Key::Notsign,
            KeyCode::Hyphen => obs::hotkeys::Key::Hyphen,
            KeyCode::Registered => obs::hotkeys::Key::Registered,
            KeyCode::Macron => obs::hotkeys::Key::Macron,
            KeyCode::Degree => obs::hotkeys::Key::Degree,
            KeyCode::Plusminus => obs::hotkeys::Key::Plusminus,
            KeyCode::Twosuperior => obs::hotkeys::Key::Twosuperior,
            KeyCode::Threesuperior => obs::hotkeys::Key::Threesuperior,
            KeyCode::Acute => obs::hotkeys::Key::Acute,
            KeyCode::Mu => obs::hotkeys::Key::Mu,
            KeyCode::Paragraph => obs::hotkeys::Key::Paragraph,
            KeyCode::Periodcentered => obs::hotkeys::Key::Periodcentered,
            KeyCode::Cedilla => obs::hotkeys::Key::Cedilla,
            KeyCode::Onesuperior => obs::hotkeys::Key::Onesuperior,
            KeyCode::Masculine => obs::hotkeys::Key::Masculine,
            KeyCode::Guillemotright => obs::hotkeys::Key::Guillemotright,
            KeyCode::Onequarter => obs::hotkeys::Key::Onequarter,
            KeyCode::Onehalf => obs::hotkeys::Key::Onehalf,
            KeyCode::Threequarters => obs::hotkeys::Key::Threequarters,
            KeyCode::Questiondown => obs::hotkeys::Key::Questiondown,
            KeyCode::Agrave => obs::hotkeys::Key::Agrave,
            KeyCode::Aacute => obs::hotkeys::Key::Aacute,
            KeyCode::Acircumflex => obs::hotkeys::Key::Acircumflex,
            KeyCode::Atilde => obs::hotkeys::Key::Atilde,
            KeyCode::Adiaeresis => obs::hotkeys::Key::Adiaeresis,
            KeyCode::Aring => obs::hotkeys::Key::Aring,
            KeyCode::Ae => obs::hotkeys::Key::Ae,
            KeyCode::Ccedilla => obs::hotkeys::Key::Ccedilla,
            KeyCode::Egrave => obs::hotkeys::Key::Egrave,
            KeyCode::Eacute => obs::hotkeys::Key::Eacute,
            KeyCode::Ecircumflex => obs::hotkeys::Key::Ecircumflex,
            KeyCode::Ediaeresis => obs::hotkeys::Key::Ediaeresis,
            KeyCode::Igrave => obs::hotkeys::Key::Igrave,
            KeyCode::Iacute => obs::hotkeys::Key::Iacute,
            KeyCode::Icircumflex => obs::hotkeys::Key::Icircumflex,
            KeyCode::Idiaeresis => obs::hotkeys::Key::Idiaeresis,
            KeyCode::Eth => obs::hotkeys::Key::Eth,
            KeyCode::Ntilde => obs::hotkeys::Key::Ntilde,
            KeyCode::Ograve => obs::hotkeys::Key::Ograve,
            KeyCode::Oacute => obs::hotkeys::Key::Oacute,
            KeyCode::Ocircumflex => obs::hotkeys::Key::Ocircumflex,
            KeyCode::Otilde => obs::hotkeys::Key::Otilde,
            KeyCode::Odiaeresis => obs::hotkeys::Key::Odiaeresis,
            KeyCode::Multiply => obs::hotkeys::Key::Multiply,
            KeyCode::Ooblique => obs::hotkeys::Key::Ooblique,
            KeyCode::Ugrave => obs::hotkeys::Key::Ugrave,
            KeyCode::Uacute => obs::hotkeys::Key::Uacute,
            KeyCode::Ucircumflex => obs::hotkeys::Key::Ucircumflex,
            KeyCode::Udiaeresis => obs::hotkeys::Key::Udiaeresis,
            KeyCode::Yacute => obs::hotkeys::Key::Yacute,
            KeyCode::Thorn => obs::hotkeys::Key::Thorn,
            KeyCode::Ssharp => obs::hotkeys::Key::Ssharp,
            KeyCode::Division => obs::hotkeys::Key::Division,
            KeyCode::Ydiaeresis => obs::hotkeys::Key::Ydiaeresis,
            KeyCode::MultiKey => obs::hotkeys::Key::MultiKey,
            KeyCode::Codeinput => obs::hotkeys::Key::Codeinput,
            KeyCode::Singlecandidate => obs::hotkeys::Key::Singlecandidate,
            KeyCode::Multiplecandidate => obs::hotkeys::Key::Multiplecandidate,
            KeyCode::Previouscandidate => obs::hotkeys::Key::Previouscandidate,
            KeyCode::ModeSwitch => obs::hotkeys::Key::ModeSwitch,
            KeyCode::Kanji => obs::hotkeys::Key::Kanji,
            KeyCode::Muhenkan => obs::hotkeys::Key::Muhenkan,
            KeyCode::Henkan => obs::hotkeys::Key::Henkan,
            KeyCode::Romaji => obs::hotkeys::Key::Romaji,
            KeyCode::Hiragana => obs::hotkeys::Key::Hiragana,
            KeyCode::Katakana => obs::hotkeys::Key::Katakana,
            KeyCode::HiraganaKatakana => obs::hotkeys::Key::HiraganaKatakana,
            KeyCode::Zenkaku => obs::hotkeys::Key::Zenkaku,
            KeyCode::Hankaku => obs::hotkeys::Key::Hankaku,
            KeyCode::ZenkakuHankaku => obs::hotkeys::Key::ZenkakuHankaku,
            KeyCode::Touroku => obs::hotkeys::Key::Touroku,
            KeyCode::Massyo => obs::hotkeys::Key::Massyo,
            KeyCode::KanaLock => obs::hotkeys::Key::KanaLock,
            KeyCode::KanaShift => obs::hotkeys::Key::KanaShift,
            KeyCode::EisuShift => obs::hotkeys::Key::EisuShift,
            KeyCode::EisuToggle => obs::hotkeys::Key::EisuToggle,
            KeyCode::Hangul => obs::hotkeys::Key::Hangul,
            KeyCode::HangulStart => obs::hotkeys::Key::HangulStart,
            KeyCode::HangulEnd => obs::hotkeys::Key::HangulEnd,
            KeyCode::HangulHanja => obs::hotkeys::Key::HangulHanja,
            KeyCode::HangulJamo => obs::hotkeys::Key::HangulJamo,
            KeyCode::HangulRomaja => obs::hotkeys::Key::HangulRomaja,
            KeyCode::HangulJeonja => obs::hotkeys::Key::HangulJeonja,
            KeyCode::HangulBanja => obs::hotkeys::Key::HangulBanja,
            KeyCode::HangulPrehanja => obs::hotkeys::Key::HangulPrehanja,
            KeyCode::HangulPosthanja => obs::hotkeys::Key::HangulPosthanja,
            KeyCode::HangulSpecial => obs::hotkeys::Key::HangulSpecial,
            KeyCode::DeadGrave => obs::hotkeys::Key::DeadGrave,
            KeyCode::DeadAcute => obs::hotkeys::Key::DeadAcute,
            KeyCode::DeadCircumflex => obs::hotkeys::Key::DeadCircumflex,
            KeyCode::DeadTilde => obs::hotkeys::Key::DeadTilde,
            KeyCode::DeadMacron => obs::hotkeys::Key::DeadMacron,
            KeyCode::DeadBreve => obs::hotkeys::Key::DeadBreve,
            KeyCode::DeadAbovedot => obs::hotkeys::Key::DeadAbovedot,
            KeyCode::DeadDiaeresis => obs::hotkeys::Key::DeadDiaeresis,
            KeyCode::DeadAbovering => obs::hotkeys::Key::DeadAbovering,
            KeyCode::DeadDoubleacute => obs::hotkeys::Key::DeadDoubleacute,
            KeyCode::DeadCaron => obs::hotkeys::Key::DeadCaron,
            KeyCode::DeadCedilla => obs::hotkeys::Key::DeadCedilla,
            KeyCode::DeadOgonek => obs::hotkeys::Key::DeadOgonek,
            KeyCode::DeadIota => obs::hotkeys::Key::DeadIota,
            KeyCode::DeadVoicedSound => obs::hotkeys::Key::DeadVoicedSound,
            KeyCode::DeadSemivoicedSound => obs::hotkeys::Key::DeadSemivoicedSound,
            KeyCode::DeadBelowdot => obs::hotkeys::Key::DeadBelowdot,
            KeyCode::DeadHook => obs::hotkeys::Key::DeadHook,
            KeyCode::DeadHorn => obs::hotkeys::Key::DeadHorn,
            KeyCode::Back => obs::hotkeys::Key::Back,
            KeyCode::Forward => obs::hotkeys::Key::Forward,
            KeyCode::Stop => obs::hotkeys::Key::Stop,
            KeyCode::Refresh => obs::hotkeys::Key::Refresh,
            KeyCode::Volumedown => obs::hotkeys::Key::Volumedown,
            KeyCode::Volumemute => obs::hotkeys::Key::Volumemute,
            KeyCode::Volumeup => obs::hotkeys::Key::Volumeup,
            KeyCode::Bassboost => obs::hotkeys::Key::Bassboost,
            KeyCode::Bassup => obs::hotkeys::Key::Bassup,
            KeyCode::Bassdown => obs::hotkeys::Key::Bassdown,
            KeyCode::Trebleup => obs::hotkeys::Key::Trebleup,
            KeyCode::Trebledown => obs::hotkeys::Key::Trebledown,
            KeyCode::Mediaplay => obs::hotkeys::Key::Mediaplay,
            KeyCode::Mediastop => obs::hotkeys::Key::Mediastop,
            KeyCode::Mediaprevious => obs::hotkeys::Key::Mediaprevious,
            KeyCode::Medianext => obs::hotkeys::Key::Medianext,
            KeyCode::Mediarecord => obs::hotkeys::Key::Mediarecord,
            KeyCode::Mediapause => obs::hotkeys::Key::Mediapause,
            KeyCode::Mediatoggleplaypause => obs::hotkeys::Key::Mediatoggleplaypause,
            KeyCode::Homepage => obs::hotkeys::Key::Homepage,
            KeyCode::Favorites => obs::hotkeys::Key::Favorites,
            KeyCode::Search => obs::hotkeys::Key::Search,
            KeyCode::Standby => obs::hotkeys::Key::Standby,
            KeyCode::Openurl => obs::hotkeys::Key::Openurl,
            KeyCode::Launchmail => obs::hotkeys::Key::Launchmail,
            KeyCode::Launchmedia => obs::hotkeys::Key::Launchmedia,
            KeyCode::Launch0 => obs::hotkeys::Key::Launch0,
            KeyCode::Launch1 => obs::hotkeys::Key::Launch1,
            KeyCode::Launch2 => obs::hotkeys::Key::Launch2,
            KeyCode::Launch3 => obs::hotkeys::Key::Launch3,
            KeyCode::Launch4 => obs::hotkeys::Key::Launch4,
            KeyCode::Launch5 => obs::hotkeys::Key::Launch5,
            KeyCode::Launch6 => obs::hotkeys::Key::Launch6,
            KeyCode::Launch7 => obs::hotkeys::Key::Launch7,
            KeyCode::Launch8 => obs::hotkeys::Key::Launch8,
            KeyCode::Launch9 => obs::hotkeys::Key::Launch9,
            KeyCode::Launcha => obs::hotkeys::Key::Launcha,
            KeyCode::Launchb => obs::hotkeys::Key::Launchb,
            KeyCode::Launchc => obs::hotkeys::Key::Launchc,
            KeyCode::Launchd => obs::hotkeys::Key::Launchd,
            KeyCode::Launche => obs::hotkeys::Key::Launche,
            KeyCode::Launchf => obs::hotkeys::Key::Launchf,
            KeyCode::Launchg => obs::hotkeys::Key::Launchg,
            KeyCode::Launchh => obs::hotkeys::Key::Launchh,
            KeyCode::Monbrightnessup => obs::hotkeys::Key::Monbrightnessup,
            KeyCode::Monbrightnessdown => obs::hotkeys::Key::Monbrightnessdown,
            KeyCode::Keyboardlightonoff => obs::hotkeys::Key::Keyboardlightonoff,
            KeyCode::Keyboardbrightnessup => obs::hotkeys::Key::Keyboardbrightnessup,
            KeyCode::Keyboardbrightnessdown => obs::hotkeys::Key::Keyboardbrightnessdown,
            KeyCode::Poweroff => obs::hotkeys::Key::Poweroff,
            KeyCode::Wakeup => obs::hotkeys::Key::Wakeup,
            KeyCode::Eject => obs::hotkeys::Key::Eject,
            KeyCode::Screensaver => obs::hotkeys::Key::Screensaver,
            KeyCode::Www => obs::hotkeys::Key::Www,
            KeyCode::Memo => obs::hotkeys::Key::Memo,
            KeyCode::Lightbulb => obs::hotkeys::Key::Lightbulb,
            KeyCode::Shop => obs::hotkeys::Key::Shop,
            KeyCode::History => obs::hotkeys::Key::History,
            KeyCode::Addfavorite => obs::hotkeys::Key::Addfavorite,
            KeyCode::Hotlinks => obs::hotkeys::Key::Hotlinks,
            KeyCode::Brightnessadjust => obs::hotkeys::Key::Brightnessadjust,
            KeyCode::Finance => obs::hotkeys::Key::Finance,
            KeyCode::Community => obs::hotkeys::Key::Community,
            KeyCode::Audiorewind => obs::hotkeys::Key::Audiorewind,
            KeyCode::Backforward => obs::hotkeys::Key::Backforward,
            KeyCode::Applicationleft => obs::hotkeys::Key::Applicationleft,
            KeyCode::Applicationright => obs::hotkeys::Key::Applicationright,
            KeyCode::Book => obs::hotkeys::Key::Book,
            KeyCode::Cd => obs::hotkeys::Key::Cd,
            KeyCode::Calculator => obs::hotkeys::Key::Calculator,
            KeyCode::Todolist => obs::hotkeys::Key::Todolist,
            KeyCode::Cleargrab => obs::hotkeys::Key::Cleargrab,
            KeyCode::Close => obs::hotkeys::Key::Close,
            KeyCode::Copy => obs::hotkeys::Key::Copy,
            KeyCode::Cut => obs::hotkeys::Key::Cut,
            KeyCode::Display => obs::hotkeys::Key::Display,
            KeyCode::Dos => obs::hotkeys::Key::Dos,
            KeyCode::Documents => obs::hotkeys::Key::Documents,
            KeyCode::Excel => obs::hotkeys::Key::Excel,
            KeyCode::Explorer => obs::hotkeys::Key::Explorer,
            KeyCode::Game => obs::hotkeys::Key::Game,
            KeyCode::Go => obs::hotkeys::Key::Go,
            KeyCode::Itouch => obs::hotkeys::Key::Itouch,
            KeyCode::Logoff => obs::hotkeys::Key::Logoff,
            KeyCode::Market => obs::hotkeys::Key::Market,
            KeyCode::Meeting => obs::hotkeys::Key::Meeting,
            KeyCode::Menukb => obs::hotkeys::Key::Menukb,
            KeyCode::Menupb => obs::hotkeys::Key::Menupb,
            KeyCode::Mysites => obs::hotkeys::Key::Mysites,
            KeyCode::News => obs::hotkeys::Key::News,
            KeyCode::Officehome => obs::hotkeys::Key::Officehome,
            KeyCode::Option => obs::hotkeys::Key::Option,
            KeyCode::Paste => obs::hotkeys::Key::Paste,
            KeyCode::Phone => obs::hotkeys::Key::Phone,
            KeyCode::Calendar => obs::hotkeys::Key::Calendar,
            KeyCode::Reply => obs::hotkeys::Key::Reply,
            KeyCode::Reload => obs::hotkeys::Key::Reload,
            KeyCode::Rotatewindows => obs::hotkeys::Key::Rotatewindows,
            KeyCode::Rotationpb => obs::hotkeys::Key::Rotationpb,
            KeyCode::Rotationkb => obs::hotkeys::Key::Rotationkb,
            KeyCode::Save => obs::hotkeys::Key::Save,
            KeyCode::Send => obs::hotkeys::Key::Send,
            KeyCode::Spell => obs::hotkeys::Key::Spell,
            KeyCode::Splitscreen => obs::hotkeys::Key::Splitscreen,
            KeyCode::Support => obs::hotkeys::Key::Support,
            KeyCode::Taskpane => obs::hotkeys::Key::Taskpane,
            KeyCode::Terminal => obs::hotkeys::Key::Terminal,
            KeyCode::Tools => obs::hotkeys::Key::Tools,
            KeyCode::Travel => obs::hotkeys::Key::Travel,
            KeyCode::Video => obs::hotkeys::Key::Video,
            KeyCode::Word => obs::hotkeys::Key::Word,
            KeyCode::Xfer => obs::hotkeys::Key::Xfer,
            KeyCode::Zoomin => obs::hotkeys::Key::Zoomin,
            KeyCode::Zoomout => obs::hotkeys::Key::Zoomout,
            KeyCode::Away => obs::hotkeys::Key::Away,
            KeyCode::Messenger => obs::hotkeys::Key::Messenger,
            KeyCode::Webcam => obs::hotkeys::Key::Webcam,
            KeyCode::Mailforward => obs::hotkeys::Key::Mailforward,
            KeyCode::Pictures => obs::hotkeys::Key::Pictures,
            KeyCode::Music => obs::hotkeys::Key::Music,
            KeyCode::Battery => obs::hotkeys::Key::Battery,
            KeyCode::Bluetooth => obs::hotkeys::Key::Bluetooth,
            KeyCode::Wlan => obs::hotkeys::Key::Wlan,
            KeyCode::Uwb => obs::hotkeys::Key::Uwb,
            KeyCode::Audioforward => obs::hotkeys::Key::Audioforward,
            KeyCode::Audiorepeat => obs::hotkeys::Key::Audiorepeat,
            KeyCode::Audiorandomplay => obs::hotkeys::Key::Audiorandomplay,
            KeyCode::Subtitle => obs::hotkeys::Key::Subtitle,
            KeyCode::Audiocycletrack => obs::hotkeys::Key::Audiocycletrack,
            KeyCode::Time => obs::hotkeys::Key::Time,
            KeyCode::Hibernate => obs::hotkeys::Key::Hibernate,
            KeyCode::View => obs::hotkeys::Key::View,
            KeyCode::Topmenu => obs::hotkeys::Key::Topmenu,
            KeyCode::Powerdown => obs::hotkeys::Key::Powerdown,
            KeyCode::Suspend => obs::hotkeys::Key::Suspend,
            KeyCode::Contrastadjust => obs::hotkeys::Key::Contrastadjust,
            KeyCode::Medialast => obs::hotkeys::Key::Medialast,
            KeyCode::Call => obs::hotkeys::Key::Call,
            KeyCode::Camera => obs::hotkeys::Key::Camera,
            KeyCode::Camerafocus => obs::hotkeys::Key::Camerafocus,
            KeyCode::Context1 => obs::hotkeys::Key::Context1,
            KeyCode::Context2 => obs::hotkeys::Key::Context2,
            KeyCode::Context3 => obs::hotkeys::Key::Context3,
            KeyCode::Context4 => obs::hotkeys::Key::Context4,
            KeyCode::Flip => obs::hotkeys::Key::Flip,
            KeyCode::Hangup => obs::hotkeys::Key::Hangup,
            KeyCode::No => obs::hotkeys::Key::No,
            KeyCode::Select => obs::hotkeys::Key::Select,
            KeyCode::Yes => obs::hotkeys::Key::Yes,
            KeyCode::Togglecallhangup => obs::hotkeys::Key::Togglecallhangup,
            KeyCode::Voicedial => obs::hotkeys::Key::Voicedial,
            KeyCode::Lastnumberredial => obs::hotkeys::Key::Lastnumberredial,
            KeyCode::Execute => obs::hotkeys::Key::Execute,
            KeyCode::Printer => obs::hotkeys::Key::Printer,
            KeyCode::Play => obs::hotkeys::Key::Play,
            KeyCode::Sleep => obs::hotkeys::Key::Sleep,
            KeyCode::Zoom => obs::hotkeys::Key::Zoom,
            KeyCode::Cancel => obs::hotkeys::Key::Cancel,
            KeyCode::Mouse1 => obs::hotkeys::Key::Mouse1,
            KeyCode::Mouse2 => obs::hotkeys::Key::Mouse2,
            KeyCode::Mouse3 => obs::hotkeys::Key::Mouse3,
            KeyCode::Mouse4 => obs::hotkeys::Key::Mouse4,
            KeyCode::Mouse5 => obs::hotkeys::Key::Mouse5,
            KeyCode::Mouse6 => obs::hotkeys::Key::Mouse6,
            KeyCode::Mouse7 => obs::hotkeys::Key::Mouse7,
            KeyCode::Mouse8 => obs::hotkeys::Key::Mouse8,
            KeyCode::Mouse9 => obs::hotkeys::Key::Mouse9,
            KeyCode::Mouse10 => obs::hotkeys::Key::Mouse10,
            KeyCode::Mouse11 => obs::hotkeys::Key::Mouse11,
            KeyCode::Mouse12 => obs::hotkeys::Key::Mouse12,
            KeyCode::Mouse13 => obs::hotkeys::Key::Mouse13,
            KeyCode::Mouse14 => obs::hotkeys::Key::Mouse14,
            KeyCode::Mouse15 => obs::hotkeys::Key::Mouse15,
            KeyCode::Mouse16 => obs::hotkeys::Key::Mouse16,
            KeyCode::Mouse17 => obs::hotkeys::Key::Mouse17,
            KeyCode::Mouse18 => obs::hotkeys::Key::Mouse18,
            KeyCode::Mouse19 => obs::hotkeys::Key::Mouse19,
            KeyCode::Mouse20 => obs::hotkeys::Key::Mouse20,
            KeyCode::Mouse21 => obs::hotkeys::Key::Mouse21,
            KeyCode::Mouse22 => obs::hotkeys::Key::Mouse22,
            KeyCode::Mouse23 => obs::hotkeys::Key::Mouse23,
            KeyCode::Mouse24 => obs::hotkeys::Key::Mouse24,
            KeyCode::Mouse25 => obs::hotkeys::Key::Mouse25,
            KeyCode::Mouse26 => obs::hotkeys::Key::Mouse26,
            KeyCode::Mouse27 => obs::hotkeys::Key::Mouse27,
            KeyCode::Mouse28 => obs::hotkeys::Key::Mouse28,
            KeyCode::Mouse29 => obs::hotkeys::Key::Mouse29,
            KeyCode::BackslashRt102 => obs::hotkeys::Key::BackslashRt102,
            KeyCode::Open => obs::hotkeys::Key::Open,
            KeyCode::Find => obs::hotkeys::Key::Find,
            KeyCode::Redo => obs::hotkeys::Key::Redo,
            KeyCode::Undo => obs::hotkeys::Key::Undo,
            KeyCode::Front => obs::hotkeys::Key::Front,
            KeyCode::Props => obs::hotkeys::Key::Props,
            KeyCode::VkCancel => obs::hotkeys::Key::VkCancel,
            KeyCode::KeyCode0x07 => obs::hotkeys::Key::Key0x07,
            KeyCode::KeyCode0x0a => obs::hotkeys::Key::Key0x0a,
            KeyCode::KeyCode0x0b => obs::hotkeys::Key::Key0x0b,
            KeyCode::KeyCode0x0e => obs::hotkeys::Key::Key0x0e,
            KeyCode::KeyCode0x0f => obs::hotkeys::Key::Key0x0f,
            KeyCode::KeyCode0x16 => obs::hotkeys::Key::Key0x16,
            KeyCode::VkJunja => obs::hotkeys::Key::VkJunja,
            KeyCode::VkFinal => obs::hotkeys::Key::VkFinal,
            KeyCode::KeyCode0x1a => obs::hotkeys::Key::Key0x1a,
            KeyCode::VkAccept => obs::hotkeys::Key::VkAccept,
            KeyCode::VkModechange => obs::hotkeys::Key::VkModechange,
            KeyCode::VkSelect => obs::hotkeys::Key::VkSelect,
            KeyCode::VkPrint => obs::hotkeys::Key::VkPrint,
            KeyCode::VkExecute => obs::hotkeys::Key::VkExecute,
            KeyCode::VkHelp => obs::hotkeys::Key::VkHelp,
            KeyCode::KeyCode0x30 => obs::hotkeys::Key::Key0x30,
            KeyCode::KeyCode0x31 => obs::hotkeys::Key::Key0x31,
            KeyCode::KeyCode0x32 => obs::hotkeys::Key::Key0x32,
            KeyCode::KeyCode0x33 => obs::hotkeys::Key::Key0x33,
            KeyCode::KeyCode0x34 => obs::hotkeys::Key::Key0x34,
            KeyCode::KeyCode0x35 => obs::hotkeys::Key::Key0x35,
            KeyCode::KeyCode0x36 => obs::hotkeys::Key::Key0x36,
            KeyCode::KeyCode0x37 => obs::hotkeys::Key::Key0x37,
            KeyCode::KeyCode0x38 => obs::hotkeys::Key::Key0x38,
            KeyCode::KeyCode0x39 => obs::hotkeys::Key::Key0x39,
            KeyCode::KeyCode0x3a => obs::hotkeys::Key::Key0x3a,
            KeyCode::KeyCode0x3b => obs::hotkeys::Key::Key0x3b,
            KeyCode::KeyCode0x3c => obs::hotkeys::Key::Key0x3c,
            KeyCode::KeyCode0x3d => obs::hotkeys::Key::Key0x3d,
            KeyCode::KeyCode0x3e => obs::hotkeys::Key::Key0x3e,
            KeyCode::KeyCode0x3f => obs::hotkeys::Key::Key0x3f,
            KeyCode::KeyCode0x40 => obs::hotkeys::Key::Key0x40,
            KeyCode::KeyCode0x41 => obs::hotkeys::Key::Key0x41,
            KeyCode::KeyCode0x42 => obs::hotkeys::Key::Key0x42,
            KeyCode::KeyCode0x43 => obs::hotkeys::Key::Key0x43,
            KeyCode::KeyCode0x44 => obs::hotkeys::Key::Key0x44,
            KeyCode::KeyCode0x45 => obs::hotkeys::Key::Key0x45,
            KeyCode::KeyCode0x46 => obs::hotkeys::Key::Key0x46,
            KeyCode::KeyCode0x47 => obs::hotkeys::Key::Key0x47,
            KeyCode::KeyCode0x48 => obs::hotkeys::Key::Key0x48,
            KeyCode::KeyCode0x49 => obs::hotkeys::Key::Key0x49,
            KeyCode::KeyCode0x4a => obs::hotkeys::Key::Key0x4a,
            KeyCode::KeyCode0x4b => obs::hotkeys::Key::Key0x4b,
            KeyCode::KeyCode0x4c => obs::hotkeys::Key::Key0x4c,
            KeyCode::KeyCode0x4d => obs::hotkeys::Key::Key0x4d,
            KeyCode::KeyCode0x4e => obs::hotkeys::Key::Key0x4e,
            KeyCode::KeyCode0x4f => obs::hotkeys::Key::Key0x4f,
            KeyCode::KeyCode0x50 => obs::hotkeys::Key::Key0x50,
            KeyCode::KeyCode0x51 => obs::hotkeys::Key::Key0x51,
            KeyCode::KeyCode0x52 => obs::hotkeys::Key::Key0x52,
            KeyCode::KeyCode0x53 => obs::hotkeys::Key::Key0x53,
            KeyCode::KeyCode0x54 => obs::hotkeys::Key::Key0x54,
            KeyCode::KeyCode0x55 => obs::hotkeys::Key::Key0x55,
            KeyCode::KeyCode0x56 => obs::hotkeys::Key::Key0x56,
            KeyCode::KeyCode0x57 => obs::hotkeys::Key::Key0x57,
            KeyCode::KeyCode0x58 => obs::hotkeys::Key::Key0x58,
            KeyCode::KeyCode0x59 => obs::hotkeys::Key::Key0x59,
            KeyCode::KeyCode0x5a => obs::hotkeys::Key::Key0x5a,
            KeyCode::VkLwin => obs::hotkeys::Key::VkLwin,
            KeyCode::VkRwin => obs::hotkeys::Key::VkRwin,
            KeyCode::VkApps => obs::hotkeys::Key::VkApps,
            KeyCode::KeyCode0x5e => obs::hotkeys::Key::Key0x5e,
            KeyCode::VkSleep => obs::hotkeys::Key::VkSleep,
            KeyCode::VkSeparator => obs::hotkeys::Key::VkSeparator,
            KeyCode::KeyCode0x88 => obs::hotkeys::Key::Key0x88,
            KeyCode::KeyCode0x89 => obs::hotkeys::Key::Key0x89,
            KeyCode::KeyCode0x8a => obs::hotkeys::Key::Key0x8a,
            KeyCode::KeyCode0x8b => obs::hotkeys::Key::Key0x8b,
            KeyCode::KeyCode0x8c => obs::hotkeys::Key::Key0x8c,
            KeyCode::KeyCode0x8d => obs::hotkeys::Key::Key0x8d,
            KeyCode::KeyCode0x8e => obs::hotkeys::Key::Key0x8e,
            KeyCode::KeyCode0x8f => obs::hotkeys::Key::Key0x8f,
            KeyCode::VkOemFjJisho => obs::hotkeys::Key::VkOemFjJisho,
            KeyCode::VkOemFjLoya => obs::hotkeys::Key::VkOemFjLoya,
            KeyCode::VkOemFjRoya => obs::hotkeys::Key::VkOemFjRoya,
            KeyCode::KeyCode0x97 => obs::hotkeys::Key::Key0x97,
            KeyCode::KeyCode0x98 => obs::hotkeys::Key::Key0x98,
            KeyCode::KeyCode0x99 => obs::hotkeys::Key::Key0x99,
            KeyCode::KeyCode0x9a => obs::hotkeys::Key::Key0x9a,
            KeyCode::KeyCode0x9b => obs::hotkeys::Key::Key0x9b,
            KeyCode::KeyCode0x9c => obs::hotkeys::Key::Key0x9c,
            KeyCode::KeyCode0x9d => obs::hotkeys::Key::Key0x9d,
            KeyCode::KeyCode0x9e => obs::hotkeys::Key::Key0x9e,
            KeyCode::KeyCode0x9f => obs::hotkeys::Key::Key0x9f,
            KeyCode::VkLshift => obs::hotkeys::Key::VkLshift,
            KeyCode::VkRshift => obs::hotkeys::Key::VkRshift,
            KeyCode::VkLcontrol => obs::hotkeys::Key::VkLcontrol,
            KeyCode::VkRcontrol => obs::hotkeys::Key::VkRcontrol,
            KeyCode::VkLmenu => obs::hotkeys::Key::VkLmenu,
            KeyCode::VkRmenu => obs::hotkeys::Key::VkRmenu,
            KeyCode::VkBrowserBack => obs::hotkeys::Key::VkBrowserBack,
            KeyCode::VkBrowserForward => obs::hotkeys::Key::VkBrowserForward,
            KeyCode::VkBrowserRefresh => obs::hotkeys::Key::VkBrowserRefresh,
            KeyCode::VkBrowserStop => obs::hotkeys::Key::VkBrowserStop,
            KeyCode::VkBrowserSearch => obs::hotkeys::Key::VkBrowserSearch,
            KeyCode::VkBrowserFavorites => obs::hotkeys::Key::VkBrowserFavorites,
            KeyCode::VkBrowserHome => obs::hotkeys::Key::VkBrowserHome,
            KeyCode::VkVolumeMute => obs::hotkeys::Key::VkVolumeMute,
            KeyCode::VkVolumeDown => obs::hotkeys::Key::VkVolumeDown,
            KeyCode::VkVolumeUp => obs::hotkeys::Key::VkVolumeUp,
            KeyCode::VkMediaNextTrack => obs::hotkeys::Key::VkMediaNextTrack,
            KeyCode::VkMediaPrevTrack => obs::hotkeys::Key::VkMediaPrevTrack,
            KeyCode::VkMediaStop => obs::hotkeys::Key::VkMediaStop,
            KeyCode::VkMediaPlayPause => obs::hotkeys::Key::VkMediaPlayPause,
            KeyCode::VkLaunchMail => obs::hotkeys::Key::VkLaunchMail,
            KeyCode::VkLaunchMediaSelect => obs::hotkeys::Key::VkLaunchMediaSelect,
            KeyCode::VkLaunchApp1 => obs::hotkeys::Key::VkLaunchApp1,
            KeyCode::VkLaunchApp2 => obs::hotkeys::Key::VkLaunchApp2,
            KeyCode::KeyCode0xb8 => obs::hotkeys::Key::Key0xb8,
            KeyCode::KeyCode0xb9 => obs::hotkeys::Key::Key0xb9,
            KeyCode::KeyCode0xc1 => obs::hotkeys::Key::Key0xc1,
            KeyCode::KeyCode0xc2 => obs::hotkeys::Key::Key0xc2,
            KeyCode::KeyCode0xc3 => obs::hotkeys::Key::Key0xc3,
            KeyCode::KeyCode0xc4 => obs::hotkeys::Key::Key0xc4,
            KeyCode::KeyCode0xc5 => obs::hotkeys::Key::Key0xc5,
            KeyCode::KeyCode0xc6 => obs::hotkeys::Key::Key0xc6,
            KeyCode::KeyCode0xc7 => obs::hotkeys::Key::Key0xc7,
            KeyCode::KeyCode0xc8 => obs::hotkeys::Key::Key0xc8,
            KeyCode::KeyCode0xc9 => obs::hotkeys::Key::Key0xc9,
            KeyCode::KeyCode0xca => obs::hotkeys::Key::Key0xca,
            KeyCode::KeyCode0xcb => obs::hotkeys::Key::Key0xcb,
            KeyCode::KeyCode0xcc => obs::hotkeys::Key::Key0xcc,
            KeyCode::KeyCode0xcd => obs::hotkeys::Key::Key0xcd,
            KeyCode::KeyCode0xce => obs::hotkeys::Key::Key0xce,
            KeyCode::KeyCode0xcf => obs::hotkeys::Key::Key0xcf,
            KeyCode::KeyCode0xd0 => obs::hotkeys::Key::Key0xd0,
            KeyCode::KeyCode0xd1 => obs::hotkeys::Key::Key0xd1,
            KeyCode::KeyCode0xd2 => obs::hotkeys::Key::Key0xd2,
            KeyCode::KeyCode0xd3 => obs::hotkeys::Key::Key0xd3,
            KeyCode::KeyCode0xd4 => obs::hotkeys::Key::Key0xd4,
            KeyCode::KeyCode0xd5 => obs::hotkeys::Key::Key0xd5,
            KeyCode::KeyCode0xd6 => obs::hotkeys::Key::Key0xd6,
            KeyCode::KeyCode0xd7 => obs::hotkeys::Key::Key0xd7,
            KeyCode::KeyCode0xd8 => obs::hotkeys::Key::Key0xd8,
            KeyCode::KeyCode0xd9 => obs::hotkeys::Key::Key0xd9,
            KeyCode::KeyCode0xda => obs::hotkeys::Key::Key0xda,
            KeyCode::VkOem8 => obs::hotkeys::Key::VkOem8,
            KeyCode::KeyCode0xe0 => obs::hotkeys::Key::Key0xe0,
            KeyCode::VkOemAx => obs::hotkeys::Key::VkOemAx,
            KeyCode::VkIcoHelp => obs::hotkeys::Key::VkIcoHelp,
            KeyCode::VkIco00 => obs::hotkeys::Key::VkIco00,
            KeyCode::VkProcesskey => obs::hotkeys::Key::VkProcesskey,
            KeyCode::VkIcoClear => obs::hotkeys::Key::VkIcoClear,
            KeyCode::VkPacket => obs::hotkeys::Key::VkPacket,
            KeyCode::KeyCode0xe8 => obs::hotkeys::Key::Key0xe8,
            KeyCode::VkOemReset => obs::hotkeys::Key::VkOemReset,
            KeyCode::VkOemJump => obs::hotkeys::Key::VkOemJump,
            KeyCode::VkOemPa1 => obs::hotkeys::Key::VkOemPa1,
            KeyCode::VkOemPa2 => obs::hotkeys::Key::VkOemPa2,
            KeyCode::VkOemPa3 => obs::hotkeys::Key::VkOemPa3,
            KeyCode::VkOemWsctrl => obs::hotkeys::Key::VkOemWsctrl,
            KeyCode::VkOemCusel => obs::hotkeys::Key::VkOemCusel,
            KeyCode::VkOemAttn => obs::hotkeys::Key::VkOemAttn,
            KeyCode::VkOemFinish => obs::hotkeys::Key::VkOemFinish,
            KeyCode::VkOemCopy => obs::hotkeys::Key::VkOemCopy,
            KeyCode::VkOemAuto => obs::hotkeys::Key::VkOemAuto,
            KeyCode::VkOemEnlw => obs::hotkeys::Key::VkOemEnlw,
            KeyCode::VkAttn => obs::hotkeys::Key::VkAttn,
            KeyCode::VkCrsel => obs::hotkeys::Key::VkCrsel,
            KeyCode::VkExsel => obs::hotkeys::Key::VkExsel,
            KeyCode::VkEreof => obs::hotkeys::Key::VkEreof,
            KeyCode::VkPlay => obs::hotkeys::Key::VkPlay,
            KeyCode::VkZoom => obs::hotkeys::Key::VkZoom,
            KeyCode::VkNoname => obs::hotkeys::Key::VkNoname,
            KeyCode::VkPa1 => obs::hotkeys::Key::VkPa1,
            KeyCode::VkOemClear => obs::hotkeys::Key::VkOemClear,
        })
    }
}

pub struct Service;

#[tonic::async_trait]
impl General for Service {
    async fn get_version(&self, request: Request<()>) -> Result<Response<VersionReply>, Status> {
        use self::version_reply::SemVer;

        info!(
            "General.GetVersion request from {:?}",
            request.remote_addr()
        );

        Ok(Response::new(VersionReply {
            obs_studio_version: Some(SemVer {
                major: obs::libobs_sys::LIBOBS_API_MAJOR_VER,
                minor: obs::libobs_sys::LIBOBS_API_MINOR_VER,
                patch: obs::libobs_sys::LIBOBS_API_PATCH_VER,
            }),
            obs_remote_version: Some(SemVer {
                major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
                minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
                patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
            }),
            supported_image_export_formats: vec![],
        }))
    }

    async fn get_auth_required(
        &self,
        request: Request<()>,
    ) -> Result<Response<AuthRequiredReply>, Status> {
        info!(
            "General.GetAuthRequired request from {:?}",
            request.remote_addr()
        );

        Ok(Response::new(AuthRequiredReply {
            auth_required: false,
            ..Default::default()
        }))
    }

    async fn authenticate(
        &self,
        request: Request<AuthenticateRequest>,
    ) -> Result<Response<()>, Status> {
        info!(
            "General.Authenticate request from {:?}",
            request.remote_addr()
        );

        Ok(Response::new(()))
    }

    async fn set_filename_formatting(
        &self,
        request: Request<SetFilenameFormattingRequest>,
    ) -> Result<Response<()>, Status> {
        let filename_formatting = request.into_inner().filename_formatting;
        if filename_formatting.is_empty() {
            return Err(Status::invalid_argument(
                "filename formatting mustn't be empty",
            ));
        }

        let res = || {
            let config = obs::frontend::profile_config();
            config.set_string("Output", "FilenameFormatting", &filename_formatting);
            config.save()?;
            Ok(Response::new(()))
        };

        res().map_err(|e: anyhow::Error| Status::internal(e.to_string()))
    }

    async fn get_filename_formatting(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetFilenameFormattingReply>, Status> {
        let res = || {
            Ok(Response::new(GetFilenameFormattingReply {
                filename_formatting: obs::frontend::profile_config()
                    .string("Output", "FilenameFormatting")
                    .unwrap_or_default(),
            }))
        };

        res().map_err(|e: anyhow::Error| Status::internal(e.to_string()))
    }

    #[allow(clippy::cast_precision_loss)]
    async fn get_stats(&self, request: Request<()>) -> Result<Response<StatsReply>, Status> {
        use obs::video::Video;
        use stats_reply::ObsStats;

        let main_video = Video::get();
        let path = {
            let config = obs::frontend::profile_config();
            let path = if config
                .string("Output", "Mode")
                .as_deref()
                .unwrap_or_default()
                == "Advanced"
            {
                config.string("AdvOut", "RecFilePath")
            } else {
                config.string("SimpleOutput", "FilePath")
            };

            path.unwrap_or_default()
        };

        Ok(Response::new(StatsReply {
            stats: Some(ObsStats {
                fps: obs::active_fps(),
                render_total_frames: obs::total_frames(),
                render_missed_frames: obs::lagged_frames(),
                output_total_frames: main_video.total_frames(),
                output_skipped_frames: main_video.skipped_frames(),
                average_frame_time: obs::average_frame_time_ns() as f64 / 1_000_000.0,
                cpu_usage: obs::os::cpu_usage(),
                memory_usage: obs::os::memory_usage() as f64 / 1024.0 / 1024.0,
                free_disk_space: obs::os::free_disk_space(&path) as f64 / 1024.0 / 1024.0,
            }),
        }))
    }

    async fn get_video_info(
        &self,
        request: Request<()>,
    ) -> Result<Response<VideoInfoReply>, Status> {
        use obs::video::{Colorspace, Format, RangeType, ScaleType, VideoInfo};

        let video = match VideoInfo::get() {
            Some(video) => video,
            None => return Err(Status::unavailable("currently no video active")),
        };

        Ok(Response::new(VideoInfoReply {
            base_width: video.base_size.0,
            base_height: video.base_size.1,
            output_width: video.output_size.0,
            output_height: video.output_size.1,
            fps: f64::from(*video.fps.numer()) / f64::from(*video.fps.denom()),
            video_format: match video.output_format {
                Format::Nv12 => video_info_reply::VideoFormat::Nv12,
                Format::I420 => video_info_reply::VideoFormat::I420,
                Format::I444 => video_info_reply::VideoFormat::I444,
                Format::Rgba => video_info_reply::VideoFormat::Rgba,
                _ => {
                    return Err(Status::internal(format!(
                        "unsupported video format `{:?}`",
                        video.output_format
                    )))
                }
            } as i32,
            color_space: match video.colorspace {
                Colorspace::Default | Colorspace::Cs709 => video_info_reply::ColorSpace::S709,
                Colorspace::Cs601 => video_info_reply::ColorSpace::S601,
                Colorspace::Srgb => video_info_reply::ColorSpace::Srgb,
                _ => video_info_reply::ColorSpace::Unspecified,
            } as i32,
            color_range: match video.range {
                RangeType::Default | RangeType::Partial => video_info_reply::ColorRange::Partial,
                RangeType::Full => video_info_reply::ColorRange::Full,
                RangeType::Unknown(_) => video_info_reply::ColorRange::Unspecified,
            } as i32,
            scale_type: match video.scale_type {
                ScaleType::Bilinear => video_info_reply::ScaleType::Bilinear,
                ScaleType::Area => video_info_reply::ScaleType::Area,
                ScaleType::Bicubic => video_info_reply::ScaleType::Bicubic,
                ScaleType::Lanczos => video_info_reply::ScaleType::Lanczos,
                ScaleType::Disable | ScaleType::Point | ScaleType::Unknown(_) => {
                    return Err(Status::internal(format!(
                        "unsupported scale type `{:?}`",
                        video.scale_type
                    )))
                }
            } as i32,
        }))
    }

    #[allow(clippy::cast_possible_truncation)]
    async fn open_projector(
        &self,
        request: Request<OpenProjectorRequest>,
    ) -> Result<Response<()>, Status> {
        use self::open_projector_request::ProjectorType;

        info!(
            "General.OpenProjector request from {:?}",
            request.remote_addr()
        );

        let request = request.into_inner();

        obs::frontend::open_projector(
            match request.r#type() {
                ProjectorType::Unspecified | ProjectorType::Default => "Default",
                ProjectorType::Preview => "Preview",
                ProjectorType::Source => "Source",
                ProjectorType::Scene => "Scene",
                ProjectorType::StudioProgram => "StudioProgram",
                ProjectorType::Multiview => "Multiview",
            },
            request.monitor.unwrap_or(-1) as i32,
            request.geometry.as_deref().unwrap_or_default(),
            request.name.as_deref().unwrap_or_default(),
        );

        Ok(Response::new(()))
    }

    async fn trigger_hotkey_by_name(
        &self,
        request: Request<TriggerHotkeyByNameRequest>,
    ) -> Result<Response<()>, Status> {
        let hotkey_name = request.into_inner().hotkey_name;
        precondition!(!hotkey_name.is_empty(), "name mustn't be empty");

        let hotkey = Hotkey::by_name(&hotkey_name)
            .ok_or_else(|| Status::failed_precondition(format!("`{hotkey_name}` doesn't exist")))?;

        hotkey.trigger_routed_callback(true);

        Ok(Response::new(()))
    }

    async fn trigger_hotkey_by_sequence(
        &self,
        request: Request<TriggerHotkeyBySequenceRequest>,
    ) -> Result<Response<()>, Status> {
        let request = request.into_inner();
        let code: Option<Key> = request.code().into();
        let code = code.ok_or_else(|| Status::failed_precondition("key code must be specified"))?;
        let modifiers = request.modifiers.unwrap_or_default();

        let combo = KeyCombination {
            key: code,
            modifiers: {
                let mut flags = InteractionFlags::empty();
                flags.set(InteractionFlags::SHIFT_KEY, modifiers.shift);
                flags.set(InteractionFlags::ALT_KEY, modifiers.alt);
                flags.set(InteractionFlags::CONTROL_KEY, modifiers.control);
                flags.set(InteractionFlags::COMMAND_KEY, modifiers.command);
                flags
            },
        };

        combo.inject_event(false);
        combo.inject_event(true);
        combo.inject_event(false);

        Ok(Response::new(()))
    }
}
