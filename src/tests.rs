#[cfg(test)]
mod tests {
    use crate::phone_key_encoder_decoder;

    #[test]
    fn successful_encode_tests() {
        assert_eq!("83377778083377778", phone_key_encoder_decoder::encode(String::from("test test")).unwrap());

        assert_eq!("2.2.2.2022.22.22.220222.222.222.222", phone_key_encoder_decoder::encode(String::from("aaaa bbbb cccc")).unwrap());

        assert_eq!("2.22.22203.33.33304.44.44405.55.555", phone_key_encoder_decoder::encode(String::from("abc def ghi jkl")).unwrap());
    }

    #[test]
    fn failing_encode_tests() {
        assert!(phone_key_encoder_decoder::encode(String::from(". undefined char")).is_err());

        assert!(phone_key_encoder_decoder::encode(String::from("_ undefined char")).is_err());
    }

    #[test]
    fn successful_decode_tests() {
        assert_eq!("hello world", phone_key_encoder_decoder::decode(String::from("4433555.555666096667775553")).unwrap());

        assert_eq!("tttteeeesssstttt", phone_key_encoder_decoder::decode(String::from("8.8.8.833.33.33.337777.7777.7777.77778.8.8.8")).unwrap());

        assert_eq!("rust is awesome", phone_key_encoder_decoder::decode(String::from("777887777804447777029337777666.633")).unwrap());
    }

    #[test]
    fn failing_decode_tests() {
        assert!(phone_key_encoder_decoder::decode(String::from("12345678")).is_err());

        assert!(phone_key_encoder_decoder::decode(String::from("9012.3456")).is_err());

        assert!(phone_key_encoder_decoder::decode(String::from("44-22.5")).is_err());
    }
}