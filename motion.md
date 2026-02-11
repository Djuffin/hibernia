# Motion Prediction in Hibernia

This document explains the implementation of motion prediction in the Hibernia H.264 decoder and outlines a plan to refactor the codebase to remove hardcoded sentinel values.

## Overview

Motion prediction (Inter prediction) in H.264 exploits temporal redundancy between frames. The decoder constructs a prediction for the current block by referencing previously decoded pictures. The offset between the current block and the reference block is defined by a motion vector (MV).

## Key Components

### 1. Motion Vector Derivation (`predict_mv_l0`)

The H.264 standard specifies a process to predict motion vectors from neighboring blocks (A, B, C). This is implemented in `src/h264/parser.rs` via `predict_mv_l0`.
The prediction depends on the availability of neighboring blocks and their reference indices.

### 2. Motion Information Storage (`MbMotion`)

The `MbMotion` struct in `src/h264/macroblock.rs` stores the motion information for a macroblock. It maintains a 4x4 grid of `PartitionInfo`, where each cell corresponds to a 4x4 pixel block.

```rust
pub struct MbMotion {
    pub partitions: [[PartitionInfo; 4]; 4],
}

pub struct PartitionInfo {
    pub ref_idx_l0: u8,
    pub mv_l0: MotionVector,
}
```

### 3. Motion Calculation (`calculate_motion`)

The `calculate_motion` function in `src/h264/parser.rs` is responsible for populating `MbMotion` for P-macroblocks. It iterates through the partitions defined by the macroblock type (e.g., 16x16, 16x8, 8x8) and calculates the final motion vectors by adding the Motion Vector Difference (MVD) from the bitstream to the predicted motion vector (MVP).

### 4. Neighbor Access (`get_motion_at_coord`)

To calculate MVP, the decoder needs to access motion information of neighboring blocks. `get_motion_at_coord` facilitates this. It handles:
-   Neighbors in adjacent macroblocks (already decoded).
-   Neighbors within the same macroblock (potentially not yet decoded).

## The Issue: Sentinel Value 255

When calculating motion vectors for partitions within the current macroblock, the decoder fills the `MbMotion` grid incrementally. To distinguish between "decoded/available" partitions and "not yet decoded" partitions, the code currently uses a hardcoded sentinel value:

```rust
// src/h264/parser.rs

// 255 is used as a sentinel for "not yet decoded" in calculate_motion
if info.ref_idx_l0 == 255 {
    None
} else {
    Some(info)
}
```

In `calculate_motion`, the grid is initialized with this sentinel:

```rust
// Mark all partitions as "Not yet decoded" (Unavailable) using sentinel ref_idx 255.
for row in motion.partitions.iter_mut() {
    for part in row.iter_mut() {
        part.ref_idx_l0 = 255;
    }
}
```

**Problems with this approach:**
1.  **Correctness**: While `ref_idx_l0` is typically small, `255` is technically a valid value for a reference index (u8). Using it as a sentinel could strictly lead to collision, although unlikely in typical profiles.
2.  **Readability**: Magic numbers make the code harder to understand.
3.  **Type Safety**: The type system is not used to express "absence" of value.

## Refactoring Plan

We will refactor `MbMotion` to use `Option<PartitionInfo>` to explicitly represent the "not yet decoded" state.

### Step 1: Modify `MbMotion` Struct

Update `src/h264/macroblock.rs` to wrap `PartitionInfo` in `Option`.

```rust
// Before
pub struct MbMotion {
    pub partitions: [[PartitionInfo; 4]; 4],
}

// After
pub struct MbMotion {
    pub partitions: [[Option<PartitionInfo>; 4]; 4],
}
```

### Step 2: Update `calculate_motion` and `get_motion_at_coord`

Update `src/h264/parser.rs`:

1.  **`calculate_motion`**:
    -   Remove the loop that sets `ref_idx_l0 = 255`.
    -   `MbMotion::default()` will naturally initialize partitions to `None`.
    -   When filling the grid, wrap the `PartitionInfo` in `Some(...)`.

2.  **`get_motion_at_coord`**:
    -   Update to handle the `Option` returned by `current_mb_motion`.
    -   Remove the check `if info.ref_idx_l0 == 255`. `None` will inherently mean "not available".

### Step 3: Update Consumers of `MbMotion`

Update other parts of the code that access `MbMotion` partitions to handle the `Option`.

1.  **`src/h264/decoder.rs`**:
    -   In `render_luma_inter_prediction` and `render_chroma_inter_prediction`, unwrap or check for `Some(partition)`. Since these run on fully decoded blocks, partitions should always be `Some`.

2.  **`src/h264/deblocking.rs`**:
    -   Update `check_motion_discontinuity` (specifically the `get_part` closure) to return the `Option` directly (or map it appropriately).

### Step 4: Verify

-   Compile to ensure all type errors are resolved.
-   Run tests (`cargo test`) to ensure no regression in decoding logic.
