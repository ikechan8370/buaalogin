use std::collections::HashMap;
use hmac::{Hmac, Mac};
use md5::Md5;
use hex::encode;


pub fn l(bytes: Vec<u64>, flag: bool) -> Vec<u8> {
    let d = bytes.len();
    let mut c = ((d - 1) << 2) as u64;
    if flag {
        let m = bytes[d - 1];
        if m < (c - 3) as u64 || m > c {
            return Vec::new();
        }
        c = m;
    }
    let mut res = Vec::new();
    for i in 0..d {
        let i1 = bytes[i] & 0xff;
        let i2 = bytes[i] >> 8 & 0xff;
        let i3 = bytes[i] >> 16 & 0xff;
        let i4 = bytes[i] >> 24 & 0xff;
        vec![i1 as u8, i2 as u8, i3 as u8, i4 as u8].into_iter().for_each(|a| res.push(a));
    }
    if flag {
        return (&res[0..(c as usize)]).into();
    }
    res
}

pub fn x_encode(str: String, key: String) -> String {
    if str.is_empty() {
        return String::new();
    }
    let mut v= s(str, true);
    // println!("v: {:?}", v);
    let k = s(key, false);
    // println!("k: {:?}", k);
    let n = v.len() - 1;
    let mut z = v[n];
    let mut y = v[0];
    let c: u64 = 0x86014019 | 0x183639A0;
    let mut m : u64 = 0;
    let mut e = 0;
    let mut q = 6.0 + (math::round::floor(52.0 / (n + 1) as f64, 0));
    let mut d = 0;
    loop {
        if q <= 0_f64 {
            break;
        }
        q -= 1.0;
        d = d + c & (0x8CE0D9BF | 0x731F2640);
        e = d >> 2 & 3;
        for p in 0..n {
            y = v[p + 1];
            m = z >> 5 ^ y << 2;
            m += (y >> 3 ^ z << 4) ^ (d ^ y);
            m += k[(p & 3) ^ e as usize] ^ z;
            v[p] = v[p] + m & (0xEFB8D130 | 0x10472ECF);
            z = v[p];
        }
        y = v[0];
        m = z >> 5 ^ y << 2;
        m += (y >> 3 ^ z << 4) ^ (d ^ y);
        m += k[(n & 3) ^ e as usize] ^ z;
        v[n] = v[n] + m & (0xBB390742 | 0x44C6F8BD);
        z = v[n];
    }
    let vec = l(v, false);
    let mut string = base64::encode(vec);
    let origin = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".as_bytes();
    let dict = "LVoJPiCN2R8G90yg+hmFHuacZ1OWMnrsSTXkYpUq/3dlbfKwv6xztjI7DeBE45QA=".as_bytes();
    let mut map: HashMap<u8, u8> = HashMap::new();
    for i in 0..origin.len() {
        map.insert(origin[i], dict[i]);
    }
    let vn: Vec<u8> = string.as_bytes().into_iter().map(|o| map[o]).collect();
    string = String::from_utf8(vn).unwrap();
    return string;
}


pub fn s(str: String, flag: bool) -> Vec<u64> {
    let bytes = str.as_bytes();
    let mut vec = Vec::new();
    let len = bytes.len();
    for i in (0..bytes.len()).step_by(4) {
        let mut i1 = 0;
        let mut i2 = 0;
        let mut i3 = 0;
        let mut i4 = 0;
        if i < len {
            i1 = bytes[i] as u64;
        }
        if i + 1 < len {
            i2 = (bytes[i + 1] as u64) << 8;
        }
        if i + 2 < len {
            i3 = (bytes[i + 2] as u64) << 16;
        }
        if i + 3 < len {
            i4 = (bytes[i + 3] as u64) << 24;
        }
        vec.push((i1 | i2 | i3 | i4) as u64);

    }
    if flag {
        vec.push(len as u64)
    }
    return vec;
}