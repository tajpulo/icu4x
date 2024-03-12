// generated by diplomat-tool

part of 'lib.g.dart';

/// An ICU4X Bidi object, containing loaded bidi data
///
/// See the [Rust documentation for `BidiClassAdapter`](https://docs.rs/icu/latest/icu/properties/bidi/struct.BidiClassAdapter.html) for more information.
final class Bidi implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  Bidi._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XBidi_destroy));

  /// Creates a new [`Bidi`] from locale data.
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/properties/bidi/struct.BidiClassAdapter.html#method.new) for more information.
  ///
  /// Throws [Error] on failure.
  factory Bidi(DataProvider provider) {
    final result = _ICU4XBidi_create(provider._ffi);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return Bidi._fromFfi(result.union.ok, []);
  }

  /// Use the data loaded in this object to process a string and calculate bidi information
  ///
  /// Takes in a Level for the default level, if it is an invalid value it will default to LTR
  ///
  /// See the [Rust documentation for `new_with_data_source`](https://docs.rs/unicode_bidi/latest/unicode_bidi/struct.BidiInfo.html#method.new_with_data_source) for more information.
  BidiInfo forText(String text, int defaultLevel) {
    final textView = text.utf8View;
    final textArena = _FinalizedArena();
    // This lifetime edge depends on lifetimes: 'text
    core.List<Object> textEdges = [textArena];
    final result = _ICU4XBidi_for_text(_ffi, textView.allocIn(textArena.arena), textView.length, defaultLevel);
    return BidiInfo._fromFfi(result, [], textEdges);
  }

  /// Utility function for producing reorderings given a list of levels
  ///
  /// Produces a map saying which visual index maps to which source index.
  ///
  /// The levels array must not have values greater than 126 (this is the
  /// Bidi maximum explicit depth plus one).
  /// Failure to follow this invariant may lead to incorrect results,
  /// but is still safe.
  ///
  /// See the [Rust documentation for `reorder_visual`](https://docs.rs/unicode_bidi/latest/unicode_bidi/struct.BidiInfo.html#method.reorder_visual) for more information.
  ReorderedIndexMap reorderVisual(core.List<int> levels) {
    final temp = ffi2.Arena();
    final levelsView = levels.uint8View;
    final result = _ICU4XBidi_reorder_visual(_ffi, levelsView.allocIn(temp), levelsView.length);
    temp.releaseAll();
    return ReorderedIndexMap._fromFfi(result, []);
  }

  /// Check if a Level returned by level_at is an RTL level.
  ///
  /// Invalid levels (numbers greater than 125) will be assumed LTR
  ///
  /// See the [Rust documentation for `is_rtl`](https://docs.rs/unicode_bidi/latest/unicode_bidi/struct.Level.html#method.is_rtl) for more information.
  static bool levelIsRtl(int level) {
    final result = _ICU4XBidi_level_is_rtl(level);
    return result;
  }

  /// Check if a Level returned by level_at is an LTR level.
  ///
  /// Invalid levels (numbers greater than 125) will be assumed LTR
  ///
  /// See the [Rust documentation for `is_ltr`](https://docs.rs/unicode_bidi/latest/unicode_bidi/struct.Level.html#method.is_ltr) for more information.
  static bool levelIsLtr(int level) {
    final result = _ICU4XBidi_level_is_ltr(level);
    return result;
  }

  /// Get a basic RTL Level value
  ///
  /// See the [Rust documentation for `rtl`](https://docs.rs/unicode_bidi/latest/unicode_bidi/struct.Level.html#method.rtl) for more information.
  static final int levelRtl = () {
    final result = _ICU4XBidi_level_rtl();
    return result;
  }();

  /// Get a simple LTR Level value
  ///
  /// See the [Rust documentation for `ltr`](https://docs.rs/unicode_bidi/latest/unicode_bidi/struct.Level.html#method.ltr) for more information.
  static final int levelLtr = () {
    final result = _ICU4XBidi_level_ltr();
    return result;
  }();
}

@meta.ResourceIdentifier('ICU4XBidi_destroy')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XBidi_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XBidi_destroy(ffi.Pointer<ffi.Void> self);

@meta.ResourceIdentifier('ICU4XBidi_create')
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XBidi_create')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XBidi_create(ffi.Pointer<ffi.Opaque> provider);

@meta.ResourceIdentifier('ICU4XBidi_for_text')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size, ffi.Uint8)>(isLeaf: true, symbol: 'ICU4XBidi_for_text')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _ICU4XBidi_for_text(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Uint8> textData, int textLength, int defaultLevel);

@meta.ResourceIdentifier('ICU4XBidi_reorder_visual')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size)>(isLeaf: true, symbol: 'ICU4XBidi_reorder_visual')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _ICU4XBidi_reorder_visual(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Uint8> levelsData, int levelsLength);

@meta.ResourceIdentifier('ICU4XBidi_level_is_rtl')
@ffi.Native<ffi.Bool Function(ffi.Uint8)>(isLeaf: true, symbol: 'ICU4XBidi_level_is_rtl')
// ignore: non_constant_identifier_names
external bool _ICU4XBidi_level_is_rtl(int level);

@meta.ResourceIdentifier('ICU4XBidi_level_is_ltr')
@ffi.Native<ffi.Bool Function(ffi.Uint8)>(isLeaf: true, symbol: 'ICU4XBidi_level_is_ltr')
// ignore: non_constant_identifier_names
external bool _ICU4XBidi_level_is_ltr(int level);

@meta.ResourceIdentifier('ICU4XBidi_level_rtl')
@ffi.Native<ffi.Uint8 Function()>(isLeaf: true, symbol: 'ICU4XBidi_level_rtl')
// ignore: non_constant_identifier_names
external int _ICU4XBidi_level_rtl();

@meta.ResourceIdentifier('ICU4XBidi_level_ltr')
@ffi.Native<ffi.Uint8 Function()>(isLeaf: true, symbol: 'ICU4XBidi_level_ltr')
// ignore: non_constant_identifier_names
external int _ICU4XBidi_level_ltr();