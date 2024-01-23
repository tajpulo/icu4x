// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// An ICU4X word-break segmenter, capable of finding word breakpoints in strings.
///
/// See the [Rust documentation for `WordSegmenter`](https://docs.rs/icu/latest/icu/segmenter/struct.WordSegmenter.html) for more information.
final class WordSegmenter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  WordSegmenter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XWordSegmenter_destroy));

  /// Construct an [`WordSegmenter`] with automatically selecting the best available LSTM
  /// or dictionary payload data.
  ///
  /// Note: currently, it uses dictionary for Chinese and Japanese, and LSTM for Burmese,
  /// Khmer, Lao, and Thai.
  ///
  /// See the [Rust documentation for `new_auto`](https://docs.rs/icu/latest/icu/segmenter/struct.WordSegmenter.html#method.new_auto) for more information.
  ///
  /// Throws [Error] on failure.
  factory WordSegmenter.auto(DataProvider provider) {
    final result = _ICU4XWordSegmenter_create_auto(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return WordSegmenter._(result.union.ok);
  }

  /// Construct an [`WordSegmenter`] with LSTM payload data for Burmese, Khmer, Lao, and
  /// Thai.
  ///
  /// Warning: [`WordSegmenter`] created by this function doesn't handle Chinese or
  /// Japanese.
  ///
  /// See the [Rust documentation for `new_lstm`](https://docs.rs/icu/latest/icu/segmenter/struct.WordSegmenter.html#method.new_lstm) for more information.
  ///
  /// Throws [Error] on failure.
  factory WordSegmenter.lstm(DataProvider provider) {
    final result = _ICU4XWordSegmenter_create_lstm(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return WordSegmenter._(result.union.ok);
  }

  /// Construct an [`WordSegmenter`] with dictionary payload data for Chinese, Japanese,
  /// Burmese, Khmer, Lao, and Thai.
  ///
  /// See the [Rust documentation for `new_dictionary`](https://docs.rs/icu/latest/icu/segmenter/struct.WordSegmenter.html#method.new_dictionary) for more information.
  ///
  /// Throws [Error] on failure.
  factory WordSegmenter.dictionary(DataProvider provider) {
    final result = _ICU4XWordSegmenter_create_dictionary(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return WordSegmenter._(result.union.ok);
  }

  /// Segments a string.
  ///
  /// Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
  /// to the WHATWG Encoding Standard.
  ///
  /// See the [Rust documentation for `segment_utf16`](https://docs.rs/icu/latest/icu/segmenter/struct.WordSegmenter.html#method.segment_utf16) for more information.
  WordBreakIteratorUtf16 segment(String input) {
    final temp = ffi2.Arena();
    final inputView = input.utf16View;
    final result = _ICU4XWordSegmenter_segment_utf16(_underlying, inputView.pointer(temp), inputView.length);
    temp.releaseAll();
    return WordBreakIteratorUtf16._(result);
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XWordSegmenter_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XWordSegmenter_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XWordSegmenter_create_auto')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XWordSegmenter_create_auto(ffi.Pointer<ffi.Opaque> provider);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XWordSegmenter_create_lstm')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XWordSegmenter_create_lstm(ffi.Pointer<ffi.Opaque> provider);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XWordSegmenter_create_dictionary')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XWordSegmenter_create_dictionary(ffi.Pointer<ffi.Opaque> provider);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint16>, ffi.Size)>(isLeaf: true, symbol: 'ICU4XWordSegmenter_segment_utf16')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _ICU4XWordSegmenter_segment_utf16(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Uint16> inputData, int inputLength);
