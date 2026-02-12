**C.4** **Decoder conformance**


A decoder conforming to this Recommendation | International Standard fulfils the following requirements.


A decoder claiming conformance to a specific profile and level shall be able to decode successfully all conforming
bitstreams specified for decoder conformance in clause C.3, provided that all sequence parameter sets and picture
parameter sets referred to in the VCL NAL units, and appropriate buffering period and picture timing SEI messages are
conveyed to the decoder, in a timely manner, either in the bitstream (by non-VCL NAL units), or by external means not
specified by this Recommendation | International Standard.


There are two types of conformance that can be claimed by a decoder: output timing conformance and output order
conformance.


To check conformance of a decoder, test bitstreams conforming to the claimed profile and level, as specified in clause C.3
are delivered by a hypothetical stream scheduler (HSS) both to the HRD and to the decoder under test (DUT). All pictures





output by the HRD shall also be output by the DUT and, for each picture output by the HRD, the values of all samples that
are output by the DUT for the corresponding picture shall be equal to the values of the samples output by the HRD.


For output timing decoder conformance, the HSS operates as described above, with delivery schedules selected only from
the subset of values of SchedSelIdx for which the bit rate and CPB size are restricted as specified in Annex A, Annex G,
Annex H, and Annex I for the specified profile and level, or with "interpolated" delivery schedules as specified below for
which the bit rate and CPB size are restricted as specified in Annex A, Annex G, Annex H, and Annex I. The same delivery
schedule is used for both the HRD and DUT.


When the HRD parameters and the buffering period SEI messages are present with cpb_cnt_minus1 greater than 0, the
decoder shall be capable of decoding the bitstream as delivered from the HSS operating using an "interpolated" delivery
schedule specified as having peak bit rate r, CPB size c( r ), and initial CPB removal delay ( f( r ) ÷ r ) as follows:


 = ( r − BitRate[ SchedSelIdx − 1 ] ) ÷ ( BitRate[ SchedSelIdx ] − BitRate[ SchedSelIdx − 1 ] ), (C-17)


c( r ) =       - CpbSize[ SchedSelIdx ] + (1 − )      - CpbSize[ SchedSelIdx−1 ], (C-18)


f( r ) =   initial_cpb_removal_delay[ SchedSelIdx ] * BitRate[ SchedSelIdx ] +
( 1 −  )  initial_cpb_removal_delay[ SchedSelIdx − 1 ] * BitRate[ SchedSelIdx − 1 ] (C-19)


for any SchedSelIdx > 0 and r such that BitRate[ SchedSelIdx − 1 ] <= r <= BitRate[ SchedSelIdx ] such that r and c( r )
are within the limits as specified in Annex A, Annex G, Annex H, and Annex I for the maximum bit rate and buffer size
for the specified profile and level.

NOTE 1 – initial_cpb_removal_delay[ SchedSelIdx ] can be different from one buffering period to another and have to be
re-calculated.


For output timing decoder conformance, an HRD as described above is used and the timing (relative to the delivery time
of the first bit) of picture output is the same for both HRD and the DUT up to a fixed delay.


For output order decoder conformance, the HSS delivers the bitstream to the DUT "by demand" from the DUT, meaning
that the HSS delivers bits (in decoding order) only when the DUT requires more bits to proceed with its processing.

NOTE 2 – This means that for this test, the coded picture buffer of the DUT could be as small as the size of the largest access unit.


A modified HRD as described below is used, and the HSS delivers the bitstream to the HRD by one of the schedules
specified in the bitstream such that the bit rate and CPB size are restricted as specified in Annex A, Annex G, Annex H,
and Annex I. The order of pictures output shall be the same for both HRD and the DUT.


For output order decoder conformance, the HRD CPB size is equal to CpbSize[ SchedSelIdx ] for the selected schedule
and the DPB size is equal to MaxDpbFrames. Removal time from the CPB for the HRD is equal to final bit arrival time
and decoding is immediate. The operation of the DPB of this HRD is specified in clause C.4.1.


**C.4.1** **Operation of the output order DPB**


