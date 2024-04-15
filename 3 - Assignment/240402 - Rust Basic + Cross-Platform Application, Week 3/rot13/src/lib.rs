use std::io::Read;

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

// Implement the `Read` trait for `RotDecoder`.
impl<R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        const ALPHABETS: u8 = ('Z' as u8) - ('A' as u8) + 1u8;
        const UPPER: u8 = 'A' as u8;
        const LOWER: u8 = 'a' as u8;
        let ret = self.input.read(buf)?;
        for x in buf {
            let x_ = *x;
            *x = if (UPPER <= x_) && (x_ < UPPER + ALPHABETS) {
                UPPER + (x_ - UPPER + 13u8) % ALPHABETS
            } else if (LOWER <= x_) && (x_ < LOWER + ALPHABETS) {
                LOWER + (x_ - LOWER + 13u8) % ALPHABETS
            } else {
                x_
            };
        }
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joke() {
        let mut rot = RotDecoder {
            input: "Gb trg gb gur bgure fvqr!".as_bytes(),
            rot: 13,
        };
        let mut ret = String::new();

        rot.read_to_string(&mut ret).unwrap();

        assert_eq!(&ret, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> {
            input: input.as_ref(),
            rot: 13,
        };
        let mut buf = [0u8; 256];

        assert_eq!(rot.read(&mut buf).unwrap(), 256);

        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }

    #[test]
    fn round_trip() {
        let input = "Why did the chicken cross the road?";
        let rot = RotDecoder {
            input: input.as_bytes(),
            rot: 13,
        };
        let mut rotrot = RotDecoder {
            input: rot,
            rot: 13,
        };
        let mut ret = String::new();

        rotrot.read_to_string(&mut ret).unwrap();

        assert_eq!(&ret, input);
    }
}
