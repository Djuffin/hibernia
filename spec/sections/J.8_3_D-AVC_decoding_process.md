**bvsp_flag_lX** [ mbPartIdx ] has the same semantics as bvsp_flag_lX[ mbPartIdx ] in clause J.7.4.6.1.


sub_mb_type shall be equal to P_L0_8x8 when both of the following apply:


- The macroblock type is equal to a P macroblock type.


- bvsp_flag_l0[ mbPartIdx ] is equal to 1.


sub_mb_type shall be equal to B_L0_8x8, B_L1_8x8, or B_Bi_8x8 when both of the following apply:


- The macroblock type is equal to a B macroblock type.


- bvsp_flag_l0[ mbPartIdx ] is equal to 1 or bvsp_flag_l1[ mbPartIdx ] is equal to 1.


**J.8** **3D-AVC decoding process**


This clause specifies the decoding process for an access unit of a coded video sequence conforming to one or more of the
profiles specified in Annex J. Specifically, this clause specifies how the decoded picture with multiple texture view
components and multiple depth view components is derived from syntax elements and global variables that are derived
from NAL units in an access unit when the decoder is decoding the operation point identified by the target temporal level
and the target output views.


The decoding process is specified such that all decoders shall produce numerically identical results for the target output
views. Any decoding process that produces identical results for the target output views to the process described here
conforms to the decoding process requirements of this Recommendation | International Standard.


Unless stated otherwise, the syntax elements and derived upper-case variables that are referred to by the decoding process
specified in this clause and all child processes invoked from the process specified in this clause are the syntax elements
and derived upper-case variables for the current access unit.


The target output texture and depth views are either specified by external means not specified in this Specification, or,
when not specified by external means, there shall be one target output texture view which is the base texture view.


NOTE – The association of VOIdx values to view_id values according to the decoding process of clause I.8 may differ from that
of the decoding process of clause H.8.


A target output view may include only a texture view, only a depth view, or both the texture view and the depth view,
which have the same view_id value.


All sub-bitstreams that can be derived using the sub-bitstream extraction process with pIdTarget equal to any value in the
range of 0 to 63, inclusive, tIdTarget equal to any value in the range of 0 to 7, inclusive, viewIdTargetList consisting of
any one or more viewIdTarget's identifying the views in the bitstream as inputs as specified in clause J.8.3 shall result in a
set of coded video sequences, with each coded video sequence conforming to one or more of the profiles specified in
Annex A, Annex H, Annex I and Annex J.


Let vOIdxList be a list of integer values specifying the VOIdx values of the view components of the access unit. The
variable VOIdxMax is set equal to the maximum value of the entries in the list vOIdxList, and the variable vOIdxMin is
set to the minimum value of the entries in the list vOIdxList. VOIdxMax shall be the same for all access units within a
coded video sequence. vOIdxMin shall be the same for all anchor access units within a coded video sequence. When the
current access unit is an anchor access unit, the variable VOIdxMin is set to vOIdxMin.


The 3D-AVC decoding process specified in this clause is repeatedly invoked for each texture and depth view component
with VOIdx from vOIdxMin to VOIdxMax, inclusive, which is present in the list vOIdxList, in increasing order of VOIdx
and in decoding order of texture or depth view components as specified in clause J.7.4.1.2.5.


Outputs of the multiview video decoding process are decoded samples of the current primary coded picture including all
decoded texture and depth view components of the target output texture and depth views.


For each texture view component and each depth view component, TextureFirstFlag is set equal to
( NumDepthViews = = 0 | | ( ViewCompOrder( 0, view_idx ) < ViewCompOrder( 1, view_idx ) ? 1 : 0 ) ), and the
specifications in clause I.8 apply, with the decoding process for reference picture lists construction being modified in
clause J.8.1. The 3D-AVC inter prediction, inter-view prediction, view synthesis prediction and inter prediction with
adaptive luminance compensation processes are specified in clause J.8.3. The decoding process for depth range parameters
is specified in clause J.8.4. Additionally, the specification of bitstream subsets is specified in clause J.8.2.


**J.8.1** **3D-AVC decoding process for reference picture lists construction**


The specifications of clause I.8.1 apply with the following additions:


- When DepthFlag is equal to 0, the variable VspRefExist is specified after applying clause H.8.2 as follows:





   - If seq_view_synthesis_flag is equal to 0 (view synthesis prediction is disabled), VspRefExist is set to 0.


   - Otherwise, if the current slice is a P or SP slice and there exists at least one inter-view reference component in
RefPicList0, or if the current slice is a B slice and there exists at least one inter-view reference component in
either RefPicList0 or RefPicList1, VspRefExist is set to 1;


   - Otherwise, VspRefExist is set to 0.


- The variable VspRefL0Flag[ mbPartIdx ] is specified as follows:


   - If VspRefExist is equal to 1, ref_idx_l0[ mbPartIdx ] is present, and ref_idx_l0[ mbPartIdx ] indicates an interview reference component, the variable VspRefL0Flag[ mbPartIdx ] is set to 1.


   - Otherwise, the variable VspRefL0Flag[ mbPartIdx ] is set to 0.


- The variable VspRefL1Flag[ mbPartIdx ] is specified as follows:


   - If VspRefExist is equal to 1, ref_idx_l1[ mbPartIdx ] is present, and ref_idx_l1[ mbPartIdx ] indicates an interview reference component, the variable VspRefL1Flag[ mbPartIdx ] is set to 1.


   - Otherwise, the variable VspRefL1Flag[ mbPartIdx ] is set to 0.


**J.8.2** **3D-AVC inter prediction, inter-view prediction, view synthesis prediction and adaptive luminance**
**compensation**


This process is invoked when decoding P and B macroblock types and when nal_unit_type is equal to 21.


Outputs of this process are Inter prediction samples for the current macroblock that are a 16x16 array predL of luma samples
and when ChromaArrayType is not equal to 0 two (MbWidthC)x(MbHeightC) arrays predCb and predCr of chroma samples,
one for each of the chroma components Cb and Cr.


When DepthFlag is equal to 0 and dmvp_flag or slice_vsp_flag is equal to 1, the variables DepthRefPicList0,
DepthRefPicList1 for B slices, and DepthCurrPic are specified as follows. The variable DepthRefPicList0 is specified to
consist of the depth view components of the view component pairs for which the texture view components are in
RefPicList0 in the order that RefPicList0[ i ] and DepthRefPicList0[ i ] form a view component pair for any value of
i = 0.. num_ref_idx_l0_active_minus1. The variable DepthRefPicList1 is specified for B slices to consist of the depth view
components of the view component pairs for which the texture view components are in RefPicList1 in the order that
RefPicList1[ i ] and DepthRefPicList1[ i ] form a view component pair for any value of
i = 0.. num_ref_idx_l1_active_minus1. The variable DepthCurrPic is specified to be the decoded sample array of the depth
view component of the view component pair for which the texture view component is the current texture view component
when TextureFirstFlag is equal to 0 and it is specified to be the decoded sample array of the depth view component of the
view component pair for which the texture view component is the texture view component of the base view when
TextureFirstFlag is equal to 1.


The partitioning of a macroblock is specified by mb_type. Each macroblock partition is referred to by mbPartIdx. When
the macroblock partitioning consists of partitions that are equal to sub-macroblocks, each sub-macroblock can be further
partitioned into sub-macroblock partitions as specified by sub_mb_type[ mbPartIdx ]. Each sub-macroblock partition is
referred to by subMbPartIdx. When the macroblock partitioning does not consist of sub-macroblocks, subMbPartIdx is set
equal to 0.


The following steps are specified for each macroblock partition or for each sub-macroblock partition.


The functions MbPartWidth( ), MbPartHeight( ), SubMbPartWidth( ), and SubMbPartHeight( ) describing the width and
height of macroblock partitions and sub-macroblock partitions are specified in Tables 7-14, 7-17, 7-18, and J-3. For the
decoding processes specified in this clause, its subclauses, and any subclauses invoked by the decoding processes specified
in this clause or its subclauses, references to Table 7-13 are replaced by references to Table J-3.


When nal_unit_type is equal to 21, DepthFlag is equal to 0, TextureFirstFlag is equal to 1, InterViewRefAvailable is equal
to 1 and either dmvp_flag or seq_view_synthesis_flag is equal to 1, DvMBX is set equal to zero when CurrMbAddr is
equal to first_mb_in_slice, and clause J.8.2.1.8 is invoked.


The range of the macroblock partition index mbPartIdx is derived as follows:


- If mb_type is equal to B_Skip or B_Direct_16x16, mbPartIdx proceeds over values 0..3.


- Otherwise (mb_type is not equal to B_Skip or B_Direct_16x16), mbPartIdx proceeds over values
0..NumMbPart( mb_type ) − 1.


