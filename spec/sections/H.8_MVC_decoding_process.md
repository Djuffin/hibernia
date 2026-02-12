**H.8** **MVC decoding process**


This clause specifies the decoding process for an access unit of a coded video sequence conforming to one or more of the
profiles specified in Annex H. Specifically, this clause specifies how the decoded picture with multiple view components
is derived from syntax elements and global variables that are derived from NAL units in an access unit when the decoder
is decoding the operation point identified by the target temporal level and the target output views.


The decoding process is specified such that all decoders shall produce numerically identical results for the target output
views. Any decoding process that produces identical results for the target output views to the process described here
conforms to the decoding process requirements of this Recommendation | International Standard.


Unless stated otherwise, the syntax elements and derived upper-case variables that are referred to by the decoding process
specified in this clause and all child processes invoked from the process specified in this clause are the syntax elements
and derived upper-case variables for the current access unit.


The target output views are either specified by external means not specified in this Specification, or, when not specified
by external means, there shall be one target output view which is the base view. Let OutputVOIdxList be the list of VOIdx
values, in increasing order of VOIdx, of all the target output views in one access unit. The list OutputVOIdxList shall not
change within a coded video sequence.


All sub-bitstreams that can be derived using the sub-bitstream extraction process with pIdTarget equal to any value in the
range of 0 to 63, inclusive, tIdTarget equal to any value in the range of 0 to 7, inclusive, viewIdTargetList consisting of
any one or more viewIdTarget's identifying the views in the bitstream as inputs as specified in clause H.8.5 shall result in
a set of coded video sequences, with each coded video sequence conforming to one or more of the profiles specified in
Annex A and Annex H.


Let vOIdxList be a list of integer values specifying the VOIdx values of the view components of the access unit. The
variable VOIdxMax is set equal to the maximum value of the entries in the list vOIdxList, and the variable vOIdxMin is
set to the minimum value of the entries in the list vOIdxList. VOIdxMax shall be the same for all access units within a
coded video sequence. vOIdxMin shall be the same for all anchor access units within a coded video sequence. When the
current access unit is an anchor access unit, the variable VOIdxMin is set to vOIdxMin.


The multiview video decoding process specified in this clause is repeatedly invoked for each view component with VOIdx
from vOIdxMin to VOIdxMax, inclusive, which is present in the list vOIdxList, in increasing order of VOIdx.


Outputs of the multiview video decoding process are decoded samples of the current primary coded picture including all
decoded view components.


For each view component, the specifications in clause 8 apply, with the decoding processes for picture order count,
reference picture lists construction and decoded reference picture marking being modified in clauses H.8.1, H.8.2 and
H.8.3, respectively. The MVC inter prediction and inter-view prediction process is specified in clause H.8.4. The
specification of bitstream subsets is specified in clause H.8.5. Additionally, when mfc_format_idc is present, the
recommended enhanced-resolution picture reconstruction process is as described in clause H.8.6.


**H.8.1** **MVC decoding process for picture order count**


The process specified in this clause is invoked for a particular view with view order index VOIdx. The specifications in
clause 8.2.1 apply independently for each view, with "picture" being replaced by "view component", "frame" being
replaced by "frame view component", and "field" being replaced by "field view component".


The following constraints shall be obeyed:


- When the view components of an access unit have field_pic_flag equal to 0 or (field_pic_flag equal to 1 and
bottom_field_flag equal to 0), it is a requirement of bitstream conformance that the bitstream shall not contain data
that result in different values of TopFieldOrderCnt for the view components of the access unit.


- When the view components of an access unit have field_pic_flag equal to 0 or (field_pic_flag equal to 1 and
bottom_field_flag equal to 1), it is a requirement of bitstream conformance that the bitstream shall not contain data
that result in different values of BottomFieldOrderCnt for the view components of the access unit.





**H.8.2** **MVC decoding process for reference picture lists construction**


This process is invoked at the beginning of the decoding process for each P, SP or B slice.


During the invocation of this process, when clauses 8.2.4.1 and 8.2.4.2 are invoked, only the reference pictures having the
same value of view_id as the current slice are considered. All clauses of clause 8 are invoked with "picture" being replaced
by "view component", "frame" being replaced by "frame view component", and "field" being replaced by "field view
component".


Decoded reference pictures are marked as "used for short-term reference" or "used for long-term reference" as specified in
clause H.8.3. Short-term reference pictures are identified by the values of frame_num. Long-term reference pictures are
assigned a long-term frame index as specified in clause H.8.3.


In addition to reference pictures marked as "used for short-term reference" or "used for long-term reference", inter-view
reference components and inter-view only reference components of the current access unit may also be included in a
reference picture list. Inter-view reference components and inter-view only reference components are identified by the
value of view_id.


Clause 8.2.4.1 is invoked to specify


- the assignment of variables FrameNum, FrameNumWrap, and PicNum to each of the short-term reference pictures,
and


- the assignment of variable LongTermPicNum to each of the long-term reference pictures.


Reference pictures and, when present, inter-view only reference components, are addressed through reference indices as
specified in clause 8.2.4.1. A reference index is an index into a reference picture list. When decoding a P or SP slice, there
is a single reference picture list RefPicList0. When decoding a B slice, there is a second independent reference picture list
RefPicList1 in addition to RefPicList0.


At the beginning of the decoding process for each slice, reference picture list RefPicList0, and for B slices RefPicList1,
are derived as specified by the following ordered steps:


1. Depending on non_idr_flag, the following applies:


     - If non_idr_flag is equal to 1, the initial reference picture list RefPicList0 and for B slices RefPicList1 are
derived as specified in clause 8.2.4.2.


     - Otherwise (non_idr_flag is equal to 0), all (num_ref_idx_l0_active_minus1 + 1) entries of the initial
reference picture list RefPicList0 are set equal to "no reference picture" and, for B slices, all
(num_ref_idx_l1_active_minus1 + 1) entries of the initial reference picture list RefPicList1 are set equal to
"no reference picture".


2. Inter-view reference components or inter-view only reference components are appended to the initial reference

picture list RefPicList0 and for B slices RefPicList1 as specified in clause H.8.2.1.


3. When ref_pic_list_modification_flag_l0 is equal to 1 or, when decoding a B slice,
ref_pic_list_modification_flag_l1 is equal to 1, the reference picture list RefPicList0 and for B slices RefPicList1
are modified as specified in clause H.8.2.2.


NOTE – The modification process for reference picture lists specified in clause H.8.2.2 allows the contents of RefPicList0
and for B slices RefPicList1 to be modified in a flexible fashion. In particular, it is possible for a reference picture that is
currently marked "used for reference" to be inserted into RefPicList0 and for B slices RefPicList1 even when the reference
picture is not in the initial reference picture list derived as specified in clauses 8.2.4.2 and H.8.2.1.


The number of entries in the modified reference picture list RefPicList0 is num_ref_idx_l0_active_minus1 + 1, and for B
slices the number of entries in the modified reference picture list RefPicList1 is num_ref_idx_l1_active_minus1 + 1. A
reference picture or inter-view only reference component may appear at more than one index in the modified reference
picture lists RefPicList0 or RefPicList1.


During the invocation of the process specified in clause H.8.2.1, an inter-view prediction reference appended to
RefPicListX (with X being 0 or 1) may not exist. However, an inter-view prediction reference that does not exist shall not
be in the modified RefPicListX after the invocation of the process specified in clause H.8.2.2.


When anchor_pic_flag is equal to 1, the bitstream shall not contain data that result in any entry of the reference picture list
RefPicList0 or, for B slices, any entry of the reference picture list RefPicList1 that does not represent a view component
of the current access unit.





**H.8.2.1** **Initialization process for reference picture list for inter-view prediction references**


Inputs to this process are a reference picture list RefPicListX (with X being 0 or 1), inter_view_flag and view dependency
information that has been decoded from the seq_parameter_set_mvc_extension( ).


The output of this process is a possibly modified reference picture list RefPicListX, which is still referred to as the initial
reference picture list RefPicListX.


With i being the value of VOIdx for the current slice, inter-view reference components and inter-view only reference
components (the corresponding NAL units have inter_view_flag equal to 1) are appended to the reference picture list as
specified in the following.


- If the current slice has anchor_pic_flag equal to 1, for each value of reference view index j from 0 to
num_anchor_refs_lX[ i ] − 1, inclusive, in ascending order of j, the inter-view prediction reference with view_id equal
to anchor_ref_lX[ i ][ j ] from the same access unit as the current slice is appended to RefPicListX.


- Otherwise (the current slice has anchor_pic_flag equal to 0), for each value of reference view index j from 0 to
num_non_anchor_refs_lX[ i ] − 1, inclusive, in ascending order of j, the inter-view prediction reference with view_id
equal to non_anchor_ref_lX[ i ][ j ] from the same access unit as the current slice is appended to RefPicListX.


NOTE 1 – View components with inter_view_flag equal to 0 are not appended to the reference picture list.


NOTE 2 – When a NAL unit with nal_unit_type equal to 1 or 5 is not immediately preceded by a NAL unit with nal_unit_type
equal to 14, the value of inter_view_flag is inferred to be equal to 1. Encoders that do not encode a prefix NAL unit before each
NAL unit with nal_unit_type equal to 1 or 5 and devices that remove prefix NAL units from a bitstream should take into
consideration this inferred value to avoid potential mismatches in the reference picture lists between the encoder and decoder.


