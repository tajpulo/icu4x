// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// An object that runs the ICU4X locale fallback algorithm.
///
/// See the [Rust documentation for `LocaleFallbacker`](https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbacker.html) for more information.
final class LocaleFallbacker implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  LocaleFallbacker._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XLocaleFallbacker_destroy));

  /// Creates a new `LocaleFallbacker` from a data provider.
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbacker.html#method.new) for more information.
  ///
  /// Throws [Error] on failure.
  factory LocaleFallbacker(DataProvider provider) {
    final result = _ICU4XLocaleFallbacker_create(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return LocaleFallbacker._(result.union.ok);
  }

  /// Creates a new `LocaleFallbacker` without data for limited functionality.
  ///
  /// See the [Rust documentation for `new_without_data`](https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbacker.html#method.new_without_data) for more information.
  factory LocaleFallbacker.withoutData() {
    final result = _ICU4XLocaleFallbacker_create_without_data();
    return LocaleFallbacker._(result);
  }

  /// Associates this `LocaleFallbacker` with configuration options.
  ///
  /// See the [Rust documentation for `for_config`](https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbacker.html#method.for_config) for more information.
  ///
  /// Throws [Error] on failure.
  LocaleFallbackerWithConfig forConfig(LocaleFallbackConfig config) {
    final result = _ICU4XLocaleFallbacker_for_config(_underlying, config._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return LocaleFallbackerWithConfig._(result.union.ok);
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XLocaleFallbacker_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XLocaleFallbacker_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XLocaleFallbacker_create')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XLocaleFallbacker_create(ffi.Pointer<ffi.Opaque> provider);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true, symbol: 'ICU4XLocaleFallbacker_create_without_data')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _ICU4XLocaleFallbacker_create_without_data();

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, _LocaleFallbackConfigFfi)>(isLeaf: true, symbol: 'ICU4XLocaleFallbacker_for_config')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XLocaleFallbacker_for_config(ffi.Pointer<ffi.Opaque> self, _LocaleFallbackConfigFfi config);
