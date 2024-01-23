// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// See the [Rust documentation for `WordBreakIterator`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html) for more information.
final class WordBreakIteratorLatin1 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  WordBreakIteratorLatin1._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XWordBreakIteratorLatin1_destroy));

  /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
  /// out of range of a 32-bit signed integer.
  ///
  /// See the [Rust documentation for `next`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html#method.next) for more information.
  int next() {
    final result = _ICU4XWordBreakIteratorLatin1_next(_underlying);
    return result;
  }

  /// Return the status value of break boundary.
  ///
  /// See the [Rust documentation for `word_type`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html#method.word_type) for more information.
  SegmenterWordType get wordType {
    final result = _ICU4XWordBreakIteratorLatin1_word_type(_underlying);
    return SegmenterWordType.values[result];
  }

  /// Return true when break boundary is word-like such as letter/number/CJK
  ///
  /// See the [Rust documentation for `is_word_like`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html#method.is_word_like) for more information.
  bool get isWordLike {
    final result = _ICU4XWordBreakIteratorLatin1_is_word_like(_underlying);
    return result;
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XWordBreakIteratorLatin1_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XWordBreakIteratorLatin1_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XWordBreakIteratorLatin1_next')
// ignore: non_constant_identifier_names
external int _ICU4XWordBreakIteratorLatin1_next(ffi.Pointer<ffi.Opaque> self);

@ffi.Native<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XWordBreakIteratorLatin1_word_type')
// ignore: non_constant_identifier_names
external int _ICU4XWordBreakIteratorLatin1_word_type(ffi.Pointer<ffi.Opaque> self);

@ffi.Native<ffi.Bool Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XWordBreakIteratorLatin1_is_word_like')
// ignore: non_constant_identifier_names
external bool _ICU4XWordBreakIteratorLatin1_is_word_like(ffi.Pointer<ffi.Opaque> self);
