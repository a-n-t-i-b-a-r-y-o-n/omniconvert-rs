mod formats;
mod cheat;
mod game;
mod omniconvert;
mod token;
mod armax;

#[cfg(test)]
mod tests {
    use crate::armax;
    use crate::omniconvert;

    #[test]
    fn armax_generate_seeds() {
        let correct_seeds: [u32; 32] = [
            0x21323E26, 0x3105192B, 0x0E243331, 0x2F0C2F17, 0x132B0F33, 0x2402182B, 0x25111D1F, 0x39181F36,
            0x130A2B29, 0x03392B1E, 0x2A3C3D35, 0x3010122B, 0x29001D0A, 0x26273F32, 0x09252E2D, 0x083A071F,
            0x35390512, 0x14203F37, 0x28130A2F, 0x03311D1F, 0x002C3F33, 0x132E323C, 0x2A281138, 0x0C051B37,
            0x34052C27, 0x202E1D3A, 0x07121F2D, 0x0D28333C, 0x160B3138, 0x3214273B, 0x02150B1F, 0x020B3E3C,
        ];

        let test_seeds = armax::seeds::generate(false);

        assert_eq!(test_seeds.len(), correct_seeds.len());

        assert_eq!(test_seeds, correct_seeds)
    }

    #[test]
    fn armax_recognize_single() {
        // Full test code: UQRN-ER36-M3RD5\nWC60-T93N-MGJBW\n7QTG-QEQB-YXP60\nVFE7-FK9B-M32EA\nKQEK-5ZFB-F8UP9
        assert_eq!(true, armax::is_armax_code("UQRN-ER36-M3RD5"))
    }

    #[test]
    fn armax_decode_single() {
        // Full test code: UQRN-ER36-M3RD5\nWC60-T93N-MGJBW\n7QTG-QEQB-YXP60\nVFE7-FK9B-M32EA\nKQEK-5ZFB-F8UP9
        assert_eq!(Some(vec!((3589363552 as u32, 1721823442 as u32))), armax::decrypt::alpha_to_octets(vec!("UQRNER36M3RD5")))
    }

    /*
    #[test]
    fn minimal_conversion() {
        omniconvert::minimal_conversion();
        assert!(true);
    }
     */
}


