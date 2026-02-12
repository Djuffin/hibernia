**A.3** **Levels**


The following is specified for expressing the constraints in this annex.

- Let access unit n be the n-th access unit in decoding order with the first access unit being access unit 0.

- Let picture n be the primary coded picture or the corresponding decoded picture of access unit n.





Let the variable fR be derived as follows:


- If the level number is equal to 6.0, 6.1, or 6.2, fR is set equal to 1 ÷ 300.


- Otherwise, if picture n is a frame, fR is set equal to 1 ÷ 172.


- Otherwise (picture n is a field), fR is set equal to 1 ÷ (172 * 2).


**A.3.1** **Level limits common to the Baseline, Constrained Baseline, Main, and Extended profiles**


Bitstreams conforming to the Baseline, Constrained Baseline, Main, or Extended profiles at a specified level shall obey
the following constraints:


a) The nominal removal time of access unit n with n > 0 from the CPB as specified in clause C.1.2, satisfies the

constraint that tr,n( n ) − tr( n − 1 ) is greater than or equal to Max( PicSizeInMbs ÷ MaxMBPS, fR ), where
MaxMBPS is the value specified in Table A-1 that applies to picture n − 1 and PicSizeInMbs is the number of
macroblocks in picture n − 1.


b) The difference between consecutive output times of pictures from the DPB as specified in clause C.2.2, satisfies

the constraint that  to,dpb( n ) >= Max( PicSizeInMbs ÷ MaxMBPS, fR ), where MaxMBPS is the value specified
in Table A-1 for picture n and PicSizeInMbs is the number of macroblocks of picture n, provided that picture n is
a picture that is output and is not the last picture of the bitstream that is output.


c) The sum of the NumBytesInNALunit variables for access unit 0 is less than or equal to

384 *( Max( PicSizeInMbs, fR * MaxMBPS ) + MaxMBPS * ( tr( 0 ) − tr,n( 0 ) ) ) ÷ MinCR, where MaxMBPS
and MinCR are the values specified in Table A-1 that apply to picture 0 and PicSizeInMbs is the number of
macroblocks in picture 0.


d) The sum of the NumBytesInNALunit variables for access unit n with n > 0 is less than or equal to

384 * MaxMBPS * ( tr( n ) − tr( n − 1 ) ) ÷ MinCR, where MaxMBPS and MinCR are the values specified in
Table A-1 that apply to picture n.


e) PicWidthInMbs * FrameHeightInMbs <= MaxFS, where MaxFS is specified in Table A-1


f) PicWidthInMbs <= Sqrt( MaxFS * 8 )


g) FrameHeightInMbs <= Sqrt( MaxFS * 8 )


h) max_dec_frame_buffering <= MaxDpbFrames, where MaxDpbFrames is equal to
Min( MaxDpbMbs / ( PicWidthInMbs * FrameHeightInMbs ), 16 ) and MaxDpbMbs is given in Table A-1.


i) For the VCL HRD parameters, BitRate[ SchedSelIdx ] <= 1000 * MaxBR and CpbSize[ SchedSelIdx ] <= 1000

     - MaxCPB for at least one value of SchedSelIdx, where BitRate[ SchedSelIdx ] and CpbSize[ SchedSelIdx ] are
given as follows:


     - If vcl_hrd_parameters_present_flag is equal to 1, BitRate[ SchedSelIdx ] and CpbSize[ SchedSelIdx ] are
given by Equations E-71 and E-72, respectively, using the syntax elements of the hrd_parameters( ) syntax
structure that immediately follows vcl_hrd_parameters_present_flag.


     - Otherwise (vcl_hrd_parameters_present_flag is equal to 0), BitRate[ SchedSelIdx ] and
CpbSize[ SchedSelIdx ] are inferred as specified in clause E.2.2 for VCL HRD parameters.


MaxBR and MaxCPB are specified in Table A-1 in units of 1000 bits/s and 1000 bits, respectively. The bitstream
shall satisfy these conditions for at least one value of SchedSelIdx in the range 0 to cpb_cnt_minus1, inclusive.


j) For the NAL HRD parameters, BitRate[ SchedSelIdx ] <= 1200 * MaxBR and CpbSize[ SchedSelIdx ] <= 1200

     - MaxCPB for at least one value of SchedSelIdx, where BitRate[ SchedSelIdx ] and CpbSize[ SchedSelIdx ] are
given as follows:


     - If nal_hrd_parameters_present_flag is equal to 1, BitRate[ SchedSelIdx ] and CpbSize[ SchedSelIdx ] are
given by Equations E-71 and E-72, respectively, using the syntax elements of the hrd_parameters( ) syntax
structure that immediately follows nal_hrd_parameters_present_flag.


     - Otherwise (nal_hrd_parameters_present_flag is equal to 0), BitRate[ SchedSelIdx ] and
CpbSize[ SchedSelIdx ] are inferred as specified in clause E.2.2 for NAL HRD parameters.


MaxBR and MaxCPB are specified in Table A-1 in units of 1200 bits/s and 1200 bits, respectively. The bitstream
shall satisfy these conditions for at least one value of SchedSelIdx in the range 0 to cpb_cnt_minus1.


k) The vertical motion vector component range for luma motion vectors does not exceed the range from −MaxVmvR

to ( MaxVmvR − 0.25 ) in units of luma frame samples, where MaxVmvR is specified in Table A-1





NOTE 1 – When chroma_format_idc is equal to 1 and the current macroblock is a field macroblock, the motion vector
component range for chroma motion vectors may exceed the range from −MaxVmvR to ( MaxVmvR − 0.25 ) in units of
luma frame samples, due to the method of deriving chroma motion vectors as specified in clause 8.4.1.4.


l) The horizontal motion vector component range for luma motion vectors does not exceed the following range:


     - If the level number is less than 6.0, the specified range is from −2048 to 2047.75, inclusive, in units of luma
samples.


     - Otherwise, the specified range is from −8192 to 8191.75, inclusive, in units of luma samples.


m) Let setOf2Mb be the set of unsorted pairs of macroblocks that contains the unsorted pairs of macroblocks

(mbA, mbB) of a coded video sequence for which any of the following conditions are true:


     - mbA and mbB are macroblocks that belong to the same slice and are consecutive in decoding order,


     - arbitrary slice order is not allowed, mbA is the last macroblock (in decoding order) of a slice, and mbB is
the first macroblock (in decoding order) of the next slice in decoding order,

NOTE 2 – The macroblocks mbA and mbB can belong to different pictures.


     - arbitrary slice order is allowed, mbA is the last macroblock (in decoding order) of a slice of a particular
picture, and mbB is the first macroblock (in decoding order) of any other slice of the same picture,


     - arbitrary slice order is allowed, mbA is the last macroblock (in decoding order) of a slice of a particular
picture, and mbB is the first macroblock (in decoding order) of any slice of the next picture in decoding
order.


For each unsorted pair of macroblocks (mbA, mbB) of the set setOf2Mb, the total number of motion vectors
(given by the sum of the number of motion vectors for macroblock mbA and the number of motion vectors for
macroblock mbB) does not exceed MaxMvsPer2Mb, where MaxMvsPer2Mb is specified in Table A-1. The
number of motion vectors for each macroblock is the value of the variable MvCnt after the completion of the intra
or inter prediction process for the macroblock.

NOTE 3 – The constraint specifies that the total number of motion vectors for two consecutive macroblocks in
decoding order must not exceed MaxMvsPer2Mb. When arbitrary slice order is allowed, it is specified that this
constraint must also be obeyed when slices of a picture are reordered, e.g., during transmission.


n) The number of bits of macroblock_layer( ) data for any macroblock is not greater than 3200. Depending on