Inter-view reference components and inter-view only reference components are appended to the reference picture list
starting from the first entry position of "no reference picture" in the initial reference picture list RefPicListX or starting
from the entry position num_ref_idx_lX_active_minus1+1 of the initial reference picture list RefPicListX, whichever is
the earliest position.


When the number of entries in the initial reference picture list RefPicListX is greater than
(num_ref_idx_lX_active_minus1 + 1), the extra entries past position num_ref_idx_lX_active_minus1 are discarded from
the initial reference picture list RefPicListX.


**H.8.2.2** **Modification process for reference picture lists**


Input to this process is reference picture list RefPicList0 and, when decoding a B slice, also reference picture list
RefPicList1.


Outputs of this process are a possibly modified reference picture list RefPicList0 and, when decoding a B slice, also a
possibly modified reference picture list RefPicList1.


When ref_pic_list_modification_flag_l0 is equal to 1, the following ordered steps are specified:


1. Let refIdxL0 be an index into the reference picture list RefPicList0. It is initially set equal to 0.


2. The corresponding syntax elements modification_of_pic_nums_idc are processed in the order they occur in the

bitstream. For each of these syntax elements, the following applies:


   - If modification_of_pic_nums_idc is equal to 0 or equal to 1, the process specified in clause H.8.2.2.1 is invoked
with RefPicList0 and refIdxL0 given as input, and the output is assigned to RefPicList0 and refIdxL0.


   - Otherwise, if modification_of_pic_nums_idc is equal to 2, the process specified in clause H.8.2.2.2 is invoked
with RefPicList0 and refIdxL0 given as input, and the output is assigned to RefPicList0 and refIdxL0.


   - Otherwise, if modification_of_pic_nums_idc is equal to 4 or equal to 5, the process specified in clause H.8.2.2.3
is invoked with RefPicList0 and refIdxL0 given as input, and the output is assigned to RefPicList0 and refIdxL0.


   - Otherwise (modification_of_pic_nums_idc is equal to 3), the modification process for reference picture list
RefPicList0 is finished.


When ref_pic_list_modification_flag_l1 is equal to 1, the following ordered steps are specified:


1. Let refIdxL1 be an index into the reference picture list RefPicList1. It is initially set equal to 0.


2. The corresponding syntax elements modification_of_pic_nums_idc are processed in the order they occur in the

bitstream. For each of these syntax elements, the following applies:


   - If modification_of_pic_nums_idc is equal to 0 or equal to 1, the process specified in clause H.8.2.2.1 is invoked
with RefPicList1 and refIdxL1 given as input, and the output is assigned to RefPicList1 and refIdxL1.





   - Otherwise, if modification_of_pic_nums_idc is equal to 2, the process specified in clause H.8.2.2.2 is invoked
with RefPicList1 and refIdxL1 given as input, and the output is assigned to RefPicList1 and refIdxL1.


   - Otherwise, if modification_of_pic_nums_idc is equal to 4 or equal to 5, the process specified in clause H.8.2.2.3
is invoked with RefPicList1 and refIdxL1 given as input, and the output is assigned to RefPicList1 and refIdxL1.


   - Otherwise (modification_of_pic_nums_idc is equal to 3), the modification process for reference picture list
RefPicList1 is finished.


**H.8.2.2.1** **Modification process of reference picture lists for short-term reference pictures for inter prediction**


Inputs to this process are an index refIdxLX and a reference picture list RefPicListX (with X being 0 or 1).


Outputs of this process are an incremented index refIdxLX and a modified reference picture list RefPicListX.


The variable picNumLXNoWrap is derived as follows:


- If modification_of_pic_nums_idc is equal to 0,


if( picNumLXPred − ( abs_diff_pic_num_minus1 + 1 ) < 0 )
picNumLXNoWrap = picNumLXPred − ( abs_diff_pic_num_minus1 + 1 ) + MaxPicNum (H-1)
else
picNumLXNoWrap = picNumLXPred − ( abs_diff_pic_num_minus1 + 1 )


- Otherwise (modification_of_pic_nums_idc is equal to 1),


if( picNumLXPred + ( abs_diff_pic_num_minus1 + 1 ) >= MaxPicNum )
picNumLXNoWrap = picNumLXPred + ( abs_diff_pic_num_minus1 + 1 ) − MaxPicNum (H-2)
else
picNumLXNoWrap = picNumLXPred + ( abs_diff_pic_num_minus1 + 1 )


picNumLXPred is the prediction value for the variable picNumLXNoWrap. When the process specified in this clause is
invoked the first time for a slice (that is, for the first occurrence of modification_of_pic_nums_idc equal to 0 or 1 in the
ref_pic_list_modification( ) syntax), picNumL0Pred and picNumL1Pred are initially set equal to CurrPicNum. After each
assignment of picNumLXNoWrap, the value of picNumLXNoWrap is assigned to picNumLXPred.


The variable picNumLX is derived as specified by the following pseudo-code:


if( picNumLXNoWrap > CurrPicNum )
picNumLX = picNumLXNoWrap − MaxPicNum (H-3)
else
picNumLX = picNumLXNoWrap


picNumLX shall be equal to the PicNum of a reference picture that is marked as "used for short-term reference" and shall
not be equal to the PicNum of a short-term reference picture that is marked as "non-existing" _._


The following procedure is conducted to place the picture with short-term picture number picNumLX into the index
position refIdxLX, shift the position of any other remaining pictures to later in the list, and increment the value of refIdxLX:


for( cIdx = num_ref_idx_lX_active_minus1 + 1; cIdx > refIdxLX; cIdx−    - )
RefPicListX[ cIdx ] = RefPicListX[ cIdx − 1]
RefPicListX[ refIdxLX++ ] = short-term reference picture with PicNum equal to picNumLX
nIdx = refIdxLX
for( cIdx = refIdxLX; cIdx <= num_ref_idx_lX_active_minus1 + 1; cIdx++ ) (H-4)
if( PicNumF( RefPicListX[ cIdx ] ) != picNumLX | | viewID(RefPicListX[ cIdx ] ) != currViewID )
RefPicListX[ nIdx++ ] = RefPicListX[ cIdx ]


In the above, the function viewID(refpic) returns the view_id of the reference picture refpic, the variable currViewID is
equal to the view_id of the current slice, and the function PicNumF( RefPicListX[ cIdx ] ) is derived as follows:


- If the reference picture RefPicListX[ cIdx ] is marked as "used for short-term reference",
PicNumF( RefPicListX[ cIdx ] ) is the PicNum of the picture RefPicListX[ cIdx ].


- Otherwise (the reference picture RefPicListX[ cIdx ] is not marked as "used for short-term reference"),
PicNumF( RefPicListX[ cIdx ] ) is equal to MaxPicNum.


NOTE 1 – The value of picNumLX can never be equal to MaxPicNum.


NOTE 2 – Within this pseudo-code procedure, the length of the list RefPicListX is temporarily made one element longer than the
length needed for the final list. After the execution of this procedure, only elements 0 through num_ref_idx_lX_active_minus1 of
the list need to be retained.





**H.8.2.2.2** **Modification process of reference picture lists for long-term reference pictures for inter prediction**


Inputs to this process are an index refIdxLX (with X being 0 or 1) and reference picture list RefPicListX.


Outputs of this process are an incremented index refIdxLX and a modified reference picture list RefPicListX.


The following procedure is conducted to place the picture with long-term picture number long_term_pic_num into the
index position refIdxLX, shift the position of any other remaining pictures to later in the list, and increment the value of
refIdxLX:


for( cIdx = num_ref_idx_lX_active_minus1 + 1; cIdx > refIdxLX; cIdx−    - )
RefPicListX[ cIdx ] = RefPicListX[ cIdx − 1]
RefPicListX[ refIdxLX++ ] = long-term reference picture with LongTermPicNum equal to long_term_pic_num
nIdx = refIdxLX
for( cIdx = refIdxLX; cIdx <= num_ref_idx_lX_active_minus1 + 1; cIdx++ ) (H-5)
if( LongTermPicNumF( RefPicListX[ cIdx ] ) != long_term_pic_num | |
viewID(RefPicListX[ cIdx ] ) != currViewID )
RefPicListX[ nIdx++ ] = RefPicListX[ cIdx ]


In the above, the function viewID(refpic) returns the view_id of the reference picture refpic, the variable currViewID is
equal to the view_id of the current slice, and the function LongTermPicNumF( RefPicListX[ cIdx ] ) is derived as follows:


- If the reference picture RefPicListX[ cIdx ] is marked as "used for long-term reference",
LongTermPicNumF( RefPicListX[ cIdx ] ) is the LongTermPicNum of the picture RefPicListX[ cIdx ].


- Otherwise (the reference picture RefPicListX[ cIdx ] is not marked as "used for long-term reference"),
LongTermPicNumF( RefPicListX[ cIdx ] ) is equal to 2 * ( MaxLongTermFrameIdx + 1 ).


NOTE 1 – The value of long_term_pic_num can never be equal to 2 * ( MaxLongTermFrameIdx + 1 ).


