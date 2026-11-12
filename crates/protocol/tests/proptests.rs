use proptest::prelude::*;
use pyxillum_protocol::*;

proptest! {
    #[test]
    fn test_assembler_never_panics_on_arbitrary_bytes(bytes in prop::collection::vec(any::<u8>(), 0..64)) {
        let mut asm = StatusAssembler::new();
        let _ = asm.feed(&bytes);
    }

    #[test]
    fn test_color_roundtrip(r in any::<u8>(), g in any::<u8>(), b in any::<u8>(), level in any::<u8>()) {
        let cmd = Command::SetColor {
            color: RgbColor::new(r, g, b),
            level,
        };
        let encoded = cmd.encode();
        assert_eq!(encoded.len(), 7);
        assert_eq!(encoded[0], COMMAND_PREFIX);
        assert_eq!(encoded[1], 0x69);
        assert_eq!(encoded[2], 0x04);
        assert_eq!(encoded[3], r);
        assert_eq!(encoded[4], g);
        assert_eq!(encoded[5], b);
        assert_eq!(encoded[6], level);
    }
}
