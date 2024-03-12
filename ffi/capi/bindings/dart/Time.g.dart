// generated by diplomat-tool

part of 'lib.g.dart';

/// An ICU4X Time object representing a time in terms of hour, minute, second, nanosecond
///
/// See the [Rust documentation for `Time`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html) for more information.
final class Time implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  Time._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XTime_destroy));

  /// Creates a new [`Time`] given field values
  ///
  /// See the [Rust documentation for `Time`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html) for more information.
  ///
  /// Throws [Error] on failure.
  factory Time(int hour, int minute, int second, int nanosecond) {
    final result = _ICU4XTime_create(hour, minute, second, nanosecond);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return Time._fromFfi(result.union.ok, []);
  }

  /// Creates a new [`Time`] representing midnight (00:00.000).
  ///
  /// See the [Rust documentation for `Time`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html) for more information.
  ///
  /// Throws [Error] on failure.
  factory Time.midnight() {
    final result = _ICU4XTime_create_midnight();
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return Time._fromFfi(result.union.ok, []);
  }

  /// Returns the hour in this time
  ///
  /// See the [Rust documentation for `hour`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html#structfield.hour) for more information.
  int get hour {
    final result = _ICU4XTime_hour(_ffi);
    return result;
  }

  /// Returns the minute in this time
  ///
  /// See the [Rust documentation for `minute`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html#structfield.minute) for more information.
  int get minute {
    final result = _ICU4XTime_minute(_ffi);
    return result;
  }

  /// Returns the second in this time
  ///
  /// See the [Rust documentation for `second`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html#structfield.second) for more information.
  int get second {
    final result = _ICU4XTime_second(_ffi);
    return result;
  }

  /// Returns the nanosecond in this time
  ///
  /// See the [Rust documentation for `nanosecond`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html#structfield.nanosecond) for more information.
  int get nanosecond {
    final result = _ICU4XTime_nanosecond(_ffi);
    return result;
  }
}

@meta.ResourceIdentifier('ICU4XTime_destroy')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XTime_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XTime_destroy(ffi.Pointer<ffi.Void> self);

@meta.ResourceIdentifier('ICU4XTime_create')
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Uint8, ffi.Uint8, ffi.Uint8, ffi.Uint32)>(isLeaf: true, symbol: 'ICU4XTime_create')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XTime_create(int hour, int minute, int second, int nanosecond);

@meta.ResourceIdentifier('ICU4XTime_create_midnight')
@ffi.Native<_ResultOpaqueInt32 Function()>(isLeaf: true, symbol: 'ICU4XTime_create_midnight')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XTime_create_midnight();

@meta.ResourceIdentifier('ICU4XTime_hour')
@ffi.Native<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XTime_hour')
// ignore: non_constant_identifier_names
external int _ICU4XTime_hour(ffi.Pointer<ffi.Opaque> self);

@meta.ResourceIdentifier('ICU4XTime_minute')
@ffi.Native<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XTime_minute')
// ignore: non_constant_identifier_names
external int _ICU4XTime_minute(ffi.Pointer<ffi.Opaque> self);

@meta.ResourceIdentifier('ICU4XTime_second')
@ffi.Native<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XTime_second')
// ignore: non_constant_identifier_names
external int _ICU4XTime_second(ffi.Pointer<ffi.Opaque> self);

@meta.ResourceIdentifier('ICU4XTime_nanosecond')
@ffi.Native<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XTime_nanosecond')
// ignore: non_constant_identifier_names
external int _ICU4XTime_nanosecond(ffi.Pointer<ffi.Opaque> self);