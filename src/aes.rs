#![allow(unused)]

use std::time::Instant;
use rand::random;
use crate::benchmark::Bencher;

const ITERATIONS: usize = 400_000;
const ROUNDS: usize = 10;

type Msg = [[u8; 8]; 8];
type Key = [[u8; 8]; 8];
type Keys = [Key; ROUNDS];

const MSG: Msg = [
    [b'T', b'H', b'E', b' ', b'D', b'E', b'A', b' '],
    [b'T', b'H', b' ', b'S', b'T', b'A', b'R', b' '],
    [b'H', b'A', b'S', b' ', b'A', b' ', b'W', b' '],
    [b'W', b'E', b'A', b'K', b'N', b'E', b'S', b' '],
    [b'S', b'.', b' ', b'H', b'I', b'T', b' ', b' '],
    [b'T', b'H', b'E', b' ', b'R', b'E', b'A', b' '],
    [b'C', b'T', b'O', b'R', b'.', b' ', b' ', b' '],
    [b'C', b'T', b'O', b'R', b'.', b' ', b' ', b' '],
];

#[cfg(unix)]
const TEST_MSG: Msg = [
    [34, 248, 174, 185, 217, 68, 185, 32],
    [228, 195, 11, 95, 214, 219, 73, 0],
    [162, 50, 11, 7, 229, 246, 186, 0],
    [82, 153, 120, 138, 132, 5, 251, 0],
    [252, 86, 25, 254, 112, 82, 97, 0],
    [85, 12, 168, 152, 126, 236, 151, 0],
    [153, 196, 108, 214, 56, 181, 88, 0],
    [67, 84, 79, 82, 46, 32, 32, 32],
];

#[cfg(windows)]
const TEST_MSG: Msg = [
    [192, 158, 86, 194, 58, 148, 33, 32],
    [234, 206, 165, 7, 176, 145, 211, 0],
    [202, 38, 9, 207, 217, 225, 251, 0],
    [145, 78, 41, 111, 197, 137, 174, 0],
    [145, 95, 251, 218, 7, 94, 157, 0],
    [97, 156, 2, 240, 31, 126, 171, 0],
    [164, 148, 125, 246, 41, 245, 121, 0],
    [67, 84, 79, 82, 46, 32, 32, 32],
];

const SUBSTITUTE: [u8; 256] = [
    162, 12, 95, 59, 248, 181, 100, 229, 157, 50, 250, 255, 17, 119, 219, 234, 9, 155, 61, 177,
    146, 89, 94, 55, 222, 18, 57, 138, 173, 26, 77, 134, 117, 67, 218, 240, 39, 168, 36, 5, 142,
    198, 125, 220, 37, 133, 135, 104, 226, 190, 116, 111, 186, 54, 40, 197, 34, 113, 182, 252, 107,
    208, 175, 114, 0, 31, 28, 62, 98, 41, 247, 174, 150, 30, 183, 78, 172, 43, 145, 188, 244, 217,
    153, 90, 25, 210, 80, 209, 143, 32, 205, 127, 147, 24, 178, 152, 103, 253, 53, 13, 213, 230,
    70, 4, 141, 201, 212, 3, 251, 235, 11, 14, 29, 7, 170, 2, 254, 76, 42, 242, 68, 56, 130, 129,
    22, 176, 160, 215, 128, 154, 238, 88, 165, 64, 10, 132, 85, 159, 136, 137, 73, 120, 58, 110,
    199, 44, 108, 191, 79, 52, 167, 124, 194, 249, 20, 21, 149, 139, 193, 72, 65, 144, 118, 192,
    51, 106, 63, 122, 243, 236, 81, 203, 233, 115, 164, 109, 92, 27, 66, 195, 87, 179, 19, 237, 33,
    180, 101, 15, 91, 156, 206, 225, 102, 105, 202, 161, 48, 185, 99, 83, 69, 196, 166, 231, 140,
    241, 38, 123, 223, 112, 204, 169, 75, 211, 47, 246, 151, 96, 121, 158, 126, 97, 163, 8, 216,
    232, 187, 60, 6, 214, 227, 16, 171, 239, 131, 46, 148, 71, 221, 224, 86, 93, 45, 35, 74, 200,
    23, 184, 1, 82, 245, 189, 207, 84, 49, 228,
];

