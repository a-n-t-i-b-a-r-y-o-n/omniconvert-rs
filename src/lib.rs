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