NOTE 2 – Within this pseudo-code procedure, the length of the list RefPicListX is temporarily made one element longer than the
length needed for the final list. After the execution of this procedure, only elements 0 through num_ref_idx_lX_active_minus1 of
the list need to be retained.


**H.8.2.2.3** **Modification process for reference picture lists for inter-view prediction references**


Inputs to this process are reference picture list RefPicListX (with X being 0 or 1) and an index refIdxLX into this list.


Outputs of this process are a modified reference picture list RefPicListX (with X being 0 or 1) and an incremented index
refIdxLX.


Let currVOIdx be the variable VOIdx of the current slice. The variable maxViewIdx is derived as follows:


- If the current slice has anchor_pic_flag equal to 1, maxViewIdx is set equal to num_anchor_refs_lX[ currVOIdx ] − 1.


- Otherwise (the current slice has anchor_pic_flag equal to 0), maxViewIdx is set equal to
num_non_anchor_refs_lX[ currVOIdx ] − 1.


The variable picViewIdxLX is derived as follows:


- If modification_of_pic_nums_idc is equal to 4,


if( picViewIdxLXPred − ( abs_diff_view_idx_minus1 + 1 ) < 0 )

picViewIdxLX = picViewIdxLXPred − ( abs_diff_view_idx_minus1 + 1 ) + maxViewIdx + 1 (H-6)
else

picViewIdxLX = picViewIdxLXPred − ( abs_diff_view_idx_minus1 + 1 )


- Otherwise (modification_of_pic_nums_idc is equal to 5),


if( picViewIdxLXPred + ( abs_diff_view_idx_minus1 + 1 ) >= maxViewIdx + 1 )

picViewIdxLX = picViewIdxLXPred + ( abs_diff_view_idx_minus1 + 1 ) − ( maxViewIdx + 1 ) (H-7)
else

picViewIdxLX = picViewIdxLXPred + ( abs_diff_view_idx_minus1 + 1 )


picViewIdxLXPred is the prediction value for the variable picViewIdxLX. When the process specified in this clause is
invoked the first time for a slice (that is, for the first occurrence of modification_of_pic_nums_idc equal to 4 or 5 in the
ref_pic_list_modification( ) syntax), picViewIdxL0Pred and picViewIdxL1Pred are initially set equal to −1. After each
assignment of picViewIdxLX, the value of picViewIdxLX is assigned to picViewIdxLXPred.


The bitstream shall not contain data that result in picViewIdxLX less than 0 or picViewIdxLX greater than maxViewIdx.


The variable targetViewID is derived as follows:





- If the current slice has anchor_pic_flag equal to 1,


targetViewID = anchor_refs_lX[ currVOIdx ][ picViewIdxLX ] (H-8)


- Otherwise (the current slice has anchor_pic_flag equal to 0),


targetViewID = non_anchor_refs_lX[ currVOIdx ][ picViewIdxLX ] (H-9)


The following procedure is conducted to place the inter-view prediction reference with reference view index equal to
picViewIdxLX into the index position refIdxLX and shift the position of any other remaining pictures to later in the list:


for( cIdx = num_ref_idx_lX_active_minus1 + 1; cIdx > refIdxLX; cIdx−    - )
RefPicListX[ cIdx ] = RefPicListX[ cIdx − 1]
RefPicListX[ refIdxLX++ ] = inter-view prediction reference with view_id equal to targetViewID
nIdx = refIdxLX
for( cIdx = refIdxLX; cIdx <= num_ref_idx_lX_active_minus1 + 1; cIdx++ ) (H-10)
if( viewID(RefPicListX[ cIdx ]) != targetViewID | | PictureOrderCnt(RefPicListX[ cIdx ]) != currPOC )
RefPicListX[ nIdx++ ] = RefPicListX[ cIdx ]


In the above, the function viewID(refpic) returns the view_id of the reference picture refpic, the variable currViewID is
equal to the view_id of the current slice, and the variable currPOC is equal to PicOrderCnt( ) of the current slice.


**H.8.3** **MVC decoded reference picture marking process**


The process specified in this clause is invoked for a particular view with view order index VOIdx. The specifications in
clause 8.2.5 apply with "picture" being replaced by "view component", "frame" being replaced by "frame view
component", and "field" being replaced by "field view component". During the invocation of the process for a particular
view, only view components of the particular view are considered. The marking of view components of other views is not
changed.


NOTE – A view component of a picture may have a different marking status than other view components of the same picture.


**H.8.4** **MVC inter prediction and inter-view prediction process**


For both inter-prediction and inter-view prediction, the specifications in clause 8.4 apply. For the invocation of the MVC
inter prediction and inter-view prediction process as specified in this clause, the inter-view reference components and interview only reference components that are included in the reference picture lists are considered as not being marked as "used
for short-term reference" or "used for long-term reference".


NOTE – This implies that when RefPicList1[ 0 ] represents an inter-view reference component or an inter-view only reference
component, the variable colZeroFlag in clause 8.4.1.2.2 is always derived to be equal to 0.


When mfc_format_idc is present, the additional processing for an inter-view prediction reference as specified in
clause H.8.4.1 is invoked before the inter-view prediction reference is used for inter-view prediction.


**H.8.4.1** **Additional processing for an inter-view prediction reference**


This process is invoked when mfc_format_idc is present to modify each colour component array of an inter-view prediction
reference used in decoding view components of the non-base view. When rpu_filter_enabled_flag is equal to 1, the
modification process consists of a one-dimensional downsampling filtering process followed by a one-dimensional
upsampling filtering process. The modification process converts an inter-view prediction reference from the frame packing
arrangement format of the view components in the base view to the frame packing arrangement format of the view
components in the non-base view. When rpu_filter_enabled_flag is equal to 0, all sample values for each colour component
of an inter-view prediction reference are set equal to 128.


Inputs of this process are:


- two variables refW and refH specifying the width and height, respectively, of the inter-view prediction reference
picture sample array,


- a (refW)x(refH) inter-view prediction reference picture sample array refPicture for either the luma or a chroma
component.


Output of this process is a (refW)x(refH) modified inter-view prediction reference picture sample array RpuPicture for
either the luma or a chroma component.


The input sample array refPicture corresponds to a decoded sample array SL, SCb or SCr, respectively, for each colour
component derived in clause 8.7 for a decoded frame or complementary field pair or field of a decoded frame from the
base view.


The output sample array RpuPicture corresponds to a decoded sample array SL, SCb or SCr, respectively, for each colour





component derived in clause 8.7 for a previously-decoded reference frame or complementary reference field pair or field
of a reference frame.


Depending on the value of rpu_field_processing_flag, the following applies:


- If rpu_field_processing_flag is equal to 1, the inter-view prediction reference picture refPicture is a field, the modified
inter-view prediction reference picture RpuPicture is a field, and the inter-view prediction reference modification is
applied to the individual inter-view prediction reference fields separately.


- Otherwise (rpu_field_processing_flag is equal to 0), the inter-view prediction reference picture refPicture is a frame,
the modified inter-view prediction reference picture RpuPicture is a frame, and the inter-view prediction reference
modification is applied to the inter-view prediction reference frame.


The mathematical function Clip1( ) is defined with Clip1( ) being substituted with Clip1Y( ) for the luma component and
Clip1( ) being substituted with Clip1C( ) for the chroma components, respectively.


The variables refW and refH are derived as follows:


- For the luma component, refW is set equal to PicWidthInSamplesL and refH is set equal to PicHeightInSamplesL.


- For the chroma components, refW is set equal to PicWidthInSamplesC and refH is set equal to PicHeightInSamplesC.


The variables subW and subH are derived as follows:


- For the luma component, subW is set equal to SubWidthC and subH is set equal to SubHeightC.


- For the chroma components, subW is set equal to 1 and subH is set equal to 1.


The variable view0OffsetX is derived as follows:


- If view0_grid_position_x is equal to 4 or 8, view0OffsetX is set equal to 0,


- Otherwise (view0_grid_position_x is equal to 12), view0OffsetX is set equal to 1.


The variable view1OffsetX is derived as follows:


- If view1_grid_position_x is equal to 4 or 8, view1OffsetX is set equal to 0,


- Otherwise (view1_grid_position_x is equal to 12), view1OffsetX is set equal to 1.


The variable view0OffsetY is derived as follows:


- If view0_grid_position_y is equal to 4 or 8, view0OffsetY is set equal to 0,


- Otherwise (view0_grid_position_y is equal to 12), view0OffsetY is set equal to 1.


The variable view1OffsetY is derived as follows:


- If view1_grid_position_y is equal to 4 or 8, view1OffsetY is set equal to 0,


- Otherwise (view1_grid_position_y is equal to 12), view1OffsetY is set equal to 1.


The variable RpuW specifying the width of an active area of the reference picture is derived as specified by the following
ordered steps:


1. leftOffset = frame_crop_left_offset* subW


2. rightOffset = frame_crop_right_offset * subW


3. RpuW = refW − leftOffset − rightOffset (H-11)


The variable RpuH specifying the height of an active area of the reference picture is derived as specified by the following
ordered steps:


1. topOffset = frame_crop_top_offset * subH * ( 2 − frame_mbs_only_flag )


