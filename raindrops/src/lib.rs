use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct RainDropSounds(Vec<RainDropSound>);

#[derive(Debug)]
enum RainDropSound {
    Pling,
    Plang,
    Plong,
    Mute(u32),
}

impl From<u32> for RainDropSounds {
    fn from(n: u32) -> Self {
        let mut sounds: Vec<RainDropSound> = vec![];

        if n % 3 == 0 {
            sounds.push(RainDropSound::Pling)
        }
        if n % 5 == 0 {
            sounds.push(RainDropSound::Plang)
        }
        if n % 7 == 0 {
            sounds.push(RainDropSound::Plong)
        }

        if sounds.is_empty() {
            sounds.push(RainDropSound::Mute(n));
        }

        RainDropSounds(sounds)
    }
}

impl Display for RainDropSound {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            RainDropSound::Pling => {
                write!(f, "Pling")
            }
            RainDropSound::Plang => {
                write!(f, "Plang")
            }
            RainDropSound::Plong => {
                write!(f, "Plong")
            }
            RainDropSound::Mute(n) => {
                write!(f, "{}", n)
            }
        }
    }
}

impl Display for RainDropSounds {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let s: String = self
            .0
            .iter()
            .map(|rain_drop_sound| rain_drop_sound.to_string())
            .collect();
        write!(f, "{}", s)
    }
}

pub fn raindrops(n: u32) -> String {
    RainDropSounds::from(n).to_string()
}
