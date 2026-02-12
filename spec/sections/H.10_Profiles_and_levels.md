**H.10** **Profiles and levels**


The specifications in Annex A apply. Additional profiles and specific values of profile_idc are specified in the following.


The profiles that are specified in clause H.10.1 are also referred to as the profiles specified in Annex H.


**H.10.1** **Profiles**


All constraints for picture parameter sets that are specified in the following are constraints for picture parameter sets that
become the active picture parameter set or an active view picture parameter set inside the bitstream. All constraints for
MVC sequence parameter sets that are specified in the following are constraints for MVC sequence parameter sets that
become the active MVC sequence parameter set or an active view MVC sequence parameter set inside the bitstream.


**H.10.1.1Multiview High profile**


Bitstreams conforming to the Multiview High profile shall obey the following constraints:

- The base view bitstream as specified in clause H.8.5.4 shall obey all constraints of the Progressive High profile
specified in clause A.2.4.1 and all active sequence parameter sets shall fulfil one or more of the following conditions:

   - profile_idc is equal to 100 or 77 and constraint_set4_flag is equal to 1,

   - (profile_idc is equal to 66 or constraint_set0_flag is equal to 1) and constraint_set1_flag is equal to 1,

   - profile_idc is equal to 77 and constraint_set0_flag is equal to 1,

   - profile_idc is equal to 88, constraint_set1_flag is equal to 1, and constraint_set4_flag is equal to 1.

- Only I, P, and B slice types may be present.

- NAL unit streams shall not contain nal_unit_type values in the range of 2 to 4, inclusive.

- MVC sequence parameter sets shall have frame_mbs_only_flag equal to 1.

- Arbitrary slice order is not allowed.

- Picture parameter sets shall have num_slice_groups_minus1 equal to 0 only.

- Picture parameter sets shall have redundant_pic_cnt_present_flag equal to 0 only.

- MVC sequence parameter sets shall have chroma_format_idc in the range of 0 to 1 inclusive.

- MVC sequence parameter sets shall have bit_depth_luma_minus8 equal to 0 only.





- MVC sequence parameter sets shall have bit_depth_chroma_minus8 equal to 0 only.

- MVC sequence parameter sets shall have qpprime_y_zero_transform_bypass_flag equal to 0 only.

- The level constraints specified for the Multiview High profile in clause H.10.2 shall be fulfilled.


Conformance of a bitstream to the Multiview High profile is indicated by profile_idc being equal to 118.


Decoders conforming to the Multiview High profile at a specific level shall be capable of decoding all bitstreams in which
both of the following conditions are true:

a) All active MVC sequence parameter sets have one or more of the following conditions fulfilled:

   - profile_idc is equal to 118,

   - profile_idc is equal to 100 or 77 and constraint_set4_flag is equal to 1,

   - profile_idc is equal to 88, constraint_set1_flag is equal to 1, and constraint_set4_flag is equal to 1,

   - profile_idc is equal to 77 and constraint_set0_flag is equal to 1,

   - (profile_idc is equal to 66 or constraint_set0_flag is equal to 1) and constraint_set1_flag is equal to 1.

b) All active MVC sequence parameter sets have one or more of the following conditions fulfilled:

   - level_idc or (level_idc and constraint_set3_flag) represent a level less than or equal to the specific level,

   - level_idc[ i ] or (level_idc[ i ] and constraint_set3_flag) represent a level less than or equal to the specific level.


**H.10.1.2Stereo High profile**


Bitstreams conforming to the Stereo High profile shall obey the following constraints:


- The base view bitstream as specified in clause H.8.5.4 shall obey all constraints of the High profile specified in
clause A.2.4 and all active sequence parameter sets shall fulfil one of the following conditions:

   - profile_idc is equal to 77 or constraint_set1_flag is equal to 1,

   - profile_idc is equal to 100.


- Only I, P, and B slice types may be present.


- NAL unit streams shall not contain nal_unit_type values in the range of 2 to 4, inclusive.