2. botOffset = frame_crop_bottom_offset * subH * ( 2 − frame_mbs_only_flag )


3. if ( rpu_field_processing_flag )

topOffset = topOffset >> 1, botOffset = botOffset >> 1


4. RpuH = refH − topOffset − botOffset (H-12)


The variable SbsV is set equal to RpuW >> 1. In the side-by-side arrangement, the view boundary position between the
left and right views is set equal to SbsV + leftOffset.





The variable TabV is set equal to RpuH >> 1. In the top-bottom arrangement, the view boundary position between the left
and right views is set equal to TabV + topOffset.


The filtered samples of picture sample array rpuPicture[ x, y ], with x = 0..refW − 1 and y = 0..refH − 1, are derived as
follows:


- If rpu_filter_enabled_flag is equal to 0, the following applies:


RpuPicture[ x, y ] = 128 with x = 0..refW − 1 and y = 0..refH − 1 (H-13)


- Otherwise (rpu_filter_enabled_flag is equal to 1), the following applies:


Let tempPicture[ x, y ] be a (SbsV)x(TabV) array of samples with x = 0..SbsV − 1 and y = 0..TabV − 1.


    - If mfc_format_idc is equal to 0, let tempRefPic[ x, y ] be a (SbsV)x(RpuH) array of samples with
x = 0..SbsV − 1 and y = 0..RpuH − 1, and tempRpuPic[ x, y ] be a (RpuW)x(TabV) array of samples with
x = 0..RpuW − 1 and y = 0..TabV − 1.


    - Otherwise (mfc_format_idc is equal to 1), let tempRefPic[ x, y ] be a (RpuW)x(TabV) array of samples
with x = 0..RpuW − 1 and y = 0..TabV − 1, and tempRpuPic[ x, y ] be a (SbsV)x(RpuH) array of samples
with x = 0..SbsV − 1 and y = 0..RpuH − 1.


The filtered samples of picture sample array RpuPicture[ x, y ] with x = leftOffset..RpuW − 1 + leftOffset and
y = topOffset..RpuH − 1 + topOffset are derived as specified by the following ordered steps:


    - If mfc_format_idc is equal to 0, the following applies:


1. RpuPicture[ x, y ] with x = leftOffset.. RpuW − 1 + leftOffset and y = topOffset..TabV − 1 + topOffset

is derived from the input of the array refPicture[ x, y ] with x = leftOffset..SbsV − 1 + leftOffset and
y = topOffset..RpuH − 1 + topOffset as specified by the following ordered steps:


a. tempRefPic[ x, y ] = refPicture[ x + leftOffset, y + topOffset ]


with x = 0..SbsV − 1 and y = 0..RpuH − 1 (H-14)


b. Apply the following one-dimensional downsampling process:


tempPicture[ x, y ] =
Clip1( ( 4 * tempRefPic[ x, Clip3( 0, 2 * y − 2, RpuH − 1 ) ] +
7 * tempRefPic[ x, Clip3( 0, 2 * y − 1, RpuH − 1 ) ] +
10 * tempRefPic[ x, Clip3( 0, 2 * y, RpuH − 1 ) ] +
7 * tempRefPic[ x, Clip3( 0, 2 * y + 1, RpuH − 1 ) ] +
4 * tempRefPic[ x, Clip3( 0, 2 * y + 2, RpuH − 1 ) ] + 32 ) >> 6 ) (H-15)


with x = 0..SbsV − 1 and y = 0..TabV − 1


c. Apply the following one-dimensional upsampling process:


gMin = − 2 − View0OffsetX (H-16)


tempRpuPic[ 2 * x + view0OffsetX, y ] = tempPicture[ x, y ] (H-17)


tempRpuPic[ 2 * x + ( 1 − view0OffsetX ), y ] =
Clip1( ( 3 * tempPicture[ Clip3( 0, x + gMin, SbsV − 1 ), y ] −
17 * tempPicture[ Clip3( 0, x + gMin + 1, SbsV − 1 ), y ] +
78 * tempPicture[ Clip3( 0, x + gMin + 2, SbsV − 1 ), y ] +
78 * tempPicture[ Clip3( 0, x + gMin + 3, SbsV − 1 ), y ] −
17 * tempPicture[ Clip3( 0, x + gMin + 4, SbsV − 1 ), y ] +
3 * tempPicture[ Clip3( 0, x + gMin + 5, SbsV − 1 ), y ] + 64) >> 7 ) (H-18)


with x = 0..SbsV − 1 and y = 0..TabV − 1


d. RpuPicture[ x + leftOffset, y + topOffset ] = tempRpuPic[ x, y ]


with x = 0..RpuW − 1 and y = 0..TabV − 1 (H-19)


2. RpuPicture[ x, y ] with x = leftOffset..RpuW − 1 + leftOffset and
y = TabV + topOffset..RpuH − 1 + topOffset is derived from the input of the array refPicture[ x, y ] with
x = SbsV + leftOffset..RpuW − 1 + leftOffset and y = topOffset..RpuH − 1 + topOffset as specified by
the following ordered steps:


a. tempRefPic[ x, y ] = refPicture[ x + SbsV + leftOffset, y + topOffset ]





with x = 0..SbsV − 1 and y = 0..RpuH − 1 (H-20)


b. Apply the following one-dimensional downsampling process:


tempPicture[ x, y ] =
Clip1( ( 4 * tempRefPic[ x, Clip3( 0, 2 * y − 2, RpuH − 1 ) ] +
7 * tempRefPic[ x, Clip3( 0, 2 * y − 1, RpuH − 1 ) ] +
10 * tempRefPic[ x, Clip3( 0, 2 * y, RpuH − 1 ) ] +
7 * tempRefPic[ x, Clip3( 0, 2 * y + 1, RpuH − 1 ) ] +
4 * tempRefPic[ x, Clip3( 0, 2 * y + 2, RpuH − 1 ) ] + 32 ) >> 6 ) (H-21)


with x = 0..SbsV − 1 and y = 0..TabV − 1


c. Apply the following one-dimensional upsampling process:


gMin = − 2 − View1OffsetX (H-22)


tempRpuPic[ 2 * x + view1OffsetX, y ] = tempPicture[ x, y ] (H-23)


tempRpuPic[ 2 * x + ( 1 − view1OffsetX ), y ] =
Clip1( ( 3 * tempPicture[ Clip3( 0, x + gMin, SbsV − 1 ), y ] −
17 * tempPicture[ Clip3( 0, x + gMin + 1, SbsV − 1 ), y ] +
78 * tempPicture[ Clip3( 0, x + gMin + 2, SbsV − 1 ), y ] +
78 * tempPicture[ Clip3( 0, x + gMin + 3, SbsV − 1 ), y ] −
17 * tempPicture[ Clip3( 0, x + gMin + 4, SbsV − 1 ), y ] +
3 * tempPicture[ Clip3( 0, x + gMin + 5, SbsV − 1 ), y ] + 64) >> 7 ) (H-24)


with x = 0..SbsV − 1 and y = 0..TabV − 1


d. RpuPicture[ x + leftOffset, y + TabV + topOffset ] = tempRpuPic[ x, y ]


with x = 0..RpuW − 1 and y = 0..TabV − 1 (H-25)


- Otherwise ( mfc_format_idc is equal to 1), the following applies:


1. RpuPicture[ x, y ] with x = leftOffset..SbsV − 1 + leftOffset and y = topOffset..RpuH − 1 + topOffset is

derived from the input of the array refPicture[ x, y ] with x = leftOffset..RpuW − 1 + leftOffset and
y = topOffset..TabV − 1 + topOffset as specified by the following ordered steps:


a. tempRefPic[ x, y ] = refPicture[ x + leftOffset, y + topOffset ]


with x = 0..RpuW − 1 and y = 0..TabV − 1 (H-26)


b. Apply the following one-dimensional downsampling process:


tempPicture[ x, y ] =
Clip1( ( 4 * tempRefPic[ Clip3( 0, 2 * x − 2, RpuW − 1 ), y ] +
7 * tempRefPic[ Clip3( 0, 2 * x − 1, RpuW − 1 ), y ] +
10 * tempRefPic[ Clip3( 0, 2 * x, RpuW − 1 ), y ] +
7 * tempRefPic[ Clip3( 0, 2 * x + 1, RpuW − 1 ), y ] +
4 * tempRefPic[ Clip3( 0, 2 * x + 2, RpuW − 1 ), y ] + 32 ) >> 6 ) (H-27)


with x = 0..SbsV − 1 and y = 0..TabV − 1


c. Apply the following one-dimensional upsampling process:


gMin = − 2 − View0OffsetY (H-28)


tempRpuPic[ x, 2 * y + view0OffsetY ] = tempPicture[ x, y ] (H-29)


tempRpuPic[ x, 2 * y + ( 1 − view0OffsetY ) ] =
Clip1( ( 3 * tempPicture[ x, Clip3( 0, y + gMin, TabV − 1 ) ] −
17 * tempPicture[ x, Clip3( 0, y + gMin + 1, TabV − 1 ) ] +
78 * tempPicture[ x, Clip3( 0, y + gMin + 2, TabV − 1 ) ] +
78 * tempPicture[ x, Clip3( 0, y + gMin + 3, TabV − 1 ) ] −
17 * tempPicture[ x, Clip3( 0, y + gMin + 4, TabV − 1 ) ] +
3 * tempPicture[ x, Clip3( 0, y + gMin + 5, TabV − 1 ) ] + 64 ) >> 7 ) (H-30)


