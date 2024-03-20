use crate::{Error, Properties};
use std::borrow::Cow;

pub(crate) fn load<'bytes>(mut content: &'bytes [u8]) -> Result<Properties<'bytes>, Error> {
    let mut pairs = Vec::new();
    let mut line = 0;
    let mut partial = None::<(&'bytes str, String)>;

    while !content.is_empty() {
        let mut cur = match memchr::memchr2(b'\n', b'\r', content) {
            Some(br) => {
                let (cur, next) = content.split_at(br);
                content = &next[1..];
                cur
            }

            None => {
                let next = content;
                content = &content[content.len()..];
                next
            }
        };

        line += 1;

        if let Some(comment) = memchr::memchr2(b'#', b'!', cur) {
            cur = &cur[..comment];
        }

        if let Some((key, mut value)) = partial {
            if odd_backslash(cur) {
                let ext = std::str::from_utf8(&cur[0..cur.len() - 1])
                    .map_err(|e| Error::InvalidUtf8(line, e))?
                    .trim();

                value.push_str(ext);

                partial = Some((key, value));
            } else {
                let ext = std::str::from_utf8(&cur[0..cur.len()])
                    .map_err(|e| Error::InvalidUtf8(line, e))?
                    .trim();

                value.push_str(ext);

                pairs.push((Cow::Borrowed(key), Cow::Owned(value)));
                partial = None;
            }
        } else {
            if cur.is_empty() {
                continue;
            }

            let Some(assign) = memchr::memchr2(b'=', b':', cur) else {
                let s = std::str::from_utf8(cur).map_err(|e| Error::InvalidUtf8(line, e))?;

                return Err(Error::MalformedLine(line, s.to_string()));
            };

            let (pre, post) = cur.split_at(assign);

            let key = std::str::from_utf8(pre)
                .map_err(|e| Error::InvalidUtf8(line, e))?
                .trim();

            if odd_backslash(post) {
                let value = std::str::from_utf8(&post[1..])
                    .map_err(|e| Error::InvalidUtf8(line, e))?
                    .trim();

                let mut value = value.to_string();
                value.pop();

                partial = Some((key, value));
            } else {
                let val = match escape(&post[1..]) {
                    Some(buf) => buf,
                    None => {
                        let s =
                            std::str::from_utf8(cur).map_err(|e| Error::InvalidUtf8(line, e))?;
                        return Err(Error::InvalidEscape(line, s.to_string()));
                    }
                };

                let value = match val {
                    Cow::Owned(bytes) => {
                        let mut string = String::from_utf8(bytes)
                            .map_err(|e| Error::InvalidUtf8(line, e.utf8_error()))?;

                        string = string.trim().to_string(); // not the most efficient thing

                        Cow::Owned(string)
                    }

                    Cow::Borrowed(bytes) => Cow::Borrowed(
                        std::str::from_utf8(bytes)
                            .map_err(|e| Error::InvalidUtf8(line, e))?
                            .trim(),
                    ),
                };

                pairs.push((Cow::Borrowed(key), value));
            }
        }
    }

    Ok(Properties { pairs })
}

fn odd_backslash(line: &[u8]) -> bool {
    let mut cnt = 0;

    for ch in line.iter().rev() {
        if *ch == b'\\' {
            cnt += 1
        } else {
            break;
        }
    }

    cnt & 1 == 1
}

fn escape(line: &[u8]) -> Option<Cow<[u8]>> {
    if line.is_empty() {
        return Some(Cow::Borrowed(line));
    }

    let last = line.len() - 1;

    if !line[0..last].contains(&b'\\') {
        return Some(Cow::Borrowed(line));
    }

    let mut vec = Vec::with_capacity(line.len());
    let mut it = 0;

    while it < last {
        let ch = line[it];

        if ch == b'\\' {
            it += 1;
            let ch = line[it];

            match ch {
                b'r' => vec.push(b'\r'),
                b'n' => vec.push(b'\n'),
                b't' => vec.push(b'\t'),
                b'\\' => vec.push(b'\\'),
                b'u' => {
                    if it + 5 > line.len() {
                        return None;
                    }

                    it += 1;

                    let mut buf = [0, 0, 0, 0];

                    let len = unicode_encode(
                        [line[it], line[it + 1], line[it + 2], line[it + 3]],
                        &mut buf,
                    );

                    if len == 0 {
                        return None;
                    }

                    for ch in &buf[0..len] {
                        vec.push(*ch);
                    }

                    it += 3; // last line in loop brings it to 4
                }

                _ => return None,
            }
        } else {
            vec.push(ch);
        }

        it += 1;
    }

    vec.push(line[last]);

    Some(Cow::Owned(vec))
}

fn unicode_encode(unicode: [u8; 4], out: &mut [u8; 4]) -> usize {
    let mut code: u16 = 0;

    let cur = hex(unicode[0]);
    if cur == 0xff {
        return 0;
    }

    code |= cur as u16;
    code <<= 4;

    let cur = hex(unicode[1]);
    if cur == 0xff {
        return 0;
    }

    code |= cur as u16;
    code <<= 4;

    let cur = hex(unicode[2]);
    if cur == 0xff {
        return 0;
    }

    code |= cur as u16;
    code <<= 4;

    let cur = hex(unicode[3]);
    if cur == 0xff {
        return 0;
    }

    code |= cur as u16;

    if code & 0xFF80 == 0x0 {
        out[0] = code as u8;
        1
    } else if code & 0xF800 == 0x0 {
        out[1] = (code & 0x003F) as u8;
        code >>= 6;
        out[0] = (code & 0x001F) as u8;

        out[0] |= 0xC0;
        out[1] |= 0x80;

        2
    } else {
        out[2] = (code & 0x003F) as u8;
        code >>= 6;

        out[1] = (code & 0x003F) as u8;
        code >>= 6;

        out[0] = (code & 0x000F) as u8;

        out[0] |= 0xE0;
        out[1] |= 0x80;
        out[2] |= 0x80;

        3
    } // not handling mult unicode for now
}

fn hex(ch: u8) -> u8 {
    match ch {
        b'0'..=b'9' => ch - b'0',
        b'a'..=b'f' => ch - b'a' + 10,
        b'A'..=b'F' => ch - b'A' + 10,
        _ => 0xff,
    }
}
#[test]
fn test_odd_backslashh() {
    assert!(odd_backslash(b"hello\\"));
    assert!(!odd_backslash(b"hello\\\\"));
    assert!(odd_backslash(b"hello\\\\\\"));
}
