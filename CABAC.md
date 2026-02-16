# Context-based Adaptive Binary Arithmetic Coding (CABAC) in H.264

This document provides a detailed explanation of the CABAC entropy decoding process as specified in **Section 9.3** of the ITU-T H.264 Recommendation.

## 1. Overview

CABAC (Context-based Adaptive Binary Arithmetic Coding) is an entropy coding method used in H.264 (Main and High profiles) to achieve high compression efficiency. Unlike CAVLC (Context-based Adaptive Variable Length Coding), which uses look-up tables for variable-length codes, CABAC uses arithmetic coding driven by adaptive probability models.

The core idea is to encode syntax elements (like motion vectors, coefficients, and flags) as a sequence of binary decisions ("bins"). Each bin is encoded/decoded based on a probability model (context) that adapts to the local statistics of the stream.

## 2. The CABAC Decoding Pipeline

The decoding of a syntax element follows three main stages:

1.  **Binarization**: The requested syntax element (which might be non-binary, e.g., a motion vector difference) is mapped to a unique binary string (sequence of bins).
2.  **Context Modeling**: For each bin in the bin string, a context index (`ctxIdx`) is derived. This index points to a probability model.
3.  **Arithmetic Decoding**: The arithmetic decoding engine uses the probability model associated with `ctxIdx` to decode the bin value (0 or 1) from the bitstream.

### Flowchart (Conceptual)

```
Request Syntax Element
       |
       v
  Binarization (Determine bin string structure)
       |
       v
  For each Bin in Bin String:
       |
       +--> Determine ctxIdx (Context Modeling)
       |    (Or use Bypass Mode)
       |
       v
  Arithmetic Decoding Engine (DecodeBin)
       |
       +--> Update Context Model (Probability Estimate)
       |
       v
  Match Bin String -> Return Syntax Element Value
```

## 3. Initialization

Before parsing slice data, the CABAC engine and context variables must be initialized.

*   **Context Variables**: There are 1024 possible contexts (`ctxIdx` 0 to 1023). Each context maintains a probability state index (`pStateIdx`, 0-63) and the value of the Most Probable Symbol (`valMPS`, 0 or 1).
    *   Initialization values depend on `SliceQPY` (slice quantization parameter) and pre-defined tables (tables 9-12 to 9-33 in the spec).
*   **Decoding Engine**: The internal registers `codIRange` (interval range) and `codIOffset` (current bits within the range) are initialized. `codIRange` starts at 510. `codIOffset` is loaded with 9 bits from the bitstream.

## 4. Binarization

Binarization maps non-binary syntax elements into a string of bins. The type of binarization depends on the syntax element (see Spec Table 9-34).

Common binarization schemes:
*   **Unary (U)**: Value `x` becomes `x` ones followed by a zero (e.g., 3 -> `1110`).
*   **Truncated Unary (TU)**: Like Unary, but stops at a maximum length `cMax`. If `x == cMax`, the trailing zero is omitted.
*   **Fixed-Length (FL)**: Binary representation of fixed bit-depth.
*   **Concatenated Unary / k-th order Exp-Golomb (UEGk)**: A prefix (TU) followed by a suffix (Exp-Golomb) for larger values. Used for motion vector differences (`mvd`) and coefficient levels.

## 5. Context Modeling (`ctxIdx` Derivation)

This is the "Adaptive" part of CABAC. The probability model for a bin depends on **context**.

### Derivation Logic
The context index `ctxIdx` for a specific bin (`binIdx`) is calculated as:
`ctxIdx = ctxIdxOffset + ctxIdxInc`

*   **`ctxIdxOffset`**: A base offset specific to the syntax element (e.g., `mb_type`, `ref_idx`).
*   **`ctxIdxInc`**: An increment that refines the context based on:
    *   **Bin Index (`binIdx`)**: Earlier bins in the string might use different contexts than later ones.
    *   **Neighboring Information**: Values from blocks to the Left (A) and Above (B).
        *   Example: `mb_skip_flag` context depends on whether neighbors A and B were skipped.
    *   **Prior Decoded Bins**: For some elements, `ctxIdxInc` depends on previously decoded bins of the same element.
    *   **Block Category (`ctxBlockCat`)**: For residual data, contexts differ based on block type (Luma DC, Luma AC, Chroma, etc.).

## 6. Arithmetic Decoding Engine

The engine maintains a probability interval. The state is defined by:
*   `codIRange`: Current interval width (quantized to 9 bits precision for calculations).
*   `codIOffset`: The actual bitstream bits inside the current interval.

### 6.1. Decode Decision (Regular Mode)
Used when `bypassFlag == 0`.

1.  **LPS Range Calculation**: The range for the Least Probable Symbol (LPS) is determined via a look-up table (`rangeTabLPS`) using the current `codIRange` (quantized) and the context's `pStateIdx`.
    *   `codIRangeLPS = rangeTabLPS[pStateIdx][qCodIRangeIdx]`
2.  **Interval Subdivision**:
    *   `codIRange = codIRange - codIRangeLPS`
3.  **Decision**:
    *   If `codIOffset >= codIRange`: The bin is the **LPS**.
        *   `binVal = 1 - valMPS`
        *   `codIOffset -= codIRange`
        *   `codIRange = codIRangeLPS`
    *   Otherwise: The bin is the **MPS**.
        *   `binVal = valMPS`
4.  **State Update**:
    *   If MPS decoded: `pStateIdx` transitions to a state representing higher MPS probability (up to max 63).
    *   If LPS decoded: `pStateIdx` transitions to a lower probability state. If it hits 0, `valMPS` flips (0 <-> 1).
