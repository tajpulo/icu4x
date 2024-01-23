// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// See the [Rust documentation for `AnyCalendar`](https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendar.html) for more information.
final class Calendar implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  Calendar._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XCalendar_destroy));

  /// Creates a new [`Calendar`] from the specified date and time.
  ///
  /// See the [Rust documentation for `new_for_locale`](https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendar.html#method.new_for_locale) for more information.
  ///
  /// Throws [Error] on failure.
  factory Calendar.forLocale(DataProvider provider, Locale locale) {
    final result = _ICU4XCalendar_create_for_locale(provider._underlying, locale._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return Calendar._(result.union.ok);
  }

  /// Creates a new [`Calendar`] from the specified date and time.
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendar.html#method.new) for more information.
  ///
  /// Throws [Error] on failure.
  factory Calendar.forKind(DataProvider provider, AnyCalendarKind kind) {
    final result = _ICU4XCalendar_create_for_kind(provider._underlying, kind.index);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return Calendar._(result.union.ok);
  }

  /// Returns the kind of this calendar
  ///
  /// See the [Rust documentation for `kind`](https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendar.html#method.kind) for more information.
  AnyCalendarKind get kind {
    final result = _ICU4XCalendar_kind(_underlying);
    return AnyCalendarKind.values[result];
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XCalendar_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XCalendar_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCalendar_create_for_locale')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XCalendar_create_for_locale(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Int32)>(isLeaf: true, symbol: 'ICU4XCalendar_create_for_kind')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XCalendar_create_for_kind(ffi.Pointer<ffi.Opaque> provider, int kind);

@ffi.Native<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCalendar_kind')
// ignore: non_constant_identifier_names
external int _ICU4XCalendar_kind(ffi.Pointer<ffi.Opaque> self);