entropy_coding_mode_flag, the bits of macroblock_layer( ) data are counted as follows:


     - If entropy_coding_mode_flag is equal to 0, the number of bits of macroblock_layer( ) data is given by the
number of bits in the macroblock_layer( ) syntax structure for a macroblock.


     - Otherwise (entropy_coding_mode_flag is equal to 1), the number of bits of macroblock_layer( ) data for a
macroblock is given by the number of times read_bits( 1 ) is called in clauses 9.3.3.2.2 and 9.3.3.2.3 when
parsing the macroblock_layer( ) associated with the macroblock.


Table A-1 specifies the limits for each level. A definition of all levels identified in the "Level number" column of Table A-1
is specified for the Baseline, Constrained Baseline, Main, and Extended profiles. Each entry in Table A-1 indicates, for the
level corresponding to the row of the table, the absence or value of a limit that is imposed by the variable corresponding
to the column of the table, as follows:


- If the table entry is marked as "-", no limit is imposed by the value of the variable as a requirement of bitstream
conformance to the profile at the specified level.


- Otherwise, the table entry specifies the value of the variable for the associated limit that is imposed as a requirement
of bitstream conformance to the profile at the specified level.


For purposes of comparison of level capabilities, a level shall be considered to be a lower (higher) level than some other
level if the level appears nearer to the top (bottom) row of Table A-1 than the other level.


In bitstreams conforming to the Baseline, Constrained Baseline, Main, or Extended profiles, the conformance of the
bitstream to a specified level is indicated by the syntax elements level_idc and constraint_set3_flag as follows:


- If level_idc is equal to 11 and constraint_set3_flag is equal to 1, the indicated level is level 1b.


- Otherwise (level_idc is not equal to 11 or constraint_set3_flag is not equal to 1), level_idc is equal to a value of ten
times the level number (of the indicated level) specified in Table A-1.





**Table A-1 – Level limits**





















|Level<br>number|Max<br>macroblock<br>processing rate<br>MaxMBPS<br>(MB/s)|Max<br>frame size<br>MaxFS<br>(MBs)|Max decoded<br>picture buffer<br>size<br>MaxDpbMbs<br>(MBs)|Max<br>video<br>bit rate MaxBR<br>(1000 bits/s,<br>1200 bits/s,<br>cpbBrVclFactor<br>bits/s, or<br>cpbBrNalFactor<br>bits/s)|Max<br>CPB size<br>MaxCPB<br>(1000 bits,<br>1200 bits,<br>cpbBrVclFactor<br>bits, or<br>cpbBrNalFactor<br>bits)|Vertical MV<br>component limit<br>MaxVmvR<br>(luma frame<br>samples)|Min<br>compression<br>ratio MinCR|Max number of<br>motion vectors<br>per two<br>consecutive MBs<br>MaxMvsPer2Mb|
|---|---|---|---|---|---|---|---|---|
|**1 **|1 485|99|396|64|175|64|2|-|
|**1b**|1 485|99|396|128|350|64|2|-|
|**1.1**|3 000|396|900|192|500|128|2|-|
|**1.2**|6 000|396|2 376|384|1 000|128|2|-|
|**1.3**|11 880|396|2 376|768|2 000|128|2|-|
|**2 **|11 880|396|2 376|2 000|2 000|128|2|-|
|**2.1**|19 800|792|4 752|4 000|4 000|256|2|-|
|**2.2**|20 250|1 620|8 100|4 000|4 000|256|2|-|
|**3 **|40 500|1 620|8 100|10 000|10 000|256|2|32|
|**3.1**|108 000|3 600|18 000|14 000|14 000|512|4|16|
|**3.2**|216 000|5 120|20 480|20 000|20 000|512|4|16|
|**4 **|245 760|8 192|32 768|20 000|25 000|512|4|16|
|**4.1**|245 760|8 192|32 768|50 000|62 500|512|2|16|
|**4.2**|522 240|8 704|34 816|50 000|62 500|512|2|16|
|**5 **|589 824|22 080|110 400|135 000|135 000|512|2|16|
|**5.1**|983 040|36 864|184 320|240 000|240 000|512|2|16|
|**5.2**|2 073 600|36 864|184 320|240 000|240 000|512|2|16|
|**6 **|4 177 920|139 264|696 320|240 000|240 000|8 192|2|16|
|**6.1**|8 355 840|139 264|696 320|480 000|480 000|8 192|2|16|
|**6.2**|16 711 680|139 264|696 320|800 000|800 000|8 192|2|16|


Levels with non-integer level numbers in Table A-1 are referred to as "intermediate levels".

NOTE 4 – All levels have the same status, but some applications may choose to use only the integer-numbered levels.


Informative clause A.3.4 shows the effect of these limits on frame rates for several example picture formats.


**A.3.2** **Level limits common to the High, Progressive High, Constrained High, High 10, Progressive High 10,**
**High 4:2:2,** **High 4:4:4 Predictive,** **High 10 Intra,** **High 4:2:2 Intra,** **High 4:4:4 Intra,** **and**
**CAVLC 4:4:4 Intra profiles**


Bitstreams conforming to the High, Progressive High, Constrained High, High 10, Progressive High 10, High 4:2:2,
High 4:4:4 Predictive, High 10 Intra, High 4:2:2 Intra, High 4:4:4 Intra, or CAVLC 4:4:4 Intra profiles at a specified level
shall obey the following constraints:


a) The nominal removal time of access unit n (with n > 0) from the CPB as specified in clause C.1.2, satisfies the

constraint that tr,n( n ) − tr( n − 1 ) is greater than or equal to Max( PicSizeInMbs ÷ MaxMBPS, fR ), where
MaxMBPS is the value specified in Table A-1 that applies to picture n − 1, and PicSizeInMbs is the number of
macroblocks in picture n − 1.


b) The difference between consecutive output times of pictures from the DPB as specified in clause C.2.2, satisfies

the constraint that  to,dpb( n ) >= Max( PicSizeInMbs ÷ MaxMBPS, fR ), where MaxMBPS is the value specified
in Table A-1 for picture n, and PicSizeInMbs is the number of macroblocks of picture n, provided that picture n
is a picture that is output and is not the last picture of the bitstream that is output.


c) PicWidthInMbs * FrameHeightInMbs <= MaxFS, where MaxFS is specified in Table A-1





d) PicWidthInMbs <= Sqrt( MaxFS * 8 )


e) FrameHeightInMbs <= Sqrt( MaxFS * 8 )


f) max_dec_frame_buffering <= MaxDpbFrames, where MaxDpbFrames is equal to
Min( MaxDpbMbs / ( PicWidthInMbs * FrameHeightInMbs ), 16 ) and MaxDpbMbs is specified in Table A-1.


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
consecutive macroblocks (in decoding order) with the same value of colour_plane_id must not exceed
MaxMvsPer2Mb. For macroblocks that are consecutive in decoding order but are associated with a different value
of colour_plane_id, no constraint for the total number of motion vectors is specified.


j) The number of bits of macroblock_layer( ) data for any macroblock is not greater than 128 + RawMbBits.
Depending on entropy_coding_mode_flag, the bits of macroblock_layer( ) data are counted as follows:


     - If entropy_coding_mode_flag is equal to 0, the number of bits of macroblock_layer( ) data is given by the
number of bits in the macroblock_layer( ) syntax structure for a macroblock.


     - Otherwise (entropy_coding_mode_flag is equal to 1), the number of bits of macroblock_layer( ) data for a
macroblock is given by the number of times read_bits( 1 ) is called in clauses 9.3.3.2.2 and 9.3.3.2.3 when
parsing the macroblock_layer( ) associated with the macroblock.