For each value of mbPartIdx, the variables partWidth and partHeight for the width and height of each macroblock partition
or sub-macroblock partition in the macroblock are derived as follows:





- If mb_type is not equal to P_8x8, P_8x8ref0, B_Skip, B_Direct_16x16, or B_8x8, subMbPartIdx is set equal to 0 and
the following applies:


partWidth = MbPartWidth( mb_type ) (J-13)


partHeight = MbPartHeight( mb_type ) (J-14)


- Otherwise, if mb_type is equal to P_8x8 or P_8x8ref0, or mb_type is equal to B_8x8 and sub_mb_type[ mbPartIdx ]
is not equal to B_Direct_8x8, subMbPartIdx proceeds over values
0..NumSubMbPart( sub_mb_type[ mbPartIdx ] ) − 1, and partWidth and partHeight are derived as:


partWidth = SubMbPartWidth( sub_mb_type[ mbPartIdx ] ) (J-15)


partHeight = SubMbPartHeight( sub_mb_type[ mbPartIdx ] ). (J-16)


- Otherwise (mb_type is equal to B_Skip or B_Direct_16x16, or mb_type is equal to B_8x8 and
sub_mb_type[ mbPartIdx ] is equal to B_Direct_8x8), the following applies:


   - If either MbVSSkipFlag or mb_direct_type_flag is equal to 1, subMbPartIdx is set to 0 and partWidth and
partHeight are derived as:


partWidth = 8 (J-17)


partHeight = 8 (J-18)


   - Otherwise (both MbVSSkipFlag and mb_direct_type_flag are equal to 0), subMbPartIdx proceeds over values
0..3, and partWidth and partHeight are derived as:


partWidth = 4 (J-19)


partHeight = 4 (J-20)


When ChromaArrayType is not equal to 0, the variables partWidthC and partHeightC are derived as:


partWidthC = partWidth / SubWidthC (J-21)
partHeightC = partHeight / SubHeightC (J-22)


Let the variable MvCnt be initially set equal to 0 before any invocation of clause J.8.2.1, J.8.2.3 or 8.4.1 for the macroblock.


The Inter prediction process for a macroblock partition mbPartIdx and a sub-macroblock partition subMbPartIdx consists
of the following ordered steps:


1. The following applies:


   - If nal_unit_type is equal to 21 and DepthFlag is equal to 0, the following applies:


     - If mb_alc_skip_flag is equal to 1 or mb_alc_flag is equal to 1, clause J.8.2.3 is invoked.


     - Otherwise, if dmvp_flag or slice_vsp_flag is equal to 1, clause J.8.2.1 is invoked.


     - Otherwise, clause 8.4.1 is invoked.


   - Otherwise, the derivation process for motion vector components and reference indices as specified in clause 8.4.1
is invoked.


Inputs to the processes in clauses J.8.2.1, J.8.2.3 and 8.4.1 are:


   - a macroblock partition mbPartIdx,


   - a sub-macroblock partition subMbPartIdx.


Outputs of the processes in clauses J.8.2.1, J.8.2.3 and 8.4.1 are:


   - luma motion vectors mvL0 and mvL1 and when ChromaArrayType is not equal to 0, the chroma motion vectors
mvCL0 and mvCL1


   - reference indices refIdxL0 and refIdxL1


   - prediction list utilization flags predFlagL0 and predFlagL1





   - the sub-macroblock partition motion vector count subMvCnt.


2. The variable MvCnt is incremented by subMvCnt.


3. When (weighted_pred_flag is equal to 1 and (slice_type % 5) is equal to 0 or 3) or (weighted_bipred_idc is greater

than 0 and (slice_type % 5) is equal to 1), the following applies:


   - If mb_alc_skip_flag is equal to 1 or mb_alc_flag is equal to 1, clause J.8.2.4 is invoked.


   - Otherwise, the derivation process for prediction weights as specified in clause 8.4.3 is invoked.


Inputs to these processes in clauses 8.4.3 and J.8.2.4 are:


   - reference indices refIdxL0 and refIdxL1


   - prediction list utilization flags predFlagL0 and predFlagL1


Outputs of these processes in clauses 8.4.3 and J.8.2.4 are variables for weighted prediction logWDC, w0C, w1C, o0C,
o1C with C being replaced by L and, when ChromaArrayType is not equal to 0, Cb and Cr.


4. When (nal_unit_type is equal to 21 and depth_weighted_pred_flag is equal to 1 and (slice_type % 5) is equal to 0

or 3) or (depth_weighted_bipred_flag is equal to 1 and (slice_type % 5) is equal to 1), the derivation process for
prediction weights in depth-range-based weighted prediction in clause J.8.2.2 is invoked.


5. The decoding process for Inter prediction samples as specified in clause 8.4.2 is invoked.


Inputs to this process are:


   - a macroblock partition mbPartIdx,


   - a sub-macroblock partition subMbPartIdx,


   - variables specifying partition width and height for luma and chroma (if available), partWidth, partHeight,
partWidthC (if available), and partHeightC (if available),


   - luma motion vectors mvL0 and mvL1 and when ChromaArrayType is not equal to 0, the chroma motion vectors
mvCL0 and mvCL1,


   - reference indices refIdxL0 and refIdxL1,


   - prediction list utilization flags predFlagL0 and predFlagL1,


   - variables for weighted prediction logWDC, w0C, w1C, o0C, o1C with C being replaced by L and, when

ChromaArrayType is not equal to 0, Cb and Cr.


Outputs of this process are inter prediction samples (pred); which are a (partWidth)x(partHeight) array predPartL of
prediction luma samples and when ChromaArrayType is not equal to 0 two (partWidthC)x(partHeightC) arrays
predPartCr, and predPartCb of prediction chroma samples, one for each of the chroma components Cb and Cr.


For use in derivation processes of variables invoked later in the decoding process, the following assignments are made:


MvL0[ mbPartIdx ][ subMbPartIdx ] = mvL0 (J-23)


MvL1[ mbPartIdx ][ subMbPartIdx ] = mvL1 (J-24)


RefIdxL0[ mbPartIdx ] = refIdxL0 (J-25)


RefIdxL1[ mbPartIdx ] = refIdxL1 (J-26)


PredFlagL0[ mbPartIdx ] = predFlagL0 (J-27)


PredFlagL1[ mbPartIdx ] = predFlagL1 (J-28)


The location of the upper-left sample of the macroblock partition relative to the upper-left sample of the macroblock is
derived by invoking the inverse macroblock partition scanning process as described in clause 6.4.2.1 with mbPartIdx as
the input and ( xP, yP ) as the output.


The location of the upper-left sample of the sub-macroblock partition relative to the upper-left sample of the macroblock
partition is derived by invoking the inverse sub-macroblock partition scanning process as described in clause J.6.1 with
subMbPartIdx as the input and ( xS, yS ) as the output.





The macroblock prediction is formed by placing the macroblock or sub-macroblock partition prediction samples in their
correct relative positions in the macroblock, as follows.


The variable predL[ xP + xS + x, yP + yS + y ] with x = 0..partWidth − 1, y = 0..partHeight − 1 is derived by:


predL[ xP + xS + x, yP + yS + y ] = predPartL[ x, y ] (J-29)


When ChromaArrayType is not equal to 0, the variable predC with x = 0..partWidthC − 1, y = 0..partHeightC − 1, and C
in predC and predPartC being replaced by Cb or Cr is derived by:


predC[ xP / SubWidthC + xS / SubWidthC + x, yP / SubHeightC + yS / SubHeightC + y ] =predPartC[ x, y ]
(J-30)


**J.8.2.1** **Derivation process for motion vector components and reference indices**


Inputs to this process are:


- a macroblock partition mbPartIdx,


- a sub-macroblock partition subMbPartIdx.


Outputs of this process are:


- luma motion vectors mvL0 and mvL1 and when ChromaArrayType is not equal to 0, the chroma motion vectors
mvCL0 and mvCL1,


- reference indices refIdxL0 and refIdxL1,


- prediction list utilization flags predFlagL0 and predFlagL1,


- a motion vector count variable subMvCnt.


For the derivation of the variables mvL0 and mvL1 as well as refIdxL0 and refIdxL1, the following applies:


- If mb_type is equal to P_Skip, the following applies:


   - If MbVSSkipFlag is equal to 0, the following applies:


      - If nal_unit_type is equal to 21 and DepthFlag is equal to 0 and dmvp_flag is equal to 1, the depth-based
derivation process for luma motion vectors for skipped macroblock in P and SP slices in clause J.8.2.1.2 is
invoked with the output being the luma motion vectors mvL0 and reference indices refIdxL0, and
predFlagL0 is set equal to 1.


      - Otherwise (nal_unit_type is not equal to 21 or DepthFlag is equal to 1 or dmvp_flag is equal to 0), the
