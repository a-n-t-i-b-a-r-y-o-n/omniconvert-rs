use crate::armax::table;

// Default ActionReplay MAX seed?
// TODO: Investigate ARMAX seeds.
// This is declared in omniconvert.c as `armseeds` for use in batch ARMAX encryption/decryption functions.
// In armax.c, `armBatchDecryptFull()` receives it as the `ar2key` parameter, but it appears unused.
pub const DEFAULT_SEED: u32 = 0x04030209;

// Generate ActionReplay MAX seeds
pub fn generate(reverse: bool) -> [u32; 32] {
    let mut output: [u32; 32] = [0u32; 32];

    let mut tmp: u8;
    let mut tmp2: u8;

    let mut arr0: [u8; 56] = [0u8; 56];
    let mut arr1: [u8; 56] = [0u8; 56];
    let mut arr2: [u8; 8] = [0u8; 8];

    for i in 0..56 {
        tmp = table::G0[i] - 1;
        arr0[i] = (
            (
                (
                    0i32 - (table::GS[(tmp >> 3) as usize] & table::G1[(tmp & 7) as usize]) as i32
                ) as u32
            ) >> 31
        ) as u8;
    }

    for i in 0..16 {
        arr2 = [0u8; 8];
        tmp2 = table::G2[i];

        for j in 0..56 {
            tmp = tmp2+j;

            if j > 0x1B {
                if tmp > 0x37 {
                    tmp -= 0x1C;
                }
            }
            else if tmp > 0x1B {
                tmp -= 0x1C;
            }

            arr1[j as usize] = arr0[tmp as usize];
        }

        for j in 0..48 {
            if arr1[(table::G3[j]-1) as usize] > 0 {
                continue;
            }
            tmp = (((j * 0x2AAB) >> 16) - (j >> 0x1F)) as u8;
            arr2[tmp as usize] |= (table::G1[(j - (tmp * 6) as usize) as usize] >> 2);
        }

        output[i << 1] = (
            (
                (arr2[0] as u32) << 24)  |
                ((arr2[2] as u32) << 16) |
                ((arr2[4] as u32) << 8)  |
                (arr2[6] as u32)
        );
        output[(i << 1) + 1] = (
            (
                (arr2[1] as u32) << 24)  |
                ((arr2[3] as u32) << 16) |
                ((arr2[5] as u32) << 8)  |
                (arr2[7] as u32)
        );
    }

    if !reverse {
        let mut z = 31;

        let mut range = (0..16).into_iter();
        while let (Some(x), Some(y)) = (range.next(), range.next()) {
            output.swap(x, z-1);
            output.swap(y, z);
            z -= 2;
        }
    }

    output
}