const POWER: [[u8; 256]; 6] = [
    [
        0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 0, 33, 68, 105, 144, 185,
        228, 17, 64, 113, 164, 217, 16, 73, 132, 193, 0, 65, 132, 201, 16, 89, 164, 241, 64, 145,
        228, 57, 144, 233, 68, 161, 0, 97, 196, 41, 144, 249, 100, 209, 64, 177, 36, 153, 16, 137,
        4, 129, 0, 129, 4, 137, 16, 153, 36, 177, 64, 209, 100, 249, 144, 41, 196, 97, 0, 161, 68,
        233, 144, 57, 228, 145, 64, 241, 164, 89, 16, 201, 132, 65, 0, 193, 132, 73, 16, 217, 164,
        113, 64, 17, 228, 185, 144, 105, 68, 33, 0, 225, 196, 169, 144, 121, 100, 81, 64, 49, 36,
        25, 16, 9, 4, 1, 0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 0, 33,
        68, 105, 144, 185, 228, 17, 64, 113, 164, 217, 16, 73, 132, 193, 0, 65, 132, 201, 16, 89,
        164, 241, 64, 145, 228, 57, 144, 233, 68, 161, 0, 97, 196, 41, 144, 249, 100, 209, 64, 177,
        36, 153, 16, 137, 4, 129, 0, 129, 4, 137, 16, 153, 36, 177, 64, 209, 100, 249, 144, 41,
        196, 97, 0, 161, 68, 233, 144, 57, 228, 145, 64, 241, 164, 89, 16, 201, 132, 65, 0, 193,
        132, 73, 16, 217, 164, 113, 64, 17, 228, 185, 144, 105, 68, 33, 0, 225, 196, 169, 144, 121,
        100, 81, 64, 49, 36, 25, 16, 9, 4, 1,
    ],
    [
        0, 1, 8, 27, 64, 125, 216, 87, 0, 217, 232, 51, 192, 149, 184, 47, 0, 49, 200, 203, 64, 45,
        152, 135, 0, 9, 168, 227, 192, 69, 120, 95, 0, 97, 136, 123, 64, 221, 88, 183, 0, 57, 104,
        147, 192, 245, 56, 143, 0, 145, 72, 43, 64, 141, 24, 231, 0, 105, 40, 67, 192, 165, 248,
        191, 0, 193, 8, 219, 64, 61, 216, 23, 0, 153, 232, 243, 192, 85, 184, 239, 0, 241, 200,
        139, 64, 237, 152, 71, 0, 201, 168, 163, 192, 5, 120, 31, 0, 33, 136, 59, 64, 157, 88, 119,
        0, 249, 104, 83, 192, 181, 56, 79, 0, 81, 72, 235, 64, 77, 24, 167, 0, 41, 40, 3, 192, 101,
        248, 127, 0, 129, 8, 155, 64, 253, 216, 215, 0, 89, 232, 179, 192, 21, 184, 175, 0, 177,
        200, 75, 64, 173, 152, 7, 0, 137, 168, 99, 192, 197, 120, 223, 0, 225, 136, 251, 64, 93,
        88, 55, 0, 185, 104, 19, 192, 117, 56, 15, 0, 17, 72, 171, 64, 13, 24, 103, 0, 233, 40,
        195, 192, 37, 248, 63, 0, 65, 8, 91, 64, 189, 216, 151, 0, 25, 232, 115, 192, 213, 184,
        111, 0, 113, 200, 11, 64, 109, 152, 199, 0, 73, 168, 35, 192, 133, 120, 159, 0, 161, 136,
        187, 64, 29, 88, 247, 0, 121, 104, 211, 192, 53, 56, 207, 0, 209, 72, 107, 64, 205, 24, 39,
        0, 169, 40, 131, 192, 229, 248, 255,
    ],
    [
        0, 1, 16, 81, 0, 113, 16, 97, 0, 161, 16, 49, 0, 145, 16, 193, 0, 65, 16, 17, 0, 177, 16,
        33, 0, 225, 16, 241, 0, 209, 16, 129, 0, 129, 16, 209, 0, 241, 16, 225, 0, 33, 16, 177, 0,
        17, 16, 65, 0, 193, 16, 145, 0, 49, 16, 161, 0, 97, 16, 113, 0, 81, 16, 1, 0, 1, 16, 81, 0,
        113, 16, 97, 0, 161, 16, 49, 0, 145, 16, 193, 0, 65, 16, 17, 0, 177, 16, 33, 0, 225, 16,
        241, 0, 209, 16, 129, 0, 129, 16, 209, 0, 241, 16, 225, 0, 33, 16, 177, 0, 17, 16, 65, 0,
        193, 16, 145, 0, 49, 16, 161, 0, 97, 16, 113, 0, 81, 16, 1, 0, 1, 16, 81, 0, 113, 16, 97,
        0, 161, 16, 49, 0, 145, 16, 193, 0, 65, 16, 17, 0, 177, 16, 33, 0, 225, 16, 241, 0, 209,
        16, 129, 0, 129, 16, 209, 0, 241, 16, 225, 0, 33, 16, 177, 0, 17, 16, 65, 0, 193, 16, 145,
        0, 49, 16, 161, 0, 97, 16, 113, 0, 81, 16, 1, 0, 1, 16, 81, 0, 113, 16, 97, 0, 161, 16, 49,
        0, 145, 16, 193, 0, 65, 16, 17, 0, 177, 16, 33, 0, 225, 16, 241, 0, 209, 16, 129, 0, 129,
        16, 209, 0, 241, 16, 225, 0, 33, 16, 177, 0, 17, 16, 65, 0, 193, 16, 145, 0, 49, 16, 161,
        0, 97, 16, 113, 0, 81, 16, 1,
    ],
    [
        0, 1, 32, 243, 0, 53, 96, 167, 0, 169, 160, 27, 0, 93, 224, 79, 0, 81, 32, 67, 0, 133, 96,
        247, 0, 249, 160, 107, 0, 173, 224, 159, 0, 161, 32, 147, 0, 213, 96, 71, 0, 73, 160, 187,
        0, 253, 224, 239, 0, 241, 32, 227, 0, 37, 96, 151, 0, 153, 160, 11, 0, 77, 224, 63, 0, 65,
        32, 51, 0, 117, 96, 231, 0, 233, 160, 91, 0, 157, 224, 143, 0, 145, 32, 131, 0, 197, 96,
        55, 0, 57, 160, 171, 0, 237, 224, 223, 0, 225, 32, 211, 0, 21, 96, 135, 0, 137, 160, 251,
        0, 61, 224, 47, 0, 49, 32, 35, 0, 101, 96, 215, 0, 217, 160, 75, 0, 141, 224, 127, 0, 129,
        32, 115, 0, 181, 96, 39, 0, 41, 160, 155, 0, 221, 224, 207, 0, 209, 32, 195, 0, 5, 96, 119,
        0, 121, 160, 235, 0, 45, 224, 31, 0, 33, 32, 19, 0, 85, 96, 199, 0, 201, 160, 59, 0, 125,
        224, 111, 0, 113, 32, 99, 0, 165, 96, 23, 0, 25, 160, 139, 0, 205, 224, 191, 0, 193, 32,
        179, 0, 245, 96, 103, 0, 105, 160, 219, 0, 29, 224, 15, 0, 17, 32, 3, 0, 69, 96, 183, 0,
        185, 160, 43, 0, 109, 224, 95, 0, 97, 32, 83, 0, 149, 96, 7, 0, 9, 160, 123, 0, 189, 224,
        175, 0, 177, 32, 163, 0, 229, 96, 87, 0, 89, 160, 203, 0, 13, 224, 255,
    ],
    [
        0, 1, 64, 217, 0, 9, 64, 145, 0, 241, 64, 41, 0, 185, 64, 161, 0, 97, 64, 249, 0, 233, 64,
        49, 0, 81, 64, 73, 0, 153, 64, 65, 0, 193, 64, 25, 0, 201, 64, 209, 0, 177, 64, 105, 0,
        121, 64, 225, 0, 33, 64, 57, 0, 169, 64, 113, 0, 17, 64, 137, 0, 89, 64, 129, 0, 129, 64,
        89, 0, 137, 64, 17, 0, 113, 64, 169, 0, 57, 64, 33, 0, 225, 64, 121, 0, 105, 64, 177, 0,
        209, 64, 201, 0, 25, 64, 193, 0, 65, 64, 153, 0, 73, 64, 81, 0, 49, 64, 233, 0, 249, 64,
        97, 0, 161, 64, 185, 0, 41, 64, 241, 0, 145, 64, 9, 0, 217, 64, 1, 0, 1, 64, 217, 0, 9, 64,
        145, 0, 241, 64, 41, 0, 185, 64, 161, 0, 97, 64, 249, 0, 233, 64, 49, 0, 81, 64, 73, 0,
        153, 64, 65, 0, 193, 64, 25, 0, 201, 64, 209, 0, 177, 64, 105, 0, 121, 64, 225, 0, 33, 64,
        57, 0, 169, 64, 113, 0, 17, 64, 137, 0, 89, 64, 129, 0, 129, 64, 89, 0, 137, 64, 17, 0,
        113, 64, 169, 0, 57, 64, 33, 0, 225, 64, 121, 0, 105, 64, 177, 0, 209, 64, 201, 0, 25, 64,
        193, 0, 65, 64, 153, 0, 73, 64, 81, 0, 49, 64, 233, 0, 249, 64, 97, 0, 161, 64, 185, 0, 41,
        64, 241, 0, 145, 64, 9, 0, 217, 64, 1,
    ],
    [
        0, 1, 128, 139, 0, 45, 128, 247, 0, 121, 128, 195, 0, 101, 128, 111, 0, 113, 128, 123, 0,
        29, 128, 103, 0, 233, 128, 179, 0, 85, 128, 223, 0, 225, 128, 107, 0, 13, 128, 215, 0, 89,
        128, 163, 0, 69, 128, 79, 0, 81, 128, 91, 0, 253, 128, 71, 0, 201, 128, 147, 0, 53, 128,
        191, 0, 193, 128, 75, 0, 237, 128, 183, 0, 57, 128, 131, 0, 37, 128, 47, 0, 49, 128, 59, 0,
        221, 128, 39, 0, 169, 128, 115, 0, 21, 128, 159, 0, 161, 128, 43, 0, 205, 128, 151, 0, 25,
        128, 99, 0, 5, 128, 15, 0, 17, 128, 27, 0, 189, 128, 7, 0, 137, 128, 83, 0, 245, 128, 127,
        0, 129, 128, 11, 0, 173, 128, 119, 0, 249, 128, 67, 0, 229, 128, 239, 0, 241, 128, 251, 0,
        157, 128, 231, 0, 105, 128, 51, 0, 213, 128, 95, 0, 97, 128, 235, 0, 141, 128, 87, 0, 217,
        128, 35, 0, 197, 128, 207, 0, 209, 128, 219, 0, 125, 128, 199, 0, 73, 128, 19, 0, 181, 128,
        63, 0, 65, 128, 203, 0, 109, 128, 55, 0, 185, 128, 3, 0, 165, 128, 175, 0, 177, 128, 187,
        0, 93, 128, 167, 0, 41, 128, 243, 0, 149, 128, 31, 0, 33, 128, 171, 0, 77, 128, 23, 0, 153,
        128, 227, 0, 133, 128, 143, 0, 145, 128, 155, 0, 61, 128, 135, 0, 9, 128, 211, 0, 117, 128,
        255,
    ],
];