The decoded picture buffer contains frame buffers. When a coded video sequence conforming to one or more of the profiles
specified in Annex A is decoded by applying the decoding process specified in clauses 2 to 9, each of the frame buffers
may contain a decoded frame, a decoded complementary field pair or a single (non-paired) decoded field that is marked as
"used for reference" or is held for future output (reordered pictures). When a coded video sequence conforming to one or
more of the profiles specified in Annex G is decoded by applying the decoding process specified in Annex G, each frame
buffer may contain a decoded frame, a decoded complementary field pair, a single (non-paired) decoded field, a decoded
reference base frame, a decoded reference base complementary field pair or a single (non-paired) decoded reference base
field that is marked as "used for reference" (reference pictures) or is held for future output (reordered or delayed pictures).
When a coded video sequence conforming to one or more of the profiles specified in Annex H is decoded by applying the
decoding process specified in Annex H, each of the frame buffers may contain a decoded frame view component, a decoded
complementary field view component pair, or a single (non-paired) decoded field view component that is marked as "used
for reference" (reference pictures) or is held for future output (reordered or delayed pictures) or is held for inter-view
prediction (inter-view only reference components). When a coded video sequence conforming to one or more of the profiles
specified in Annex I is decoded by applying the decoding process specified in Annex I, each of the frame buffers of the
texture DPB may contain a decoded texture frame view component, a decoded complementary texture field view
component pair, a single (non-paired) decoded texture field view component that is marked as "used for reference"
(reference pictures) or is held for future output (reordered or delayed pictures) or is held for inter-view prediction (interview only reference components). When a coded video sequence conforming to one or more of the profiles specified in
Annex I is decoded by applying the decoding process specified in Annex I, each of the frame buffers of the depth DPB
may contain a decoded depth frame view component, a decoded complementary depth field view component pair, or a
single (non-paired) decoded depth field view component that is marked as "used for reference" (reference pictures) or is
held for future output (reordered or delayed pictures) or is held as reference for inter-view prediction (inter-view only
reference components).





At HRD initialization, the DPB fullness, measured in non-empty frame buffers, is set equal to 0. The following steps all
happen instantaneously when an access unit is removed from the CPB, and in the order listed. When the decoding process
specified in Annex H or Annex I is applied, the view components of the current primary coded picture are processed by
applying the ordered steps to each view component in increasing order of the associated view order index VOIdx. The
invocation of the process for a depth view component, if present, follows the invocation of the process for the texture view
component within the same view component.


1. The process of decoding gaps in frame_num and storing "non-existing" frames as specified in clause C.4.2 is

invoked.


2. The picture decoding and output process as specified in clause C.4.3 is invoked.


3. The process of removing pictures from the DPB before possible insertion of the current picture as specified in

clause C.4.4 is invoked.


4. The process of marking and storing the current decoded picture as specified in clause C.4.5 is invoked.

NOTE – When the decoding process specified in Annex G is applied, the DPB is only operated for decoded pictures and
reference base pictures associated with decoded pictures. The DPB is not operated for layer pictures with dependency_id less
than DependencyIdMax (and associated reference base pictures). All decoded pictures and associated reference base pictures are
decoded pictures and associated reference base pictures for dependency_id equal to DependencyIdMax, which represent the
results of the decoding process specified in clause G.8.


**C.4.2** **Decoding of gaps in frame_num and storage of "non-existing" pictures**


When the decoding process specified in Annex H is applied, the process specified in this clause is invoked for a particular
view with view order index VOIdx, with "picture" being replaced by "view component", "frame" being replaced by "frame
view component", and "field" being replaced by "field view component". During the invocation of the process for a
particular view, only view components of the particular view are considered and view components of other views are not
marked as "unused for reference" or removed from the DPB.


