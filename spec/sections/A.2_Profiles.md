**A.2** **Profiles**


All constraints for picture parameter sets that are specified in clauses A.2.1 to A.2.11 are constraints for picture parameter
sets that are activated in the bitstream. All constraints for sequence parameter sets that are specified in clauses A.2.1 to
A.2.11 are constraints for sequence parameter sets that are activated in the bitstream.


**A.2.1** **Baseline profile**


Bitstreams conforming to the Baseline profile shall obey the following constraints:

- Only I and P slice types may be present.

- NAL unit streams shall not contain nal_unit_type values in the range of 2 to 4, inclusive.

- Sequence parameter sets shall have frame_mbs_only_flag equal to 1.

- The syntax elements chroma_format_idc, bit_depth_luma_minus8, bit_depth_chroma_minus8,
qpprime_y_zero_transform_bypass_flag, and seq_scaling_matrix_present_flag shall not be present in sequence
parameter sets.

- Picture parameter sets shall have weighted_pred_flag and weighted_bipred_idc both equal to 0.

- Picture parameter sets shall have entropy_coding_mode_flag equal to 0.

- Picture parameter sets shall have num_slice_groups_minus1 in the range of 0 to 7, inclusive.

- The syntax elements transform_8x8_mode_flag, pic_scaling_matrix_present_flag, and
second_chroma_qp_index_offset shall not be present in picture parameter sets.

- The syntax element level_prefix shall not be greater than 15 (when present).

- The syntax elements pcm_sample_luma[ i ], with i = 0..255, and pcm_sample_chroma[ i ], with
i = 0..2 * MbWidthC * MbHeightC − 1, shall not be equal to 0 (when present).

- The level constraints specified for the Baseline profile in clause A.3 shall be fulfilled.


Conformance of a bitstream to the Baseline profile is indicated by profile_idc being equal to 66.





Decoders conforming to the Baseline profile at a specific level shall be capable of decoding all bitstreams in which
profile_idc is equal to 66 or constraint_set0_flag is equal to 1 and in which level_idc and constraint_set3_flag represent a
level less than or equal to the specified level.


**A.2.1.1** **Constrained Baseline profile**


Bitstreams conforming to the Constrained Baseline profile shall obey all constraints specified in clause A.2.1 for the
Baseline profile and all constraints specified in clause A.2.2 for the Main profile.


Conformance of a bitstream to the Constrained Baseline profile is indicated by profile_idc being equal to 66 with
constraint_set1_flag being equal to 1.

NOTE – This specification of the Constrained Baseline profile is technically identical to specification of the use of the Baseline
profile with constraint_set1_flag equal to 1. Thus, any existing specifications (in other documents that reference this
Recommendation | International Standard) that have referred to the use of the Baseline profile with constraint_set1_flag equal to 1
should thus be interpreted as continuing in force as being technically identical to referring to the use of the Constrained Baseline
profile (without any need for revision of these existing specifications to instead refer explicitly to the use of the Constrained Baseline
profile).


Decoders conforming to the Constrained Baseline profile at a specific level shall be capable of decoding all bitstreams in
which all of the following are true:

- profile_idc is equal to 66 or constraint_set0_flag is equal to 1,

- constraint_set1_flag is equal to 1,

- level_idc and constraint_set3_flag represent a level less than or equal to the specified level.


**A.2.2** **Main profile**


Bitstreams conforming to the Main profile shall obey the following constraints:

- Only I, P, and B slice types may be present.

- NAL unit streams shall not contain nal_unit_type values in the range of 2 to 4, inclusive.

- Arbitrary slice order is not allowed.

- The syntax elements chroma_format_idc, bit_depth_luma_minus8, bit_depth_chroma_minus8,
qpprime_y_zero_transform_bypass_flag, and seq_scaling_matrix_present_flag shall not be present in sequence
parameter sets.

- Picture parameter sets shall have num_slice_groups_minus1 equal to 0 only.

- Picture parameter sets shall have redundant_pic_cnt_present_flag equal to 0 only.

- The syntax elements transform_8x8_mode_flag, pic_scaling_matrix_present_flag, and
second_chroma_qp_index_offset shall not be present in picture parameter sets.

- The syntax element level_prefix shall not be greater than 15 (when present).

- The syntax elements pcm_sample_luma[ i ], with i = 0..255, and pcm_sample_chroma[ i ], with
i = 0..2 * MbWidthC * MbHeightC − 1, shall not be equal to 0 (when present).

