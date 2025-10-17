# `macroblock.rs` - H.264 Macroblock Definitions

This file defines the data structures and enums related to H.264 macroblocks, which are the basic 16x16 pixel processing units in the H.264 standard.

## Key Data Structures and Enums

### `MbAddr`

A type alias for `u32` representing a macroblock address.

### `MbNeighborName`

An enum representing the four neighboring macroblocks of a given macroblock: `A` (Left), `B` (Above), `C` (Above-right), and `D` (Above-left).

### `IMbType` and `PMbType`

Enums representing the different types of macroblocks for I-slices and P-slices, respectively, as defined in the H.264 specification.

### `MbPredictionMode`

An enum that specifies the prediction mode for a macroblock, which can be `Intra_4x4`, `Intra_8x8`, `Intra_16x16`, `Pred_L0`, or `Pred_L1`.

### `Intra_4x4_SamplePredMode`, `Intra_16x16_SamplePredMode`, `Intra_Chroma_Pred_Mode`

Enums that define the specific intra-prediction modes for 4x4 luma blocks, 16x16 luma blocks, and chroma blocks.

### `CodedBlockPattern`

A struct that represents the coded block pattern (CBP) for a macroblock. The CBP indicates which sub-blocks within the macroblock contain non-zero transform coefficients.

### `PcmMb`

A struct representing a PCM (Pulse Code Modulation) macroblock, which contains raw, uncompressed pixel values.

### `IMb`

A struct representing an I-macroblock, which is intra-coded. It contains information about the macroblock type, prediction modes, CBP, quantization parameters, and residual data.

### `P`

A struct representing a P-macroblock, which is inter-coded.

### `Macroblock`

An enum that can be one of `I(IMb)`, `PCM(PcmMb)`, or `P(P)`, representing the different types of macroblocks.

## Core Functionality

### `get_neighbor_mbs`

A function that, given a macroblock address and a neighbor name, returns the address of the neighboring macroblock if it exists.

### `get_4x4luma_block_location` and `get_4x4chroma_block_location`

Functions that return the top-left coordinates of a 4x4 luma or chroma block within a macroblock, given the block's index.

### `get_4x4luma_block_index` and `get_4x4chroma_block_index`

Functions that return the index of a 4x4 luma or chroma block, given its coordinates within a macroblock.

### `get_4x4luma_block_neighbor` and `get_4x4chroma_block_neighbor`

Functions that, given a 4x4 block index and a neighbor name, return the index of the neighboring block and whether that neighbor is in a different macroblock.

### `IMb::MbPartPredMode` and `Macroblock::MbPartPredMode`

Methods that return the prediction mode for a given partition of a macroblock.

### `Macroblock::get_nc`

A method that calculates the number of non-zero transform coefficients (`nC`) for a given block within a macroblock.

### `Macroblock::get_coded_block_pattern`

A method that returns the `CodedBlockPattern` for a macroblock.

### `Macroblock::set_residual`

A method that sets the residual data for a macroblock.

## Testing

The file includes a `tests` module with unit tests for the neighbor-finding and block-scanning functions, ensuring their correctness according to the H.264 specification.