#[inline]
fn power(base: u8, exponent: u8) -> u8 {
    match exponent {
        0 => 1,
        1 => base,
        n => POWER[(n - 2) as usize][base as usize],
    }
}

#[inline]
fn powers(column: [u8; 8]) -> [u8; 8] {
    [
        power(column[0], 1),
        power(column[1], 2),
        power(column[2], 3),
        power(column[3], 4),
        power(column[4], 5),
        power(column[5], 6),
        power(column[6], 7),
        0,
    ]
}

#[inline]
fn type_1_mix_powers(column: [u8; 8]) -> u8 {
    1 * column[0]
        + 2 * column[1]
        + 3 * column[2]
        + 4 * column[3]
        + 5 * column[4]
        + 6 * column[5]
        + 7 * column[6]
}

#[inline]
fn type_2_mix_powers(column: [u8; 8]) -> u8 {
    8 * column[0]
        + 7 * column[1]
        + 6 * column[2]
        + 5 * column[3]
        + 4 * column[4]
        + 3 * column[5]
        + 2 * column[6]
}

#[inline]
fn type_3_mix_powers(column: [u8; 8]) -> u8 {
    8 * (column[0] + column[1] + column[2] + column[3] + column[4] + column[5] + column[6])
}

#[inline]
fn type_4_mix_powers(column: [u8; 8]) -> u8 {
    4 * column[0]
        + 5 * column[1]
        + 6 * column[2]
        + 8 * column[3]
        + 6 * column[4]
        + 4 * column[5]
        + 2 * column[6]
}