- Arbitrary slice order is not allowed.


- Picture parameter sets shall have num_slice_groups_minus1 equal to 0 only.


- Picture parameter sets shall have redundant_pic_cnt_present_flag equal to 0 only.


- MVC sequence parameter sets shall have chroma_format_idc in the range of 0 to 1 inclusive.


- MVC sequence parameter sets shall have bit_depth_luma_minus8 equal to 0 only.


- MVC sequence parameter sets shall have bit_depth_chroma_minus8 equal to 0 only.


- MVC sequence parameter sets shall have qpprime_y_zero_transform_bypass_flag equal to 0 only.


- When num_views_minus1 is present in an MVC sequence parameter set, its value shall be less than 2.


- For each access unit, the value of level_idc for all active view MVC sequence parameter set RBSPs shall be the same
as the value of level_idc for the active MVC sequence parameter set RBSP.


- The level constraints specified for the Stereo High profile in clause H.10.2 shall be fulfilled.


Conformance of a bitstream to the Stereo High profile is indicated by profile_idc being equal to 128.


Decoders conforming to the Stereo High profile at a specific level shall be capable of decoding all bitstreams in which
both of the following conditions are true:

a) All active MVC sequence parameter sets have one or more of the following conditions fulfilled:


   - profile_idc is equal to 128,


   - profile_idc is equal to 118 and constraint_set5_flag is equal to 1,


   - profile_idc is equal to 100,


   - profile_idc is equal to 77 or constraint_set1_flag is equal to 1.

b) All active MVC sequence parameter sets have one or more of the following conditions fulfilled:





   - level_idc or (level_idc and constraint_set3_flag) represent a level less than or equal to the specific level,


   - level_idc[ i ] or (level_idc[ i ] and constraint_set3_flag) represent a level less than or equal to the specific level.


**H.10.1.3MFC High profile**


Bitstreams conforming to the MFC High profile shall obey the following constraints:


- The base view bitstream as specified in clause H.8.5.4 shall obey all constraints of the High profile specified in
clause A.2.4 and all active sequence parameter sets shall fulfil one of the following conditions:


   - profile_idc is equal to 77 or constraint_set1_flag is equal to 1,


   - profile_idc is equal to 100.


- Only I, P, and B slice types may be present.


- NAL unit streams shall not contain nal_unit_type values in the range of 2 to 4, inclusive.


- Arbitrary slice order is not allowed.


- Picture parameter sets shall have num_slice_groups_minus1 equal to 0 only.


- Picture parameter sets shall have redundant_pic_cnt_present_flag equal to 0 only.


- MVC sequence parameter sets shall have chroma_format_idc in the range of 0 to 1 inclusive.


- MVC sequence parameter sets shall have bit_depth_luma_minus8 equal to 0 only.


- MVC sequence parameter sets shall have bit_depth_chroma_minus8 equal to 0 only.


- MVC sequence parameter sets shall have qpprime_y_zero_transform_bypass_flag equal to 0 only.


- When num_views_minus1 is present in an MVC sequence parameter set, its value shall be less than 2.


- For each access unit, the value of level_idc for all active view MVC sequence parameter set RBSPs shall be the same
as the value of level_idc for the active MVC sequence parameter set RBSP.


- The level constraints specified for the MFC High profile in clause H.10.2 shall be fulfilled.


Conformance of a bitstream to the MFC High profile is indicated by profile_idc being equal to 134.


Decoders conforming to the MFC High profile at a specific level shall be capable of decoding all bitstreams in which both
of the following conditions are true:


a) All active MVC sequence parameter sets have one or more of the following conditions fulfilled:


   - profile_idc is equal to 134,


   - profile_idc is equal to 128,


   - profile_idc is equal to 118 and constraint_set5_flag is equal to 1,


   - profile_idc is equal to 100,


   - profile_idc is equal to 77 or constraint_set1_flag is equal to 1.