with x = 0..SbsV − 1 and y = 0..TabV − 1


d. RpuPicture[ x + leftOffset, y + topOffset ] = tempRpuPic[ x, y ]


with x = 0..SbsV − 1 and y = 0..RpuH − 1 (H-31)





2. RpuPicture[ x, y ] with x = SbsV + leftOffset..RpuW − 1 + leftOffset and
y = topOffset..RpuH − 1 + topOffset is derived from the input of the array refPicture[ x, y ] with
x = leftOffset..RpuW − 1 + leftOffset and y = TabV + topOffset..RpuH − 1 + topOffset as specified by
the following ordered steps:


a. tempRefPic[ x, y ] = refPicture[ x + leftOffset, y + topOffset + TabV ]


with x = 0..RpuW − 1 and y = 0..TabV − 1 (H-32)


b. Apply the following one-dimensional downsampling process:


tempPicture[ x, y ] =
Clip1( ( 4 * tempRefPic[ Clip3( 0, 2 * x − 2, RpuW − 1 ), y ] +
7 * tempRefPic[ Clip3( 0, 2 * x − 1, RpuW − 1 ), y ] +
10 * tempRefPic[ Clip3( 0, 2 * x, RpuW − 1 ), y ] +
7 * tempRefPic[ Clip3( 0, 2 * x + 1, RpuW − 1 ), y ] +
4 * tempRefPic[ Clip3( 0, 2 * x + 2, RpuW − 1 ), y ] + 32 ) >> 6 ) (H-33)


with x = 0..SbsV − 1 and y = 0..TabV − 1


c. Apply the following one-dimensional upsampling process:


gMin = − 2 − View1OffsetY (H-34)


tempRpuPic[ x, 2 * y + view1OffsetY ] = tempPicture[ x, y ] (H-35)


tempRpuPic[ x, 2 * y + ( 1 − view1OffsetY ) ] ] =
Clip1( ( 3 * tempPicture[ x, Clip3( 0, y + gMin, TabV − 1 ) ] −
17 * tempPicture[ x, Clip3( 0, y + gMin + 1, TabV − 1 ) ] +
78 * tempPicture[ x, Clip3( 0, y + gMin + 2, TabV − 1 ) ] +
78 * tempPicture[ x, Clip3( 0, y + gMin + 3, TabV − 1 ) ] −
17 * tempPicture[ x, Clip3( 0, y + gMin + 4, TabV − 1 ) ] +
3 * tempPicture[ x, Clip3( 0, y + gMin + 5, TabV − 1 ) ] + 64 ) >> 7 ) (H-36)


with x = 0..SbsV − 1 and y = 0..TabV − 1


d. RpuPicture[ x + SbsV + leftOffset, y + topOffset ] = tempRpuPic[ x, y ]


with x = 0..SbsV − 1 and y = 0..RpuH − 1 (H-37)


The padded filtered samples of picture sample array RpuPicture[ x, y ] outside frame cropping rectangle with
x = 0..leftOffset − 1 or x = RpuW + leftOffset..refW − 1 or y = 0..topOffset − 1 or y = RpuH + topOffset..refH − 1
are derived as specified by the following ordered steps:


1. RpuPicture[ x, y ] = RpuPicture[ leftOffset, y ]


with x = 0.. leftOffset − 1 and y = topOffset..RpuH − 1+ topOffset (H-38)


2. RpuPicture[ x, y ] = RpuPicture[ RpuW − 1 + leftOffset, y ]


with x = RpuW+ leftOffset..refW − 1 and y = topOffest..RpuH − 1+ topOffset (H-39)


3. RpuPicture[ x, y ] = RpuPicture[ x, topOffset ]


with x = 0..refW − 1 and y = 0..topOffset − 1 (H-40)


4. RpuPicture[ x, y ] = RpuPicture[ x, RpuH − 1 + topOffset]


with x = 0..refW − 1 and y = RpuH+ topOffset..refH − 1 (H-41)


NOTE – If each view component in the base view is a side-by-side frame packing arrangement picture, the inter-view reference
picture is first vertically downsampled and then horizontally upsampled in a conversion to the top-bottom format. Otherwise (each
view component in the base view is a top-bottom frame packing arrangement picture), the inter-view reference picture is first
horizontally downsampled and then vertically upsampled in a conversion to the side-by-side format.


**H.8.5** **Specification of bitstream subsets**


Clauses H.8.5.1 and H.8.5.2 specify the processes for deriving required anchor and non-anchor view components,
respectively, that are used in the sub-bitstream extraction process. Clause H.8.5.3 specifies the sub-bitstream extraction
process. Clause H.8.5.4 specifies the base view bitstream subset. Clause H.8.5.5 gives an informative example for creation
of a base view in case the original base view in the input bitstream to the bitstream extraction process is not included in
the output bitstream subset.





**H.8.5.1** **Derivation process for required anchor view components**


This process is recursively invoked to derive the set of required anchor view components for a specified view.


Input to this process is a variable viewId, representing a view with view_id equal to viewId, with its corresponding view
order index denoted by vOIdx.


Outputs of this process are a possibly updated VOIdxList, and additional invocations of the derivation process based on
the inter-view dependency for anchor view components in the view with view_id equal to viewId as specified in the
sequence parameter set MVC extension.


The following ordered steps are specified:


1. When vOIdx is not already included in VOIdxList, add vOIdx to VOIdxList.


2. Depending on num_anchor_refs_l0[ vOIdx ] and num_anchor_refs_l1[ vOIdx ], the following applies:


     - If both num_anchor_refs_l0[ vOIdx ] and num_anchor_refs_l1[ vOIdx ] are equal to 0, terminate this
process.


     - Otherwise (num_anchor_refs_l0[ vOIdx ] or num_anchor_refs_l1[ vOIdx ] is not equal to 0), the following
ordered steps are specified:


a. When num_anchor_refs_l0[ vOIdx ] is not equal to 0, invoke the process specified in clause H.8.5.1 for
each viewId equal to anchor_ref_l0[ vOIdx ][ i ] for all i in the range of 0 to
num_anchor_refs_l0[ vOIdx ] − 1, inclusive, in ascending order of i.


b. When num_anchor_refs_l1[ vOIdx ] is not equal to 0, invoke the process specified in clause H.8.5.1 for

each viewId equal to anchor_ref_l1[ vOIdx ][ i ] for all i in the range of 0 to
num_anchor_refs_l1[ vOIdx ] − 1, inclusive, in ascending order of i.


**H.8.5.2** **Derivation process for required non-anchor view components**


This process is recursively invoked to derive the set of required non-anchor view components for a specified view.


Input to this process is a variable viewId, representing a view with view_id equal to viewId, with its corresponding view
order index denoted by vOIdx.


Outputs of this process are a possibly updated VOIdxList, and additional invocations of the derivation process based on
the inter-view dependency for non-anchor view components in the view with view_id equal to viewId as specified in the
sequence parameter set MVC extension.


The following ordered steps are specified:


1. When vOIdx is not already included in VOIdxList, add vOIdx to VOIdxList.


2. Depending on num_non_anchor_refs_l0[ vOIdx ] and num_non_anchor_refs_l1[ vOIdx ], the following applies:


     - If both num_non_anchor_refs_l0[ vOIdx ] and num_non_anchor_refs_l1[ vOIdx ] are equal to 0, terminate
this process.


     - Otherwise (num_non_anchor_refs_l0[ vOIdx ] or num_non_anchor_l1[ vOIdx ] is not equal to 0), the
following ordered steps are specified:


a. When num_non_anchor_refs_l0[ vOIdx ] is not equal to 0, invoke the process specified in
clause H.8.5.2 for each viewId equal to non_anchor_ref_l0[ vOIdx ][ i ] for all i in the range of 0 to
num_non_anchor_l0[ vOIdx ] − 1, inclusive, in ascending order of i.


b. When num_non_anchor_refs_l1[ vOIdx ] is not equal to 0, invoke the process specified in

clause H.8.5.2 for each viewId equal to non_anchor_ref_l1[ vOIdx ][ i ] for all i in the range of 0 to
num_non_anchor_l1[ vOIdx ] − 1, inclusive, in ascending order of i.


**H.8.5.3** **Sub-bitstream extraction process**


It is requirement of bitstream conformance that any sub-bitstream that is the output of the process specified in this clause
with pIdTarget equal to any value in the range of 0 to 63, inclusive, tIdTarget equal to any value in the range of 0 to 7,
inclusive, viewIdTargetList consisting of any one or more values of viewIdTarget identifying the views in the bitstream,
shall be conforming to this Recommendation | International Standard.

NOTE 1 – A conforming bitstream contains one or more coded slice NAL units with priority_id equal to 0 and temporal_id equal
to 0.
NOTE 2 – It is possible that not all operation points of sub-bitstreams resulting from the sub-bitstream extraction process have an
applicable level_idc or level_idc[ i ]. In this case, each coded video sequence in a sub-bitstream must still conform to one or more