When the decoding process specified in Annex I is applied, the process specified in this clause for Annex H is invoked for
particular texture view and depth view with view order index VOIdx, with each "view component" being replaced by
"texture view component" or "depth view component", "frame view component" being replaced by "texture frame view
component" or "depth frame view component", and "field view component" being replaced by "texture field view
component". During the invocation of the process for a particular texture view, only the texture view components of the
particular view are considered and during the invocation of the process for a particular depth view, only the depth view
components of the particular view are considered and view components of other views are not marked as "unused for
reference" or removed from the DPB.


The DPB fullness represents the total number of non-empty frame buffers. When the decoding process specified in
Annex H is applied, this includes frame buffers that contain view components of other views. When the decoding process
specified in Annex I is applied, this includes frame buffers that contain texture or depth view components of other views.


When applicable, gaps in frame_num are detected by the decoding process and the necessary number of "non-existing"
frames are inferred in the order specified by the generation of values of UnusedShortTermFrameNum in Equation 7-24
and are marked as specified in clauses 8.2.5.2 and G.8.2.5. Frame buffers containing a frame or a complementary field pair
or a non-paired field which are marked as "not needed for output" and "unused for reference" are emptied (without output),
and the DPB fullness is decremented by the number of frame buffers emptied. Each "non-existing" frame is stored in the
DPB as follows:


- When there is no empty frame buffer (i.e., DPB fullness is equal to DPB size), the "bumping" process specified in
clause C.4.5.3 is invoked repeatedly until there is an empty frame buffer in which to store the "non-existing" frame.


- The "non-existing" frame is stored in an empty frame buffer and is marked as "not needed for output", and the DPB
fullness is incremented by one.


**C.4.3** **Picture decoding**


When the decoding process specified in Annex H is applied, the process specified in this clause is invoked for a particular
view with view order index VOIdx.


When the decoding process specified in Annex I is applied, the process specified for Annex H in this clause is invoked for
a particular texture view and depth view with view order index VOIdx.


The decoding of the current picture or view component (when applying the decoding process specified in Annex H or
Annex I) is specified as follows:


- If the decoding process specified in clause 8 or Annex G is applied, the current primary coded picture n is decoded
and is temporarily stored (not in the DPB).





- Otherwise (the decoding process specified in Annex H or Annex I is applied), the view component with view order
index VOIdx of the current primary coded picture n is decoded and is temporarily stored (not in the DPB).


**C.4.4** **Removal of pictures from the DPB before possible insertion of the current picture**


When the decoding process specified in Annex H is applied, the process specified in this clause is invoked for a particular
view with view order index VOIdx, with "picture" being replaced by "view component", "frame" being replaced by "frame
view component", and "field" being replaced by "field view component".


When the decoding process specified in Annex I is applied, the process specified in this clause for Annex H is invoked for
particular texture view and depth view with view order index VOIdx, with each "view component" being replaced by
"texture view component" or "depth view component", "frame view component" being replaced by "texture frame view
component" or "depth frame view component", and "field view component" being replaced by "texture field view
component". During the invocation of the process for a particular texture view, only the texture view components of the
particular view are considered and during the invocation of the process for a particular depth view, only the depth view
components of the particular view are considered.


When the decoding process specified in Annex H or Annex I is applied, the following process is specified for emptying
frame buffers containing inter-view only reference components of the current access unit. By this process, frame buffers
that contain view components of the current view with view order index VOIdx are not emptied, but frame buffers that
contain inter-view only reference components of other views may be emptied. The process is specified as follows:


- If the view order index VOIdx of the current view is equal to VOIdxMax, all frame buffers containing a frame or a
complementary field pair or a non-paired field which are marked as "not needed for output" and "unused for reference"
are emptied (without output), and the DPB fullness is decremented by the number of frame buffers emptied.

NOTE 1 – At this stage of the process, all frame buffers that contain a frame or a complementary field pair or a
non-paired field marked as "not needed for output" and "unused for reference" are frame buffers that contain an interview only reference component (of the current access unit and a view with view order index less than VOIdx) with
OutputFlag equal to 0.


- Otherwise (the view order index VOIdx of the current view is less than VOIdxMax), frame buffers containing a frame
or a complementary field pair or a non-paired field for which both of the following conditions are true are emptied
(without output), and the DPB fullness is decremented by the number of frame buffers emptied:


   - the frame or complementary field pair or non-paired field is marked as "not needed for output" and "unused for
