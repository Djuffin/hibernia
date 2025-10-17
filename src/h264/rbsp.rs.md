# `rbsp.rs` - Raw Byte Sequence Payload (RBSP) Reader

This file provides a `RbspReader` struct for reading H.264 Raw Byte Sequence Payload (RBSP) data from a bitstream. RBSP is the raw data payload of a Network Abstraction Layer (NAL) unit, before the emulation prevention bytes have been removed.

## Key Data Structures

### `RbspReader<'a>`

A struct that wraps the `bitstream_io::BitReader` to provide methods for reading H.264-specific data types, such as Exp-Golomb coded integers.

- `reader`: A `BitReader` instance that reads from a `Cursor` over a byte slice.

## Core Functionality

### `RbspReader::new`

Creates a new `RbspReader` from a byte slice.

### `RbspReader::align`

Aligns the reader to the next byte boundary.

### `RbspReader::is_aligned`

Checks if the reader is currently byte-aligned.

### `RbspReader::f`

Reads a single bit (flag) from the bitstream.

### `RbspReader::u`

Reads an unsigned integer of a specified number of bits.

### `RbspReader::ue`

Reads an unsigned Exp-Golomb coded integer. Exp-Golomb coding is a variable-length coding scheme used extensively in H.264 to represent non-negative integers.

### `RbspReader::se`

Reads a signed Exp-Golomb coded integer.

### `RbspReader::peek_or_pad16`

Peeks at the next 16 bits in the bitstream without advancing the reader's position. If there are fewer than 16 bits remaining, it pads the result with zeros.

### `RbspReader::position`

Returns the current position of the reader in bits.

### `RbspReader::skip`

Skips a specified number of bits in the bitstream.

### `RbspReader::read_till_one`

Reads bits from the stream until a '1' is encountered, and returns the number of leading zeros. This is a helper for `ue` and `se` parsing.

### `RbspReader::remaining`

Returns the number of remaining bits in the bitstream.

## Testing

The file includes a `tests` module with unit tests for the `ue`, `se`, and `peek_or_pad16` methods. These tests cover various input values to ensure the correct implementation of the Exp-Golomb decoding and peeking logic.
