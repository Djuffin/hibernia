**G.10** **Profiles and levels**


The specifications in Annex A apply. Additional profiles and specific values of profile_idc are specified in the following.


The profiles that are specified in clause G.10.1 are also referred to as the profiles specified in Annex G.


**G.10.1** **Profiles**


All constraints for picture parameter sets that are specified in clauses G.10.1.1 to G.10.1.3 are constraints for picture
parameter sets that become the active picture parameter set or an active layer picture parameter set inside the bitstream.
All constraints for SVC sequence parameter sets that are specified in clauses G.10.1.1 to G.10.1.3 are constraints for SVC
sequence parameter sets that become the active SVC sequence parameter set or an active layer SVC sequence parameter
set inside the bitstream. All constraints for sequence parameter sets of the base layer bitstream that are specified in
clauses G.10.1.1 to G.10.1.3 are constraints for sequence parameter sets that are activated in the base layer bitstream.


**G.10.1.1Scalable Baseline profile**


Bitstreams conforming to the Scalable Baseline profile shall obey the following constraints:


a) The base layer bitstream as specified in clause G.8.8.2 shall obey the following constraints:


i) All constraints of the Baseline and Constrained Baseline profiles specified in clauses A.2.1 and A.2.1.1

shall be obeyed.


ii) Sequence parameter sets should have profile_idc equal to 66. Sequence parameter sets may have

profile_idc equal to 77 or 88. Sequence parameter sets shall not have profile_idc equal to a value other
than 66, 77, or 88.


iii) Sequence parameter sets shall have constraint_set0_flag, constraint_set1_flag, and constraint_set2_flag

equal to 1.
NOTE 1 – The above constraint implies that picture parameter sets must have num_slice_groups_minus1 equal to 0 and
redundant_pic_cnt_present_flag equal to 0 and that arbitrary slice order is not allowed.
NOTE 2 – In addition to the base layer constraints specified above in items i) through iii), the value of the syntax element
constrained_intra_pred_flag for picture parameter sets of the base layer stream is constrained as specified below in item l).


b) A list of integer values specifying layer representation identifiers is derived by invoking the process specified in

clause G.8.1.1 with the output being the list dqIdList. The SVC sequence parameter sets that are referred to by
coded slice NAL units with DQId greater than 0 and DQId in the list dqIdList shall have profile_idc equal to 83
or (profile_idc equal to 86 and constraint_set0_flag equal to 1).


c) Only I, P, EI, EP, and EB slices shall be present.


d) SVC sequence parameter sets shall have chroma_format_idc equal to 1.


e) SVC sequence parameter sets shall have bit_depth_luma_minus8 equal to 0.


f) SVC sequence parameter sets shall have bit_depth_chroma_minus8 equal to 0.


g) SVC sequence parameter sets shall have separate_colour_plane_flag equal to 0.


h) SVC sequence parameter sets shall have qpprime_y_zero_transform_bypass_flag equal to 0.


i) SVC sequence parameter sets shall have frame_mbs_only_flag equal to 1.


j) Picture parameter sets shall have num_slice_groups_minus1 in the range of 0 to 7, inclusive.


k) The value of slice_group_map_type, when present in picture parameter sets, shall be equal to 2.


l) A list of integer values specifying layer representation identifiers is derived by invoking the process specified in
clause G.8.1.1 with the output being the list dqIdList. The variable numDQEntries is set equal to the number of
elements in the list dqIdList. When numDQEntries is greater than 1, for any element dqIdList[ i ] with
i = 1..(numDQEntries − 1), when MaxTCoeffLevelPredFlag is equal to 0 for any layer representation with DQId
in the set specified by dqIdList[ k ] with k = 0..i, the picture parameter set that is referenced by the coded slice





NAL units of the layer representation with DQId equal to dqIdList[ i ] shall have constrained_intra_pred_flag
equal to 1.


m) For each present layer representation with dependency_id greater than 0, quality_id equal to 0, and

MinNoInterLayerPredFlag equal to 0, one of the following constraints shall be obeyed.


     - ScaledRefLayerPicWidthInSamplesL is equal to RefLayerPicWidthInSamplesL and
ScaledRefLayerPicHeightInSamplesL is equal to RefLayerPicHeightInSamplesL


     - ScaledRefLayerPicWidthInSamplesL is equal to (1.5 * RefLayerPicWidthInSamplesL) and
ScaledRefLayerPicHeightInSamplesL is equal to (1.5 * RefLayerPicHeightInSamplesL)


     - ScaledRefLayerPicWidthInSamplesL is equal to (2 * RefLayerPicWidthInSamplesL) and
ScaledRefLayerPicHeightInSamplesL is equal to (2 * RefLayerPicHeightInSamplesL)


n) For each present layer representation with dependency_id greater than 0, quality_id equal to 0, and

MinNoInterLayerPredFlag equal to 0, all of the following constraints shall be obeyed.


     - (ScaledRefLayerLeftOffset % 16) is equal to 0


     - (ScaledRefLayerTopOffset % 16) is equal to 0


o) The level constraints specified in clause G.10.2 shall be fulfilled.


Conformance of a bitstream to the Scalable Baseline profile is indicated by profile_idc equal to 83.


Decoders conforming to the Scalable Baseline profile at a specific level shall be capable of decoding all bitstreams in
which both of the following conditions are true:


a) All active SVC sequence parameter sets have one of the following conditions fulfilled:


     - profile_idc is equal to 83,


     - profile_idc is equal to 86 and constraint_set0_flag is equal to 1,


     - profile_idc is equal to 66 and constraint_set1_flag is equal to 1,


     - profile_idc is equal to 77 and constraint_set0_flag is equal to 1,


     - profile_idc is equal to 88, constraint_set0_flag is equal to 1, and constraint_set1_flag is equal to 1.


b) level_idc or (level_idc and constraint_set3_flag) for all active SVC sequence parameter sets represent a level less

than or equal to the specified level.


**G.10.1.1.1Scalable Constrained Baseline profile**


Bitstreams conforming to the Scalable Constrained Baseline profile shall obey the following constraints:


a) The base layer bitstream as specified in clause G.8.8.2 shall obey the following constraints:


i) All constraints of the Baseline and Constrained Baseline profiles specified in clauses A.2.1 and A.2.1.1

shall be obeyed.


ii) Sequence parameter sets should have profile_idc equal to 66. Sequence parameter sets may have

profile_idc equal to 77 or 88. Sequence parameter sets shall not have profile_idc equal to a value other
than 66, 77, or 88.


iii) Sequence parameter sets shall have constraint_set0_flag, constraint_set1_flag, and constraint_set2_flag

equal to 1.
NOTE 1 – The above constraint implies that picture parameter sets must have num_slice_groups_minus1 equal
to 0 and redundant_pic_cnt_present_flag equal to 0 and that arbitrary slice order is not allowed.
NOTE 2 – In addition to the base layer constraints specified above in items i) through iii), the value of the syntax
element constrained_intra_pred_flag for picture parameter sets of the base layer stream is constrained as specified
below in item m).


b) A list of integer values specifying layer representation identifiers is derived by invoking the process specified in

clause G.8.1.1 with the output being the list dqIdList. The SVC sequence parameter sets that are referred to by
coded slice NAL units with DQId greater than 0 and DQId in the list dqIdList shall have profile_idc equal to 83
and both constraint_set1_flag and constraint_set5_flag equal to 1 or (profile_idc equal to 86 and both
constraint_set0_flag and constraint_set5_flag equal to 1).


c) Only I, P, EI, and EP slices shall be present.


d) SVC sequence parameter sets shall have chroma_format_idc equal to 1.





e) SVC sequence parameter sets shall have bit_depth_luma_minus8 equal to 0.


f) SVC sequence parameter sets shall have bit_depth_chroma_minus8 equal to 0.


g) SVC sequence parameter sets shall have separate_colour_plane_flag equal to 0.


h) SVC sequence parameter sets shall have qpprime_y_zero_transform_bypass_flag equal to 0.


i) SVC sequence parameter sets shall have frame_mbs_only_flag equal to 1.


j) Picture parameter sets shall have redundant_pic_cnt_present_flag equal to 0.


k) Picture parameter sets shall have num_slice_groups_minus1 equal to 0.


l) Arbitrary slice order is not allowed.


m) A list of integer values specifying layer representation identifiers is derived by invoking the process specified in

clause G.8.1.1 with the output being the list dqIdList. The variable numDQEntries is set equal to the number of
elements in the list dqIdList. When numDQEntries is greater than 1, for any element dqIdList[ i ] with
i = 1..(numDQEntries − 1), when MaxTCoeffLevelPredFlag is equal to 0 for any layer representation with DQId
in the set specified by dqIdList[ k ] with k = 0..i, the picture parameter set that is referenced by the coded slice
NAL units of the layer representation with DQId equal to dqIdList[ i ] shall have constrained_intra_pred_flag
equal to 1.


n) For each present layer representation with dependency_id greater than 0, quality_id equal to 0, and

MinNoInterLayerPredFlag equal to 0, one of the following constraints shall be obeyed.


     - ScaledRefLayerPicWidthInSamplesL is equal to RefLayerPicWidthInSamplesL and
ScaledRefLayerPicHeightInSamplesL is equal to RefLayerPicHeightInSamplesL


     - ScaledRefLayerPicWidthInSamplesL is equal to (1.5 * RefLayerPicWidthInSamplesL) and
ScaledRefLayerPicHeightInSamplesL is equal to (1.5 * RefLayerPicHeightInSamplesL)


     - ScaledRefLayerPicWidthInSamplesL is equal to (2 * RefLayerPicWidthInSamplesL) and
ScaledRefLayerPicHeightInSamplesL is equal to (2 * RefLayerPicHeightInSamplesL)


o) For each present layer representation with dependency_id greater than 0, quality_id equal to 0, and

MinNoInterLayerPredFlag equal to 0, all of the following constraints shall be obeyed.


     - (ScaledRefLayerLeftOffset % 16) is equal to 0


     - (ScaledRefLayerTopOffset % 16) is equal to 0


p) The level constraints specified in clause G.10.2 shall be fulfilled.


Conformance of a bitstream to the Scalable Constrained Baseline profile is indicated by constraint_set5_flag being equal
to 1 with profile_idc equal to 83.


Decoders conforming to the Scalable Constrained Baseline profile at a specific level shall be capable of decoding all
bitstreams in which both of the following conditions are true:


a) All active SVC sequence parameter sets have one of the following conditions fulfilled:


     - profile_idc is equal to 83 and constraint_set5_flag is equal to 1,


     - profile_idc is equal to 86, constraint_set0_flag is equal to 1, and constraint_set5_flag equal to 1,


     - profile_idc is equal to 66 and constraint_set1_flag is equal to 1,


     - profile_idc is equal to 77 and constraint_set0_flag is equal to 1,


     - profile_idc is equal to 88, constraint_set0_flag is equal to 1, and constraint_set1_flag is equal to 1.


b) level_idc or (level_idc and constraint_set3_flag) for all active SVC sequence parameter sets represent a level less

than or equal to the specified level.





**G.10.1.2Scalable High profile**


Bitstreams conforming to the Scalable High profile shall obey the following constraints:


a) The base layer bitstream as specified in clause G.8.8.2 shall obey the following constraints:


i) All constraints of the High profile specified in clause A.2.4 shall be obeyed.


ii) Sequence parameter sets should have profile_idc equal to 100. Sequence parameter sets may have

profile_idc equal to 66, 77, or 88 and constraint_set1_flag equal to 1. Sequence parameter sets shall not
have profile_idc equal to a value other than 66, 77, 88, or 100.


iii) The syntax element direct_spatial_mv_pred_flag shall be equal to 1.
NOTE – In addition to the base layer constraints specified above in items i) through iii), the value of the syntax element
constrained_intra_pred_flag for picture parameter sets of the base layer stream is constrained as specified below in item k).


b) A list of integer values specifying layer representation identifiers is derived by invoking the process specified in

clause G.8.1.1 with the output being the list dqIdList. The SVC sequence parameter sets that are referred to by
coded slice NAL units with DQId greater than 0 and DQId in the list dqIdList shall have profile_idc equal to 86
or (profile_idc equal to 83 and constraint_set1_flag equal to 1).


c) Only I, P, B, EI, EP, and EB slices shall be present.


d) SVC sequence parameter sets shall have chroma_format_idc equal to 1.


e) SVC sequence parameter sets shall have bit_depth_luma_minus8 equal to 0.


f) SVC sequence parameter sets shall have bit_depth_chroma_minus8 equal to 0.


g) SVC sequence parameter sets shall have separate_colour_plane_flag equal to 0.


h) SVC sequence parameter sets shall have qpprime_y_zero_transform_bypass_flag equal to 0.


i) Picture parameter sets shall have redundant_pic_cnt_present_flag equal to 0.


j) Picture parameter sets shall have num_slice_groups_minus1 equal to 0.


k) A list of integer values specifying layer representation identifiers is derived by invoking the process specified in

clause G.8.1.1 with the output being the list dqIdList. The variable numDQEntries is set equal to the number of
elements in the list dqIdList. When numDQEntries is greater than 1, for any element dqIdList[ i ] with
i = 1..(numDQEntries − 1), when MaxTCoeffLevelPredFlag is equal to 0 for any layer representation with DQId
in the set specified by dqIdList[ k ] with k = 0..i, the picture parameter set that is referenced by the coded slice
NAL units of the layer representation with DQId equal to dqIdList[ i ] shall have constrained_intra_pred_flag
equal to 1.


l) Arbitrary slice order is not allowed.


m) The level constraints specified in clause G.10.2 shall be fulfilled.


Conformance of a bitstream to the Scalable High profile is indicated by profile_idc equal to 86.


Decoders conforming to the Scalable High profile at a specific level shall be capable of decoding all bitstreams in which
both of the following conditions are true:


a) All active SVC sequence parameter sets have one of the following conditions fulfilled:


     - profile_idc is equal to 86,


     - profile_idc is equal to 83 and constraint_set1_flag is equal to 1,


     - profile_idc is equal to 77 or 100,


     - profile_idc is equal to 66 or 88 and constraint_set1_flag is equal to 1.


b) level_idc or (level_idc and constraint_set3_flag) for all active SVC sequence parameter sets represent a level less

than or equal to the specified level.


**G.10.1.2.1Scalable Constrained High profile**


Bitstreams conforming to the Scalable Constrained High profile shall obey the following constraints:


a) The base layer bitstream as specified in clause G.8.8.2 shall obey the following constraints:


i) All constraints of the Constrained High profile specified in clause A.2.4.2 shall be obeyed.





ii) Sequence parameter sets should have profile_idc equal to 100. Sequence parameter sets may have

profile_idc equal to 66, 77, or 88 and constraint_set1_flag equal to 1. Sequence parameter sets shall not
have profile_idc equal to a value other than 66, 77, 88, or 100.


iii) The syntax element direct_spatial_mv_pred_flag shall be equal to 1.
NOTE – In addition to the base layer constraints specified above in items i) through iii), the value of the syntax
element constrained_intra_pred_flag for picture parameter sets of the base layer stream is constrained as specified
below in item l).


b) A list of integer values specifying layer representation identifiers is derived by invoking the process specified in

clause G.8.8.1 with the output being the list dqIdList. The SVC sequence parameter sets that are referred to by
coded slice NAL units with DQId greater than 0 and DQId in the list dqIdList shall have profile_idc equal to 86
and constraint_set5_flag equal to 1 or (profile_idc equal to 83 and both constraint_set1_flag and
constraint_set5_flag equal to 1).


c) Only I, P, EI, and EP slices shall be present.


d) SVC sequence parameter sets shall have chroma_format_idc equal to 1.


e) SVC sequence parameter sets shall have bit_depth_luma_minus8 equal to 0.