reference",

NOTE 2 – At this stage of the process, all frame buffers that contain a frame or a complementary field pair or a
non-paired field marked as "not needed for output" and "unused for reference" are frame buffers that contain an interview only reference component (of the current access unit and a view with view order index less than VOIdx) with
OutputFlag equal to 0.


   - one of the following conditions is true:


      - the current view component is a view component of an anchor picture and the view_id of the frame or
complementary field pair or non-paired field is not equal to any value of anchor_ref_lX[ k ][ j ], with X
being equal to 0 or 1, k being any integer value greater than the view order index VOIdx of the current view,
and j being any integer value in the range of 0 to Max( 0, num_anchor_refs_lX[ k ] − 1 ), inclusive,


      - the current view component is not a view component of an anchor picture and the view_id of the frame or
complementary field pair or non-paired field is not equal to any value of non_anchor_ref_lX[ k ][ j ], with
X being equal to 0 or 1, k being any integer value greater than the view order index VOIdx of the current
view, and j being any integer value in the range of 0 to Max( 0, num_non_anchor_refs_lX[ k ] − 1 ),
inclusive.


When the decoding process specified in Annex H or Annex I is applied, for the following processes specified in this clause,
only view components of the particular view for which this clause is invoked are considered, and frame buffers containing
view components of other views are not emptied. The DPB fullness represents the total number of non-empty frame buffers,
including frame buffers that contain view components of other views.


The removal of pictures from the DPB before possible insertion of the current picture proceeds as follows:


- If the decoded picture is an IDR picture the following applies:

1. All reference pictures in the DPB are marked as "unused for reference" as specified in clause 8.2.5 when a

coded video sequence conforming to one or more of the profiles specified in Annex A is decoded by applying
the decoding process specified in clauses 2 to 9, or as specified in clause G.8.2.4 when a coded video sequence
conforming to one or more of the profiles specified in Annex G is decoded by applying the decoding process
specified in Annex G, or as specified in clause H.8.3 when a coded video sequence conforming to one or more
of the profiles specified in Annex H is decoded by applying the decoding process specified in Annex H, or as





specified in clause I.8.3 when a coded video sequence conforming to one or more of the profiles specified in
Annex I is decoded by applying the decoding process specified in Annex I.

2. When the IDR picture is not the first IDR picture decoded and the value of PicWidthInMbs or

FrameHeightInMbs or max_dec_frame_buffering derived from the active sequence parameter set is different
from the value of PicWidthInMbs or FrameHeightInMbs or max_dec_frame_buffering derived from the
sequence parameter set that was active for the preceding picture, respectively, no_output_of_prior_pics_flag is
inferred to be equal to 1 by the HRD, regardless of the actual value of no_output_of_prior_pics_flag.

NOTE 3 – Decoder implementations should try to handle changes in the value of PicWidthInMbs or FrameHeightInMbs
or max_dec_frame_buffering more gracefully than the HRD.

3. When no_output_of_prior_pics_flag is equal to 1 or is inferred to be equal to 1, all frame buffers in the DPB

are emptied without output of the pictures they contain, and DPB fullness is set to 0.


- Otherwise (the decoded picture is not an IDR picture), the decoded reference picture marking process is invoked as
specified in clause 8.2.5 when a coded video sequence conforming to one or more of the profiles specified in Annex A
is decoded by applying the decoding process specified in clauses 2 to 9, or as specified in clause G.8.2.4 when a coded
video sequence conforming to one or more of the profiles specified in Annex G is decoded by applying the decoding
process specified in Annex G, or as specified in clause H.8.3 when a coded video sequence conforming to one or
more of the profiles specified in Annex H is decoded by applying the decoding process specified in Annex H, or as
specified in clause I.8.3 when a coded video sequence conforming to one or more of the profiles specified in Annex I
is decoded by applying the decoding process specified in Annex I. Frame buffers containing a frame or a
complementary field pair or a non-paired field which are marked as "not needed for output" and "unused for reference"
are emptied (without output), and the DPB fullness is decremented by the number of frame buffers emptied.


