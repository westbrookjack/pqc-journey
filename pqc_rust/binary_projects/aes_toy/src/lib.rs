pub struct AesState {
    state: [u8; 16]
}

//Here is the AES standard S-box
const AES_SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5,
    0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0,
    0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc,
    0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a,
    0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0,
    0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b,
    0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85,
    0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5,
    0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17,
    0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88,
    0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c,
    0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9,
    0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6,
    0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e,
    0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94,
    0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68,
    0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];
//I am storing this where a_ij = AES_MATRIX[4i+j] and AES_MATRIX[n] = a_n/4,n%4
const AES_MIX_COLUMNS: [u8; 16] = [2,3,1,1,1,2,3,1,1,1,2,3,3,1,1,2];


impl AesState {
    pub fn new(input: [u8; 16], key: [u8; 16]) -> Self {
        let state = xor_state(input, key);
        Self { state }
    }
    // This replaces each byte of state with that byte (as a number:usize)'s index of AES_SBOX.
    fn sub_bytes(&mut self) {
        for i in 0..16 {
            self.state[i] = AES_SBOX[self.state[i] as usize];
        }
    }
        
    fn shift_rows(&mut self) {
        let mut temp = [0u8; 16]; // Initializes all entries to 0
        for i in 0..16 {
            temp[i] = self.state[4*((i%4+i/4)%4)+i%4];
        }
        self.state = temp;
    }

    fn mix_columns(&mut self) {
        let mut temp = [0u8; 4]; // Initializes all entries to 0
        for j in 0..4 {
            for i in 0..4 {
                temp[i] = self.state[4*j+i];
            }
            temp = gmatrix(AES_MIX_COLUMNS, temp);
            for i in 0.. 4 {
                self.state[4*j+i] = temp[i];
            }
        }
    }

    fn add_round_key(&mut self, round_key: [u8; 16]) { 
        for i in 0..16 {
            self.state[i] ^= round_key[i];
        }
 }
    //Note: rounds is 1-indexed
    pub fn encrypt(&mut self, rounds: usize, round_keys: &[ [u8; 16] ]) {
        self.add_round_key(round_keys[0]);

        for r in 1..rounds {
            self.sub_bytes();
            self.shift_rows();
            self.mix_columns();
            self.add_round_key(round_keys[r]);
        }
        self.sub_bytes();
        self.shift_rows();
        self.add_round_key(round_keys[rounds]);
    }

    pub fn output(&self) -> [u8; 16] {
        self.state
    }
}

pub fn key_schedule(key: &[u8; 16], num_rounds: usize) -> Vec<[u8; 16]> {
    // For now, repeat the same key every round (insecure but simple)
    vec![*key; num_rounds + 1] // one key for each round + initial
}


//here, we are identifying {0,1}^8 with GF(2,8) (basis 1,x,x^2,...,x^7) where x has minimal polynomial x^8+x^4+x^3+x+1 over GF(2),
// and where the i-th letter from the left is identified with that value times x^i. Thus 3 = 00000011 is identified with x+1--similarly 2 is identified with x

//This function takes a byte, identifies it with an element of GF(2,8), multiplies that result by x, and returns that result converted back to a byte.
fn gmul2(b: u8) -> u8 {
    let shifted = b << 1;
    //This part of the code checks if b and 10000000 is equal to 1, i.e. if the msb of b is 1. If yes, we will get an x^8 in the product, which reduces to x^4+x^3+x+1, identified with 27=0xb1,
    //which we must add, modulo 2, which is why we XOR with 27
    if b & 0x80 != 0 {
        return shifted ^ 0x1b;
    }
    return shifted;
    
}

//returns the index from the left of the msb of b. if b=0, returns 0 as well.
// fn msb_index(b:u8) -> u8 {
//     for i in 0..8 {
//         if b<<i & 0x80 != 0 {
//             return i;
//         }
//     }
//     return 0;
// }


//multiplies the GF(2,8) version of a with x^k. Assume 0<= k <8
fn gmul_power2(b:u8, k:u8) -> u8 {
    let mut result:u8 = b;
    for _i in 0..k {
        result = gmul2(result);
    }
    result
}

fn gmul(a:u8, b:u8) -> u8 {
    let mut result:u8 = 0;
    for i in 0 .. 8 {
        if (a>>i)&1 != 0 {
            result ^= gmul_power2(b,i);
        }
    }
    result
}

//assume the matrix fills rows first (as opposed to columns first)
fn gmatrix(m:[u8;16], v:[u8;4]) -> [u8;4] {
    let mut result = [0u8; 4]; // Initializes all entries to 0

    for i in 0.. 4 {
            for j in 0.. 4 {
                result[i] ^= gmul(m[4*i+j],v[j]);
            }
    }
    result
}

fn xor_state(a: [u8; 16], b: [u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = a[i] ^ b[i];
    }
    out
}