Table A-1 specifies the limits for each level. A definition of all levels identified in the "Level number" column of Table A-1
is specified for the High, Progressive High, Constrained High, High 10, Progressive High 10, High 4:2:2,
High 4:4:4 Predictive, High 10 Intra, High 4:2:2 Intra, High 4:4:4 Intra, and CAVLC 4:4:4 Intra profiles. Each entry in
Table A-1 indicates, for the level corresponding to the row of the table, the absence or value of a limit that is imposed by
the variable corresponding to the column of the table, as follows:


- If the table entry is marked as "-", no limit is imposed by the value of the variable as a requirement of bitstream
conformance to the profile at the specified level.


- Otherwise, the table entry specifies the value of the variable for the associated limit that is imposed as a requirement
of bitstream conformance to the profile at the specified level.


The use of the MinCR parameter column of Table A-1 for the High, High 10, High 4:2:2, High 4:4:4 Predictive,
High 10 Intra, High 4:2:2 Intra, High 4:4:4 Intra, and CAVLC 4:4:4 Intra profiles is specified in clause A.3.3.





In bitstreams conforming to the High, High 10, High 4:2:2, High 4:4:4 Predictive, High 10 Intra, High 4:2:2 Intra,
High 4:4:4 Intra, or CAVLC 4:4:4 Intra profiles, the conformance of the bitstream to a specified level is indicated by the
syntax element level_idc as follows:

- If level_idc is equal to 9, the indicated level is level 1b.

- Otherwise (level_idc is not equal to 9), level_idc is equal to a value of ten times the level number (of the indicated
level) specified in Table A-1.


**A.3.3** **Profile-specific level limits**


a) In bitstreams conforming to the Main, High, Progressive High, Constrained High, High 10, Progressive High 10,

High 4:2:2, High 4:4:4 Predictive, High 10 Intra, High 4:2:2 Intra, High 4:4:4 Intra, or CAVLC 4:4:4 Intra
profiles, the removal time of access unit 0 shall satisfy the constraint that the number of slices in picture 0 is less
than or equal to ( Max( PicSizeInMbs, fR * MaxMBPS ) + MaxMBPS * ( tr( 0 ) − tr,n( 0 ) ) ) ÷ SliceRate, where
MaxMBPS and SliceRate are the values specified in Tables A-1 and A-4, respectively, that apply to picture 0 and
PicSizeInMbs is the number of macroblocks in picture 0.


b) In bitstreams conforming to the Main, High, Progressive High, Constrained High, High 10, Progressive High 10,

High 4:2:2, High 4:4:4 Predictive, High 10 Intra, High 4:2:2 Intra, High 4:4:4 Intra, or CAVLC 4:4:4 Intra
profiles, the difference between consecutive removal times of access units n and n − 1 with n > 0 shall satisfy the
constraint that the number of slices in picture n is less than or equal
to MaxMBPS * ( tr( n ) − tr( n − 1 ) ) ÷ SliceRate, where MaxMBPS and SliceRate are the values specified in
Tables A-1 and A-4, respectively, that apply to picture n.


c) In bitstreams conforming to the Main, High, Progressive High, High 10, Progressive High 10, High 4:2:2,

High 4:4:4 Predictive profiles, sequence parameter sets shall have direct_8x8_inference_flag equal to 1 for the
levels specified in Table A-4.

NOTE 1 – direct_8x8_inference_flag is not relevant to the Baseline, Constrained Baseline, Constrained High,
High 10 Intra, High 4:2:2 Intra, High 4:4:4 Intra, and CAVLC 4:4:4 Intra profiles as these profiles do not allow B slice
types, and direct_8x8_inference_flag is equal to 1 for all levels of the Extended profile.


d) In bitstreams conforming to the Main, High, High 10, High 4:2:2, High 4:4:4 Predictive, High 10 Intra,

High 4:2:2 Intra, High 4:4:4 Intra, CAVLC 4:4:4 Intra, or Extended profiles, sequence parameter sets shall have
frame_mbs_only_flag equal to 1 for the levels specified in Table A-4 for the Main, High, High 10, High 4:2:2,
High 4:4:4 Predictive, High 10 Intra, High 4:2:2 Intra, High 4:4:4 Intra, or CAVLC 4:4:4 Intra profiles and in
Table A-5 for the Extended profile.

NOTE 2 – frame_mbs_only_flag is equal to 1 for all levels of the Baseline, Constrained Baseline, Progressive High,
Constrained High, and Progressive High 10 profiles (specified in clauses A.2.1, A.2.1.1, A.2.4.1, A.2.4.2, and A.2.5.1,
respectively).


e) In bitstreams conforming to the Main, High, Progressive High, High 10, Progressive High 10, High 4:2:2,

High 4:4:4 Predictive, or Extended profiles, the value of sub_mb_type[ mbPartIdx ] with mbPartIdx = 0..3 in B
macroblocks with mb_type equal to B_8x8 shall not be equal to B_Bi_8x4, B_Bi_4x8, or B_Bi_4x4 for the levels
in which MinLumaBiPredSize is shown as 8x8 in Table A-4 for the Main, High, Progressive High, High 10,
Progressive High 10, High 4:2:2, High 4:4:4 Predictive profiles and in Table A-5 for the Extended profile.


f) In bitstreams conforming to the Baseline, Constrained Baseline, or Extended profiles, ( xIntmax − xIntmin + 6 ) *
( yIntmax − yIntmin + 6 ) <= MaxSubMbRectSize in macroblocks coded with mb_type equal to P_8x8, P_8x8ref0
or B_8x8 for all invocations of the process specified in clause 8.4.2.2.1 used to generate the predicted luma sample
array for a single reference picture list (reference picture list 0 or reference picture list 1) for each 8x8 submacroblock with the macroblock partition index mbPartIdx, where
NumSubMbPart( sub_mb_type[ mbPartIdx ] ) > 1, where MaxSubMbRectSize is specified in Table A-3 for the
Baseline and Constrained Baseline profiles and in Table A-5 for the Extended profile and

      - xIntmin is the minimum value of xIntL among all luma sample predictions for the sub-macroblock

      - xIntmax is the maximum value of xIntL among all luma sample predictions for the sub-macroblock

      - yIntmin is the minimum value of yIntL among all luma sample predictions for the sub-macroblock

      - yIntmax is the maximum value of yIntL among all luma sample predictions for the sub-macroblock


g) In bitstreams conforming to the High, Progressive High, Constrained High, High 10, Progressive High 10,

High 4:2:2, High 4:4:4 Predictive, High 10 Intra, High 4:2:2 Intra, High 4:4:4 Intra, or CAVLC 4:4:4 Intra
profiles, for the VCL HRD parameters, BitRate[ SchedSelIdx ] <= cpbBrVclFactor * MaxBR and
CpbSize[ SchedSelIdx ] <= cpbBrVclFactor * MaxCPB for at least one value of SchedSelIdx, where
cpbBrVclFactor is specified in Table A-2 and BitRate[ SchedSelIdx ] and CpbSize[ SchedSelIdx ] are given as
follows:





   - If vcl_hrd_parameters_present_flag is equal to 1, BitRate[ SchedSelIdx ] and CpbSize[ SchedSelIdx ] are
given by Equations E-71 and E-72, respectively, using the syntax elements of the hrd_parameters( ) syntax
structure that immediately follows vcl_hrd_parameters_present_flag.

   - Otherwise (vcl_hrd_parameters_present_flag is equal to 0), BitRate[ SchedSelIdx ] and
CpbSize[ SchedSelIdx ] are inferred as specified in clause E.2.2 for VCL HRD parameters.