- The level constraints specified for the Main profile in clause A.3 shall be fulfilled.


Conformance of a bitstream to the Main profile is indicated by profile_idc being equal to 77.


Decoders conforming to the Main profile at a specified level shall be capable of decoding all bitstreams in which profile_idc
is equal to 77 or constraint_set1_flag is equal to 1 and in which level_idc and constraint_set3_flag represent a level less
than or equal to the specified level.


**A.2.3** **Extended profile**


Bitstreams conforming to the Extended profile shall obey the following constraints:

- Sequence parameter sets shall have direct_8x8_inference_flag equal to 1.

- The syntax elements chroma_format_idc, bit_depth_luma_minus8, bit_depth_chroma_minus8,
qpprime_y_zero_transform_bypass_flag, and seq_scaling_matrix_present_flag shall not be present in sequence
parameter sets.

- Picture parameter sets shall have entropy_coding_mode_flag equal to 0.

- Picture parameter sets shall have num_slice_groups_minus1 in the range of 0 to 7, inclusive.

- The syntax elements transform_8x8_mode_flag, pic_scaling_matrix_present_flag, and
second_chroma_qp_index_offset shall not be present in picture parameter sets.

- The syntax element level_prefix shall not be greater than 15 (when present).





- The syntax elements pcm_sample_luma[ i ], with i = 0..255, and pcm_sample_chroma[ i ], with
i = 0..2 * MbWidthC * MbHeightC − 1, shall not be equal to 0 (when present).

- The level constraints specified for the Extended profile in clause A.3 shall be fulfilled.


Conformance of a bitstream to the Extended profile is indicated by profile_idc being equal to 88.


Decoders conforming to the Extended profile at a specified level shall be capable of decoding all bitstreams in which
profile_idc is equal to 88 or constraint_set2_flag is equal to 1 and in which level_idc represents a level less than or equal
to specified level.


Decoders conforming to the Extended profile at a specified level shall also be capable of decoding all bitstreams in which
profile_idc is equal to 66 or constraint_set0_flag is equal to 1, in which level_idc and constraint_set3_flag represent a level
less than or equal to the specified level.


**A.2.4** **High profile**


Bitstreams conforming to the High profile shall obey the following constraints:

- Only I, P, and B slice types may be present.

- NAL unit streams shall not contain nal_unit_type values in the range of 2 to 4, inclusive.

- Arbitrary slice order is not allowed.

- Picture parameter sets shall have num_slice_groups_minus1 equal to 0 only.

- Picture parameter sets shall have redundant_pic_cnt_present_flag equal to 0 only.

- Sequence parameter sets shall have chroma_format_idc in the range of 0 to 1 inclusive.

- Sequence parameter sets shall have bit_depth_luma_minus8 equal to 0 only.

- Sequence parameter sets shall have bit_depth_chroma_minus8 equal to 0 only.

- Sequence parameter sets shall have qpprime_y_zero_transform_bypass_flag equal to 0 only.

- The level constraints specified for the High profile in clause A.3 shall be fulfilled.


Conformance of a bitstream to the High profile is indicated by profile_idc being equal to 100. Decoders conforming to the
High profile at a specific level shall be capable of decoding all bitstreams in which either or both of the following conditions
are true:

- (profile_idc is equal to 77 or constraint_set1_flag is equal to 1) and the combination of level_idc and
constraint_set3_flag represent a level less than or equal to the specified level,

- profile_idc is equal to 100 and level_idc represents a level less than or equal to the specified level.
NOTE – The value 100 for profile_idc indicates that the bitstream conforms to the High profile as specified in this clause. When
profile_idc is equal to 100 and constraint_set3_flag is equal to 1, this indicates that the bitstream conforms to the High profile and
additionally conforms to the constraints specified for the High 10 Intra profile in clause A.2.8. For example, such a bitstream must
have bit_depth_luma_minus8 equal to 0, have bit_depth_chroma_minus8 equal to 0, obey the MinCR, MaxBR and MaxCPB
constraints of the High profile, contain only IDR pictures, have max_num_ref_frames equal to 0, have dpb_output_delay equal to 0,
and obey the maximum slice size constraint of the High 10 Intra profile.


**A.2.4.1** **Progressive High profile**