#[inline]
fn extract_column(msg: &Msg, column_index: usize) -> [u8; 8] {
    [
        msg[0][column_index],
        msg[1][column_index],
        msg[2][column_index],
        msg[3][column_index],
        msg[4][column_index],
        msg[5][column_index],
        msg[6][column_index],
        0,
    ]
}

fn mix_and_add_key(msg: &mut Msg, key: &Key) {
    let mut power_cache = [
        powers(extract_column(msg, 0)),
        powers(extract_column(msg, 1)),
        powers(extract_column(msg, 2)),
        powers(extract_column(msg, 3)),
        powers(extract_column(msg, 4)),
        powers(extract_column(msg, 5)),
        powers(extract_column(msg, 6)),
        [0; 8],
    ];

    for column in 0..7 {
        msg[0][column] = type_1_mix_powers(power_cache[column]);
        power_cache[column][0] = power(msg[0][column], 1);
    }

    for column in 0..7 {
        msg[1][column] = type_2_mix_powers(power_cache[column]);
        power_cache[column][1] = power(msg[1][column], 2);
    }

    for column in 0..7 {
        msg[2][column] = type_1_mix_powers(power_cache[column]);
        power_cache[column][2] = power(msg[2][column], 3);
    }

    for column in 0..7 {
        msg[3][column] = type_2_mix_powers(power_cache[column]);
        power_cache[column][3] = power(msg[3][column], 4);
    }

    for column in 0..7 {
        msg[4][column] = type_3_mix_powers(power_cache[column]);
        power_cache[column][4] = power(msg[4][column], 5);
    }

    for column in 0..7 {
        msg[5][column] = type_4_mix_powers(power_cache[column]);
        power_cache[column][5] = power(msg[5][column], 6);
    }

    for row in 0..6 {
        msg[row] = (u64::from_ne_bytes(msg[row]) ^ u64::from_ne_bytes(key[row])).to_ne_bytes();
    }
    // unsafe {
    //     let msg_0 = msg.as_mut_ptr() as *mut u128;
    //     let msg_1 = msg_0.add(1);
    //     let msg_2 = msg_0.add(2);
    //
    //     let key_0 = key.as_ptr() as *mut u128;
    //     let key_1 = key_0.add(1);
    //     let key_2 = key_0.add(2);
    //
    //     *msg_0 ^= *key_0;
    //     *msg_1 ^= *key_1;
    //     *msg_2 ^= *key_2;
    // }

    msg[6] = key[6];
}

