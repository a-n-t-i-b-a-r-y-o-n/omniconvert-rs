use crate::cheat::{Cheat, CheatState, UnknownCheat};
use crate::{ar2, armax, Region};
use crate::token::Token;

pub struct ARMAXCheat {
    pub game_id:        u32,            //  Parent Game ID
    pub region:         u8,             //  Game region
    pub id:             u32,            //  Cheat ID
    pub name:           String,         //  Cheat name
    pub comment:        String,         //  Cheat comment(s)
    pub enable_code:    bool,           //  Whether this code is the 'Master Code'
    pub codes:          Vec<u32>,       //  Codes composing this cheat
    pub state:          CheatState,     //  Decryption/translation state
}

impl From<UnknownCheat> for ARMAXCheat {
    fn from(input: UnknownCheat) -> Self {
        ARMAXCheat {
            game_id: if let Some(id) = input.parent { id } else { 0 },
            region: match input.region {
                Region::USA => 0,
                Region::PAL => 1,
                Region::Japan => 2,
                Region::Unknown => 3,
            },
            id: if let Some(id) = input.id { id } else { 0 },
            name: input.name,
            comment: if let Some(c) = input.comment { c } else { String::new() },
            enable_code: input.enable,
            codes: if let Some(c) = input.codes { c } else { vec![] },
            state: CheatState::New
        }
    }
}

impl ARMAXCheat {
    // Decrypt address/value pairs from pairs of u32 codes
    pub fn decrypt(&mut self, armax_seeds: &[u32; 32], ar2_seeds: &[u8; 4]) -> bool {
        // Output codes
        let mut out_codes: Vec<u32> = vec!();

        // TODO: Make an ARMAX disc hash if we're using ARMAX output w/ auto verifier

        // Iterate through addr/val pairs
        for i in (0..self.codes.len()).step_by(2) {
            // Decrypt each pair and add to output
            let (addr, val) =
                armax::decrypt::decrypt_pair((self.codes[i], self.codes[i+1]), armax_seeds);
            out_codes.push(addr);
            out_codes.push(val);
        }
        // TODO: [ARMAXCheat::decrypt] Check for trailing address/value?

        if out_codes.len() > 0 {

            // TODO: [ARMAXCheat::decrypt] Verify decrypted codes with CRC16

            // Apply mask to 1st code
            out_codes[0] &= 0x0FFFFFFF;

            // Determine ARMAX verifier code count (given two u32 code octets per line)
            let verifier_code_count = (armax::decrypt::read_verifier_length(&out_codes) as usize) * 2;

            // Determine non-ARMAX-verifier (i.e. AR2) code count
            let ar2_code_count = out_codes.len() - verifier_code_count;

            if ar2_code_count > 0 {

                // Separate MAX verifier codes from AR2 non-verifier codes that still require AR2 decryption
                let (max_codes, ar2_codes) = out_codes.split_at(verifier_code_count);

                // Clone AR2 codes to mutable vector
                let mut ar2_codes = ar2_codes.to_vec();

                // Swap bytes of AR2 codes
                for i in 0..ar2_code_count {
                    ar2_codes[i] = armax::swap_bytes(ar2_codes[i]);
                }

                // Decrypt all AR2 codes
                ar2_codes = ar2::decrypt::decrypt_cheat(ar2_codes, ar2_seeds);

                // Re-combine decrypted ARMAX codes and newly-decrypted AR2 codes
                out_codes = max_codes.to_vec();
                out_codes.append(&mut ar2_codes.to_vec());
            }

            // Update our codes to the decrypted ones
            self.codes = out_codes;

            // Read cheat metadata from decrypted codes
            self.read_cheat_meta();

            return true;
        }
        return false;
    }

    // Read metadata from decrypted codes and update provided input Cheat
    fn read_cheat_meta(&mut self) {
        // Key array for bit string operations
        let mut key: [u32; 3] = [
            0u32,
            4u32,   // Skip reading CRC bytes
            self.codes.len() as u32,
        ];

        // WARNING: READING PERMUTES THE KEY ARRAY - ORDER MATTERS!
        self.game_id = armax::decrypt::read_bit_string(&self.codes, &mut key, 13);
        self.id = armax::decrypt::read_bit_string(&self.codes, &mut key, 19);
        self.enable_code = armax::decrypt::read_bit_string(&self.codes, &mut key, 1) == 1;
        let _unknown= armax::decrypt::read_bit_string(&self.codes, &mut key, 1) == 1;
        self.region = armax::decrypt::read_bit_string(&self.codes, &mut key, 2) as u8;
    }
}

impl Cheat for ARMAXCheat {

    fn new() -> ARMAXCheat {
        ARMAXCheat {
            game_id: 0,
            region: 0,
            id: 0,
            name: "New Cheat".to_string(),
            comment: "".to_string(),
            enable_code: false,
            codes: vec![],
            state: CheatState::New,
        }
    }

    fn from_tokens(input: &Vec<Token>) -> Self {
        todo!()
    }

    fn state(&self) -> CheatState {
        // TODO: [ARMAXCheat] Fix unnecessary cloning by using lifetimes
        self.state.clone()
    }

    fn region(&self) -> Region {
        match self.region {
            0 => Region::USA,
            1 => Region::PAL,
            2 => Region::Japan,
            _ => Region::Unknown
        }
    }

    fn id(&self) -> Option<u32> { Some(self.id) }

    fn parent_id(&self) -> Option<u32> { Some(self.game_id) }

    fn enable_code(&self) -> bool { self.enable_code }

    fn name(&self) -> String { String::from(&self.name) }

    fn comment(&self) -> Option<String> {
        if self.comment.is_empty() {
            return None
        }
        Some(String::from(&self.comment))
    }

    fn codes(&self) -> Option<Vec<u32>> {
        if self.codes.len() == 0 {
            return None
        }
        // TODO: [ARMAXCheat] Fix unnecessary cloning by using lifetimes
        Some(self.codes.clone())
    }
}