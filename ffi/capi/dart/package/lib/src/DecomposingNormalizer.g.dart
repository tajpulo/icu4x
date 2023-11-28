// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// See the [Rust documentation for `DecomposingNormalizer`](https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html) for more information.
final class DecomposingNormalizer implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  DecomposingNormalizer._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XDecomposingNormalizer_destroy'));

  /// Construct a new ICU4XDecomposingNormalizer instance for NFC
  ///
  /// See the [Rust documentation for `new_nfd`](https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.new_nfd) for more information.
  ///
  /// Throws [Error] on failure.
  factory DecomposingNormalizer.nfd(DataProvider provider) {
    final result = _ICU4XDecomposingNormalizer_create_nfd(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return DecomposingNormalizer._(result.union.ok);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XDecomposingNormalizer_create_nfd =
    _capi<ffi.NativeFunction<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>>('ICU4XDecomposingNormalizer_create_nfd')
      .asFunction<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Construct a new ICU4XDecomposingNormalizer instance for NFKC
  ///
  /// See the [Rust documentation for `new_nfkd`](https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.new_nfkd) for more information.
  ///
  /// Throws [Error] on failure.
  factory DecomposingNormalizer.nfkd(DataProvider provider) {
    final result = _ICU4XDecomposingNormalizer_create_nfkd(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return DecomposingNormalizer._(result.union.ok);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XDecomposingNormalizer_create_nfkd =
    _capi<ffi.NativeFunction<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>>('ICU4XDecomposingNormalizer_create_nfkd')
      .asFunction<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Normalize a (potentially ill-formed) UTF8 string
  ///
  /// Errors are mapped to REPLACEMENT CHARACTER
  ///
  /// See the [Rust documentation for `normalize_utf8`](https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.normalize_utf8) for more information.
  ///
  /// Throws [Error] on failure.
  String normalize(String s) {
    final temp = ffi2.Arena();
    final sLength = s.utf8Length;
    final writeable = _Writeable();
    final result = _ICU4XDecomposingNormalizer_normalize(_underlying, Utf8Encoder().allocConvert(temp, s, length: sLength), sLength, writeable._underlying);
    temp.releaseAll();
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return writeable.finalize();
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XDecomposingNormalizer_normalize =
    _capi<ffi.NativeFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size, ffi.Pointer<ffi.Opaque>)>>('ICU4XDecomposingNormalizer_normalize')
      .asFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, int, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Check if a (potentially ill-formed) UTF8 string is normalized
  ///
  /// Errors are mapped to REPLACEMENT CHARACTER
  ///
  /// See the [Rust documentation for `is_normalized_utf8`](https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.is_normalized_utf8) for more information.
  bool isNormalized(String s) {
    final temp = ffi2.Arena();
    final sLength = s.utf8Length;
    final result = _ICU4XDecomposingNormalizer_is_normalized(_underlying, Utf8Encoder().allocConvert(temp, s, length: sLength), sLength);
    temp.releaseAll();
    return result;
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XDecomposingNormalizer_is_normalized =
    _capi<ffi.NativeFunction<ffi.Bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size)>>('ICU4XDecomposingNormalizer_is_normalized')
      .asFunction<bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, int)>(isLeaf: true);
}