fn add_key(msg: &mut Msg, key: &Key) {
    for row in 0..7 {
        msg[row] = (u64::from_ne_bytes(msg[row]) ^ u64::from_ne_bytes(key[row])).to_ne_bytes();
    }
    // unsafe {
    //     let msg_0 = msg.as_mut_ptr() as *mut u128;
    //     let msg_1 = msg_0.add(1);
    //     let msg_2 = msg_0.add(2);
    //     let msg_3 = msg_0.add(3) as *mut u64;
    //
    //     let key_0 = key.as_ptr() as *mut u128;
    //     let key_1 = key_0.add(1);
    //     let key_2 = key_0.add(2);
    //     let key_3 = key_0.add(3) as *mut u64;
    //
    //     *msg_0 ^= *key_0;
    //     *msg_1 ^= *key_1;
    //     *msg_2 ^= *key_2;
    //     *msg_3 ^= *key_3;
    // }
}

#[inline]
fn substitute(value: u8) -> u8 {
    SUBSTITUTE[value as usize]
}

fn substitute_bytes_and_shift(msg: &mut Msg) {
    for row in &mut msg[..7] {
        for value in &mut row[..7] {
            *value = substitute(*value);
        }
    }

    msg[1] = [
        msg[1][1], msg[1][2], msg[1][3], msg[1][4], msg[1][5], msg[1][6], msg[1][0], 0,
    ];
    msg[2] = [
        msg[2][2], msg[2][3], msg[2][4], msg[2][5], msg[2][6], msg[2][0], msg[2][1], 0,
    ];
    msg[3] = [
        msg[3][3], msg[3][4], msg[3][5], msg[3][6], msg[3][0], msg[3][1], msg[3][2], 0,
    ];
    msg[4] = [
        msg[4][4], msg[4][5], msg[4][6], msg[4][0], msg[4][1], msg[4][2], msg[4][3], 0,
    ];
    msg[5] = [
        msg[5][5], msg[5][6], msg[5][0], msg[5][1], msg[5][2], msg[5][3], msg[5][4], 0,
    ];
    msg[6] = [
        msg[6][6], msg[6][0], msg[6][1], msg[6][2], msg[6][3], msg[6][4], msg[6][5], 0,
    ];
}