derivation process for luma motion vectors for skipped macroblock in P and SP slices in clause 8.4.1.1 is
invoked with the output being the luma motion vectors mvL0 and reference indices refIdxL0, and
predFlagL0 is set equal to 1.


   - Otherwise (MbVSSkipFlag is equal to 1), the derivation process for luma motion vectors for VSP skipped
macroblock in P and SP slices in clause J.8.2.1.3 is invoked with mbPartIdx as input and with the output being
the luma motion vectors mvL0 and reference indices refIdxL0, and predFlagL0 is set equal to 1.


   - mvL1 and refIdxL1 are marked as not available and predFlagL1 is set equal to 0. The motion vector count
variable subMvCnt is set equal to 1.


- Otherwise, if mb_type is equal to B_Skip or B_Direct_16x16 or sub_mb_type[ mbPartIdx ] is equal to B_Direct_8x8,
the following applies.


   - The variable vspFlag is specified as follows:


vspFlag = !( sub_type[ mbPartIdx ] = = B_Direct_8x8 | |
( mb_type = = B_Skip && MbVSSkipFlag = = 0 ) | | (J-31)
( mb_type = = B_Direct_16x16 && !mb_direct_type_flag ) )


   - If vspFlag is equal to 0 and nal_unit_type is equal to 21 and DepthFlag is equal to 0 and dmvp_flag is equal to
1, the depth-based derivation process for luma motion vectors for B_Skip, B_Direct_16x16, and B_Direct_8x8
in B slices in clause J.8.2.1.4 is invoked with mbPartIdx and subMbPartIdx as the input and the output being the
luma motion vectors mvL0, mvL1, the reference indices refIdxL0, refIdxL1, the motion vector count variable
subMvCnt, and the prediction utilization flags predFlagL0 and predFlagL1.


   - Otherwise, if both of the following are true:





      - vspFlag is equal to 0, and


      - nal_unit_type is not equal to 21 or DepthFlag is equal to 1 or dmvp_flag is equal to 0,


the derivation process for luma motion vectors for B_Skip, B_Direct_16x16, and B_Direct_8x8 in B slices in

clause 8.4.1.2 is invoked with mbPartIdx and subMbPartIdx as the input and the output being the luma motion
vectors mvL0, mvL1, the reference indices refIdxL0, refIdxL1, the motion vector count variable subMvCnt, and
the prediction utilization flags predFlagL0 and predFlagL1.


   - Otherwise (vspFlag is equal to 1), the derivation process in clause J.8.2.1.6 is invoked with with mbPartIdx as
input and with the output being the luma motion vectors mvL0 and mvL1 and reference indices refIdxL0 and
refIdxL1.


- Otherwise, for X being replaced by either 0 or 1 in the variables predFlagLX, mvLX, refIdxLX, and in Pred_LX and
in the syntax elements ref_idx_lX and mvd_lX, the following applies:


1. The variables refIdxLX and predFlagLX are derived as follows:

       - If MbPartPredMode( mb_type, mbPartIdx ) or SubMbPredMode( sub_mb_type[ mbPartIdx ] ) is equal
to Pred_LX or to BiPred,


refIdxLX = ref_idx_lX[ mbPartIdx ] (J-32)


predFlagLX = 1 (J-33)


       - Otherwise, the variables refIdxLX and predFlagLX are specified by


refIdxLX = −1 (J-34)


predFlagLX = 0 (J-35)


2. The motion vector count variable subMvCnt is set equal to predFlagL0 + predFlagL1.


3. The variable currSubMbType is derived as follows:

       - If the macroblock type is equal to B_8x8, currSubMbType is set equal to sub_mb_type[ mbPartIdx ].

       - Otherwise (the macroblock type is not equal to B_8x8), currSubMbType is set equal to "na".


4. The following applies:

       - If VspRefLXFlag[ mbPartIdx ] is equal to 0 or both VspRefLXFlag[ mbPartIdx ] is equal to 1 and
bvsp_flag_lX[ mbPartIdx ] is equal to 0, the following applies:


        - When predFlagLX is equal to 1 and DepthFlag is equal to 0 and dmvp_flag is equal to 1, the derivation
process for luma motion vector prediction in clause J.8.2.1.7 is invoked with mbPartIdx subMbPartIdx,
refIdxLX, and currSubMbType as the inputs and the output being mvpLX.


        - When predFlagLX is equal to 1 and either DepthFlag is equal to 1 or dmvp_flag is equal to 0, the
derivation process for luma motion vector prediction in clause 8.4.1.3 is invoked with mbPartIdx
subMbPartIdx, refIdxLX, and currSubMbType as the inputs and the output being mvpLX.


        - The luma motion vectors are derived by


mvLX[ 0 ] = mvpLX[ 0 ] + mvd_lX[ mbPartIdx ][ subMbPartIdx ][ 0 ] (J-36)


mvLX[ 1 ] = mvpLX[ 1 ] + mvd_lX[ mbPartIdx ][ subMbPartIdx ][ 1 ] (J-37)


       - Otherwise (VspRefLXFlag[ mbPartIdx ] is equal to 1 and bvsp_flag_lX[ mbPartIdx ] is equal to 1), the
following applies:


        - If TextureFirstFlag is equal to 0, the depth-based disparity value derivation process in clause J.8.2.1.1
is invoked with depthPic equal to DepthCurrPic, (textureX, textureY) equal to the location of the topleft sample of macroblock partition mbPartIdx, tBlWidth equal to the width the macroblock partition
mbPartIdx, tBlHeight equal to the height the macroblock partition mbPartIdx, srcViewId equal to
view_id and refViewId equal to the view_id of refIdxLX as inputs and the output assigned to
mvLX[ 0 ] and mvLX[ 1 ] is set equal to 0.


        - Otherwise (TextureFirstFlag is equal to 1), the depth-based disparity value derivation process in
clause J.8.2.1.1 is invoked with depthPic equal to DepthCurrPic, (textureX, textureY) equal to sum of





( DvMBX >> 2, 0 ) and the location of the top-left sample of current macroblock partition, tBlWidth
equal to the width the macroblock partition mbPartIdx, tBlHeight equal to the height the macroblock
partition mbPartIdx, srcViewId equal to view_id and refViewId equal to 0 as inputs and the output
assigned to mvLX[ 0 ] and mvLX[ 1 ] is set equal to 0.


When ChromaArrayType is not equal to 0 and predFlagLX (with X being either 0 or 1) is equal to 1, the derivation process
for chroma motion vectors in clause 8.4.1.4 is invoked with mvLX and refIdxLX as input and the output being mvCLX.


**J.8.2.1.1** **Depth-based disparity value derivation process**


Inputs to this process are:


- a decoded depth view component depthPic,


- the location (textureX, textureY) of the block in a texture view component for which the disparity value is derived,


- the width tBlWidth and the height tBlHeight of the block in a texture view component for which the disparity value is
derived,


- the view_id value srcViewId of the texture view component for which the disparity value is derived, and


- the view_id value refViewId of the reference view for the disparity value.


Output of this process is a disparity value dispVal.


The derivation of the disparity value dispVal is specified with the following ordered steps.


1. The variables depthX, depthY, blWidth and blHeight are specified as follows:


dHM = depth_hor_mult_minus1 + 1
dVM = depth_ver_mult_minus1 + 1
depthX = Clip3( DepthCropLeftCoord, DepthCropRightCoord,
( ( textureX + grid_pos_x[ srcViewId ] )* dHM ) >> depth_hor_rsh )
depthY = Clip3( DepthCropTopCoord, DepthCropBottomCoord,
( ( textureY + grid_pos_y[ srcViewId ] ) * dVM ) >> depth_ver_rsh )
depthXN = Clip3( DepthCropLeftCoord, DepthCropRightCoord,
( ( textureX + grid_pos_x[ srcViewId ] + tBlWidth − 1 ) * dHM ) >> depth_hor_rsh ) (J-38)
depthYN = Clip3( DepthCropTopCoord, DepthCropBottomCoord,
( ( textureY + grid_pos_y[ srcViewId ] + tBlHeight − 1 ) * dVM ) >> depth_ver_rsh )
blWidth = depthXN − depthX + 1
blHeight = depthYN − depthY + 1


2. The variable maxDepth is specified as follows:


maxDepth = INT_MIN
for( j = 0; j < blHeight; j += ( blHeight − 1 ) )
for( i = 0; i < blWidth; i += ( blWidth − 1 ) ) (J-39)
if( depthPic[ depthX + i, depthY + j ] > maxDepth )
maxDepth = depthPic[ depthX + i, depthY + j ]


3. The variable dispVal is specified as follows:


log2Div = BitDepthY + 6
srcIndex = ViewIdTo3DVAcquisitionParamIndex( srcViewId )
refIndex = ViewIdTo3DVAcquisitionParamIndex( refViewId ) (J-40)
dispVal = ( NdrInverse[ maxDepth ] * DisparityScale[ dps_id ][ srcIndex ][ refIndex ] +
( DisparityOffset[ dps_id ][ srcIndex ][ refIndex ] << BitDepthY ) +
( 1 << ( log2Div − 1 ) ) ) >> log2Div


