// generated by diplomat-tool

part of 'lib.g.dart';

final class _WeekendContainsDayFfi extends ffi.Struct {
  @ffi.Bool()
  external bool monday;
  @ffi.Bool()
  external bool tuesday;
  @ffi.Bool()
  external bool wednesday;
  @ffi.Bool()
  external bool thursday;
  @ffi.Bool()
  external bool friday;
  @ffi.Bool()
  external bool saturday;
  @ffi.Bool()
  external bool sunday;
}

/// Documents which days of the week are considered to be a part of the weekend
///
/// See the [Rust documentation for `weekend`](https://docs.rs/icu/latest/icu/calendar/week/struct.WeekCalculator.html#method.weekend) for more information.
final class WeekendContainsDay {
  bool monday;
  bool tuesday;
  bool wednesday;
  bool thursday;
  bool friday;
  bool saturday;
  bool sunday;

  WeekendContainsDay({required this.monday, required this.tuesday, required this.wednesday, required this.thursday, required this.friday, required this.saturday, required this.sunday});

  // This struct contains borrowed fields, so this takes in a list of
  // "edges" corresponding to where each lifetime's data may have been borrowed from
  // and passes it down to individual fields containing the borrow.
  // This method does not attempt to handle any dependencies between lifetimes, the caller
  // should handle this when constructing edge arrays.
  // ignore: unused_element
  WeekendContainsDay._fromFfi(_WeekendContainsDayFfi ffi) :
    monday = ffi.monday,
    tuesday = ffi.tuesday,
    wednesday = ffi.wednesday,
    thursday = ffi.thursday,
    friday = ffi.friday,
    saturday = ffi.saturday,
    sunday = ffi.sunday;

  // ignore: unused_element
  _WeekendContainsDayFfi _toFfi(ffi.Allocator temp) {
    final struct = ffi.Struct.create<_WeekendContainsDayFfi>();
    struct.monday = monday;
    struct.tuesday = tuesday;
    struct.wednesday = wednesday;
    struct.thursday = thursday;
    struct.friday = friday;
    struct.saturday = saturday;
    struct.sunday = sunday;
    return struct;
  }

  @override
  bool operator ==(Object other) =>
      other is WeekendContainsDay &&
      other.monday == monday &&
      other.tuesday == tuesday &&
      other.wednesday == wednesday &&
      other.thursday == thursday &&
      other.friday == friday &&
      other.saturday == saturday &&
      other.sunday == sunday;

  @override
  int get hashCode => Object.hashAll([
        monday,
        tuesday,
        wednesday,
        thursday,
        friday,
        saturday,
        sunday,
      ]);
}