b) All active MVC sequence parameter sets have one or more of the following conditions fulfilled:


   - level_idc or (level_idc and constraint_set3_flag) represent a level less than or equal to the specific level,


   - level_idc[ i ] or (level_idc[ i ] and constraint_set3_flag) represent a level less than or equal to the specific level.


**H.10.2** **Levels**


The following is specified for expressing the constraints in this clause:


- Let access unit n be the n-th access unit in decoding order with the first access unit being access unit 0.


- Let picture n be the primary coded picture or the corresponding decoded picture of access unit n.


Let the variable fR be derived as follows:


- If the level number is equal to 6.0, 6.1, or 6.2, fR is set equal to 1 ÷ 300.


- Otherwise, if picture n is a frame, fR is set equal to 1 ÷ 172.


- Otherwise (picture n is a field), fR is set equal to 1 ÷ (172 * 2).





The value of mvcScaleFactor is set equal to 2.


The value of NumViews is set equal to applicable_op_num_views_minus1[ i ][ j ] plus 1, which indicates the number of
views required for decoding the target output views corresponding to the j-th operation point for level_idc[ i ] as signalled
in the subset sequence parameter set.


**H.10.2.1Level limits common to Multiview High, Stereo High, and MFC High profiles**


Bitstreams conforming to the Multiview High profile at a specified level shall obey the following constraints:


a) The nominal removal time of access unit n (with n > 0) from the CPB as specified in clause C.1.2, satisfies the

constraint that tr,n( n ) − tr( n − 1 ) is greater than or equal
to Max( NumViews * PicSizeInMbs ÷ ( mvcScaleFactor * MaxMBPS ), fR ), where MaxMBPS is the value
specified in Table A-1 that applies to picture n − 1, and PicSizeInMbs is the number of macroblocks in a single
view component of picture n − 1.


b) The difference between consecutive output times of pictures from the DPB as specified in clause C.2.2, satisfies

the constraint that  to,dpb( n ) >= Max( NumViews * PicSizeInMbs ÷ ( mvcScaleFactor * MaxMBPS ), fR ),
where MaxMBPS is the value specified in Table A-1 for picture n, and PicSizeInMbs is the number of
macroblocks of a single view component of picture n, provided that picture n is a picture that is output and is not
the last picture of the bitstream that is output.


c) PicWidthInMbs * FrameHeightInMbs <= MaxFS, where MaxFS is specified in Table A-1.


d) PicWidthInMbs <= Sqrt( MaxFS * 8 ), where MaxFS is specified in Table A-1.


e) FrameHeightInMbs <= Sqrt( MaxFS * 8 ), where MaxFS is specified in Table A-1.


f) max_dec_frame_buffering <= MaxDpbFrames, where MaxDpbFrames is equal to
Min( mvcScaleFactor * MaxDpbMbs / ( PicWidthInMbs * FrameHeightInMbs ),
Max( 1, Ceil( log2( NumViews ) ) ) * 16 ) and MaxDpbMbs is specified in Table A-1.


g) The vertical motion vector component range for luma motion vectors does not exceed the range from −MaxVmvR

to ( MaxVmvR − 0.25 ) in units of luma frame samples, where MaxVmvR is specified in Table A-1.


h) The horizontal motion vector component range for luma motion vectors does not exceed the following range:


     - If the level number is less than 6.0, the specified range is from −2048 to 2047.75, inclusive, in units of luma
samples.


     - Otherwise, the specified range is from −8192 to 8191.75, inclusive, in units of luma samples.


i) Let setOf2Mb be the set of unsorted pairs of macroblocks that contains the unsorted pairs of macroblocks
(mbA, mbB) of a coded video sequence for which any of the following conditions are true:


     - mbA and mbB are macroblocks that belong to the same slice and are consecutive in decoding order,


     - separate_colour_plane_flag is equal to 0, mbA is the last macroblock (in decoding order) of a slice, and