**J.8.2.1.2** **Depth-based derivation process for luma motion vectors for skipped macroblocks in P and SP slices**


This process is invoked when mb_type is equal to P_Skip, nal_unit_type is equal to 21, DepthFlag is equal to 0, dmvp_flag
is equal to 1 and MbVSSkipFlag is equal to 0.


Outputs of this process are:


- the motion vector mvL0,


- the reference index refIdxL0.





For the derivation of the motion vector mvL0 and refIdxL0 of a P_Skip macroblock type, the following ordered steps are
specified:


1. The process specified in clause J.8.2.1.5 is invoked with mbPartIdx set equal to 0, subMbPartIdx set equal to 0,

currSubMbType set equal to "na", and listSuffixFlag equal to 0 as input and the output is assigned to the motion
vector mvL0 and the reference index refIdxL0.


2. When refIdxL0 is equal to -1, the following applies:


      - The reference index refIdxL0 is set to 0.


      - The derivation process for luma motion vector prediction in clause J.8.2.1.7 is invoked with mbPartIdx set

equal to 0, subMbPartIdx set equal to 0, refIdxL0, and currSubMbType = "na" as the inputs and the output
being mvL0.


**J.8.2.1.3** **Derivation process for luma motion vectors for VSP skipped macroblocks in P and SP slices**


This process is invoked when mb_type is equal to P_Skip, nal_unit_type is equal to 21, DepthFlag is equal to 0, and
MbVSSkipFlag is equal to 1.


Inputs to this process are current macroblock partition index mbPartIdx.


Outputs of this process are the motion vector mvL0 and the reference index refIdxL0.


The inverse macroblock scanning process as specified in clause 6.4.1 is invoked with CurrMbAddr as the input and the
output is assigned to ( x1, y1 ).


The inverse macroblock partition scanning process specified in clause 6.4.2.1 is invoked with mbPartIdx as the input and
the output assigned to ( dx1, dy1 ).


The reference index refIdxL0 for a VSP skipped macroblock is derived as the inter-view picture that appears first in
RefPicList0.


If TextureFirstFlag is equal to 0, the variable refViewId is set equal to the view_id of the inter-view picture refIdxL0.
Otherwise (TextureFirstFlag is equal to1) the variable refViewId is set to 0.


The variable shiftedX is set to ( TextureFirstFlag ? ( DvMBX >> 2 ) : 0 ).


The depth-based disparity value derivation process in clause J.8.2.1.1 is invoked with depthPic equal to DepthCurrPic,
textureX equal to x1 + dx1+ shiftedX, textureY equal to y1 + dy1, tBlWidth equal to 8, tBlHeight equal to 8, srcViewId
equal to view_id and refViewId equal to refViewId as inputs and the output assigned to mvL0[ 0 ].


mvL0[ 1 ] is set equal to 0.


**J.8.2.1.4** **Derivation process for luma motion vectors for B_Skip, B_Direct_16x16, and B_Direct_8x8**


Inputs to this process are current macroblock partition index mbPartIdx and subMbPartIdx.


Outputs of this process are the reference indices refIdxL0, refIdxL1, the motion vectors mvL0 and mvL1, the motion vector
count variable subMvCnt, and the prediction list utilization flags, predFlagL0 and predFlagL1.


For the derivation of output, the following ordered steps are specified:


1. Let the variable currSubMbType be set equal to sub_mb_type[ mbPartIdx ].


2. The process specified in clause J.8.2.1.5 is invoked with mbPartIdx set equal to 0, subMbPartIdx set equal to 0,

currSubMbType and listSuffixFlag set equal to 0 as input and the output is assigned to the motion vector mvL0
and the reference index refIdxL0.


3. The process specified in clause J.8.2.1.5 is invoked with mbPartIdx set equal to 0, subMbPartIdx set equal to 0,

currSubMbType and listSuffixFlag set equal to 1 as input and the output is assigned to the motion vector mvL1
and the reference index refIdxL1.


4. When both reference indices refIdxL0 and refIdxL1 are equal to -1, the following applies:


      - The reference index refIdxL0 is set equal to 0.


      - The derivation process for luma motion vector prediction in clause J.8.2.1.7 is invoked with mbPartIdx set

equal to 0, subMbPartIdx set equal to 0, refIdxLX (with X being 0 or 1), and currSubMbType as the inputs
and the output being mvLX.


**J.8.2.1.5** **Derivation process for the motion vector in inter-view reference**


Inputs to this process are mbPartIdx, subMbPartIdx, and listSuffixFlag.





Outputs of this process are the motion vector mvCorrespond and the reference index refIdxCorrespond.


Inter-view reference picture InterViewPic and an offset vector dV are derived as follows:


- If TextureFirstFlag is equal to 0, the following ordered steps apply:


   - The inverse macroblock scanning process as specified in clause 6.4.1 is invoked with CurrMbAddr as the input
and the output is assigned to ( x1, y1 ).


   - The inverse macroblock partition scanning process specified in clause 6.4.2.1 is invoked with mbPartIdx as the
input and the output assigned to ( dx1, dy1 ).


   - The inverse sub-macroblock partition scanning process specified in clause J.6.1 is invoked with mbPartIdx and
subMbPartIdx as the input and the output assigned to ( dx2, dy2 ).


   - The following applies to derive an inter-view reference picture or inter-view only reference picture InterViewPic
and to set the variable interViewAvailable:


interViewAvailable = 0
for( cIdx = 0; cIdx <= num_ref_idx_l0_active_minus1 && !interViewAvailable; cIdx++ )
if ( view order index of RefPicList0[ cIdx ] is not equal to view_idx ) {
InterViewPic = RefPicList0[ cIdx ] (J-41)
interViewAvailable = 1
}


   - When interViewAvailable is equal to 1, the depth-based disparity value derivation process in clause J.8.2.1.1 is
invoked with depthPic equal to DepthCurrPic, textureX equal to x1 + dx1 + dx2, textureY equal to y1 + dy1 +
dy2, tBlWidth equal to the width the sub-macroblock partition CurrMbAddr\mbPartIdx\subMbPartIdx,
tBlHeight equal to the height the sub-macroblock partition CurrMbAddr\mbPartIdx\subMbPartIdx, srcViewId
equal to view_id and refViewId equal to the view_id of InterViewPic as inputs and the output assigned to dV[ 0 ]
and dV[ 1 ] is set to 0.


- Otherwise (TextureFirstFlag is equal to 1), the following ordered steps apply:


   - dV is set to (DvMBX, 0) and interViewAvailable is set to InterViewRefAvailable.


   - When interViewAvailable is equal to 1, InterViewPic is set to be the texture view component of the base view.


The refIdxCorrespond and mvCorrespond are set as follows:


- If interViewAvailable is equal to 0, refIdxCorrespond is set to -1, and mvCorrespond[ 0 ] and mvCorrespond[ 1 ] are
both set to 0.


- Otherwise, the following step applies in order.


   - The variable xCorrespond is set equal to x1 + 7 + ( dV[ 0 ] >> 2 ), and the variable yCorrespond is set equal to

y1 + 7 + ( dV[ 1 ] >> 2 ).


   - The variable mbAddrCorrespond is set equal to ( ( CurrMbAddr / PicWidthInMbs ) + ( dV[ 1 ] >> 6 ) ) *

PicWidthInMbs + ( CurrMbAddr % PicWidthInMbs ) + ( dV[ 0 ] >> 6 ).


   - The variable xRelative is set equal to xCorrespond − ( ( xCorrespond >> 4 ) << 4 ), and the variable yRelative is

set equal to yCorrespond − ( ( yCorrespond >> 4 ) << 4 ).


   - Set mbTypeCorrespond to the syntax element mb_type of the macroblock with address mbAddrCorrespond

inside the picture InterViewPic. When mbTypeCorrespond is equal to P_8x8, P_8x8ref0, or B_8x8,
subMbTypeCorrespond is set to be the syntax element sub_mb_type of the macroblock with address
mbAddrCorrespond inside the picture InterViewPic.


   - Set mbPartIdxCorrespond to the macroblock partition index of the corresponding partition and

subMbPartIdxCorrespond to the sub-macroblock partition index of the corresponding sub-macroblock partition.
The derivation process for macroblock and sub-macroblock partition indices as specified in clause 6.4.13.4 is
invoked with the luma location equal to ( xRelative, yRelative ), the macroblock type equal to
mbTypeCorrespond, and when mbTypeCorrespond is equal to P_8x8, P_8x8ref0, or B_8x8, the list of submacroblock types subMbTypeCorrespond as the inputs and the outputs are the macroblock partition index
mbPartIdxCorrespond and the sub-macroblock partition index subMbPartIdxCorrespond.


   - The motion vector mvCorrespond and the reference index refIdxCorrespond are derived as follows:


        - If the macroblock mbAddrCorrespond is coded as Intra prediction mode, both components of