fn encrypt_once(msg: &mut Msg, keys: &Keys) {
    let mut key_index = 0;

    add_key(msg, &keys[key_index]);

    for _ in 0..ROUNDS {
        key_index = (key_index + 1) % ROUNDS;

        substitute_bytes_and_shift(msg);
        mix_and_add_key(msg, &keys[key_index]);
    }

    substitute_bytes_and_shift(msg);
    add_key(msg, &keys[key_index]);

    key_index = (key_index + 1) % ROUNDS;
}

fn encrypt(msg: &mut Msg, keys: &Keys, iterations: usize) {
    let mut key_index = 0;

    for _ in 0..iterations {
        add_key(msg, &keys[key_index]);

        for _ in 0..ROUNDS {
            key_index = (key_index + 1) % ROUNDS;

            substitute_bytes_and_shift(msg);
            mix_and_add_key(msg, &keys[key_index]);
        }

        substitute_bytes_and_shift(msg);
        add_key(msg, &keys[key_index]);

        key_index = (key_index + 1) % ROUNDS;
    }
}

macro_rules! error {
    ($($t:tt)*) => {
        eprintln!($($t)*);
        unsafe {
            ::libc::exit(1);
        }
    };
}

mod stdin {
    use core::str::Utf8Error;
    use heapless::string::String;