Bitstreams conforming to the Progressive High profile shall obey all constraints specified in clause A.2.4 for the High
profile, and shall additionally obey the constraint that sequence parameter sets shall have frame_mbs_only_flag equal to 1.


Conformance of a bitstream to the Progressive High profile is indicated by profile_idc being equal to 100 with
constraint_set4_flag being equal to 1.


Decoders conforming to the Progressive High profile at a specific level shall be capable of decoding all bitstreams in which
one or more of the following conditions are true:

`–` (profile_idc is equal to 66 or constraint_set0_flag is equal to 1), constraint_set1_flag is equal to 1, and the combination
of level_idc and constraint_set3_flag represents a level less than or equal to the specified level.

`–` profile_idc is equal to 77, constraint_set0_flag is equal to 1, and the combination of level_idc and constraint_set3_flag
represents a level less than or equal to the specified level.

`–` profile_idc is equal to 77, constraint_set4_flag is equal to 1, and the combination of level_idc and constraint_set3_flag
represents a level less than or equal to the specified level.

`–` profile_idc is equal to 88, constraint_set1_flag is equal to 1, constraint_set4_flag is equal to 1, and the combination of
level_idc and constraint_set3_flag represents a level less than or equal to the specified level.

`–` profile_idc is equal to 100, constraint_set4_flag is equal to 1, and level_idc represents a level less than or equal to the
specified level.





**A.2.4.2** **Constrained High profile**


Bitstreams conforming to the Constrained High profile shall obey all constraints specified in clause A.2.4.1 for the
Progressive High profile, and shall additionally obey the constraint that B slice types shall not be present.


Conformance of a bitstream to the Constrained High profile is indicated by profile_idc being equal to 100 with both
constraint_set4_flag and constraint_set5_flag being equal to 1.


Decoders conforming to the Constrained High profile at a specific level shall be capable of decoding all bitstreams in
which one or more of the following conditions are true:

`–` (profile_idc is equal to 66 or constraint_set0_flag is equal to 1), constraint_set1_flag is equal to 1, and the combination
of level_idc and constraint_set3_flag represents a level less than or equal to the specified level.

`–` profile_idc is equal to 77, constraint_set0_flag is equal to 1, and the combination of level_idc and constraint_set3_flag
represents a level less than or equal to the specified level.

`–` profile_idc is equal to 77, constraint_set4_flag is equal to 1, constraint_set5_flag is equal to 1, and level_idc represents
a level less than or equal to the specified level.

`–` profile_idc is equal to 88, constraint_set1_flag is equal to 1, constraint_set4_flag is equal to 1, constraint_set5_flag is
equal to 1, and the combination of level_idc and constraint_set3_flag represents a level less than or equal to the
specified level.

`–` profile_idc is equal to 100, constraint_set4_flag is equal to 1, constraint_set5_flag is equal to 1, and level_idc
represents a level less than or equal to the specified level.


**A.2.5** **High 10 profile**


Bitstreams conforming to the High 10 profile shall obey the following constraints:

- Only I, P, and B slice types may be present.

- NAL unit streams shall not contain nal_unit_type values in the range of 2 to 4, inclusive.

- Arbitrary slice order is not allowed.

- Picture parameter sets shall have num_slice_groups_minus1 equal to 0 only.

- Picture parameter sets shall have redundant_pic_cnt_present_flag equal to 0 only.

- Sequence parameter sets shall have chroma_format_idc in the range of 0 to 1 inclusive.

- Sequence parameter sets shall have bit_depth_luma_minus8 in the range of 0 to 2 inclusive.

- Sequence parameter sets shall have bit_depth_chroma_minus8 in the range of 0 to 2 inclusive.

- Sequence parameter sets shall have qpprime_y_zero_transform_bypass_flag equal to 0 only.

- The level constraints specified for the High 10 profile in clause A.3 shall be fulfilled.


Conformance of a bitstream to the High 10 profile is indicated by profile_idc being equal to 110. Decoders conforming to
the High 10 profile at a specific level shall be capable of decoding all bitstreams in which either or both of the following
conditions are true:

- (profile_idc is equal to 77 or constraint_set1_flag is equal to 1) and the combination of level_idc and
constraint_set3_flag represent a level less than or equal to the specified level,

- profile_idc is equal to 100 or 110 and level_idc represents a level less than or equal to the specified level.


**A.2.5.1** **Progressive High 10 profile**