When the current picture has a memory_management_control_operation equal to 5 or is an IDR picture for which
no_output_of_prior_pics_flag is not equal to 1 and is not inferred to be equal to 1, the following two steps are performed.

1. Frame buffers containing a frame or a complementary field pair or a non-paired field which are marked as "not
needed for output" and "unused for reference" are emptied (without output), and the DPB fullness is decremented
by the number of frame buffers emptied.

2. All non-empty frame buffers in the DPB are emptied by repeatedly invoking the "bumping" process specified in
clause C.4.5.3, and the DPB fullness is set to 0.


**C.4.5** **Current decoded picture marking and storage**


When the decoding process specified in Annex H is applied, the process specified in this clause is invoked for a particular
view with view order index VOIdx, with "picture" being replaced by "view component", "frame" being replaced by "frame
view component", and "field" being replaced by "field view component". During the invocation of the process for a
particular view, only view components of the particular view are considered and frame buffers containing view components
of other views are not emptied.


When the decoding process specified in Annex I is applied, the process specified in this clause for Annex H is invoked for
particular texture view and depth view with view order index VOIdx, with each "view component" being replaced by
"texture view component" or "depth view component", "frame view component" being replaced by "texture frame view
component" or "depth frame view component", and "field view component" being replaced by "texture field view
component". During the invocation of the process for a particular texture view, only the texture view components of the
particular view are considered and during the invocation of the process for a particular depth view, only the depth view
components of the particular view are considered and frame buffers containing view components of other views are not
emptied.


The DPB fullness represents the total number of non-empty frame buffers. When the decoding process specified in
Annex H is applied, this includes frame buffers that contain view components of other views. When the decoding process
specified in Annex I is applied, this includes frame buffers that contain texture or depth view components of other views.


The marking and storage of the current decoded picture is specified as follows:


- If the current picture is a reference picture, the storage and marking process for decoded reference pictures as specified
in clause C.4.5.1 is invoked.


- Otherwise (the current picture is a non-reference picture), the storage and marking process for decoded non-reference
pictures as specified in clause C.4.5.2 is invoked.





**C.4.5.1** **Storage and marking of a reference decoded picture into the DPB**


The current picture is stored in the DPB as follows:


- If the current decoded picture is the second field (in decoding order) of a complementary reference field pair, and the
first field of the pair is still in the DPB, the current picture is stored in the same frame buffer as the first field of the
pair and the following applies:

   - If the current decoded picture has OutputFlag equal to 1, it is marked as "needed for output".

   - Otherwise (the current decoded picture has OutputFlag equal to 0), it is marked as "not needed for output".


- Otherwise, the following operations are performed:

1. When there is no empty frame buffer (i.e., DPB fullness is equal to DPB size), the "bumping" process specified

in clause C.4.5.3 is invoked repeatedly until there is an empty frame buffer in which to store the current decoded
picture.

2. The current decoded picture is stored in an empty frame buffer, the DPB fullness is incremented by one, and

the following applies:

      - If the current decoded picture has OutputFlag equal to 1, it is marked as "needed for output".

      - Otherwise (the current decoded picture has OutputFlag equal to 0), it is marked as "not needed for output".


When the coded video sequence conforms to one or more of the profiles specified in Annex G and the decoding process
specified in Annex G is applied and the current picture has store_ref_base_pic_flag equal to 1 (i.e., the current picture is
associated with a reference base picture), the associated reference base picture is stored in the DPB as follows:


- If the reference base picture is a second field (in decoding order) of a complementary reference base field pair, and
the first field of the pair is still in the DPB, the reference base picture is stored in the same frame buffer as the first
field of the pair and marked as "not needed for output".


- Otherwise, the following operations are performed:

1. When there is no empty frame buffer (i.e., DPB fullness is equal to DPB size), the "bumping" process

