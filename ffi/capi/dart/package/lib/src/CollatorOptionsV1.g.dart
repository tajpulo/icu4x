// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// See the [Rust documentation for `CollatorOptions`](https://docs.rs/icu/latest/icu/collator/struct.CollatorOptions.html) for more information.
final class _CollatorOptionsV1Ffi extends ffi.Struct {
  @ffi.Int32()
  external int strength;
  @ffi.Int32()
  external int alternateHandling;
  @ffi.Int32()
  external int caseFirst;
  @ffi.Int32()
  external int maxVariable;
  @ffi.Int32()
  external int caseLevel;
  @ffi.Int32()
  external int numeric;
  @ffi.Int32()
  external int backwardSecondLevel;
}

final class CollatorOptionsV1 {
  final _CollatorOptionsV1Ffi _underlying;

  CollatorOptionsV1._(this._underlying);

  factory CollatorOptionsV1() {
    final pointer = ffi2.calloc<_CollatorOptionsV1Ffi>();
    final result = CollatorOptionsV1._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  CollatorStrength get strength => CollatorStrength.values[_underlying.strength];
  set strength(CollatorStrength strength) {
    _underlying.strength = strength.index;
  }

  CollatorAlternateHandling get alternateHandling => CollatorAlternateHandling.values[_underlying.alternateHandling];
  set alternateHandling(CollatorAlternateHandling alternateHandling) {
    _underlying.alternateHandling = alternateHandling.index;
  }

  CollatorCaseFirst get caseFirst => CollatorCaseFirst.values[_underlying.caseFirst];
  set caseFirst(CollatorCaseFirst caseFirst) {
    _underlying.caseFirst = caseFirst.index;
  }

  CollatorMaxVariable get maxVariable => CollatorMaxVariable.values[_underlying.maxVariable];
  set maxVariable(CollatorMaxVariable maxVariable) {
    _underlying.maxVariable = maxVariable.index;
  }

  CollatorCaseLevel get caseLevel => CollatorCaseLevel.values[_underlying.caseLevel];
  set caseLevel(CollatorCaseLevel caseLevel) {
    _underlying.caseLevel = caseLevel.index;
  }

  CollatorNumeric get numeric => CollatorNumeric.values[_underlying.numeric];
  set numeric(CollatorNumeric numeric) {
    _underlying.numeric = numeric.index;
  }

  CollatorBackwardSecondLevel get backwardSecondLevel => CollatorBackwardSecondLevel.values[_underlying.backwardSecondLevel];
  set backwardSecondLevel(CollatorBackwardSecondLevel backwardSecondLevel) {
    _underlying.backwardSecondLevel = backwardSecondLevel.index;
  }

  @override
  bool operator ==(Object other) =>
      other is CollatorOptionsV1 &&
      other._underlying.strength == _underlying.strength &&
      other._underlying.alternateHandling == _underlying.alternateHandling &&
      other._underlying.caseFirst == _underlying.caseFirst &&
      other._underlying.maxVariable == _underlying.maxVariable &&
      other._underlying.caseLevel == _underlying.caseLevel &&
      other._underlying.numeric == _underlying.numeric &&
      other._underlying.backwardSecondLevel == _underlying.backwardSecondLevel;

  @override
  int get hashCode => Object.hashAll([
        _underlying.strength,
        _underlying.alternateHandling,
        _underlying.caseFirst,
        _underlying.maxVariable,
        _underlying.caseLevel,
        _underlying.numeric,
        _underlying.backwardSecondLevel,
      ]);
}