mvCorrespond are set equal to 0 and refIdxCorrespond is set equal to −1.





        - Otherwise (the macroblock mbAddrCorrespond is not coded as Intra prediction mode), the prediction
utilization flags predFlagLXCorrespond is set equal to PredFlagLX[ mbPartIdxCorrespond ], the
prediction utilization flag of the macroblock partition mbAddrCorrespond\mbPartIdxCorrespond of
the picture InterViewPic. In addition, the following applies.


           - When predFlagLXCorrespond is equal to 1 and RefIdxLX[ mbPartIdxCorrespond ] is less than
or equal to num_ref_idx_lX_active_minus1, the mvCorrespond and the reference index
refIdxCorrespond are set equal to MvLX[ mbPartIdxCorrespond ][ subMbPartIdxCorrespond ]
and RefIdxLX[ mbPartIdxCorrespond ], respectively, which are the motion vector mvLX and the
reference index refIdxLX that have been assigned to the (sub-)macroblock partition
mbAddrCorrespond\mbPartIdxCorrespond\subMbPartIdxCorrespond inside the picture
InterViewPic.


**J.8.2.1.6** **Derivation process for luma motion vectors for VSP skipped/direct macroblocks in B slices**


Inputs to this process are current macroblock partition index mbPartIdx.


Outputs of this process are the motion vector mvL0, mvL1 and the reference index refIdxL0, refIdxL1.


The inverse macroblock scanning process as specified in clause 6.4.1 is invoked with CurrMbAddr as the input and the
output is assigned to ( x1, y1 ).


The inverse macroblock partition scanning process specified in clause 6.4.2.1 is invoked with mbPartIdx as the input and
the output assigned to ( dx1, dy1 ).


The reference index refIdxLX for a VSP skipped/direct macroblock is derived as the inter-view reference component that
appears first in the reference picture list X, with X being replaced by 0 or 1. When there is no inter-view picture in the
reference picture list X, refIdxLX is set equal to 0.


The variable refViewIdX is set equal to the view_id of the inter-view reference component refIdxLX if TextureFirstFlag
is equal to 0 and set equal to 0 otherwise.


The variable shiftedX is set to ( TextureFirstFlag ? ( DvMBX >> 2 ) : 0 ).


The motion vector mvLX, with X being replaced by 0 or 1, is derived as follows:


- The depth-based disparity value derivation process in clause J.8.2.1.1 is invoked with depthPic equal to DepthCurrPic,
textureX equal to x1 + dx1 + shiftedX, textureY equal to y1 + dy1, tBlWidth equal to the width the macroblock
partition mbPartIdx, tBlHeight equal to the height the macroblock partition mbPartIdx, srcViewId equal to view_id
and refViewId equal to the refViewIdX as inputs and the output assigned to mvLX[ 0 ].


- mvLX[ 1 ] is set equal to 0.


**J.8.2.1.7** **Derivation process for luma motion vector prediction**


Inputs to this process are:


- the macroblock partition index mbPartIdx,


- the sub-macroblock partition index subMbPartIdx,


- the reference index of the current partition refIdxLX (with X being 0 or 1),


- the variable currSubMbType.


Output of this process is the prediction mvpLX of the motion vector mvLX (with X being 0 or 1).


The specifications of clause 8.4.1.3 apply with the following changes.


- The following additional sentence is applied


      - If refIdxLX is not equal to refIdxLXN for any N = A, B, or C and X equal to 0 or 1, the following applies:


mbAddrN\mbPartIdxN\subMbPartIdxN is marked as not available
refIdxLXN = -1
mvLXN 0 ] = 0 (J-42)
mvLXN[ 1 ] = 0


after the following paragraph in clause 8.4.1.3:


      - The derivation process for the neighbouring blocks for motion data in clause 8.4.1.3.2 is invoked with

mbPartIdx, subMbPartIdx, currSubMbType, and listSuffixFlag = X (with X being 0 or 1 for refIdxLX being





refIdxL0 or refIdxL1, respectively) as the input and with mbAddrN\mbPartIdxN\subMbPartIdxN, reference
indices refIdxLXN and the motion vectors mvLXN with N being replaced by A, B, or C as the output


- The following additional sentence is applied


      - Otherwise, if refIdxLX is a reference index to an inter-view reference component or an inter-view only

reference component, the depth-based derivation process for median luma motion vector prediction in
clause J.8.2.1.7.1 is invoked with mbAddrN\mbPartIdxN\subMbPartIdxN, mvLXN, refIdxLXN with N
being replaced by A, B, or C, and refIdxLX as the inputs and the output is assigned to the motion vector
predictor mvpLX.


      - Otherwise, if refIdxLX is a reference index to a reference picture which is not an inter-view reference

component or an inter-view only reference component, the depth-based derivation process for median luma
temporal motion vector prediction in clause J.8.2.1.7.2 is invoked with
mbAddrN\mbPartIdxN\subMbPartIdxN, mvLXN, refIdxLXN with N being replaced by A, B, or C, and
refIdxLX as the inputs and the output is assigned to the motion vector predictor mvpLX.


after the following paragraph in clause 8.4.1.3


      - Otherwise, if MbPartWidth( mb_type ) is equal to 8, MbPartHeight( mb_type ) is equal to 16, mbPartIdx is

equal to 1, and refIdxLXC is equal to refIdxLX, the motion vector predictor mvpLX is set equal to mvLXC.


**J.8.2.1.7.1** **Depth-based derivation process for median luma motion vector prediction**


Inputs to this process are:


- the neighbouring partitions mbAddrN\mbPartIdxN\subMbPartIdxN (with N being replaced by A, B, or C),


- the motion vectors mvLXN (with N being replaced by A, B, or C) of the neighbouring partitions,


- the reference indices refIdxLXN (with N being replaced by A, B, or C) of the neighbouring partitions,,


- the reference index refIdxLX of the current partition.


Output of this process is the motion vector prediction mvpLX.


When either partition mbAddrN\mbPartIdxN\subMbPartIdxN is not available or refIdxLXN is not equal to refIdxLX,
mvLXN is derived as specified by the following:


  - If TextureFirstFlag is equal to 0, the following steps apply in order:


1. The inverse macroblock scanning process as specified in clause 6.4.1 is invoked with CurrMbAddr as the input

and the output is assigned to ( x1, y1 ).


2. The inverse macroblock partition scanning process specified in clause 6.4.2.1 is invoked with mbPartIdx as the

input and the output assigned to ( dx1, dy1 ).


3. The inverse sub-macroblock partition scanning process specified in clause J.6.1 is invoked with mbPartIdx and

subMbPartIdx as the input and the output assigned to ( dx2, dy2 ).


4. The modification process of inter-view motion vector in median luma motion vector prediction as specified in

clause J.8.2.1.7.1.1 is invoked with depthPic being equal to DepthRefPicList0[ refIdxL0 ], mbx1 being equal to
x1 and mby1 being equal to y1 as inputs and the output is assigned to the motion vector mvLXN.


  - Otherwise (TextureFirstFlag is equal to 1), mvLXN is set equal to ( DvMBX, 0 ).


Each component of the motion vector prediction mvpLX is given by the median of the corresponding vector components
of the motion vector mvLXA, mvLXB, and mvLXC:


mvpLX[ 0 ] = Median( mvLXA[ 0 ], mvLXB[ 0 ], mvLXC[ 0 ] ) (J-43)
mvpLX[ 1 ] = Median( mvLXA[ 1 ], mvLXB[ 1 ], mvLXC[ 1 ] ) (J-44)


**J.8.2.1.7.1.1** **Modification process for inter view motion vector in median luma motion vector prediction**


Inputs to this process are:


- depth reference view component depthPic,


- the location of a top-left sample ( mbx1, mby1 ) of the current macroblock.


Output of this process is the motion vector mv.


Let refViewId be the view_id value of depthPic.





The variable mv is derived as follows:


- The depth-based disparity value derivation process in clause J.8.2.1.1is invoked with depthPic equal to DepthCurrPic,
textureX equal to mbx1, textureY equal to mby1, tBlWidth equal to 16, tBlHeight equal to 16, srcViewId equal to
view_id and refViewId equal to the refViewId as inputs and the output assigned to mv[ 0 ].


- mv[ 1 ] is set equal to 0.


**J.8.2.1.7.2** **Depth-based derivation process for median luma temporal motion vector prediction**


Inputs to this process are:


- the neighbouring partitions mbAddrN\mbPartIdxN\subMbPartIdxN (with N being replaced by A, B, or C),


- the motion vectors mvLXN (with N being replaced by A, B, or C) of the neighbouring partitions,


