mod ar2;
mod armax;
mod cheat;
mod formats;
mod game;
mod magic;
mod omniconvert;
mod token;

#[cfg(test)]
mod armax_tests {
    use crate::armax;
    use crate::cheat::Cheat;
    use crate::game::{Game, Region};
    use crate::omniconvert;

    // "Enable Code" for Kingdom Hearts (USA)
    const TEST_CHEAT_SINGLE: &str =
r#"UQRN-ER36-M3RD5
WC60-T93N-MGJBW
7QTG-QEQB-YXP60
VFE7-FK9B-M32EA
KQEK-5ZFB-F8UP9"#;

    // "Enable Code", "Have All Trinities", and "Save Anywhere" for Kingdom Hearts (USA)
    const TEST_CHEAT_MULTIPLE: &str =
r#"Enable Code
UQRN-ER36-M3RD5
WC60-T93N-MGJBW
7QTG-QEQB-YXP60
VFE7-FK9B-M32EA
KQEK-5ZFB-F8UP9

Have All Trinities
PMGE-KJ9D-X4WRN
QJNC-EWMH-UQ48H

Save Anywhere
# Press Pause to access the menu
3QYW-CWCU-R0BCC
3WQR-X7EE-ADTJA"#;

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
        assert_eq!(true, armax::is_armax_code("UQRN-ER36-M3RD5"))
    }

    #[test]
    fn armax_decode_single() {
        assert_eq!(Some(vec!((3589363552 as u32, 1721823442 as u32))), armax::decrypt::alpha_to_octets(vec!("UQRNER36M3RD5")))
    }

    // Part of decrypt_pair()
    #[test]
    fn armax_unscramble_1() {
        // Default state
        let state: omniconvert::State = omniconvert::State::new();

        // Dummy game object
        let mut game: Game = Game::new();

        // Tokenize input
        let tokens = omniconvert::read_input(TEST_CHEAT_SINGLE, state.incrypt.code.format);

        // Parse input into cheats
        game.cheats = omniconvert::build_cheat_list(tokens);

        assert!(game.cheats.len() > 0);

        // Get first code of first cheat
        let mut codes = game.cheats[0].codes.iter();
        if let (Some(in_addr), Some(in_val)) = (codes.next(), codes.next()) {
            // Swap bytes
            let addr = armax::swap_bytes(*in_addr);
            let val = armax::swap_bytes(*in_val);
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
        // Default state
        let state: omniconvert::State = omniconvert::State::new();
        let seeds = state.armax_seeds;

        // Dummy game object
        let mut game: Game = Game::new();

        // Tokenize input
        let tokens = omniconvert::read_input(TEST_CHEAT_SINGLE, state.incrypt.code.format);

        // Parse input into cheats
        game.cheats = omniconvert::build_cheat_list(tokens);

        assert!(game.cheats.len() > 0);

        // Get first code of first cheat
        let mut codes = game.cheats[0].codes.iter();
        if let (Some(in_addr), Some(in_val)) = (codes.next(), codes.next()) {
            // Swap bytes
            let mut addr = armax::swap_bytes(*in_addr);
            let mut val = armax::swap_bytes(*in_val);
            // Unscramble step 1
            let unscrambled = armax::decrypt::unscramble_1(addr, val);
            addr = unscrambled.0;
            val = unscrambled.1;

            assert_eq!(unscrambled.0, 2122126365);
            assert_eq!(unscrambled.1, 4190765348);

            // Apply seeds
            for i in (0..32).step_by(4) {
                let mut tmp = armax::rotate_right(val, 4) ^ seeds[i];
                let mut tmp2 = val ^ seeds[i+1];
                addr ^= armax::decrypt::octet_mask(tmp, tmp2);

                tmp = armax::rotate_right(addr,4) ^ seeds[i+2];
                tmp2 = addr ^ seeds[i+3];
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
        // Default state
        let state: omniconvert::State = omniconvert::State::new();
        let seeds = state.armax_seeds;

        // Dummy game object
        let mut game: Game = Game::new();

        // Tokenize input
        let tokens = omniconvert::read_input(TEST_CHEAT_SINGLE, state.incrypt.code.format);

        // Parse input into cheats
        game.cheats = omniconvert::build_cheat_list(tokens);

        assert!(game.cheats.len() > 0);

        // Get first code of first cheat
        let mut codes = game.cheats[0].codes.iter();
        if let (Some(in_addr), Some(in_val)) = (codes.next(), codes.next()) {
            // Swap bytes
            let mut addr = armax::swap_bytes(*in_addr);
            let mut val = armax::swap_bytes(*in_val);
            // Unscramble step 1
            let unscrambled = armax::decrypt::unscramble_1(addr, val);
            addr = unscrambled.0;
            val = unscrambled.1;

            // TEST: unscramble_1
            assert_eq!(unscrambled.0, 2122126365);
            assert_eq!(unscrambled.1, 4190765348);

            // Apply seeds
            for i in (0..32).step_by(4) {
                let mut tmp = armax::rotate_right(val, 4) ^ seeds[i];
                let mut tmp2 = val ^ seeds[i+1];
                addr ^= armax::decrypt::octet_mask(tmp, tmp2);

                tmp = armax::rotate_right(addr,4) ^ seeds[i+2];
                tmp2 = addr ^ seeds[i+3];
                val ^= armax::decrypt::octet_mask(tmp, tmp2);
            }

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

    // Decrypt single ActionReplay MAX octet pair
    #[test]
    fn armax_decrypt_single_pair() {
        // Default state
        let state: omniconvert::State = omniconvert::State::new();
        let seeds = &state.armax_seeds;

        // Dummy game object
        let mut game: Game = Game::new();

        // Tokenize input
        let tokens = omniconvert::read_input(TEST_CHEAT_SINGLE, state.incrypt.code.format);

        // Parse input into cheats
        game.cheats = omniconvert::build_cheat_list(tokens);

        assert!(game.cheats.len() > 0);

        // Decrypt
        let mut codes = game.cheats[0].codes.iter();
        if let (Some(in_addr), Some(in_val)) = (codes.next(), codes.next()) {
            let (out_addr, out_val) = armax::decrypt::decrypt_pair((*in_addr, *in_val), seeds);

            assert_eq!(out_addr, 2169439932);
            assert_eq!(out_val, 678980011);

        }
        else {
            assert!(false);
        }

        assert!(game.cheats.len() > 0);
        assert!(game.cheats[0].codes.len() > 0);


    }

    // Decrypt single ActionReplay MAX cheat
    #[test]
    fn armax_decrypt_cheat() {
        // Default state
        let state: omniconvert::State = omniconvert::State::new();

        // Tokenize input
        let tokens = omniconvert::read_input(TEST_CHEAT_SINGLE, state.incrypt.code.format);

        // Parse input into cheats
        let encrypted_cheats: Vec<Cheat> = omniconvert::build_cheat_list(tokens);

        // Decrypt
        let decrypted_cheats: Vec<Cheat> = encrypted_cheats
            .into_iter()
            .map(|cheat| {
                armax::decrypt::decrypt_cheat(cheat, &state.armax_seeds, &state.ar2_seeds)
            })
            .collect();

        // Sanity check
        assert_eq!(decrypted_cheats.len(), 1);
        assert_eq!(decrypted_cheats[0].codes.len(), 10);

        // Cheat metadata
        assert_eq!(decrypted_cheats[0].enable_code, true);
        assert_eq!(decrypted_cheats[0].game_id, 0x029E);

        // Verifier lines
        assert_eq!(decrypted_cheats[0].codes.split_at(4).0, vec!(0x014F06BC, 0x287869AB, 0x74680000, 0x00000000));

        // Full code
        assert_eq!(decrypted_cheats[0].codes, vec!(0x014F06BC, 0x287869AB, 0x74680000, 0x00000000, 0xC411F668, 0x00000800, 0x0C0F0094, 0x00000001, 0xC4000000, 0x00010801))
    }

    // Decrypt multiple ActionReplay MAX cheats
    #[test]
    fn armax_decrypt_game() {
        // Default state
        let state: omniconvert::State = omniconvert::State::new();

        // Dummy game object
        let mut game: Game = Game {
            id: 0,
            name: "Kingdom Hearts".to_string(),
            cheats: vec![],
            region: Region::Unknown
        };

        // Tokenize input
        let tokens = omniconvert::read_input(TEST_CHEAT_MULTIPLE, state.incrypt.code.format);

        // Build list of encrypted cheats, then decrypt and add to game
        game.cheats = omniconvert::build_cheat_list(tokens)
            .into_iter()
            .map(|cheat| {
                armax::decrypt::decrypt_cheat(cheat, &state.armax_seeds, &state.ar2_seeds)
            })
            .collect();

        // Pull Game metadata from first cheat
        game.id = game.cheats[0].game_id;
        game.region = match game.cheats[0].region {
            0 => Region::USA,
            1 => Region::PAL,
            2 => Region::Japan,
            _ => Region::Unknown
        };

        // Game
        assert_eq!(game.id, 0x029E);
        assert_eq!(game.region, Region::USA);
        assert_eq!(game.cheats.len(), 3);

        // Cheat codes
        assert_eq!(game.cheats[0].enable_code, true);           // Enable code flag
        assert_eq!(game.cheats[0].name, "Enable Code");         // Cheat name
        assert_eq!(                                             // Verifier
            game.cheats[0].codes.split_at(4).0,
            vec!(0x014F06BC, 0x287869AB, 0x74680000, 0x00000000)
        );
        assert_eq!(game.cheats[0].codes, vec!(0x014F06BC, 0x287869AB, 0x74680000, 0x00000000, 0xC411F668, 0x00000800, 0x0C0F0094, 0x00000001, 0xC4000000, 0x00010801));
        assert_eq!(game.cheats[1].enable_code, false);          // Enable code flag
        assert_eq!(game.cheats[1].name, "Have All Trinities");  // Cheat name
        assert_eq!(                                             // Verifier
            game.cheats[1].codes.split_at(2).0,
            vec!(0x014F06BC, 0x50800000)
        );
        assert_eq!(game.cheats[1].codes, vec!(0x014F06BC, 0x50800000, 0x003F38AB, 0x0000007F));
        assert_eq!(game.cheats[2].enable_code, false);          // Enable code flag
        assert_eq!(game.cheats[2].name, "Save Anywhere");       // Cheat name
        //assert_eq!(game.cheats[2].comment, "Press pause to access the menu");
        assert_eq!(                                             // Verifier
            game.cheats[2].codes.split_at(2).0,
            vec!(0x014F06BC, 0x60800000)
        );
        assert_eq!(game.cheats[2].codes, vec!(0x014F06BC, 0x60800000, 0x044865E0, 0x00114288));

    }
}