specified in clause C.4.5.3 is invoked repeatedly until there is an empty frame buffer in which to store the
reference base picture.

2. The reference base picture is stored in an empty frame buffer and marked as "not needed for output" and the

DPB fullness is incremented by one.


**C.4.5.2** **Storage and marking of a non-reference decoded picture into the DPB**


The current picture is associated with a variable StoreInterViewOnlyRefFlag, which is derived as follows:


- If the decoding process specified in Annex H or Annex I is applied, the current view component has a view order
index VOIdx less than VOIdxMax and inter_view_flag equal to 1, StoreInterViewOnlyRefFlag is set equal to 1.


- Otherwise, StoreInterViewOnlyRefFlag is set equal to 0.


The current picture is stored in the DPB or output as follows:

- If the current decoded picture is the second field (in decoding order) of a complementary non-reference field pair and
the first field of the pair is still in the DPB, the current picture is stored in the same frame buffer as the first field of
the pair and the following applies:


   - If the current decoded picture has OutputFlag equal to 1, it is marked as "needed for output".


   - Otherwise (the current decoded picture has OutputFlag equal to 0), it is marked as "not needed for output".

- Otherwise, if the current picture has OutputFlag equal to 0 and StoreInterViewOnlyRefFlag equal to 0, the DPB is
not modified and the current picture is not output.


- Otherwise, if the current picture has StoreInterViewOnlyRefFlag equal to 1, the following operations are performed:

1. When there is no empty frame buffer (i.e., DPB fullness is equal to DPB size), the "bumping" process specified

in clause C.4.5.3 is invoked repeatedly until there is an empty frame buffer in which to store the current decoded
picture.

2. The current decoded picture is stored in an empty frame buffer, the DPB fullness is incremented by one, and

the following applies:

      - If the current decoded picture has OutputFlag equal to 1, it is marked as "needed for output".

      - Otherwise (the current decoded picture has OutputFlag equal to 0), it is marked as "not needed for output".





- Otherwise, the following operations are performed repeatedly until the current decoded picture has been cropped and
output or has been stored in the DPB:


   - If there is no empty frame buffer (i.e., DPB fullness is equal to DPB size), the following applies:

      - If the current picture does not have a lower value of PicOrderCnt( ) than all pictures in the DPB that are
marked as "needed for output", the "bumping" process described in clause C.4.5.3 is performed.

      - Otherwise (the current picture has a lower value of PicOrderCnt( ) than all pictures in the DPB that are
marked as "needed for output"), the current picture is cropped, using the cropping rectangle specified in
the active sequence parameter set for the picture and the cropped picture is output.


   - Otherwise (there is an empty frame buffer, i.e., DPB fullness is less than DPB size), the current decoded picture
is stored in an empty frame buffer and is marked as "needed for output", and the DPB fullness is incremented
by one.


**C.4.5.3** **"Bumping" process**


When the decoding process specified in Annex H is applied, the process specified in this clause is invoked for a particular
view with view order index VOIdx, with "picture" being replaced by "view component", "frame" being replaced by "frame
view component", and "field" being replaced by "field view component". During the invocation of the process for a
particular view, only view components of the particular view are considered and frame buffers containing view components
of other views are not emptied.


When the decoding process specified in Annex I is applied, the process specified in this clause for Annex H is invoked for
particular texture view and depth view with view order index VOIdx, with each "view component" being replaced by
"texture view component" or "depth view component", "frame view component" being replaced by "texture frame view
component" or "depth frame view component", and "field view component" being replaced by "texture field view
component". During the invocation of the process for a particular texture view, only the texture view components of the
particular view are considered while respective depth view components may be cropped and output too During the
invocation of the process for a particular depth view, only the depth view components of the particular view are considered
and frame buffers containing view components of other views are not emptied. The DPB fullness represents the total
number of non-empty frame buffers, including frame buffers that contain view components of other views, for the texture
DPB or the depth DPB depending on whether the process is invoked for a texture view or a depth view, respectively.