mbB is the first macroblock (in decoding order) of the next slice in decoding order,


     - separate_colour_plane_flag is equal to 1, mbA is the last macroblock (in decoding order) of a slice with a
particular value of colour_plane_id, and mbB is the first macroblock (in decoding order) of the next slice
with the same value of colour_plane_id in decoding order.


NOTE 1 – In the two above conditions, the macroblocks mbA and mbB can belong to different pictures.


For each unsorted pair of macroblocks (mbA, mbB) of the set setOf2Mb, the total number of motion vectors
(given by the sum of the number of motion vectors for macroblock mbA and the number of motion vectors for
macroblock mbB) does not exceed MaxMvsPer2Mb, where MaxMvsPer2Mb is specified in Table A-1. The
number of motion vectors for each macroblock is the value of the variable MvCnt after the completion of the intra
or inter prediction process for the macroblock.


NOTE 2 – When separate_colour_plane_flag is equal to 0, the constraint specifies that the total number of motion
vectors for two consecutive macroblocks in decoding order must not exceed MaxMvsPer2Mb. When
separate_colour_plane_flag is equal to 1, the constraint specifies that the total number of motion vectors for two
consecutive macroblocks with the same value of colour_plane_id in decoding order must not exceed
MaxMvsPer2Mb. For macroblocks that are consecutive in decoding order but are associated with a different value
of colour_plane_id, no constraint for the total number of motion vectors is specified.


j) The number of bits of macroblock_layer( ) data for any macroblock is not greater than 128 + RawMbBits.
Depending on entropy_coding_mode_flag, the bits of macroblock_layer( ) data are counted as follows:





     - If entropy_coding_mode_flag is equal to 0, the number of bits of macroblock_layer( ) data is given by the
number of bits in the macroblock_layer( ) syntax structure for a macroblock.


     - Otherwise (entropy_coding_mode_flag is equal to 1), the number of bits of macroblock_layer( ) data for a
macroblock is given by the number of times read_bits( 1 ) is called in clauses 9.3.3.2.2 and 9.3.3.2.3 when
parsing the macroblock_layer( ) associated with the macroblock.


k) The removal time of access unit 0 shall satisfy the constraint that the number of slices in picture 0 is less than or

equal to mvcScaleFactor * ( Max( PicSizeInMbs, fR * MaxMBPS ) + MaxMBPS * ( tr( 0 ) − tr,n( 0 ) ) ) ÷
SliceRate, where MaxMBPS and SliceRate are the values specified in Tables A-1 and A-4, respectively, that
apply to picture 0 and PicSizeInMbs is the number of macroblocks in a single view component of picture 0.


l) The removal time of access unit 0 shall satisfy the constraint that the number of slices in each view component of
picture 0 is less than or equal to ( Max( PicSizeInMbs, fR * MaxMBPS ) + MaxMBPS *
( tr( 0 ) − tr,n( 0 ) ) ) ÷ SliceRate, where MaxMBPS and SliceRate are the values specified in Tables A-1 and A-4,
respectively, that apply to picture 0 and PicSizeInMbs is the number of macroblocks in a single view component
of picture 0.


m) The difference between consecutive removal times of access units n and n − 1 with n > 0 shall satisfy the constraint

that the number of slices in picture n is less than or equal to
mvcScaleFactor * MaxMBPS * ( tr( n ) − tr( n − 1 ) ) ÷ SliceRate, where SliceRate is the value specified in
Table A-4 that applies to picture n.


n) The difference between consecutive removal times of access units n and n − 1 with n > 0 shall satisfy the constraint

that the number of slices in each view component of picture n is less than or equal to
MaxMBPS * ( tr( n ) − tr( n − 1 ) ) ÷ SliceRate, where SliceRate is the value specified in Table A-4 that applies to
picture n.


o) MVC sequence parameter sets shall have direct_8x8_inference_flag equal to 1 for the levels specified in

Table A-4.


