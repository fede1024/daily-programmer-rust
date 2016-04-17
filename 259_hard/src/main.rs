use std::fmt;

struct Encoding {
    data: [i64; 24],
    len: usize,
    val: i64,
}

struct EncodingGenerator {
    count: i64,
}

impl Iterator for EncodingGenerator {
    type Item = Encoding;

    fn next(&mut self) -> Option<Encoding> {
        let mut enc = Encoding{data: [0i64; 24], len: 0, val: 0};

        if self.count == 0 {
            enc.len = 1;
            enc.val = 1;
            self.count += 1;
            return Some(enc);
        }

        let mut c = self.count;
        while c > 0 {
            enc.data[enc.len] = c % 3;
            enc.len += 1;
            enc.val = match c % 3 {
                0 => enc.val + (enc.len as i64),
                1 => enc.val - (enc.len as i64),
                2 => enc.val * (enc.len as i64),
                _ => enc.val,
            };
            c /= 3;
        }
        self.count += 1;
        Some(enc)
    }
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in self.data.iter().take(self.len) { write!(f, "{}", x).unwrap(); };
        Ok(())
    }
}

fn find_enc(x: i64) -> Box<Iterator<Item=Encoding>> {
    let it = EncodingGenerator{count: 0}
        .filter(move |e| e.val == x)
        .scan(0, |len, e| {
            if *len == 0 || e.len == *len {
                *len = e.len;
                Some(e)
            } else {
                None
            }});
    Box::new(it)
}

fn main() {
    for x in 0..501 {
        println!("Encodings for: {}", x);
        for enc in find_enc(x) {
            println!("  {}", enc);
        }
    }
}