- the reference indices refIdxLXN (with N being replaced by A, B, or C) of the neighbouring partitions,


- the reference index refIdxLX of the current partition.


Output of this process is the motion vector prediction mvpLX.


When either partition mbAddrN\mbPartIdxN\subMbPartIdxN is not available or refIdxLXN is not equal to refIdxLX,
mvLXN is derived as specified by the following ordered steps:


1. When TextureFirstFlag is equal to 0, the inverse macroblock scanning process as specified in clause 6.4.1 is
invoked with CurrMbAddr as the input and the output is assigned to ( x1, y1 ).


2. When TextureFirstFlag is equal to 0, the inverse macroblock partition scanning process specified in clause 6.4.2.1
is invoked with mbPartIdx as the input and the output assigned to ( dx1, dy1 ).


3. When TextureFirstFlag is equal to 0, the inverse sub-macroblock partition scanning process specified in
clause J.6.1 is invoked with mbPartIdx and subMbPartIdx as the input and the output assigned to ( dx2, dy2 ).


4. When TextureFirstFlag is equal to 0, the process specified in clause J.8.2.1.7.2.1 is invoked with depthPic set to
DepthCurrPic, mbx1 set to x1, mby1 set to y1 and listSuffixFlag as input and InterViewPic, an offset vector dV
and a variable interViewAvailable as outputs.


5. When TextureFirstFlag is equal to 1, dV is set equal to ( DvMBX, 0 ) and a variable interViewAvailable is set
equal to InterViewRefAvailable.


6. The refIdxCorrespond and mvCorrespond are set as follows:


      - If interViewAvailable is equal to 0, refIdxCorrespond is set to -1, and mvCorrespond[ 0 ] and

mvCorrespond[ 1 ] are both set to 0.


      - Otherwise, the following steps apply in order.


        - The variable luma4x4BlkIdx is set equal to ( 4 * mbPartIdx + subMbPartIdx ).


        - The inverse 4x4 luma block scanning process as specified in clause 6.4.3 is invoked with
luma4x4BlkIdx as the input and ( x, y ) as the output. In addition, ( xCorrespond, yCorrespond ) is set
equal to ( x + ( dV[ 0 ] >> 4 ), y + ( dV[ 1 ] >> 4 ) ) and mbAddrCorrespond is set equal to
( ( CurrMbAddr / PicWidthInMbs ) + ( dV[ 1 ] >> 6 ) ) * PicWidthInMbs + ( CurrMbAddr %
PicWidthInMbs ) + ( dV[ 0 ] >> 6 ).


        - Set mbTypeCorrespond to the syntax element mb_type of the macroblock with address
mbAddrCorrespond inside the picture InterViewPic. When mbTypeCorrespond is equal to P_8x8,
P_8x8ref0, or B_8x8, subMbTypeCorrespond is set to be the syntax element sub_mb_type of the
macroblock with address mbAddrCorrespond inside the picture InterViewPic.


        - Set mbPartIdxCorrespond to the macroblock partition index of the corresponding partition and
subMbPartIdxCorrespond to the sub-macroblock partition index of the corresponding sub-macroblock
partition. The derivation process for macroblock and sub-macroblock partition indices as specified in
clause 6.4.13.4 is invoked with the luma location equal to ( xCorrespond, yCorrespond ), the
macroblock type equal to mbTypeCorrespond, and when mbTypeCorrespond is equal to P_8x8,
P_8x8ref0, or B_8x8, the list of sub-macroblock types subMbTypeCorrespond as the inputs and the
outputs are the macroblock partition index mbPartIdxCorrespond and the sub-macroblock partition
index subMbPartIdxCorrespond.


        - The motion vector mvCorrespond and the reference index refIdxCorrespond are derived as follows:





           - If the macroblock mbAddrCorrespond is coded as Intra prediction mode, both components of

mvCorrespond are set equal to 0 and refIdxCorrespond is set equal to −1.


           - Otherwise (the macroblock mbAddrCorrespond is not coded as Intra prediction mode), the

prediction utilization flags predFlagLXCorrespond is set equal to
PredFlagLX[ mbPartIdxCorrespond ], the prediction utilization flag of the macroblock partition
mbAddrCorrespond\mbPartIdxCorrespond of the picture InterViewPic. In addition, the following
applies.


             - When predFlagLXCorrespond is equal to 1, the mvCorrespond and the reference index
refIdxCorrespond are set equal to
MvLX[ mbPartIdxCorrespond ][ subMbPartIdxCorrespond ] and
RefIdxLX[ mbPartIdxCorrespond ], respectively, which are the motion vector mvLX and
the reference index refIdxLX that have been assigned to the (sub-)macroblock partition
mbAddrCorrespond\mbPartIdxCorrespond\subMbPartIdxCorrespond inside the picture
InterViewPic.


7. The motion vectors mvLXN is derived as follows:


      - If refIdxCorrespond is equal to refIdxLX, the following applies:


mvLXN[ 0 ] = mvCorrespond[ 0 ]
mvLXN[ 1 ] = mvCorrespond[ 1 ] (J-45)


      - Otherwise, the following applies:


mvLXN[ 0 ] = 0
mvLXN[ 1 ] = 0


8. The following applies for the derivation of mvpLX[ 0 ] and mvpLX[ 1 ]:


mvpLX[ 0 ] = Median( mvLXA[ 0 ], mvLXB[ 0 ], mvLXC[ 0 ] ) (J-46)
mvpLX[ 1 ] = Median( mvLXA[ 1 ], mvLXB[ 1 ], mvLXC[ 1 ] ) (J-47)


**J.8.2.1.7.2.1** **Derivation process for the disparity vector and the inter-view reference**


Inputs to this process are depth reference view component depthPic, the location of a top-left sample ( mbx1, mby1 ) of
the current macroblock and the listSuffixFlag.


Outputs of this process are a picture InterViewPic, an offset vector dV and a variable interViewAvailable.


The variable interViewAvailable is set equal to 0.


The following applies to derive an inter-view reference picture or inter-view only reference picture, InterViewPic, with X
set to 1 when listSuffixFlag is 1 or 0 otherwise:


for( cIdx = 0;cIdx<num_ref_idx_l0_active_minus1 + 1 && !interViewAvailable; cIdx ++ )
if ( view order index of RefPicList0[ cIdx ] is not equal to view_idx) {
InterViewPic = RefPicList0[ cIdx ] (J-48)
interViewAvailable = 1
}


When interViewAvailable is equal to 1, the depth-based disparity value derivation process in clause J.8.2.1.1 is invoked
with depthPic equal to DepthCurrPic, textureX equal to mbx1, textureY equal to mby1, tBlWidth equal to 16, tBlHeight
equal to 16, srcViewId equal to view_id and refViewId equal to view_id of InterViewPic as inputs and the output assigned
to dV.


**J.8.2.1.8** **Macroblock-level neighbouring block based disparity vector derivation process**


Input to this process is a macroblock currMB.


Let the variable availableDvFlag equal to 0, ( xP, yP ) be equal to the output of the clause 6.4.2.1 (the location of upperleft luma sample for currMB partition 0).


The variables dvMBCur and DvMBX are derived as specified by the following ordered steps:


1. For each X from 0 to 1, the following steps apply in order.





        - When availableDvFlag is equal to 0 and RefPicListY[ 0 ] is available (with Y equal to 1-X), the
following applies:


           - Set refPicListCol0 to the reference picture list 0 of RefPicListY[ 0 ].


           - mvColL0 and refIdxColL0 are set to the motion vector mvL0 and reference index refIdxL0 that
have been assigned to the block covering ( xP + 16, yP + 16 ) in picture RefPicListY[ 0 ],
respectively.


           - When refPicListCol0[ refIdxColL0 ] is available, the view order index of
refPicListCol0[ refIdxColL0 ] is unequal to the view_idx, and mvColL0[ 0 ] is unequal to 0,
dvMBCur is set equal to mvColL0[ 0 ] and availableDvFlag is set to 1.


2. The process in clause 6.4.11.7 is invoked with mbPartIdx equal to 0, currSubMbType equal to P_L0_16x16, and
subMbPartIdx equal to 0 as input and the output is assigned to mbAddrN\mbPartIdxN\subMbPartIdxN with N being
replaced by A, B, C, or D. For each N being A, B, C and D, its reference index refIdxL0N and motion vector mvL0N
are set equal to RefIdxL0[ mbPartIdxN ] and MvL0[ mbPartIdxN ][ subMbPartIdxN ], respectively, which are the
reference index refIdxL0 and motion vector mvL0 that have been assigned to the (sub-)macroblock partition
mbAddrN\mbPartIdxN\subMbPartIdxN, and when availableDvFlag is equal to 0, the following applies:


      - When RefPicList0[ refIdxL0N ] is available and the view order index of RefPicList0[ refIdxL0N ] is