Bitstreams conforming to the Progressive High 10 profile shall obey all constraints specified in clause A.2.5 for the
High 10 profile, and shall additionally obey the constraint that sequence parameter sets shall have frame_mbs_only_flag
equal to 1.


Conformance of a bitstream to the Progressive High 10 profile is indicated by profile_idc being equal to 110 with
constraint_set4_flag being equal to 1.


Decoders conforming to the Progressive High 10 profile at a specific level shall be capable of decoding all bitstreams in
which one or more of the following conditions are true:

- (profile_idc is equal to 66 or constraint_set0_flag is equal to 1), constraint_set1_flag is equal to 1, and the combination
of level_idc and constraint_set3_flag represents a level less than or equal to the specified level.

- profile_idc is equal to 77, constraint_set0_flag is equal to 1, and the combination of level_idc and constraint_set3_flag
represents a level less than or equal to the specified level.





- profile_idc is equal to 77, constraint_set4_flag is equal to 1, and the combination of level_idc and constraint_set3_flag
represents a level less than or equal to the specified level.

- profile_idc is equal to 88, constraint_set1_flag is equal to 1, constraint_set4_flag is equal to 1, and the combination
of level_idc and constraint_set3_flag represents a level less than or equal to the specified level.

- profile_idc is equal to 100 or 110, constraint_set4_flag is equal to 1, and level_idc represents a level less than or equal
to the specified level.


**A.2.6** **High 4:2:2 profile**


Bitstreams conforming to the High 4:2:2 profile shall obey the following constraints:

- Only I, P, and B slice types may be present.

- NAL unit streams shall not contain nal_unit_type values in the range of 2 to 4, inclusive.

- Arbitrary slice order is not allowed.

- Picture parameter sets shall have num_slice_groups_minus1 equal to 0 only.

- Picture parameter sets shall have redundant_pic_cnt_present_flag equal to 0 only.

- Sequence parameter sets shall have chroma_format_idc in the range of 0 to 2 inclusive.

- Sequence parameter sets shall have bit_depth_luma_minus8 in the range of 0 to 2 inclusive.

- Sequence parameter sets shall have bit_depth_chroma_minus8 in the range of 0 to 2 inclusive.

- Sequence parameter sets shall have qpprime_y_zero_transform_bypass_flag equal to 0 only.

- The level constraints specified for the High 4:2:2 profile in clause A.3 shall be fulfilled.


Conformance of a bitstream to the High 4:2:2 profile is indicated by profile_idc being equal to 122. Decoders conforming
to the High 4:2:2 profile at a specific level shall be capable of decoding all bitstreams in which either or both of the
following conditions are true:

- (profile_idc is equal to 77 or constraint_set1_flag is equal to 1) and the combination of level_idc and
constraint_set3_flag represent a level less than or equal to the specified level,

- profile_idc is equal to 100, 110, or 122 and level_idc represents a level less than or equal to the specified level.


**A.2.7** **High 4:4:4 Predictive profile**


Bitstreams conforming to the High 4:4:4 Predictive profile shall obey the following constraints:

- Only I, P, B slice types may be present.

- NAL unit streams shall not contain nal_unit_type values in the range of 2 to 4, inclusive.

- Arbitrary slice order is not allowed.

- Picture parameter sets shall have num_slice_groups_minus1 equal to 0 only.

- Picture parameter sets shall have redundant_pic_cnt_present_flag equal to 0 only.

- Sequence parameter sets shall have bit_depth_luma_minus8 in the range of 0 to 6 inclusive.

- Sequence parameter sets shall have bit_depth_chroma_minus8 in the range of 0 to 6 inclusive.

- The level constraints specified for the High 4:4:4 Predictive profile in clause A.3 shall be fulfilled.


Conformance of a bitstream to the High 4:4:4 Predictive profile is indicated by profile_idc being equal to 244. Decoders
conforming to the High 4:4:4 Predictive profile at a specific level shall be capable of decoding all bitstreams in which
either or both of the following conditions are true:

- (profile_idc is equal to 77 or constraint_set1_flag is equal to 1) and the combination of level_idc and
constraint_set3_flag represent a level less than or equal to the specified level,

- profile_idc is equal to 44, 100, 110, 122, or 244 and the value of level_idc represents a level less than or equal to the
specified level.


**A.2.8** **High 10 Intra profile**