f) SVC sequence parameter sets shall have bit_depth_chroma_minus8 equal to 0.


g) SVC sequence parameter sets shall have separate_colour_plane_flag equal to 0.


h) SVC sequence parameter sets shall have qpprime_y_zero_transform_bypass_flag equal to 0.


i) SVC sequence parameter sets shall have frame_mbs_only_flag equal to 1.


j) Picture parameter sets shall have redundant_pic_cnt_present_flag equal to 0.


k) Picture parameter sets shall have num_slice_groups_minus1 equal to 0.


l) A list of integer values specifying layer representation identifiers is derived by invoking the process specified in
clause G.8.8.1 with the output being the list dqIdList. The variable numDQEntries is set equal to the number of
elements in the list dqIdList. When numDQEntries is greater than 1, for any element dqIdList[ i ] with
i = 1..(numDQEntries − 1), when MaxTCoeffLevelPredFlag is equal to 0 for any layer representation with DQId
in the set specified by dqIdList[ k ] with k = 0..i, the picture parameter set that is referenced by the coded slice
NAL units of the layer representation with DQId equal to dqIdList[ i ] shall have constrained_intra_pred_flag
equal to 1.


m) Arbitrary slice order is not allowed.


n) For each present layer representation with dependency_id greater than 0, quality_id equal to 0, and

MinNoInterLayerPredFlag equal to 0, one of the following constraints shall be obeyed.


     - ScaledRefLayerPicWidthInSamplesL is equal to RefLayerPicWidthInSamplesL and
ScaledRefLayerPicHeightInSamplesL is equal to RefLayerPicHeightInSamplesL


     - ScaledRefLayerPicWidthInSamplesL is equal to (1.5 * RefLayerPicWidthInSamplesL) and
ScaledRefLayerPicHeightInSamplesL is equal to (1.5 * RefLayerPicHeightInSamplesL)


     - ScaledRefLayerPicWidthInSamplesL is equal to (2 * RefLayerPicWidthInSamplesL) and
ScaledRefLayerPicHeightInSamplesL is equal to (2 * RefLayerPicHeightInSamplesL)


o) For each present layer representation with dependency_id greater than 0, quality_id equal to 0, and

MinNoInterLayerPredFlag equal to 0, all of the following constraints shall be obeyed.


     - (ScaledRefLayerLeftOffset % 16) is equal to 0


     - (ScaledRefLayerTopOffset % 16) is equal to 0


p) The level constraints specified in clause G.10.2 shall be fulfilled.


Conformance of a bitstream to the Scalable Constrained High profile is indicated by constraint_set5_flag being equal to 1
with profile_idc equal to 86.


Decoders conforming to the Scalable Constrained High profile at a specific level shall be capable of decoding all bitstreams
in which both of the following conditions are true:


a) All active SVC sequence parameter sets have one or more of the following conditions fulfilled:


     - profile_idc is equal to 86 and constraint_set5_flag is equal to 1,





     - profile_idc is equal to 83, constraint_set1_flag is equal to 1, and constraint_set5_flag is equal to 1,


     - (profile_idc is equal to 66 or constraint_set0_flag is equal to 1), constraint_set1_flag is equal to 1,


     - profile_idc is equal to 77 and constraint_set0_flag is equal to 1,


     - profile_idc is equal to 77, constraint_set4_flag is equal to 1, and constraint_set5_flag is equal to 1,


     - profile_idc is equal to 88, constraint_set1_flag is equal to 1, constraint_set4_flag is equal to 1, and
constraint_set5_flag is equal to 1,


     - profile_idc is equal to 100 and constraint_set4_flag is equal to 1, and constraint_set5_flag is equal to 1,


b) level_idc or (level_idc and constraint_set3_flag) for all active SVC sequence parameter sets represent a level less

than or equal to the specified level.


**G.10.1.3Scalable High Intra profile**


Bitstreams conforming to the Scalable High Intra profile shall obey the following constraints:


a) The base layer bitstream as specified in clause G.8.8.2 shall obey the following constraints:


i) All constraints of the High profile specified in clause A.2.4 shall be obeyed.


ii) Sequence parameter sets should have profile_idc equal to 100 and constraint_set3_flag equal to 1.

Sequence parameter sets may have profile_idc equal to 66, 77, or 88 and constraint_set1_flag equal to 1.
Sequence parameter sets shall not have profile_idc equal to a value other than 66, 77, 88, or 100.


b) A list of integer values specifying layer representation identifiers is derived by invoking the process specified in

clause G.8.1.1 with the output being the list dqIdList. The SVC sequence parameter sets that are referred to by
coded slice NAL units with DQId greater than 0 and DQId in the list dqIdList shall have profile_idc equal to 86
and constraint_set3_flag equal to 1.


c) All constraints of the Scalable High profile specified in clause G.10.1.2 shall be obeyed.


d) All pictures shall be IDR pictures.


e) SVC sequence parameter sets shall have max_num_ref_frames equal to 0.


f) When vui_parameters_present_flag is equal to 1 and bitstream_restriction_flag is equal to 1, SVC sequence
parameter sets shall have max_num_reorder_frames equal to 0.


g) When vui_parameters_present_flag is equal to 1 and bitstream_restriction_flag is equal to 1, SVC sequence

parameter sets shall have max_dec_frame_buffering equal to 0.


