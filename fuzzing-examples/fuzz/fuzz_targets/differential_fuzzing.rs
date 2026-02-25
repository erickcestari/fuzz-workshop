#![no_main]

use libfuzzer_sys::fuzz_target;

fn double(x: i32) -> Option<i32> {
    x.checked_mul(2)
}

fn double2(x: i32) -> Option<i32> {
    // Off-by-one: using >= instead of >
    if x >= i32::MAX / 2 || x < i32::MIN / 2 {
        None
    } else {
        Some(x * 2)
    }
}

fuzz_target!(|data: &[u8]| {
    if let Some(x) = consume_i32(data) {
        let res = double(x);
        let res2 = double2(x);
        if res != res2 {panic!("x: {}, res: {:?}, res2: {:?}", x, res, res2);}
    }
});

fn consume_i32(data: &[u8]) -> Option<i32> {
    if data.len() >= 4 {
        let bytes: [u8; 4] = data[0..4].try_into().unwrap();
        return Some(i32::from_le_bytes(bytes));
    }
    None
}