MaxBR and MaxCPB are specified in Table A-1 in units of cpbBrVclFactor bits/s and cpbBrVclFactor bits,
respectively. The bitstream shall satisfy these conditions for at least one value of SchedSelIdx in the range 0 to
cpb_cnt_minus1, inclusive.


h) In bitstreams conforming to the High, Progressive High, Constrained High, High 10, Progressive High 10,

High 4:2:2, High 4:4:4 Predictive, High 10 Intra, High 4:2:2 Intra, High 4:4:4 Intra, or CAVLC 4:4:4 Intra
profiles, for the NAL HRD parameters, BitRate[ SchedSelIdx ] <= cpbBrNalFactor * MaxBR and
CpbSize[ SchedSelIdx ] <= cpbBrNalFactor * MaxCPB for at least one value of SchedSelIdx, where
cpbBrNalFactor is specified in Table A-2 and BitRate[ SchedSelIdx ] and CpbSize[ SchedSelIdx ] are given as
follows:

   - If nal_hrd_parameters_present_flag is equal to 1, BitRate[ SchedSelIdx ] and CpbSize[ SchedSelIdx ] are
given by Equations E-71 and E-72, respectively, using the syntax elements of the hrd_parameters( ) syntax
structure that immediately follows nal_hrd_parameters_present_flag.

   - Otherwise (nal_hrd_parameters_present_flag is equal to 0), BitRate[ SchedSelIdx ] and
CpbSize[ SchedSelIdx ] are inferred as specified in clause E.2.2 for NAL HRD parameters.


MaxBR and MaxCPB are specified in Table A-1 in units of cpbBrNalFactor bits/s and cpbBrNalFactor bits,
respectively. The bitstream shall satisfy these conditions for at least one value of SchedSelIdx in the range 0 to
cpb_cnt_minus1, inclusive.


i) In bitstreams conforming to the High, Progressive High, or Constrained High profiles, the sum of the
NumBytesInNALunit variables for access unit 0 is less than or equal to
384 * ( Max( PicSizeInMbs, fR * MaxMBPS ) + MaxMBPS * ( tr( 0 ) − tr,n( 0 ) ) ) ÷ MinCR, where MaxMBPS
and MinCR are the values specified in Table A-1 that apply to picture 0 and PicSizeInMbs is the number of
macroblocks in picture 0.

NOTE 3 – Such a limit involving MinCR is not imposed for bitstream conformance to the High 10,
Progressive High 10, High 4:2:2, High 4:4:4 Predictive, High 10 Intra, High 4:2:2 Intra, High 4:4:4 Intra, and
CAVLC 4:4:4 Intra profiles.


j) In bitstreams conforming to the High, Progressive High, or Constrained High profiles, the sum of the
NumBytesInNALunit variables for access unit n with n   - 0 is less than or equal to
384 * MaxMBPS * ( tr( n ) − tr( n − 1 ) ) ÷ MinCR, where MaxMBPS and MinCR are the values specified in
Table A-1 that apply to picture n.

NOTE 4 – Such a limit involving MinCR is not imposed for bitstream conformance to the High 10,
Progressive High 10, High 4:2:2, High 4:4:4 Predictive, High 10 Intra, High 4:2:2 Intra, High 4:4:4 Intra, and
CAVLC 4:4:4 Intra profiles.


k) In bitstreams conforming to the High 10, Progressive High 10, High 4:2:2, High 4:4:4 Predictive, High 10 Intra,

High 4:2:2 Intra, High 4:4:4 Intra, or CAVLC 4:4:4 Intra profiles, when the level number is less than 6.0 and
PicSizeInMbs is greater than 1620, the number of macroblocks in any coded slice shall not exceed MaxFS / 4,
where MaxFS is specified in Table A-1.


**Table A-2 – Specification of cpbBrVclFactor**

**and cpbBrNalFactor**











|Profile|cpbBrVclFactor|cpbBrNalFactor|
|---|---|---|
|**High**<br>**Progressive High**<br>**Constrained High**|1 250|1 500|
|**High 10**<br>**Progressive High 10**<br>**High 10 Intra**|3 000|3 600|
|**High 4:2:2**<br>**High 4:2:2 Intra**|4 000|4 800|
|**High 4:4:4 Predictive**<br>**High 4:4:4 Intra**<br>**CAVLC 4:4:4 Intra**|4 000|4 800|





**A.3.3.1** **Level limits of the Baseline and Constrained Baseline profile**


Table A-3 specifies limits for each level that are specific to bitstreams conforming to the Baseline or Constrained Baseline
profiles. Each entry in Table A-3 indicates, for the level corresponding to the row of the table, the absence or value of a
limit that is imposed by the variable corresponding to the column of the table, as follows:


- If the table entry is marked as "-", no limit is imposed by the value of the variable as a requirement of bitstream
conformance to the profile at the specified level.


- Otherwise, the table entry specifies the value of the variable for the associated limit that is imposed as a requirement
of bitstream conformance to the profile at the specified level.


**Table A-3 – Baseline and Constrained**

**Baseline profile level limits**

|Level number|MaxSubMbRectSize|
|---|---|
|**1 **|576|
|**1b**|576|
|**1.1**|576|
|**1.2**|576|
|**1.3**|576|
|**2 **|576|
|**2.1**|576|
|**2.2**|576|
|**3 **|576|
|**3.1**|-|
|**3.2**|-|
|**4 **|-|
|**4.1**|-|
|**4.2**|-|
|**5 **|-|
|**5.1**|-|
|**5.2**|-|
|**6.0**|-|
|**6.1**|-|
|**6.2**|-|



**A.3.3.2** **Level limits of the Main, High, Progressive High, Constrained High, High 10, Progressive High 10,**

**High 4:2:2, High 4:4:4 Predictive, High 10 Intra, High 4:2:2 Intra, High 4:4:4 Intra, and**
**CAVLC 4:4:4 Intra profile**


Table A-4 specifies limits for each level that are specific to bitstreams conforming to the Main, High, Progressive High,
Constrained High, High 10, Progressive High 10, High 4:2:2, High 4:4:4 Predictive, High 10 Intra, High 4:2:2 Intra,
High 4:4:4 Intra, or CAVLC 4:4:4 Intra profiles. Each entry in Table A-4 indicates, for the level corresponding to the row
of the table, the absence or value of a limit that is imposed by the variable corresponding to the column of the table, as
follows:


- If the table entry is marked as "-", no limit is imposed by the value of the variable as a requirement of bitstream
conformance to the profile at the specified level.


- Otherwise, the table entry specifies the value of the variable for the associated limit that is imposed as a requirement
of bitstream conformance to the profile at the specified level.





NOTE – The constraints for MinLumaBiPredSize and direct_8x8_inference_flag are not relevant to the
Constrained High, High 10 Intra, High 4:2:2 Intra, High 4:4:4 Intra, and CAVLC 4:4:4 Intra profiles, as these profiles
do not support B slices.


**Table A-4 – Main, High, Progressive High, Constrained High, High 10, Progressive High 10, High 4:2:2,**
**High 4:4:4 Predictive, High 10 Intra, High 4:2:2 Intra, High 4:4:4 Intra, and CAVLC 4:4:4 Intra profile level**

**limits**

|Level number|SliceRate|MinLumaBiPredSize|direct_8x8_inference_flag|frame_mbs_only_flag|
|---|---|---|---|---|
|**1 **|-|-|-|1|
|**1b**|-|-|-|1|
|**1.1**|-|-|-|1|
|**1.2**|-|-|-|1|
|**1.3**|-|-|-|1|
|**2 **|-|-|-|1|
|**2.1**|-|-|-|-|
|**2.2**|-|-|-|-|
|**3 **|22|-|1|-|
|**3.1**|60|8x8|1|-|
|**3.2**|60|8x8|1|-|
|**4 **|60|8x8|1|-|
|**4.1**|24|8x8|1|-|
|**4.2**|24|8x8|1|1|
|**5 **|24|8x8|1|1|
|**5.1**|24|8x8|1|1|
|**5.2**|24|8x8|1|1|
|**6 **|24|8x8|1|1|
|**6.1**|24|8x8|1|1|
|**6.2**|24|8x8|1|1|