h) Picture timing SEI messages, whether present in the bitstream (by non-VCL NAL units) or conveyed equivalently

by other means not specified in this Recommendation | International Standard, shall have dpb_output_delay equal
to 0.


i) The level constraints specified in clause G.10.2 shall be fulfilled.


Conformance of a bitstream to the Scalable High Intra profile is indicated by constraint_set3_flag being equal to 1 with
profile_idc equal to 86.


Decoders conforming to the Scalable High Intra profile at a specific level shall be capable of decoding all bitstreams in
which both of the following conditions are true:


a) All active SVC sequence parameter sets have profile_idc equal to 86 or 100 and constraint_set3_flag equal to 1.


b) level_idc or (level_idc and constraint_set3_flag) for all active SVC sequence parameter sets represents a level

less than or equal to the specified level.


The operation of the deblocking filter process for target representation as specified in clause G.8.7.2 is not required for
decoder conformance to the Scalable High Intra profile.


**G.10.2** **Levels**


The following is specified for expressing the constraints in this clause:


- Let access unit n be the n-th access unit in decoding order with the first access unit being access unit 0.


- Let picture n be the primary coded picture or the corresponding decoded picture of access unit n.


The variable fR is derived as follows:





- If the level number is equal to 6.0, 6.1, or 6.2, fR is set equal to 1 ÷ 300.


- Otherwise, if picture n is a frame, fR is set equal to (1 ÷ 172).


- Otherwise (picture n is a field), fR is set equal to ( 1 ÷ ( 172 * 2 ) ).


**G.10.2.1Level limits common to Scalable Baseline, Scalable Constrained Baseline, Scalable High, Scalable**

**Constrained High, and Scalable High Intra profiles**


The variable dqIdMax is set equal to the maximum value of DQId for the layer representation of the access unit.


The variable refLayerDQId is set equal to the value of MaxRefLayerDQId for the layer representation with DQId equal to
dqIdMax.


A list of integer values specifying layer representation identifiers for the access unit is derived by invoking the process
specified in clause G.8.1.1 with the output being the list dqIdList. The variable numDQEntries is set equal to the number
of elements in the list dqIdList.


A variable dependentDId is derived by the following pseudo-code:


dependentDId = 0
for( i = 0; i < numDQEntries; i++ )
if( ( dqIdList[ i ] % 16 ) = = 0 ) (G-365)
dependentDId++


The variable svcPicSizeInMbs is derived as follows:


- If numDQEntries is less than 3, svcPicSizeInMbs is set equal to PicSizeInMbs for the layer representation with DQId
equal to dqIdMax.


- Otherwise (numDQEntries is greater than 2), svcPicSizeInMbs is derived by applying the following ordered steps:


1. svcPicSizeInMbs is set equal to PicSizeInMbs for the layer representation with DQId equal to dqIdMax.


2. The variable refLayerMbs is set equal to 0.


3. For each element dqIdList[ i ] with i = 2..(numDQEntries − 1), with refLayerPicSizeInMbs being the

variable PicSizeInMbs for the layer representation with DQId equal to dqIdList[ i ], the variable
refLayerMbs is modified by


refLayerMbs += refLayerPicSizeInMbs (G-366)


4. svcPicSizeInMbs is modified by


svcPicSizeInMbs += ( refLayerMbs + 1 ) >> 1 (G-367)


Bitstreams conforming to the Scalable Baseline, Scalable Constrained Baseline, Scalable High, Scalable Constrained High,
or Scalable High Intra profiles at a specific level shall obey the following constraints:


a) The nominal removal time of access unit n with n > 0 from the CPB as specified in clause C.1.2, satisfies the

constraint that tr,n( n ) − tr( n − 1 ) is greater than or equal to Max( svcPicSizeInMbs ÷ MaxMBPS, fR ), where
MaxMBPS is the value specified in Table A-1 that applies to picture n − 1 and svcPicSizeInMbs is derived for
picture n − 1.


b) The difference between consecutive output times of pictures from the DPB as specified in clause C.2.2, satisfies

the constraint that  to,dpb( n ) >= Max( svcPicSizeInMbs ÷ MaxMBPS, fR ), where MaxMBPS is the value
specified in Table A-1 for picture n, and svcPicSizeInMbs is derived for picture n, provided that picture n is a
picture that is output and is not the last picture of the bitstream that is output.


c) PicWidthInMbs * FrameHeightInMbs <= MaxFS, where MaxFS is specified in Table A-1. PicWidthInMbs and

FrameHeightInMbs are the derived variables for the layer representation with DQId equal to dqIdMax.


d) PicWidthInMbs <= Sqrt( MaxFS * 8 ), where MaxFS is specified in Table A-1 and PicWidthInMbs is the derived

variable for the layer representation with DQId equal to dqIdMax.


e) FrameHeightInMbs <= Sqrt( MaxFS * 8 ), where MaxFS is specified in Table A-1 and FrameHeightInMbs is the

derived variable for the layer representation with DQId equal to dqIdMax.


f) max_dec_frame_buffering <= MaxDpbFrames, where MaxDpbFrames is equal to
Min( MaxDpbMbs / ( PicWidthInMbs * FrameHeightInMbs ), 16 ) and MaxDpbMbs is specified in Table A-1.
PicWidthInMbs and FrameHeightInMbs are the derived variables for the layer representation with DQId equal
to dqIdMax.