of the profiles specified in Annex A and Annex H, but may not satisfy the level constraints specified in clauses A.3 and H.10.2,
respectively.


Inputs to this process are:


- a variable pIdTarget (when present),


- a variable tIdTarget (when present),


- a list viewIdTargetList consisting of one or more values of viewIdTarget (when present).


Outputs of this process are a sub-bitstream and a list of VOIdx values VOIdxList.


When pIdTarget is not present as input to this clause, pIdTarget is inferred to be equal to 63.


When tIdTarget is not present as input to this clause, tIdTarget is inferred to be equal to 7.


When viewIdTargetList is not present as input to this clause, there shall be one value of viewIdTarget inferred in
viewIdTargetList and the value of viewIdTarget is inferred to be equal to view_id of the base view.


The sub-bitstream is derived by applying the following operations in sequential order:


1. Let VOIdxList be empty and minVOIdx be the VOIdx value of the base view.


2. For each value of viewIdTarget included in viewIdTargetList, invoke the process specified in clause H.8.5.1 with

the value of viewIdTarget as input.


3. For each value of viewIdTarget included in viewIdTargetList, invoke the process specified in clause H.8.5.2 with

the value of viewIdTarget as input.


4. Mark all VCL NAL units and filler data NAL units for which any of the following conditions are true as "to be

removed from the bitstream":


     - priority_id is greater than pIdTarget,


     - temporal_id is greater than tIdTarget,


     - view_id is not in the viewIdTargetList.


5. Remove all access units for which all VCL NAL units are marked as "to be removed from the bitstream".


6. Remove all VCL NAL units and filler data NAL units that are marked as "to be removed from the bitstream".


7. When VOIdxList contains only one value of VOIdx that is equal to minVOIdx, remove the following NAL units:


     - all NAL units with nal_unit_type equal to 14 or 15,


     - all NAL units with nal_unit_type equal to 6 in which the first SEI message has payloadType in the range
of 36 to 44, inclusive, or equal to 46.


NOTE 3 – When VOIdxList contains only one value of VOIdx equal to minVOIdx, the sub-bitstream contains only the base
view or only a temporal subset of the base view.


8. Let maxTId be the maximum temporal_id of all the remaining VCL NAL units. Remove all NAL units with

nal_unit_type equal to 6 that only contain SEI messages that are part of an MVC scalable nesting SEI message
with any of the following properties:


     - operation_point_flag is equal to 0 and all_view_components_in_au_flag is equal to 0 and none of
sei_view_id[ i ] for all i in the range of 0 to num_view_components_minus1, inclusive, corresponds to a
VOIdx value included in VOIdxList,


     - operation_point_flag is equal to 1 and either sei_op_temporal_id is greater than maxTId or the list of
sei_op_view_id[ i ] for all i in the range of 0 to num_view_components_op_minus1, inclusive, is not a subset
of viewIdTargetList (i.e., it is not true that sei_op_view_id[ i ] for any i in the range of 0 to
num_view_components_op_minus1, inclusive, is equal to a value in viewIdTargetList).


9. Remove each view scalability information SEI message and each operation point not present SEI message, when

present.


10. When VOIdxList does not contain a value of VOIdx equal to minVOIdx, the view with VOIdx equal to the

minimum VOIdx value included in VOIdxList is converted to the base view of the extracted sub-bitstream. An
informative procedure that outlines key processing steps to create a base view is described in clause H.8.5.5.


NOTE 4 – When VOIdxList does not contain a value of VOIdx equal to minVOIdx, the resulting sub-bitstream
according to the operation steps 1-9 above does not contain a base view that conforms to one or more profiles specified
in Annex A. In this case, by this operation step, the remaining view with the new minimum VOIdx value is converted





to be the new base view that conforms to one or more profiles specified in Annex A.


**H.8.5.4** **Specification of the base view bitstream**


A bitstream that conforms to one or more profiles as specified in Annex H shall contain a base view bitstream that conforms
to one or more of the profiles specified in Annex A. This base view bitstream is derived by invoking the sub-bitstream
extraction process as specified in clause H.8.5.3 with no input and the base view bitstream being the output.

NOTE – Although all multiview bitstreams that conform to one or more of the profiles specified in this annex contain a base view
bitstream that conforms to one or more of the profiles specified in Annex A, the complete multiview bitstream (prior to operation of
the base view extraction process specified in this clause) may not conform to any profile specified in Annex A.


**H.8.5.5** **Creation of a base view during sub-bitstream extraction (informative)**


According to the sub-bitstream extraction process specified in clause H.8.5.3, the resulting sub-bitstream shall contain a
base view. When the resulting bitstream does not contain a base view, the following procedure may be used to create a
base view during sub-bitstream extraction.


When VOIdxList does not contain a value of VOIdx equal to minVOIdx, let newBaseViewId be equal to the view_id for
which the VOIdx value is equal to the minimum VOIdx value included in VOIdxList, and apply the following operations
in sequential order:


1. Remove all NAL units with nal_unit_type equal to 7.


2. For all subset sequence parameter set NAL units (with nal_unit_type equal to 15) that are referred to by at least

one remaining VCL NAL unit with view_id equal to newBaseViewId, apply the following operations in
sequential order:


a. Set nal_unit_type to 7.


b. Set profile_idc to 100.


c. Set level_idc to level_idc[ i ], with i equal to the value that for one value of j in the range of 0 to
num_applicable_ops_minus1[ i ], inclusive, applicable_op_temporal_id[ i ][ j ] is equal to maxTId,
applicable_op_num_target_views_minus1[ i ][ j ] is equal to 0, and
applicable_op_target_view_id[ i ][ j ][ k ] for k equal to 0 is equal to newBaseViewId.


d. Remove all the syntax elements after the syntax structure seq_parameter_set_data( ) and before the syntax

structure rbsp_trailing_bits( ), and change RBSP trailing bits appropriately.


3. Remove all SEI NAL units (with nal_unit_type equal to 6) for which the first contained SEI message has

payloadType in the range of 0 to 23, inclusive.


4. For each SEI NAL unit (with nal_unit_type equal to 6) containing an MVC scalable nesting SEI message, the

following operations are applied in sequential order:


a. When none of the following properties is true for the MVC scalable nesting SEI message, the SEI NAL unit
is removed:


        - operation_point_flag is equal to 0 and all_view_components_in_au_flag is equal to 1,


        - operation_point_flag is equal to 0, all_view_components_in_au_flag is equal to 0, and at least one of
the values of sei_view_id[ i ] for all i in the range of 0 to num_view_components_minus1, inclusive, is
equal to the value of one of the viewIdTarget's in viewIdTargetList,


        - operation_point_flag is equal to 1, sei_op_temporal_id is equal to or less than maxtIdT, and the list of
sei_op_view_id[ i ] for all i in the range of 0 to num_view_components_op_minus1, inclusive, is a
subset of viewIdTargetList (i.e., it is true that sei_op_view_id[ i ] for any i in the range of 0 to
num_view_components_op_minus1, inclusive, is equal to a value in viewIdTargetList).


b. When the SEI NAL unit is not removed, the following applies:


        - If VOIdxList contains only one VOIdx value, the SEI NAL unit is replaced by an SEI NAL unit
containing only the original nested SEI message not as part of an MVC scalable nesting SEI message.


        - Otherwise (VOIdxList contains more than one VOIdx value), when any of the following properties is
true for the MVC scalable nesting SEI message, a new SEI NAL unit containing only the nested SEI
message not as part of an MVC scalable nesting SEI message is generated and inserted immediately
before the original SEI NAL unit in decoding order:


           - operation_point_flag is equal to 0 and all_view_components_in_au_flag is equal to 1,





           - operation_point_flag is equal to 0, all_view_components_in_au_flag is equal to 0, and for the
values of sei_view_id[ i ] for all i in the range of 0 to num_view_components_minus1, inclusive,
one is equal to newBaseViewId, and at least another one is equal to the value of one of the
viewIdTarget's in viewIdTargetList.


5. When VOIdxList contains only one value of VOIdx, remove the following NAL units:


      - all NAL units with nal_unit_type equal to 15,


      - all NAL units with nal_unit_type equal to 6 in which the first SEI message has payloadType in the range
of 36 to 44, inclusive.


6. For each NAL unit nalUnit with nal_unit_type equal to 20 and view_id equal to newBaseViewId, the following

operations are applied in sequential order:


a. Depending on non_idr_flag, the following applies:


        - If non_idr_flag is equal to 0, set nal_unit_type equal to 5.


        - Otherwise (non_idr_flag is equal to 1), set nal_unit_type equal to 1.


b. When VOIdxList contains more than one VOIdx value, generate a prefix NAL unit with the same NAL unit

header (including NAL unit header MVC extension) as the NAL unit nalUnit, except that nal_unit_type is
set to 14 and priority_id may be changed, and insert the prefix NAL unit immediately before the NAL unit
nalUnit in decoding order. After the last application of this operation, at least one of all the inserted prefix
NAL units by the applications of this operation shall have priority_id equal to 0.


c. Remove the NAL unit header MVC extension of nalUnit.


**H.8.6** **MFC enhanced resolution picture reconstruction**


This clause does not form an integral part of this Recommendation | International Standard.


