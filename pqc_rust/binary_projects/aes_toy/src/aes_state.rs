use crate::utils::{gmatrix,AES_SBOX, AES_INV_SBOX, AES_MIX_COLUMNS, AES_INV_MIX_COLUMNS};

pub struct AesState {
    state: [u8; 16]
}





impl AesState {
    //Constructor, default state is input
    pub fn new(s: &[u8; 16]) -> Self {
        Self { state: *s }
    }
    // This replaces each byte of state with that byte (as a number:usize)'s index of AES_SBOX.
    fn sub_bytes(&mut self) {
        for i in 0..16 {
            self.state[i] = AES_SBOX[self.state[i] as usize];
        }
    }

    fn inv_sub_bytes(&mut self) {
        for i in 0..16 {
            self.state[i] = AES_INV_SBOX[self.state[i] as usize];
        }
    }
        
    fn shift_rows(&mut self) {
        let mut temp = [0u8; 16]; // Initializes all entries to 0
        for i in 0..16 {
            temp[i] = self.state[4*(((i%4)+(i/4))%4)+(i%4)];
        }
        self.state = temp;
    }

    fn inv_shift_rows(&mut self) {
        let mut temp = [0u8; 16]; // Initializes all entries to 0
        for i in 0..16 {
            temp[i] = self.state[4*(((i/4)+4-(i%4))%4)+(i%4)];
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

    fn inv_mix_columns(&mut self) {
        let mut temp = [0u8; 4]; // Initializes all entries to 0
        for j in 0..4 {
            for i in 0..4 {
                temp[i] = self.state[4*j+i];
            }
            temp = gmatrix(AES_INV_MIX_COLUMNS, temp);
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
    pub fn output(&self) -> [u8; 16] {
        self.state
    }

    pub fn encrypt(&mut self, round_keys: &[[u8; 16];11]) {
        self.add_round_key(round_keys[0]);

        for r in 1..10 {
            self.sub_bytes();
            self.shift_rows();
            self.mix_columns();
            self.add_round_key(round_keys[r]);
        }
        self.sub_bytes();
        self.shift_rows();
        self.add_round_key(round_keys[10]);
    }

    

    pub fn decrypt(&mut self, round_keys: &[[u8; 16];11]) {
        self.add_round_key(round_keys[10]);
        self.inv_shift_rows();
        self.inv_sub_bytes();
        for r in (1..10).rev() {
            self.add_round_key(round_keys[r]);
            self.inv_mix_columns();
            self.inv_shift_rows();
            self.inv_sub_bytes();
        }

        self.add_round_key(round_keys[0]);
    }
}

