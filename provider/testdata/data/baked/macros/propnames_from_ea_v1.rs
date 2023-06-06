// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_propnames_from_ea_v1 {
    () => {
        icu_properties::provider::names::PropertyValueNameToEnumMapV1 {
            map: unsafe {
                #[allow(unused_unsafe)]
                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0C\0\0\0\0\0\x01\0\n\0\x0B\0\x14\0\x15\0\x1E\0\x1F\0!\0'\0.\0/\0AAmbiguousFFullwidthHHalfwidthNNaNarrowNeutralWWide") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\x01\0\x03\0\x03\0\x02\0\x02\0\0\0\x04\0\x04\0\0\0\x05\0\x05\0") })
            },
        }
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __lookup_propnames_from_ea_v1 {
    ($ locale : expr) => {{
        static SINGLETON: <icu_properties::provider::EastAsianWidthNameToValueV1Marker as icu_provider::DataMarker>::Yokeable = singleton_propnames_from_ea_v1!();
        if $locale.is_empty() {
            Ok(&SINGLETON)
        } else {
            Err(icu_provider::DataErrorKind::ExtraneousLocale)
        }
    }};
}
/// Implement [`DataProvider<EastAsianWidthNameToValueV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_from_ea_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::EastAsianWidthNameToValueV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::EastAsianWidthNameToValueV1Marker>, icu_provider::DataError> {
                match lookup_propnames_from_ea_v1!(req.locale) {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_owned(icu_provider::prelude::zerofrom::ZeroFrom::zero_from(payload))) }),
                    Err(e) => Err(e.with_req(<icu_properties::provider::EastAsianWidthNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