    pub(crate) fn read_bytes(buf: &mut [u8]) -> usize {
        unsafe {
            const STDIN_DESCRIPTOR: core::ffi::c_int = 0;
            #[cfg(windows)]
            {
                libc::read(
                    STDIN_DESCRIPTOR,
                    buf.as_mut_ptr().cast(),
                    buf.len() as core::ffi::c_uint,
                ) as usize
            }
            #[cfg(unix)]
            {
                libc::read(STDIN_DESCRIPTOR, buf.as_mut_ptr().cast(), buf.len()) as usize
            }
        }
    }

    pub(crate) fn read_string<const N: usize>() -> Result<String<N>, Utf8Error> {
        let mut buf = heapless::Vec::from_array([0; N]);
        let read = read_bytes(&mut buf);
        buf.truncate(read);
        String::from_utf8(buf)
    }
}

fn seed_zero() {
    unsafe {
        libc::srand(0);
    }
}

fn read_and_seed() {
    println!("READY");
    let Ok(string) = stdin::read_string::<32>() else {
        error!("bytes read from stdin could not be converted to utf8");
    };
    let Ok(seed) = string.trim().parse() else {
        error!("string read from stdin could not be parsed as integer");
    };

    eprintln!("Using seed {seed}");

    unsafe {
        libc::srand(seed);
    }
}

fn write(msg: &Msg) {
    for row in &msg[..7] {
        for &value in &row[..4] {
            print!("{value:x}");
        }
    }
    println!("\nDONE");
}

fn generate(keys: &mut Keys) {
    for key in keys {
        for row in &mut key[..7] {
            for value in &mut row[..7] {
                *value = (unsafe { libc::rand() } % u8::MAX as i32) as u8;
            }
        }
    }
}

#[repr(align(64))]
struct Aligned<T>(T);

/*
substitute: 50+50 loads, 50 stores = 150 ops
shift: 50 loads, 50 stores = 100 ops
mix prep: 50+50 loads, 50 stores = 150 ops
mix calc: 350 loads, 350 muls, 350 adds = 1050 ops
mix back: 50 loads, 50+50 stores = 150 ops
total: 1600 ops per round
10 rounds: 16k ops
 */

pub(crate) fn main() {
    let bencher = Bencher::new();
    
    let table: [u8; 256] = random();
    bencher.benchmark(|| 0, |i| table[i as usize]).report_as("table_lookup");

    let key: Aligned<Key> = Aligned(random());
    bencher.benchmark(|| Aligned(random()), |mut msg: Aligned<Msg>| {
        add_key(&mut msg.0, &key.0);
        msg
    }).report_as("add_key");

    let key: Aligned<Key> = Aligned(random());
    bencher.benchmark(|| Aligned(random()), |mut msg: Aligned<Msg>| {
        mix_and_add_key(&mut msg.0, &key.0);
        msg
    }).report_as("mix_and_add_key");

    let keys: Aligned<Keys> = Aligned(random());
    bencher.benchmark(|| Aligned(random()), |mut msg: Aligned<Msg>| {
        encrypt_once(&mut msg.0, &keys.0);
        msg
    }).report_as("encrypt_once");

    let mut keys: Aligned<Keys> = Aligned(Default::default());

    seed_zero();
    generate(&mut keys.0);
    bencher.benchmark(|| Aligned(MSG), |mut msg: Aligned<Msg>| {
        encrypt(&mut msg.0, &keys.0, ITERATIONS);
        msg
    }).report_as("encrypt");

    /*let mut keys: Aligned<Keys> = Aligned(Default::default());

    // real shit
    read_and_seed();

    let start_time = Instant::now();

    generate(&mut keys.0);

    let mut msg = Aligned(Default::default());
    msg = Aligned(MSG);
    encrypt(&mut msg.0, &keys.0, ITERATIONS);

    let end_time = Instant::now();

    write(&msg.0);

    eprintln!("86f69b272b1ccc617e4c7fa3aa7031f1f976f0ba2a90e59f99c46cd6 expected");
    eprintln!("took {:?}", (end_time - start_time));*/
}
