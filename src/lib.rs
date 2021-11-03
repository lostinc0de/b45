//pub mod base45;
use std::fmt;

#[derive(Debug, Clone)]
pub struct DecodingError {
    msg: String,
    input: String,
}

impl DecodingError {
    fn new(msg: &str, input: &str) -> Self {
        DecodingError {
            msg: String::from(msg),
            input: String::from(input),
        }
    }
}

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.msg, self.input)
    }
}

pub fn encode_bytes(input: &[u8]) -> String {
    static LUT: [char; 45] = [ '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
                               'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
                               'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
                               'U', 'V', 'W', 'X', 'Y', 'Z', ' ', '$', '%', '*',
                               '+', '-', '.', '/', ':' ];
    let mut ret = String::with_capacity(input.len() * 3);
    for bytes in input.chunks(2) {
        let mut val = 0u16;
        if bytes.len() == 2 {
            val += bytes[0] as u16 * 256u16;
            val += bytes[1] as u16;
        } else {
            val += bytes[0] as u16;
        }
        // Compute the three bytes satisfying val = b1 * 45**0 + b2 * 45**1 + b3 * 45**2
        let mut arr = [u8::MAX; 3];
        for i in (0..3).rev() {
            let exp = i as u32;
            let div = 45u16.pow(exp);
            let tmp = val / div;
            // At least two bytes need to be set - even if though they are zero
            if tmp > 0 || i <= 1 {
                arr[i] = tmp as u8;
            }
            val -= tmp * div;
        }
        // Write the actual character of the byte to the string
        for b in arr {
            if b < 45 {
                let c = LUT[b as usize];
                ret.push(c);
            }
        }
    }
    ret
}

pub fn encode(input: &str) -> String {
    encode_bytes(input.as_bytes())
}

fn decode_byte(b: u8) -> u8 {
    let ret;
    if b >= b'0' && b <= b'9' {
        ret = b - b'0';
    } else if b >= b'A' && b <= b'Z' {
        ret = b - b'A' + 10;
    } else {
        match b {
            b' ' => ret = 36,
            b'$' => ret = 37,
            b'%' => ret = 38,
            b'*' => ret = 39,
            b'+' => ret = 40,
            b'-' => ret = 41,
            b'.' => ret = 42,
            b'/' => ret = 43,
            b':' => ret = 44,
            _ => ret = u8::MAX,
        }
    }
    ret
}

// Returns the decoded data as bytes
pub fn decode_to_bytes(input: &str) -> Result<Vec<u8>, DecodingError> {
    let mut ret = Vec::<u8>::with_capacity(input.len());
    for bytes in input.as_bytes().chunks(3) {
        let mut num = 0u16;
        for (ind, &byte) in bytes.iter().enumerate() {
            let dec = decode_byte(byte);
            if dec == u8::MAX {
                return Err(DecodingError::new("Error decoding base45 string", &input));
            }
            // Check for values too large -> decoding error
            let tmp = dec as u32 * 45u32.pow(ind as u32);
            if tmp > u16::MAX as u32 {
                return Err(DecodingError::new("Error decoding base45 string", &input));
            }
            num += tmp as u16;
        }
        let b1 = (num >> 8) as u8;
        if b1 > 0 {
            ret.push(b1);
        }
        let b2 = num as u8;
        ret.push(b2);
    }
    Ok(ret)
}

// Returns the decoded string
pub fn decode(input: &str) -> Result<String, DecodingError> {
    match decode_to_bytes(&input) {
        Ok(b) => {
            match String::from_utf8(b) {
                Ok(s) => Ok(s),
                Err(_e) => Err(DecodingError::new("Error transforming decoded string to UTF-8", &input)),
            }
        },
        Err(_e) => Err(_e),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encode() {
        assert_eq!(crate::encode("AB"), String::from("BB8"));
        assert_eq!(crate::encode("base-45"), String::from("UJCLQE7W581"));
        assert_eq!(crate::encode("Hello!!"), String::from("%69 VD92EX0"));
        assert_eq!(crate::encode("ietf!"), String::from("QED8WEX0"));
    }

    #[test]
    fn test_decode() {
        assert_eq!(crate::decode("BB8").unwrap(), String::from("AB"));
        assert_eq!(crate::decode("QED8WEX0").unwrap(), String::from("ietf!"));
        assert_eq!(crate::decode("UJCLQE7W581").unwrap(), String::from("base-45"));
        assert_eq!(crate::decode("%69 VD92EX0").unwrap(), String::from("Hello!!"));
        assert_eq!(crate::decode_to_bytes("QED8WEX0").unwrap(), vec![105, 101, 116, 102, 33]);
    }
}
