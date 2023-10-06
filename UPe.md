# UniPen extended (UPe)

UniPen extended (UPe) is derived from the UniPen 1.0 Format. It intends to support Unicode, and contains additional keywords. The primary purpose of this specification is to facilitate modifying existing UniPen datasets.

> Note that certain keywords may have different behavior compared to the original specification.

The version number of this format is `2.0`.

# Defintions

- `CHARACTER`: One character of the following [Unicode categories](http://www.unicode.org/reports/tr44/#General_Category_Values): Letter (`L`), Mark (`M`), Number (`N`), Punctuation (`P`), Symbol (`S`).
- `SEPARATOR`: 

# Types

| Type              | Definition                                                                   |
| ----------------- | ---------------------------------------------------------------------------- |
| `[N]` (Number)    | ASCII digits 0-9 followed by an optional decimal point and additional digits |
| `[S]` (String)    |                                                                              |
| `[F]` (Free Text) |                                                                              |
| `[L]` (Label)     |                                                                              |

# K
