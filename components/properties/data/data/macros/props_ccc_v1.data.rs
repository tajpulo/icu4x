// @generated
/// Implement `DataProvider<CanonicalCombiningClassV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_ccc_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.66"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_CCC_V1: &'static <icu::properties::provider::CanonicalCombiningClassV1Marker as icu_provider::DataMarker>::Yokeable = &icu::properties::provider::PropertyCodePointMapV1::CodePointTrie(icu::collections::codepointtrie::CodePointTrie::from_parts(icu::collections::codepointtrie::CodePointTrieHeader { high_start: 125440u32, shifted12_high_start: 31u16, index3_null_offset: 336u16, data_null_offset: 0u32, null_value: 0u32, trie_type: icu::collections::codepointtrie::TrieType::Small }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\0@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\0\xBD\0\0\0\0\0\0\0\0\0\xFA\0\0\0\0\0\0\0)\x01i\x01\x99\x01\xCE\x01\0\0\xFF\x01.\x02m\x02\0\0\x82\x02\xC0\x02\xEE\x02\x16\x03L\x03\x8C\x03\xC9\x03\x8C\x03\xFC\x03\x8C\x03;\x04\x8C\x03;\x04\x8C\x03;\x04\0\0;\x04\x8C\x03n\x04\x8C\x03;\x04\x85\x04;\x04\0\0\xC2\x04\xCD\x04\x08\x05\x14\x05O\x05w\x05\xB1\x05\xF1\x05+\x06\xD9\x04\xF2\x04\xFF\x04\x15\x055\x05E\x05]\x05|\x05\0\0\x10\0 \x000\0@\0P\0`\0p\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\x80\0\x90\0\xA0\0\xB0\0\xBD\0\xCD\0\xDD\0\xED\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\xFA\0\n\x01\x1A\x01*\x01\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0)\x019\x01I\x01Y\x01i\x01y\x01\x89\x01\x99\x01\xA9\x01\xB9\x01\xC9\x01\xCE\x01\xDE\x01\xEE\x01\xFE\x01\0\0\x10\0 \x000\0\xFF\x01\x0F\x02\x1F\x02/\x02.\x02>\x02N\x02^\x02m\x02}\x02\x8D\x02\x9D\x02\0\0\x10\0 \x000\0\x82\x02\x92\x02\xA2\x02\xB2\x02\xC0\x02\xD0\x02\xE0\x02\xF0\x02\xEE\x02\xFE\x02\x0E\x03\x1E\x03\x16\x03&\x036\x03F\x03L\x03\\\x03l\x03|\x03\x8C\x03\x9C\x03\xAC\x03\xBC\x03\xC9\x03\xD9\x03\xE9\x03\xF9\x03\x8C\x03\x9C\x03\xAC\x03\xBC\x03\xFC\x03\x0C\x04\x1C\x04,\x04\x8C\x03\x9C\x03\xAC\x03\xBC\x03;\x04K\x04[\x04k\x04\x8C\x03\x9C\x03\xAC\x03\xBC\x03;\x04K\x04[\x04k\x04\x8C\x03\x9C\x03\xAC\x03\xBC\x03;\x04K\x04[\x04k\x04\0\0\x10\0 \x000\0;\x04K\x04[\x04k\x04\x8C\x03\x9C\x03\xAC\x03\xBC\x03n\x04~\x04\x8E\x04\x9E\x04\x8C\x03\x9C\x03\xAC\x03\xBC\x03;\x04K\x04[\x04k\x04\x85\x04\x95\x04\xA5\x04\xB5\x04;\x04K\x04[\x04k\x04\0\0\x10\0 \x000\0\xC2\x04\xD2\x04\xE2\x04\xF2\x04\xCD\x04\xDD\x04\xED\x04\xFD\x04\x08\x05\x18\x05(\x058\x05\x14\x05$\x054\x05D\x05O\x05_\x05o\x05\x7F\x05w\x05\x87\x05\x97\x05\xA7\x05\xB1\x05\xC1\x05\xD1\x05\xE1\x05\xF1\x05\x01\x06\x11\x06!\x06+\x06;\x06K\x06[\x06\0\0\0\0\0\0d\x06\0\0\0\0\0\0\0\0$\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xBC\x04\0\0\x05\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0r\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8A\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x96\x06\0\0\0\0\0\0\0\0\t\x04\xA1\x06\0\0\0\0\0\0\xB1\x06\xC0\x06\0\0\0\0\0\0\0\0\0\0\0\0\xC4\x03\x05\x04\0\0\xCF\x06\xE9\0\0\0\0\0\xB6\x04\0\0\0\0\0\0\xC2\x03\xDF\x06\0\0\0\0\0\0\xC1\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xEF\x06\xFF\x06\r\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1D\x07-\x07\x80\x007\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0G\x07V\x07\xEC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0q\0\xEB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFA\x03\0\0\0\0\0\0\0\0\0\0\0\0\x80\0\x80\0\0\0\0\0f\x07\0\0\0\0\0\0\0\0\0\0\0\0v\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0q\0\x82\x07\0\0r\0\0\0\0\0\0\0\0\0\0\0\xEB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x04\0\0\xCA\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05\x04\0\0\x80\0\xEB\0\0\0\0\0\xFC\x02\0\0\0\0\x06\x04\0\0\0\0\0\0\0\0\0\0\xC5\x03\t\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x07\xA2\x07\0\0\0\0\x03\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xC9\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xA4\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xB4\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0$\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xED\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xC4\x07\0\0\0\0\xD4\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xE4\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xF0\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xF8\x07\0\0\0\0\0\0\0\0\xFA\x02\0\0\0\0\0\0\0\0\x05\x08\xED\x01\0\0\0\0\x15\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x04\0\0\0\0%\x08\0\0\0\0\0\0+\x08\0\0\0\0\0\0\0\0\xEA\0\0\0\0\08\x08\0\0\0\0\0\0\xC5\x03\0\0\0\0\0\0\0\0H\x08\0\0\0\0\0\0S\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Z\x08\0\0\0\0\0\0\0\0e\x08\xC9\x03\0\0r\x08\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x08,\x04\0\0\0\0\0\0\0\0\0\0\0\0\x8E\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFA\x03\x91\x08\0\0\0\0\0\0\xFA\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8A\x08\0\0\0\0\0\0\0\0\0\0\0\0\xFE\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0+\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xB3\x04\xC5\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\t\x04\0\0\0\0\0\0\x05\x04\x02\x04\0\0\0\0\0\0\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFA\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9F\x08\0\0\0\0\0\0\0\0\x02\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xA2\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xB2\x08\0\0\0\0\0\0\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xC2\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xC4\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xD3\x08\xE1\x08\xEE\x08\0\0\xFA\x08\0\0\0\0\0\0\0\0\0\0\x08\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x18\t \t.\t\0\0\0\0\0\0\0\0\0\0q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0,\x04\0\0\0\0\0\0t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\09\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0I\t\0\0\0\0\0\0\0\0\0\0\0\0U\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0H\0X\0p\0\x8F\0\xAF\0\xCF\0\xEF\0\x0F\x01/\x01:\x01P\x01_\x01}\x01\x9C\x01\xBC\x01P\x01\xDC\x01P\x01P\x01P\x01P\x01P\x01\xEE\x01P\x01\x0E\x02P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01(\x02H\x02e\x02P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01\x84\x02P\x01\xA2\x02\xA5\x02\xC5\x02P\x01P\x01P\x01\xE5\x02\xF4\x02\n\x03&\x03C\x03_\x03|\x03\x99\x03\xB8\x03\xD5\x03\xEF\x03P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01\x04\x04P\x01\x18\x04P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x018\x04P\x01P\x01P\x01P\x01P\x01P\x01P\x01P\x01B\x04^\x04P\x01P\x01P\x01P\x01P\x01P\x01~\x04\x94\x04\xA6\x04P\x01\xB9\x04") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE8\xDC\xDC\xDC\xDC\xE8\xD8\xDC\xDC\xDC\xDC\xDC\xCA\xCA\xDC\xDC\xDC\xDC\xCA\xCA\xDC\xDC\xDC\xDC\xDC\xDC\xDC\xDC\xDC\xDC\xDC\x01\x01\x01\x01\x01\xDC\xDC\xDC\xDC\xE6\xE6\xE6\xE6\xE6\xF0\xE6\xDC\xDC\xDC\xE6\xE6\xE6\xDC\xDC\0\xE6\xE6\xE6\xDC\xDC\xDC\xDC\xE6\xE8\xDC\xDC\xE6\xE9\xEA\xEA\xE9\xEA\xEA\xE9\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xE6\xE6\xE6\xE6\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xDC\xE6\xE6\xE6\xE6\xDC\xE6\xE6\xE6\xDE\xDC\xE6\xE6\xE6\xE6\xE6\xE6\xDC\xDC\xDC\xDC\xDC\xDC\xE6\xE6\xDC\xE6\xE6\xDE\xE4\xE6\n\x0B\x0C\r\x0E\x0F\x10\x11\x12\x13\x13\x14\x15\x16\0\x17\0\x18\x19\0\xE6\xDC\0\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\x1E\x1F \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1B\x1C\x1D\x1E\x1F !\"\xE6\xE6\xDC\xDC\xE6\xE6\xE6\xE6\xE6\xDC\xE6\xE6\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xE6\xE6\xE6\xE6\xE6\xE6\xE6\0\0\xE6\xE6\xE6\xE6\xDC\xE6\0\0\xE6\xE6\0\xDC\xE6\xE6\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xE6\xDC\xE6\xE6\xDC\xE6\xE6\xDC\xDC\xDC\xE6\xDC\xDC\xE6\xDC\xE6\xE6\xDC\xE6\xDC\xE6\xDC\xE6\xDC\xE6\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xDC\xE6\0\0\0\0\0\0\0\0\0\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xE6\xE6\xE6\xE6\0\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\0\xE6\xE6\xE6\0\xE6\xE6\xE6\xE6\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xDC\xDC\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xE6\xDC\xDC\xDC\xE6\xE6\xE6\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xE6\xE6\xE6\xE6\xE6\xDC\xDC\xDC\xDC\xDC\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\0\xDC\xE6\xE6\xDC\xE6\xE6\xDC\xE6\xE6\xE6\xDC\xDC\xDC\x1B\x1C\x1D\xE6\xE6\xE6\xDC\xE6\xE6\xDC\xDC\xE6\xE6\xE6\xE6\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\t\0\0\0\xE6\xDC\xE6\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\t\0\0\0\0\0\0\0T[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\t\t\0\0\0\0\0\0\0\0\0\0\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0gg\t\0\0\0\0\0\0\0\0kkkk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0vv\t\0\0\0\0\0\0\0\0zzzz\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xDC\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xDC\0\xDC\0\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x82\0\x84\0\0\0\0\0\x82\x82\x82\x82\0\0\x82\0\xE6\xE6\t\0\xE6\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x07\0\t\t\0\0\0\0\0\t\0\0\0\0\0\0\0\0\0\0\xE6\0\0\0\0\0\0\0\0\0\xE4\0\0\0\0\0\0\0\0\0\xDE\xE6\xDC\0\0\0\0\0\0\0\xE6\xDC\0\0\0\0\0\0\0\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\0\0\xDC\xE6\xE6\xE6\xE6\xE6\xDC\xDC\xDC\xDC\xDC\xDC\xE6\xE6\xDC\0\xDC\xE6\xE6\xDC\xDC\xE6\xE6\xE6\xE6\xE6\xDC\xE6\xE6\xE6\xE6\0\0\0\0\0\0\0\0\0\0\0\xE6\xDC\xE6\xE6\xE6\0\0\t\t\0\0\0\0\0\0\0\0\0\0\0\0\xE6\xE6\xE6\0\x01\xDC\xDC\xDC\xDC\xDC\xE6\xE6\xDC\xDC\xDC\xDC\xE6\0\x01\x01\x01\x01\x01\x01\x01\0\0\0\0\xDC\0\0\0\0\xE6\0\0\0\xE6\xE6\0\0\0\0\0\0\xE6\xE6\xDC\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xDC\xE6\xE6\xEA\xD6\xDC\xCA\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE8\xE4\xE4\xDC\xDA\xE6\xE9\xDC\xE6\xDC\xE6\xE6\x01\x01\xE6\xE6\xE6\xE6\x01\x01\x01\xE6\xE6\0\0\0\xE6\0\0\0\x01\x01\xE6\xDC\xE6\x01\x01\xDC\xDC\xDC\xDC\0\0\0\0\0\0\0\0\0\0\xDA\xE4\xE8\xDE\xE0\xE0\0\0\0\0\0\0\0\0\0\x08\x08\0\0\0\0\0\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\0\0\xE6\0\xE6\xE6\xDC\0\0\xE6\xE6\0\0\0\0\0\xE6\xE6\0\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1A\0\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xDC\xDC\xDC\xDC\xDC\xDC\xDC\xE6\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\xDC\0\xE6\0\0\0\0\0\0\0\0\xE6\x01\xDC\0\0\0\0\t\0\0\0\0\0\xE6\xDC\0\0\0\0\0\0\0\0\0\xE6\xE6\xE6\xE6\0\0\0\0\0\0\0\0\0\0\0\xE6\xE6\0\0\0\0\0\0\xDC\xDC\xE6\xE6\xE6\xDC\xE6\xDC\xDC\xDC\0\0\xE6\xDC\xE6\xDC\0\0\0\0\0\0\0\0\0\0\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\t\x07\0\0\0\0\0\t\t\0\0\0\0\0\0\0\0\0\0\0\t\0\0\0\0\0\0\0\0\0\x07\0\0\0\0\0\t\x07\0\0\0\0\0\0\0\0\0\x07\t\0\0\0\0\0\0\0\0\0\0\0\x07\x07\0\0\0\0\0\0\xE6\xE6\xE6\xE6\xE6\xE6\xE6\0\0\0\t\0\0\0\x07\0\0\0\0\0\0\0\0\0\t\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x07\0\t\t\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\0\0\0\0\0\0\0\0\0\0\0\x06\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\0\0\xD8\xD8\x01\x01\x01\0\0\0\xE2\xD8\xD8\xD8\0\0\0\0\0\0\0\0\xDC\xDC\xDC\xDC\xDC\0\0\xE6\xE6\xE6\xE6\xE6\xDC\xDC\0\0\0\0\0\0\0\0\0\0\xE6\xE6\xE6\xE6\0\0\xE6\xE6\xE6\0\0\0\0\0\0\0\0\0\0\0\xE6\xE6\xE6\xE6\xE6\xE6\xE6\0\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\xE6\0\0\xE6\xE6\xE6\xE6\xE6\0\xE6\xE6\0\xE6\xE6\xE6\xE6\xE6\0\0\0\0\0\0\0\0\0\0\0\0\xE8\xE8\xDC\xE6\xDC\xDC\xDC\xDC\xDC\xDC\xDC\0\0\0\0\0\0\0\0\0\xE6\xE6\xE6\xE6\xE6\xE6\x07\0\0\0\0\0\0") }, icu::properties::CanonicalCombiningClass(0u8)));
        }
        #[clippy::msrv = "1.66"]
        impl icu_provider::DataProvider<icu::properties::provider::CanonicalCombiningClassV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::CanonicalCombiningClassV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_CCC_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::properties::provider::CanonicalCombiningClassV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
