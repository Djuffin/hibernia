**C.2** **Operation of the decoded picture buffer (DPB)**


The decoded picture buffer contains frame buffers. When a coded video sequence conforming to one or more of the profiles
specified in Annex A is decoded by applying the decoding process specified in clauses 2 to 9, each of the frame buffers
may contain a decoded frame, a decoded complementary field pair or a single (non-paired) decoded field that is marked as
"used for reference" (reference pictures) or is held for future output (reordered or delayed pictures). When a coded video
sequence conforming to one or more of the profiles specified in Annex G is decoded by applying the decoding process
specified in Annex G, each frame buffer may contain a decoded frame, a decoded complementary field pair, a single (nonpaired) decoded field, a decoded reference base frame, a decoded reference base complementary field pair or a single (nonpaired) decoded reference base field that is marked as "used for reference" (reference pictures) or is held for future output
(reordered or delayed pictures). When a coded video sequence conforming to one or more of the profiles specified in
Annex H is decoded by applying the decoding process specified in Annex H, each of the frame buffers may contain a
decoded frame view component, a decoded complementary field view component pair, or a single (non-paired) decoded
field view component that is marked as "used for reference" (reference pictures) or is held for future output (reordered or
delayed pictures) or is held as reference for inter-view prediction (inter-view only reference components). When a coded
video sequence conforming to one or more of the profiles specified in Annex I is decoded by applying the decoding process
specified in Annex I, each of the frame buffers of the texture DPB may contain: a decoded depth frame view component,
a decoded complementary texture field view component pair, or a single (non-paired) decoded texture field view
component that is marked as "used for reference" (reference pictures) or is held for future output (reordered or delayed
pictures) or is held as reference for inter-view prediction (inter-view only reference components). When a coded video
sequence conforming to one or more of the profiles specified in Annex I is decoded by applying the decoding process
specified in Annex I, each of the frame buffers of the depth DPB may contain a decoded depth frame view component, a





decoded complementary depth field view component pair, or a single (non-paired) decoded depth field view component
that is marked as "used for reference" (reference pictures) or is held for future output (reordered or delayed pictures) or is
held as reference for inter-view prediction (inter-view only reference components).


Prior to initialization, the DPB is empty (the DPB fullness is set to zero). The following steps specified in this clause all
happen instantaneously at tr( n ) and in the order listed. When the decoding process specified in Annex H or Annex I is
applied, the view components of the current primary coded picture are processed by applying the ordered steps to each
view component in increasing order of the associated view order index VOIdx. During the invocation of the process for a
particular texture view, only the texture view components of the particular view are considered. During the invocation of
the process for a particular depth view, only the depth view components of the particular view are considered. For each
view component of the current primary coded picture, the corresponding depth view component with the same view order
index VOIdx, if present, is processed after the texture view component.


1. The process of decoding gaps in frame_num and storing "non-existing" frames as specified in clause C.2.1 is

invoked.


2. The picture decoding and output process as specified in clause C.2.2 is invoked.


3. The process of removing pictures from the DPB before possible insertion of the current picture as specified in

clause C.2.3 is invoked.


4. The process of marking and storing the current decoded picture as specified in clause C.2.4 is invoked.

NOTE – When the decoding process specified in Annex G is applied, the DPB is only operated for decoded pictures and
reference base pictures associated with decoded pictures. The DPB is not operated for layer pictures with dependency_id less
than DependencyIdMax (and associated reference base pictures). All decoded pictures and associated reference base pictures are
decoded pictures and associated reference base pictures for dependency_id equal to DependencyIdMax, which represent the
results of the decoding process specified in clause G.8.


**C.2.1** **Decoding of gaps in frame_num and storage of "non-existing" frames**


When the decoding process specified in Annex H is applied, the process specified in this clause is invoked for a particular
view with view order index VOIdx, with "picture" being replaced by "view component", "frame" being replaced by "frame
view component", and "field" being replaced by "field view component". During the invocation of the process for a
particular view, only view components of the particular view are considered and view components of other views are not
marked as "unused for reference" or removed from the DPB. When the decoding process specified in Annex I is applied,
the process specified in this clause for Annex H is invoked for particular texture view or depth view with view order index
VOIdx, with each "view component" being replaced by "texture view component" or "depth view component", "frame
view component" being replaced by "texture frame view component" or "depth frame view component", and "field view
component" being replaced by "texture field view component". During the invocation of the process for a particular texture
view, only the texture view components of the particular view are considered and during the invocation of the process for
a particular depth view, only the depth view components of the particular view are considered and view components of
other views are not marked as "unused for reference" or removed from the DPB.


