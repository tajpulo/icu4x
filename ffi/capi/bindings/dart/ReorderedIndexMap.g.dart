// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// Thin wrapper around a vector that maps visual indices to source indices
///
/// `map[visualIndex] = sourceIndex`
///
/// Produced by `reorder_visual()` on [`Bidi`].
final class ReorderedIndexMap implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ReorderedIndexMap._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XReorderedIndexMap_destroy));

  /// Get this as a slice/array of indices
  core.List<int> get asSlice {
    final result = _ICU4XReorderedIndexMap_as_slice(_underlying);
    return core.Iterable.generate(result._length).map((i) => result._pointer[i]).toList(growable: false);
  }

  /// The length of this map
  int get length {
    final result = _ICU4XReorderedIndexMap_len(_underlying);
    return result;
  }

  /// Whether this map is empty
  bool get isEmpty {
    final result = _ICU4XReorderedIndexMap_is_empty(_underlying);
    return result;
  }

  /// Get element at `index`. Returns 0 when out of bounds
  /// (note that 0 is also a valid in-bounds value, please use `len()`
  /// to avoid out-of-bounds)
  int operator [](int index) {
    final result = _ICU4XReorderedIndexMap_get(_underlying, index);
    return result;
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XReorderedIndexMap_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XReorderedIndexMap_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<_SliceUsize Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XReorderedIndexMap_as_slice')
// ignore: non_constant_identifier_names
external _SliceUsize _ICU4XReorderedIndexMap_as_slice(ffi.Pointer<ffi.Opaque> self);

@ffi.Native<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XReorderedIndexMap_len')
// ignore: non_constant_identifier_names
external int _ICU4XReorderedIndexMap_len(ffi.Pointer<ffi.Opaque> self);

@ffi.Native<ffi.Bool Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XReorderedIndexMap_is_empty')
// ignore: non_constant_identifier_names
external bool _ICU4XReorderedIndexMap_is_empty(ffi.Pointer<ffi.Opaque> self);

@ffi.Native<ffi.Size Function(ffi.Pointer<ffi.Opaque>, ffi.Size)>(isLeaf: true, symbol: 'ICU4XReorderedIndexMap_get')
// ignore: non_constant_identifier_names
external int _ICU4XReorderedIndexMap_get(ffi.Pointer<ffi.Opaque> self, int index);
