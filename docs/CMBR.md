# CMBR Docs
### datawizard
## Specification Version 0.0.1

**DISCLAIMER: The documentation is still very much so in the making, it is nothing more than a WIP.**

## 1. Definitions

* u24 - 24 bit unsigned integer
* u8 - 8 bit unsigned integer
* NAG - Numerical annotation glyph. See [Wikipedia](https://en.wikipedia.org/wiki/Numeric_Annotation_Glyphs) for more information. 
* Variation table - A table that denotes

## 2. Move notation

The move is represented with an u24.

### 2.1 First part
<img width="460" alt="Screenshot 2024-07-09 at 2 25 13 PM" src="https://github.com/Whitegabriella789/CMBR/assets/172323441/a3074f80-cd5a-4fea-a09b-d3b2ea82e7e9">

To and from squares are defined as an index of a chessboard square. The values are between 0-63. Where 0 is ’a1’ and 63 is ’h8’.

Pieces value represent some piece. See the table for more info

The flags are defined as individual values. They’re bit-wise ored (|) together to get the final value.


## CMBR Flag Enumeration

| Flag Name | Binary Value | Note |
--- | --- | ---
| FlagNone | 0b00000000 | Empty flag |
| FlagCheck | 0b00000001 | Move is a check |
| FlagMate | 0b00000010 | Move is a checkmate |
| FlagCapture | 0b00000100 | Move is a capture |
| FlagNag | 0b00001000 | If this flag is set, the first 8 bits are replaced with a NAG index |
| FlagPromotesBishop | 0b01000000 | Move promotes to bishop |
| FlagPromotesKnight | 0b01010000 | Move promotes to knight |
| FlagPromotesRook | 0b01100000 | Move promotes to rook |
| FlagPromotesQueen | 0b01110000 | Move promotes to queen |
| FlagIsVariationPointer | 0b10000000 | If this flag is set, the first 16 bits are replaced with an index to the variations table |

## Pieces to Binary Value Table
| Piece | Binary Value |
--- | ---
| White pawn | 0b0000 |
| White knight | 0b0001 |
| White bishop | 0b0010 | 
| White rook | 0b0011 | 
| White queen | 0b0100 |
| White king | 0b0101 |
| White castles short | 0b0110 |
| White castles long | 0b0111 |
| Black pawn | 0b1000 |
| Black knight | 0b1001 |
| Black bishop | 0b1010 |
| Black rook | 0b1011 |
| Black queen | 0b1100 |
| Black king | 0b1101 |
| Black castles short | 0b1110 |
| Black castles long | 0b1111 |
