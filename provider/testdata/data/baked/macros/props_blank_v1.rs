// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_props_blank_v1 {
    () => {
        icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
            #[allow(unused_unsafe)]
            icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\t\0\0\0\n\0\0\0 \0\0\0!\0\0\0\xA0\0\0\0\xA1\0\0\0\x80\x16\0\0\x81\x16\0\0\0 \0\0\x0B \0\0/ \0\x000 \0\0_ \0\0` \0\0\x000\0\0\x010\0\0") }, 18usize)
        })
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __lookup_props_blank_v1 {
    ($ locale : expr) => {{
        static SINGLETON: <icu_properties::provider::BlankV1Marker as icu_provider::DataMarker>::Yokeable = singleton_props_blank_v1!();
        if $locale.is_empty() {
            Ok(&SINGLETON)
        } else {
            Err(icu_provider::DataErrorKind::ExtraneousLocale)
        }
    }};
}
/// Implement [`DataProvider<BlankV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_blank_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::BlankV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::BlankV1Marker>, icu_provider::DataError> {
                match lookup_props_blank_v1!(req.locale) {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_owned(icu_provider::prelude::zerofrom::ZeroFrom::zero_from(payload))) }),
                    Err(e) => Err(e.with_req(<icu_properties::provider::BlankV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