The DPB fullness represents the total number of non-empty frame buffers. When the decoding process specified in
Annex H is applied; this includes frame buffers that contain view components of other views. When the decoding process
specified in Annex I is applied, this includes frame buffers that contain texture or depth view components of other views.


When applicable, gaps in frame_num are detected by the decoding process and the generated frames are marked and
inserted into the DPB as specified below.


Gaps in frame_num are detected by the decoding process and the generated frames are marked as specified in
clauses 8.2.5.2 and G.8.2.5.


After the marking of each generated frame, each picture m marked by the "sliding window" process as "unused for
reference" is removed from the DPB when it is also marked as "non-existing" or its DPB output time is less than or equal
to the CPB removal time of the current picture n; i.e., to,dpb( m ) <= tr( n ), or it has OutputFlag equal to 0. When a frame or
the last field in a frame buffer is removed from the DPB, the DPB fullness is decremented by one. The "non-existing"
generated frame is inserted into the DPB and the DPB fullness is incremented by one.


**C.2.2** **Picture decoding and output**


When the decoding process specified in Annex H is applied, the process specified in this clause is invoked for a particular
view with view order index VOIdx.


When the decoding process specified in Annex I is applied, the process specified in this clause is invoked for a particular
texture view or depth view with view order index VOIdx.


The decoding of the current picture or view component (when applying the decoding process specified in Annex H or
Annex I) and the derivation of the DPB output time (if applicable) is specified as follows:





- If the decoding process specified in clause 8 or Annex G is applied, the following applies:


   - The current primary coded picture n is decoded.


   - When picture n has OutputFlag equal to 1, its DPB output time to,dpb( n ) is derived by


to,dpb( n ) = tr( n ) + tc * dpb_output_delay( n ) (C-12)


where dpb_output_delay( n ) is the value of dpb_output_delay specified in the picture timing SEI message
associated with access unit n.


- Otherwise (the decoding process specified in Annex H or Annex I is applied), the following applies:


   - The view component with view order index VOIdx of the current primary coded picture n is decoded.


   - When VOIdx is equal to VOIdxMin and any of the view components of picture n has OutputFlag equal to 1, the
DPB output time to,dpb( n ) for picture n is derived by Equation C-12, where dpb_output_delay( n ) is the value
of dpb_output_delay specified in the picture timing SEI message associated with access unit n.


The output of the current picture or view component (when applying the decoding process specified in Annex H) is
specified as follows:


- If OutputFlag is equal to 1 and to,dpb( n ) = tr( n ), the current picture or view component is output.

NOTE 1 – When the current picture or view component has nal_ref_idc greater than 0 (when using the decoding process
specified in Annex G, nal_ref_idc is the syntax element of the target layer representation), it will be stored in the DPB.


- Otherwise, if OutputFlag is equal to 0, the current picture or view component is not output, but it may be stored in the
DPB as specified in clause C.2.4.


- Otherwise (OutputFlag is equal to 1 and to,dpb( n ) > tr( n ) ), the current picture or view component is output later and
will be stored in the DPB (as specified in clause C.2.4) and is output at time to,dpb( n ) unless indicated not to be output
by the decoding or inference of no_output_of_prior_pics_flag equal to 1 at a time that precedes to,dpb( n ).

NOTE 2 – When the coded video sequence conforms to a profile specified in Annex H and the decoding process specified in
Annex H is used, the view components of all the target output views of a picture are output at the same time instant and in
increasing order of the view order index VOIdx.
NOTE 3 – When the coded video sequence conforms to a profile specified in Annex I and the decoding process specified in
Annex I is used, the view components of all the target output views of a picture are output at the same time instant and in
increasing order of the view order index VOIdx. A depth view component, if present, follows the texture view component
within the same view component.


When output, the picture or view component shall be cropped, using the cropping rectangle specified in the active sequence
parameter set for the picture or view component.


When the decoding process specified in clause 8 or Annex G is applied, the current picture n is a picture that is output and
is not the last picture of the bitstream that is output, the value of  to,dpb( n ) is derived by


 to,dpb( n ) = to,dpb( nn ) − to,dpb( n ) (C-13)


where nn indicates the picture that follows after picture n in output order and has OutputFlag equal to 1.


When the decoding process specified in Annex H or Annex I is applied, the current picture n is a picture that contains at
least one view component that is output and the current picture is not the last picture of the bitstream that contains at least
one view component that is output and VOIdx is equal to VOIdxMin, the value of  to,dpb( n ) is derived by Equation C- **13**,
where nn indicates the picture that follows after picture n in output order and contains at least one any view component
with OutputFlag equal to 1.


The decoded picture or view component is temporarily stored (not in the DPB).


**C.2.3** **Removal of pictures from the DPB before possible insertion of the current picture**