unequal to the view_idx, and mvL0N[ 0 ] is unequal to 0, dvMBCur is set to mvL0N[ 0 ] and
availableDvFlag is set to 1.


3. When availableDvFlag is equal to 0, dvMBCur is set to DvMBX.


4. When seq_view_synthesis_flag is equal to 1, the following steps apply in order.


      - The variables currIndex and refIndex are derived by:


currIndex = ViewIdTo3DVAcquisitionParamIndex( view_id of the current view )
refIndex = ViewIdTo3DVAcquisitionParamIndex( view_id of DepthCurrPic ) (J-49)


      - The depth-based disparity value derivation process in clause J.8.2.1.1 is invoked with depthPic equal to

DepthCurrPic, ( textureX, textureY ) equal to ( xP + ( dvMBCur >> 2 ), yP ), tBlWidth equal to 16,
tBlHeight equal to 16, srcViewId equal to currIndex and refViewId equal to refIndex and the output
assigned to dvMBCur.


5. DvMBX is set equal to dvMBCur.


**J.8.2.2** **Derivation of prediction weights in depth-range-based weighted prediction**


The process specified in this clause is invoked when either or both of the following conditions apply:


- nal_unit_type is equal to 21 and depth_weighted_pred_flag is equal to 1 and ( slice_type % 5 ) is equal to 0 or 3,

- depth_weighted_bipred_flag is equal to 1 and ( slice_type % 5 ) is equal to 1


Inputs to this process are:


- the reference indices refIdxL0 and refIdxL1,


- the prediction utilization flags predFlagL0 and predFlagL1.


Outputs of this process are variables for weighted prediction logWDC, w0C, w1C, o0C, and o1C with C being replaced by L
(luma).


The variables currIndex, refIndex, dpsIdCurr and dpsIdRef are derived as follows:


logWDC = 5 (J-50)
currIndex = ViewIdTo3DVAcquisitionParamIndex( view_id of the current view ) (J-51)
refIndex = ViewIdTo3DVAcquisitionParamIndex( view_id of the view of the reference picture) (J-52)
dpsIdCurr = dps_id of the current picture (J-53)
dpsIdRef = dps_id of the reference picture (J-54)


When predFlagL0 is equal to 1, the following is applied.


- The derivation process for a single prediction weight in depth-range-based weighted prediction specified in
clause J.8.2.2.1 is invoked with zNearCurr equal to ZNear[ dpsIdCurr, currIndex ], zFarCurr equal to ZFar[ dpsIdCurr,
currIndex ], zNearRef equal to ZNear[ dpsIdRef, refIndex ], zFarRef equal to ZFar[ dpsIdRef, refIndex ], and
reference list identifier X equal to 0.





When predFlagL1 is equal to 1 and depth_weighted_bipred_flag is equal to 1, the following is applied.


- The derivation process for a single prediction weight in depth-range-based weighted prediction specified in
clause J.8.2.2.1 is invoked with zNearCurr equal to ZNear[ dpsIdCurr, currIndex], zFarCurr equal to ZFar[ dpsIdCurr,
currIndex ], zNearRef equal to ZNear[ dpsIdRe, refIndex ], zFarRef equal to ZFar[ dpsIdRef, refIndex ], and
reference list identifier X equal to 1.


**J.8.2.2.1** **Derivation of weight and offset parameteres**


Inputs of this process are variables zNearCurr, zNearRef, zFarCurr, and zFarRef and the reference list identifier X with X
being replaced by 0 or 1.


Outputs of this process are variables wXC and oXC for weighted prediction.


The variable wXC with X being replaced by 0 or 1 and C being replaced by L is calculated by the following steps.


1. scaleW is set equal to 8.


2. Calculate variable wFactorA as follows:


k = zFarRef − zNearRef
m = zFarRef
x = ( k + ( m >> 1 ) ) / m
signVal = ( ( k − x * m ) < 0 ) ? −1 : 1 (J-55)
wFactorA = ( x << scaleW )
wFactorA += ( ( ( k − x * m ) << scaleW ) + signVal * ( m >> 1 ) ) / m


3. Calculate variable wFactorB as follows:


k = zFarCurr
m = zFarCurr − zNearCurr
x = ( k + ( m >> 1 ) ) / m
signVal = ( ( k − x * m ) < 0 ) ? −1 : 1 (J-56)
wFactorB = ( x << scaleW )
wFactorB += ( ( ( k − x * m ) << scaleW ) + signVal * ( m >> 1) ) / m


4. Calculate variable wFactorC as follows:


k = zNearCurr
m = zNearRef
x = ( k + ( m >> 1 ) ) / m (J-57)
signVal = ( ( k − x * m ) < 0 ) ? −1 : 1
wFactorC = ( x << scaleW )
wFactorC += ( ( ( k − x * m ) << scaleW ) + signVal * ( m >> 1 ) ) / m


5. Calculate variable wXC:


wXC = ( wFactorA * wFactorB * wFactorC + ( 1 << ( scaleW * 3 − logWDC − 1 ) ) )
>> ( scaleW * 3 − logWDC )
wXC = Clip( −127, 128, wXC ) (J-58)


The variable oXC is calculated by the following steps:


1. scaleO is set equal to 8.


2. Calculate variable oFactorA as follows:


oFactorA=( ( zNearCurr << ( scaleO ) ) + ( zFarRef >> 1 ) ) / zFarRef (J-59)


3. Calculate variable oFactorB:





k = zFarCurr − zFarRef
m = zFarCurr − zNearCurr
signVal= ( k < 0 ) ? −1 : 1
x = ( k + signVal * ( m >> 1 ) ) / m (J-60)
signVal = ( ( k − x * m ) < 0 ) ? −1 : 1
oFactorB = ( x << scaleO )
oFactorB += ( ( ( k − x * m ) << scaleO ) + signVal * ( m >> 1 ) ) / m


4. Calculate variable oXC:


oXC = ( oFactorA * oFactorB + ( 1 << ( scaleO * 2 − 8 − 1 ) ) ) >> ( scaleO * 2 − 8 ) (J-61)
oXC = Clip( −127, 128, oXC )


**J.8.2.3** **Derivation process for motion vectors and reference indices for adaptive luminance compensation**


Inputs to this process are:


- a macroblock partition mbPartIdx,


- a sub-macroblock partition subMbPartIdx.


Outputs of this process are:


- luma motion vectors mvL0 and mvL1 and when ChromaArrayType is not equal to 0, the chroma motion vectors
mvCL0 and mvCL1,


- reference indices refIdxL0 and refIdxL1,


- prediction list utilization flags predFlagL0 and predFlagL1,


- a motion vector count variable subMvCnt.


The motion vector count variable subMvCnt is set equal to 1.


Set interViewAvailable equal to 0.


The reference index refIdxL0 for a skipped macroblock is derived as:


for( cIdx = 0; cIdx <= num_ref_idx_l0_active_minus1 && !interViewAvailable; cIdx ++ )
if (RefPicList0[ cIdx ] and the current view component have different values of view oder index) {
refIdxL0 = cIdx (J-62)
interViewAvailable = 1
}


If dmvp_flag is equal to 1 and mb_alc_skip_flag is equal to 1, clause J.8.2.1.7 is invoked and mvL0 is set equal to mvpL0,
the output of clause J.8.2.1.7.


Otherwise, if dmvp_flag is equal to 1 and mb_alc_flag is equal to 1, clause J.8.2.1 is invoked for derivation of mvL0.


Otherwise, clause 8.4.1 is invoked and mvL0 is set equal to mvpL0, the output of clause 8.4.1.


**J.8.2.4** **Derivation process for prediction weights in adaptive luminance compensation**


Inputs to this process are:


- reference index refIdxL0


- the luma sample array of the selected reference picture refPicL0L.


- the current partition given by its partition index mbPartIdx and its sub-macroblock partition index subMbPartIdx


- Luma4x4BlkIdx


- the width and height partWidth, partHeight of this partition in luma-sample units


- a luma motion vector mvL0 given in quarter-luma-sample units


- array cSL containing already constructed luma samples prior to deblocking filter process.


Outputs of this process are:


- variables for weighted prediction of the current partition logWDC, w0C, w1C, o0C, o1C, with C being replaced by
L and, when ChromaArrayType is not equal to 0, Cb and Cr.





The variables W1C, O1C are derived as follows for C equal to L, Cb or Cr:


logWDC = 0 (J-63)
w1C = 0 (J-64)
o1C = 0 (J-65)


The variables W0C, O0C are derived as follows for C if equal to Cb or Cr:


logWDC = 0 (J-66)
w0C = 1 (J-67)
o0C=0 (J-68)


When C is equal to L for luma samples, clauses J.8.2.4.1 through J.8.2.4.5 are invoked sequentially to derive LogWDL,
w0C, and o0C.