This informative clause describes a process for reconstructing the enhanced resolution stereo views from the coded video
sequences conforming to the MFC High profile. The process is applied to each colour component independently.


Inputs of this process are:


- a (refW)x(refH) modified inter-view prediction reference picture sample array RpuPicture derived in clause H.8.4.1,


- a (RpuW)x(RpuH) cropped decoded picture sample array decBasePicture for a luma or a chroma component of the
base view component with VOIdx equal to 0,


- a (RpuW)x(RpuH) cropped decoded picture sample array decEnhPicture for a luma or a chroma component of the
view component with VOIdx equal to 1.


Outputs of this process are:


- a (RpuW)x(RpuH) enhanced resolution picture sample array leftPicture for a luma or a chroma component of the left
view,


- a (RpuW)x(RpuH) enhanced resolution picture sample array rightPicture for a luma or a chroma component of the
right view.


The input sample array decBasePicture is the output cropped picture of the decoded sample arrays SL, SCb or SCr
respectively for each colour component derived in clause 8.7 for a decoded frame or complementary field pair or field of
a decoded frame from a base view. The input sample array decEnhPicture is the output cropped picture of the decoded
sample arrays SL, SCb or SCr respectively for each colour component derived in clause 8.7 for a decoded frame or
complementary field pair or field of a decoded frame of a non- base view.


Depending on the value of rpu_field_processing_flag, the following applies:


- If rpu_field_processing_flag is equal to 1, the modified inter-view prediction reference picture RpuPicture is a field,
the cropped decoded view component of the base view, decBasePicture, is a field, the cropped decoded view
component of the non-base view, decEnhPicture, is a field, the enhanced resolution left view picture leftPicture is a
field, the enhanced resolution right view picture rightPicture is a field, and the enhanced resolution picture
reconstruction is applied to the individual fields separately.


- Otherwise (rpu_field_processing_flag is equal to 0), the modified inter-view prediction reference picture RpuPicture
is a frame, the cropped decoded view component of the base view, decBasePicture, is a frame, the cropped decoded
view component of the non-base view, decEnhPicture, is a frame, the enhanced resolution left view picture leftPicture





is a frame, the enhanced resolution right view picture rightPicture is a frame, and the enhanced resolution picture
reconstruction is applied to the frame.


The mathematical function Clip1( ) is defined with Clip1( ) being substituted with Clip1Y( ) for the luma component and
Clip1( ) being substituted with Clip1C( ) for the chroma components, respectively.


The variable tVal is set equal to ( 1 << ( BitDepthY − 1 ) ) for the luma component and tVal is set equal to
( 1 << ( BitDepthC − 1 ) ) for a chroma component, respectively.


Let upBasePic[ x, y ] be an (RpuW)x(RpuH) array of samples with x = 0..RpuW − 1 and y = 0..RpuH − 1.


Let resPicture[ x, y ] be an (RpuW)x(RpuH) array of samples with x = 0..RpuW − 1 and y = 0..RpuH − 1.


Let upResPic[ x, y ] be an (RpuW)x(RpuH) array of samples with x = 0..RpuW − 1 and y = 0..RpuH − 1.


- If mfc_format_idc is equal to 0, let tempDecBasePic[ x, y ] be a (SbsV)x(RpuH) array of samples with x = 0..SbsV − 1
and y = 0..RpuH − 1, tempDecEnhPic[ x, y ] be a (RpuW)x(TabV) array of samples with x = 0..RpuW − 1 and
y = 0..TabV − 1, and tempRpuPic[ x, y ] be a (RpuW)x(TabV) array of samples with x = 0..RpuW − 1 and
y = 0..TabV − 1.


- Otherwise (mfc_format_idc is equal to 1), let tempDecBasePic[ x, y ] be a (RpuW)x(TabV) array of samples with
x = 0..RpuW − 1 and y = 0..TabV − 1, tempDecEnhPic[ x, y ] be a (SbsV)x(RpuH) array of samples with
x = 0..SbsV − 1 and y = 0..RpuH − 1, and tempRpuPic[ x, y ] be a (SbsV)x(RpuH) array of samples with
x = 0..SbsV − 1 and y = 0..RpuH − 1.


The samples of enhanced resolution picture sample array for the left view leftPicture[ x, y ] and the right view
rightPicture[ x, y ] with x = 0..RpuW − 1 and y = 0..RpuH − 1 are derived as follows:


- If mfc_format_idc is equal to 0, the following applies:


1. leftPicture[ x, y ] with x = 0..RpuW − 1 and y = 0..RpuH − 1 is derived from the input of the arrays

decBasePicture[ x, y ] with x = 0..SbsV − 1 and y = 0..RpuH − 1, decEnhPicture[ x, y ] with x = 0..RpuW − 1 and
y = 0..TabV − 1 and RpuPicture[ x, y ] with x = leftOffset..RpuW − 1 + leftOffset and
y = topOffset..TabV − 1 + topOffset as specified by the following ordered steps:


a. tempDecBasePic[ x, y ] = decBasePicture[ x, y ] (H-42)


with x = 0..SbsV − 1 and y = 0..RpuH − 1


b. tempDecEnhPic[ x, y ] = decEnhPicture[ x, y ] (H-43)


with x = 0..RpuW − 1 and y = 0..TabV − 1


c. tempRpuPic[ x, y ] = RpuPicture[ x + leftOffset, y + topOffset ] (H-44)


with x = 0..RpuW− 1 and y = 0..TabV − 1


d. Apply the following one-dimensional upsampling process:


gMin = − 2 − View0OffsetX (H-45)


upBasePic[ 2 * x + view0OffsetX, y ] = tempDecBasePic[ x, y ] (H-46)


upBasePic[ 2 * x + ( 1 − view0OffsetX ), y ] =
Clip1( ( 3 * tempDecBasePic[ Clip3( 0, x + gMin, SbsV − 1 ), y ] −
17 * tempDecBasePic[ Clip3( 0, x + gMin + 1, SbsV − 1 ), y ] +
78 * tempDecBasePic[ Clip3( 0, x + gMin + 2, SbsV − 1 ), y ] +
78 * tempDecBasePic[ Clip3( 0, x + gMin + 3, SbsV − 1 ), y ] −
17 * tempDecBasePic[ Clip3( 0, x + gMin + 4, SbsV − 1 ), y ] +
3 * tempDecBasePic[ Clip3( 0, x + gMin + 5, SbsV − 1 ), y ] + 64 ) >> 7 ) (H-47)


with x = 0..SbsV − 1 and y = 0..RpuH − 1


e. resPicture[ x, y ] = Clip3( − tVal, tVal − 1, ( tempDecEnhPic[ x, y ] − tempRpuPic[ x, y ] ) ) (H-48)


with x = 0..RpuW − 1 and y = 0..TabV − 1


f. Apply the following one-dimensional upsampling process:


upResPic[ x, 2 * y] = resPicture[ x, y ] (H-49)


upResPic[ x, 2 * y + 1 ] =
Clip3( − tVal, tVal − 1, ( 3 * resPicture[ x, Clip3( 0,y − 2, TabV − 1 ) ] −
17 * resPicture[ x, Clip3( 0,y − 1, TabV − 1 ) ] +





78 * resPicture[ x, Clip3( 0, y, TabV − 1 ) ] +
78 * resPicture[ x, Clip3( 0, y + 1, TabV − 1 ) ] −
17 * resPicture[ x, Clip3( 0,y + 2, TabV − 1 ) ] +
3 * resPicture[ x, Clip3( 0, y + 3, TabV − 1 ) ] + 64 ) >> 7 ) ) (H-50)


with x = 0..RpuW − 1 and y = 0..TabV − 1


g. leftPicture[ x, y ] = Clip1( upBasePic[ x, y ] + upResPic[ x, y ] ) (H-51)


with x = 0..RpuW − 1 and y = 0..RpuH − 1


2. rightPicture[ x, y ] with x = 0..RpuW − 1 and y = 0..RpuH − 1 is derived from the input of the arrays

decBasePicture[ x, y ] with x = SbsV..RpuW − 1 and y = 0..RpuH − 1, decEnhPicture[ x, y ] with
x = 0..RpuW − 1 and y = TabV..RpuH − 1 and RpuPicture[ x, y ] with x = leftOffset..RpuW − 1 + leftOffset and
y = TabV + topOffset..RpuH − 1 + topOffset as specified by the following ordered steps:


a. tempDecBasePic[ x, y ] = decBasePicture[ x + SbsV, y ] (H-52)


with x = 0..SbsV − 1 and y = 0..RpuH − 1


b. tempDecEnhPic[ x, y ] = decEnhPicture[ x, y + TabV ] (H-53)


with x = 0..RpuW − 1 and y = 0..TabV − 1


c. tempRpuPic[ x, y ] = RpuPicture[ x + leftOffset, y + TabV + topOffset ] (H-54)


with x = 0..RpuW− 1 and y = 0..TabV − 1


d. Apply the following one-dimensional upsampling process:


gMin = − 2 − View1OffsetX, (H-55)


upBasePic[ 2 * x + view1OffsetX, y ] = tempDecBasePic[ x, y ], (H-56)