g) The vertical motion vector component range for luma motion vectors does not exceed the range from −MaxVmvR

to ( MaxVmvR − 0.25 ) in units of luma frame samples, where MaxVmvR is specified in Table A-1.


h) The horizontal motion vector component range for luma motion vectors does not exceed the following range:


     - If the level number is less than 6.0, the specified range is from −2048 to 2047.75, inclusive, in units of luma
samples.


     - Otherwise, the specified range is from −8192 to 8191.75, inclusive, in units of luma samples.


i) For each layer representation, the total number of motion vectors per two macroblocks mbAddrA and mbAddrB
with (mbAddrA + 1) equal to mbAddrB does not exceed MaxMvsPer2Mb, where MaxMvsPer2Mb is specified
in Table A-1 given the level that is indicated in the SVC sequence parameter set that is referenced by the layer
representation. The number of motion vectors for each macroblock is the value of the variable MvCnt after the
completion of the base decoding process for slices without resolution change specified in clause G.8.1.4.1 (when
SpatialResolutionChangeFlag is equal to 0) or after completion the base decoding process for slices with
resolution change specified in clause G.8.1.4.2 (when SpatialResolutionChangeFlag is equal to 1).

NOTE – Due to the constraint specified in clause G.8.8.1, the number of motion vectors for the layer representation
with DQId equal to 0 is additionally constrained as specified in Annex A.


j) The number of bits of macroblock_layer( ) and macroblock_layer_in_scalable_extension( ) data for any
macroblock in any layer representation is not greater than 128 + RawMbBits. Depending on
entropy_coding_mode_flag, the bits of macroblock_layer( ) data are counted as follows:


     - If entropy_coding_mode_flag is equal to 0, the number of bits of macroblock_layer( ) data is given by the
number of bits in the macroblock_layer( ) syntax structure for a macroblock.


     - Otherwise (entropy_coding_mode_flag is equal to 1), the number of bits of macroblock_layer( ) data for a
macroblock is given by the number of times read_bits( 1 ) is called in clauses 9.3.3.2.2 and 9.3.3.2.3 when
parsing the macroblock_layer( ) associated with the macroblock.


k) The variable dependentDId specified at the beginning of this clause shall not exceed 3.


l) For each layer representation present in an access unit that has MinNoInterLayerPredFlag equal to 0, the
following applies:


1. The variables numILIntraPredSamples and numRefLayerILIntraPredMbs are derived as specified in

the derivation process for variables related to inter-layer intra prediction in clause G.8.6.2.5 with DQId
being the input.


2. The following constraint shall be obeyed.


numRefLayerILIntraPredMbs * 256 <= 1.5 * numILIntraPredSamples (G-368)


m) When MaxRefLayerDQId is greater than or equal to 0 for a particular layer representation, the value of level_idc

in the SVC sequence parameter set that is referenced by the particular layer representation shall represent a level
that is greater than or equal to the level that is represented by the value of level_idc or (level_idc and
constraint_set3_flag) in the SVC sequence parameter set that is referenced by the layer representation with DQId
equal to MaxRefLayerDQId.


Table A-1 specifies the limits for each level. A definition of all levels identified in the "Level number" column of Table A-1
is specified for the Scalable Baseline, Scalable Constrained Baseline, Scalable High, Scalable Constrained High, and
Scalable High Intra profiles. Each entry in Table A-1 indicates, for the level corresponding to the row of the table, the
absence or value of a limit that is imposed by the variable corresponding to the column of the table, as follows:


- If the table entry is marked as "-", no limit is imposed by the value of the variable as a requirement of bitstream
conformance to the profile at the specified level.


- Otherwise, the table entry specifies the value of the variable for the associated limit that is imposed as a requirement
of bitstream conformance to the profile at the specified level.


In bitstreams conforming to the Scalable Baseline, Scalable Constrained Baseline, Scalable High,
Scalable Constrained High, or Scalable High Intra profiles, the conformance of the bitstream to a specified level is
indicated by the syntax element level_idc as follows:


- If level_idc is equal to 9, the indicated level is level 1b.


- Otherwise (level_idc is not equal to 9), level_idc is equal to a value of ten times the level number (of the indicated
level) specified in Table A-1.





**G.10.2.2Profile specific level limits**


The variable dqIdMax is set equal to the maximum value of DQId for the layer representation of the access unit.


A list of integer values specifying layer representation identifiers for the access unit is derived by invoking the process
specified in clause G.8.1.1 with the output being the list dqIdList. The variable numDQEntries is set equal to the number
of elements in the list dqIdList.


The variable numSVCSlices is derived as specified by the following pseudo-code:


numSVCSlices = 0
for( i = 0; i < numDQEntries; i++ ) (G-369)
numSVCSlices += number of slices in layer representation with DQId equal to dqIdList[ i ]


The variable svcPicSizeInMbs is derived as specified in clause G.10.2.1.


The following constraints are specified:


a) In bitstreams conforming to the Scalable Baseline, Scalable Constrained Baseline, Scalable High, Scalable

Constrained High, or Scalable High Intra profiles, the removal time of access unit 0 shall satisfy the constraint
that the numSVCSlices variable for picture 0 is less than or equal to
( Max( svcPicSizeInMbs, fR * MaxMBPS ) + MaxMBPS * ( tr( 0 ) − tr,n( 0 ) ) ) ÷ SliceRate, where MaxMBPS
and SliceRate are the values that apply to picture 0. MaxMBPS is specified in Table A-1. For Scalable Baseline
and Scalable Constrained Baseline profiles, SliceRate is specified in Table G-16. For Scalable High, Scalable
Constrained High, and Scalable High Intra profiles, SliceRate is specified in Table A-4.