Bitstreams conforming to the High 10 Intra profile shall obey the following constraints:

- All constraints specified in clause A.2.5 for the High 10 profile shall be obeyed.

- All pictures shall be IDR pictures.

- Sequence parameter sets shall have max_num_ref_frames equal to 0.





- When vui_parameters_present_flag is equal to 1 and bitstream_restriction_flag is equal to 1, sequence parameter sets
shall have max_num_reorder_frames equal to 0.

- When vui_parameters_present_flag is equal to 1 and bitstream_restriction_flag is equal to 1, sequence parameter sets
shall have max_dec_frame_buffering equal to 0.

- Picture timing SEI messages, whether present in the bitstream (by non-VCL NAL units) or conveyed equivalently by
other means not specified in this Recommendation | International Standard, shall have dpb_output_delay equal to 0.

- The level constraints specified for the High 10 Intra profile in clause A.3 shall be fulfilled.


Conformance of a bitstream to the High 10 Intra profile is indicated by constraint_set3_flag being equal to 1 with
profile_idc equal to 110. Decoders conforming to the High 10 Intra profile at a specific level shall be capable of decoding
all bitstreams in which all of the following conditions are true:

- profile_idc is equal to 100 or 110,

- constraint_set3_flag is equal to 1,

- level_idc represents a level less than or equal to the specified level.
NOTE 1 – The value 100 for profile_idc indicates that the bitstream conforms to the High profile as specified in clause A.2.4. When
profile_idc is equal to 100 and constraint_set3_flag is equal to 1, this indicates that the bitstream conforms to the High profile and
additionally conforms to the constraints specified for the High 10 Intra profile in this clause. For example, such a bitstream must
have bit_depth_luma_minus8 equal to 0, have bit_depth_chroma_minus8 equal to 0, obey the MinCR, MaxBR and MaxCPB
constraints of the High profile, contain only IDR pictures, have max_num_ref_frames equal to 0, have dpb_output_delay equal to 0,
and obey the maximum slice size constraint of the High 10 Intra profile.


The operation of the deblocking filter process specified in clause 8.7 is not required for decoder conformance to the
High 10 Intra profile.

NOTE 2 – The deblocking filter process specified in clause 8.7 or some similar post-processing filter should be performed, although
this is not a requirement for decoder conformance to the High 10 Intra profile. The syntax elements sent by an encoder for control
of the deblocking filter process specified in clause 8.7 are considered only as advisory information for decoders conformance to the
High 10 Intra profile. However, the application of the deblocking filter process specified in clause 8.7 is required for decoder
conformance to the High 10, High 4:2:2, and High 4:4:4 Predictive profiles when decoding bitstreams that conform to the High 10
Intra profile.


**A.2.9** **High 4:2:2 Intra profile**


Bitstreams conforming to the High 4:2:2 Intra profile shall obey the following constraints:

- All constraints specified in clause A.2.6 for the High 4:2:2 profile shall be obeyed.

- All pictures shall be IDR pictures.

- Sequence parameter sets shall have max_num_ref_frames equal to 0.

- When vui_parameters_present_flag is equal to 1 and bitstream_restriction_flag is equal to 1, sequence parameter sets
shall have max_num_reorder_frames equal to 0.

- When vui_parameters_present_flag is equal to 1 and bitstream_restriction_flag is equal to 1, sequence parameter sets
shall have max_dec_frame_buffering equal to 0.

- Picture timing SEI messages, whether present in the bitstream (by non-VCL NAL units) or conveyed equivalently by
other means not specified in this Recommendation | International Standard, shall have dpb_output_delay equal to 0.

- The level constraints specified for the High 4:2:2 Intra profile in clause A.3 shall be fulfilled.


Conformance of a bitstream to the High 4:2:2 Intra profile is indicated by constraint_set3_flag being equal to 1 with
profile_idc equal to 122. Decoders conforming to the High 4:2:2 Intra profile at a specific level shall be capable of decoding
all bitstreams in which all of the following conditions are true:

- profile_idc is equal to 100, 110, or 122,

- constraint_set3_flag is equal to 1,

- level_idc represents a level less than or equal to the specified level.


The operation of the deblocking filter process specified in clause 8.7 is not required for decoder conformance to the
High 4:2:2 Intra profile.