The DPB fullness represents the total number of non-empty frame buffers. When the decoding process specified in
Annex H is applied, this includes frame buffers that contain view components of other views. When the decoding process
specified in Annex I is applied, this includes frame buffers that contain texture or depth view components of other views.


The "bumping" process is invoked in the following cases.

- There is no empty frame buffer (i.e., DPB fullness is equal to DPB size) and an empty frame buffer is needed for
storage of an inferred "non-existing" frame, as specified in clause C.4.2.

- The current picture is an IDR picture and no_output_of_prior_pics_flag is not equal to 1 and is not inferred to be
equal to 1, as specified in clause C.4.4.

- The current picture has memory_management_control_operation equal to 5, as specified in clause C.4.4.

- There is no empty frame buffer (i.e., DPB fullness is equal to DPB size) and an empty frame buffer is needed for
storage of a decoded (non-IDR) reference picture or a reference base picture, as specified in clause C.4.5.1.

- There is no empty frame buffer (i.e., DPB fullness is equal to DPB size) and the current picture is a non-reference
picture that is not the second field of a complementary non-reference field pair and the current picture has OutputFlag
equal to 1 and there are pictures in the DPB that are marked as "needed for output" that precede the current nonreference picture in output order, as specified in clause C.4.5.2, so an empty buffer is needed for storage of the current
picture.

- There is no empty frame buffer (i.e., DPB fullness is equal to DPB size) and the current picture is a non-reference
picture that is not the second field of a complementary non-reference field pair and the current picture has
StoreInterViewOnlyRefFlag equal to 1, as specified in clause C.4.5.2, so an empty buffer is needed for storage of the
current picture.


The "bumping" process consists of the following ordered steps:

1. The picture or complementary reference field pair that is considered first for output is selected as follows:

a. The frame buffer is selected that contains the picture having the smallest value of PicOrderCnt( ) of all
pictures in the DPB marked as "needed for output".

b. Depending on the frame buffer, the following applies:





        - If this frame buffer contains a complementary non-reference field pair with both fields marked as
"needed for output" and both fields have the same PicOrderCnt( ), the first of these two fields in
decoding order is considered first for output.

        - Otherwise, if this frame buffer contains a complementary reference field pair with both fields marked
as "needed for output" and both fields have the same PicOrderCnt( ), the entire complementary reference
field pair is considered first for output.

NOTE – When the two fields of a complementary reference field pair have the same value of PicOrderCnt( ), this
"bumping" process will output these pictures together, although the two fields have different output times from a
decoder that satisfies output timing conformance criteria (as specified in clause C.2.2).

        - Otherwise, the picture in this frame buffer that has the smallest value of PicOrderCnt( ) is considered
first for output.

2. Depending on whether a single picture or a complementary reference field pair is considered for output, the

following applies:

    - If a single picture is considered first for output, this picture is cropped, using the cropping rectangle specified
in the active sequence parameter set for the picture, the cropped picture is output, and the picture is marked as
"not needed for output".

    - Otherwise (a complementary reference field pair is considered first for output), the two fields of the
complementary reference field pair are both cropped, using the cropping rectangle specified in the active
sequence parameter set for the pictures, the two fields of the complementary reference field pair are output
together, and both fields of the complementary reference field pair are marked as "not needed for output".

3. When there is a single depth view component or a complementary depth view component pair having the same

values of view_id and PicOrderCnt( ) as the single picture or complementary reference field pair considered for
output, the single depth view component or complementary depth view component pair are output as in step 2.

4. The frame buffer that included the picture or complementary reference field pair that was cropped and output is

checked, and when any of the following conditions are true, the frame buffer is emptied and the DPB fullness is
decremented by 1:

    - The frame buffer contains a non-reference non-paired field.

    - The frame buffer contains a non-reference frame.

    - The frame buffer contains a complementary non-reference field pair with both fields marked as "not needed
for output".

    - The frame buffer contains a non-paired reference field marked as "unused for reference".

    - The frame buffer contains a reference frame with both fields marked as "unused for reference".

    - The frame buffer contains a complementary reference field pair with both fields marked as "unused for
reference" and "not needed for output".
