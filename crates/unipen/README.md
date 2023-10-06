# `unipen`

`unipen` is a library for the parsing, validation, and querying of the UniPen format for on-line handwriting data.

It also includes a binary crate, which parses, validates, and outputs debug information on the data contained within a UniPen file.

_Usage:_

```
unipen <file> [include-dir]
```

It can provide syntax errors on UniPen files, courtesy of the `pest` crate:

```
Pest rule error:    --> unipen-ICROW-2003/NIC-Pc95-heleen.dat:126:15
    |
126 | .SKILL        ???
    |               ^---
    |
    = expected r_bad, r_ok, or r_good
Error: "Error occured during parsing."
```

## Comparison with UpLib and UpTools

| Feature                     | UpLib | `unipen-rs` |
| --------------------------- | ----- | ----------- |
| Syntax checking             | No    | Yes         |
| Support for UniPen metadata | No    | Yes         |
| Recursive `.INCLUDE`        | Yes   | Yes         |
| Multiple include paths      | Yes   | No          |
| No `unipen.def` required    | No    | Yes         |

UpLib does not attempt to parse the entirety of the UniPen data, and is primarily focused on extracting various features about sequences of UniPen components and segments. `unipen-rs` intends to read and validate _all_ non-commented information in a UniPen file.

UpLib can try multiple paths when processing an `.INCLUDE` statement. `unipen-rs` only supports one base path.

UpLib requires the parsing of a `unipen.def` document type definition in order to bootstrap the parser. `unipen-rs` forgoes this by defining a PEG grammar for the specification.

## Datasets

Because `unipen-rs` has stricter syntax requirements compared to UpLib, UniPen datasets must be patched in order to parse without syntax errors. The following is a list of pre-patched datasets:

- [`train_r01_v07`](https://github.com/GreenCappuccino/train_r01_v07)
- [`Unipen-ICROW-03`](https://github.com/GreenCappuccino/Unipen-ICROW-03)

## References

I. Guyon, L. Schomaker, R. Plamondon, M. Liberman and S. Janet, "UNIPEN project of on-line data exchange and recognizer benchmarks," Proceedings of the 12th IAPR International Conference on Pattern Recognition, Vol. 3 - Conference C: Signal Processing (Cat. No.94CH3440-5), Jerusalem, Israel, 1994, pp. 29-33 vol.2, [doi: 10.1109/ICPR.1994.576870](https://doi.org/10.1109/ICPR.1994.576870).
