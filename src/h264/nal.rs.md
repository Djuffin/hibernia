# `nal.rs` - Network Abstraction Layer (NAL) Unit

This file defines the data structures related to H.264 Network Abstraction Layer (NAL) units. NAL units are the basic building blocks of an H.264 bitstream. Each NAL unit consists of a header and a payload (RBSP).

## Key Data Structures

### `NalUnitType`

An enum representing the type of a NAL unit. The NAL unit type determines how the payload of the NAL unit should be interpreted. Some common NAL unit types include:

- `SeqParameterSet`: Sequence Parameter Set (SPS)
- `PicParameterSet`: Picture Parameter Set (PPS)
- `IDRSlice`: A slice from an Instantaneous Decoding Refresh (IDR) picture (a self-contained keyframe).
- `NonIDRSlice`: A slice from a non-IDR picture.

### `NalHeader`

A struct representing the header of a NAL unit. The NAL header is a single byte that contains three fields:

- `forbidden_zero_bit`: A bit that should always be zero.
- `nal_ref_idc`: A 2-bit field that indicates whether the NAL unit is a reference picture.
- `nal_unit_type`: A 5-bit field that specifies the type of the NAL unit.