NOTE – The deblocking filter process specified in clause 8.7 or some similar post-processing filter should be performed, although
this is not a requirement for decoder conformance to the High 4:2:2 Intra profile. The syntax elements sent by an encoder for control
of the deblocking filter process specified in clause 8.7 are considered only as advisory information for decoders conformance to the
High 4:2:2 Intra profile. However, the application of the deblocking filter process specified in clause 8.7 is required for decoder
conformance to the High 4:2:2, and High 4:4:4 Predictive profiles when decoding bitstreams that conform to the High 4:2:2 Intra
profile.





**A.2.10** **High 4:4:4 Intra profile**


Bitstreams conforming to the High 4:4:4 Intra profile shall obey the following constraints:

- All constraints specified in clause A.2.7 for the High 4:4:4 Predictive profile shall be obeyed.

- All pictures shall be IDR pictures.

- Sequence parameter sets shall have max_num_ref_frames equal to 0.

- When vui_parameters_present_flag is equal to 1 and bitstream_restriction_flag is equal to 1, sequence parameter sets
shall have max_num_reorder_frames equal to 0.

- When vui_parameters_present_flag is equal to 1 and bitstream_restriction_flag is equal to 1, sequence parameter sets
shall have max_dec_frame_buffering equal to 0.

- Picture timing SEI messages, whether present in the bitstream (by non-VCL NAL units) or conveyed equivalently by
other means not specified in this Recommendation | International Standard, shall have dpb_output_delay equal to 0.

- The level constraints specified for the High 4:4:4 Intra profile in clause A.3 shall be fulfilled.


Conformance of a bitstream to the High 4:4:4 Intra profile is indicated by constraint_set3_flag being equal to 1 with
profile_idc equal to 244. Decoders conforming to the High 4:4:4 Intra profile at a specific level shall be capable of
decoding all bitstreams in which all of the following conditions are true:

- profile_idc is equal to 44, 100, 110, 122, or 244,

- constraint_set3_flag is equal to 1,

- level_idc represents a level less than or equal to the specified level.


The operation of the deblocking filter process specified in clause 8.7 is not required for decoder conformance to the
High 4:4:4 Intra profile.

NOTE – The deblocking filter process specified in clause 8.7 or some similar post-processing filter should be performed, although
this is not a requirement for decoder conformance to the High 4:4:4 Intra and CAVLC 4:4:4 Intra profiles. The syntax elements sent
by an encoder for control of the deblocking filter process specified in clause 8.7 are considered only as advisory information for
decoders conformance to the High 4:4:4 Intra and CAVLC 4:4:4 Intra profiles. However, the application of the deblocking filter
process specified in clause 8.7 is required for decoder conformance to the High 4:4:4 Predictive profile when decoding bitstreams
that conform to the High 4:4:4 Intra and CAVLC 4:4:4 Intra profiles.


**A.2.11** **CAVLC 4:4:4 Intra profile**


Bitstreams conforming to the CAVLC 4:4:4 Intra profile shall obey the following constraints:

- All constraints specified in clause A.2.10 for the High 4:4:4 Intra profile shall be obeyed.

- Picture parameter sets shall have entropy_coding_mode_flag equal to 0.

- The level constraints specified for the CAVLC 4:4:4 Intra profile in clause A.3 shall be fulfilled.


Conformance of a bitstream to the CAVLC 4:4:4 Intra profile is indicated by profile_idc being equal to 44. Decoders
conforming to the CAVLC 4:4:4 Intra profile at a specific level shall be capable of decoding all bitstreams in which all of
the following conditions are true:

- profile_idc is equal to 44,

- level_idc represents a level less than or equal to the specified level.


The operation of the deblocking filter process specified in clause 8.7 is not required for decoder conformance to the
CAVLC 4:4:4 Intra profile.

NOTE – The deblocking filter process specified in clause 8.7 or some similar post-processing filter should be performed, although
this is not a requirement for decoder conformance to the High 4:4:4 Intra and CAVLC 4:4:4 Intra profiles. The syntax elements sent
by an encoder for control of the deblocking filter process specified in clause 8.7 are considered only as advisory information for
decoders conformance to the High 4:4:4 Intra and CAVLC 4:4:4 Intra profiles. However, the application of the deblocking filter
process specified in clause 8.7 is required for decoder conformance to the High 4:4:4 Predictive profile when decoding bitstreams
that conform to the High 4:4:4 Intra and CAVLC 4:4:4 Intra profiles.