p) The value of sub_mb_type[ mbPartIdx ] with mbPartIdx = 0..3 in B macroblocks with mb_type equal to B_8x8

shall not be equal to B_Bi_8x4, B_Bi_4x8, or B_Bi_4x4 for the levels in which MinLumaBiPredSize is shown
as 8x8 in Table A-4.


q) For the VCL HRD parameters, BitRate[ SchedSelIdx ] <= cpbBrVclFactor * MaxBR and
CpbSize[ SchedSelIdx ] <= cpbBrVclFactor *MaxCPB for at least one value of SchedSelIdx, where
cpbBrVclFactor is equal to 1250. With vui_mvc_vcl_hrd_parameters_present_flag[ i ] being the syntax element,
in the MVC VUI parameters extension of the active MVC sequence parameter set, that is associated with the VCL
HRD parameters that are used for conformance checking (as specified in Annex C), BitRate[ SchedSelIdx ] and
CpbSize[ SchedSelIdx ] are given as follows:

      - If vui_mvc_vcl_hrd_parameters_present_flag equal to 1, BitRate[ SchedSelIdx ] and
CpbSize[ SchedSelIdx ] are given by Equations E-71 and E-72, respectively, using the syntax elements of
the hrd_parameters( ) syntax structure that immediately follows
vui_mvc_vcl_hrd_parameters_present_flag.

      - Otherwise (vui_mvc_vcl_hrd_parameters_present_flag equal to 0), BitRate[ SchedSelIdx ] and
CpbSize[ SchedSelIdx ] are inferred as specified in clause E.2.2 for VCL HRD parameters.


MaxBR and MaxCPB are specified in Table A-1 in units of cpbBrVclFactor bits/s and cpbBrVclFactor bits,
respectively. The bitstream shall satisfy these conditions for at least one value of SchedSelIdx in the range 0 to
cpb_cnt_minus1, inclusive.


r) For the NAL HRD parameters, BitRate[ SchedSelIdx ] <= cpbBrNalFactor * MaxBR and
CpbSize[ SchedSelIdx ] <= cpbBrNalFactor *MaxCPB for at least one value of SchedSelIdx, where
cpbBrNalFactor is equal to 1500. With vui_mvc_nal_hrd_parameters_present_flag[ i ] being the syntax element,
in the MVC VUI parameters extension of the active MVC sequence parameter set, that is associated with the
NAL HRD parameters that are used for conformance checking (as specified in Annex C), BitRate[ SchedSelIdx ]
and CpbSize[ SchedSelIdx ] are given as follows:

      - If vui_mvc_nal_hrd_parameters_present_flag equal to 1, BitRate[ SchedSelIdx ] and
CpbSize[ SchedSelIdx ] are given by Equations E-71 and E-72, respectively, using the syntax elements of
the hrd_parameters( ) syntax structure that immediately follows
vui_mvc_nal_hrd_parameters_present_flag.

      - Otherwise (vui_mvc_nal_hrd_parameters_present_flag equal to 0), BitRate[ SchedSelIdx ] and
CpbSize[ SchedSelIdx ] are inferred as specified in clause E.2.2 for NAL HRD parameters.





MaxBR and MaxCPB are specified in Table A-1 in units of cpbBrNalFactor bits/s and cpbBrNalFactor bits,
respectively. The bitstream shall satisfy these conditions for at least one value of SchedSelIdx in the range 0 to
cpb_cnt_minus1, inclusive.


s) The sum of the NumBytesInNALunit variables for access unit 0 is less than or equal to
384 * mvcScaleFactor * ( Max( PicSizeInMbs, fR * MaxMBPS ) + MaxMBPS * ( tr( 0 ) − tr,n( 0 ) ) ) ÷ MinCR,
where MaxMBPS and MinCR are the values specified in Table A-1 that apply to picture 0 and PicSizeInMbs is
the number of macroblocks in a single view component of picture 0.