When the decoding process specified in Annex H is applied, the process specified in this clause is invoked for a particular
view with view order index VOIdx, with "picture" being replaced by "view component", "frame" being replaced by "frame
view component", and "field" being replaced by "field view component".


When the decoding process specified in Annex I is applied, the process specified in this clause for Annex I is invoked for
particular texture view and depth view with view order index VOIdx, with each "view component" being replaced by
"texture view component" or "depth view component", "frame view component" being replaced by "texture frame view
component" or "depth frame view component", and "field view component" being replaced by "texture field view
component". During the invocation of the process for a particular texture view, only the texture view components of the





particular view are considered and during the invocation of the process for a particular depth view, only the depth view
components of the particular view are considered.


When the decoding process specified in Annex H or Annex I is applied, the following process is specified for removing
inter-view only reference components of the current access unit from the DPB. By this process, view components of the
current view with view order index VOIdx are not removed from the DPB, but inter-view only reference components of
other views may be removed. The removal of inter-view only reference components is specified as follows:


- If the view order index VOIdx of the current view is equal to VOIdxMax, all inter-view only reference components
m for which any of the following conditions are true are removed from the DPB:


   - OutputFlag is equal to 0,


   - The DPB output time to,dpb( m ) of the picture containing the view component m is less than or equal to the CPB
removal time tr( n ) of the current picture.


- Otherwise (the view order index VOIdx of the current view is less than VOIdxMax), all inter-view only reference
components m for which both of the following conditions are true are removed from the DPB:


   - OutputFlag is equal to 0 or the DPB output time to,dpb( m ) of the picture containing the view component m is
less than or equal to the CPB removal time tr( n ) of the current picture,


   - One of the following conditions is true:


      - The current view component is a view component of an anchor picture and the view_id of the inter-view
only reference component m is not equal to any value of anchor_ref_lX[ k ][ j ], with X being equal to 0
or 1, k being any integer value greater than the view order index VOIdx of the current view, and j being any
integer value in the range of 0 to Max( 0, num_anchor_refs_lX[ k ] − 1 ), inclusive,


      - The current view component is not a view component of an anchor picture and the view_id of the inter-view
only reference component m is not equal to any value of non_anchor_ref_lX[ k ][ j ], with X being equal
to 0 or 1, k being any integer value greater than the view order index VOIdx of the current view, and j being
any integer value in the range of 0 to Max( 0, num_non_anchor_refs_lX[ k ] − 1 ), inclusive.


When the decoding process specified in Annex H is applied, for the following processes specified in this clause, only view
components of the particular view for which this clause is invoked are considered, and view components of other views
are not marked as "unused for reference" or removed from the DPB. When the decoding process specified in Annex I is
applied, for the following processes specified for Annex I in this clause, during the invocation of the process for a particular
texture view, only texture view components of the particular texture view are considered and during the invocation of the
process for a particular depth view, only depth view components of the particular depth view are considered, and view
components of other views are not marked as "unused for reference" or removed from the DPB.


The DPB fullness represents the total number of non-empty frame buffers. When the decoding process specified in
Annex H is applied, this includes frame buffers that contain texture view components of other views. When the decoding
process specified in Annex I is applied, this includes frame buffers that contain texture or depth view components of other
views.


The removal of pictures from the DPB before possible insertion of the current picture proceeds as follows:


- If the decoded picture is an IDR picture the following applies:

1. All reference pictures in the DPB are marked as "unused for reference" as specified in clause 8.2.5.1 when a coded

video sequence conforming to one or more of the profiles specified in Annex A is decoded by applying the
decoding process specified in clauses 2 to 9, or as specified in clause G.8.2.4 when a coded video sequence
conforming to one or more of the profiles specified in Annex G is decoded by applying the decoding process
specified in Annex G, or as specified in clause H.8.3 when a coded video sequence conforming to one or more of
the profiles specified in Annex H is decoded by applying the decoding process specified in Annex H, or as
specified in clause I.8.3 when a coded video sequence conforming to one or more of the profiles specified in
Annex I is decoded by applying the decoding process specified in Annex I.

2. When the IDR picture is not the first IDR picture decoded and the value of PicWidthInMbs or FrameHeightInMbs

or max_dec_frame_buffering derived from the active sequence parameter set is different from the value of
PicWidthInMbs or FrameHeightInMbs or max_dec_frame_buffering derived from the sequence parameter set
that was active for the preceding picture, respectively, no_output_of_prior_pics_flag is inferred to be equal to 1
by the HRD, regardless of the actual value of no_output_of_prior_pics_flag.

NOTE – Decoder implementations should try to handle frame or DPB size changes more gracefully than the HRD in
regard to changes in PicWidthInMbs or FrameHeightInMbs.

3. When no_output_of_prior_pics_flag is equal to 1 or is inferred to be equal to 1, all frame buffers in the DPB are