b) In bitstreams conforming to the Scalable Baseline, Scalable Constrained Baseline, Scalable High, Scalable

Constrained High, or Scalable High Intra profiles, the difference between consecutive removal times of access
units n and n − 1 with n > 0 shall satisfy the constraint that the numSVCSlices variable for picture n is less than
or equal to MaxMBPS * ( tr( n ) − tr( n − 1 ) ) ÷ SliceRate, where MaxMBPS and SliceRate are the values that
apply to picture n. MaxMBPS is specified in Table A-1. For the Scalable Baseline and Scalable Constrained
Baseline profiles, SliceRate is specified in Table G-16. For the Scalable High, Scalable Constrained High, and
Scalable High Intra profiles, SliceRate is specified in Table A-4.


c) In bitstreams conforming to the Scalable High profile, SVC sequence parameter sets shall have

direct_8x8_inference_flag equal to 1 for the levels specified in Table A-4. In bitstreams conforming to the
Scalable Baseline profile, SVC sequence parameter sets shall have direct_8x8_inference_flag equal to 1.

NOTE 1 – direct_8x8_inference_flag is not relevant to the Scalable Constrained Baseline, Scalable Constrained
High, and Scalable High Intra profiles, as these profiles do not allow B or EB slice types.


d) In bitstreams conforming to the Scalable High or Scalable High Intra profiles, SVC sequence parameter sets shall

have frame_mbs_only_flag equal to 1 for the levels specified in Table A-4.


e) In bitstreams conforming to the Scalable High profile, for all macroblocks mbAddr and macroblock partitions

mbPartIdx, the value of subMbType[ mbAddr ][ mbPartIdx ] that is derived as specified in clause G.8.1.5.1.1
shall not be equal to B_Bi_8x4, B_Bi_4x8, or B_Bi_4x4 for the levels in which MinLumaBiPredSize is shown
as 8x8 in Table A-4. In bitstreams conforming to the Scalable Baseline profile, for all macroblocks mbAddr and
macroblock partitions mbPartIdx, the value of subMbType[ mbAddr ][ mbPartIdx ] that is derived as specified
in clause G.8.1.5.1.1 shall not be equal to B_Bi_8x4, B_Bi_4x8, or B_Bi_4x4.

NOTE 2 – The above constraint is not relevant to the Scalable Constrained Baseline, Scalable Constrained High,
and Scalable High Intra profiles, as these profiles do not allow B or EB slice types.


f) In bitstreams conforming to the Scalable Baseline or Scalable Constrained Baseline profiles,
( xIntmax − xIntmin + 6 ) * ( yIntmax − yIntmin + 6 ) <= MaxSubMbRectSize in macroblocks coded with macroblock
type equal to P_8x8, P_8x8ref0 or B_8x8 for all invocations of the process specified in clause 8.4.2.2.1 used to
generate the predicted luma sample array for a single reference picture list (reference picture list 0 or reference
picture list 1) for each 8x8 sub-macroblock with the macroblock partition index mbPartIdx, where
NumSubMbPart( sub_mb_type[ mbPartIdx ] ) > 1, where MaxSubMbRectSize is specified in Table G-16 and


     - xIntmin is the minimum value of xIntL among all luma sample predictions for the sub-macroblock,


     - xIntmax is the maximum value of xIntL among all luma sample predictions for the sub-macroblock,


     - yIntmin is the minimum value of yIntL among all luma sample predictions for the sub-macroblock,


     - yIntmax is the maximum value of yIntL among all luma sample predictions for the sub-macroblock.


g) In bitstreams conforming to the Scalable Baseline, Scalable Constrained Baseline, Scalable High, Scalable

Constrained High, or Scalable High Intra profiles, for the VCL HRD parameters,
BitRate[ SchedSelIdx ] <= cpbBrVclFactor * MaxBR and CpbSize[ SchedSelIdx ] <=





cpbBrVclFactor * MaxCPB for at least one value of SchedSelIdx, where cpbBrVclFactor is specified in
Table G-17. With vui_ext_vcl_hrd_parameters_present_flag[ i ] being the syntax element, in the SVC VUI
parameters extension of the active SVC sequence parameter set, that is associated with the VCL HRD parameters
that are used for conformance checking (as specified in Annex C), BitRate[ SchedSelIdx ] and
CpbSize[ SchedSelIdx ] are given as follows:


     - If vui_ext_vcl_hrd_parameters_present_flag[ i ] is equal to 1, BitRate[ SchedSelIdx ] and
CpbSize[ SchedSelIdx ] are given by Equations E-71 and E-72, respectively, using the syntax elements of
the hrd_parameters( ) syntax structure that immediately follows
vui_ext_vcl_hrd_parameters_present_flag[ i ].


     - Otherwise (vui_ext_vcl_hrd_parameters_present_flag[ i ] is equal to 0), BitRate[ SchedSelIdx ] and
CpbSize[ SchedSelIdx ] are inferred as specified in clause E.2.2 for VCL HRD parameters.


