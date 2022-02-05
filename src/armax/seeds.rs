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
        let a: u8 = tmp >> 3;
        let b: u8 = tmp & 7;
        let c: u32 = (table::GS[a as usize] & table::G1[b as usize]) as u32;
        let d: u32 = (u32::MAX as u64 + 1u64 - c as u64) as u32;
        let e: u8 = (d >> 31) as u8;
        arr0[i] = e;
        /*
        arr0[i] = (
            (
                (
                    0i32 - (table::GS[(tmp >> 3) as usize] & table::G1[(tmp & 7) as usize]) as i32
                ) as u32
            ) >> 31
        ) as u8;

         */
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

            let a0: u8 = arr0[tmp as usize];

            arr1[j as usize] = a0;
        }



        for j in 0..48 {

            let a1: u8 = table::G3[j];
            let b1: u8 = a1-1;
            let c1 = arr1[(b1) as usize];

            if c1 == 0 {
                continue;
            }

            let d1 = (j * 0x2AAB);
            let e1 = (d1 >> 16);
            let f1 = (j >> 0x1F);

            tmp = (e1 - f1) as u8;

            let g1 = (tmp * 6);
            let h1 = table::G1[(j - g1 as usize)];
            let i1 = (h1 as usize >> 2) as u8;
            let j1 = arr2[tmp as usize] | i1;

            arr2[tmp as usize] = j1;
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