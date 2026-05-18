# rtcc

Data structures and traits to be implemented by real-time clock / calendar devices.

Prefer to use only the methods from the `DateTimeAccess` rather than the individual
methods from the `Rtcc` trait to avoid situations where the passing of time
makes the results of the method calls inconsistent if you combine the results
of several methods.

For example, this can happen at certain timepoints:
1. The time is `01:59:59`
2. A call to `hours()` returns 1.
3. The time is increased to `02:00:00`.
4. A call to `minutes()` returns 0.
5. A call to `seconds()` returns 0.
6. Your system thinks it is `01:00:00`.

The same applies to the date as well, as well as when calling setter methods.

# Features

The optional `defmt` feature implements [defmt::Format] on some types.

## Modules

### [`rtcc`](rtcc.md)

*1 enum, 2 traits*