**A.3.3.3** **Level limits of the Extended profile**


Table A-5 specifies limits for each level that are specific to bitstreams conforming to the Extended profile. Each entry in
Table A-5 indicates, for the level corresponding to the row of the table, the absence or value of a limit that is imposed by
the variable corresponding to the column of the table, as follows:


- If the table entry is marked as "-", no limit is imposed by the value of the variable as a requirement of bitstream
conformance to the profile at the specified level.


- Otherwise, the table entry specifies the value of the variable for the associated limit that is imposed as a requirement
of bitstream conformance to the profile at the specified level.





**Table A-5 – Extended profile level limits**

|Level number|MaxSubMbRectSize|MinLumaBiPredSize|frame_mbs_only_flag|
|---|---|---|---|
|**1 **|576|-|1|
|**1b**|576|-|1|
|**1.1**|576|-|1|
|**1.2**|576|-|1|
|**1.3**|576|-|1|
|**2 **|576|-|1|
|**2.1**|576|-|-|
|**2.2**|576|-|-|
|**3 **|576|-|-|
|**3.1**|-|8x8|-|
|**3.2**|-|8x8|-|
|**4 **|-|8x8|-|
|**4.1**|-|8x8|-|
|**4.2**|-|8x8|1|
|**5 **|-|8x8|1|
|**5.1**|-|8x8|1|
|**5.2**|-|8x8|1|
|**6 **|-|8x8|1|
|**6.1**|-|8x8|1|
|**6.2**|-|8x8|1|






**A.3.4** **Effect of level limits on frame rate (informative)**


This clause does not form an integral part of this Recommendation | International Standard.


**Table A-6 – Maximum frame rates (frames per second) for some example frame sizes**













|Level:|Col2|Col3|Col4|Col5|1|1b|1.1|1.2|1.3|2|2.1|
|---|---|---|---|---|---|---|---|---|---|---|---|
|**Max frame size (macroblocks):**|||||**99**|**99**|**396**|**396**|**396**|**396**|**792**|
|**Max macroblocks/second:**|||||**1 485**|**1 485**|**3 000**|**6 000**|**11 880**|**11 880**|**19 800**|
|||||||||||||
|**Max frame size (samples):**|||||25 344|25 344|101 376|101 376|101 376|101 376|202 752|
|**Max samples/second:**|||||380 160|380 160|768 000|1 536 000|3 041 280|3 041 280|5 068 800|
|**Format**|**Luma**<br>**Width**|**Luma**<br>**Height**|**MBs**<br>**Total**|**Luma**<br>**Samples**||||||||
|**SQCIF**|**128**|**96**|48|12 288|30.9|30.9|62.5|125.0|172.0|172.0|172.0|
|**QCIF**|**176**|**144**|99|25 344|15.0|15.0|30.3|60.6|120.0|120.0|172.0|
|**QVGA**|**320**|**240**|300|76 800|-|-|10.0|20.0|39.6|39.6|66.0|
|**525 SIF**|**352**|**240**|330|84 480|-|-|9.1|18.2|36.0|36.0|60.0|
|**CIF**|**352**|**288**|396|101 376|-|-|7.6|15.2|30.0|30.0|50.0|
|**525 HHR**|**352**|**480**|660|168 960|-|-|-|-|-|-|30.0|
|**625 HHR**|**352**|**576**|792|202 752|-|-|-|-|-|-|25.0|
|**VGA**|**640**|**480**|1 200|307 200|-|-|-|-|-|-|-|
|**525 4SIF**|**704**|**480**|1 320|337 920|-|-|-|-|-|-|-|
|**525 SD**|**720**|**480**|1 350|345 600|-|-|-|-|-|-|-|
|**4CIF**|**704**|**576**|1 584|405 504|-|-|-|-|-|-|-|
|**625 SD**|**720**|**576**|1 620|414 720|-|-|-|-|-|-|-|
|**SVGA**|**800**|**600**|1 900|486 400|-|-|-|-|-|-|-|
|**XGA**|**1024**|**768**|3 072|786 432|-|-|-|-|-|-|-|
|**720p HD**|**1280**|**720**|3 600|921 600|-|-|-|-|-|-|-|
|**4VGA**|**1280**|**960**|4 800|1 228 800|-|-|-|-|-|-|-|
|**SXGA**|**1280**|**1024**|5 120|1 310 720|-|-|-|-|-|-|-|
|**525 16SIF**|**1408**|**960**|5 280|1 351 680|-|-|-|-|-|-|-|
|**16CIF**|**1408**|**1152**|6 336|1 622 016|-|-|-|-|-|-|-|
|**4SVGA**|**1600**|**1200**|7 500|1 920 000|-|-|-|-|-|-|-|
|**1080 HD**|**1920**|**1088**|8 160|2 088 960|-|-|-|-|-|-|-|
|**2Kx1K**|**2048**|**1024**|8 192|2 097 152|-|-|-|-|-|-|-|
|**2Kx1080**|**2048**|**1088**|8 704|2 228 224|-|-|-|-|-|-|-|
|**4XGA**|**2048**|**1536**|12 288|3 145 728|-|-|-|-|-|-|-|
|**16VGA**|**2560**|**1920**|19 200|4 915 200|-|-|-|-|-|-|-|
|**3616x1536 (2.35:1)**|**3616**|**1536**|21 696|5 554 176|-|-|-|-|-|-|-|
|**3672x1536 (2.39:1)**|**3680**|**1536**|22 080|5 652 480|-|-|-|-|-|-|-|
|**3840x2160**|**3840**|**2160**|31 035|7 948 800|-|-|-|-|-|-|-|
|**4Kx2K**|**4096**|**2048**|32 768|8 388 608|-|-|-|-|-|-|-|
|**4096x2160**|**4096**|**2160**|34 560|8 847 360|-|-|-|-|-|-|-|
|**4096x2304 (16:9)**|**4096**|**2304**|36 864|9 437 184|-|-|-|-|-|-|-|
|**7680x4320**|**7680**|**4320**|129 600|33 177 600|-|-|-|-|-|-|-|
|**8192x4096**|**8192**|**4096**|131 072|33 554 432|-|-|-|-|-|-|-|
|**8192x4320**|**8192**|**4320**|138 240|35 389 440|-|-|-|-|-|-|-|





**Table A-6 (continued) – Maximum frame rates (frames per second) for some example frame sizes**