upBasePic[ 2 * x + ( 1 − view1OffsetX ), y ] =
Clip1( ( 3 * tempDecBasePic[ Clip3( 0, x + gMin, SbsV − 1 ), y ] −
17 * tempDecBasePic[ Clip3( 0, x + gMin + 1, SbsV − 1 ), y ] +
78 * tempDecBasePic[ Clip3( 0, x + gMin + 2, SbsV − 1 ), y ] +
78 * tempDecBasePic[ Clip3( 0, x + gMin + 3, SbsV − 1 ), y ] −
17 * tempDecBasePic[ Clip3( 0, x + gMin + 4, SbsV − 1 ), y ] +
3 * tempDecBasePic[ Clip3( 0, x + gMin + 5, SbsV − 1 ), y ] + 64 ) >> 7 ) (H-57)


with x = 0..SbsV − 1 and y = 0..RpuH − 1


e. resPicture[ x, y ] = Clip3( − tVal, tVal − 1, ( tempDecEnhPic[ x, y ] − tempRpuPic[ x, y ] ) ) (H-58)


with x = 0..RpuW − 1 and y = 0..TabV − 1


f. Apply the following one-dimensional upsampling process:


upResPic[ x, 2 * y] = resPicture[ x, y ] (H-59)


upResPic[ x, 2 * y + 1 ] =
Clip3( − tVal, tVal − 1, ( 3 * resPicture[ x, Clip3( 0,y − 2, TabV − 1 ) ] −
17 * resPicture[ x, Clip3( 0, y − 1, TabV − 1 ) ] +
78 * resPicture[ x, Clip3( 0, y, TabV − 1 ) ] +
78 * resPicture[ x, Clip3( 0, y + 1, TabV − 1 ) ] −
17 * resPicture[ x, Clip3( 0, y + 2, TabV − 1 ) ] +
3 * resPicture[ x, Clip3( 0, y + 3, TabV − 1 ) ] + 64 ) >> 7 ) ) (H-60)


with x = 0..RpuW − 1 and y = 0..TabV − 1


g. rightPicture[ x, y ] = Clip1( upBasePic[ x, y ] + upResPic[ x, y ] ) (H-61)


with x = 0..RpuW − 1 and y = 0..RpuH − 1


- Otherwise ( mfc_format_idc is equal to 1 ), the following applies:


1. leftPicture[ x, y ] with x = 0..RpuW − 1 and y = 0..RpuH − 1 is derived from the input of the arrays

decBasePicture[ x, y ] with x = 0..RpuW − 1 and y = 0..TabV − 1, decEnhPicture[ x, y ] with x = 0..SbsV − 1 and
y = 0..RpuH − 1 and RpuPicture[ x, y ] with x = leftOffset..SbsV − 1 + leftOffset and
y = topOffset..RpuH − 1 + topOffset as specified by the following ordered steps:


a. tempDecBasePic[ x, y ] = decBasePicture[ x, y ] (H-62)





with x = 0..RpuW − 1 and y = 0..TabV − 1


b. tempDecEnhPic[ x, y ] = decEnhPicture[ x, y ] (H-63)


with x = 0..SbsV − 1 and y = 0..RpuH − 1


c. tempRpuPic[ x, y ] = RpuPicture[ x + leftOffset, y + topOffset ] (H-64)


with x = 0..SbsV− 1 and y = 0..RpuH − 1


d. Apply the following one-dimensional upsampling process:


gMin = − 2 − View0OffsetY (H-65)


upBasePic[ x, 2 * y + view0OffsetY ] = tempDecBasePic[ x, y ] (H-66)


upBasePic[ x, 2 * y + ( 1 − view0OffsetY ) ] =
Clip1( ( 3 * tempDecBasePic[ x, Clip3( 0, y + gMin, TabV − 1 ) ] −
17 * tempDecBasePic[ x, Clip3( 0, y + gMin + 1, TabV − 1 ) ] +
78 * tempDecBasePic[ x, Clip3( 0, y + gMin + 2, TabV − 1 ) ] +
78 * tempDecBasePic[ x, Clip3( 0, y + gMin + 3, TabV − 1 ) ] −
17 * tempDecBasePic[ x, Clip3( 0, y + gMin + 4, TabV − 1 ) ] +
3 * tempDecBasePic[ x, Clip3( 0, y + gMin + 5, TabV − 1 ) ] + 64 ) >> 7 ) (H-67)


with x = 0..RpuW − 1 and y = 0..TabV − 1


e. resPicture[ x, y ] = Clip3( − tVal, tVal − 1, ( tempDecEnhPic[ x, y ] − tempRpuPic[ x, y ] ) ) (H-68)


with x = 0..SbsV − 1 and y = 0..RpuH − 1


f. Apply the following one-dimensional upsampling process:


upResPic[ 2 * x, y ] = resPicture[ x, y ] (H-69)


upResPic[ 2 * x + 1, y ] =
Clip3(− tVal, tVal − 1, ( 3 * resPicture[ Clip3( 0, x − 2, TabV − 1 ), y ] −
17 * resPicture[ Clip3( 0, x − 1, SbsV − 1 ), y ] +
78* resPicture[ Clip3( 0, x, SbsV − 1 ), y ] +
78* resPicture[ Clip3( 0, x + 1, SbsV − 1 ), y ] −
17 * resPicture[ Clip3( 0, x + 2, SbsV − 1 ), y ] +
3* resPicture[ Clip3( 0, x + 3, SbsV − 1 ), y ] + 64 ) >> 7 ) ) (H-70)


with x = 0..SbsV − 1 and y = 0..RpuH − 1


g. leftPicture[ x, y ] = Clip1( upBasePic[ x, y ] + upResPic[ x, y ] ) (H-71)


with x = 0..RpuW − 1 and y = 0..RpuH − 1


2. rightPicture[ x, y ] with x = 0..RpuW − 1 and y = 0..RpuH − 1 is derived from the input of the arrays

decBasePicture[ x, y ] with x = 0..RpuW − 1 and y = TabV..RpuH − 1, decEnhPicture[ x, y ] with
x = SbsV..RpuW − 1 and y = 0..RpuH − 1 and RpuPicture[ x, y ] with
x = SbsV + leftOffset..RpuW − 1 + leftOffset and y = topOffset..RpuH − 1 + topOffset as specified by the
following ordered steps:


a. tempDecBasePic[ x, y ] = decBasePicture[ x, y + TabV ] (H-72)


with x = 0..RpuW − 1 and y = 0..TabV − 1


b. tempDecEnhPic[ x, y ] = decEnhPicture[ x + SbsV, y ] (H-73)


with x = 0..SbsV − 1 and y = 0..RpuH − 1


c. tempRpuPic[ x, y ] = RpuPicture[ x + SbsV + leftOffset, y + topOffset ] (H-74)


with x = 0..SbsV− 1 and y = 0..RpuH − 1


d. Apply the following one-dimensional upsampling process:


gMin = − 2 − View1OffsetY (H-75)


upBasePic[ x, 2 * y + view1OffsetY ] = tempDecBasePic[ x, y ] (H-76)


upBasePic[ x, 2 * y + ( 1 − view1OffsetY ) ] =
Clip1( ( 3 * tempDecBasePic[ x, Clip3( 0, y + gMin, TabV − 1 ) ] −
17 * tempDecBasePic[ x, Clip3( 0, y + gMin + 1, TabV − 1 ) ] +





78 * tempDecBasePic[ x, Clip3( 0, y + gMin + 2, TabV − 1 ) ] +
78 * tempDecBasePic[ x, Clip3( 0, y + gMin + 3, TabV − 1 ) ] −
17 * tempDecBasePic[ x, Clip3( 0, y + gMin + 4, TabV − 1 ) ] +
3 * tempDecBasePic[ x, Clip3( 0, y + gMin + 5, TabV − 1 ) ] + 64 ) >> 7 ) (H-77)


with x = 0..RpuW − 1 and y = 0..TabV − 1


e. resPicture[ x, y ] = Clip3( − tVal, tVal − 1, ( tempDecEnhPic[ x, y ] − tempRpuPic[ x, y ] ) ) (H-78)


with x = 0..SbsV − 1 and y = 0..RpuH − 1


f. Apply the following one-dimensional upsampling process:


upResPic[ 2 * x, y ] = resPicture[ x, y ] (H-79)


upResPic[ 2 * x + 1, y ] =
Clip3(− tVal, tVal − 1, ( 3 * resPicture[ Clip3( 0, x − 2, SbsV − 1 ), y ] −
17 * resPicture[ Clip3( 0, x − 1, SbsV − 1 ), y ] +
78* resPicture[ Clip3( 0, x, SbsV − 1 ), y ] +
78* resPicture[ Clip3( 0, x + 1, SbsV − 1 ), y ] −
17 * resPicture[ Clip3( 0, x + 2, SbsV − 1 ), y ] +
3* resPicture[ Clip3( 0, x + 3, SbsV − 1 ), y ] + 64 ) >> 7 ) ) (H-80)


with x = 0..SbsV − 1 and y = 0..RpuH − 1


g. rightPicture[ x, y ] = Clip1( upBasePic[ x, y ] + upResPic[ x, y ] ) (H-81)


with x = 0..RpuW − 1 and y = 0..RpuH − 1
