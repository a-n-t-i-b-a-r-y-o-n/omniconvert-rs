mod formats;
mod cheat;
mod game;
mod omniconvert;
mod token;
mod armax;

#[cfg(test)]
mod tests {
    use crate::armax;
    use crate::game::Game;
    use crate::omniconvert;

    #[test]
    fn armax_generate_seeds() {
        let correct_seeds: [u32; 32] = [
            0x21323E26, 0x3105192B, 0x0E243331, 0x2F0C2F17, 0x132B0F33, 0x2402182B, 0x25111D1F, 0x39181F36,
            0x130A2B29, 0x03392B1E, 0x2A3C3D35, 0x3010122B, 0x29001D0A, 0x26273F32, 0x09252E2D, 0x083A071F,
            0x35390512, 0x14203F37, 0x28130A2F, 0x03311D1F, 0x002C3F33, 0x132E323C, 0x2A281138, 0x0C051B37,
            0x34052C27, 0x202E1D3A, 0x07121F2D, 0x0D28333C, 0x160B3138, 0x3214273B, 0x02150B1F, 0x020B3E3C,
        ];

        let test_seeds = armax::seeds::generate();

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

    #[test]
    fn armax_alpha_to_octets() {

    }

    #[test]
    fn armax_unscramble_1() {
        // Test input: "Enable Code" for Kingdom Hearts
        let test_input = "UQRN-ER36-M3RD5\nWC60-T93N-MGJBW\n7QTG-QEQB-YXP60\nVFE7-FK9B-M32EA\nKQEK-5ZFB-F8UP9";

        // Default state
        let mut state: omniconvert::State = omniconvert::State::new();

        // Dummy game object
        let mut game: Game = Game::new();

        // Tokenize input
        let tokens = omniconvert::read_input(test_input, state.incrypt.code.format);

        // Parse input into cheats
        game.cheats = omniconvert::build_cheat_list(tokens);

        assert!(game.cheats.len() > 0);

        // Decrypt codes of first cheat
        let mut codes = game.cheats[0].codes.iter();
        while let (Some(in_addr), Some(in_val)) = (codes.next(), codes.next()) {
            // Swap bytes
            let mut addr = armax::decrypt::swap_bytes(*in_addr);
            let mut val = armax::decrypt::swap_bytes(*in_val);

            // Unscramble step 1
            let unscrambled = armax::decrypt::unscramble_1(addr, val);

            println!("({:08X}/{:08X}) -> ({:08X}/{:08X})", addr, val, unscrambled.0, unscrambled.1);
        }
    }

    #[test]
    fn armax_unscramble_2() {

    }

    #[test]
    fn armax_decrypt_single() {
        // Test input: "Enable Code" for Kingdom Hearts
        let test_input = "UQRN-ER36-M3RD5\nWC60-T93N-MGJBW\n7QTG-QEQB-YXP60\nVFE7-FK9B-M32EA\nKQEK-5ZFB-F8UP9";

        // Default state
        let mut state: omniconvert::State = omniconvert::State::new();

        // Dummy game object
        let mut game: Game = Game::new();

        // Tokenize input
        let tokens = omniconvert::read_input(test_input, state.incrypt.code.format);

        // Parse input into cheats
        game.cheats = omniconvert::build_cheat_list(tokens);

        assert!(game.cheats.len() > 0);

        // Decrypt
        for cheat in &mut game.cheats {
            cheat.codes = armax::decrypt::batch(&mut cheat.codes, &state.armax_seeds);
        }

        assert!(game.cheats.len() > 0);
        assert!(game.cheats[0].codes.len() > 0);


    }

    /*
    #[test]
    fn minimal_conversion() {
        omniconvert::minimal_conversion();
        assert!(true);
    }
     */
}


