mod formats;
mod cheat;
mod game;
mod omniconvert;
mod token;
mod armax;
mod magic;

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

    // Part of decrypt_pair()
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

        // Get first code of first cheat
        let mut codes = game.cheats[0].codes.iter();
        if let (Some(in_addr), Some(in_val)) = (codes.next(), codes.next()) {
            // Swap bytes
            let mut addr = armax::decrypt::swap_bytes(*in_addr);
            let mut val = armax::decrypt::swap_bytes(*in_val);
            // Unscramble step 1
            let unscrambled = armax::decrypt::unscramble_1(addr, val);

            assert_eq!(unscrambled.0, 2122126365);
            assert_eq!(unscrambled.1, 4190765348);
        }
        else {
            assert!(false);
        }

    }

    // Part of decrypt_pair()
    #[test]
    fn armax_apply_seeds() {
        // Test input: "Enable Code" for Kingdom Hearts
        let test_input = "UQRN-ER36-M3RD5\nWC60-T93N-MGJBW\n7QTG-QEQB-YXP60\nVFE7-FK9B-M32EA\nKQEK-5ZFB-F8UP9";

        // Default state
        let mut state: omniconvert::State = omniconvert::State::new();
        let seeds = state.armax_seeds;

        // Dummy game object
        let mut game: Game = Game::new();

        // Tokenize input
        let tokens = omniconvert::read_input(test_input, state.incrypt.code.format);

        // Parse input into cheats
        game.cheats = omniconvert::build_cheat_list(tokens);

        assert!(game.cheats.len() > 0);

        // Get first code of first cheat
        let mut codes = game.cheats[0].codes.iter();
        if let (Some(in_addr), Some(in_val)) = (codes.next(), codes.next()) {
            // Swap bytes
            let mut addr = armax::decrypt::swap_bytes(*in_addr);
            let mut val = armax::decrypt::swap_bytes(*in_val);
            // Unscramble step 1
            let unscrambled = armax::decrypt::unscramble_1(addr, val);
            addr = unscrambled.0;
            val = unscrambled.1;

            assert_eq!(unscrambled.0, 2122126365);
            assert_eq!(unscrambled.1, 4190765348);

            // Apply seeds
            let mut range = (0..32).into_iter();
            while let (
                // Seed indexes
                Some(seed_a), Some(seed_b), Some(seed_c), Some(seed_d)
            ) = (range.next(), range.next(), range.next(), range.next()) {

                let mut tmp = armax::decrypt::rotate_right(val, 4) ^ seeds[seed_a];
                let mut tmp2 = val ^ seeds[seed_b];
                addr ^= armax::decrypt::octet_mask(tmp, tmp2);

                tmp = armax::decrypt::rotate_right(addr,4) ^ seeds[seed_c];
                tmp2 = addr ^ seeds[seed_d];
                val ^= armax::decrypt::octet_mask(tmp, tmp2);
            }

            assert_eq!(addr, 870574636);
            assert_eq!(val, 3363966584);
        }
        else {
            assert!(false);
        }
    }

    // Part of decrypt_pair()
    #[test]
    fn armax_unscramble_2() {
        // Test input: "Enable Code" for Kingdom Hearts
        let test_input = "UQRN-ER36-M3RD5\nWC60-T93N-MGJBW\n7QTG-QEQB-YXP60\nVFE7-FK9B-M32EA\nKQEK-5ZFB-F8UP9";

        // Default state
        let mut state: omniconvert::State = omniconvert::State::new();
        let seeds = state.armax_seeds;

        // Dummy game object
        let mut game: Game = Game::new();

        // Tokenize input
        let tokens = omniconvert::read_input(test_input, state.incrypt.code.format);

        // Parse input into cheats
        game.cheats = omniconvert::build_cheat_list(tokens);

        assert!(game.cheats.len() > 0);

        // Get first code of first cheat
        let mut codes = game.cheats[0].codes.iter();
        if let (Some(in_addr), Some(in_val)) = (codes.next(), codes.next()) {
            // Swap bytes
            let mut addr = armax::decrypt::swap_bytes(*in_addr);
            let mut val = armax::decrypt::swap_bytes(*in_val);
            // Unscramble step 1
            let unscrambled = armax::decrypt::unscramble_1(addr, val);
            addr = unscrambled.0;
            val = unscrambled.1;

            // TEST: unscramble_1
            assert_eq!(unscrambled.0, 2122126365);
            assert_eq!(unscrambled.1, 4190765348);

            // Apply seeds
            let mut range = (0..32).into_iter();
            while let (
                // Seed indexes
                Some(seed_a), Some(seed_b), Some(seed_c), Some(seed_d)
            ) = (range.next(), range.next(), range.next(), range.next()) {

                let mut tmp = armax::decrypt::rotate_right(val, 4) ^ seeds[seed_a];
                let mut tmp2 = val ^ seeds[seed_b];
                addr ^= armax::decrypt::octet_mask(tmp, tmp2);

                tmp = armax::decrypt::rotate_right(addr,4) ^ seeds[seed_c];
                tmp2 = addr ^ seeds[seed_d];
                val ^= armax::decrypt::octet_mask(tmp, tmp2);
            }

            // TEST: apply_seeds
            assert_eq!(addr, 870574636);
            assert_eq!(val, 3363966584);

            let unscrambled = armax::decrypt::unscramble_2(addr, val);
            addr = unscrambled.0;
            val = unscrambled.1;

            assert_eq!(addr, 2875815976);
            assert_eq!(val, 3154530177);

        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn armax_decrypt_single() {
        // Test input: "Enable Code" for Kingdom Hearts
        let test_input = "UQRN-ER36-M3RD5\nWC60-T93N-MGJBW\n7QTG-QEQB-YXP60\nVFE7-FK9B-M32EA\nKQEK-5ZFB-F8UP9";

        // Default state
        let mut state: omniconvert::State = omniconvert::State::new();
        let seeds = &state.armax_seeds;

        // Dummy game object
        let mut game: Game = Game::new();

        // Tokenize input
        let tokens = omniconvert::read_input(test_input, state.incrypt.code.format);

        // Parse input into cheats
        game.cheats = omniconvert::build_cheat_list(tokens);

        assert!(game.cheats.len() > 0);

        // Decrypt
        let mut codes = game.cheats[0].codes.iter();
        if let (Some(in_addr), Some(in_val)) = (codes.next(), codes.next()) {
            let (out_addr, out_val) = armax::decrypt::decrypt_pair((*in_addr, *in_val), seeds);

            assert_eq!(out_addr, 2169439932);
            assert_eq!(out_val, 678980011);

            let mut addr = out_addr;
            let mut val = out_val;

            println!("{} / {}", addr, val);
        }
        else {
            assert!(false);
        }

        assert!(game.cheats.len() > 0);
        assert!(game.cheats[0].codes.len() > 0);


    }

    #[test]
    fn armax_decrypt_batch() {
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

        // Sanity check
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