t) The sum of the NumBytesInNALunit variables for the VCL NAL units of each view component of access unit 0
is less than or equal to 384 * ( Max( PicSizeInMbs, fR * MaxMBPS ) + MaxMBPS * ( tr( 0 ) − tr,n( 0 ) ) ) ÷
MinCR, where MaxMBPS and MinCR are the values specified in Table A-1 that apply to picture 0 and
PicSizeInMbs is the number of macroblocks in a single view component of picture 0.


u) The sum of the NumBytesInNALunit variables for access unit n with n > 0 is less than or equal to

384 * mvcScaleFactor * MaxMBPS * ( tr( n ) − tr( n − 1 ) ) ÷ MinCR, where MaxMBPS and MinCR are the
values specified in Table A-1 that apply to picture n.


v) The sum of the NumBytesInNALunit variables for the VCL NAL units of each view component of access unit n

with n > 0 is less than or equal to 384 * MaxMBPS * ( tr( n ) − tr( n − 1 ) ) ÷ MinCR, where MaxMBPS and
MinCR are the values specified in Table A-1 that apply to picture n.


w) When PicSizeInMbs is greater than 1620, the number of macroblocks in any coded slice shall not exceed MaxFS

/ 4, where MaxFS is specified in Table A-1.


x) max_num_ref_frames shall be less than or equal to MaxDpbFrames / mvcScaleFactor for each view component,

where MaxDpbFrames is specified in item f ).


Table A-1 specifies the limits for each level. A definition of all levels identified in the "Level number" column of Table A-1
is specified for the Multiview High, Stereo High, and MFC High profiles. Table A-4 specifies limits for each level that are
specific to bitstreams conforming to the Multiview High, Stereo High, and MFC High profiles. Each entry in Tables A-1
and A-4 indicates, for the level corresponding to the row of the table, the absence or value of a limit that is imposed by the
variable corresponding to the column of the table, as follows:


- If the table entry is marked as "-", no limit is imposed by the value of the variable as a requirement of bitstream
conformance to the profile at the specified level.


- Otherwise, the table entry specifies the value of the variable for the associated limit that is imposed as a requirement
of bitstream conformance to the profile at the specified level.


For coded video sequences conforming to the Multiview High, Stereo High, or MFC High profile, the level_idc value is
specified as follows:


- If level_idc is not equal to 0, level_idc indicates the level that applies to the coded video sequence operating with all
the views being target output views.

NOTE 3 – A level_idc value that is not equal to zero may indicate a higher level than necessary to decode the coded video
sequence operating with all the views being target output views. This may occur when a subset of views or temporal subsets
are removed from a coded video sequence according to the sub-bitstream extraction process specified in clause H.8.5.3, and
the level_idc value is not updated accordingly.


- Otherwise (level_idc is equal to 0), the level that applies to the coded video sequence operating with all the views
being target output views is unspecified.

NOTE 4 – When profile_idc is equal to 118, 128, or 134 and level_idc is equal to 0, there may exist a level indicated by
level_idc[ i ] that is applicable to the coded video sequence operating with all the views being target output views. This may
occur when a subset of views or temporal subsets are removed from a coded video sequence according to the sub-bitstream
extraction process specified in clause H.8.5.3, and a particular value of level_idc[ i ] corresponds to the resulting coded video
sequence.


In bitstreams conforming to the Multiview High, Stereo High, or MFC High profiles, the conformance of the bitstream to
a specified level is indicated by the syntax element level_idc or level_idc[ i ] as follows:


- If level_idc or level_idc[ i ] is equal to 9, the indicated level is level 1b.


- Otherwise (level_idc or level_idc[ i ] is not equal to 9), level_idc or level_idc[ i ] is equal to a value of ten times the
level number (of the indicated level) specified in Table A-1.


**H.10.2.2Profile specific level limits**


a) In bitstreams conforming to the Stereo High or MFC High profile, MVC sequence parameter sets shall have

frame_mbs_only_flag equal to 1 for the levels specified in Table A-4.
