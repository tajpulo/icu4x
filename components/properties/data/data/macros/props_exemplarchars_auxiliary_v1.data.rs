// @generated
/// Implement `DataProvider<ExemplarCharactersAuxiliaryV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_exemplarchars_auxiliary_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.66"]
        impl icu_provider::DataProvider<icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker>, icu_provider::DataError> {
                static FA: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\x06\0\0&\x06\0\0@\x06\0\0A\x06\0\0C\x06\0\0D\x06\0\0I\x06\0\0K\x06\0\0N\x06\0\0Q\x06\0\0R\x06\0\0S\x06\0\0V\x06\0\0W\x06\0\0p\x06\0\0q\x06\0\0\x0C \0\0\x10 \0\0") }, 15u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FA_AF: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"%\x06\0\0&\x06\0\0C\x06\0\0D\x06\0\0I\x06\0\0K\x06\0\0N\x06\0\0Q\x06\0\0R\x06\0\0S\x06\0\0V\x06\0\0W\x06\0\0p\x06\0\0q\x06\0\0|\x06\0\0}\x06\0\0\x81\x06\0\0\x82\x06\0\0\x85\x06\0\0\x86\x06\0\0\x89\x06\0\0\x8A\x06\0\0\x93\x06\0\0\x94\x06\0\0\x96\x06\0\0\x97\x06\0\0\x9A\x06\0\0\x9B\x06\0\0\xAB\x06\0\0\xAC\x06\0\0\xBC\x06\0\0\xBD\x06\0\0\x0C \0\0\x10 \0\0") }, 23u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SD: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"&\x06\0\0'\x06\0\0N\x06\0\0Q\x06\0\0") }, 4u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KS: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"&\x06\0\0'\x06\0\0N\x06\0\0Q\x06\0\0T\x06\0\0X\x06\0\0_\x06\0\0`\x06\0\0\x0E \0\0\x10 \0\0") }, 11u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static ZH_HANT: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"(N\0\0)N\0\x006N\0\x007N\0\0?N\0\0@N\0\0MN\0\0NN\0\0sN\0\0tN\0\0\x85N\0\0\x86N\0\0\xA0N\0\0\xA1N\0\0\xC2N\0\0\xC3N\0\0\x0FO\0\0\x10O\0\0PO\0\0QO\0\0\xB6O\0\0\xB7O\0\0\xCFO\0\0\xD0O\0\0\tP\0\0\nP\0\0}P\0\0~P\0\0\x85P\0\0\x86P\0\0\x98P\0\0\x99P\0\0\xF3P\0\0\xF4P\0\0?Q\0\0@Q\0\0FQ\0\0GQ\0\0LQ\0\0MQ\0\0yQ\0\0zQ\0\0\x82Q\0\0\x83Q\0\0\x96Q\0\0\x97Q\0\0\xABQ\0\0\xACQ\0\0\xCBQ\0\0\xCCQ\0\0\xCDQ\0\0\xCEQ\0\0\xE0Q\0\0\xE1Q\0\0\xF5Q\0\0\xF6Q\0\0\xF8Q\0\0\xF9Q\0\0\x12R\0\0\x13R\0\0(R\0\0)R\0\0+R\0\0,R\0\0.R\0\0/R\0\08R\0\09R\0\0CR\0\0DR\0\0\xF3R\0\0\xF4R\0\0\xF9R\0\0\xFAR\0\0\xFER\0\0\xFFR\0\0\x15S\0\0\x16S\0\0\x19S\0\0\x1BS\0\0#S\0\0$S\0\0/S\0\x000S\0\08S\0\09S\0\0QS\0\0RS\0\0^S\0\0_S\0\0`S\0\0aS\0\0iS\0\0jS\0\0yS\0\0zS\0\0\x82S\0\0\x83S\0\0\xB6S\0\0\xB7S\0\0\xC9S\0\0\xCAS\0\0\xF6S\0\0\xF7S\0\0;T\0\0<T\0\0\xFAT\0\0\xFBT\0\0\x07U\0\0\x08U\0\x005U\0\x006U\0\0dU\0\0eU\0\0\xAAU\0\0\xABU\0\0\xB2U\0\0\xB3U\0\0\x1FV\0\0 V\0\0AV\0\0BV\0\0SV\0\0TV\0\0XV\0\0YV\0\0\x8FV\0\0\x90V\0\0\xD7V\0\0\xD8V\0\0QW\0\0RW\0\0$X\0\0%X\0\0\x85X\0\0\x86X\0\0\x8EX\0\0\x8FX\0\0\x93X\0\0\x94X\0\0\x9FX\0\0\xA0X\0\0\xB3X\0\0\xB4X\0\0\xE4X\0\0\xE5X\0\0\xE9X\0\0\xEAX\0\0\xFAX\0\0\xFBX\0\0\x02Y\0\0\x03Y\0\0\nY\0\0\x0BY\0\0eY\0\0fY\0\0\x96Y\0\0\x97Y\0\x000[\0\x001[\0\0U[\0\0V[\0\0\\[\0\0][\0\0u[\0\0v[\0\0\x80[\0\0\x81[\0\0\xF8[\0\0\xF9[\0\0\xFA[\0\0\xFB[\0\0\"\\\0\0#\\\0\08\\\0\09\\\0\0?\\\0\0@\\\0\0M\\\0\0N\\\0\0Q\\\0\0R\\\0\0n\\\0\0o\\\0\0\xC7\\\0\0\xC8\\\0\0\xBC]\0\0\xBD]\0\0\xDB]\0\0\xDC]\0\0\xFD]\0\0\xFF]\0\0\x06^\0\0\x07^\0\0\x1A^\0\0\x1B^\0\0_^\0\0`^\0\0z^\0\0{^\0\0\x7F^\0\0\x80^\0\0\xC1^\0\0\xC2^\0\0\xC8^\0\0\xC9^\0\0\xDA^\0\0\xDB^\0\0\xDF^\0\0\xE0^\0\0\xF4^\0\0\xF5^\0\0\xFE^\0\0\xFF^\0\0\x0B_\0\0\x0C_\0\0\x13_\0\0\x14_\0\0P_\0\0Q_\0\0a_\0\0b_\0\0s_\0\0t_\0\0\xE1_\0\0\xE2_\0\0\x8Aa\0\0\x8Ba\0\0\xE8a\0\0\xE9a\0\0\xF8a\0\0\xF9a\0\0\x1Fb\0\0 b\0\0nb\0\0ob\0\0sb\0\0tb\0\0Bc\0\0Cc\0\0Oc\0\0Pc\0\0gc\0\0hc\0\0\xA0c\0\0\xA1c\0\0\xB0c\0\0\xB1c\0\0\xF9c\0\0\xFAc\0\0\x0Fd\0\0\x10d\0\0@d\0\0Ad\0\0Td\0\0Ud\0\0\x95d\0\0\x96d\0\0\xB2d\0\0\xB3d\0\0\0e\0\0\x01e\0\0$e\0\0%e\0\x004e\0\x005e\0\0^e\0\0_e\0\0\x91e\0\0\x92e\0\0\x9Ce\0\0\x9De\0\0\xA4e\0\0\xA5e\0\0\xA7e\0\0\xA8e\0\0\xE0e\0\0\xE1e\0\0\x88f\0\0\x89f\0\0\xAEf\0\0\xAFf\0\0\xC7f\0\0\xC8f\0\0\xECf\0\0\xEDf\0\0\xF3f\0\0\xF4f\0\0\x14g\0\0\x15g\0\0Vg\0\0Wg\0\0\xAFg\0\0\xB0g\0\0\x13h\0\0\x14h\0\0\x17h\0\0\x18h\0\0=h\0\0>h\0\0Fh\0\0Gh\0\0vh\0\0wh\0\0\x7Fh\0\0\x80h\0\0\xCDh\0\0\xCEh\0\0\xD5h\0\0\xD6h\0\0\xFAh\0\0\xFBh\0\0\x12i\0\0\x13i\0\0Ti\0\0Ui\0\0\xCCi\0\0\xCDi\0\0Dj\0\0Ej\0\0Gj\0\0Hj\0\0Xj\0\0Zj\0\0\xACj\0\0\xADj\0\0\xB8j\0\0\xB9j\0\0\xC3j\0\0\xC4j\0\0\xDAj\0\0\xDBj\0\0\xFBj\0\0\xFCj\0\0\x16k\0\0\x17k\0\0 k\0\0!k\0\0yk\0\0zk\0\0\x8Bk\0\0\x8Ck\0\0\xADk\0\0\xAEk\0\0\xB3k\0\0\xB4k\0\0\xCBk\0\0\xCCk\0\0\x14l\0\0\x15l\0\0Al\0\0Bl\0\0\xABl\0\0\xACl\0\0\xAEl\0\0\xAFl\0\0\xE3l\0\0\xE4l\0\0cm\0\0dm\0\0tm\0\0um\0\0\x85m\0\0\x86m\0\0\x8Em\0\0\x8Fm\0\0\xAEm\0\0\xAFm\0\0\xC7m\0\0\xC8m\0\0\xCBm\0\0\xCCm\0\0>n\0\0?n\0\0Xn\0\0Yn\0\0\x9Cn\0\0\x9Dn\0\0?o\0\0@o\0\0\x8Eo\0\0\x8Fo\0\0\xA1o\0\0\xA2o\0\0\xD5o\0\0\xD6o\0\0Xp\0\0Yp\0\0\xD8p\0\0\xD9p\0\0\xF9p\0\0\xFAp\0\0\nq\0\0\x0Bq\0\0\x19q\0\0\x1Aq\0\x000q\0\x001q\0\0Nq\0\0Oq\0\0nq\0\0oq\0\0\xD5q\0\0\xD6q\0\0\xD9q\0\0\xDAq\0\0\xE6q\0\0\xE7q\0\0\xEDq\0\0\xEEq\0\0\rr\0\0\x0Er\0\0;r\0\0<r\0\0?r\0\0@r\0\0ar\0\0br\0\0\x80r\0\0\x81r\0\0\xACr\0\0\xADr\0\0\xC4r\0\0\xC5r\0\0\xE1r\0\0\xE2r\0\0\xF8r\0\0\xF9r\0\0)s\0\0*s\0\0>s\0\0@s\0\0zs\0\0{s\0\0~s\0\0\x7Fs\0\x003t\0\x004t\0\0Zt\0\0[t\0\0\xE2t\0\0\xE3t\0\0\x15u\0\0\x16u\0\0+u\0\0,u\0\0\x8Au\0\0\x8Cu\0\0\x92u\0\0\x93u\0\0\xB2u\0\0\xB3u\0\0\xBEu\0\0\xBFu\0\0&v\0\0(v\0\0vv\0\0wv\0\0\x82v\0\0\x83v\0\0\xBAv\0\0\xBBv\0\0\xBFv\0\0\xC0v\0\0\xC6v\0\0\xC7v\0\0\xC8v\0\0\xC9v\0\0\xD2v\0\0\xD3v\0\0\xD4v\0\0\xD5v\0\0\xE5v\0\0\xE6v\0\0(w\0\0*w\0\0Ow\0\0Pw\0\0\x87w\0\0\x88w\0\0\x8Cw\0\0\x8Dw\0\0\xAAw\0\0\xABw\0\0\xE2w\0\0\xE3w\0\0\x91x\0\0\x92x\0\0\xDAx\0\0\xDBx\0\0\x01y\0\0\x02y\0\0+y\0\0,y\0\0Hy\0\0Iy\0\0\xB1y\0\0\xB2y\0\0\xB8y\0\0\xB9y\0\0\xBEy\0\0\xC0y\0\0;z\0\0<z\0\0@z\0\0Az\0\0tz\0\0uz\0\0\x84z\0\0\x85z\0\0\xFFz\0\0\0{\0\0R{\0\0S{\0\0w{\0\0x{\0\0\x8F{\0\0\x90{\0\0\x94{\0\0\x95{\0\0\xF7{\0\0\xF8{\0\0\r|\0\0\x0E|\0\0`|\0\0a|\0\0\xD6|\0\0\xD7|\0\0\xF0|\0\0\xF1|\0\0\xF8|\0\0\xF9|\0\0\t}\0\0\n}\0\0\x0B}\0\0\x0C}\0\0\x17}\0\0\x18}\0\0.}\0\0/}\0\x003}\0\x004}\0\0\xBD}\0\0\xBE}\0\0\xBF}\0\0\xC0}\0\0+~\0\0,~\0\0C~\0\0D~\0\0a~\0\0b~\0\0i~\0\0j~\0\0\x8F~\0\0\x90~\0\0\x96~\0\0\x97~\0\0\x9C~\0\0\x9D~\0\x006\x7F\0\x007\x7F\0\0H\x7F\0\0I\x7F\0\0P\x7F\0\0R\x7F\0\0i\x7F\0\0j\x7F\0\0\xAF\x7F\0\0\xB0\x7F\0\0\x12\x80\0\0\x13\x80\0\0s\x80\0\0t\x80\0\0~\x80\0\0\x80\x80\0\0\x8C\x80\0\0\x8D\x80\0\0\x96\x80\0\0\x97\x80\0\0\xBA\x80\0\0\xBB\x80\0\0\x08\x81\0\0\t\x81\0\0\x16\x81\0\0\x17\x81\0\0P\x81\0\0Q\x81\0\0y\x81\0\0z\x81\0\0\x9A\x81\0\0\x9B\x81\0\0\xA0\x81\0\0\xA1\x81\0\0\xC2\x81\0\0\xC3\x81\0\0\xDF\x81\0\0\xE0\x81\0\0\xFC\x81\0\0\xFD\x81\0\0\x1B\x82\0\0\x1C\x82\0\0G\x82\0\0H\x82\0\0n\x82\0\0o\x82\0\0x\x82\0\0y\x82\0\0\x92\x82\0\0\x93\x82\0\0\x99\x82\0\0\x9A\x82\0\0\xAD\x82\0\0\xAE\x82\0\0\xBD\x82\0\0\xBE\x82\0\0\xD7\x82\0\0\xD8\x82\0\0\xE3\x82\0\0\xE4\x82\0\0\x04\x83\0\0\x05\x83\0\0(\x83\0\0)\x83\0\x005\x83\0\x006\x83\0\08\x83\0\09\x83\0\0\x93\x83\0\0\x94\x83\0\0\x96\x83\0\0\x97\x83\0\0\xC7\x83\0\0\xC8\x83\0\0\xCC\x83\0\0\xCD\x83\0\0\xF1\x83\0\0\xF2\x83\0\0\x0E\x84\0\0\x0F\x84\0\x005\x84\0\x006\x84\0\0u\x84\0\0v\x84\0\0\x9C\x84\0\0\x9D\x84\0\0\xB8\x84\0\0\xB9\x84\0\0\xC4\x84\0\0\xC5\x84\0\0\xC9\x84\0\0\xCA\x84\0\0\xEC\x84\0\0\xED\x84\0\0\x14\x85\0\0\x15\x85\0\0%\x85\0\0&\x85\0\0,\x85\0\0-\x85\0\0I\x85\0\0J\x85\0\0~\x85\0\0\x7F\x85\0\0\x91\x85\0\0\x92\x85\0\0\xAF\x85\0\0\xB0\x85\0\0\x0B\x86\0\0\x0C\x86\0\0\x11\x86\0\0\x12\x86\0\0?\x86\0\0@\x86\0\0M\x86\0\0N\x86\0\0k\x86\0\0l\x86\0\0y\x86\0\0z\x86\0\0\x8A\x86\0\0\x8B\x86\0\0\x93\x86\0\0\x94\x86\0\0\xA9\x86\0\0\xAA\x86\0\0\xAF\x86\0\0\xB0\x86\0\0\xDB\x86\0\0\xDC\x86\0\0\0\x87\0\0\x01\x87\0\0\x18\x87\0\0\x19\x87\0\0%\x87\0\0&\x87\0\x004\x87\0\x005\x87\0\0Y\x87\0\0Z\x87\0\0_\x87\0\0a\x87\0\0f\x87\0\0g\x87\0\0t\x87\0\0u\x87\0\0x\x87\0\0y\x87\0\0\x82\x87\0\0\x84\x87\0\0\x9E\x87\0\0\x9F\x87\0\0\xBA\x87\0\0\xBB\x87\0\0\xC0\x87\0\0\xC1\x87\0\0\xC4\x87\0\0\xC5\x87\0\0\xCB\x87\0\0\xCC\x87\0\0\xD1\x87\0\0\xD2\x87\0\0\xF3\x87\0\0\xF4\x87\0\0\xFB\x87\0\0\xFC\x87\0\0\x05\x88\0\0\x06\x88\0\0\x15\x88\0\0\x16\x88\0\0\x1F\x88\0\0 \x88\0\0#\x88\0\0$\x88\0\0k\x88\0\0l\x88\0\0\x8D\x88\0\0\x8E\x88\0\0\xCF\x88\0\0\xD0\x88\0\0\xD8\x88\0\0\xDA\x88\0\0\xF1\x88\0\0\xF2\x88\0\0\xF9\x88\0\0\xFA\x88\0\0\x10\x89\0\0\x11\x89\0\0j\x89\0\0k\x89\0\0o\x89\0\0p\x89\0\0~\x89\0\0\x7F\x89\0\0\x1D\x8A\0\0\x1E\x8A\0\0:\x8A\0\0;\x8A\0\0\x0E\x8B\0\0\x0F\x8B\0\0,\x8B\0\0-\x8B\0\0N\x8C\0\0O\x8C\0\0T\x8C\0\0V\x8C\0\0Z\x8C\0\0[\x8C\0\0x\x8C\0\0z\x8C\0\0\x1B\x8D\0\0\x1C\x8D\0\0\xC6\x8D\0\0\xC7\x8D\0\0\xE8\x8D\0\0\xE9\x8D\0\0\xEA\x8D\0\0\xEB\x8D\0\0)\x8E\0\0*\x8E\0\0\xAC\x8E\0\0\xAD\x8E\0\0\xF8\x8E\0\0\xF9\x8E\0\0N\x8F\0\0O\x8F\0\0\x9C\x8F\0\0\x9D\x8F\0\0\xA3\x8F\0\0\xA4\x8F\0\0\xB5\x8F\0\0\xB6\x8F\0\0^\x90\0\0_\x90\0\0\x91\x90\0\0\x92\x90\0\0\x19\x91\0\0\x1A\x91\0\0K\x91\0\0L\x91\0\0j\x91\0\0k\x91\0\0\xAC\x91\0\0\xAD\x91\0\0\xC6\x91\0\0\xC7\x91\0\0\xD8\x91\0\0\xD9\x91\0\0\x14\x92\0\0\x16\x92\0\0E\x92\0\0F\x92\0\0[\x92\0\0\\\x92\0\0d\x92\0\0e\x92\0\0\xC1\x92\0\0\xC2\x92\0\0(\x93\0\0)\x93\0\x006\x93\0\x007\x93\0\0J\x93\0\0K\x93\0\0\x9A\x93\0\0\x9B\x93\0\0\xAC\x93\0\0\xAD\x93\0\0\xC8\x93\0\0\xC9\x93\0\0\xE2\x93\0\0\xE3\x93\0\0:\x94\0\0;\x94\0\0p\x94\0\0q\x94\0\0}\x94\0\0~\x94\0\0\x7F\x94\0\0\x80\x94\0\0\xA9\x95\0\0\xAA\x95\0\0\x1C\x96\0\0\x1D\x96\0\x001\x96\0\x002\x96\0\0\xB4\x96\0\0\xB5\x96\0\0\xB6\x96\0\0\xB7\x96\0\0\xB9\x96\0\0\xBA\x96\0\0\xC0\x96\0\0\xC1\x96\0\0\xCC\x96\0\0\xCD\x96\0\0\x04\x97\0\0\x05\x97\0\0\x1C\x97\0\0\x1D\x97\0\0Q\x97\0\0R\x97\0\0Y\x97\0\0Z\x97\0\0t\x97\0\0u\x97\0\0\xA0\x97\0\0\xA1\x97\0\0\xAD\x97\0\0\xAE\x97\0\0\xED\x97\0\0\xEE\x97\0\0\x0C\x98\0\0\r\x98\0\08\x98\0\09\x98\0\0[\x98\0\0\\\x98\0\0\xB1\x98\0\0\xB2\x98\0\0\xC6\x98\0\0\xC7\x98\0\0\xEA\x98\0\0\xEB\x98\0\0\x03\x99\0\0\x04\x99\0\0\x0C\x99\0\0\r\x99\0\0\x1A\x99\0\0\x1B\x99\0\x005\x99\0\x006\x99\0\0>\x99\0\0?\x99\0\0\xDD\x99\0\0\xDE\x99\0\0\xF1\x99\0\0\xF2\x99\0\0U\x9A\0\0V\x9A\0\0\xB0\x9A\0\0\xB1\x9A\0\0\xB7\x9A\0\0\xB8\x9A\0\0\xCF\x9A\0\0\xD0\x9A\0\0\xDF\x9A\0\0\xE0\x9A\0\0\r\x9B\0\0\x0E\x9B\0\0/\x9B\0\x000\x9B\0\x002\x9B\0\x003\x9B\0\0w\x9B\0\0x\x9B\0\0\x91\x9B\0\0\x92\x9B\0\0\xC9\x9B\0\0\xCB\x9B\0\0\xE8\x9B\0\0\xE9\x9B\0\0w\x9C\0\0x\x9C\0\0\xE9\x9C\0\0\xEA\x9C\0\0\xF6\x9C\0\0\xF7\x9C\0\0(\x9D\0\0)\x9D\0\0a\x9D\0\0b\x9D\0\0\xB4\x9D\0\0\xB5\x9D\0\0\x1A\x9E\0\0\x1B\x9E\0\0u\x9E\0\0v\x9E\0\0}\x9E\0\0~\x9E\0\0\xCD\x9E\0\0\xCE\x9E\0\0\xDB\x9E\0\0\xDC\x9E\0\0\xF9\x9E\0\0\xFA\x9E\0\0\xFD\x9E\0\0\xFE\x9E\0\0\x0E\x9F\0\0\x0F\x9F\0\0,\x9F\0\0-\x9F\0\0\x90\x9F\0\0\x91\x9F\0\0\xA0\x9F\0\0\xA1\x9F\0\0") }, 495u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KY: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"2\x04\0\x003\x04\0\0D\x04\0\0E\x04\0\0F\x04\0\0G\x04\0\0I\x04\0\0J\x04\0\0L\x04\0\0M\x04\0\0") }, 5u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FF_ADLM: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b">\xE9\x01\0D\xE9\x01\0") }, 6u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AR: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"@\x06\0\0A\x06\0\0o\x06\0\0p\x06\0\0~\x06\0\0\x7F\x06\0\0\x86\x06\0\0\x87\x06\0\0\x98\x06\0\0\x99\x06\0\0\x9C\x06\0\0\x9D\x06\0\0\xA2\x06\0\0\xA3\x06\0\0\xA4\x06\0\0\xA6\x06\0\0\xA7\x06\0\0\xAA\x06\0\0\xAF\x06\0\0\xB0\x06\0\0\xCC\x06\0\0\xCD\x06\0\0\x0C \0\0\x10 \0\0") }, 18u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HI: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"D\t\0\0E\t\0\0\x0C \0\0\x0E \0\0") }, 3u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TG: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"F\x04\0\0G\x04\0\0I\x04\0\0J\x04\0\0K\x04\0\0M\x04\0\0") }, 4u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static UK: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"J\x04\0\0L\x04\0\0M\x04\0\0N\x04\0\0Q\x04\0\0R\x04\0\0") }, 4u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\n\0\0\0\0\0\x04\0\x08\0\x0C\0\x10\0\x14\0\x18\0\x1C\0 \0$\0\xD0\xB0\xCC\x81\xD0\xB5\xCC\x81\xD0\xB8\xCC\x81\xD0\xBE\xCC\x81\xD1\x83\xCC\x81\xD1\x8E\xCC\x81\xD1\x8F\xCC\x81\xD1\x94\xCC\x81\xD1\x96\xCC\x81\xD1\x97\xCC\x81") },
                ));
                static BG: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"K\x04\0\0L\x04\0\0M\x04\0\0N\x04\0\0P\x04\0\0R\x04\0\0]\x04\0\0^\x04\0\0c\x04\0\0d\x04\0\0k\x04\0\0l\x04\0\0") }, 7u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x04\0\x08\0\x0C\0\x10\0\x14\0\xD0\xB0\xCC\x80\xD0\xBE\xCC\x80\xD1\x83\xCC\x80\xD1\x8A\xCC\x80\xD1\x8E\xCC\x80\xD1\x8F\xCC\x80") },
                ));
                static YUE_HANS: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"MN\0\0NN\0\0\xC2N\0\0\xC3N\0\0\x0FO\0\0\x10O\0\0PO\0\0QO\0\0\xA3O\0\0\xA4O\0\0\xF3P\0\0\xF4P\0\0FQ\0\0GQ\0\0QQ\0\0RQ\0\08R\0\09R\0\0\xCBR\0\0\xCCR\0\0QS\0\0RS\0\0^S\0\0_S\0\0\x80T\0\0\x81T\0\0\x05V\0\0\x06V\0\0$X\0\0%X\0\0\x8EX\0\0\x8FX\0\0\xE4X\0\0\xE5X\0\0\\[\0\0][\0\0\x7F\\\0\0\x80\\\0\0\xC7\\\0\0\xC8\\\0\0\xFD]\0\0\xFE]\0\0\x9Ce\0\0\x9De\0\0\x19f\0\0\x1Af\0\0<f\0\0=f\0\0\x17h\0\0\x18h\0\0Ti\0\0Ui\0\0Qm\0\0Rm\0\0\x85m\0\0\x86m\0\0Xn\0\0Yn\0\0\x8Eo\0\0\x8Fo\0\0\x7Fp\0\0\x80p\0\0\xC4r\0\0\xC5r\0\x003t\0\x004t\0\0Zt\0\0[t\0\0+u\0\0,u\0\0\x91x\0\0\x92x\0\0\x01y\0\0\x02y\0\0\xF0~\0\0\xF1~\0\0\x92\x82\0\0\x93\x82\0\0\xD7\x82\0\0\xD8\x82\0\0(\x83\0\0)\x83\0\x005\x83\0\x006\x83\0\0\xEC\x84\0\0\xED\x84\0\0\xA9\x86\0\0\xAA\x86\0\0\xF0\x86\0\0\xF1\x86\0\0\0\x87\0\0\x01\x87\0\0\xD8\x88\0\0\xD9\x88\0\0,\x8C\0\0-\x8C\0\0c\x8D\0\0d\x8D\0\0K\x91\0\0L\x91\0\0\xFD\x95\0\0\xFE\x95\0\0G\x96\0\0H\x96\0\0\x1C\x97\0\0\x1D\x97\0\0") }, 53u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static YUE: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"MN\0\0NN\0\0\xC2N\0\0\xC3N\0\0\x0FO\0\0\x10O\0\0PO\0\0QO\0\0\xB6O\0\0\xB7O\0\0\xF3P\0\0\xF4P\0\0FQ\0\0GQ\0\0LQ\0\0MQ\0\0yQ\0\0zQ\0\0\xF8Q\0\0\xF9Q\0\0+R\0\0,R\0\08R\0\09R\0\0\xF3R\0\0\xF4R\0\0QS\0\0RS\0\0^S\0\0_S\0\0`S\0\0aS\0\0\xF6S\0\0\xF7S\0\0\x05V\0\0\x06V\0\0$X\0\0%X\0\0\x8EX\0\0\x8FX\0\0\xE4X\0\0\xE5X\0\0eY\0\0fY\0\0\\[\0\0][\0\0\xC7\\\0\0\xC8\\\0\0\xBC]\0\0\xBD]\0\0\xFD]\0\0\xFE]\0\0\x17h\0\0\x18h\0\0Ti\0\0Ui\0\0\x85m\0\0\x86m\0\0>n\0\0?n\0\0\x8Eo\0\0\x8Fo\0\0Xp\0\0Yp\0\0\xE6q\0\0\xE7q\0\0\xC4r\0\0\xC5r\0\x003t\0\x004t\0\0Zt\0\0[t\0\0+u\0\0,u\0\0\x91x\0\0\x92x\0\0\x01y\0\0\x02y\0\0\x9C~\0\0\x9D~\0\0G\x82\0\0H\x82\0\0\x92\x82\0\0\x93\x82\0\0\xD7\x82\0\0\xD8\x82\0\0(\x83\0\0)\x83\0\0\xEC\x84\0\0\xED\x84\0\0\xA9\x86\0\0\xAA\x86\0\0\0\x87\0\0\x01\x87\0\0\xD8\x88\0\0\xD9\x88\0\0,\x8B\0\0-\x8B\0\0K\x91\0\0L\x91\0\0\xB4\x96\0\0\xB5\x96\0\0\xC0\x96\0\0\xC1\x96\0\0\xEA\x9A\0\0\xEB\x9A\0\0") }, 53u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MK: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"P\x04\0\0Q\x04\0\0]\x04\0\0^\x04\0\0") }, 2u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MY: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\"\x10\0\0#\x10\0\0(\x10\0\0)\x10\0\x003\x10\0\x005\x10\0\0@\x10\0\0J\x10\0\0P\x10\0\0[\x10\0\0b\x10\0\0c\x10\0\0d\x10\0\0f\x10\0\0u\x10\0\0v\x10\0\0}\x10\0\0\x7F\x10\0\0\x86\x10\0\0\x87\x10\0\0\x88\x10\0\0\x89\x10\0\0\x8A\x10\0\0\x8B\x10\0\0\x8F\x10\0\0\x9A\x10\0\0") }, 45u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static UR: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\x06\0\0\x04\x06\0\0\"\x06\0\0%\x06\0\0&\x06\0\0'\x06\0\0)\x06\0\0*\x06\0\0G\x06\0\0H\x06\0\0J\x06\0\0S\x06\0\0T\x06\0\0U\x06\0\0V\x06\0\0Y\x06\0\0p\x06\0\0q\x06\0\0z\x06\0\0~\x06\0\0\xBA\x06\0\0\xBB\x06\0\0\xC2\x06\0\0\xC4\x06\0\0\x0C \0\0\x10 \0\0") }, 35u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KO: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\x11\0\0\x13\x11\0\0a\x11\0\0v\x11\0\0\xA8\x11\0\0\xC3\x11\0\0\x18N\0\0\x19N\0\x002N\0\x003N\0\0CN\0\0DN\0\0EN\0\0FN\0\0VN\0\0WN\0\0]N\0\0_N\0\0kN\0\0lN\0\0~N\0\0\x7FN\0\0\x82N\0\0\x83N\0\0\x98N\0\0\x99N\0\0\xA4N\0\0\xA5N\0\0\xACN\0\0\xADN\0\0\xC7N\0\0\xC8N\0\0\xCAN\0\0\xCCN\0\0\xF6N\0\0\xF8N\0\0\x01O\0\0\x02O\0\0\x0BO\0\0\x0CO\0\0\x0EO\0\0\x0FO\0\0=O\0\0>O\0\0sO\0\0tO\0\0vO\0\0wO\0\0\x83O\0\0\x84O\0\0\x86O\0\0\x87O\0\0\x8AO\0\0\x8BO\0\0\x9BO\0\0\x9CO\0\0\xC2O\0\0\xC3O\0\0\xD3O\0\0\xD4O\0\0\xF1O\0\0\xF2O\0\0\x0BP\0\0\x0CP\0\0\x1EP\0\0\x1FP\0\0&P\0\0'P\0\0(P\0\0)P\0\0GP\0\0IP\0\0eP\0\0fP\0\0\x80P\0\0\x81P\0\0\x91P\0\0\x92P\0\0\xBEP\0\0\xBFP\0\0\xC5P\0\0\xC6P\0\0\xD1P\0\0\xD2P\0\0\xF9P\0\0\xFAP\0\0\x06Q\0\0\x07Q\0\0\tQ\0\0\nQ\0\0:Q\0\0;Q\0\0IQ\0\0JQ\0\0KQ\0\0LQ\0\0bQ\0\0cQ\0\0gQ\0\0hQ\0\0lQ\0\0mQ\0\0qQ\0\0rQ\0\0vQ\0\0xQ\0\0|Q\0\0}Q\0\0\x80Q\0\0\x81Q\0\0\xA0Q\0\0\xA1Q\0\0\xF1Q\0\0\xF2Q\0\0\nR\0\0\x0BR\0\0.R\0\0/R\0\08R\0\09R\0\0;R\0\0<R\0\0KR\0\0LR\0\0[R\0\0\\R\0\0\x87R\0\0\x88R\0\0\x8DR\0\0\x8ER\0\0\x92R\0\0\x93R\0\0\x9FR\0\0\xA1R\0\0\xA4R\0\0\xA5R\0\0\xABR\0\0\xACR\0\0\xC1R\0\0\xC2R\0\0\xCDR\0\0\xCER\0\0\xD8R\0\0\xD9R\0\0\xE4R\0\0\xE5R\0\0\xF8R\0\0\xF9R\0\0\xFBR\0\0\xFCR\0\0\xFER\0\0\xFFR\0\0!S\0\0\"S\0\0#S\0\0$S\0\0@S\0\0AS\0\0WS\0\0XS\0\0fS\0\0gS\0\0tS\0\0vS\0\0wS\0\0xS\0\0\x7FS\0\0\x80S\0\0\xA5S\0\0\xA6S\0\0\xBBS\0\0\xBCS\0\0\xCAS\0\0\xCBS\0\0\xE3S\0\0\xE4S\0\0\xE5S\0\0\xE6S\0\0\xE9S\0\0\xEAS\0\0\xEBS\0\0\xECS\0\0\xEFS\0\0\xF0S\0\0\x04T\0\0\x05T\0\0\tT\0\0\nT\0\0\x1BT\0\0\x1CT\0\0JT\0\0KT\0\0qT\0\0rT\0\0uT\0\0vT\0\0\x8ET\0\0\x8FT\0\0\xACT\0\0\xADT\0\0\xE5T\0\0\xE6T\0\0\xEDT\0\0\xEET\0\0SU\0\0TU\0\0\x80U\0\0\x81U\0\0\x87U\0\0\x88U\0\0\x9DU\0\0\x9EU\0\0\xABU\0\0\xADU\0\0\xDCU\0\0\xDDU\0\0\tV\0\0\nV\0\0\x14V\0\0\x15V\0\0hV\0\0iV\0\0\xCAV\0\0\xCBV\0\0\xF0V\0\0\xF1V\0\0\xFAV\0\0\xFBV\0\0\x08W\0\0\tW\0\0\x0BW\0\0\x0CW\0\0-W\0\0.W\0\0;W\0\0<W\0\0GW\0\0HW\0\0NW\0\0OW\0\0QW\0\0RW\0\0dW\0\0eW\0\0pW\0\0qW\0\0uW\0\0vW\0\0\xA2W\0\0\xA3W\0\0\xFAW\0\0\xFBW\0\0\xFCW\0\0\xFDW\0\0\0X\0\0\x01X\0\0\x05X\0\0\x06X\0\0\x08X\0\0\tX\0\0*X\0\0+X\0\0:X\0\0;X\0\0JX\0\0KX\0\0OX\0\0PX\0\0\x83X\0\0\x84X\0\0\xBEX\0\0\xBFX\0\0\xD9X\0\0\xDAX\0\0\xDEX\0\0\xDFX\0\0\x14Y\0\0\x15Y\0\0GY\0\0IY\0\0NY\0\0OY\0\0QY\0\0RY\0\0xY\0\0yY\0\0\x93Y\0\0\x94Y\0\0\x97Y\0\0\x98Y\0\0\xD1Y\0\0\xD2Y\0\0\xDCY\0\0\xDDY\0\0\xE6Y\0\0\xE7Y\0\0\x18Z\0\0\x19Z\0\0\x1CZ\0\0\x1DZ\0\0\xC1Z\0\0\xC2Z\0\0\x0C[\0\0\r[\0\0T[\0\0U[\0\0c[\0\0e[\0\0\x8F[\0\0\x90[\0\0\x98[\0\0\x99[\0\0\xA2[\0\0\xA3[\0\0\xAE[\0\0\xAF[\0\0\xB6[\0\0\xB7[\0\0\xC4[\0\0\xC5[\0\0\xC7[\0\0\xC8[\0\0\xE1[\0\0\xE2[\0\0\xEC[\0\0\xED[\0\0;\\\0\0<\\\0\0@\\\0\0A\\\0\0E\\\0\0G\\\0\0H\\\0\0I\\\0\0\x90\\\0\0\x91\\\0\0\xA1\\\0\0\xA2\\\0\0\xAC\\\0\0\xAD\\\0\0\x0E]\0\0\x0F]\0\0\x11]\0\0\x12]\0\0\x17]\0\0\x18]\0\0L]\0\0M]\0\0P]\0\0Q]\0\0\x87]\0\0\x88]\0\0\xA0]\0\0\xA1]\0\0\xE5]\0\0\xE6]\0\0\xE7]\0\0\xE9]\0\0\xF1]\0\0\xF2]\0\0\xFE]\0\0\xFF]\0\0r^\0\0s^\0\0y^\0\0z^\0\0~^\0\0\x7F^\0\0\x9A^\0\0\x9B^\0\0\xAB^\0\0\xAC^\0\0\xB7^\0\0\xB8^\0\0\xCA^\0\0\xCB^\0\0\xD0^\0\0\xD1^\0\0\xD3^\0\0\xD4^\0\0\xE3^\0\0\xE4^\0\0\xFA^\0\0\xFB^\0\0\x13_\0\0\x14_\0\0:_\0\0;_\0\0J_\0\0K_\0\0\x91_\0\0\x92_\0\0\xCC_\0\0\xCD_\0\0%`\0\0&`\0\0*`\0\0+`\0\0/`\0\x000`\0\0P`\0\0Q`\0\0]`\0\0^`\0\0j`\0\0k`\0\0m`\0\0n`\0\0\xB8`\0\0\xB9`\0\0\x06a\0\0\x07a\0\0\x1Fa\0\0 a\0\0'a\0\0(a\0\x007a\0\08a\0\0>a\0\0?a\0\0Ja\0\0Ka\0\0ca\0\0ea\0\0ha\0\0ia\0\0va\0\0xa\0\0\xA9a\0\0\xAAa\0\0\xACa\0\0\xADa\0\0\xBEa\0\0\xBFa\0\0\xC3a\0\0\xC4a\0\0\xC7a\0\0\xC8a\0\0\xE6a\0\0\xE7a\0\0\xF6a\0\0\xF7a\0\0\xFCa\0\0\xFDa\0\0\x08b\0\0\tb\0\0\x12b\0\0\x13b\0\0\x1Fb\0\0 b\0\0!b\0\0\"b\0\0qb\0\0rb\0\0\x80b\0\0\x81b\0\0\x89b\0\0\x8Ab\0\0\xC9b\0\0\xCAb\0\0\xCFb\0\0\xD1b\0\0\xD2b\0\0\xD3b\0\0\xD8b\0\0\xD9b\0\0\xECb\0\0\xEDb\0\0\xEEb\0\0\xEFb\0\0\xF1b\0\0\xF2b\0\0\xF3b\0\0\xF4b\0\0\xF7b\0\0\xF8b\0\0\xFFb\0\0\0c\0\0Oc\0\0Pc\0\0nc\0\0oc\0\0rc\0\0sc\0\0zc\0\0{c\0\0\x98c\0\0\x99c\0\0\x9Bc\0\0\x9Cc\0\0\xA7c\0\0\xA8c\0\0\xC0c\0\0\xC1c\0\0\xC6c\0\0\xC7c\0\0\xEDc\0\0\xEEc\0\0\xCAd\0\0\xCBd\0\0\xCEd\0\0\xCFd\0\0\xD2d\0\0\xD3d\0\0\xDAd\0\0\xDBd\0\0\xE7d\0\0\xE8d\0\0*e\0\0+e\0\x007e\0\08e\0\09e\0\0:e\0\0;e\0\0<e\0\0Ee\0\0Fe\0\0Ne\0\0Oe\0\0Qe\0\0Re\0\0be\0\0ce\0\0le\0\0me\0\0re\0\0se\0\0\x9Be\0\0\x9Ce\0\0\xA4e\0\0\xA5e\0\0\xD7e\0\0\xD8e\0\0\xE3e\0\0\xE4e\0\0\x06f\0\0\x07f\0\0\x11f\0\0\x12f\0\0of\0\0pf\0\0wf\0\0xf\0\0\x87f\0\0\x88f\0\0\x96f\0\0\x97f\0\0\xA0f\0\0\xA1f\0\0\xBBf\0\0\xBCf\0\0\xE0f\0\0\xE1f\0\0\xF2f\0\0\xF3f\0\0\xF4f\0\0\xF5f\0\0\xF7f\0\0\xF8f\0\0\x17g\0\0\x18g\0\0\x1Eg\0\0 g\0\0:g\0\0;g\0\0Fg\0\0Gg\0\0^g\0\0_g\0\0pg\0\0qg\0\0\x8Fg\0\0\x90g\0\0\x9Cg\0\0\x9Dg\0\0\xAFg\0\0\xB0g\0\0\xB6g\0\0\xB7g\0\0\xB8g\0\0\xB9g\0\0\xD1g\0\0\xD2g\0\0\xE9g\0\0\xEAg\0\0\xECg\0\0\xEDg\0\0\xEFg\0\0\xF0g\0\0!h\0\0\"h\0\09h\0\0:h\0\0<h\0\0=h\0\0@h\0\0Ah\0\0Bh\0\0Ch\0\0Th\0\0Uh\0\0\x7Fh\0\0\x80h\0\0\x8Fh\0\0\x90h\0\0\x97h\0\0\x98h\0\0\xB0h\0\0\xB2h\0\0\xC4h\0\0\xC5h\0\0\xCBh\0\0\xCCh\0\0\xCDh\0\0\xCEh\0\0\xD8h\0\0\xD9h\0\0\xE8h\0\0\xE9h\0\0\xFAh\0\0\xFBh\0\0Wi\0\0Xi\0\0`i\0\0ai\0\0ui\0\0vi\0\0\xC1i\0\0\xC2i\0\0\xCBi\0\0\xCCi\0\0\xD0i\0\0\xD1i\0\0\xE8i\0\0\xE9i\0\0\xEAi\0\0\xEBi\0\0\xFBi\0\0\xFCi\0\0\xFFi\0\0\0j\0\0\x02j\0\0\x03j\0\0Dj\0\0Ej\0\0Kj\0\0Lj\0\0Xj\0\0Yj\0\0_j\0\0`j\0\0\x84j\0\0\x85j\0\0\x8Ej\0\0\x8Fj\0\0\xA2j\0\0\xA3j\0\0\xC3j\0\0\xC4j\0\0\x04k\0\0\x05k\0\0\nk\0\0\x0Bk\0\0:k\0\0;k\0\0>k\0\0?k\0\0Lk\0\0Mk\0\0Pk\0\0Qk\0\0xk\0\0yk\0\0\xBCk\0\0\xBDk\0\0\xC6k\0\0\xC7k\0\0\xECk\0\0\xEDk\0\0#l\0\0$l\0\0Bl\0\0Cl\0\0_l\0\0`l\0\0hl\0\0il\0\0rl\0\0sl\0\0zl\0\0{l\0\0}l\0\0~l\0\0\x82l\0\0\x83l\0\0\xBDl\0\0\xBEl\0\0\x1Bm\0\0\x1Cm\0\08m\0\09m\0\0jm\0\0km\0\0\x87m\0\0\x88m\0\0\xC3m\0\0\xC4m\0\0\xC7m\0\0\xC8m\0\0\x1Bn\0\0\x1Cn\0\0 n\0\0!n\0\x004n\0\x005n\0\0sn\0\0tn\0\0\x9Dn\0\0\x9En\0\0\xAAn\0\0\xABn\0\0\xD1n\0\0\xD2n\0\0\xFEn\0\0\xFFn\0\0\x11o\0\0\x12o\0\0To\0\0Uo\0\0po\0\0qo\0\0\x97o\0\0\x98o\0\0\xC0o\0\0\xC1o\0\0\xEBo\0\0\xECo\0\0Lp\0\0Mp\0\0xp\0\0yp\0\0\x85p\0\0\x86p\0\0\x9Ap\0\0\x9Bp\0\0\xACp\0\0\xADp\0\0\xD9p\0\0\xDAp\0\0\xF1p\0\0\xF2p\0\0Vq\0\0Wq\0\0\x1Br\0\0\x1Cr\0\0}r\0\0~r\0\0\xACr\0\0\xADr\0\0\xC2r\0\0\xC3r\0\0\xD7r\0\0\xD8r\0\0\xE1r\0\0\xE2r\0\0\xFCr\0\0\xFDr\0\0Ws\0\0Xs\0\0\x96s\0\0\x97s\0\0\x98s\0\0\x99s\0\0\xC2s\0\0\xC3s\0\0\xCFs\0\0\xD0s\0\0\xD6s\0\0\xD7s\0\0\xD9s\0\0\xDAs\0\0\xDEs\0\0\xDFs\0\0\xEAs\0\0\xEBs\0\0\x03t\0\0\x04t\0\0&t\0\0't\0\0(t\0\0)t\0\0*t\0\0+t\0\0/t\0\x000t\0\x004t\0\x005t\0\0~t\0\0\x7Ft\0\0\x82t\0\0\x83t\0\0\x9Ft\0\0\xA0t\0\0\xA3t\0\0\xA4t\0\0\xA5t\0\0\xA6t\0\0\xCAt\0\0\xCBt\0\0\xD8t\0\0\xD9t\0\0\xDCt\0\0\xDDt\0\0\x04u\0\0\x05u\0\0\x18u\0\0\x19u\0\x002u\0\x003u\0\x007u\0\08u\0\0Gu\0\0Hu\0\0Lu\0\0Mu\0\0xu\0\0yu\0\0zu\0\0{u\0\0\x7Fu\0\0\x80u\0\0\x86u\0\0\x87u\0\0\xA5u\0\0\xA6u\0\0\xB3u\0\0\xB4u\0\0\xC2u\0\0\xC3u\0\0\xD9u\0\0\xDAu\0\0\xFCu\0\0\xFDu\0\0Nv\0\0Ov\0\0iv\0\0jv\0\0xv\0\0yv\0\0\x86v\0\0\x87v\0\0\x8Ev\0\0\x8Fv\0\0\x90v\0\0\x91v\0\0\xD6v\0\0\xD7v\0\0\xE3v\0\0\xE4v\0\0\x0Bw\0\0\x0Cw\0\x007w\0\08w\0\0~w\0\0\x7Fw\0\0\xB0w\0\0\xB1w\0\0\xBCw\0\0\xBDw\0\0\xBFw\0\0\xC0w\0\0\xDCw\0\0\xDDw\0\0\xE9w\0\0\xEAw\0\0\xEFw\0\0\xF0w\0\0Ex\0\0Fx\0\0lx\0\0mx\0\0\x81x\0\0\x82x\0\0\xA3x\0\0\xA4x\0\0\xCEx\0\0\xCFx\0\0\xECx\0\0\xEDx\0\0\xEFx\0\0\xF0x\0\0\xF5x\0\0\xF6x\0\0Ay\0\0By\0\0Gy\0\0Iy\0\0[y\0\0\\y\0\0zy\0\0{y\0\0\x81y\0\0\x82y\0\0\xBDy\0\0\xBEy\0\0\xD1y\0\0\xD2y\0\0\x08z\0\0\tz\0\0<z\0\0>z\0\0?z\0\0Az\0\0vz\0\0wz\0\0yz\0\0{z\0\0\x98z\0\0\x99z\0\0\x9Fz\0\0\xA0z\0\0\xAEz\0\0\xAFz\0\0\xBAz\0\0\xBBz\0\0\xC5z\0\0\xC6z\0\0\xDFz\0\0\xE0z\0\0\xEDz\0\0\xEEz\0\0\xF6z\0\0\xF7z\0\0\xFFz\0\0\0{\0\0K{\0\0L{\0\0P{\0\0Q{\0\0`{\0\0a{\0\0\x87{\0\0\x88{\0\0\x95{\0\0\x96{\0\0\x9D{\0\0\x9E{\0\0\xA1{\0\0\xA2{\0\0!|\0\0\"|\0\0\xB3|\0\0\xB4|\0\0\xE0|\0\0\xE1|\0\0\xFB|\0\0\xFC|\0\0\xFE|\0\0\xFF|\0\0\0}\0\0\x01}\0\0\r}\0\0\x0E}\0\0\x18}\0\0\x19}\0\0\x1A}\0\0\x1B}\0\0:}\0\0;}\0\0E}\0\0F}\0\0P}\0\0Q}\0\0^}\0\0_}\0\0f}\0\0g}\0\0s}\0\0t}\0\0y}\0\0z}\0\0\x7F}\0\0\x80}\0\0\x93}\0\0\x94}\0\0\xB1}\0\0\xB2}\0\0\xBA}\0\0\xBB}\0\0\xCA}\0\0\xCB}\0\0k~\0\0l~\0\0m~\0\0n~\0\0|~\0\0}~\0\0:\x7F\0\0;\x7F\0\0P\x7F\0\0Q\x7F\0\0k\x7F\0\0l\x7F\0\0\x85\x7F\0\0\x86\x7F\0\0\x88\x7F\0\0\x89\x7F\0\0\x8C\x7F\0\0\x8D\x7F\0\0\x94\x7F\0\0\x95\x7F\0\0\xA4\x7F\0\0\xA5\x7F\0\0\xB9\x7F\0\0\xBA\x7F\0\0\xF9\x7F\0\0\xFA\x7F\0\0\x03\x80\0\0\x04\x80\0\0\x06\x80\0\0\x07\x80\0\0\t\x80\0\0\n\x80\0\0\x15\x80\0\0\x16\x80\0\0-\x80\0\0.\x80\0\0?\x80\0\0@\x80\0\0\x8C\x80\0\0\x8D\x80\0\0\x9D\x80\0\0\x9E\x80\0\0\xA1\x80\0\0\xA2\x80\0\0\xA9\x80\0\0\xAA\x80\0\0\xAF\x80\0\0\xB0\x80\0\0\xB1\x80\0\0\xB2\x80\0\0\xDB\x80\0\0\xDC\x80\0\0\xF1\x80\0\0\xF2\x80\0\0\x1A\x81\0\0\x1C\x81\0\0T\x81\0\0U\x81\0\0q\x81\0\0r\x81\0\0\x88\x81\0\0\x89\x81\0\0\x8F\x81\0\0\x90\x81\0\0\xA0\x81\0\0\xA1\x81\0\0\xD8\x81\0\0\xD9\x81\0\0\xFC\x81\0\0\xFD\x81\0\0\x05\x82\0\0\x06\x82\0\0\n\x82\0\0\x0B\x82\0\0!\x82\0\0\"\x82\0\0n\x82\0\0o\x82\0\0q\x82\0\0r\x82\0\0\x8E\x82\0\0\x8F\x82\0\0\xA5\x82\0\0\xA6\x82\0\0\xA9\x82\0\0\xAA\x82\0\0\xB9\x82\0\0\xBA\x82\0\0\xDB\x82\0\0\xDC\x82\0\0\xDF\x82\0\0\xE0\x82\0\0\xE6\x82\0\0\xE7\x82\0\0\xFD\x82\0\0\xFE\x82\0\0\x04\x83\0\0\x05\x83\0\0\x96\x83\0\0\x97\x83\0\0\xC5\x83\0\0\xC6\x83\0\0\xCA\x83\0\0\xCB\x83\0\0\xCC\x83\0\0\xCD\x83\0\0\xD3\x83\0\0\xD4\x83\0\0\xEB\x83\0\0\xEC\x83\0\0\xF0\x83\0\0\xF1\x83\0\0=\x84\0\0>\x84\0\0[\x84\0\0\\\x84\0\0u\x84\0\0v\x84\0\0\xCB\x84\0\0\xCC\x84\0\0N\x85\0\0O\x85\0\0h\x85\0\0i\x85\0\0\x91\x85\0\0\x92\x85\0\0\xC1\x85\0\0\xC2\x85\0\0\xCD\x85\0\0\xCE\x85\0\0\xFF\x85\0\0\0\x86\0\0-\x86\0\0.\x86\0\0?\x86\0\0@\x86\0\0T\x86\0\0U\x86\0\0\xA3\x86\0\0\xA4\x86\0\0\xDF\x86\0\0\xE0\x86\0\0N\x87\0\0O\x87\0\0\xBA\x87\0\0\xBB\x87\0\0\x1F\x88\0\0 \x88\0\x001\x88\0\x002\x88\0\0W\x88\0\0X\x88\0\0b\x88\0\0c\x88\0\0r\x88\0\0s\x88\0\0~\x88\0\0\x80\x88\0\0\x88\x88\0\0\x89\x88\0\0\x9E\x88\0\0\x9F\x88\0\0\xB4\x88\0\0\xB5\x88\0\0\xD9\x88\0\0\xDA\x88\0\0\xF8\x88\0\0\xF9\x88\0\0\x10\x89\0\0\x11\x89\0\0A\x89\0\0B\x89\0\0_\x89\0\0`\x89\0\0d\x89\0\0e\x89\0\0\x8B\x89\0\0\x8C\x89\0\0\x8F\x89\0\0\x90\x89\0\0\xA1\x89\0\0\xA2\x89\0\0\xB2\x89\0\0\xB3\x89\0\0\xBA\x89\0\0\xBB\x89\0\0\xC0\x89\0\0\xC1\x89\0\0\xD2\x89\0\0\xD3\x89\0\0\x08\x8A\0\0\t\x8A\0\0\x18\x8A\0\0\x19\x8A\0\0#\x8A\0\0$\x8A\0\x006\x8A\0\x007\x8A\0\0m\x8A\0\0n\x8A\0\0\x87\x8A\0\0\x88\x8A\0\0\xA1\x8A\0\0\xA2\x8A\0\0\xA5\x8A\0\0\xA6\x8A\0\0\xB2\x8A\0\0\xB3\x8A\0\0\xEB\x8A\0\0\xEC\x8A\0\0\xFE\x8A\0\0\xFF\x8A\0\0\x19\x8B\0\0\x1A\x8B\0\0\x1B\x8B\0\0\x1C\x8B\0\x003\x8B\0\x004\x8B\0\09\x8B\0\0:\x8B\0\0O\x8B\0\0P\x8B\0\0f\x8B\0\0g\x8B\0\0t\x8B\0\0u\x8B\0\x007\x8C\0\08\x8C\0\0?\x8C\0\0@\x8C\0\0H\x8C\0\0I\x8C\0\0\xA2\x8C\0\0\xA3\x8C\0\0\xAB\x8C\0\0\xAC\x8C\0\0\xB4\x8C\0\0\xB5\x8C\0\0\xC8\x8C\0\0\xC9\x8C\0\0\xFC\x8C\0\0\xFD\x8C\0\0s\x8D\0\0t\x8D\0\0w\x8D\0\0x\x8D\0\0\xCF\x8D\0\0\xD0\x8D\0\0\xDD\x8D\0\0\xDE\x8D\0\0\xE8\x8D\0\0\xE9\x8D\0\0\x1E\x8E\0\0\x1F\x8E\0\0G\x8E\0\0H\x8E\0\0v\x8E\0\0w\x8E\0\0\xAC\x8E\0\0\xAD\x8E\0\0\xC0\x8E\0\0\xC1\x8E\0\0\xCA\x8E\0\0\xCB\x8E\0\0\xCC\x8E\0\0\xCE\x8E\0\0\xFB\x8E\0\0\xFC\x8E\0\0\x03\x8F\0\0\x04\x8F\0\0\x15\x8F\0\0\x16\x8F\0\0N\x8F\0\0O\x8F\0\0_\x8F\0\0`\x8F\0\0\x9C\x8F\0\0\x9D\x8F\0\0\xD1\x8F\0\0\xD2\x8F\0\0\xE6\x8F\0\0\xE7\x8F\0\0\xF2\x8F\0\0\xF3\x8F\0\0\x02\x90\0\0\x03\x90\0\0\x11\x90\0\0\x12\x90\0\0\x15\x90\0\0\x16\x90\0\x005\x90\0\x006\x90\0\0N\x90\0\0O\x90\0\0c\x90\0\0d\x90\0\0}\x90\0\0~\x90\0\0\x8F\x90\0\0\x90\x90\0\0\xA3\x90\0\0\xA4\x90\0\0\xAF\x90\0\0\xB0\x90\0\0\xB1\x90\0\0\xB2\x90\0\0\xCA\x90\0\0\xCB\x90\0\0\xCE\x90\0\0\xCF\x90\0\0\xE1\x90\0\0\xE2\x90\0\0\xED\x90\0\0\xEE\x90\0\0j\x91\0\0k\x91\0\0\xB5\x91\0\0\xB6\x91\0\0\xD1\x91\0\0\xD2\x91\0\0\x10\x92\0\0\x11\x92\0\0\x1E\x92\0\0\x1F\x92\0\0@\x92\0\0A\x92\0\0E\x92\0\0F\x92\0\0W\x92\0\0X\x92\0\0d\x92\0\0e\x92\0\0\xB6\x92\0\0\xB7\x92\0\0\xF8\x92\0\0\xF9\x92\0\0\xFC\x92\0\0\xFD\x92\0\0!\x93\0\0\"\x93\0\0$\x93\0\0%\x93\0\0&\x93\0\0'\x93\0\0.\x93\0\0/\x93\0\0K\x93\0\0L\x93\0\0u\x93\0\0v\x93\0\0\x8C\x93\0\0\x8D\x93\0\0\xA7\x93\0\0\xA8\x93\0\0\xE1\x93\0\0\xE2\x93\0\0Q\x94\0\0S\x94\0\0[\x94\0\0\\\x94\0\0\x8B\x95\0\0\x8C\x95\0\0\x93\x95\0\0\x94\x95\0\0\x98\x95\0\0\x99\x95\0\0\xA3\x95\0\0\xA4\x95\0\0\xA8\x95\0\0\xA9\x95\0\0\xD5\x95\0\0\xD6\x95\0\0\xDC\x95\0\0\xDD\x95\0\0M\x96\0\0N\x96\0\0\x8E\x96\0\0\x8F\x96\0\0\x94\x96\0\0\x95\x96\0\0\x99\x96\0\0\x9A\x96\0\0\xC7\x96\0\0\xC8\x96\0\0\xE3\x96\0\0\xE4\x96\0\0\x8F\x97\0\0\x90\x97\0\0\xA0\x97\0\0\xA1\x97\0\0\xA8\x97\0\0\xA9\x97\0\0\xAB\x97\0\0\xAC\x97\0\0\x03\x98\0\0\x04\x98\0\08\x98\0\09\x98\0\0F\x98\0\0G\x98\0\0g\x98\0\0h\x98\0\0\xE2\x98\0\0\xE3\x98\0\0\x03\x99\0\0\x04\x99\0\0(\x99\0\0)\x99\0\0I\x99\0\0J\x99\0\0K\x99\0\0L\x99\0\0Q\x99\0\0R\x99\0\0\xD2\x99\0\0\xD3\x99\0\0\xD5\x99\0\0\xD6\x99\0\0\xF1\x99\0\0\xF2\x99\0\0\x0E\x9A\0\0\x10\x9A\0\0+\x9A\0\0,\x9A\0\0E\x9A\0\0F\x9A\0\0U\x9A\0\0V\x9A\0\0Z\x9A\0\0[\x9A\0\0e\x9A\0\0f\x9A\0\0\xA8\x9A\0\0\xA9\x9A\0\0\xD8\x9A\0\0\xD9\x9A\0\0<\x9B\0\0=\x9B\0\0A\x9B\0\0B\x9B\0\0\xAB\x9B\0\0\xAC\x9B\0\0\xE4\x9B\0\0\xE5\x9B\0\0\xE8\x9B\0\0\xE9\x9B\0\0G\x9C\0\0H\x9C\0\0\xE9\x9C\0\0\xEA\x9C\0\0Q\x9D\0\0R\x9D\0\0`\x9D\0\0a\x9D\0\0\xC4\x9D\0\0\xC5\x9D\0\0\xD7\x9D\0\0\xD8\x9D\0\0\x1E\x9E\0\0\x1F\x9E\0\0\x92\x9E\0\0\x93\x9E\0\0\xB4\x9E\0\0\xB5\x9E\0\0\xD4\x9E\0\0\xD5\x9E\0\0\x13\x9F\0\0\x14\x9F\0\0\x95\x9F\0\0\x96\x9F\0\0\x9C\x9F\0\0\x9D\x9F\0\0") }, 822u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static EL: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\x1F\0\0\x08\x1F\0\0\x10\x1F\0\0\x16\x1F\0\0 \x1F\0\0(\x1F\0\x000\x1F\0\08\x1F\0\0B\x1F\0\0E\x1F\0\0P\x1F\0\0X\x1F\0\0b\x1F\0\0h\x1F\0\0p\x1F\0\0q\x1F\0\0r\x1F\0\0s\x1F\0\0t\x1F\0\0u\x1F\0\0v\x1F\0\0w\x1F\0\0x\x1F\0\0y\x1F\0\0z\x1F\0\0{\x1F\0\0|\x1F\0\0}\x1F\0\0\xB6\x1F\0\0\xB7\x1F\0\0\xC6\x1F\0\0\xC7\x1F\0\0\xD2\x1F\0\0\xD3\x1F\0\0\xD6\x1F\0\0\xD8\x1F\0\0\xE2\x1F\0\0\xE3\x1F\0\0\xE6\x1F\0\0\xE8\x1F\0\0\xF6\x1F\0\0\xF7\x1F\0\0") }, 63u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SA: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\r\t\0\0\x0E\t\0\0\x11\t\0\0\x12\t\0\0E\t\0\0F\t\0\0I\t\0\0J\t\0\0\x0C \0\0\x0E \0\0") }, 6u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static DOI: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\r\t\0\0\x0E\t\0\0\x11\t\0\0\x12\t\0\0E\t\0\0F\t\0\0\x0C \0\0\x0E \0\0") }, 5u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PA: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\n\0\0\x04\n\0\0\x0C \0\0\x0E \0\0") }, 5u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA8\xB2\xE0\xA8\xBC") },
                ));
                static BGC: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\t\0\0\x03\t\0\0") }, 1u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MAI: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\t\0\0\r\t\0\0\x0F\t\0\0\x11\t\0\0\x13\t\0\0\x15\t\0\0a\t\0\0b\t\0\0") }, 13u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0\xE0\xA4\x85\xE0\xA4\x82\xE0\xA4\x85\xE0\xA4\x83") },
                ));
                static BHO: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\t\0\0\x0C\t\0\0\x0F\t\0\0\x11\t\0\0\x13\t\0\0\x15\t\0\0") }, 11u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static RAJ: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x06\t\0\0\x0C\t\0\0\x0F\t\0\0\x11\t\0\0\x13\t\0\0\x15\t\0\0") }, 10u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TI: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x07\x12\0\0\x08\x12\0\0G\x12\0\0H\x12\0\0\x87\x12\0\0\x88\x12\0\0\xAF\x12\0\0\xB0\x12\0\0\xCF\x12\0\0\xD0\x12\0\0\xEF\x12\0\0\xF0\x12\0\0\xF8\x12\0\0\0\x13\0\0\x0F\x13\0\0\x10\x13\0\0\x18\x13\0\0 \x13\0\0X\x13\0\0[\x13\0\0\x80\x13\0\0\x9A\x13\0\0\x80-\0\0\x97-\0\0\xA0-\0\0\xA7-\0\0\xA8-\0\0\xAF-\0\0\xB0-\0\0\xB7-\0\0\xB8-\0\0\xBF-\0\0\xC0-\0\0\xC7-\0\0\xC8-\0\0\xCF-\0\0\xD0-\0\0\xD7-\0\0\xD8-\0\0\xDF-\0\0") }, 131u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TH: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x0B \0\0\x0C \0\0") }, 1u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BRX: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x0C \0\0\x0E \0\0") }, 2u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PS_PK: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x0C \0\0\x10 \0\0") }, 4u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static ZH: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x10N\0\0\x11N\0\0\x1BN\0\0\x1CN\0\0+N\0\0,N\0\0RN\0\0TN\0\0^N\0\0_N\0\0sN\0\0tN\0\0\xA2N\0\0\xA3N\0\0\xA9N\0\0\xAAN\0\0\xADN\0\0\xAEN\0\0\xC2N\0\0\xC3N\0\0\xC6N\0\0\xC7N\0\0\xD1N\0\0\xD2N\0\0\xD3N\0\0\xD4N\0\0\xD7N\0\0\xD8N\0\0\x1EO\0\0\x1FO\0\0*O\0\0+O\0\x006O\0\x007O\0\0:O\0\0;O\0\0CO\0\0DO\0\0cO\0\0dO\0\0\x84O\0\0\x85O\0\0\x88O\0\0\x89O\0\0\x8DO\0\0\x8EO\0\0\xA3O\0\0\xA4O\0\0\xA5O\0\0\xA6O\0\0\xAEO\0\0\xAFO\0\0\xCFO\0\0\xD1O\0\0\xD8O\0\0\xD9O\0\0\xEDO\0\0\xEEO\0\0\xEFO\0\0\xF0O\0\0\xFAO\0\0\xFBO\0\0\x14P\0\0\x15P\0\0\x18P\0\0\x19P\0\0!P\0\0\"P\0\0:P\0\0;P\0\0NP\0\0OP\0\0\x7FP\0\0\x80P\0\0\x85P\0\0\x86P\0\0\x88P\0\0\x89P\0\0\x8DP\0\0\x8EP\0\0\xA3P\0\0\xA4P\0\0\xDAP\0\0\xDBP\0\0\xF3P\0\0\xF4P\0\0\xF5P\0\0\xF6P\0\0\xFBP\0\0\xFCP\0\0\\Q\0\0]Q\0\0bQ\0\0cQ\0\0\x80Q\0\0\x81Q\0\0\x89Q\0\0\x8AQ\0\0\x97Q\0\0\x98Q\0\0\xA4Q\0\0\xA6Q\0\0\xAFQ\0\0\xB0Q\0\0\xB6Q\0\0\xB7Q\0\0\xBBQ\0\0\xBCQ\0\0\xC0Q\0\0\xC1Q\0\0\xC4Q\0\0\xC5Q\0\0\xC9Q\0\0\xCAQ\0\0\xD1Q\0\0\xD2Q\0\0\xDBQ\0\0\xDCQ\0\0\xF3Q\0\0\xF4Q\0\0\xF6Q\0\0\xF7Q\0\0\xF8Q\0\0\xFAQ\0\0\xFFQ\0\0\0R\0\0\x01R\0\0\x02R\0\0\x03R\0\0\x04R\0\0 R\0\0!R\0\0(R\0\0)R\0\0.R\0\0/R\0\09R\0\0:R\0\0CR\0\0DR\0\0JR\0\0KR\0\0TR\0\0UR\0\0VR\0\0WR\0\0eR\0\0fR\0\0}R\0\0~R\0\0\x7FR\0\0\x80R\0\0\x88R\0\0\x89R\0\0\xC3R\0\0\xC4R\0\0\xD8R\0\0\xD9R\0\0\xDFR\0\0\xE0R\0\0\xFAR\0\0\xFBR\0\0\0S\0\0\x01S\0\0\x15S\0\0\x16S\0\0 S\0\0!S\0\0#S\0\0$S\0\0*S\0\0+S\0\0.S\0\0/S\0\0>S\0\0@S\0\0IS\0\0JS\0\0QS\0\0RS\0\0\\S\0\0]S\0\0^S\0\0_S\0\0dS\0\0eS\0\0fS\0\0hS\0\0uS\0\0vS\0\0xS\0\0yS\0\0\x7FS\0\0\x80S\0\0\x95S\0\0\x96S\0\0\x98S\0\0\x99S\0\0\xA2S\0\0\xA3S\0\0\xA6S\0\0\xA7S\0\0\xA8S\0\0\xA9S\0\0\xC1S\0\0\xC2S\0\0\xDBS\0\0\xDCS\0\0\xE0S\0\0\xE1S\0\0\xE8S\0\0\xEAS\0\0\xEES\0\0\xEFS\0\0\xFCS\0\0\xFES\0\0\x01T\0\0\x02T\0\0\x06T\0\0\x07T\0\0\x0FT\0\0\x10T\0\0\x15T\0\0\x16T\0\0\x1ET\0\0\x1FT\0\0 T\0\0!T\0\0(T\0\0*T\0\0-T\0\0/T\0\x001T\0\x002T\0\x004T\0\x005T\0\0<T\0\0=T\0\0UT\0\0VT\0\0[T\0\0\\T\0\0cT\0\0dT\0\0{T\0\0|T\0\0\x84T\0\0\x85T\0\0\x8BT\0\0\x8CT\0\0\x8FT\0\0\x91T\0\0\x92T\0\0\x93T\0\0\x95T\0\0\x96T\0\0\x99T\0\0\x9AT\0\0\xB3T\0\0\xB4T\0\0\xB8T\0\0\xB9T\0\0\xBDT\0\0\xBET\0\0\xC4T\0\0\xC5T\0\0\xC6T\0\0\xC7T\0\0\xD1T\0\0\xD2T\0\0\xD7T\0\0\xD8T\0\0\xE8T\0\0\xE9T\0\0\xEET\0\0\xEFT\0\0\xFAT\0\0\xFBT\0\0\xFCT\0\0\xFDT\0\0\x01U\0\0\x02U\0\0\x06U\0\0\x08U\0\0 U\0\0!U\0\0'U\0\0(U\0\0>U\0\0?U\0\0CU\0\0EU\0\0dU\0\0eU\0\0nU\0\0oU\0\0pU\0\0qU\0\0xU\0\0yU\0\0|U\0\0}U\0\0\x89U\0\0\x8AU\0\0\x98U\0\0\x99U\0\0\xA7U\0\0\xA8U\0\0\xB1U\0\0\xB2U\0\0\xB3U\0\0\xB4U\0\0\xC5U\0\0\xC6U\0\0\xD3U\0\0\xD4U\0\0\xDCU\0\0\xDDU\0\0\xE1U\0\0\xE2U\0\0\xE6U\0\0\xE7U\0\0\xFDU\0\0\xFEU\0\0\0V\0\0\x01V\0\0\x18V\0\0\x19V\0\0\x1FV\0\0 V\0\x001V\0\x003V\0\x006V\0\x007V\0\09V\0\0:V\0\0XV\0\0YV\0\0\\V\0\0]V\0\0bV\0\0cV\0\0iV\0\0kV\0\0\x8EV\0\0\x90V\0\0\xA3V\0\0\xA4V\0\0\xB7V\0\0\xB8V\0\0\xBCV\0\0\xBDV\0\0\xCAV\0\0\xCBV\0\0\xDAV\0\0\xDBV\0\0\xE4V\0\0\xE5V\0\0\xF1V\0\0\xF2V\0\0\x03W\0\0\x04W\0\0JW\0\0KW\0\0OW\0\0PW\0\0]W\0\0^W\0\0_W\0\0aW\0\0oW\0\0pW\0\0wW\0\0xW\0\0\x84W\0\0\x85W\0\0\x9BW\0\0\x9CW\0\0\xA2W\0\0\xA3W\0\0\xA6W\0\0\xA7W\0\0\xABW\0\0\xACW\0\0\xAEW\0\0\xAFW\0\0\xC2W\0\0\xC3W\0\0\xE0W\0\0\xE1W\0\0$X\0\0%X\0\x000X\0\x001X\0\x005X\0\x006X\0\0LX\0\0MX\0\0XX\0\0YX\0\0\x85X\0\0\x86X\0\0\x93X\0\0\x94X\0\0\x99X\0\0\x9AX\0\0\x9FX\0\0\xA0X\0\0\xA9X\0\0\xAAX\0\0\xF3X\0\0\xF4X\0\0\xF6X\0\0\xF7X\0\0\xF9X\0\0\xFAX\0\0-Y\0\0.Y\0\0/Y\0\x000Y\0\0NY\0\0OY\0\0`Y\0\0aY\0\0bY\0\0cY\0\0xY\0\0yY\0\0\x83Y\0\0\x85Y\0\0\x86Y\0\0\x87Y\0\0\x92Y\0\0\x94Y\0\0\xDAY\0\0\xDBY\0\0\xDCY\0\0\xDDY\0\0\xE5Y\0\0\xE6Y\0\0\xE8Y\0\0\xE9Y\0\0\xFBY\0\0\xFCY\0\0\x07Z\0\0\x08Z\0\0%Z\0\0&Z\0\x006Z\0\x007Z\0\0IZ\0\0JZ\0\0jZ\0\0kZ\0\0tZ\0\0uZ\0\0vZ\0\0wZ\0\0\x7FZ\0\0\x80Z\0\0\x9AZ\0\0\x9BZ\0\0\xB3Z\0\0\xB4Z\0\0\xC2Z\0\0\xC3Z\0\0\xC9Z\0\0\xCAZ\0\0j[\0\0k[\0\0u[\0\0v[\0\0}[\0\0~[\0\0\x85[\0\0\x86[\0\0\xA0[\0\0\xA1[\0\0\xA6[\0\0\xA7[\0\0\xAB[\0\0\xAC[\0\0\xB0[\0\0\xB1[\0\0\xB5[\0\0\xB6[\0\0\xD3[\0\0\xD4[\0\0\xE5[\0\0\xE6[\0\0\xFA[\0\0\xFB[\0\0\t\\\0\0\n\\\0\0'\\\0\0(\\\0\0,\\\0\0-\\\0\x004\\\0\x005\\\0\08\\\0\09\\\0\0?\\\0\0@\\\0\0H\\\0\0K\\\0\0N\\\0\0O\\\0\0Q\\\0\0R\\\0\0a\\\0\0b\\\0\0e\\\0\0f\\\0\0o\\\0\0p\\\0\0y\\\0\0z\\\0\0\x7F\\\0\0\x80\\\0\0\x94\\\0\0\x95\\\0\0\x96\\\0\0\x97\\\0\0\xA9\\\0\0\xAA\\\0\0\xAD\\\0\0\xAE\\\0\0\xE6\\\0\0\xE7\\\0\0\xE8\\\0\0\xE9\\\0\0\xED\\\0\0\xEE\\\0\0\xFB\\\0\0\xFC\\\0\0\x0E]\0\0\x0F]\0\0\x14]\0\0\x15]\0\0\x16]\0\0\x17]\0\0\x1B]\0\0\x1C]\0\0-]\0\0.]\0\0L]\0\0M]\0\0\xC5]\0\0\xC6]\0\0\xCD]\0\0\xCE]\0\0\xE2]\0\0\xE3]\0\0\xE9]\0\0\xEA]\0\0\xFD]\0\0\xFF]\0\0\x06^\0\0\x07^\0\0\x18^\0\0\x19^\0\0\x1A^\0\0\x1B^\0\0\x1C^\0\0\x1D^\0\0'^\0\0(^\0\x007^\0\08^\0\0L^\0\0M^\0\0b^\0\0c^\0\0\x84^\0\0\x85^\0\0\x87^\0\0\x88^\0\0\x90^\0\0\x91^\0\0\xB5^\0\0\xB7^\0\0\xCA^\0\0\xCB^\0\0\xD3^\0\0\xD4^\0\0\x13_\0\0\x14_\0\0\x1B_\0\0\x1C_\0\0'_\0\0(_\0\0W_\0\0X_\0\0d_\0\0e_\0\0j_\0\0k_\0\0\x8A_\0\0\x8B_\0\0\x98_\0\0\x9A_\0\0\xA1_\0\0\xA2_\0\0\xBD_\0\0\xBE_\0\0\xF1_\0\0\xF2_\0\0\xFF_\0\0\0`\0\0\x14`\0\0\x15`\0\0\x1C`\0\0\x1D`\0\0 `\0\0!`\0\0/`\0\x000`\0\0C`\0\0D`\0\0M`\0\0N`\0\0R`\0\0S`\0\0U`\0\0V`\0\0d`\0\0e`\0\0l`\0\0m`\0\0s`\0\0t`\0\0\x8D`\0\0\x8E`\0\0\x96`\0\0\x97`\0\0\xA6`\0\0\xA7`\0\0\xAC`\0\0\xAD`\0\0\xAF`\0\0\xB0`\0\0\xB4`\0\0\xB5`\0\0\xBC`\0\0\xBD`\0\0\xCA`\0\0\xCC`\0\0\xD5`\0\0\xD6`\0\0\xDF`\0\0\xE0`\0\0\xE6`\0\0\xE7`\0\0\xE9`\0\0\xEA`\0\0\xEB`\0\0\xEC`\0\0\xED`\0\0\xEE`\0\0\xF0`\0\0\xF1`\0\0\xF6`\0\0\xF7`\0\0\x15a\0\0\x16a\0\0#a\0\0%a\0\0?a\0\0@a\0\0La\0\0Ma\0\0ha\0\0ia\0\0wa\0\0xa\0\0\x8Ba\0\0\x8Ca\0\0\x8Ea\0\0\x8Fa\0\0\x94a\0\0\x95a\0\0\xA8a\0\0\xA9a\0\0\xC8a\0\0\xC9a\0\0\xCAa\0\0\xCBa\0\0\xE6a\0\0\xE7a\0\0\x0Eb\0\0\x0Fb\0\0\x1Ab\0\0\x1Bb\0\0\x1Fb\0\0 b\0\x003b\0\x004b\0\0Rb\0\0Sb\0\0Tb\0\0Ub\0\0[b\0\0\\b\0\0pb\0\0qb\0\0sb\0\0tb\0\0vb\0\0wb\0\0|b\0\0}b\0\0\x92b\0\0\x93b\0\0\x96b\0\0\x97b\0\0\x9Ab\0\0\x9Cb\0\0\xA0b\0\0\xA2b\0\0\xBCb\0\0\xBDb\0\0\xC2b\0\0\xC3b\0\0\xC4b\0\0\xC5b\0\0\xC7b\0\0\xC8b\0\0\xCCb\0\0\xCDb\0\0\xCEb\0\0\xCFb\0\0\xD0b\0\0\xD1b\0\0\xD3b\0\0\xD4b\0\0\xD7b\0\0\xD8b\0\0\xD9b\0\0\xDAb\0\0\xE2b\0\0\xE4b\0\0\xE7b\0\0\xE8b\0\0\xEDb\0\0\xEEb\0\0\xEFb\0\0\xF0b\0\0\xF1b\0\0\xF2b\0\0\xF4b\0\0\xF5b\0\0\xFDb\0\0\xFEb\0\0\x02c\0\0\x03c\0\0\x0Ec\0\0\x0Fc\0\0\x1Ac\0\0\x1Bc\0\0\x1Fc\0\0!c\0\0#c\0\0$c\0\0(c\0\0)c\0\0+c\0\0,c\0\0=c\0\0>c\0\0Bc\0\0Cc\0\0Ec\0\0Gc\0\0Lc\0\0Pc\0\0^c\0\0_c\0\0cc\0\0dc\0\0gc\0\0hc\0\0vc\0\0wc\0\0zc\0\0|c\0\0\x80c\0\0\x81c\0\0\x82c\0\0\x83c\0\0\x8Fc\0\0\x91c\0\0\x98c\0\0\x99c\0\0\xA0c\0\0\xA1c\0\0\xB0c\0\0\xB1c\0\0\xB7c\0\0\xB8c\0\0\xBAc\0\0\xBBc\0\0\xC9c\0\0\xCAc\0\0\xCDc\0\0\xCEc\0\0\xE3c\0\0\xE4c\0\0\xE9c\0\0\xEBc\0\0\xEDc\0\0\xEEc\0\0\xFDc\0\0\xFEc\0\0\0d\0\0\x03d\0\0\x05d\0\0\x06d\0\0\x0Fd\0\0\x10d\0\0\x13d\0\0\x15d\0\0:d\0\0;d\0\0Gd\0\0Hd\0\0gd\0\0hd\0\0yd\0\0zd\0\0\x85d\0\0\x86d\0\0\x87d\0\0\x88d\0\0\x91d\0\0\x92d\0\0\x95d\0\0\x96d\0\0\xA4d\0\0\xA5d\0\0\xA9d\0\0\xAAd\0\0\xACd\0\0\xADd\0\0\xAEd\0\0\xAFd\0\0\xB0d\0\0\xB1d\0\0\xB5d\0\0\xB6d\0\0\xBCd\0\0\xBDd\0\0\xC2d\0\0\xC3d\0\0\xC5d\0\0\xC6d\0\0\xD2d\0\0\xD3d\0\0\0e\0\0\x01e\0\0[e\0\0\\e\0\0^e\0\0_e\0\0we\0\0xe\0\0\x8Ce\0\0\x8De\0\0\x91e\0\0\x92e\0\0\x9Fe\0\0\xA0e\0\0\xA4e\0\0\xA5e\0\0\xA7e\0\0\xA8e\0\0\xA9e\0\0\xAAe\0\0\xECe\0\0\xEDe\0\0\xF1e\0\0\xF2e\0\0\xF7e\0\0\xF8e\0\0\x14f\0\0\x15f\0\0\x19f\0\0\x1Af\0\0'f\0\0(f\0\0<f\0\0=f\0\0Lf\0\0Mf\0\0Uf\0\0Vf\0\0ff\0\0gf\0\0pf\0\0qf\0\0~f\0\0\x7Ff\0\0\x87f\0\0\x88f\0\0\xD9f\0\0\xDAf\0\0\xDDf\0\0\xDEf\0\0\x14g\0\0\x15g\0\0&g\0\0'g\0\x004g\0\x005g\0\0=g\0\0>g\0\0Fg\0\0Gg\0\0Og\0\0Pg\0\0Vg\0\0Wg\0\0`g\0\0ag\0\0mg\0\0ng\0\0\x89g\0\0\x8Ag\0\0\x95g\0\0\x96g\0\0\x9Ag\0\0\x9Bg\0\0\xA3g\0\0\xA4g\0\0\xAFg\0\0\xB0g\0\0\xC4g\0\0\xC5g\0\0\xD1g\0\0\xD3g\0\0\xDCg\0\0\xDDg\0\0\xE0g\0\0\xE1g\0\0\xE9g\0\0\xEAg\0\0\xF1g\0\0\xF2g\0\0\xFFg\0\0\0h\0\0\x05h\0\0\x06h\0\0\x08h\0\0\th\0\0\x13h\0\0\x14h\0\0\x16h\0\0\x18h\0\0*h\0\0+h\0\0=h\0\0>h\0\0Bh\0\0Ch\0\0Ph\0\0Qh\0\0Th\0\0Uh\0\0fh\0\0gh\0\0hh\0\0jh\0\0vh\0\0wh\0\0\x86h\0\0\x87h\0\0\x97h\0\0\x98h\0\0\xA2h\0\0\xA3h\0\0\xA7h\0\0\xA9h\0\0\xADh\0\0\xAEh\0\0\xB3h\0\0\xB4h\0\0\xCDh\0\0\xCEh\0\0\xD5h\0\0\xD6h\0\0\xD8h\0\0\xD9h\0\0\xE0h\0\0\xE1h\0\0\xF1h\0\0\xF2h\0\0\xF5h\0\0\xF6h\0\0\xFAh\0\0\xFBh\0\0\x0Ei\0\0\x0Fi\0\0\x12i\0\0\x13i\0\0-i\0\0.i\0\0?i\0\0@i\0\0Ti\0\0Ui\0\0`i\0\0ai\0\0wi\0\0xi\0\0\x84i\0\0\x85i\0\0\x86i\0\0\x87i\0\0\x88i\0\0\x89i\0\0\x94i\0\0\x96i\0\0\xA8i\0\0\xA9i\0\0\xB4i\0\0\xB5i\0\0\xCCi\0\0\xCDi\0\0\xD0i\0\0\xD1i\0\0\xDBi\0\0\xDCi\0\0\xDFi\0\0\xE0i\0\0\xFDi\0\0\xFEi\0\0\xFFi\0\0\0j\0\0\x1Fj\0\0 j\0\0*j\0\0+j\0\0Dj\0\0Ej\0\0Gj\0\0Hj\0\0Xj\0\0Zj\0\0aj\0\0bj\0\0qj\0\0rj\0\0\x90j\0\0\x91j\0\0\xACj\0\0\xADj\0\0Gk\0\0Hk\0\0gk\0\0hk\0\0yk\0\0zk\0\0|k\0\0}k\0\0\x83k\0\0\x84k\0\0\x89k\0\0\x8Ak\0\0\x96k\0\0\x97k\0\0\xB4k\0\0\xB5k\0\0\xB7k\0\0\xB8k\0\0\xBFk\0\0\xC0k\0\0\xC1k\0\0\xC2k\0\0\xD9k\0\0\xDAk\0\0\xE1k\0\0\xE2k\0\0\xEFk\0\0\xF0k\0\0\x13l\0\0\x14l\0\0\"l\0\0#l\0\0'l\0\0)l\0\0.l\0\x000l\0\0Al\0\0Bl\0\0[l\0\0\\l\0\0^l\0\0_l\0\0pl\0\0ql\0\0yl\0\0zl\0\0\x81l\0\0\x82l\0\0\x90l\0\0\x91l\0\0\x9Bl\0\0\x9Cl\0\0\xA5l\0\0\xA7l\0\0\xAAl\0\0\xACl\0\0\xAEl\0\0\xAFl\0\0\xB8l\0\0\xB9l\0\0\xBCl\0\0\xBFl\0\0\xC4l\0\0\xC5l\0\0\xCCl\0\0\xCDl\0\0\xDEl\0\0\xDFl\0\0\xEAl\0\0\xEBl\0\0\xF5l\0\0\xF6l\0\0\xFBl\0\0\xFDl\0\0\x01m\0\0\x02m\0\0\x12m\0\0\x13m\0\0<m\0\0=m\0\0Fm\0\0Hm\0\0Jm\0\0Km\0\0cm\0\0dm\0\0xm\0\0ym\0\0\x82m\0\0\x83m\0\0\x8Cm\0\0\x8Dm\0\0\x8Em\0\0\x8Fm\0\0\x95m\0\0\x96m\0\0\x9Dm\0\0\x9Em\0\0\xA1m\0\0\xA2m\0\0\xA3m\0\0\xA5m\0\0\xA6m\0\0\xA8m\0\0\xA9m\0\0\xAAm\0\0\xAEm\0\0\xAFm\0\0\xC0m\0\0\xC1m\0\0\xC6m\0\0\xC8m\0\0\xCCm\0\0\xCDm\0\0\xE4m\0\0\xE5m\0\0\xEBm\0\0\xECm\0\0\xEEm\0\0\xEFm\0\0\xF3m\0\0\xF4m\0\0\xF9m\0\0\xFAm\0\0\nn\0\0\x0Bn\0\0\x14n\0\0\x15n\0\0\x17n\0\0\x18n\0\0\x1Dn\0\0\x1En\0\0 n\0\0!n\0\0$n\0\0%n\0\x002n\0\x003n\0\0:n\0\0;n\0\0Cn\0\0Dn\0\0Xn\0\0Yn\0\0\x7Fn\0\0\x80n\0\0\x83n\0\0\x84n\0\0\x85n\0\0\x86n\0\0\x89n\0\0\x8An\0\0\xA2n\0\0\xA3n\0\0\xAFn\0\0\xB0n\0\0\xB6n\0\0\xB7n\0\0\xBAn\0\0\xBBn\0\0\xC7n\0\0\xC8n\0\0\xD4n\0\0\xD6n\0\0\xDAn\0\0\xDBn\0\0\xDEn\0\0\xDFn\0\0\xE4n\0\0\xE5n\0\0\xE9n\0\0\xEAn\0\0\x06o\0\0\x07o\0\0\x13o\0\0\x14o\0\x001o\0\x002o\0\0>o\0\0?o\0\0Go\0\0Ho\0\0mo\0\0no\0\0\x84o\0\0\x85o\0\0\x88o\0\0\x89o\0\0\x9Co\0\0\x9Do\0\0\xA1o\0\0\xA2o\0\0\xD2o\0\0\xD3o\0\0\x11p\0\0\x12p\0\0vp\0\0wp\0\0xp\0\0yp\0\0|p\0\0}p\0\0~p\0\0\x7Fp\0\0\x8Ap\0\0\x8Bp\0\0\x92p\0\0\x93p\0\0\x95p\0\0\x96p\0\0\xABp\0\0\xAEp\0\0\xBCp\0\0\xBEp\0\0\xC1p\0\0\xC2p\0\0\xD8p\0\0\xDAp\0\0\xDBp\0\0\xDCp\0\0\xDFp\0\0\xE0p\0\0\xEBp\0\0\xECp\0\0\xF9p\0\0\xFAp\0\0\tq\0\0\x0Bq\0\0\x15q\0\0\x16q\0\0\x19q\0\0\x1Bq\0\x000q\0\x001q\0\0Nq\0\0Oq\0\0dq\0\0eq\0\0}q\0\0~q\0\0\x84q\0\0\x85q\0\0\x8Fq\0\0\x90q\0\0\x94q\0\0\x95q\0\0\x99q\0\0\x9Aq\0\0\xACq\0\0\xADq\0\0\xE5q\0\0\xE6q\0\09r\0\0:r\0\0\x80r\0\0\x82r\0\0\x84r\0\0\x85r\0\0\xACr\0\0\xADr\0\0\xB8r\0\0\xB9r\0\0\xC8r\0\0\xC9r\0\0\xDEr\0\0\xDFr\0\0\xE1r\0\0\xE2r\0\0\xEDr\0\0\xEEr\0\0\xF0r\0\0\xF1r\0\0\xF8r\0\0\xF9r\0\0\x0Es\0\0\x0Fs\0\0\x15s\0\0\x17s\0\0)s\0\0*s\0\0+s\0\0-s\0\0>s\0\0@s\0\0ms\0\0ns\0\0~s\0\0\x7Fs\0\0\x96s\0\0\x97s\0\0\xB7s\0\0\xB8s\0\0\x05t\0\0\x06t\0\0\tt\0\0\nt\0\0\x10t\0\0\x11t\0\0\"t\0\0#t\0\0Zt\0\0[t\0\0\xA7t\0\0\xA8t\0\0\xE2t\0\0\xE5t\0\0\xEEt\0\0\xEFt\0\0\xF7t\0\0\xF8t\0\0%u\0\0&u\0\0)u\0\0*u\0\0+u\0\0,u\0\0Ou\0\0Pu\0\0Tu\0\0Uu\0\0\\u\0\0]u\0\0tu\0\0uu\0\0xu\0\0yu\0\0\x99u\0\0\x9Bu\0\0\x9Fu\0\0\xA0u\0\0\xA4u\0\0\xA5u\0\0\xABu\0\0\xACu\0\0\xAEu\0\0\xAFu\0\0\xB9u\0\0\xBAu\0\0\xC7u\0\0\xC8u\0\0\xCAu\0\0\xCBu\0\0\xD2u\0\0\xD3u\0\0\xD8u\0\0\xD9u\0\0\xE2u\0\0\xE3u\0\0\xEAu\0\0\xEBu\0\0\xF0u\0\0\xF1u\0\0\xF9u\0\0\xFAu\0\0\x1Fv\0\0 v\0\0$v\0\0%v\0\0&v\0\0'v\0\0)v\0\0,v\0\08v\0\09v\0\0>v\0\0?v\0\0Lv\0\0Mv\0\0cv\0\0dv\0\0\x82v\0\0\x83v\0\0\x93v\0\0\x94v\0\0\x96v\0\0\x97v\0\0\xB1v\0\0\xB2v\0\0\xBFv\0\0\xC0v\0\0\xC6v\0\0\xC7v\0\0\xCFv\0\0\xD1v\0\0\xD4v\0\0\xD5v\0\0\xD7v\0\0\xD8v\0\0\xE5v\0\0\xE6v\0\0\xEFv\0\0\xF0v\0\0\xF2v\0\0\xF3v\0\0\xF9v\0\0\xFAv\0\0(w\0\0*w\0\0/w\0\x000w\0\x006w\0\08w\0\0Aw\0\0Bw\0\0Pw\0\0Qw\0\0fw\0\0gw\0\0kw\0\0mw\0\0yw\0\0zw\0\0\x7Fw\0\0\x80w\0\0\x84w\0\0\x86w\0\0\x8Cw\0\0\x8Dw\0\0\x8Ew\0\0\x8Fw\0\0\x92w\0\0\x93w\0\0\xA9w\0\0\xABw\0\0\xACw\0\0\xAEw\0\0\xB3w\0\0\xB4w\0\0\xBBw\0\0\xBCw\0\0\xD7w\0\0\xD8w\0\0\xE2w\0\0\xE3w\0\0\xE9w\0\0\xEAw\0\0\xEBw\0\0\xECw\0\0\xEEw\0\0\xEFw\0\0\xFEw\0\0\0x\0\0\x0Cx\0\0\rx\0\0\x16x\0\0\x17x\0\0\x1Ax\0\0\x1Bx\0\x000x\0\x001x\0\08x\0\09x\0\0>x\0\0?x\0\0Ex\0\0Fx\0\0]x\0\0^x\0\0kx\0\0lx\0\0\x8Cx\0\0\x8Dx\0\0\x91x\0\0\x92x\0\0\x98x\0\0\x99x\0\0\xB1x\0\0\xB2x\0\0\xB3x\0\0\xB4x\0\0\xBEx\0\0\xBFx\0\0\xCAx\0\0\xCBx\0\0\xD5x\0\0\xD6x\0\0\xF7x\0\0\xF8x\0\0\x01y\0\0\x02y\0\0@y\0\0Ay\0\0Hy\0\0Iy\0\0_y\0\0ay\0\0my\0\0ny\0\0wy\0\0xy\0\0\x80y\0\0\x81y\0\0\x84y\0\0\x85y\0\0\xB9y\0\0\xBAy\0\0\xBDy\0\0\xBFy\0\0\xC3y\0\0\xC4y\0\0\xC6y\0\0\xC7y\0\0\xC9y\0\0\xCAy\0\0\xE7y\0\0\xE8y\0\0\xF8y\0\0\xF9y\0\0\xFDy\0\0\xFEy\0\0\x1Az\0\0\x1Bz\0\0 z\0\0!z\0\0;z\0\0>z\0\0Wz\0\0Xz\0\0tz\0\0uz\0\0\x83z\0\0\x85z\0\0\x8Dz\0\0\x8Ez\0\0\x91z\0\0\x93z\0\0\x96z\0\0\x97z\0\0\x98z\0\0\x99z\0\0\x9Cz\0\0\x9Dz\0\0\x9Fz\0\0\xA0z\0\0\xA5z\0\0\xA6z\0\0\xBFz\0\0\xC0z\0\0\xD6z\0\0\xD7z\0\0\xE3z\0\0\xE4z\0\0\xEDz\0\0\xEEz\0\0\xFFz\0\0\0{\0\0\x06{\0\0\x07{\0\0\x0B{\0\0\x0C{\0\0\x19{\0\0\x1A{\0\0:{\0\0;{\0\0<{\0\0={\0\0O{\0\0Q{\0\0R{\0\0S{\0\0[{\0\0\\{\0\0]{\0\0^{\0\0w{\0\0x{\0\0\x95{\0\0\x96{\0\0\xA9{\0\0\xAA{\0\0\xAB{\0\0\xAC{\0\0\xB8{\0\0\xB9{\0\0\xD3{\0\0\xD4{\0\0\xE1{\0\0\xE2{\0\0\xF1{\0\0\xF2{\0\0\xF7{\0\0\xF8{\0\0\x07|\0\0\x08|\0\0'|\0\0(|\0\08|\0\09|\0\0}|\0\0~|\0\0\x91|\0\0\x92|\0\0\x98|\0\0\x99|\0\0\x9F|\0\0\xA0|\0\0\xA5|\0\0\xA6|\0\0\xAA|\0\0\xAB|\0\0\xAE|\0\0\xAF|\0\0\xB1|\0\0\xB2|\0\0\xBD|\0\0\xBE|\0\0\xD9|\0\0\xDA|\0\0\xE0|\0\0\xE1|\0\0\xEF|\0\0\xF0|\0\0\n}\0\0\x0B}\0\0n}\0\0o}\0\0\xA0~\0\0\xA1~\0\0\xA4~\0\0\xA5~\0\0\xAB~\0\0\xAD~\0\0\xB1~\0\0\xB2~\0\0\xB9~\0\0\xBB~\0\0\xC5~\0\0\xC6~\0\0\xCA~\0\0\xCB~\0\0\xCE~\0\0\xCF~\0\0\xD1~\0\0\xD3~\0\0\xDA~\0\0\xDB~\0\0\xDE~\0\0\xDF~\0\0\xE2~\0\0\xE4~\0\0\xF0~\0\0\xF1~\0\0\xF3~\0\0\xF4~\0\0\xF7~\0\0\xF9~\0\0\xFD~\0\0\xFE~\0\0\0\x7F\0\0\x01\x7F\0\0\x04\x7F\0\0\x05\x7F\0\0\x06\x7F\0\0\x07\x7F\0\0\t\x7F\0\0\n\x7F\0\0\x0E\x7F\0\0\x0F\x7F\0\0\x14\x7F\0\0\x16\x7F\0\0\x1A\x7F\0\0\x1B\x7F\0\0\x1D\x7F\0\0\x1E\x7F\0\0$\x7F\0\0%\x7F\0\0-\x7F\0\0.\x7F\0\x000\x7F\0\x001\x7F\0\0i\x7F\0\0j\x7F\0\0\x94\x7F\0\0\x95\x7F\0\0\x9A\x7F\0\0\x9B\x7F\0\0\xA1\x7F\0\0\xA2\x7F\0\0\xB9\x7F\0\0\xBA\x7F\0\0\xE9\x7F\0\0\xEA\x7F\0\0\xF1\x7F\0\0\xF2\x7F\0\0\x15\x80\0\0\x16\x80\0\0\x18\x80\0\0\x1A\x80\0\08\x80\0\09\x80\0\0;\x80\0\0<\x80\0\0=\x80\0\0>\x80\0\0?\x80\0\0@\x80\0\0B\x80\0\0C\x80\0\0F\x80\0\0G\x80\0\0K\x80\0\0L\x80\0\0\x83\x80\0\0\x84\x80\0\0\x86\x80\0\0\x88\x80\0\0\x8B\x80\0\0\x8D\x80\0\0\x98\x80\0\0\x99\x80\0\0\x9B\x80\0\0\x9C\x80\0\0\x9D\x80\0\0\x9E\x80\0\0\xA0\x80\0\0\xA1\x80\0\0\xA2\x80\0\0\xA3\x80\0\0\xAA\x80\0\0\xAB\x80\0\0\xAE\x80\0\0\xAF\x80\0\0\xB4\x80\0\0\xB5\x80\0\0\xBA\x80\0\0\xBB\x80\0\0\xBE\x80\0\0\xC1\x80\0\0\xC3\x80\0\0\xC4\x80\0\0\xDA\x80\0\0\xDB\x80\0\0\xE7\x80\0\0\xE8\x80\0\0\xF0\x80\0\0\xF1\x80\0\0\xF3\x80\0\0\xF4\x80\0\0\x02\x81\0\0\x03\x81\0\0\t\x81\0\0\x0B\x81\0\0\x0F\x81\0\0\x11\x81\0\0\x13\x81\0\0\x14\x81\0\0\x16\x81\0\0\x17\x81\0\0\x1A\x81\0\0\x1B\x81\0\0/\x81\0\x000\x81\0\0>\x81\0\0?\x81\0\0K\x81\0\0L\x81\0\0T\x81\0\0V\x81\0\0e\x81\0\0f\x81\0\0n\x81\0\0o\x81\0\0z\x81\0\0|\x81\0\0\x80\x81\0\0\x81\x81\0\0\x8A\x81\0\0\x8B\x81\0\0\x8F\x81\0\0\x90\x81\0\0\x9B\x81\0\0\x9E\x81\0\0\xA8\x81\0\0\xA9\x81\0\0\xC0\x81\0\0\xC1\x81\0\0\xCA\x81\0\0\xCB\x81\0\0\xFC\x81\0\0\xFD\x81\0\0\0\x82\0\0\x01\x82\0\0\x05\x82\0\0\x07\x82\0\0\x14\x82\0\0\x15\x82\0\x001\x82\0\x002\x82\0\x005\x82\0\x007\x82\0\0G\x82\0\0H\x82\0\0X\x82\0\0Y\x82\0\0n\x82\0\0o\x82\0\0p\x82\0\0q\x82\0\0s\x82\0\0t\x82\0\0\x8B\x82\0\0\x8C\x82\0\0\x99\x82\0\0\x9A\x82\0\0\x9C\x82\0\0\x9D\x82\0\0\xA5\x82\0\0\xA6\x82\0\0\xAF\x82\0\0\xB0\x82\0\0\xB9\x82\0\0\xBA\x82\0\0\xBD\x82\0\0\xBE\x82\0\0\xC7\x82\0\0\xC8\x82\0\0\xD1\x82\0\0\xD2\x82\0\0\xD4\x82\0\0\xD5\x82\0\0\xDB\x82\0\0\xDD\x82\0\0\xDE\x82\0\0\xE0\x82\0\0\xE3\x82\0\0\xE4\x82\0\0\xF9\x82\0\0\xFA\x82\0\0\x01\x83\0\0\x02\x83\0\0\x04\x83\0\0\x06\x83\0\0\t\x83\0\0\n\x83\0\0\x0E\x83\0\0\x0F\x83\0\0'\x83\0\0(\x83\0\0,\x83\0\0-\x83\0\x005\x83\0\x006\x83\0\08\x83\0\09\x83\0\0F\x83\0\0G\x83\0\0T\x83\0\0U\x83\0\0a\x83\0\0b\x83\0\0d\x83\0\0e\x83\0\0g\x83\0\0h\x83\0\0k\x83\0\0l\x83\0\0\x93\x83\0\0\x94\x83\0\0\xB4\x83\0\0\xB5\x83\0\0\xB9\x83\0\0\xBB\x83\0\0\xBD\x83\0\0\xBE\x83\0\0\xC7\x83\0\0\xC8\x83\0\0\xCA\x83\0\0\xCB\x83\0\0\xCC\x83\0\0\xCD\x83\0\0\xE0\x83\0\0\xE1\x83\0\0\xF1\x83\0\0\xF2\x83\0\0\x0C\x84\0\0\r\x84\0\0\x0E\x84\0\0\x0F\x84\0\0\x1D\x84\0\0\x1E\x84\0\0c\x84\0\0d\x84\0\0i\x84\0\0j\x84\0\0k\x84\0\0m\x84\0\0q\x84\0\0r\x84\0\0u\x84\0\0v\x84\0\0\x9C\x84\0\0\x9D\x84\0\0\xB2\x84\0\0\xB3\x84\0\0\xB8\x84\0\0\xB9\x84\0\0\xC4\x84\0\0\xC5\x84\0\0\xFF\x84\0\0\0\x85\0\0\x13\x85\0\0\x14\x85\0\0\x17\x85\0\0\x18\x85\0\0\x1A\x85\0\0\x1B\x85\0\0,\x85\0\0-\x85\0\0<\x85\0\0>\x85\0\0I\x85\0\0K\x85\0\0t\x85\0\0u\x85\0\0~\x85\0\0\x7F\x85\0\0\x87\x85\0\0\x88\x85\0\0\x9B\x85\0\0\x9C\x85\0\0\xAF\x85\0\0\xB0\x85\0\0\xD0\x85\0\0\xD1\x85\0\0\xD5\x85\0\0\xD6\x85\0\0\xFB\x85\0\0\xFC\x85\0\0\x11\x86\0\0\x12\x86\0\0O\x86\0\0Q\x86\0\0Z\x86\0\0[\x86\0\0\x80\x86\0\0\x81\x86\0\0\x82\x86\0\0\x83\x86\0\0\x8A\x86\0\0\x8B\x86\0\0\x8C\x86\0\0\x8D\x86\0\0\x93\x86\0\0\x94\x86\0\0\x95\x86\0\0\x96\x86\0\0\x9D\x86\0\0\x9E\x86\0\0\xA3\x86\0\0\xA5\x86\0\0\xAA\x86\0\0\xAB\x86\0\0\xAF\x86\0\0\xB0\x86\0\0\xC0\x86\0\0\xC1\x86\0\0\xC6\x86\0\0\xC7\x86\0\0\xCE\x86\0\0\xCF\x86\0\0\xD0\x86\0\0\xD1\x86\0\0\xDB\x86\0\0\xDC\x86\0\0\xE4\x86\0\0\xE5\x86\0\0\xF0\x86\0\0\xF1\x86\0\0\xFE\x86\0\0\xFF\x86\0\0\0\x87\0\0\x01\x87\0\0\x08\x87\0\0\t\x87\0\0\x12\x87\0\0\x14\x87\0\0\x15\x87\0\0\x16\x87\0\0\x17\x87\0\0\x19\x87\0\0!\x87\0\0\"\x87\0\0%\x87\0\0&\x87\0\x004\x87\0\x005\x87\0\0;\x87\0\0<\x87\0\0G\x87\0\0H\x87\0\0I\x87\0\0J\x87\0\0L\x87\0\0M\x87\0\0N\x87\0\0O\x87\0\0W\x87\0\0X\x87\0\0Y\x87\0\0Z\x87\0\0`\x87\0\0a\x87\0\0t\x87\0\0u\x87\0\0\x82\x87\0\0\x84\x87\0\0\xBA\x87\0\0\xBB\x87\0\0\xC0\x87\0\0\xC1\x87\0\0\xC6\x87\0\0\xC7\x87\0\0\xCB\x87\0\0\xCC\x87\0\0\xD1\x87\0\0\xD2\x87\0\0\x15\x88\0\0\x16\x88\0\0E\x88\0\0F\x88\0\0M\x88\0\0N\x88\0\0T\x88\0\0U\x88\0\0Y\x88\0\0Z\x88\0\0k\x88\0\0m\x88\0\0p\x88\0\0q\x88\0\0w\x88\0\0x\x88\0\0\x81\x88\0\0\x82\x88\0\0\x84\x88\0\0\x85\x88\0\0\x8D\x88\0\0\x8E\x88\0\0\x96\x88\0\0\x97\x88\0\0\x9C\x88\0\0\x9D\x88\0\0\xB1\x88\0\0\xB2\x88\0\0\xD9\x88\0\0\xDA\x88\0\0\xF3\x88\0\0\xF4\x88\0\0\xF8\x88\0\0\xFA\x88\0\0\x02\x89\0\0\x03\x89\0\0\x10\x89\0\0\x11\x89\0\0\x12\x89\0\0\x13\x89\0\0%\x89\0\0&\x89\0\0*\x89\0\0+\x89\0\0_\x89\0\0`\x89\0\0\xC5\x89\0\0\xC6\x89\0\0\xE6\x89\0\0\xE7\x89\0\0\x8A\x8A\0\0\x8B\x8A\0\0l\x8B\0\0m\x8B\0\0\xA5\x8B\0\0\xA6\x8B\0\0\xB3\x8B\0\0\xB4\x8B\0\0\xB6\x8B\0\0\xB7\x8B\0\0\xB9\x8B\0\0\xBA\x8B\0\0\xBC\x8B\0\0\xBE\x8B\0\0\xC0\x8B\0\0\xC1\x8B\0\0\xC8\x8B\0\0\xC9\x8B\0\0\xCA\x8B\0\0\xCB\x8B\0\0\xE1\x8B\0\0\xE2\x8B\0\0\xEB\x8B\0\0\xED\x8B\0\0\xF1\x8B\0\0\xF3\x8B\0\0\xF5\x8B\0\0\xF6\x8B\0\0\xFD\x8B\0\0\xFE\x8B\0\0\x06\x8C\0\0\x07\x8C\0\0\r\x8C\0\0\x0F\x8C\0\0\x10\x8C\0\0\x11\x8C\0\0\x1A\x8C\0\0\x1B\x8C\0\0#\x8C\0\0%\x8C\0\0&\x8C\0\0'\x8C\0\0,\x8C\0\0.\x8C\0\x004\x8C\0\x005\x8C\0\0A\x8C\0\0B\x8C\0\0L\x8C\0\0M\x8C\0\0Z\x8C\0\0[\x8C\0\0k\x8C\0\0l\x8C\0\0y\x8C\0\0{\x8C\0\0&\x8D\0\0'\x8D\0\0+\x8D\0\0-\x8D\0\0.\x8D\0\0/\x8D\0\x000\x8D\0\x001\x8D\0\x007\x8D\0\08\x8D\0\0;\x8D\0\0<\x8D\0\0?\x8D\0\0@\x8D\0\0A\x8D\0\0D\x8D\0\0N\x8D\0\0O\x8D\0\0X\x8D\0\0Y\x8D\0\0a\x8D\0\0b\x8D\0\0c\x8D\0\0d\x8D\0\0f\x8D\0\0g\x8D\0\0t\x8D\0\0u\x8D\0\0v\x8D\0\0w\x8D\0\0\x9F\x8D\0\0\xA0\x8D\0\0\xB4\x8D\0\0\xB5\x8D\0\0\xBE\x8D\0\0\xBF\x8D\0\0\xC6\x8D\0\0\xC7\x8D\0\0\xCB\x8D\0\0\xCC\x8D\0\0\xDB\x8D\0\0\xDC\x8D\0\0\xE4\x8D\0\0\xE5\x8D\0\0\xE8\x8D\0\0\xE9\x8D\0\0\xEA\x8D\0\0\xEB\x8D\0\0\xF5\x8D\0\0\xF6\x8D\0\0\xF7\x8D\0\0\xF8\x8D\0\0\xFA\x8D\0\0\xFB\x8D\0\0\n\x8E\0\0\x0B\x8E\0\0*\x8E\0\0+\x8E\0\x001\x8E\0\x002\x8E\0\0B\x8E\0\0C\x8E\0\0D\x8E\0\0E\x8E\0\0H\x8E\0\0I\x8E\0\0K\x8E\0\0L\x8E\0\0f\x8E\0\0g\x8E\0\0l\x8E\0\0n\x8E\0\0r\x8E\0\0s\x8E\0\0\x81\x8E\0\0\x82\x8E\0\0\x8F\x8E\0\0\x90\x8E\0\0\xAC\x8E\0\0\xAD\x8E\0\0\xAF\x8E\0\0\xB0\x8E\0\0\xBA\x8E\0\0\xBB\x8E\0\0g\x8F\0\0h\x8F\0\0t\x8F\0\0u\x8F\0\0\x7F\x8F\0\0\x80\x8F\0\0\x90\x8F\0\0\x91\x8F\0\0\x96\x8F\0\0\x98\x8F\0\0\x99\x8F\0\0\x9A\x8F\0\0\x9C\x8F\0\0\x9D\x8F\0\0\x9F\x8F\0\0\xA0\x8F\0\0\xA3\x8F\0\0\xA4\x8F\0\0\xAB\x8F\0\0\xAC\x8F\0\0\xBD\x8F\0\0\xBE\x8F\0\0\xC2\x8F\0\0\xC3\x8F\0\0\xC4\x8F\0\0\xC5\x8F\0\0\xE2\x8F\0\0\xE3\x8F\0\0\xED\x8F\0\0\xEE\x8F\0\0\xF9\x8F\0\0\xFA\x8F\0\0\x17\x90\0\0\x18\x90\0\0\x1E\x90\0\0\x1F\x90\0\0.\x90\0\0/\x90\0\0>\x90\0\0?\x90\0\0B\x90\0\0C\x90\0\0O\x90\0\0P\x90\0\0c\x90\0\0d\x90\0\0e\x90\0\0f\x90\0\0\x91\x90\0\0\x92\x90\0\0\xC1\x90\0\0\xC2\x90\0\0\xCA\x90\0\0\xCB\x90\0\0\x19\x91\0\0\x1A\x91\0\0L\x91\0\0M\x91\0\0W\x91\0\0X\x91\0\0]\x91\0\0^\x91\0\0b\x91\0\0d\x91\0\0e\x91\0\0f\x91\0\0j\x91\0\0k\x91\0\0l\x91\0\0m\x91\0\0q\x91\0\0r\x91\0\0u\x91\0\0v\x91\0\0\x7F\x91\0\0\x80\x91\0\0\x87\x91\0\0\x88\x91\0\0\x8B\x91\0\0\x8C\x91\0\0\xBA\x91\0\0\xBB\x91\0\0t\x92\0\0u\x92\0\0\x89\x94\0\0\x8A\x94\0\0\x99\x94\0\0\x9A\x94\0\0\x9D\x94\0\0\x9F\x94\0\0\xA0\x94\0\0\xA1\x94\0\0\xA5\x94\0\0\xA6\x94\0\0\xA7\x94\0\0\xA8\x94\0\0\xA9\x94\0\0\xAA\x94\0\0\xAE\x94\0\0\xB0\x94\0\0\xB3\x94\0\0\xB4\x94\0\0\xBE\x94\0\0\xBF\x94\0\0\xC2\x94\0\0\xC3\x94\0\0\xC5\x94\0\0\xC6\x94\0\0\xD0\x94\0\0\xD1\x94\0\0\xDB\x94\0\0\xDC\x94\0\0\xDD\x94\0\0\xDE\x94\0\0\xF0\x94\0\0\xF1\x94\0\0\xF2\x94\0\0\xF3\x94\0\0\xF8\x94\0\0\xF9\x94\0\0\x04\x95\0\0\x05\x95\0\0\x08\x95\0\0\t\x95\0\0\x0C\x95\0\0\r\x95\0\0\x10\x95\0\0\x12\x95\0\0\x1A\x95\0\0\x1B\x95\0\0#\x95\0\0&\x95\0\0/\x95\0\x001\x95\0\09\x95\0\0:\x95\0\0;\x95\0\0<\x95\0\0@\x95\0\0A\x95\0\0P\x95\0\0R\x95\0\0V\x95\0\0W\x95\0\0p\x95\0\0q\x95\0\0v\x95\0\0w\x95\0\0\xEF\x95\0\0\xF0\x95\0\0\xF8\x95\0\0\xF9\x95\0\0\xFA\x95\0\0\xFB\x95\0\0\xFD\x95\0\0\xFE\x95\0\0\0\x96\0\0\x01\x96\0\0\x0E\x96\0\0\x0F\x96\0\x001\x96\0\x002\x96\0\0K\x96\0\0M\x96\0\0U\x96\0\0V\x96\0\0a\x96\0\0b\x96\0\0h\x96\0\0i\x96\0\0\x85\x96\0\0\x86\x96\0\0\x8B\x96\0\0\x8C\x96\0\0\x98\x96\0\0\x9A\x96\0\0\xA7\x96\0\0\xA8\x96\0\0\xB6\x96\0\0\xB7\x96\0\0\xC0\x96\0\0\xC2\x96\0\0\xC7\x96\0\0\xC8\x96\0\0\xCC\x96\0\0\xCD\x96\0\0\xCF\x96\0\0\xD0\x96\0\0\xD5\x96\0\0\xD6\x96\0\0\xF9\x96\0\0\xFA\x96\0\0\x04\x97\0\0\x05\x97\0\0\t\x97\0\0\n\x97\0\0\x0E\x97\0\0\x0F\x97\0\0\x1C\x97\0\0\x1D\x97\0\0\x1E\x97\0\0\x1F\x97\0\0>\x97\0\0?\x97\0\0a\x97\0\0b\x97\0\0t\x97\0\0u\x97\0\0v\x97\0\0w\x97\0\0\x8D\x97\0\0\x8E\x97\0\0\xA0\x97\0\0\xA1\x97\0\0\xAD\x97\0\0\xAE\x97\0\0\xE7\x97\0\0\xE8\x97\0\0\xED\x97\0\0\xEE\x97\0\0\xF5\x97\0\0\xF6\x97\0\0w\x98\0\0x\x98\0\0\x81\x98\0\0\x83\x98\0\0\x85\x98\0\0\x86\x98\0\0\x88\x98\0\0\x89\x98\0\0\x8A\x98\0\0\x8B\x98\0\0\x93\x98\0\0\x94\x98\0\0\x96\x98\0\0\x97\x98\0\0\x9C\x98\0\0\x9D\x98\0\0\xA0\x98\0\0\xA1\x98\0\0\xA4\x98\0\0\xA5\x98\0\0\xD3\x98\0\0\xD4\x98\0\0e\x99\0\0f\x99\0\0j\x99\0\0k\x99\0\0r\x99\0\0s\x99\0\0u\x99\0\0w\x99\0\0z\x99\0\0{\x99\0\0\x7F\x99\0\0\x80\x99\0\0\x81\x99\0\0\x82\x99\0\0\x85\x99\0\0\x86\x99\0\0\x88\x99\0\0\x89\x99\0\0\x8B\x99\0\0\x8C\x99\0\0\x8D\x99\0\0\x8E\x99\0\0\x8F\x99\0\0\x90\x99\0\0\x92\x99\0\0\x93\x99\0\0n\x9A\0\0q\x9A\0\0s\x9A\0\0u\x9A\0\0y\x9A\0\0z\x9A\0\0|\x9A\0\0}\x9A\0\0\x82\x9A\0\0\x83\x9A\0\0\x84\x9A\0\0\x85\x9A\0\0\x86\x9A\0\0\x88\x9A\0\0\x8F\x9A\0\0\x90\x9A\0\0\xA1\x9A\0\0\xA2\x9A\0\0\xB0\x9A\0\0\xB1\x9A\0\0\xB7\x9A\0\0\xB8\x9A\0\0\xBC\x9A\0\0\xBD\x9A\0\0\xC5\x9A\0\0\xC6\x9A\0\0\xD3\x9A\0\0\xD4\x9A\0\0\xE6\x9A\0\0\xE7\x9A\0\0\x08\x9B\0\0\t\x9B\0\0\x13\x9B\0\0\x14\x9B\0\0A\x9B\0\0B\x9B\0\0D\x9B\0\0E\x9B\0\0O\x9B\0\0P\x9B\0\0\x7F\x9C\0\0\x80\x9C\0\0\x8D\x9C\0\0\x8E\x9C\0\0\xA4\x9C\0\0\xA5\x9C\0\0\xA8\x9C\0\0\xA9\x9C\0\0\xAB\x9C\0\0\xAC\x9C\0\0\xB8\x9C\0\0\xB9\x9C\0\0\xC4\x9C\0\0\xC5\x9C\0\0\xCD\x9C\0\0\xCE\x9C\0\0\xD6\x9C\0\0\xD7\x9C\0\0\xDE\x9C\0\0\xDF\x9C\0\0\"\x9E\0\0#\x9E\0\0%\x9E\0\0'\x9E\0\0/\x9E\0\x000\x9E\0\x003\x9E\0\x004\x9E\0\x005\x9E\0\x006\x9E\0\0=\x9E\0\0>\x9E\0\0C\x9E\0\0D\x9E\0\0I\x9E\0\0K\x9E\0\0O\x9E\0\0P\x9E\0\0f\x9E\0\0g\x9E\0\0\xCF\x9E\0\0\xD0\x9E\0\0\xD4\x9E\0\0\xD5\x9E\0\0\xDB\x9E\0\0\xDC\x9E\0\0\xEF\x9E\0\0\xF0\x9E\0\0\x0E\x9F\0\0\x0F\x9F\0\0,\x9F\0\0-\x9F\0\0\x87\x9F\0\0\x88\x9F\0\0") }, 1465u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static JA: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x11N\0\0\x12N\0\0\xA5N\0\0\xA6N\0\0\xA8N\0\0\xA9N\0\0LQ\0\0MQ\0\0NQ\0\0OQ\0\0\xE7Q\0\0\xE8Q\0\0CR\0\0DR\0\0oS\0\0pS\0\0\tV\0\0\nV\0\0\x14V\0\0\x15V\0\0\x18V\0\0\x19V\0\0\xECX\0\0\xEDX\0\0\xFAX\0\0\xFBX\0\0\t[\0\0\n[\0\0\xC5[\0\0\xC6[\0\0\xF3]\0\0\xF4]\0\0\x9A^\0\0\x9B^\0\0\xB5^\0\0\xB6^\0\0\x18_\0\0\x19_\0\0W_\0\0X_\0\0\xB6`\0\0\xB7`\0\0\x15a\0\0\x16a\0\0\nb\0\0\x0Bb\0\0\x0Cb\0\0\rb\0\0\xFCb\0\0\xFDb\0\0\xC3c\0\0\xC4c\0\0\xA7e\0\0\xA8e\0\0\x0Cf\0\0\rf\0\0Vg\0\0Wg\0\0vh\0\0wh\0\0\xB5h\0\0\xB6h\0\0Ti\0\0Ui\0\0Xn\0\0Yn\0\0\x1Aq\0\0\x1Bq\0\0\xEDq\0\0\xEEq\0\0,r\0\0-r\0\0Lr\0\0Mr\0\0]r\0\0^r\0\0ar\0\0br\0\0\xD0r\0\0\xD1r\0\0\xD7r\0\0\xD8r\0\0\xFCr\0\0\xFDr\0\0*s\0\0+s\0\0Es\0\0Fs\0\0xv\0\0yv\0\0\x91w\0\0\x92w\0\0\x87x\0\0\x88x\0\0Zy\0\0[y\0\0\x84y\0\0\x85y\0\0\x8Ey\0\0\x8Fy\0\0\xE4y\0\0\xE5y\0\0\xFFz\0\0\0{\0\0F}\0\0G}\0\0M~\0\0N~\0\0k\x7F\0\0l\x7F\0\0\x8F\x81\0\0\x90\x81\0\0\x92\x82\0\0\x93\x82\0\0\xC4\x87\0\0\xC5\x87\0\0\xF9\x87\0\0\xFA\x87\0\0\r\x88\0\0\x0E\x88\0\0#\x88\0\0$\x88\0\0\x1B\x8D\0\0\x1C\x8D\0\0D\x8E\0\0E\x8E\0\0\xB0\x8F\0\0\xB1\x8F\0\0I\x91\0\0J\x91\0\0\xF2\x92\0\0\xF3\x92\0\0\x04\x93\0\0\x05\x93\0\0(\x93\0\0)\x93\0\0\x8F\x95\0\0\x90\x95\0\0\xA9\x95\0\0\xAA\x95\0\0\xC0\x96\0\0\xC1\x96\0\0\xC9\x96\0\0\xCA\x96\0\0\xF3\x9C\0\0\xF4\x9C\0\0 \x9F\0\0!\x9F\0\0\x8D\x9F\0\0\x8E\x9F\0\0") }, 75u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HY: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x87\x05\0\0\x88\x05\0\0") }, 1u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SI: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x8E\r\0\0\x91\r\0\0\xA6\r\0\0\xA7\r\0\0\xF3\r\0\0\xF4\r\0\0\x0B \0\0\x0E \0\0") }, 8u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TT: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x93\x04\0\0\x94\x04\0\0\x9B\x04\0\0\x9C\x04\0\0") }, 2u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MN: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x97\x04\0\0\x98\x04\0\0\xBB\x04\0\0\xBC\x04\0\0\xCA\x04\0\0\xCB\x04\0\0\xD9\x04\0\0\xDA\x04\0\0") }, 4u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KM: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x9D\x17\0\0\x9F\x17\0\0\xB4\x17\0\0\xB6\x17\0\0\xCC\x17\0\0\xCD\x17\0\0\xCE\x17\0\0\xD0\x17\0\0\xD1\x17\0\0\xD2\x17\0\0\x0B \0\0\x0C \0\0") }, 9u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static IT: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xAA\0\0\0\xAB\0\0\0\xBA\0\0\0\xBB\0\0\0\xDF\0\0\0\xE0\0\0\0\xE1\0\0\0\xE8\0\0\0\xEA\0\0\0\xEC\0\0\0\xED\0\0\0\xF0\0\0\0\xF1\0\0\0\xF2\0\0\0\xF4\0\0\0\xF7\0\0\0\xF8\0\0\0\xF9\0\0\0\xFA\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0S\x01\0\0T\x01\0\0") }, 25u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static GL: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xAA\0\0\0\xAB\0\0\0\xBA\0\0\0\xBB\0\0\0\xE0\0\0\0\xE1\0\0\0\xE2\0\0\0\xE9\0\0\0\xEA\0\0\0\xED\0\0\0\xEE\0\0\0\xEF\0\0\0\xF2\0\0\0\xF3\0\0\0\xF4\0\0\0\xF7\0\0\0\xF8\0\0\0\xFA\0\0\0\xFB\0\0\0\xFC\0\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0Q\x02\0\0R\x02\0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static ES: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xAA\0\0\0\xAB\0\0\0\xBA\0\0\0\xBB\0\0\0\xE0\0\0\0\xE1\0\0\0\xE2\0\0\0\xE9\0\0\0\xEA\0\0\0\xED\0\0\0\xEE\0\0\0\xF0\0\0\0\xF2\0\0\0\xF3\0\0\0\xF4\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xFA\0\0\0\xFB\0\0\0\xFC\0\0\0\xFD\0\0\0\xFE\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 34u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PT: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xAA\0\0\0\xAB\0\0\0\xBA\0\0\0\xBB\0\0\0\xE4\0\0\0\xE7\0\0\0\xE8\0\0\0\xE9\0\0\0\xEB\0\0\0\xED\0\0\0\xEE\0\0\0\xF0\0\0\0\xF1\0\0\0\xF2\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xFA\0\0\0\xFB\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 28u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AS: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xB0\t\0\0\xB1\t\0\0\xCE\t\0\0\xCF\t\0\0\xF2\t\0\0\xF3\t\0\0\x0C \0\0\x0E \0\0") }, 5u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HE: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xB0\x05\0\0\xBA\x05\0\0\xBB\x05\0\0\xBE\x05\0\0\xBF\x05\0\0\xC0\x05\0\0\xC1\x05\0\0\xC3\x05\0\0\xC4\x05\0\0\xC5\x05\0\0\xF4\x05\0\0\xF5\x05\0\0\x0E \0\0\x10 \0\0") }, 20u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static CA: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xBA\0\0\0\xBB\0\0\0\xE1\0\0\0\xE7\0\0\0\xEA\0\0\0\xED\0\0\0\xEE\0\0\0\xEF\0\0\0\xF1\0\0\0\xF2\0\0\0\xF4\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xFA\0\0\0\xFB\0\0\0\xFC\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0@\x01\0\0A\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 30u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static LO: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xD0\x0E\0\0\xDA\x0E\0\0\x0B \0\0\x0C \0\0") }, 11u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PS: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xD2\x06\0\0\xD3\x06\0\0\x0C \0\0\x10 \0\0") }, 5u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KN: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xDE\x0C\0\0\xDF\x0C\0\0\x0C \0\0\x0E \0\0") }, 3u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FR: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xDF\0\0\0\xE0\0\0\0\xE1\0\0\0\xE2\0\0\0\xE3\0\0\0\xE6\0\0\0\xEC\0\0\0\xEE\0\0\0\xF1\0\0\0\xF4\0\0\0\xF5\0\0\0\xF7\0\0\0\xF8\0\0\0\xF9\0\0\0\xFA\0\0\0\xFB\0\0\0\x01\x01\0\0\x02\x01\0\0\x07\x01\0\0\x08\x01\0\0\x13\x01\0\0\x14\x01\0\0+\x01\0\0,\x01\0\x003\x01\0\x004\x01\0\0Y\x01\0\0Z\x01\0\0a\x01\0\0b\x01\0\0\x7F\x01\0\0\x80\x01\0\0\xD4\x01\0\0\xD5\x01\0\0") }, 23u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FI: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xDF\0\0\0\xE4\0\0\0\xE6\0\0\0\xEC\0\0\0\xED\0\0\0\xF6\0\0\0\xF8\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x05\x01\0\0\x06\x01\0\0\x07\x01\0\0\x08\x01\0\0\x0B\x01\0\0\x0C\x01\0\0\r\x01\0\0\x0E\x01\0\0\x0F\x01\0\0\x10\x01\0\0\x11\x01\0\0\x12\x01\0\0\x13\x01\0\0\x14\x01\0\0\x17\x01\0\0\x18\x01\0\0\x19\x01\0\0\x1A\x01\0\0\x1B\x01\0\0\x1C\x01\0\0\x1F\x01\0\0 \x01\0\0#\x01\0\0$\x01\0\0'\x01\0\0(\x01\0\0+\x01\0\0,\x01\0\0/\x01\0\x002\x01\0\x007\x01\0\08\x01\0\0:\x01\0\0;\x01\0\0<\x01\0\0=\x01\0\0>\x01\0\0?\x01\0\0B\x01\0\0C\x01\0\0D\x01\0\0E\x01\0\0F\x01\0\0G\x01\0\0H\x01\0\0I\x01\0\0K\x01\0\0L\x01\0\0Q\x01\0\0R\x01\0\0S\x01\0\0T\x01\0\0U\x01\0\0V\x01\0\0Y\x01\0\0Z\x01\0\0[\x01\0\0\\\x01\0\0]\x01\0\0^\x01\0\0_\x01\0\0`\x01\0\0c\x01\0\0d\x01\0\0e\x01\0\0f\x01\0\0g\x01\0\0h\x01\0\0k\x01\0\0l\x01\0\0o\x01\0\0p\x01\0\0q\x01\0\0r\x01\0\0s\x01\0\0t\x01\0\0z\x01\0\0{\x01\0\0|\x01\0\0}\x01\0\0\xE5\x01\0\0\xE6\x01\0\0\xE7\x01\0\0\xE8\x01\0\0\xE9\x01\0\0\xEA\x01\0\0\xEF\x01\0\0\xF0\x01\0\0\x19\x02\0\0\x1A\x02\0\0\x1B\x02\0\0\x1C\x02\0\0\x1F\x02\0\0 \x02\0\0\x92\x02\0\0\x93\x02\0\0") }, 80u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HSB: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xDF\0\0\0\xF0\0\0\0\xF1\0\0\0\xF3\0\0\0\xF4\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xFE\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x05\x01\0\0\x06\x01\0\0\x0F\x01\0\0\x10\x01\0\0\x11\x01\0\0\x12\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0\x17\x01\0\0\x18\x01\0\0\x19\x01\0\0\x1A\x01\0\0\x1F\x01\0\0 \x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\x000\x01\0\x002\x01\0\0:\x01\0\0;\x01\0\0>\x01\0\0?\x01\0\0H\x01\0\0I\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0Q\x01\0\0R\x01\0\0S\x01\0\0T\x01\0\0U\x01\0\0V\x01\0\0[\x01\0\0\\\x01\0\0_\x01\0\0`\x01\0\0e\x01\0\0f\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0o\x01\0\0p\x01\0\0q\x01\0\0r\x01\0\0z\x01\0\0{\x01\0\0|\x01\0\0}\x01\0\0") }, 59u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static DSB: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xDF\0\0\0\xF0\0\0\0\xF1\0\0\0\xF3\0\0\0\xF4\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xFE\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x05\x01\0\0\x06\x01\0\0\x0F\x01\0\0\x10\x01\0\0\x11\x01\0\0\x12\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0\x17\x01\0\0\x18\x01\0\0\x19\x01\0\0\x1A\x01\0\0\x1F\x01\0\0 \x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\x000\x01\0\x002\x01\0\0:\x01\0\0;\x01\0\0>\x01\0\0?\x01\0\0H\x01\0\0I\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0Q\x01\0\0R\x01\0\0S\x01\0\0T\x01\0\0Y\x01\0\0Z\x01\0\0_\x01\0\0`\x01\0\0e\x01\0\0f\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0o\x01\0\0p\x01\0\0q\x01\0\0r\x01\0\0|\x01\0\0}\x01\0\0") }, 57u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SK: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE0\0\0\0\xE1\0\0\0\xE2\0\0\0\xE3\0\0\0\xE5\0\0\0\xE9\0\0\0\xEA\0\0\0\xED\0\0\0\xEE\0\0\0\xF0\0\0\0\xF1\0\0\0\xF3\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xFA\0\0\0\xFB\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0Q\x01\0\0R\x01\0\0S\x01\0\0T\x01\0\0Y\x01\0\0Z\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0q\x01\0\0r\x01\0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static NL: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE0\0\0\0\xE1\0\0\0\xE2\0\0\0\xE4\0\0\0\xE5\0\0\0\xE9\0\0\0\xEA\0\0\0\xEB\0\0\0\xEE\0\0\0\xEF\0\0\0\xF1\0\0\0\xF2\0\0\0\xF4\0\0\0\xF5\0\0\0\xF8\0\0\0\xFA\0\0\0\xFB\0\0\0\xFC\0\0\0\xFD\0\0\0\xFE\0\0\0\xFF\0\0\0\0\x01\0\0S\x01\0\0T\x01\0\0") }, 17u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static CS: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE0\0\0\0\xE1\0\0\0\xE2\0\0\0\xE9\0\0\0\xEA\0\0\0\xED\0\0\0\xEE\0\0\0\xF0\0\0\0\xF1\0\0\0\xF3\0\0\0\xF4\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xFA\0\0\0\xFB\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0>\x01\0\0?\x01\0\0B\x01\0\0C\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0U\x01\0\0V\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 36u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AF: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE0\0\0\0\xE1\0\0\0\xE3\0\0\0\xE8\0\0\0\xEC\0\0\0\xEE\0\0\0\xF2\0\0\0\xF4\0\0\0\xF9\0\0\0\xFB\0\0\0\xFC\0\0\0\xFE\0\0\0") }, 14u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static RO: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE0\0\0\0\xE2\0\0\0\xE4\0\0\0\xE6\0\0\0\xE7\0\0\0\xEC\0\0\0\xF1\0\0\0\xF2\0\0\0\xF6\0\0\0\xF7\0\0\0\xFC\0\0\0\xFD\0\0\0_\x01\0\0`\x01\0\0c\x01\0\0d\x01\0\0") }, 14u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static DA: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE0\0\0\0\xE3\0\0\0\xE4\0\0\0\xE5\0\0\0\xE7\0\0\0\xEC\0\0\0\xED\0\0\0\xF0\0\0\0\xF1\0\0\0\xF2\0\0\0\xF3\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF9\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0S\x01\0\0T\x01\0\0\xFF\x01\0\0\0\x02\0\0") }, 23u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static EN_ZA: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE0\0\0\0\xE3\0\0\0\xE4\0\0\0\xF0\0\0\0\xF1\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0a\x01\0\0b\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0\x13\x1E\0\0\x14\x1E\0\0=\x1E\0\0>\x1E\0\0E\x1E\0\0F\x1E\0\0K\x1E\0\0L\x1E\0\0q\x1E\0\0r\x1E\0\0") }, 43u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FIL: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE0\0\0\0\xE3\0\0\0\xE8\0\0\0\xEB\0\0\0\xEC\0\0\0\xEF\0\0\0\xF2\0\0\0\xF5\0\0\0\xF9\0\0\0\xFC\0\0\0") }, 15u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static ET: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE0\0\0\0\xE4\0\0\0\xE5\0\0\0\xF0\0\0\0\xF1\0\0\0\xF5\0\0\0\xF8\0\0\0\xFC\0\0\0\x01\x01\0\0\x02\x01\0\0\x13\x01\0\0\x14\x01\0\0+\x01\0\0,\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0") }, 30u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static DE: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE0\0\0\0\xE4\0\0\0\xE5\0\0\0\xF0\0\0\0\xF1\0\0\0\xF5\0\0\0\xF8\0\0\0\xFC\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0\x1F\x01\0\0 \x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\x000\x01\0\x002\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0_\x01\0\0`\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 39u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static EU: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE0\0\0\0\xE7\0\0\0\xE8\0\0\0\xF0\0\0\0\xF2\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 36u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SU: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE0\0\0\0\xE9\0\0\0\xEA\0\0\0\xF0\0\0\0\xF1\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 37u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static EN: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE0\0\0\0\xF0\0\0\0\xF1\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 38u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static NO: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE1\0\0\0\xE2\0\0\0\xE3\0\0\0\xE5\0\0\0\xE7\0\0\0\xE9\0\0\0\xEA\0\0\0\xEB\0\0\0\xED\0\0\0\xEE\0\0\0\xF1\0\0\0\xF2\0\0\0\xF6\0\0\0\xF7\0\0\0\xFA\0\0\0\xFB\0\0\0\xFC\0\0\0\xFD\0\0\0\r\x01\0\0\x0E\x01\0\0\x11\x01\0\0\x12\x01\0\0D\x01\0\0E\x01\0\0K\x01\0\0L\x01\0\0a\x01\0\0b\x01\0\0g\x01\0\0h\x01\0\0~\x01\0\0\x7F\x01\0\0\xCE\x01\0\0\xCF\x01\0\0") }, 19u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FR_CA: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE1\0\0\0\xE2\0\0\0\xE3\0\0\0\xE6\0\0\0\xEC\0\0\0\xEE\0\0\0\xF1\0\0\0\xF4\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xF9\0\0\0\xFA\0\0\0\xFB\0\0\0\x01\x01\0\0\x02\x01\0\0\x13\x01\0\0\x14\x01\0\0+\x01\0\0,\x01\0\0\xD4\x01\0\0\xD5\x01\0\0") }, 16u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static NN: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE1\0\0\0\xE2\0\0\0\xE4\0\0\0\xE5\0\0\0\xE7\0\0\0\xE9\0\0\0\xEA\0\0\0\xEB\0\0\0\xF1\0\0\0\xF2\0\0\0\xF6\0\0\0\xF7\0\0\0\xFC\0\0\0\xFD\0\0\0\r\x01\0\0\x0E\x01\0\0\x11\x01\0\0\x12\x01\0\0D\x01\0\0E\x01\0\0K\x01\0\0L\x01\0\0a\x01\0\0b\x01\0\0g\x01\0\0h\x01\0\0~\x01\0\0\x7F\x01\0\0\xCE\x01\0\0\xCF\x01\0\0") }, 16u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static RM: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE1\0\0\0\xE3\0\0\0\xE4\0\0\0\xE8\0\0\0\xEA\0\0\0\xEC\0\0\0\xED\0\0\0\xF0\0\0\0\xF1\0\0\0\xF2\0\0\0\xF3\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xF9\0\0\0\xFA\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 31u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SV: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE1\0\0\0\xE4\0\0\0\xE6\0\0\0\xE9\0\0\0\xEB\0\0\0\xEC\0\0\0\xED\0\0\0\xF0\0\0\0\xF1\0\0\0\xF2\0\0\0\xF3\0\0\0\xF4\0\0\0\xF8\0\0\0\xF9\0\0\0\xFA\0\0\0\xFB\0\0\0\xFC\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0+\x01\0\0,\x01\0\0") }, 18u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static GU: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xF0\n\0\0\xF1\n\0\0\x0C \0\0\x0E \0\0") }, 3u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BN: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xF0\t\0\0\xFA\t\0\0\x0C \0\0\x0E \0\0") }, 12u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HI_LATN: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xF1\0\0\0\xF2\0\0\0\x01\x01\0\0\x02\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0[\x01\0\0\\\x01\0\0k\x01\0\0l\x01\0\0\r\x1E\0\0\x0E\x1E\0\0%\x1E\0\0&\x1E\0\x007\x1E\0\08\x1E\0\0A\x1E\0\0B\x1E\0\0E\x1E\0\0F\x1E\0\0G\x1E\0\0H\x1E\0\0[\x1E\0\0\\\x1E\0\0c\x1E\0\0d\x1E\0\0m\x1E\0\0n\x1E\0\0") }, 18u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x03\0\x06\0\t\0l\xCC\xA5m\xCC\x90r\xCC\xA5r\xCC\xA5\xCC\x84") },
                ));
                static KA: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xF1\x10\0\0\xFB\x10\0\0\0-\0\0&-\0\0") }, 48u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SO: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"a\0\0\0b\0\0\0e\0\0\0f\0\0\0i\0\0\0j\0\0\0o\0\0\0q\0\0\0u\0\0\0w\0\0\0z\0\0\0{\0\0\0") }, 8u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MI: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"b\0\0\0e\0\0\0f\0\0\0h\0\0\0j\0\0\0k\0\0\0l\0\0\0m\0\0\0q\0\0\0r\0\0\0s\0\0\0t\0\0\0v\0\0\0w\0\0\0x\0\0\0{\0\0\0") }, 13u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TO: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"b\0\0\0e\0\0\0g\0\0\0h\0\0\0j\0\0\0k\0\0\0q\0\0\0s\0\0\0w\0\0\0{\0\0\0\xE0\0\0\0\xE1\0\0\0\xE2\0\0\0\xE3\0\0\0\xE4\0\0\0\xE9\0\0\0\xEA\0\0\0\xED\0\0\0\xEE\0\0\0\xF0\0\0\0\xF1\0\0\0\xF3\0\0\0\xF4\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xFA\0\0\0\xFB\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x03\x01\0\0\x04\x01\0\0\x15\x01\0\0\x16\x01\0\0-\x01\0\0.\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0m\x01\0\0n\x01\0\0") }, 38u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KGP: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"b\0\0\0e\0\0\0l\0\0\0m\0\0\0q\0\0\0r\0\0\0w\0\0\0y\0\0\0z\0\0\0{\0\0\0\xAA\0\0\0\xAB\0\0\0\xBA\0\0\0\xBB\0\0\0\xE0\0\0\0\xE1\0\0\0\xE2\0\0\0\xE3\0\0\0\xE4\0\0\0\xE9\0\0\0\xEA\0\0\0\xF0\0\0\0\xF1\0\0\0\xF3\0\0\0\xF4\0\0\0\xF7\0\0\0\xF8\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0\xB0\x02\0\0\xB1\x02\0\0I\x1D\0\0J\x1D\0\0M\x1D\0\0N\x1D\0\0\x7F \0\0\x80 \0\0") }, 49u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static QU: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"b\0\0\0h\0\0\0j\0\0\0k\0\0\0o\0\0\0p\0\0\0r\0\0\0s\0\0\0v\0\0\0w\0\0\0x\0\0\0y\0\0\0z\0\0\0{\0\0\0\xE0\0\0\0\xF0\0\0\0\xF2\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 49u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static YRL: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"c\0\0\0d\0\0\0f\0\0\0g\0\0\0h\0\0\0i\0\0\0j\0\0\0k\0\0\0l\0\0\0m\0\0\0o\0\0\0p\0\0\0q\0\0\0r\0\0\0v\0\0\0w\0\0\0z\0\0\0{\0\0\0\xAA\0\0\0\xAB\0\0\0\xBA\0\0\0\xBB\0\0\0\xE0\0\0\0\xE3\0\0\0\xE4\0\0\0\xF0\0\0\0\xF1\0\0\0\xF7\0\0\0\xF8\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0\xF9\x1E\0\0\xFA\x1E\0\0") }, 50u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static CEB: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"c\0\0\0d\0\0\0f\0\0\0g\0\0\0j\0\0\0k\0\0\0q\0\0\0r\0\0\0v\0\0\0w\0\0\0x\0\0\0y\0\0\0z\0\0\0{\0\0\0\xF1\0\0\0\xF2\0\0\0") }, 8u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BR: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"c\0\0\0d\0\0\0q\0\0\0r\0\0\0\xE0\0\0\0\xEA\0\0\0\xEB\0\0\0\xF0\0\0\0\xF2\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xF9\0\0\0\xFA\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 37u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TK: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"c\0\0\0d\0\0\0q\0\0\0r\0\0\0v\0\0\0w\0\0\0x\0\0\0y\0\0\0") }, 4u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static YO: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"c\0\0\0d\0\0\0q\0\0\0r\0\0\0v\0\0\0w\0\0\0x\0\0\0y\0\0\0z\0\0\0{\0\0\0") }, 5u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KEA: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"c\0\0\0d\0\0\0q\0\0\0r\0\0\0w\0\0\0x\0\0\0\xAA\0\0\0\xAB\0\0\0\xBA\0\0\0\xBB\0\0\0\xE0\0\0\0\xF0\0\0\0\xF2\0\0\0\xF7\0\0\0\xF8\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0)\x01\0\0*\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0i\x01\0\0j\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0X\x1D\0\0Y\x1D\0\0\xBD\x1E\0\0\xBE\x1E\0\0") }, 47u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0n\xCC\x88rr") },
                ));
                static IS: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"c\0\0\0d\0\0\0q\0\0\0r\0\0\0w\0\0\0x\0\0\0z\0\0\0{\0\0\0") }, 4u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FO: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"c\0\0\0d\0\0\0q\0\0\0r\0\0\0w\0\0\0y\0\0\0z\0\0\0{\0\0\0") }, 5u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SW: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"c\0\0\0d\0\0\0q\0\0\0r\0\0\0x\0\0\0y\0\0\0") }, 3u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PCM: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"c\0\0\0d\0\0\0q\0\0\0r\0\0\0x\0\0\0y\0\0\0\xE0\0\0\0\xE1\0\0\0\xE8\0\0\0\xE9\0\0\0\xEC\0\0\0\xED\0\0\0\xF2\0\0\0\xF3\0\0\0\xF9\0\0\0\xFA\0\0\0") }, 8u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x05\0\xE1\xBA\xB9\xCC\x80\xE1\xBB\x8D\xCC\x80") },
                ));
                static IG: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"c\0\0\0d\0\0\0q\0\0\0r\0\0\0x\0\0\0y\0\0\0\xE0\0\0\0\xE2\0\0\0\xE8\0\0\0\xEA\0\0\0\xEC\0\0\0\xEE\0\0\0\xF2\0\0\0\xF4\0\0\0\xF9\0\0\0\xFB\0\0\0\x01\x01\0\0\x02\x01\0\0\x13\x01\0\0\x14\x01\0\0+\x01\0\0,\x01\0\0D\x01\0\0E\x01\0\0M\x01\0\0N\x01\0\0k\x01\0\0l\x01\0\0\xF9\x01\0\0\xFA\x01\0\0?\x1E\0\0@\x1E\0\0") }, 21u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x08\0\r\0\x12\0\x17\0\x1C\0m\xCC\x80\xE1\xBB\x8B\xCC\x80\xE1\xBB\x8B\xCC\x81\xE1\xBB\x8D\xCC\x80\xE1\xBB\x8D\xCC\x81\xE1\xBB\xA5\xCC\x80\xE1\xBB\xA5\xCC\x81") },
                ));
                static UZ: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"c\0\0\0d\0\0\0w\0\0\0x\0\0\0\xE0\0\0\0\xF0\0\0\0\xF1\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 40u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static VI: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"f\0\0\0g\0\0\0j\0\0\0k\0\0\0w\0\0\0x\0\0\0z\0\0\0{\0\0\0") }, 4u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static JV: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"f\0\0\0g\0\0\0q\0\0\0r\0\0\0v\0\0\0w\0\0\0x\0\0\0y\0\0\0z\0\0\0{\0\0\0") }, 5u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TE: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"f\x0C\0\0p\x0C\0\0\x0C \0\0\x0E \0\0") }, 12u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static WO: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"h\0\0\0i\0\0\0v\0\0\0w\0\0\0z\0\0\0{\0\0\0\xE3\0\0\0\xE4\0\0\0") }, 4u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static GD: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"j\0\0\0l\0\0\0q\0\0\0r\0\0\0v\0\0\0{\0\0\0\xE1\0\0\0\xE8\0\0\0\xE9\0\0\0\xEC\0\0\0\xED\0\0\0\xF0\0\0\0\xF1\0\0\0\xF2\0\0\0\xF3\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xF9\0\0\0\xFA\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x0B\x01\0\0\x0C\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0!\x01\0\0\"\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\x001\x01\0\x002\x01\0\0B\x01\0\0C\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0_\x01\0\0`\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0\x19\x02\0\0\x1A\x02\0\0\x0B\x1E\0\0\x0C\x1E\0\0\x1F\x1E\0\0 \x1E\0\0A\x1E\0\0B\x1E\0\0W\x1E\0\0X\x1E\0\0a\x1E\0\0b\x1E\0\0k\x1E\0\0l\x1E\0\0") }, 53u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static GA: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"j\0\0\0l\0\0\0q\0\0\0r\0\0\0v\0\0\0{\0\0\0\xE5\0\0\0\xE6\0\0\0\x0B\x01\0\0\x0C\x01\0\0!\x01\0\0\"\x01\0\0\x03\x1E\0\0\x04\x1E\0\0\x0B\x1E\0\0\x0C\x1E\0\0\x1F\x1E\0\0 \x1E\0\0A\x1E\0\0B\x1E\0\0W\x1E\0\0X\x1E\0\0a\x1E\0\0b\x1E\0\0k\x1E\0\0l\x1E\0\0") }, 18u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AST: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"j\0\0\0l\0\0\0w\0\0\0x\0\0\0\xAA\0\0\0\xAB\0\0\0\xBA\0\0\0\xBB\0\0\0\xE0\0\0\0\xE1\0\0\0\xE2\0\0\0\xE9\0\0\0\xEA\0\0\0\xED\0\0\0\xEE\0\0\0\xF0\0\0\0\xF2\0\0\0\xF3\0\0\0\xF4\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xFA\0\0\0\xFB\0\0\0\xFC\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 36u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static CY: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"k\0\0\0l\0\0\0q\0\0\0r\0\0\0v\0\0\0w\0\0\0x\0\0\0y\0\0\0z\0\0\0{\0\0\0\xE3\0\0\0\xE4\0\0\0\xE5\0\0\0\xE8\0\0\0\xF1\0\0\0\xF2\0\0\0\xF8\0\0\0\xF9\0\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 22u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SC: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"k\0\0\0l\0\0\0q\0\0\0r\0\0\0w\0\0\0z\0\0\0\xAA\0\0\0\xAB\0\0\0\xBA\0\0\0\xBB\0\0\0\xDF\0\0\0\xE0\0\0\0\xE1\0\0\0\xE8\0\0\0\xE9\0\0\0\xEC\0\0\0\xED\0\0\0\xF0\0\0\0\xF1\0\0\0\xF2\0\0\0\xF3\0\0\0\xF7\0\0\0\xF8\0\0\0\xF9\0\0\0\xFA\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0S\x01\0\0T\x01\0\0") }, 32u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AR_MA: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"o\x06\0\0p\x06\0\0~\x06\0\0\x7F\x06\0\0\x86\x06\0\0\x87\x06\0\0\x98\x06\0\0\x99\x06\0\0\x9C\x06\0\0\x9D\x06\0\0\xA2\x06\0\0\xA3\x06\0\0\xA4\x06\0\0\xA6\x06\0\0\xA7\x06\0\0\xAA\x06\0\0\xAD\x06\0\0\xAE\x06\0\0\xAF\x06\0\0\xB0\x06\0\0\xCC\x06\0\0\xCD\x06\0\0c\x07\0\0d\x07\0\0\x0C \0\0\x10 \0\0") }, 19u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HA: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"p\0\0\0r\0\0\0v\0\0\0w\0\0\0x\0\0\0y\0\0\0\xE0\0\0\0\xE3\0\0\0\xE8\0\0\0\xEB\0\0\0\xEC\0\0\0\xEF\0\0\0\xF2\0\0\0\xF5\0\0\0\xF9\0\0\0\xFC\0\0\0") }, 19u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0r\xCC\x83") },
                ));
                static HA_NE: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"p\0\0\0r\0\0\0v\0\0\0w\0\0\0x\0\0\0y\0\0\0\xE0\0\0\0\xE3\0\0\0\xE8\0\0\0\xEB\0\0\0\xEC\0\0\0\xEF\0\0\0\xF2\0\0\0\xF5\0\0\0\xF9\0\0\0\xFC\0\0\0") }, 19u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0r\xCC\x83\xCA\xBCy") },
                ));
                static PL: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"q\0\0\0r\0\0\0v\0\0\0w\0\0\0x\0\0\0y\0\0\0\xDF\0\0\0\xE1\0\0\0\xE2\0\0\0\xE3\0\0\0\xE4\0\0\0\xEC\0\0\0\xEE\0\0\0\xF0\0\0\0\xF4\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF9\0\0\0\xFA\0\0\0\xFB\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0S\x01\0\0T\x01\0\0") }, 23u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TR: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"q\0\0\0r\0\0\0w\0\0\0y\0\0\0\xDF\0\0\0\xE7\0\0\0\xE8\0\0\0\xF0\0\0\0\xF1\0\0\0\xF5\0\0\0\xF8\0\0\0\xFC\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 39u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static LT: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"q\0\0\0r\0\0\0w\0\0\0y\0\0\0\xE0\0\0\0\xE2\0\0\0\xE3\0\0\0\xE4\0\0\0\xE8\0\0\0\xEA\0\0\0\xEC\0\0\0\xEE\0\0\0\xF1\0\0\0\xF4\0\0\0\xF5\0\0\0\xF6\0\0\0\xF9\0\0\0\xFB\0\0\0)\x01\0\0*\x01\0\0i\x01\0\0j\x01\0\0\xBD\x1E\0\0\xBE\x1E\0\0") }, 19u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x19\0\0\0\0\0\x02\0\x04\0\x07\0\x0C\0\x11\0\x16\0\x19\0\x1E\0!\0$\0'\0+\0/\x003\x007\0;\0?\0C\0G\0M\0S\0W\0[\0_\0chdzd\xC5\xBEi\xCC\x87\xCC\x80i\xCC\x87\xCC\x81i\xCC\x87\xCC\x83j\xCC\x83j\xCC\x87\xCC\x83l\xCC\x83m\xCC\x83r\xCC\x83\xC4\x85\xCC\x81\xC4\x85\xCC\x83\xC4\x97\xCC\x81\xC4\x97\xCC\x83\xC4\x99\xCC\x81\xC4\x99\xCC\x83\xC4\xAF\xCC\x81\xC4\xAF\xCC\x83\xC4\xAF\xCC\x87\xCC\x81\xC4\xAF\xCC\x87\xCC\x83\xC5\xAB\xCC\x81\xC5\xAB\xCC\x83\xC5\xB3\xCC\x81\xC5\xB3\xCC\x83") },
                ));
                static BS: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"q\0\0\0r\0\0\0w\0\0\0z\0\0\0") }, 4u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static LV: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"q\0\0\0r\0\0\0w\0\0\0z\0\0\0M\x01\0\0N\x01\0\0W\x01\0\0X\x01\0\0") }, 6u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HU: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"q\0\0\0r\0\0\0w\0\0\0z\0\0\0\xE0\0\0\0\xE1\0\0\0\xE2\0\0\0\xE9\0\0\0\xEA\0\0\0\xED\0\0\0\xEE\0\0\0\xF0\0\0\0\xF1\0\0\0\xF3\0\0\0\xF4\0\0\0\xF5\0\0\0\xF8\0\0\0\xFA\0\0\0\xFB\0\0\0\xFC\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 35u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SL: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"q\0\0\0r\0\0\0w\0\0\0z\0\0\0\xE0\0\0\0\xE3\0\0\0\xE4\0\0\0\xF0\0\0\0\xF1\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xFD\0\0\0\xFF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x07\x01\0\0\x08\x01\0\0\x11\x01\0\0\x12\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0S\x01\0\0T\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0") }, 43u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SR_LATN: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"q\0\0\0r\0\0\0w\0\0\0z\0\0\0\xE5\0\0\0\xE6\0\0\0") }, 5u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SW_CD: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"q\0\0\0r\0\0\0x\0\0\0y\0\0\0") }, 2u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AZ: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"w\0\0\0x\0\0\0") }, 1u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BE: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(zerovec::ZeroVec::new(), 0u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\n\0\0\0\0\0\x04\0\x08\0\x0C\0\x10\0\x14\0\x18\0\x1C\0 \0$\0\xD0\xB0\xCC\x81\xD0\xB5\xCC\x81\xD0\xBE\xCC\x81\xD1\x83\xCC\x81\xD1\x8B\xCC\x81\xD1\x8D\xCC\x81\xD1\x8E\xCC\x81\xD1\x8F\xCC\x81\xD1\x91\xCC\x81\xD1\x96\xCC\x81") },
                ));
                static CV: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(zerovec::ZeroVec::new(), 0u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\t\0\0\0\0\0\x04\0\x08\0\x0C\0\x10\0\x14\0\x18\0\x1C\0 \0\xD0\xB0\xCC\x81\xD0\xB5\xCC\x81\xD0\xB8\xCC\x81\xD0\xBE\xCC\x81\xD1\x83\xCC\x81\xD1\x8B\xCC\x81\xD1\x8D\xCC\x81\xD1\x8E\xCC\x81\xD1\x8F\xCC\x81") },
                ));
                static SR: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(zerovec::ZeroVec::new(), 0u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x04\0\x08\0\x0C\0\x10\0\xD0\xB0\xCC\x82\xD0\xB5\xCC\x82\xD0\xB8\xCC\x82\xD0\xBE\xCC\x82\xD1\x83\xCC\x82") },
                ));
                static UND: <icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable = icu::properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(zerovec::ZeroVec::new(), 0u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static VALUES: [&<icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::DataMarker>::Yokeable; 132usize] = [&AF, &AR, &AR_MA, &AS, &AST, &AZ, &BE, &BG, &BGC, &BHO, &BN, &BR, &BRX, &BS, &CA, &CEB, &CS, &CV, &CY, &DA, &DE, &DOI, &DSB, &EL, &EN, &EN_ZA, &ES, &ET, &EU, &FA, &FA_AF, &FF_ADLM, &FI, &FIL, &FO, &FR, &FR_CA, &GA, &GD, &GL, &GU, &HA, &HA_NE, &HE, &HI, &HI_LATN, &BS, &HSB, &HU, &HY, &EN, &EN, &IG, &IS, &IT, &JA, &JV, &KA, &KEA, &KGP, &KM, &KN, &KO, &HI, &KS, &BRX, &KY, &LO, &LT, &LV, &MAI, &MI, &MK, &MN, &BRX, &HI, &MY, &HI, &NL, &NN, &NO, &BRX, &PA, &PCM, &PL, &PS, &PS_PK, &PT, &QU, &RAJ, &RM, &RO, &CV, &SA, &SC, &SD, &BRX, &SI, &SK, &SL, &SO, &AZ, &SR, &SR_LATN, &SU, &SV, &SW, &SW_CD, &BRX, &TE, &TG, &TH, &TI, &TK, &TO, &TR, &TT, &UK, &UND, &UR, &UZ, &TG, &VI, &WO, &EN, &YO, &YRL, &YUE, &YUE_HANS, &ZH, &ZH_HANT, &EN];
                static KEYS: [&str; 132usize] = ["af", "ar", "ar-MA", "as", "ast", "az", "be", "bg", "bgc", "bho", "bn", "br", "brx", "bs", "ca", "ceb", "cs", "cv", "cy", "da", "de", "doi", "dsb", "el", "en", "en-ZA", "es", "et", "eu", "fa", "fa-AF", "ff-Adlm", "fi", "fil", "fo", "fr", "fr-CA", "ga", "gd", "gl", "gu", "ha", "ha-NE", "he", "hi", "hi-Latn", "hr", "hsb", "hu", "hy", "ia", "id", "ig", "is", "it", "ja", "jv", "ka", "kea", "kgp", "km", "kn", "ko", "kok", "ks", "ks-Deva", "ky", "lo", "lt", "lv", "mai", "mi", "mk", "mn", "mni", "mr", "my", "ne", "nl", "nn", "no", "or", "pa", "pcm", "pl", "ps", "ps-PK", "pt", "qu", "raj", "rm", "ro", "ru", "sa", "sc", "sd", "sd-Deva", "si", "sk", "sl", "so", "sq", "sr", "sr-Latn", "su", "sv", "sw", "sw-CD", "ta", "te", "tg", "th", "ti", "tk", "to", "tr", "tt", "uk", "und", "ur", "uz", "uz-Cyrl", "vi", "wo", "xh", "yo", "yrl", "yue", "yue-Hans", "zh", "zh-Hant", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    const FALLBACKER: icu::locid_transform::fallback::LocaleFallbackerWithConfig<'static> = icu::locid_transform::fallback::LocaleFallbacker::new().for_config(<icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker as icu_provider::KeyedDataMarker>::KEY.fallback_config());
                    let mut fallback_iterator = FALLBACKER.fallback_for(req.locale.clone());
                    loop {
                        if let Ok(payload) = KEYS.binary_search_by(|k| fallback_iterator.get().strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                            metadata.locale = Some(fallback_iterator.take());
                            break payload;
                        }
                        fallback_iterator.step();
                    }
                };
                Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata })
            }
        }
    };
}