|Level:|Col2|Col3|Col4|Col5|2.2|3|3.1|3.2|4|4.1|4.2|
|---|---|---|---|---|---|---|---|---|---|---|---|
|**Max frame size (macroblocks):**|||||**1 620**|**1 620**|**3 600**|**5 120**|**8 192**|**8 192**|**8 704**|
|**Max macroblocks/second:**|||||**20 250**|**40 500**|**108 000**|**216 000**|**245 760**|**245 760**|**522 240**|
|||||||||||||
|**Max frame size (samples):**|||||414 720|414 720|921 600|1 310 720|2 097 152|2 097 152|2 228 224|
|**Max samples/second:**|||||5 184 000|10 368 000|27 648 000|55 296 000|62 914 560|62 914 560|133 693 440|
|**Format**|**Luma**<br>**Width**|**Luma**<br>**Height**|**MBs**<br>**Total**|**Luma**<br>**Samples**||||||||
|**SQCIF**|**128**|**96**|48|12 288|172.0|172.0|172.0|172.0|172.0|172.0|172.0|
|**QCIF**|**176**|**144**|99|25 344|172.0|172.0|172.0|172.0|172.0|172.0|172.0|
|**QVGA**|**320**|**240**|300|76 800|67.5|135.0|172.0|172.0|172.0|172.0|172.0|
|**525 SIF**|**352**|**240**|330|84 480|61.4|122.7|172.0|172.0|172.0|172.0|172.0|
|**CIF**|**352**|**288**|396|101 376|51.1|102.3|172.0|172.0|172.0|172.0|172.0|
|**525 HHR**|**352**|**480**|660|168 960|30.7|61.4|163.6|172.0|172.0|172.0|172.0|
|**625 HHR**|**352**|**576**|792|202 752|25.6|51.1|136.4|172.0|172.0|172.0|172.0|
|**VGA**|**640**|**480**|1 200|307 200|16.9|33.8|90.0|172.0|172.0|172.0|172.0|
|**525 4SIF**|**704**|**480**|1 320|337 920|15.3|30.7|81.8|163.6|172.0|172.0|172.0|
|**525 SD**|**720**|**480**|1 350|345 600|15.0|30.0|80.0|160.0|172.0|172.0|172.0|
|**4CIF**|**704**|**576**|1 584|405 504|12.8|25.6|68.2|136.4|155.2|155.2|172.0|
|**625 SD**|**720**|**576**|1 620|414 720|12.5|25.0|66.7|133.3|151.7|151.7|172.0|
|**SVGA**|**800**|**600**|1 900|486 400|-|-|56.8|113.7|129.3|129.3|172.0|
|**XGA**|**1024**|**768**|3 072|786 432|-|-|35.2|70.3|80.0|80.0|170.0|
|**720p HD**|**1280**|**720**|3 600|921 600|-|-|30.0|60.0|68.3|68.3|145.1|
|**4VGA**|**1280**|**960**|4 800|1 228 800|-|-|-|45.0|51.2|51.2|108.8|
|**SXGA**|**1280**|**1024**|5 120|1 310 720|-|-|-|42.2|48.0|48.0|102.0|
|**525 16SIF**|**1408**|**960**|5 280|1 351 680|-|-|-|-|46.5|46.5|98.9|
|**16CIF**|**1408**|**1152**|6 336|1 622 016|-|-|-|-|38.8|38.8|82.4|
|**4SVGA**|**1600**|**1200**|7 500|1 920 000|-|-|-|-|32.8|32.8|69.6|
|**1080 HD**|**1920**|**1088**|8 160|2 088 960|-|-|-|-|30.1|30.1|64.0|
|**2Kx1K**|**2048**|**1024**|8 192|2 097 152|-|-|-|-|30.0|30.0|63.8|
|**2Kx1080**|**2048**|**1088**|8 704|2 228 224|-|-|-|-|-|-|60.0|
|**4XGA**|**2048**|**1536**|12 288|3 145 728|-|-|-|-|-|-|-|
|**16VGA**|**2560**|**1920**|19 200|4 915 200|-|-|-|-|-|-|-|
|**3616x1536 (2.35:1)**|**3616**|**1536**|21 696|5 554 176|-|-|-|-|-|-|-|
|**3672x1536 (2.39:1)**|**3680**|**1536**|22 080|5 652 480|-|-|-|-|-|-|-|
|**3840x2160**|**3840**|**2160**|31 035|7 948 800|-|-|-|-|-|-|-|
|**4Kx2K**|**4096**|**2048**|32 768|8 388 608|-|-|-|-|-|-|-|
|**4096x2160**|**4096**|**2160**|34 560|8 847 360|-|-|-|-|-|-|-|
|**4096x2304 (16:9)**|**4096**|**2304**|36 864|9 437 184|-|-|-|-|-|-|-|
|**7680x4320**|**7680**|**4320**|129 600|33 177 600|-|-|-|-|-|-|-|
|**8192x4096**|**8192**|**4096**|131 072|33 554 432|-|-|-|-|-|-|-|
|**8192x4320**|**8192**|**4320**|138 240|35 389 440|-|-|-|-|-|-|-|





**Table A-6 (concluded) – Maximum frame rates (frames per second) for some example frame sizes**













|Level:|Col2|Col3|Col4|Col5|5|5.1|5.2|6|6.1|6.2|
|---|---|---|---|---|---|---|---|---|---|---|
|**Max frame size (macroblocks):**|||||**22 080**|**36 864**|**36 864**|**139 264**|**139 264**|**139 264**|
|**Max macroblocks/second:**|||||**589 824**|**983 040**|**2 073 600**|**4 177 920**|**8 355 840**|**16 711 680**|
||||||||||||
|**Max frame size (samples):**|||||5 652 480|9 437 184|9 437 184|35 651 584|35 651 584|35 651 584|
|**Max samples/second:**|||||150 994 944|251 658 240|530 841 600|1 069 547 520|2 139 095 040|4 278 190 080|
|**Format**|**Luma**<br>**Width**|**Luma**<br>**Height**|**MBs**<br>**Total**|**Luma**<br>**Samples**|||||||
|**SQCIF**|**128**|**96**|48|12 288|172.0|172.0|172.0|300.0|300.0|300.0|
|**QCIF**|**176**|**144**|99|25 344|172.0|172.0|172.0|300.0|300.0|300.0|
|**QVGA**|**320**|**240**|300|76 800|172.0|172.0|172.0|300.0|300.0|300.0|
|**525 SIF**|**352**|**240**|330|84 480|172.0|172.0|172.0|300.0|300.0|300.0|
|**CIF**|**352**|**288**|396|101 376|172.0|172.0|172.0|300.0|300.0|300.0|
|**525 HHR**|**352**|**480**|660|168 960|172.0|172.0|172.0|300.0|300.0|300.0|
|**625 HHR**|**352**|**576**|792|202 752|172.0|172.0|172.0|300.0|300.0|300.0|
|**VGA**|**640**|**480**|1 200|307 200|172.0|172.0|172.0|300.0|300.0|300.0|
|**525 4SIF**|**704**|**480**|1 320|337 920|172.0|172.0|172.0|300.0|300.0|300.0|
|**525 SD**|**720**|**480**|1 350|345 600|172.0|172.0|172.0|300.0|300.0|300.0|
|**4CIF**|**704**|**576**|1 584|405 504|172.0|172.0|172.0|300.0|300.0|300.0|
|**625 SD**|**720**|**576**|1 620|414 720|172.0|172.0|172.0|300.0|300.0|300.0|
|**SVGA**|**800**|**600**|1 900|486 400|172.0|172.0|172.0|300.0|300.0|300.0|
|**XGA**|**1024**|**768**|3 072|786 432|172.0|172.0|172.0|300.0|300.0|300.0|
|**720p HD**|**1280**|**720**|3 600|921 600|163.8|172.0|172.0|300.0|300.0|300.0|
|**4VGA**|**1280**|**960**|4 800|1 228 800|122.9|172.0|172.0|300.0|300.0|300.0|
|**SXGA**|**1280**|**1024**|5 120|1 310 720|115.2|172.0|172.0|300.0|300.0|300.0|
|**525 16SIF**|**1408**|**960**|5 280|1 351 680|111.7|172.0|172.0|300.0|300.0|300.0|
|**16CIF**|**1408**|**1152**|6 336|1 622 016|93.1|155.2|172.0|300.0|300.0|300.0|
|**4SVGA**|**1600**|**1200**|7 500|1 920 000|78.6|131.1|172.0|300.0|300.0|300.0|
|**1080 HD**|**1920**|**1088**|8 160|2 088 960|72.3|120.5|172.0|300.0|300.0|300.0|
|**2Kx1K**|**2048**|**1024**|8 192|2 097 152|72.0|120.0|172.0|300.0|300.0|300.0|
|**2Kx1080**|**2048**|**1088**|8 704|2 228 224|67.8|112.9|172.0|300.0|300.0|300.0|
|**4XGA**|**2048**|**1536**|12 288|3 145 728|48.0|80.0|168.8|300.0|300.0|300.0|
|**16VGA**|**2560**|**1920**|19 200|4 915 200|30.7|51.2|108.0|217.6|300.0|300.0|
|**3616x1536 (2.35:1)**|**3616**|**1536**|21 696|5 554 176|27.2|45.3|95.6|192.6|300.0|300.0|
|**3672x1536 (2.39:1)**|**3680**|**1536**|22 080|5 652 480|26.7|44.5|93.9|189.2|300.0|300.0|
|**3840x2160**|**3840**|**2160**|31 035|7 948 800|-|30.3|64.0|128.9|257.9|300.0|
|**4Kx2K**|**4096**|**2048**|32 768|8 388 608|-|30.0|63.3|127.5|255.0|300.0|
|**4096x2160**|**4096**|**2160**|34 560|8 847 360|-|28.4|60.0|120.9|241.8|300.0|
|**4096x2304 (16:9)**|**4096**|**2304**|36 864|9 437 184|-|26.7|56.3|113.3|226.7|300.0|
|**7680x4320**|**7680**|**4320**|129 600|33 177 600|-|-|-|32.2|64.5|128.9|
|**8192x4096**|**8192**|**4096**|131 072|33 554 432|-|-|-|31.9|63.8|127.5|
|**8192x4320**|**8192**|**4320**|138 240|35 389 440|- <br>|-|-|30.2|60.4|120.9|