**J.8.2.4.1** **Defining of coordinates and sizes of a luma block to be predicted**


Let ( xM, yM ) be equal to the output of clause 6.4.1 (the location of upper-left luma sample for the current macroblock
with address mbAddr relative to the upper-left sample of the picture).


Let ( xP, yP ) be equal to the output of clause 6.4.2.1 (the location of upper-left luma sample for the macroblock partition
mbPartIdx).


Let ( xB, yB ) be equal to the output of clause J.6.1 (the location of upper-left luma sample for the 4x4 luma block defined
by Luma4x4BlkIdx that can be 0...15) relative to the top-left sample of the sub-macroblock.


The variables xT, yT, xBlockWidth, yBlockHeight are set as follows:


- xT is set equal to xM + xP;


- yT is set equal to yM +yP;


- xBlockWidth is set equal to MbPartWidth( mb_type );


- yBlockHeight is set equal to MbPartHeight( mb_type );


If one or more of the following conditions are true, W0C is set equal to 1 and logWDC is set equal to 15.


- ( mvL0[ 0 ] + ( ( xT − 1 ) << 2 ) ) is smaller than 0;


- ( mvL0[ 1 ] + ( ( yT − 1 ) << 2 ) ) is smaller than 0;


- ( mvL0[ 0 ] + ( ( xT + xBlockWidth ) << 2 ) ) is greater than or equal to ( PicWidthInSamplesL << 2 );


- ( mvL0[ 1 ] + ( ( yT + yBlockHeight) << 2 ) ) is greater than or equal to ( PicHeightInSamplesL << 2 ).


Otherwise LRef, URef, LRec, URec sample values are derived as it is specified in clauses J.8.2.4.2 and J.8.2.4.3 followed
by calculation of variables NeighbourRefSum, NeighbourSum and W0L, O0L specified in clauses J.8.2.4.4 and J.8.2.4.5
correspondently.


**J.8.2.4.2** **Deriving of left and up reference samples of the current block**


LRec and URec blocks belong to an (PicWidthInSamplesL)x(PicHeightInSamplesL) array cSL containing constructed luma
samples prior to the deblocking filter process.


Each luma sample LRec[ 0, yL ] for 0 <= yL < yBlockHeight is specified as follows:


LRec[ 0, yL ] = cSL[ xT − 1, yT + yL ] (J-69)


Each luma sample URec[ xL, 0 ] for 0 <= xL < xBlockWidth is specified as follows:


URec[ xL, yL ] = cSL[ xT + xL, yT − 1] (J-70)


**J.8.2.4.3** **Deriving of left and up reference samples of the reference block**


For each luma sample location (0, yL) such as: 0<=yL< yBlockHeight inside LRef block, sample value LRef[ 0, yL ] is
derived by the following ordered steps:


1. The variables xIntL, yIntL, xFracL, and yFracL are derived by:





xIntL = xT + ( mvL0[ 0 ] >> 2 ) - 1 (J-71)
yIntL = yT + ( mvL0[ 1 ] >> 2 ) + yL (J-72)
xFracL= mvL0[ 0 ] & 3 (J-73)
yFracL= mvL0[ 1 ] & 3 (J-74)


2. LRef[ 0, yL ] sample is derived as an output of the process specified in clause 8.4.2.2.1 with ( xIntL, yIntL),
( xFracL, yFracL ) and refPicL0L given as input.


For each luma sample location ( xL, 0 ) such as: 0<=xL< xBlockWidth inside URef block sample value URef[ xL, 0 ] is
derived by the following ordered steps:


1. The variables xIntL, yIntL, xFracL, and yFracL are derived by:


xIntL = xT + ( mvL0[ 0 ] >> 2 ) + xL (J-75)
yIntL = yT + ( mvL0[ 1 ] >> 2 ) − 1 (J-76)
xFracL = mvL0[ 0 ] & 3 (J-77)
yFracL= mvL0[ 1 ] & 3 (J-78)


2. URef[ xL, yL ] sample is derived as an output of the process specified in clause 8.4.2.2.1 with ( xIntL, yIntL ),

( xFracL, yFracL ) and refPicL0L given as input.


**J.8.2.4.4** **Deriviation of NeighbourRefSum and NeighbourSum**


Both NeighbourRefSum and NeighbourSum are set equal to 1 and further calculated as follows:


for( j=0; j< yBlockHeight;j++ )
if ( Abs( LRec[ 0, j ] − LRef[ 0, j ] ) < 31 ) {
NeighbourRefSum = NeighbourRefSum + LRef[ 0, j ] (J-79)
NeighbourSum = NeighbourSum + LRec[ 0, j ]
}


for( i=0; i< xBlockWidth; i++ )
if ( Abs( URec[ i, 0 ] − URef[ i, 0 ] ) < 31 ) {
NeighbourRefSum = NeighbourRefSum + URef[ i, 0 ] (J-80)
NeighbourSum = NeighbourSum + URec[ i, 0 ]
}


**J.8.2.4.5** **Deriviation of prediction weights**


O0L is set equal to 0. W0L and LogWDL are derived as follows:


If ( NeighbourSum >> 4 ) is equal to ( NeighbourRefSum >> 4 ), W0L is set equal to 1 and LogWDL is set equal to 0.


Otherwise, LogWDL is set equal to 15 and W0L is equal to
( ( 1 << LogWDL ) * NeighbourSum + ( NeighbourRefSum >> 1 ) ) / NeighbourRefSum.


**J.8.3** **Specification of bitstream subsets**


The specifications of clause I.8.5 apply.


**J.8.4** **Decoding process for depth range parameters**


This process is invoked for decoding of the depth_ranges( ) syntax structure in sequence parameter set 3D-AVC extension
and for decoding of a depth parameter set RBSP.


Inputs of this process are the variables obtained from parsing the active depth range parameter set RBSP:


- the variables relative to the closest depth: ZNearSign[ dps_id, i ], ZNearExp[ dps_id, i ], ZNearMantissa[ dps_id, i ],
ZNearManLen[ dps_id, i ];


- the variables relative to the farthest depth: ZFarSign[ dps_id, i ], ZFarExp[ dps_id, i ], ZNearMantissa[ dps_id, i ],
ZNearManLen[ dps_id, i ]


where i is the index to the order of views for 3DV acquisition parameters.


Outputs of this process are


- the closest depth values ZNear[ dps_id, i ] and the respective variables respective variables ZNearSign[ dps_id, i ],
ZNearExponent[ dps_id, i ], ZNearMantissa[ dps_id, i ], ZNearManLen[ dps_id, i ];





- the farthest depth values ZFar[ dps_id, i ] and the respective variables respective variables ZFarSign[ dps_id, i ],
ZFarExponent[ dps_id, i ], ZFarMantissa[ dps_id, i ], ZFarManLen[ dps_id, i ].


Some of the views for which the 3DV acquisition parameters are specified may not be present in the coded video sequence.


The output variables x in Table J-4 are derived as follows from the respective variables f, s, e, n, and v indicated in
Table J-4.


- If f is equal to 0 in the depth parameter set RBSP, f in the active sequence parameter set 3D-AVC extension shall be
equal to 1 and x[ dps_id, i ] = x[ 0, i ], s[ dps_id, i ] = s[ 0, i ], e[ dps_id, i ] = e[ 0, i ], and n[ dps_id, i ] = n[ 0, i ] for
all values of i.


- Otherwise (f is equal to 1 in the depth parameter set RBSP), the variable x computed as follows for [ dps_id, i ] where
i is index to the order of views for 3DV acquisition parameters:


- If f is equal to 0 in the depth parameter set RBSP, f in the active sequence parameter set 3D-AVC extension shall be
equal to 1 and x[ dps_id, i ] = x[ 0, i ], s[ dps_id, i ] = s[ 0, i ], e[ dps_id, i ] = e[ 0, i ], and n[ dps_id, i ] = n[ 0, i ] for
all values of i.


- Otherwise (f is equal to 1 in the depth parameter set RBSP), the variable x computed as follows for [ dps_id, i ] where
i is index to the order of views for 3DV acquisition parameters:


      - If 0 < e < 127, x = ( −1 ) [s]      - 2 [( e − 31 )]      - ( 1 + n ÷ 2 [v] ).


      - Otherwise (e is equal to 0), x = ( −1 ) [s]      - 2 [−( 30 + v )]      - n.


NOTE – The above specification is similar to that found in IEC 60559:1989, Binary floating _-point arithmetic for microprocessor_
_systems_ .


**Table J-4 – Association between depth parameter variables and syntax elements**

|x|f|s|E|n|v|
|---|---|---|---|---|---|
|**ZNear**|z_near_flag|ZNearSign|ZNearExp|ZNearMantissa|ZNearManLen|
|**ZFar**|z_far_flag|ZFarSign|ZFarExp|ZFarMantissa|ZFarManLen|
