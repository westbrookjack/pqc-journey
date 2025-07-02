use crate::utils::{RCON, rot_word, sub_word};

pub fn key_schedule(key: &[u8; 16]) -> [[u8; 16];11] {
    let mut round_keys: [[u8; 16]; 11] = [[0u8; 16]; 11];
    let mut w: [[u8; 4]; 44] = [[0u8; 4]; 44];
    for i in 0.. 4{
        for j in 0..4 {
            w[i][j]=key[4*i+j];
        }
    }
    for i in 4..44 {
        let mut temp = w[i - 1];
        if i % 4 == 0 {
            temp = sub_word(rot_word(temp));
            temp[0] ^= RCON[i / 4 - 1];
        }
        for j in 0..4 {
            w[i][j] = w[i - 4][j] ^ temp[j];
        }
    }
    for i in 0..11{
        for j in 0..16 {
            round_keys[i][j] = w[4*i+j/4][j%4];
        }
    }
    round_keys
}