5.  **Renormalization**: If `codIRange` drops below 256, it is doubled (left-shift) and new bits are shifted into `codIOffset` from the bitstream until `codIRange >= 256`.

### 6.2. Decode Bypass
Used for bins assumed to have uniform probability (0.5), usually essentially "random" noise (e.g., sign bits, lower bits of motion vectors).

*   Skips the context look-up and state update.
*   Doubles `codIOffset` and reads 1 bit.
*   Compares `codIOffset` with `codIRange` to determine 0 or 1.
*   Much faster than regular decoding.

### 6.3. Terminate
Used for the `end_of_slice_flag` or special I_PCM modes.
*   Treats the termination bin as having an extremely low probability of being 1.
*   If decoded as 1, the engine flushes and stops.

## 7. Key Tables and Their Purpose

The H.264 specification defines numerous tables to drive the CABAC process. These tables ensure that the encoder and decoder remain perfectly synchronized.

### 7.1. Initialization Tables
These tables are used at the start of each slice (or after I_PCM blocks) to set the initial probability state (`pStateIdx`) and Most Probable Symbol (`valMPS`) for each of the 1024 context models (`ctxIdx`). The initialization is based on the Slice Quantization Parameter (`SliceQPY`).

*   **Table 9-11**: The master map. It associates every syntax element (and slice type) with a specific range of `ctxIdx` and points to the relevant initialization table (9-12 to 9-33).
*   **Tables 9-12 to 9-33**: Contain the `m` and `n` parameters for linear interpolation.
    *   **Purpose**: Calculate the initial state value `preCtxState` using the formula: `preCtxState = Clip3(1, 126, ((m * SliceQPY) >> 4) + n)`. This value is then split into `pStateIdx` and `valMPS`.
    *   **Why so many?** Different syntax elements have different statistical properties. For example, `mb_type` probabilities behave differently than `mvd` probabilities. The standard groups similar elements into tables to save space while maintaining accuracy.

### 7.2. Binarization Tables
*   **Table 9-34**: The master Binarization map.
    *   **Purpose**: Tells you *how* to convert a syntax element into a string of bins. It specifies:
        *   **Binarization Type**: (FL, U, TU, UEGk, etc.)
        *   **`ctxIdxOffset`**: The starting index for context models for this element.
        *   **`maxBinIdxCtx`**: How many bins use unique contexts before falling back to shared or bypass contexts.
    *   **Tables 9-35 (U), 9-36 to 9-38 (mb_type)**: Explicitly list the bin strings for specific syntax elements like `mb_type`, which have custom prefix/suffix tree structures.

### 7.3. Context Index Derivation Tables
Once you have a bin string, you need to know which `ctxIdx` to use for each bin. These tables map the `binIdx` (position in the string) to a `ctxIdx` increment.

*   **Table 9-39**: The primary lookup for `ctxIdxInc`.
    *   **Purpose**: Maps a `ctxIdxOffset` (from Table 9-34) and `binIdx` to a `ctxIdxInc`.
    *   **Significance**: This table encodes the structure of the syntax element. For example, it might say "For `mvd` (offset 40), bin 0 uses increment 0/1/2 (based on neighbors), bins 1-2 use increment 3/4, and bins >3 use increment 6."
*   **Table 9-40**: Offset table for Residual Data (Coefficients).
    *   **Purpose**: Residual coding is complex. This table adds offsets based on the `ctxBlockCat` (e.g., Luma DC vs. Chroma AC) so that different block types use different context ranges.
*   **Table 9-41**: Logic based on prior bins.
    *   **Purpose**: For some elements, the context for bin `N` depends on the value of bin `N-1`. This table captures those rules.
*   **Table 9-42**: Block Category (`ctxBlockCat`) definitions.
    *   **Purpose**: IDs for different types of transform blocks (0=LumaDC, 1=LumaAC, etc.) used to multiplex context models for residuals.
*   **Table 9-43**: Scan position to Context Index map.
    *   **Purpose**: For 8x8 blocks, the context changes depending on the frequency position (scan index) of the coefficient. This table maps the scan index (0-63) to a limited set of contexts to avoid needing 64 separate contexts.

### 7.4. Arithmetic Decoding Engine Tables
These tables drive the core mathematical engine. They replace complex multiplications and divisions with fast lookups.

*   **Table 9-44**: `rangeTabLPS` (The Range Table).
    *   **Dimensions**: 64 (probability states) x 4 (quantized range indices).
    *   **Purpose**: Returns the interval width (`codIRangeLPS`) for the Least Probable Symbol (LPS).
    *   **How it works**: Instead of calculating `CurrentRange * ProbabilityLPS` (which requires multiplication), the engine quantizes `CurrentRange` to 2 bits (4 values) and uses the pre-computed table. This is the critical optimization that makes CABAC feasible in hardware/software.
*   **Table 9-45**: State Transition Table (`transIdxMPS`, `transIdxLPS`).
    *   **Dimensions**: 64 states x 2 outcomes (MPS or LPS).
    *   **Purpose**: Determines the *next* probability state based on the current state and the decoded bin.
    *   **Dynamics**:
        *   **Decode MPS**: Move to a state with higher probability for the MPS (saturates at state 62).
        *   **Decode LPS**: Move to a state with lower probability (faster adaptation). If at state 0, flip the MPS value (0 becomes 1 or vice versa).

---
*Reference: ITU-T Rec. H.264 (06/2019), Section 9.3*
