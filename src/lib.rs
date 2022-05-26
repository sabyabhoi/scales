/* const S: [&'static str; 21] = [
    "Ab", "A", "A#", "Bb", "B", "B#", "Cb", "C", "C#", "Db", "D", "D#", "Eb", "E", "E#", "Fb", "F",
    "F#", "Gb", "G", "G#",
]; */

const S: [&'static str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

pub enum Key {
    Major(String),
    Minor(String),
}

pub struct Scale {
    pub scale_type: ScaleType,
    pub key: Key,
}

pub enum ScaleType {
    Diatonic,
    Pentatonic,
    Hexatonic,
}

impl Key {
    pub fn gen_keys(&self) -> Vec<&str> {
        let (s, m) = match self {
            Key::Major(root) => (root, [2, 2, 1, 2, 2, 2, 1]),
            Key::Minor(root) => (root, [2, 1, 2, 2, 1, 2, 2]),
        };

        let mut root = S
            .iter()
            .position(|&note| note == s)
            .expect("Enter a valid scale");

        m.iter()
            .map(|&i| {
                let tmp = S.iter().nth(root % 12).unwrap();
                root += i;
                *tmp
            })
            .collect::<Vec<&str>>()
    }
}

impl Scale {
    pub fn display(&self) -> Vec<&str> {
        let key = &self.key;
        let key_vec = key.gen_keys();

        match &self.scale_type {
            ScaleType::Diatonic => {
                return key_vec;
            }
            ScaleType::Pentatonic => {
                let skip = match &key {
                    Key::Major(_) => (3, 6),
                    Key::Minor(_) => (1, 5),
                };
                let key_vec = key_vec
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &e)| {
                        if i != skip.0 && i != skip.1 {
                            Some(e)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<&str>>();
                return key_vec;
            }
            _ => key_vec,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_test() {
        let c_major = Key::Major(String::from("C"));
        let c_major = c_major.gen_keys();
        assert_eq!("C D E F G A B".split(" ").collect::<Vec<&str>>(), c_major);

        let a_minor = Key::Minor(String::from("A"));
        let a_minor = a_minor.gen_keys();
        assert_eq!("A B C D E F G".split(" ").collect::<Vec<&str>>(), a_minor);
    }

    #[test]
    fn pentatonic_test() {
        let c_pent = Scale {
            scale_type: ScaleType::Pentatonic,
            key: Key::Major(String::from("C")),
        };
        assert_eq!(
            "C D E G A".split(" ").collect::<Vec<&str>>(),
            c_pent.display()
        );
    }
}