The following should be noted:

- This Recommendation | International Standard is a variable-frame-size specification. The specific frame sizes in
Table A-6 are illustrative examples only.

- As used in Table A-6, "525" refers to typical use for environments using 525 analogue scan lines (of which
approximately 480 lines contain the visible picture region), and "625" refers to environments using 625 analogue scan
lines (of which approximately 576 lines contain the visible picture region).

- XGA is also known as (aka) XVGA, 4SVGA aka UXGA, 16XGA aka 4Kx3K, CIF aka 625 SIF, 625 HHR aka 2CIF
aka half 625 D-1, aka half 625 ITU-R BT.601, 525 SD aka 525 D-1 aka 525 ITU-R BT.601, 625 SD aka 625 D-1 aka
625 ITU-R BT.601.

`–` The given maximum frame rate values that have a zero to the right of the decimal point are exact. Others have been
rounded to the nearest 0.1 frames per second, i.e., the precise maximum frame rates may be higher or lower within a
margin of plus or minus 0.05 frames per second. For example, for Level 4, the maximum frame rate for 720p HD has
been rounded up to 68.3 from a value of 68.2666…, and the maximum frame rate for 1080 HD has been rounded
down to 30.1 from a value of 30.1176….

- Frame rates given are correct for progressive scan modes. The frame rates are also correct for interlaced video coding
for the cases of frame height divisible by 32.





**A.3.5** **Effect of level limits on maximum DPB size in units of frames (informative)**


This clause does not form an integral part of this Recommendation | International Standard.


**Table A-7 – Maximum DPB size (frames) for some example frame sizes**

|Level:|Col2|Col3|Col4|1|1b|1.1|1.2|1.3|2|2.1|2.2|
|---|---|---|---|---|---|---|---|---|---|---|---|
|**Max frame size (macroblocks):**||||99|99|396|396|396|396|792|1 620|
|**Max DPB size (macroblocks):**||||396|396|900|2 376|2 376|2 376|4 752|8 100|
|**Format**|**Luma Width**|** Luma Height**|MBs Total|||||||||
|**SQCIF**|**128**|**96**|48|8|8|16|16|16|16|16|16|
|**QCIF**|**176**|**144**|99|4|4|9|16|16|16|16|16|
|**QVGA**|**320**|**240**|300|-|-|3|7|7|7|15|16|
|**525 SIF**|**352**|**240**|330|-|-|2|7|7|7|14|16|
|**CIF**|**352**|**288**|396|-|-|2|6|6|6|12|16|
|**525 HHR**|**352**|**480**|660|-|-|-|-|-|-|7|12|
|**625 HHR**|**352**|**576**|792|-|-|-|-|-|-|6|10|
|**VGA**|**640**|**480**|1 200|-|-|-|-|-|-|-|6|
|**525 4SIF**|**704**|**480**|1 320|-|-|-|-|-|-|-|6|
|**525 SD**|**720**|**480**|1 350|-|-|-|-|-|-|-|6|
|**4CIF**|**704**|**576**|1 584|-|-|-|-|-|-|-|5|
|**625 SD**|**720**|**576**|1 620|-|-|-|-|-|-|-|5|
|**SVGA**|**800**|**600**|1 900|-|-|-|-|-|-|-|-|
|**XGA**|**1024**|**768**|3 072|-|-|-|-|-|-|-|-|
|**720p HD**|**1280**|**720**|3 600|-|-|-|-|-|-|-|-|
|**4VGA**|**1280**|**960**|4 800|-|-|-|-|-|-|-|-|
|**SXGA**|**1280**|**1024**|5 120|-|-|-|-|-|-|-|-|
|**525 16SIF**|**1408**|**960**|5 280|-|-|-|-|-|-|-|-|
|**16CIF**|**1408**|**1152**|6 336|-|-|-|-|-|-|-|-|
|**4SVGA**|**1600**|**1200**|7 500|-|-|-|-|-|-|-|-|
|**1080 HD**|**1920**|**1088**|8 160|-|-|-|-|-|-|-|-|
|**2Kx1K**|**2048**|**1024**|8 192|-|-|-|-|-|-|-|-|
|**2Kx1080**|**2048**|**1088**|8 704|-|-|-|-|-|-|-|-|
|**4XGA**|**2048**|**1536**|12 288|-|-|-|-|-|-|-|-|
|**16VGA**|**2560**|**1920**|19 200|-|-|-|-|-|-|-|-|
|**3616x1536 (2.35:1)**|**3616**|**1536**|21 696|-|-|-|-|-|-|-|-|
|**3672x1536 (2.39:1)**|**3680**|**1536**|22 080|-|-|-|-|-|-|-|-|
|**3840x2160**|**3840**|**2160**|31 035|-|-|-|-|-|-|-|-|
|**4Kx2K**|**4096**|**2048**|32 768|-|-|-|-|-|-|-|-|
|**4096x2160**|**4096**|**2160**|34 560|-|-|-|-|-|-|-|-|
|**4096x2304 (16:9)**|**4096**|**2304**|36 864|-|-|-|-|-|-|-|-|
|**7680x4320**|**7680**|**4320**|129 600|-|-|-|-|-|-|-|-|
|**8192x4096**|**8192**|**4096**|131 072|-|-|-|-|-|-|-|-|
|**8192x4320**|**8192**|**4320**|138 240|-|-|-|-|-|-|-|-|






**Table A-7 (continued) – Maximum DPB size (frames) for some example frame sizes**







