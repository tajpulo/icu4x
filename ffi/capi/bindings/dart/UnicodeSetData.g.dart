// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
///
/// See the [Rust documentation for `properties`](https://docs.rs/icu/latest/icu/properties/index.html) for more information.
///
/// See the [Rust documentation for `UnicodeSetData`](https://docs.rs/icu/latest/icu/properties/sets/struct.UnicodeSetData.html) for more information.
///
/// See the [Rust documentation for `UnicodeSetDataBorrowed`](https://docs.rs/icu/latest/icu/properties/sets/struct.UnicodeSetDataBorrowed.html) for more information.
final class UnicodeSetData implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  UnicodeSetData._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XUnicodeSetData_destroy));

  /// Checks whether the string is in the set.
  ///
  /// See the [Rust documentation for `contains`](https://docs.rs/icu/latest/icu/properties/sets/struct.UnicodeSetDataBorrowed.html#method.contains) for more information.
  bool contains(String s) {
    final temp = ffi2.Arena();
    final sView = s.utf8View;
    final result = _ICU4XUnicodeSetData_contains(_underlying, sView.pointer(temp), sView.length);
    temp.releaseAll();
    return result;
  }

  /// Checks whether the code point is in the set.
  ///
  /// See the [Rust documentation for `contains_char`](https://docs.rs/icu/latest/icu/properties/sets/struct.UnicodeSetDataBorrowed.html#method.contains_char) for more information.
  bool containsChar(Rune cp) {
    final result = _ICU4XUnicodeSetData_contains_char(_underlying, cp);
    return result;
  }

  /// See the [Rust documentation for `basic_emoji`](https://docs.rs/icu/latest/icu/properties/sets/fn.basic_emoji.html) for more information.
  ///
  /// Throws [Error] on failure.
  factory UnicodeSetData.loadBasicEmoji(DataProvider provider) {
    final result = _ICU4XUnicodeSetData_load_basic_emoji(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return UnicodeSetData._(result.union.ok);
  }

  /// See the [Rust documentation for `exemplars_main`](https://docs.rs/icu/latest/icu/properties/exemplar_chars/fn.exemplars_main.html) for more information.
  ///
  /// Throws [Error] on failure.
  factory UnicodeSetData.loadExemplarsMain(DataProvider provider, Locale locale) {
    final result = _ICU4XUnicodeSetData_load_exemplars_main(provider._underlying, locale._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return UnicodeSetData._(result.union.ok);
  }

  /// See the [Rust documentation for `exemplars_auxiliary`](https://docs.rs/icu/latest/icu/properties/exemplar_chars/fn.exemplars_auxiliary.html) for more information.
  ///
  /// Throws [Error] on failure.
  factory UnicodeSetData.loadExemplarsAuxiliary(DataProvider provider, Locale locale) {
    final result = _ICU4XUnicodeSetData_load_exemplars_auxiliary(provider._underlying, locale._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return UnicodeSetData._(result.union.ok);
  }

  /// See the [Rust documentation for `exemplars_punctuation`](https://docs.rs/icu/latest/icu/properties/exemplar_chars/fn.exemplars_punctuation.html) for more information.
  ///
  /// Throws [Error] on failure.
  factory UnicodeSetData.loadExemplarsPunctuation(DataProvider provider, Locale locale) {
    final result = _ICU4XUnicodeSetData_load_exemplars_punctuation(provider._underlying, locale._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return UnicodeSetData._(result.union.ok);
  }

  /// See the [Rust documentation for `exemplars_numbers`](https://docs.rs/icu/latest/icu/properties/exemplar_chars/fn.exemplars_numbers.html) for more information.
  ///
  /// Throws [Error] on failure.
  factory UnicodeSetData.loadExemplarsNumbers(DataProvider provider, Locale locale) {
    final result = _ICU4XUnicodeSetData_load_exemplars_numbers(provider._underlying, locale._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return UnicodeSetData._(result.union.ok);
  }

  /// See the [Rust documentation for `exemplars_index`](https://docs.rs/icu/latest/icu/properties/exemplar_chars/fn.exemplars_index.html) for more information.
  ///
  /// Throws [Error] on failure.
  factory UnicodeSetData.loadExemplarsIndex(DataProvider provider, Locale locale) {
    final result = _ICU4XUnicodeSetData_load_exemplars_index(provider._underlying, locale._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return UnicodeSetData._(result.union.ok);
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XUnicodeSetData_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XUnicodeSetData_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<ffi.Bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size)>(isLeaf: true, symbol: 'ICU4XUnicodeSetData_contains')
// ignore: non_constant_identifier_names
external bool _ICU4XUnicodeSetData_contains(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Uint8> sData, int sLength);

@ffi.Native<ffi.Bool Function(ffi.Pointer<ffi.Opaque>, ffi.Uint32)>(isLeaf: true, symbol: 'ICU4XUnicodeSetData_contains_char')
// ignore: non_constant_identifier_names
external bool _ICU4XUnicodeSetData_contains_char(ffi.Pointer<ffi.Opaque> self, Rune cp);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XUnicodeSetData_load_basic_emoji')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XUnicodeSetData_load_basic_emoji(ffi.Pointer<ffi.Opaque> provider);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XUnicodeSetData_load_exemplars_main')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XUnicodeSetData_load_exemplars_main(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XUnicodeSetData_load_exemplars_auxiliary')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XUnicodeSetData_load_exemplars_auxiliary(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XUnicodeSetData_load_exemplars_punctuation')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XUnicodeSetData_load_exemplars_punctuation(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XUnicodeSetData_load_exemplars_numbers')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XUnicodeSetData_load_exemplars_numbers(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XUnicodeSetData_load_exemplars_index')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XUnicodeSetData_load_exemplars_index(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale);