MaxBR and MaxCPB are specified in Table A-1 in units of cpbBrVclFactor bits/s and cpbBrVclFactor bits,
respectively. The bitstream shall satisfy these conditions for at least one value of SchedSelIdx in the range 0 to
cpb_cnt_minus1, inclusive.


h) In bitstreams conforming to the Scalable Baseline, Scalable Constrained Baseline, Scalable High, Scalable

Constrained High, or Scalable High Intra profiles, for the NAL HRD parameters,
BitRate[ SchedSelIdx ] <= cpbBrNalFactor * MaxBR and CpbSize[ SchedSelIdx ] <=
cpbBrNalFactor * MaxCPB for at least one value of SchedSelIdx, where cpbBrNalFactor is specified in
Table G-17. With vui_ext_nal_hrd_parameters_present_flag[ i ] being the syntax element, in the SVC VUI
parameters extension of the active SVC sequence parameter set, that is associated with the NAL HRD parameters
that are used for conformance checking (as specified in Annex C), BitRate[ SchedSelIdx ] and
CpbSize[ SchedSelIdx ] are given as follows:


     - If vui_ext_nal_hrd_parameters_present_flag[ i ] is equal to 1, BitRate[ SchedSelIdx ] and
CpbSize[ SchedSelIdx ] are given by Equations E-71 and E-72, respectively, using the syntax elements of
the hrd_parameters( ) syntax structure that immediately follows
vui_ext_nal_hrd_parameters_present_flag[ i ].


     - Otherwise (vui_ext_nal_hrd_parameters_present_flag[ i ] is equal to 0), BitRate[ SchedSelIdx ] and
CpbSize[ SchedSelIdx ] are inferred as specified in clause E.2.2 for NAL HRD parameters.


MaxBR and MaxCPB are specified in Table A-1 in units of cpbBrNalFactor bits/s and cpbBrNalFactor bits,
respectively. The bitstream shall satisfy these conditions for at least one value of SchedSelIdx in the range 0 to
cpb_cnt_minus1, inclusive.


i) In bitstreams conforming to the Scalable Baseline, Scalable Constrained Baseline, Scalable High, Scalable
Constrained High, or Scalable High Intra profiles, the sum of the NumBytesInNALunit variables for access unit
0 is less than or equal to 384 * ( Max( svcPicSizeInMbs, fR * MaxMBPS ) + MaxMBPS *
( tr( 0 ) − tr,n( 0 ) ) ) ÷ MinCR, where MaxMBPS and MinCR are the values specified in Table A-1 that apply to
picture 0.


j) In bitstreams conforming to the Scalable Baseline, Scalable Constrained Baseline, Scalable High, Scalable
Constrained High, or Scalable High Intra profiles, the sum of the NumBytesInNALunit variables for access unit
n with n > 0 is less than or equal to 384 * MaxMBPS * ( tr( n ) − tr( n − 1 ) ) ÷ MinCR, where MaxMBPS and
MinCR are the values specified in Table A-1 that apply to picture n.


k) In bitstreams conforming to the Scalable Baseline or Scalable Constrained Baseline profile, picture parameter

sets shall have entropy_coding_mode_flag equal to 0 and transform_8x8_mode_flag equal to 0 for level 2.1 and
below.


l) In bitstreams conforming to the Scalable Baseline, Scalable Constrained Baseline, Scalable High, Scalable
Constrained High, and Scalable High Intra profiles, when PicSizeInMbs is greater than 1620 for DQId equal to
dqIdMax, the number of macroblocks in any coded slice shall not exceed MaxFS / 4, where MaxFS is specified
in Table A-1.


Table A-4 specifies limits for each level that are specific to bitstreams conforming to the Scalable High, Scalable
Constrained High, and Scalable High Intra profiles. Table G-16 specifies limits for each level that are specific to bitstreams
conforming to the Scalable Baseline and Scalable Constrained Baseline profiles. Each entry in Tables A-4 and G-16
indicates, for the level corresponding to the row of the table, the absence or value of a limit that is imposed by the variable
corresponding to the column of the table, as follows:


- If the table entry is marked as "-", no limit is imposed by the value of the variable as a requirement of bitstream
conformance to the profile at the specified level.





- Otherwise, the table entry specifies the value of the variable for the associated limit that is imposed as a requirement
of bitstream conformance to the profile at the specified level.


**Table G-16 – Scalable Baseline and Scalable Constrained Baseline profile level limits**

|Level number|SliceRate|MaxSubMbRectSize|
|---|---|---|
|**1 **|-|576|
|**1b**|-|576|
|**1.1**|-|576|
|**1.2**|-|576|
|**1.3**|-|576|
|**2 **|-|576|
|**2.1**|22|576|
|**2.2**|22|576|
|**3 **|22|576|
|**3.1**|60|1152|
|**3.2**|60|1152|
|**4 **|60|1440|
|**4.1**|24|1440|
|**4.2**|24|1440|
|**5 **|24|-|
|**5.1**|24|-|
|**6 **|24|-|
|**6.1**|24|-|
|**6.2**|24|-|



**Table G-17 – Specification of cpbBrVclFactor and cpbBrNalFactor**



|Profile|cpbBrVclFactor|cpbBrNalFactor|
|---|---|---|
|**ScalableBaseline,**<br>**Scalable Constrained Baseline, **<br>**Scalable High,**<br>**Scalable Constrained High, or**<br>**Scalable High Intra**|1250|1500|