|Level:|Col2|Col3|Col4|3|3.1|3.2|4|4.1|4.2|
|---|---|---|---|---|---|---|---|---|---|
|**Max frame size (macroblocks):**||||**1 620**|**3 600**|**5 120**|**8 192**|**8 192**|<br>**8 704**|
|**Max DPB size (macroblocks):**||||**8 100**|**18 000**|**20 480**|**32 768**|**32 768**|<br>**34 816**|
|**Format**|**Luma Width**|** Luma Height**|**MBs**<br>**Total**||||||<br>|
|**SQCIF**|**128**|**96**|48|16|16|16|16|16|<br>16|
|**QCIF**|**176**|**144**|99|16|16|16|16|16|<br>16|
|**QVGA**|**320**|**240**|300|16|16|16|16|16|<br>16|
|**525 SIF**|**352**|**240**|330|16|16|16|16|16|<br>16|
|**CIF**|**352**|**288**|396|16|16|16|16|16|<br>16|
|**525 HHR**|**352**|**480**|660|12|16|16|16|16|<br>16|
|**625 HHR**|**352**|**576**|792|10|16|16|16|16|<br>16|
|**VGA**|**640**|**480**|1 200|6|15|16|16|16|<br>16|
|**525 4SIF**|**704**|**480**|1 320|6|13|15|16|16|<br>16|
|**525 SD**|**720**|**480**|1 350|6|13|15|16|16|<br>16|
|**4CIF**|**704**|**576**|1 584|5|11|12|16|16|<br>16|
|**625 SD**|**720**|**576**|1 620|5|11|12|16|16|<br>16|
|**SVGA**|**800**|**600**|1 900|-|9|10|16|16|<br>16|
|**XGA**|**1024**|**768**|3 072|-|5|6|10|10|<br>11|
|**720p HD**|**1280**|**720**|3 600|-|5|5|9|9|<br>9|
|**4VGA**|**1280**|**960**|4 800|-|-|4|6|6|<br>7|
|**SXGA**|**1280**|**1024**|5 120|-|-|4|6|6|<br>6|
|**525 16SIF**|**1408**|**960**|5 280|-|-|-|6|6|<br>6|
|**16CIF**|**1408**|**1152**|6 336|-|-|-|5|5|<br>5|
|**4SVGA**|**1600**|**1200**|7 500|-|-|-|4|4|<br>4|
|**1080 HD**|**1920**|**1088**|8 160|-|-|-|4|4|<br>4|
|**2Kx1K**|**2048**|**1024**|8 192|-|-|-|4|4|<br>4|
|**2Kx1080**|**2048**|**1088**|8 704|-|-|-|-|-|<br>4|
|**4XGA**|**2048**|**1536**|12 288|-|-|-|-|-|<br>-|
|**16VGA**|**2560**|**1920**|19 200|-|-|-|-|-|<br>-|
|**3616x1536 (2.35:1)**|**3616**|**1536**|21 696|-|-|-|-|-|<br>-|
|**3672x1536 (2.39:1)**|**3680**|**1536**|22 080|-|-|-|-|-|<br>-|
|**3840x2160**|**3840**|**2160**|31 035|-|-|-|-|-|<br>-|
|**4Kx2K**|**4096**|**2048**|32 768|-|-|-|-|-|<br>-|
|**4096x2160**|**4096**|**2160**|34 560|-|-|-|-|-|<br>-|
|**4096x2304 (16:9)**|**4096**|**2304**|36 864|-|-|-|-|-|<br>-|
|**7680x4320**|**7680**|**4320**|129 600|-|-|-|-|-|<br>-|
|**8192x4096**|**8192**|**4096**|131 072|-|-|-|-|-|<br>-|
|**8192x4320**|**8192**|**4320**|138 240|-|-|-|-|-|<br>-|


**Table A-7 (concluded) – Maximum DPB size (frames) for some example frame sizes**

|Level:|Col2|Col3|Col4|5|5.1|5.2|6|6.1|6.2|
|---|---|---|---|---|---|---|---|---|---|
|**Max frame size (macroblocks):**||||**22 080**|**36 864**|**36 864**|** 129 600**|**  131 072**|**   138 240**|
|**Max DPB size (macroblocks):**||||**110 400**|** 184 320**|**  184 320**|**   696 320**|**    696 320**|**     696 320**|
|**Format**|**Luma Width**|** Luma Height**|**  MBs Total**|||||||
|**SQCIF**|**128**|**96**|48|16|16|16|16|16|16|
|**QCIF**|**176**|**144**|99|16|16|16|16|16|16|
|**QVGA**|**320**|**240**|300|16|16|16|16|16|16|
|**525 SIF**|**352**|**240**|330|16|16|16|16|16|16|
|**CIF**|**352**|**288**|396|16|16|16|16|16|16|
|**525 HHR**|**352**|**480**|660|16|16|16|16|16|16|
|**625 HHR**|**352**|**576**|792|16|16|16|16|16|16|
|**VGA**|**640**|**480**|1 200|16|16|16|16|16|16|
|**525 4SIF**|**704**|**480**|1 320|16|16|16|16|16|16|
|**525 SD**|**720**|**480**|1 350|16|16|16|16|16|16|
|**4CIF**|**704**|**576**|1 584|16|16|16|16|16|16|
|**625 SD**|**720**|**576**|1 620|16|16|16|16|16|16|
|**SVGA**|**800**|**600**|1 900|16|16|16|16|16|16|
|**XGA**|**1024**|**768**|3 072|16|16|16|16|16|16|
|**720p HD**|**1280**|**720**|3 600|16|16|16|16|16|16|
|**4VGA**|**1280**|**960**|4 800|16|16|16|16|16|16|
|**SXGA**|**1280**|**1024**|5 120|16|16|16|16|16|16|
|**525 16SIF**|**1408**|**960**|5 280|16|16|16|16|16|16|
|**16CIF**|**1408**|**1152**|6 336|16|16|16|16|16|16|
|**4SVGA**|**1600**|**1200**|7 500|14|16|16|16|16|16|
|**1080 HD**|**1920**|**1088**|8 160|13|16|16|16|16|16|
|**2Kx1K**|**2048**|**1024**|8 192|13|16|16|16|16|16|
|**2Kx1080**|**2048**|**1088**|8 704|12|16|16|16|16|16|
|**4XGA**|**2048**|**1536**|12 288|8|15|15|16|16|16|
|**16VGA**|**2560**|**1920**|19 200|5|9|9|16|16|16|
|**3616x1536 (2.35:1)**|**3616**|**1536**|21 696|5|8|8|16|16|16|
|**3672x1536 (2.39:1)**|**3680**|**1536**|22 080|5|8|8|16|16|16|
|**3840x2160**|**3840**|**2160**|31 035|-|5|5|16|16|16|
|**4Kx2K**|**4096**|**2048**|32 768|-|5|5|16|16|16|
|**4096x2160**|**4096**|**2160**|34 560|-|5|5|16|16|16|
|**4096x2304 (16:9)**|**4096**|**2304**|36 864|-|5|5|16|16|16|
|**7680x4320**|**7680**|**4320**|129 600|-|-|-|5|5|5|
|**8192x4096**|**8192**|**4096**|131 072|-|-|-|5|5|5|
|**8192x4320**|**8192**|**4320**|138 240|-|-|-|5|5|5|






The following should be noted:

- As used in Table A-7, "525" refers to typical use for environments using 525 analogue scan lines (of which
approximately 480 lines contain the visible picture region), and "625" refers to environments using 625 analogue scan
lines (of which approximately 576 lines contain the visible picture region).

- XGA is also known as (aka) XVGA, 4SVGA aka UXGA, 16XGA aka 4Kx3K, CIF aka 625 SIF, 625 HHR aka 2CIF
aka half 625 D-1, aka half 625 ITU-R BT.601, 525 SD aka 525 D-1 aka 525 ITU-R BT.601, 625 SD aka 625 D-1 aka
625 ITU-R BT.601.
