# `residual.rs` - H.264 Residual Data Handling

This file is responsible for handling the residual data in an H.264 bitstream. The residual is the difference between the predicted and actual pixel values, which is then transformed, quantized, and coded. This file contains the data structures and functions for storing, scaling, and transforming this residual data.

## Key Data Structures

### `Block4x4` and `Block2x2`

Structs representing a 4x4 and 2x2 block of transform coefficients, respectively.

### `Residual`

A struct that stores the residual data for a macroblock. This includes:

- `prediction_mode`: The prediction mode of the macroblock.
- `coded_block_pattern`: The coded block pattern (CBP).
- `qp`: The quantization parameter.
- DC and AC coefficients for luma and chroma blocks.

## Core Functionality

### `Residual::get_dc_levels_for` and `Residual::get_ac_levels_for`

Methods that provide mutable slices to the DC and AC coefficient arrays for a given color plane and block index.

### `Residual::get_nc`

Calculates the number of non-zero coefficients (`nC`) for a given block.

### `Residual::has_separate_luma_dc`

Checks if the luma DC coefficients are handled separately, which is the case for Intra_16x16 prediction mode.

### `Residual::restore`

The main function for restoring the residual data. It takes a color plane and quantization parameter (`qp`) and returns a `Vec<Block4x4>` of the reconstructed residual blocks. This function performs the following steps:
1.  **Inverse Transform**: Applies the inverse integer transform to the transform coefficients.
2.  **Scaling**: Scales the transform coefficients based on the quantization parameter.

### Scanning and Unscanning Functions

- `unscan_4x4` and `unscan_2x2`: Functions to convert a 1D index to 2D coordinates for 4x4 and 2x2 blocks, respectively, using a raster scan order.
- `un_zig_zag_4x4` and `zig_zag_4x4`: Functions to convert between a 1D index and 2D coordinates using a zig-zag scan order.

### Scaling Functions

- `level_scale_4x4`: Calculates the scaling factor for a single transform coefficient.
- `level_scale_4x4_block`: Applies scaling to a 4x4 block of coefficients.
- `dc_scale_4x4_block` and `dc_scale_2x2_block`: Apply scaling to the DC coefficients of luma and chroma blocks.

### Transformation Functions

- `transform_dc`: Applies the 2D Hadamard transform to the luma DC coefficients for Intra_16x16 macroblocks.
- `transform_chroma_dc`: Applies the 2D Hadamard transform to the chroma DC coefficients.
- `transform_4x4`: Applies the inverse integer transform to a 4x4 block of residual coefficients.

### `unzip_block_4x4` and `unscan_block_4x4`

Utility functions to convert a 1D array of 16 coefficients into a 2D `Block4x4` struct, using zig-zag and raster scan orders, respectively.

## Testing

The file includes a `tests` module with unit tests for:
- `norm_adjust_4x4`: Verifies the correctness of the `norm_adjust` table values.
- `zig_zag`: Tests the zig-zag scanning functions.
- `transform_4x4`: Tests the inverse transform with examples from the H.264 specification.
- `unscan_block_4x4`: Tests the raster scan conversion.