emptied without output of the pictures they contain, and DPB fullness is set to 0.





- Otherwise (the decoded picture is not an IDR picture), the following applies:

   - If the slice header of the current picture includes memory_management_control_operation equal to 5, all
reference pictures in the DPB are marked as "unused for reference".

   - Otherwise (the slice header of the current picture does not include memory_management_control_operation
equal to 5), the decoded reference picture marking process specified in clause 8.2.5 is invoked when a coded
video sequence conforming to one or more of the profiles specified in Annex A is decoded by applying the
decoding process specified in clauses 2 to 9, or the decoded reference picture marking process specified in
clause G.8.2.4 is invoked when a coded video sequence conforming to one or more of the profiles specified in
Annex G is decoded by applying the decoding process specified in Annex G, or the decoded reference picture
marking process specified in clause H.8.3 is invoked when a coded video sequence conforming to one or more
of the profiles specified in Annex H is decoded by applying the decoding process specified in Annex H, or the
decoded reference picture marking process specified in clause I.8.3 is invoked when a coded video sequence
conforming to one or more of the profiles specified in Annex I is decoded by applying the decoding process
specified in Annex I.


All pictures m in the DPB, for which all of the following conditions are true, are removed from the DPB:


- picture m is marked as "unused for reference" or picture m is a non-reference picture. When a picture is a reference
frame, it is considered to be marked as "unused for reference" only when both of its fields have been marked as
"unused for reference",


- picture m is marked as "non-existing" or it has OutputFlag equal to 0 or its DPB output time to,dpb( m ) is less than or
equal to the CPB removal time tr( n ) of the current picture n.


When a frame or the last field in a frame buffer is removed from the DPB, the DPB fullness is decremented by one.


**C.2.4** **Current decoded picture marking and storage**


When the decoding process specified in Annex H is applied, the process specified in this clause is invoked for a particular
view with view order index VOIdx, with "picture" being replaced by "view component", "frame" being replaced by "frame
view component", and "field" being replaced by "field view component". When the decoding process specified in Annex I
is applied, the process specified in this clause for Annex I is invoked for particular texture view and depth view with view
order index VOIdx, with each "view component" being replaced by "texture view component" and "depth view
component", "frame view component" being replaced by "texture frame view component" and "depth frame view
component", and "field view component" being replaced by "texture field view component". In clause C.2.4.2, the DPB
output time to,dpb(n) and the CPB removal time tr(n) of a view component are the DPB output time and the CPB removal
time of the picture n containing the view component.


The marking and storage of the current decoded picture is specified as follows:


- If the current picture is a reference picture, the marking and storage process for reference pictures as specified in
clause C.2.4.1 is invoked.


- Otherwise (the current picture is a non-reference picture), the storage process for non-reference pictures as specified
in clause C.2.4.2 is invoked.


**C.2.4.1** **Marking and storage of a reference picture into the DPB**


The current picture is stored in the DPB as follows:


- If the current decoded picture is a second field (in decoding order) of a complementary reference field pair, and the
first field of the pair is still in the DPB, the current decoded picture is stored in the same frame buffer as the first field
of the pair.


- Otherwise, the current decoded picture is stored in an empty frame buffer, and the DPB fullness is incremented by
one.


When the coded video sequence conforms to one or more of the profiles specified in Annex G and the decoding process
specified in Annex G is applied and the current picture has store_ref_base_pic_flag equal to 1 (i.e., the current picture is
associated with a reference base picture), the associated reference base picture is stored in the DPB as follows:


- If the reference base picture is a second field (in decoding order) of a complementary reference base field pair, and
the first field of the pair is still in the DPB, the reference base picture is stored in the same frame buffer as the first
field of the pair.


- Otherwise, the reference base picture is stored in an empty frame buffer, and the DPB fullness is incremented by one.





**C.2.4.2** **Storage of a non-reference picture into the DPB**


The variable storePicFlag is derived as follows:


- If any of the following conditions are true, storePicFlag is set equal to 1:


   - the current picture n has OutputFlag equal to 1 and to,dpb(n) > tr(n),


   - the decoding process specified in Annex H or Annex I is used and the current view component has a view order
index VOIdx less than VOIdxMax and inter_view_flag equal to 1.


- Otherwise, storePicFlag is set equal to 0.


When storePicFlag is equal to 1, the current picture is stored in the DPB as follows:


- If the current decoded picture is a second field (in decoding order) of a complementary non-reference field pair, and
the first field of the pair is still in the DPB, the current decoded picture is stored in the same frame buffer as the first
field of the pair.


- Otherwise, the current decoded picture is stored in an empty frame buffer, and the DPB fullness is incremented by
one.
