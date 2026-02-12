**D.2** **SEI payload semantics**


**D.2.1** **General SEI payload semantics**


**bit_equal_to_one** shall be equal to 1.


**bit_equal_to_zero** shall be equal to 0.





The semantics and persistence scope for each SEI message are specified in the semantics specification for each particular
SEI message.


The values of some SEI message syntax elements, including pan_scan_rect_id, scene_id, second_scene_id, snapshot_id,
progressive_refinement_id, tone_map_id, frame_packing_arrangement_id, and colour_remap_id, are split into two sets of
value ranges, where the first set is specified as "may be used as determined by the application", and the second set is
specified as "reserved for future use by ITU-T | ISO/IEC". Applications should be cautious of potential “collisions” of the
interpretation for values of these syntax elements belonging to the first set of value ranges. Since different applications
might use these IDs having values in the first set of value ranges for different purposes, particular care should be exercised
in the design of encoders that generate SEI messages with these IDs having values in the first set of value ranges, and in
the design of decoders that interpret SEI messages with these IDs. This Specification does not define any management for
these values. These IDs having values in the first set of value ranges might only be suitable for use in contexts in which
"collisions" of usage (i.e., different definitions of the syntax and semantics of an SEI message with one of these IDs having
the same value in the first set of value ranges) are unimportant, or not possible, or are managed – e.g., defined or managed
in the controlling application or transport specification, or by controlling the environment in which bitstreams are
distributed.


**D.2.2** **Buffering period SEI message semantics**


The presence of the buffering period SEI message in the bitstream is specified as follows:

- If NalHrdBpPresentFlag is equal to 1 or VclHrdBpPresentFlag is equal to 1, one buffering period SEI message can
be present in any access unit of the bitstream, and one buffering period SEI message shall be present in every IDR
access unit and every access unit associated with a recovery point SEI message.

- Otherwise (NalHrdBpPresentFlag is equal to 0 and VclHrdBpPresentFlag is equal to 0), no buffering period SEI
messages shall be present in any access unit of the bitstream.
NOTE 1 – For some applications, the frequent presence of a buffering period SEI message may be desirable.


A buffering period is specified as the set of access units between two instances of the buffering period SEI message in
decoding order.


**seq_parameter_set_id** specifies the sequence parameter set for the current coded video sequence. The value of
seq_parameter_set_id shall be equal to the value of seq_parameter_set_id in the picture parameter set referenced by the
primary coded picture associated with the buffering period SEI message. The value of seq_parameter_set_id shall be in
the range of 0 to 31, inclusive.

NOTE 2 – When the sequence parameter set identified by seq_parameter_set_id is not already active, the buffering SEI message
will activate the identified sequence parameter set for the current coded video sequence as specified in clause 7.4.1.2.1.


**initial_cpb_removal_delay[** SchedSelIdx **]** specifies the delay for the SchedSelIdx-th CPB between the time of arrival in
the CPB of the first bit of the coded data associated with the access unit associated with the buffering period SEI message
and the time of removal from the CPB of the coded data associated with the same access unit, for the first buffering period
after HRD initialization. The syntax element has a length in bits given by initial_cpb_removal_delay_length_minus1 + 1.
It is in units of a 90 kHz clock. initial_cpb_removal_delay[ SchedSelIdx ] shall not be equal to 0 and shall not exceed
90000 * ( CpbSize[ SchedSelIdx ] ÷ BitRate[ SchedSelIdx ] ), the time-equivalent of the CPB size in 90 kHz clock units.


**initial_cpb_removal_delay_offset[** SchedSelIdx **]** is used for the SchedSelIdx-th CPB in combination with the
cpb_removal_delay to specify the initial delivery time of coded access units to the CPB.
initial_cpb_removal_delay_offset[ SchedSelIdx ] is in units of a 90 kHz clock. The
initial_cpb_removal_delay_offset[ SchedSelIdx ] syntax element is a fixed length code having a length in bits given by
initial_cpb_removal_delay_length_minus1 + 1. This syntax element is not used by decoders and is needed only for the
delivery scheduler (HSS) specified in Annex C.


Over the entire coded video sequence, the sum of initial_cpb_removal_delay[ SchedSelIdx ] and
initial_cpb_removal_delay_offset[ SchedSelIdx ] shall be constant for each value of SchedSelIdx.


**D.2.3** **Picture timing SEI message semantics**
NOTE 1 – The syntax of the picture timing SEI message is dependent on the content of the sequence parameter set that is active for
the primary coded picture associated with the picture timing SEI message. However, unless the picture timing SEI message of an
IDR access unit is preceded by a buffering period SEI message within the same access unit, the activation of the associated sequence
parameter set (and, for IDR pictures that are not the first picture in the bitstream, the determination that the primary coded picture is
an IDR picture) does not occur until the decoding of the first coded slice NAL unit of the primary coded picture. Since the coded
slice NAL unit of the primary coded picture follows the picture timing SEI message in NAL unit order, there may be cases in which
it is necessary for a decoder to store the RBSP containing the picture timing SEI message until determining the parameters of the
sequence parameter that will be active for the primary coded picture, and then perform the parsing of the picture timing SEI message.


The presence of the picture timing SEI message in the bitstream is specified as follows:





- If CpbDpbDelaysPresentFlag is equal to 1 or pic_struct_present_flag is equal to 1, one picture timing SEI message
shall be present in every access unit of the coded video sequence.

- Otherwise (CpbDpbDelaysPresentFlag is equal to 0 and pic_struct_present_flag is equal to 0), no picture timing SEI
messages shall be present in any access unit of the coded video sequence.


**cpb_removal_delay** specifies how many clock ticks (see clause E.2.1) to wait after removal from the CPB of the access
unit associated with the most recent buffering period SEI message in a preceding access unit before removing from the
buffer the access unit data associated with the picture timing SEI message. This value is also used to calculate an earliest
possible time of arrival of access unit data into the CPB for the HSS, as specified in Annex C. The syntax element is a
fixed length code having a length in bits given by cpb_removal_delay_length_minus1 + 1. The cpb_removal_delay is the
remainder of a modulo 2 [(cpb_removal_delay_length_minus1 + 1)] counter.

NOTE 2 – The value of cpb_removal_delay_length_minus1 that determines the length (in bits) of the syntax element
cpb_removal_delay is the value of cpb_removal_delay_length_minus1 coded in the sequence parameter set that is active for the
primary coded picture associated with the picture timing SEI message, although cpb_removal_delay specifies a number of clock
ticks relative to the removal time of the preceding access unit containing a buffering period SEI message, which may be an access
unit of a different coded video sequence.


**dpb_output_delay** is used to compute the DPB output time of the picture. It specifies how many clock ticks to wait after
removal of an access unit from the CPB before the decoded picture can be output from the DPB (see clause C.2).

NOTE 3 – A picture is not removed from the DPB at its output time when it is still marked as "used for short-term reference" or
"used for long-term reference".
NOTE 4 – Only one dpb_output_delay is specified for a decoded picture.


The length of the syntax element dpb_output_delay is given in bits by dpb_output_delay_length_minus1 + 1. When
max_dec_frame_buffering is equal to 0, dpb_output_delay shall be equal to 0.


The output time derived from the dpb_output_delay of any picture that is output from an output timing conforming decoder
as specified in clause C.2 shall precede the output time derived from the dpb_output_delay of all pictures in any subsequent
coded video sequence in decoding order.


The output time derived from the dpb_output_delay of the second field, in decoding order, of a complementary
non-reference field pair shall exceed the output time derived from the dpb_output_delay of the first field of the same
complementary non-reference field pair.


The picture output order established by the values of this syntax element shall be the same order as established by the
values of PicOrderCnt( ) as specified in clauses C.4.1 to C.4.5, except that when the two fields of a complementary
reference field pair have the same value of PicOrderCnt( ), the two fields have different output times.


For pictures that are not output by the "bumping" process of clause C.4.5 because they precede, in decoding order, an IDR
picture with no_output_of_prior_pics_flag equal to 1 or inferred to be equal to 1, the output times derived from
dpb_output_delay shall be increasing with increasing value of PicOrderCnt( ) relative to all pictures within the same coded
video sequence subsequent to any picture having a memory_management_control_operation equal to 5.


**pic_struct** indicates whether a picture should be displayed as a frame or one or more fields, according to Table D-1. Frame
doubling (pic_struct equal to 7) indicates that the frame should be displayed two times consecutively, and frame tripling
(pic_struct equal to 8) indicates that the frame should be displayed three times consecutively.

NOTE 5 – Frame doubling can facilitate the display, for example, of 25p video on a 50p display and 29.97p video on a 59.94p
display. Using frame doubling and frame tripling in combination on every other frame can facilitate the display of 23.98p video on
a 59.94p display.


When pic_struct is present (pic_struct_present_flag is equal to 1), the constraints specified in the third column of Table D-1
shall be obeyed.

NOTE 6 – When pic_struct_present_flag is equal to 0, then in many cases default values may be inferred. In the absence of other
indications of the intended display type of a picture, the decoder should infer the value of pic_struct as follows:

   - If field_pic_flag is equal to 1, pic_struct should be inferred to be equal to (1 + bottom_field_flag).

   - Otherwise, if TopFieldOrderCnt is equal to BottomFieldOrderCnt, pic_struct should be inferred to be equal to 0.

   - Otherwise, if TopFieldOrderCnt is less than BottomFieldOrderCnt, pic_struct should be inferred to be equal to 3.

   - Otherwise (field_pic_flag is equal to 0 and TopFieldOrderCnt is greater than BottomFieldOrderCnt), pic_struct should be

inferred to be equal to 4.
pic_struct is only a hint as to how the decoded video should be displayed on an assumed display type (e.g., interlaced or progressive)
at an assumed display rate. When another display type or display rate is used by the decoder, then pic_struct does not indicate the
display method, but may aid in processing the decoded video for the alternative display. When it is desired for pic_struct to have an
effective value in the range of 5 to 8, inclusive, pic_struct_present_flag should be equal to 1, as the above inference rule will not
produce these values.





**Table D-1 – Interpretation of pic_struct**












|Value|Indicated display of picture|Restrictions|NumClockTS|
|---|---|---|---|
|0|(progressive) frame|field_pic_flag shall be 0,<br>TopFieldOrderCnt shall be equal to<br>BottomFieldOrderCnt|1|
|1|top field|field_pic_flag shall be 1,<br>bottom_field_flag shall be 0|1|
|2|bottom field|field_pic_flag shall be 1,<br>bottom_field_flag shall be 1|1|
|3|top field, bottom field, in that order|field_pic_flag shall be 0,<br>TopFieldOrderCnt shall be less than or<br>equal to BottomFieldOrderCnt|2|
|4|bottom field, top field, in that order|field_pic_flag shall be 0,<br>BottomFieldOrderCnt shall be less than<br>or equal to TopFieldOrderCnt|2|
|5|top field, bottom field, top field repeated,<br>in that order|field_pic_flag shall be 0,<br>TopFieldOrderCnt shall be less than or<br>equal to BottomFieldOrderCnt|3|
|6|bottom field, top field, bottom field<br>repeated, in that order|field_pic_flag shall be 0,<br>BottomFieldOrderCnt shall be less than<br>or equal to TopFieldOrderCnt|3|
|7|frame doubling|field_pic_flag shall be 0,<br>fixed_frame_rate_flag shall be 1,<br>TopFieldOrderCnt shall be equal to<br>BottomFieldOrderCnt|2|
|8|frame tripling|field_pic_flag shall be 0,<br>fixed_frame_rate_flag shall be 1,<br>TopFieldOrderCnt shall be equal to<br>BottomFieldOrderCnt|3|
|9..15|reserved|||



When fixed_frame_rate_flag is equal to 1, it is a requirement of bitstream conformance that the constraints specified as
follows shall be obeyed throughout the operation of the following process, which is operated in output order.


1. Prior to output of the first picture of the bitstream (in output order) and prior to the output of the first picture (in

output order) of each subsequent coded video sequence for which the content of the active sequence parameter set
differs from that of the previously-active sequence parameter set, the variable lastFieldBottom is set equal to "not
determined".


2. After the output of each picture, the value of lastFieldBottom is checked and set as follows, using the values of

field_pic_flag, bottom_field_flag, pic_struct, TopFieldOrderCnt and BottomFieldOrderCnt (when applicable) for
the picture that was output.

    - If field_pic_flag is equal to 1, it is a requirement of bitstream conformance that the value of lastFieldBottom
shall not be equal to bottom_field_flag. The value of lastFieldBottom is then set equal to bottom_field_flag.

    - Otherwise (field_pic_flag is equal to 0), the following applies:

        - If pic_struct is present and is equal to 3 or 5, it is a requirement of bitstream conformance that the value
of lastFieldBottom shall not be equal to 0. The value of lastFieldBottom is then set equal to
1 − ( ( pic_struct − 1 ) >> 2 ).

        - Otherwise, if pic_struct is present and is equal to 4 or 6, it is a requirement of bitstream conformance
that the value of lastFieldBottom shall not be equal to 1. The value of lastFieldBottom is then set equal
to ( ( pic_struct − 1 ) >> 2 ).





        - Otherwise, if TopFieldOrderCnt is less than BottomFieldOrderCnt, it is a requirement of bitstream
conformance that the value of lastFieldBottom shall not be equal to 0. The value of lastFieldBottom is
then set equal to 1.

        - Otherwise, if TopFieldOrderCnt is greater than BottomFieldOrderCnt, it is a requirement of bitstream
conformance that the value of lastFieldBottom shall not be equal to 1. The value of lastFieldBottom is
then set equal to 0.

        - Otherwise (TopFieldOrderCnt is equal to BottomFieldOrderCnt and pic_struct is not present or is not
in the range of 3 to 6, inclusive), lastFieldBottom may have any value, and its value is not changed.


NumClockTS is determined by pic_struct as specified in Table D-1. There are up to NumClockTS sets of clock timestamp
information for a picture, as specified by clock_timestamp_flag[ i ] for each set. The sets of clock timestamp information
apply to the field(s) or the frame(s) associated with the picture by pic_struct.


The contents of the clock timestamp syntax elements indicate a time of origin, capture, or alternative ideal display. This
indicated time is computed as


clockTimestamp = ( ( hH * 60 + mM ) * 60 + sS ) * time_scale +
nFrames * ( num_units_in_tick * ( 1 + nuit_field_based_flag ) ) + tOffset, (D-1)


in units of clock ticks of a clock with clock frequency equal to time_scale Hz, relative to some unspecified point in time
for which clockTimestamp is equal to 0. Output order and DPB output timing are not affected by the value of
clockTimestamp. When two or more frames with pic_struct equal to 0 are consecutive in output order and have equal
values of clockTimestamp, the indication is that the frames represent the same content and that the last such frame in output
order is the preferred representation.

NOTE 7 – clockTimestamp time indications may aid display on devices with refresh rates other than those well-matched to DPB
output times.


**clock_timestamp_flag[** i **]** equal to 1 indicates that a number of clock timestamp syntax elements are present and follow
immediately. clock_timestamp_flag[ i ] equal to 0 indicates that the associated clock timestamp syntax elements are not
present. When NumClockTS is greater than 1 and clock_timestamp_flag[ i ] is equal to 1 for more than one value of i, the
value of clockTimestamp shall be non-decreasing with increasing value of i.


**ct_type** indicates the scan type (interlaced or progressive) of the source material as specified in Table D-2.


Two fields of a coded frame may have different values of ct_type.


When clockTimestamp is equal for two fields of opposite parity that are consecutive in output order, both with ct_type
equal to 0 (progressive) or ct_type equal to 2 (unknown), the two fields are indicated to have come from the same original
progressive frame. Two consecutive fields in output order shall have different values of clockTimestamp when the value
of ct_type for either field is 1 (interlaced).


**Table D-2 – Mapping of ct_type to source picture scan**

|Value|Original<br>picture scan|
|---|---|
|0|progressive|
|1|interlaced|
|2|unknown|
|3|reserved|



**nuit_field_based_flag** is used in calculating clockTimestamp, as specified in Equation D-1.


**counting_type** specifies the method of dropping values of the n_frames as specified in Table D-3.





**Table D-3 – Definition of counting_type values**

|Value|Interpretation|
|---|---|
|0|no dropping of n_frames count values and no use of<br>time_offset|
|1|no dropping of n_frames count values|
|2|dropping of individual zero values of n_frames count|
|3|dropping of individual MaxFPS − 1 values of n_frames<br>count|
|4|dropping of the two lowest (value 0 and 1) n_frames<br>counts<br>when<br>seconds_value<br>is<br>equal<br>to 0<br>and<br>minutes_value is not an integer multiple of 10|
|5|dropping of unspecified individual n_frames count values|
|6|dropping of unspecified numbers of unspecified n_frames<br>count values|
|7..31|reserved|



**full_timestamp_flag** equal to 1 specifies that the n_frames syntax element is followed by seconds_value, minutes_value,
and hours_value. full_timestamp_flag equal to 0 specifies that the n_frames syntax element is followed by seconds_flag.


**discontinuity_flag** equal to 0 indicates that the difference between the current value of clockTimestamp and the value of
clockTimestamp computed from the previous clock timestamp in output order can be interpreted as the time difference
between the times of origin or capture of the associated frames or fields. discontinuity_flag equal to 1 indicates that the
difference between the current value of clockTimestamp and the value of clockTimestamp computed from the previous
clock timestamp in output order should not be interpreted as the time difference between the times of origin or capture of
the associated frames or fields. When discontinuity_flag is equal to 0, the value of clockTimestamp shall be greater than
or equal to all values of clockTimestamp present for the preceding picture in DPB output order.


**cnt_dropped_flag** specifies the skipping of one or more values of n_frames using the counting method specified by
counting_type.


**n_frames** specifies the value of nFrames used to compute clockTimestamp. n_frames shall be less than


MaxFPS = Ceil( time_scale ÷ ( 2 * num_units_in_tick ) ) (D-2)


NOTE 8 – n_frames is a frame-based counter. For field-specific timing indications, time_offset should be used to indicate a distinct
clockTimestamp for each field.


When counting_type is equal to 2 and cnt_dropped_flag is equal to 1, n_frames shall be equal to 1 and the value of
n_frames for the previous picture in output order shall not be equal to 0 unless discontinuity_flag is equal to 1.

NOTE 9 – When counting_type is equal to 2, the need for increasingly large magnitudes of tOffset in Equation D-1 when using
fixed non-integer frame rates (e.g., 12.5 frames per second with time_scale equal to 50 and num_units_in_tick equal to 2 and
nuit_field_based_flag equal to 0) can be avoided by occasionally skipping over the value n_frames equal to 0 when counting (e.g.,
counting n_frames from 0 to 12, then incrementing seconds_value and counting n_frames from 1 to 12, then incrementing
seconds_value and counting n_frames from 0 to 12, etc.).


When counting_type is equal to 3 and cnt_dropped_flag is equal to 1, n_frames shall be equal to 0 and the value of
n_frames for the previous picture in output order shall not be equal to MaxFPS − 1 unless discontinuity_flag is equal to 1.

NOTE 10 – When counting_type is equal to 3, the need for increasingly large magnitudes of tOffset in Equation D-1 when using
fixed non-integer frame rates (e.g., 12.5 frames per second with time_scale equal to 50 and num_units_in_tick equal to 2 and
nuit_field_based_flag equal to 0) can be avoided by occasionally skipping over the value n_frames equal to MaxFPS − 1 when
counting (e.g., counting n_frames from 0 to 12, then incrementing seconds_value and counting n_frames from 0 to 11, then
incrementing seconds_value and counting n_frames from 0 to 12, etc.).


When counting_type is equal to 4 and cnt_dropped_flag is equal to 1, n_frames shall be equal to 2 and the specified value
of sS shall be zero and the specified value of mM shall not be an integer multiple of ten and n_frames for the previous
picture in output order shall not be equal to 0 or 1 unless discontinuity_flag is equal to 1.

NOTE 11 – When counting_type is equal to 4, the need for increasingly large magnitudes of tOffset in Equation D-1 when using
fixed non-integer frame rates (e.g., 30000÷1001 frames per second with time_scale equal to 60000 and num_units_in_tick equal
to 1 001 and nuit_field_based_flag equal to 1) can be reduced by occasionally skipping over the values of n_frames equal to 0 and  355


when counting (e.g., counting n_frames from 0 to 29, then incrementing seconds_value and counting n_frames from 0 to 29, etc.,
until the seconds_value is zero and minutes_value is not an integer multiple of ten, then counting n_frames from 2 to 29, then
incrementing seconds_value and counting n_frames from 0 to 29, etc.). This counting method is well known in industry and is often
referred to as "NTSC drop-frame" counting.


When counting_type is equal to 5 or 6 and cnt_dropped_flag is equal to 1, n_frames shall not be equal to 1 plus the value
of n_frames for the previous picture in output order modulo MaxFPS unless discontinuity_flag is equal to 1.

NOTE 12 – When counting_type is equal to 5 or 6, the need for increasingly large magnitudes of tOffset in Equation D-1 when
using fixed non-integer frame rates can be avoided by occasionally skipping over some values of n_frames when counting. The
specific values of n_frames that are skipped are not specified when counting_type is equal to 5 or 6.


**seconds_flag** equal to 1 specifies that seconds_value and minutes_flag are present when full_timestamp_flag is equal to 0.
seconds_flag equal to 0 specifies that seconds_value and minutes_flag are not present.


**seconds_value** specifies the value of sS used to compute clockTimestamp. The value of seconds_value shall be in the
range of 0 to 59, inclusive. When seconds_value is not present, the previous seconds_value in decoding order shall be used
as sS to compute clockTimestamp.


**minutes_flag** equal to 1 specifies that minutes_value and hours_flag are present when full_timestamp_flag is equal to 0
and seconds_flag is equal to 1. minutes_flag equal to 0 specifies that minutes_value and hours_flag are not present.


**minutes_value** specifies the value of mM used to compute clockTimestamp. The value of minutes_value shall be in the
range of 0 to 59, inclusive. When minutes_value is not present, the previous minutes_value in decoding order shall be used
as mM to compute clockTimestamp.


**hours_flag** equal to 1 specifies that hours_value is present when full_timestamp_flag is equal to 0 and seconds_flag is
equal to 1 and minutes_flag is equal to 1.


**hours_value** specifies the value of hH used to compute clockTimestamp. The value of hours_value shall be in the range
of 0 to 23, inclusive. When hours_value is not present, the previous hours_value in decoding order shall be used as hH to
compute clockTimestamp.


**time_offset** specifies the value of tOffset used to compute clockTimestamp. The number of bits used to represent
time_offset shall be equal to time_offset_length. When time_offset is not present, the value 0 shall be used as tOffset to
compute clockTimestamp.


**D.2.4** **Pan-scan rectangle SEI message semantics**


The pan-scan rectangle SEI message syntax elements specify the coordinates of a rectangle relative to the cropping
rectangle of the sequence parameter set. Each coordinate of this rectangle is specified in units of one-sixteenth sample
spacing relative to the luma sampling grid.


**pan_scan_rect_id** contains an identifying number that may be used to identify the purpose of the pan-scan rectangle (for
example, to identify the rectangle as the area to be shown on a particular display device or as the area that contains a
particular actor in the scene). The value of pan_scan_rect_id shall be in the range of 0 to 2 [32] - 2, inclusive.


Values of pan_scan_rect_id from 0 to 255, inclusive, and from 512 to 2 [31] - 1, inclusive, may be used as determined by the
application. Values of pan_scan_rect_id from 256 to 511, inclusive, and from 2 [31] to 2 [32] - 2, inclusive, are reserved for
future use by ITU-T | ISO/IEC. Decoders encountering a value of pan_scan_rect_id in the range of 256 to 511, inclusive,
or in the range of 2 [31] to 2 [32] - 2, inclusive, shall ignore it.


**pan_scan_rect_cancel_flag** equal to 1 indicates that the SEI message cancels the persistence of any previous pan-scan
rectangle SEI message in output order. pan_scan_rect_cancel_flag equal to 0 indicates that pan-scan rectangle information
follows.


**pan_scan_cnt_minus1** specifies the number of pan-scan rectangles that are present in the SEI message.
pan_scan_cnt_minus1 shall be in the range of 0 to 2, inclusive. pan_scan_cnt_minus1 equal to 0 indicates that a single
pan-scan rectangle is present that applies to all fields of the decoded picture. pan_scan_cnt_minus1 shall be equal to 0
when the current picture is a field. pan_scan_cnt_minus1 equal to 1 indicates that two pan-scan rectangles are present, the
first of which applies to the first field of the picture in output order and the second of which applies to the second field of
the picture in output order. pan_scan_cnt_minus1 equal to 2 indicates that three pan-scan rectangles are present, the first
of which applies to the first field of the picture in output order, the second of which applies to the second field of the picture
in output order, and the third of which applies to a repetition of the first field as a third field in output order.


**pan_scan_rect_left_offset[** i **]**, **pan_scan_rect_right_offset[** i **]**, **pan_scan_rect_top_offset[** i **]**, and
**pan_scan_rect_bottom_offset[** i **]**, specify, as signed integer quantities in units of one-sixteenth sample spacing relative
to the luma sampling grid, the location of the pan-scan rectangle. The values of each of these four syntax elements shall
be in the range of −2 [31] + 1 to 2 [31] - 1, inclusive.





The pan-scan rectangle is specified, in units of one-sixteenth sample spacing relative to a luma frame sampling grid, as the
region with frame horizontal coordinates from 16*CropUnitX * frame_crop_left_offset + pan_scan_rect_left_offset[ i ] to
16 * ( 16 * PicWidthInMbs − CropUnitX * frame_crop_right_offset ) + pan_scan_rect_right_offset[ i ] − 1 and with
vertical coordinates from 16 *CropUnitY * frame_crop_top_offset + pan_scan_rect_top_offset[ i ] to
16 * ( 16 * PicHeightInMbs − CropUnitY * frame_crop_bottom_offset ) + pan_scan_rect_bottom_offset[ i ] − 1,
inclusive. The value of 16 * CropUnitX * frame_crop_left_offset + pan_scan_rect_left_offset[ i ] shall be less than or
equal to 16 * ( 16 * PicWidthInMbs − CropUnitX * frame_crop_right_offset ) + pan_scan_rect_right_offset[ i ] − 1; and
the value of 16 * CropUnitY * frame_crop_top_offset + pan_scan_rect_top_offset[ i ] shall be less than or equal to
16 * ( 16 * PicHeightInMbs − CropUnitY * frame_crop_bottom_offset ) + pan_scan_rect_bottom_offset[ i ] − 1.


When the pan-scan rectangular area includes samples outside of the cropping rectangle, the region outside of the cropping
rectangle may be filled with synthesized content (such as black video content or neutral grey video content) for display.


**pan_scan_rect_repetition_period** specifies the persistence of the pan-scan rectangle SEI message and may specify a
picture order count interval within which another pan-scan rectangle SEI message with the same value of pan_scan_rect_id
or the end of the coded video sequence shall be present in the bitstream. The value of pan_scan_rect_repetition_period
shall be in the range of 0 to 16 384, inclusive. When pan_scan_cnt_minus1 is greater than 0,
pan_scan_rect_repetition_period shall not be greater than 1.


pan_scan_rect_repetition_period equal to 0 specifies that the pan-scan rectangle information applies to the current decoded
picture only.


pan_scan_rect_repetition_period equal to 1 specifies that the pan-scan rectangle information persists in output order until
any of the following conditions are true:

- A new coded video sequence begins.

- A picture in an access unit containing a pan-scan rectangle SEI message with the same value of pan_scan_rect_id is
output having PicOrderCnt( ) greater than PicOrderCnt( CurrPic ).


pan_scan_rect_repetition_period equal to 0 or equal to 1 indicates that another pan-scan rectangle SEI message with the
same value of pan_scan_rect_id may or may not be present.


pan_scan_rect_repetition_period greater than 1 specifies that the pan-scan rectangle information persists until any of the
following conditions are true:

- A new coded video sequence begins.

- A picture in an access unit containing a pan-scan rectangle SEI message with the same value of pan_scan_rect_id is
output having PicOrderCnt( ) greater than PicOrderCnt( CurrPic ) and less than or equal to PicOrderCnt( CurrPic ) +
pan_scan_rect_repetition_period.


pan_scan_rect_repetition_period greater than 1 indicates that another pan-scan rectangle SEI message with the same value
of pan_scan_rect_id shall be present for a picture in an access unit that is output having PicOrderCnt( ) greater than
PicOrderCnt( CurrPic ) and less than or equal to PicOrderCnt( CurrPic ) + pan_scan_rect_repetition_period; unless the
bitstream ends or a new coded video sequence begins without output of such a picture.


**D.2.5** **Filler payload SEI message semantics**


This message contains a series of payloadSize bytes of value 0xFF, which can be discarded.


**ff_byte** shall be a byte having the value 0xFF.


**D.2.6** **User data registered by Rec. ITU-T T.35 SEI message semantics**


This message contains user data registered as specified by Rec. ITU-T T.35, the contents of which are not specified by this
Recommendation | International Standard.


**itu_t_t35_country_code** shall be a byte having a value specified as a country code by Rec. ITU-T T.35 Annex A.


**itu_t_t35_country_code_extension_byte** shall be a byte having a value specified as a country code by Rec. ITU-T T.35
Annex B.


**itu_t_t35_payload_byte** shall be a byte containing data registered as specified by Rec. ITU-T T.35.


The ITU-T T.35 terminal provider code and terminal provider oriented code shall be contained in the first one or more
bytes of the itu_t_t35_payload_byte, in the format specified by the Administration that issued the terminal provider code.
Any remaining itu_t_t35_payload_byte data shall be data having syntax and semantics as specified by the entity identified
by the ITU-T T.35 country code and terminal provider code.





**D.2.7** **User data unregistered SEI message semantics**


This message contains unregistered user data identified by a UUID, the contents of which are not specified by this
Recommendation | International Standard.


**uuid_iso_iec_11578** shall have a value specified as a UUID according to the procedures of ISO/IEC 11578:1996 Annex A.


**user_data_payload_byte** shall be a byte containing data having syntax and semantics as specified by the UUID generator.


**D.2.8** **Recovery point SEI message semantics**


The recovery point SEI message assists a decoder in determining when the decoding process will produce acceptable
pictures for display after the decoder initiates random access or after the encoder indicates a broken link in the coded video
sequence. When the decoding process is started with the access unit in decoding order associated with the recovery point
SEI message, all decoded pictures at or subsequent to the recovery point in output order specified in this SEI message are
indicated to be correct or approximately correct in content. Decoded pictures produced by random access at or before the
picture associated with the recovery point SEI message need not be correct in content until the indicated recovery point,
and the operation of the decoding process starting at the picture associated with the recovery point SEI message may
contain references to pictures not available in the decoded picture buffer.


In addition, by use of the broken_link_flag, the recovery point SEI message can indicate to the decoder the location of
some pictures in the bitstream that can result in serious visual artefacts when displayed, even when the decoding process
was begun at the location of a previous IDR access unit in decoding order.

NOTE 1 – The broken_link_flag can be used by encoders to indicate the location of a point after which the decoding process for the
decoding of some pictures may cause references to pictures that, though available for use in the decoding process, are not the pictures
that were used for reference when the bitstream was originally encoded (e.g., due to a splicing operation performed during the
generation of the bitstream).


The recovery point is specified as a count in units of frame_num increments subsequent to the frame_num of the current
access unit at the position of the SEI message.

NOTE 2 – When HRD information is present in the bitstream, a buffering period SEI message should be associated with the access
unit associated with the recovery point SEI message in order to establish initialization of the HRD buffer model after a random
access.


Any picture parameter set RBSP that is referred to by a picture associated with a recovery point SEI message or by any
picture following such a picture in decoding order shall be available to the decoding process prior to its activation,
regardless of whether or not the decoding process is started at the beginning of the bitstream or with the access unit, in
decoding order, that is associated with the recovery point SEI message.


Any sequence parameter set RBSP that is referred to by a picture associated with a recovery point SEI message or by any
picture following such a picture in decoding order shall be available to the decoding process prior to its activation,
regardless of whether or not the decoding process is started at the beginning of the bitstream or with the access unit, in
decoding order, that is associated with the recovery point SEI message.


**recovery_frame_cnt** specifies the recovery point of output pictures in output order. All decoded pictures in output order
are indicated to be correct or approximately correct in content starting at the output order position of the reference picture
having the frame_num equal to the frame_num of the VCL NAL units for the current access unit incremented by
recovery_frame_cnt in modulo MaxFrameNum arithmetic. recovery_frame_cnt shall be in the range of 0 to
MaxFrameNum − 1, inclusive.


**exact_match_flag** indicates whether decoded pictures at and subsequent to the specified recovery point in output order
derived by starting the decoding process at the access unit associated with the recovery point SEI message shall be an exact
match to the pictures that would be produced by starting the decoding process at the location of a previous IDR access unit
in the NAL unit stream. The value 0 indicates that the match need not be exact and the value 1 indicates that the match
shall be exact.


When decoding starts from the location of the recovery point SEI message, all references to not available reference pictures
shall be inferred as references to pictures containing only macroblocks coded using Intra macroblock prediction modes
and having sample values given by Y samples equal to ( 1 << ( BitDepthY − 1 ) ), Cb samples equal to
( 1 << ( BitDepthC − 1 ) ), and Cr samples equal to ( 1 << ( BitDepthC − 1 ) ) (mid-level grey) for purposes of determining
the conformance of the value of exact_match_flag.

NOTE 3 – When performing random access, decoders should infer all references to not available reference pictures as references to
pictures containing only intra macroblocks and having sample values given by Y equal to ( 1 << ( BitDepthY − 1 ) ), Cb equal to
( 1 << ( BitDepthC − 1 ) ), and Cr equal to ( 1 << ( BitDepthC − 1 ) ) (mid-level grey), regardless of the value of exact_match_flag.


When exact_match_flag is equal to 0, the quality of the approximation at the recovery point is chosen by the encoding
process and is not specified by this Recommendation | International Standard.

NOTE 4 – Under some circumstances, the decoding process of pictures depends on the difference DiffPicOrderCnt( picA, picB )
between the PicOrderCnt( ) values for two pictures picA and picB. However, no particular values of TopFieldOrderCnt and





BottomFieldOrderCnt (as applicable) are specified to be assigned to the reference pictures that are not available due to the initiation
of random access at the location of a picture associated with a recovery point SEI message. Also, no particular value has been
specified for initialization (for random access purposes) of the related variables prevPicOrderCntMsb, prevPicOrderCntLsb,
prevFraneNumOffset, and prevFrameNum. Thus, any values for these variables may be assigned that could hypothetically have
resulted from operation of the decoding process starting with a hypothetical preceding IDR picture in decoding order, although such
values may not be the same as the values that would have been obtained if the decoding process had started with the actual preceding
IDR picture in the bitstream. When performing random access at a picture associated with a recovery point SEI message, it is
suggested that decoders should derive the picture order count variables TopFieldOrderCnt and BottomFieldOrderCnt according to
the following method:

  - A bit range greater than 32 bits should be allocated for the variables TopFieldOrderCnt and BottomFieldOrderCnt for each
current picture to be decoded, as well as for the intermediate variables used for deriving these variables as specified in
clause 8.2.1. (Due to the lack of assurance of correspondence of the values used for initialization of the related variables when
random access is performed to the values that would be obtained if the decoding process had begun with the preceding IDR
picture in decoding order, the calculations involving these variables in the decoding process of subsequent pictures may result
in violation of the 32 bit range.)

  - Any value within in the range of −2 [31] to 2 [31]   - 1, inclusive, may be assigned to the values of the variables TopFieldOrderCnt
and BottomFieldOrderCnt of the reference pictures that are not available due to the random access operation. For example, the
value 0 may be assigned to these variables.

  - For the derivation of the picture order count variables for the picture at which random access is performed,
prevPicOrderCntMsb may be set equal to any integer multiple of MaxPicOrderCntLsb in the range of −2 [31] to 2 [31]     - 1, inclusive,
prevPicOrderCntLsb may be set equal to any value in the range of 0 to MaxPicOrderCntLsb − 1, inclusive,
prevFrameNumOffset may be set equal to any integer multiple of MaxFrameNum in the range of 0 to 2 [31]     - 1, inclusive, and
prevFrameNum may be set equal to any value in the range of 0 to MaxFrameNum − 1, inclusive. For example, the value 0 may
be assigned to all of the variables prevPicOrderCntMsb, prevPicOrderCntLsb, prevFrameNumOffset, and prevFrameNum.
When exact_match_flag is equal to 1, it is a requirement of bitstream conformance that the values of the samples in the
decoded pictures at or subsequent to the recovery point in output order shall be independent of the values that a decoder
assigns to the variables prevPicOrderCntMsb, prevPicOrderCntLsb, prevFrameNumOffset, and prevFrameNum used in
clause 8.2.1 for deriving the picture order count variables for the initialization of the decoding process at the picture
associated with the recovery point SEI message, and of the values that are assigned to the TopFieldOrderCnt and
BottomFieldOrderCnt variables of the reference pictures that are not available due to the random access operation.


**broken_link_flag** indicates the presence or absence of a broken link in the NAL unit stream at the location of the recovery
point SEI message and is assigned further semantics as follows:

- If broken_link_flag is equal to 1, pictures produced by starting the decoding process at the location of a previous IDR
access unit may contain undesirable visual artefacts to the extent that decoded pictures at and subsequent to the access
unit associated with the recovery point SEI message in decoding order should not be displayed until the specified
recovery point in output order.

- Otherwise (broken_link_flag is equal to 0), no indication is given regarding any potential presence of visual artefacts.


Regardless of the value of the broken_link_flag, pictures subsequent to the specified recovery point in output order are
specified to be correct or approximately correct in content.

NOTE 5 – When a sub-sequence information SEI message is present in conjunction with a recovery point SEI message in which
broken_link_flag is equal to 1 and when sub_seq_layer_num is equal to 0, sub_seq_id should be different from the latest sub_seq_id
for sub_seq_layer_num equal to 0 that was decoded prior to the location of the recovery point SEI message. When broken_link_flag
is equal to 0, the sub_seq_id in sub-sequence layer 0 should remain unchanged.


**changing_slice_group_idc** equal to 0 indicates that decoded pictures are correct or approximately correct in content at
and subsequent to the recovery point in output order when all macroblocks of the primary coded pictures are decoded
within the changing slice group period, i.e., the period between the access unit associated with the recovery point SEI
message (inclusive) and the specified recovery point (inclusive) in decoding order. changing_slice_group_idc shall be
equal to 0 when num_slice_groups_minus1 is equal to 0 in any primary coded picture within the changing slice group
period.


When changing_slice_group_idc is equal to 1 or 2, num_slice_groups_minus1 shall be equal to 1 and the
macroblock-to-slice-group map type 3, 4, or 5 shall be applied in each primary coded picture in the changing slice group
period.


changing_slice_group_idc equal to 1 indicates that within the changing slice group period no sample values outside the
decoded macroblocks covered by slice group 0 are used for inter prediction of any macroblock within slice group 0. In
addition, changing_slice_group_idc equal to 1 indicates that when all macroblocks in slice group 0 within the changing
slice group period are decoded, decoded pictures will be correct or approximately correct in content at and subsequent to
the specified recovery point in output order regardless of whether any macroblock in slice group 1 within the changing
slice group period is decoded.


changing_slice_group_idc equal to 2 indicates that within the changing slice group period no sample values outside the
decoded macroblocks covered by slice group 1 are used for inter prediction of any macroblock within slice group 1. In
addition, changing_slice_group_idc equal to 2 indicates that when all macroblocks in slice group 1 within the changing





slice group period are decoded, decoded pictures will be correct or approximately correct in content at and subsequent to
the specified recovery point in output order regardless of whether any macroblock in slice group 0 within the changing
slice group period is decoded.


changing_slice_group_idc shall be in the range of 0 to 2, inclusive.


**D.2.9** **Decoded reference picture marking repetition SEI message semantics**


The decoded reference picture marking repetition SEI message is used to repeat the decoded reference picture marking
syntax structure that was located in the slice headers of an earlier picture in the same coded video sequence in decoding
order.


**original_idr_flag** shall be equal to 1 when the decoded reference picture marking syntax structure occurred originally in
an IDR picture. original_idr_flag shall be equal to 0 when the repeated decoded reference picture marking syntax structure
did not occur in an IDR picture originally.


**original_frame_num** shall be equal to the frame_num of the picture where the repeated decoded reference picture marking
syntax structure originally occurred. The picture indicated by original_frame_num is the previous coded picture having the
specified value of frame_num. The value of original_frame_num used to refer to a picture having a
memory_management_control_operation equal to 5 shall be 0.


**original_field_pic_flag** shall be equal to the field_pic_flag of the picture where the repeated decoded reference picture
marking syntax structure originally occurred.


**original_bottom_field_flag** shall be equal to the bottom_field_flag of the picture where the repeated decoded reference
picture marking syntax structure originally occurred.


dec_ref_pic_marking( ) shall contain a copy of the decoded reference picture marking syntax structure of the picture that
has a value of frame_num equal to original_frame_num. The IdrPicFlag used in the specification of the repeated
dec_ref_pic_marking( ) syntax structure shall be the IdrPicFlag of the slice header(s) of the picture that has a value of
frame_num equal to original_frame_num (i.e., IdrPicFlag as used in clause 7.3.3.3 shall be considered equal to
original_idr_flag).


**D.2.10** **Spare picture SEI message semantics**


This SEI message indicates that certain slice group map units, called spare slice group map units, in one or more decoded
reference pictures resemble the co-located slice group map units in a specified decoded picture called the target picture. A
spare slice group map unit may be used to replace a co-located, incorrectly decoded slice group map unit, in the target
picture. A decoded picture containing spare slice group map units is called a spare picture.


A spare picture SEI message shall not be present in an IDR access unit. The value of the PicSizeInMapUnits variable for
the target picture (as specified later in this clause) shall be equal to the value of the PicSizeInMapUnits variable for the
sequence parameter set that is active when processing the spare picture SEI message.


For all spare pictures identified in a spare picture SEI message, the value of frame_mbs_only_flag shall be equal to the
value of frame_mbs_only_flag of the target picture in the same SEI message. The spare pictures in the SEI message are
constrained as follows:

- If the target picture is a decoded field, all spare pictures identified in the same SEI message shall be decoded fields.

- Otherwise (the target picture is a decoded frame), all spare pictures identified in the same SEI message shall be
decoded frames.


For all spare pictures identified in a spare picture SEI message, the values of pic_width_in_mbs_minus1 and
pic_height_in_map_units_minus1 shall be equal to the values of pic_width_in_mbs_minus1 and
pic_height_in_map_units_minus1, respectively, of the target picture in the same SEI message. The picture associated (as
specified in clause 7.4.1.2.3) with this SEI message shall appear after the target picture, in decoding order.


**target_frame_num** indicates the frame_num of the target picture.


**spare_field_flag** equal to 0 indicates that the target picture and the spare pictures are decoded frames. spare_field_flag
equal to 1 indicates that the target picture and the spare pictures are decoded fields.


**target_bottom_field_flag** equal to 0 indicates that the target picture is a top field. target_bottom_field_flag equal to 1
indicates that the target picture is a bottom field.


A target picture is a decoded reference picture for which the corresponding primary coded picture precedes the current
picture, in decoding order, and in which the values of frame_num, field_pic_flag (when present) and bottom_field_flag
(when present) are equal to target_frame_num, spare_field_flag and target_bottom_field_flag, respectively.





**num_spare_pics_minus1** indicates the number of spare pictures for the specified target picture. The number of spare
pictures is equal to num_spare_pics_minus1 + 1. The value of num_spare_pics_minus1 shall be in the range of 0 to 15,
inclusive.


**delta_spare_frame_num[** i **]** is used to identify the spare picture that contains the i-th set of spare slice group map units,
hereafter called the i-th spare picture, as specified below. The value of delta_spare_frame_num[ i ] shall be in the range of
0 to MaxFrameNum − 2 + spare_field_flag, inclusive.


The frame_num of the i-th spare picture, spareFrameNum[ i ], is derived as follows for all values of i from 0 to
num_spare_pics_minus1, inclusive:


candidateSpareFrameNum = target_frame_num − 1 + spare_field_flag
for ( i = 0; i <= num_spare_pics_minus1; i++ ) {
if( candidateSpareFrameNum < 0 )
candidateSpareFrameNum = MaxFrameNum − 1
spareFrameNum[ i ] = candidateSpareFrameNum − delta_spare_frame_num[ i ] (D-3)
if( spareFrameNum[ i ] < 0 )
spareFrameNum[ i ] = MaxFrameNum + spareFrameNum[ i ]
candidateSpareFrameNum = spareFrameNum[ i ] − 1 + spare_field_flag
}


**spare_bottom_field_flag[** i **]** equal to 0 indicates that the i-th spare picture is a top field. spare_bottom_field_flag[ i ]
equal to 1 indicates that the i-th spare picture is a bottom field.


The 0-th spare picture is a decoded reference picture for which the corresponding primary coded picture precedes the target
picture, in decoding order, and in which the values of frame_num, field_pic_flag (when present) and bottom_field_flag
(when present) are equal to spareFrameNum[ 0 ], spare_field_flag and spare_bottom_field_flag[ 0 ], respectively. The i-th
spare picture is a decoded reference picture for which the corresponding primary coded picture precedes the ( i − 1 )-th
spare picture, in decoding order, and in which the values of frame_num, field_pic_flag (when present) and
bottom_field_flag (when present) are equal to spareFrameNum[ i ], spare_field_flag and spare_bottom_field_flag[ i ],
respectively.


**spare_area_idc[** i **]** indicates the method used to identify the spare slice group map units in the i-th spare picture.
spare_area_idc[ i ] shall be in the range of 0 to 2, inclusive. spare_area_idc[ i ] equal to 0 indicates that all slice group map
units in the i-th spare picture are spare units. spare_area_idc[ i ] equal to 1 indicates that the value of the syntax element
spare_unit_flag[ i ][ j ] is used to identify the spare slice group map units. spare_area_idc[ i ] equal to 2 indicates that the
zero_run_length[ i ][ j ] syntax element is used to derive the values of spareUnitFlagInBoxOutOrder[ i ][ j ], as described
below.


**spare_unit_flag[** i **][** j **]** equal to 0 indicates that the j-th slice group map unit in raster scan order in the i-th spare picture
is a spare unit. spare_unit_flag[ i ][ j ] equal to 1 indicates that the j-th slice group map unit in raster scan order in the i-th
spare picture is not a spare unit.


**zero_run_length[** i **][** j **]** is used to derive the values of spareUnitFlagInBoxOutOrder[ i ][ j ] when spare_area_idc[ i ] is
equal to 2. In this case, the spare slice group map units identified in spareUnitFlagInBoxOutOrder[ i ][ j ] appear in counterclockwise box-out order, as specified in clause 8.2.2.4, for each spare picture. spareUnitFlagInBoxOutOrder[ i ][ j ] equal
to 0 indicates that the j-th slice group map unit in counter-clockwise box-out order in the i-th spare picture is a spare unit.
spareUnitFlagInBoxOutOrder[ i ][ j ] equal to 1 indicates that the j-th slice group map unit in counter-clockwise box-out
order in the i-th spare picture is not a spare unit.


When spare_area_idc[ 0 ] is equal to 2, spareUnitFlagInBoxOutOrder[ 0 ][ j ] is derived as specified by the following
pseudo-code:


for( j = 0, loop = 0; j < PicSizeInMapUnits; loop++ ) {
for( k = 0; k < zero_run_length[ 0 ][ loop ]; k++ )
spareUnitFlagInBoxOutOrder[ 0 ][ j++ ] = 0 (D-4)
spareUnitFlagInBoxOutOrder[ 0 ][ j++ ] = 1
}





When spare_area_idc[ i ] is equal to 2 and the value of i is greater than 0, spareUnitFlagInBoxOutOrder[ i ][ j ] is derived
as specified by the following pseudo-code:


for( j = 0, loop = 0; j < PicSizeInMapUnits; loop++ ) {
for( k = 0; k < zero_run_length[ i ][ loop ]; k++ )
spareUnitFlagInBoxOutOrder[ i ][ j ] = spareUnitFlagInBoxOutOrder[ i − 1 ][ j++ ] (D-5)
spareUnitFlagInBoxOutOrder[ i ][ j ] = !spareUnitFlagInBoxOutOrder[ i − 1 ][ j++ ]
}


**D.2.11** **Scene information SEI message semantics**


A scene and a scene transition are herein defined as a set of consecutive pictures in output order.

NOTE 1 – Decoded pictures within one scene generally have similar content. The scene information SEI message is used to label
pictures with scene identifiers and to indicate scene changes. The message specifies how the source pictures for the labelled pictures
were created. The decoder may use the information to select an appropriate algorithm to conceal transmission errors. For example,
a specific algorithm may be used to conceal transmission errors that occurred in pictures belonging to a gradual scene transition.
Furthermore, the scene information SEI message may be used in a manner determined by the application, such as for indexing the
scenes of a coded sequence.


A scene information SEI message labels all pictures, in decoding order, from the primary coded picture to which the SEI
message is associated (inclusive), as specified in clause 7.4.1.2.3, to the primary coded picture to which the next scene
information SEI message (if present) in decoding order is associated (exclusive) or (otherwise) to the last access unit in
the bitstream (inclusive). These pictures are herein referred to as the target pictures.


**scene_info_present_flag** equal to 0 indicates that the scene or scene transition to which the target pictures belong is
unspecified. scene_info_present_flag equal to 1 indicates that the target pictures belong to the same scene or scene
transition.


**scene_id** identifies the scene to which the target pictures belong. When the value of scene_transition_type of the target
pictures is less than 4, and the previous picture in output order is marked with a value of scene_transition_type less than 4,
and the value of scene_id is the same as the value of scene_id of the previous picture in output order, this indicates that the
source scene for the target pictures and the source scene for the previous picture (in output order) are considered by the
encoder to have been the same scene. When the value of scene_transition_type of the target pictures is greater than 3, and
the previous picture in output order is marked with a value of scene_transition_type less than 4, and the value of scene_id
is the same as the value of scene_id of the previous picture in output order, this indicates that one of the source scenes for
the target pictures and the source scene for the previous picture (in output order) are considered by the encoder to have
been the same scene. When the value of scene_id is not equal to the value of scene_id of the previous picture in output
order, this indicates that the target pictures and the previous picture (in output order) are considered by the encoder to have
been from different source scenes.


The value of scene_id shall be in the range of 0 to 2 [32] - 2, inclusive.


Values of scene_id in the range of 0 to 255, inclusive, and in the range of 512 to 2 [31] - 1, inclusive, may be used as
determined by the application. Values of scene_id in the range of 256 to 511, inclusive, and in the range of 2 [31] to 2 [32] - 2,
inclusive, are reserved for future use by ITU-T | ISO/IEC. Decoders encountering a value of scene_id in the range of 256
to 511, inclusive, or in the range of 2 [31] to 2 [32] - 2, inclusive, shall ignore (remove from the bitstream and discard) it.


**scene_transition_type** specifies in which type of a scene transition (if any) the target pictures are involved. The valid
values of scene_transition_type are specified in Table D-4.


**Table D-4 – scene_transition_type values**

|Value|Description|
|---|---|
|0|No transition|
|1|Fade to black|
|2|Fade from black|
|3|Unspecified transition from or to constant colour|
|4|Dissolve|
|5|Wipe|
|6|Unspecified mixture of two scenes|



When scene_transition_type is greater than 3, the target pictures include contents both from the scene labelled by its
scene_id and the next scene, in output order, which is labelled by second_scene_id (see below). The term "the current





scene" is used to indicate the scene labelled by scene_id. The term "the next scene" is used to indicate the scene labelled
by second_scene_id. It is not required for any following picture, in output order, to be labelled with scene_id equal to
second_scene_id of the current SEI message.


Scene transition types are specified as follows.


"No transition" specifies that the target pictures are not involved in a gradual scene transition.

NOTE 2 – When two consecutive pictures in output order have scene_transition_type equal to 0 and different values of scene_id, a
scene cut occurred between the two pictures.


"Fade to black" indicates that the target pictures are part of a sequence of pictures, in output order, involved in a fade to
black scene transition, i.e., the luma samples of the scene gradually approach zero and the chroma samples of the scene
gradually approach 128.

NOTE 3 – When two pictures are labelled to belong to the same scene transition and their scene_transition_type is "Fade to black",
the later one, in output order, is darker than the previous one.


"Fade from black" indicates that the target pictures are part of a sequence of pictures, in output order, involved in a fade
from black scene transition, i.e., the luma samples of the scene gradually diverge from zero and the chroma samples of the
scene may gradually diverge from 128.

NOTE 4 – When two pictures are labelled to belong to the same scene transition and their scene_transition_type is "Fade from
black", the later one in output order is lighter than the previous one.


"Dissolve" indicates that the sample values of each target picture (before encoding) were generated by calculating a sum
of co-located weighted sample values of a picture from the current scene and a picture from the next scene. The weight of
the current scene gradually decreases from full level to zero level, whereas the weight of the next scene gradually increases
from zero level to full level. When two pictures are labelled to belong to the same scene transition and their
scene_transition_type is "Dissolve", the weight of the current scene for the later one, in output order, is less than the weight
of the current scene for the previous one, and the weight of the next scene for the later one, in output order, is greater than
the weight of the next scene for the previous one.


"Wipe" indicates that some of the sample values of each target picture (before encoding) were generated by copying
co-located sample values of a picture in the current scene and the remaining sample values of each target picture (before
encoding) were generated by copying co-located sample values of a picture in the next scene. When two pictures are
labelled to belong to the same scene transition and their scene_transition_type is "Wipe", the number of samples copied
from the next scene to the later picture in output order is greater than the number of samples copied from the next scene to
the previous picture.


**second_scene_id** identifies the next scene in the gradual scene transition in which the target pictures are involved. The
value of second_scene_id shall not be equal to the value of scene_id. The value of second_scene_id shall not be equal to
the value of scene_id in the previous picture in output order. When the next picture in output order is marked with a value
of scene_transition_type less than 4, and the value of second_scene_id is the same as the value of scene_id of the next
picture in output order, this indicates that the encoder considers one of the source scenes for the target pictures and the
source scene for the next picture (in output order) to have been the same scene. When the value of second_scene_id is not
equal to the value of scene_id or second_scene_id (if present) of the next picture in output order, this indicates that the
encoder considers the target pictures and the next picture (in output order) to have been from different source scenes.


When the value of scene_id of a picture is equal to the value of scene_id of the following picture in output order and the
value of scene_transition_type in both of these pictures is less than 4, this indicates that the encoder considers the two
pictures to have been from the same source scene. When the values of scene_id, scene_transition_type and
second_scene_id (if present) of a picture are equal to the values of scene_id, scene_transition_type and second_scene_id
(respectively) of the following picture in output order and the value of scene_transition_type is greater than 0, this indicates
that the encoder considers the two pictures to have been from the same source gradual scene transition.


The value of second_scene_id shall be in the range of 0 to 2 [32] - 2, inclusive.


Values of second_scene_id in the range of 0 to 255, inclusive, and in the range of 512 to 2 [31] - 1, inclusive, may be used as
determined by the application. Values of second_scene_id in the range of 256 to 511, inclusive, and in the range of 2 [31]
to 2 [32] - 2, inclusive, are reserved for future use by ITU-T | ISO/IEC. Decoders encountering a value of second_scene_id
in the range of 256 to 511, inclusive, or in the range of 2 [31] to 2 [32] - 2, inclusive, shall ignore (remove from the bitstream
and discard) it.


**D.2.12** **Sub-sequence information SEI message semantics**


The sub-sequence information SEI message is used to indicate the position of a picture in data dependency hierarchy that
consists of sub-sequence layers and sub-sequences.


A sub-sequence layer contains a subset of the coded pictures in a sequence. Sub-sequence layers are numbered with
non-negative integers. A layer having a larger layer number is a higher layer than a layer having a smaller layer number.





The layers are ordered hierarchically based on their dependency on each other so that any picture in a layer shall not be
predicted from any picture on any higher layer.

NOTE 1 – In other words, any picture in layer 0 must not be predicted from any picture in layer 1 or above, pictures in layer 1 may
be predicted from layer 0, pictures in layer 2 may be predicted from layers 0 and 1, etc.
NOTE 2 – The subjective quality is expected to increase along with the number of decoded layers.


A sub-sequence is a set of coded pictures within a sub-sequence layer. A picture shall reside in one sub-sequence layer and
in one sub-sequence only. Any picture in a sub-sequence shall not be predicted from any picture in another sub-sequence
in the same or in a higher sub-sequence layer. A sub-sequence in layer 0 can be decoded independently of any picture that
does not belong to the sub-sequence.


The sub-sequence information SEI message concerns the current access unit. The primary coded picture in the access unit
is herein referred to as the current picture.


The sub-sequence information SEI message shall not be present unless gaps_in_frame_num_value_allowed_flag in the
sequence parameter set referenced by the picture associated with the sub-sequence SEI message is equal to 1.


**sub_seq_layer_num** specifies the sub-sequence layer number of the current picture. When sub_seq_layer_num is greater
than 0, memory management control operations shall not be used in any slice header of the current picture. When the
current picture resides in a sub-sequence for which the first picture in decoding order is an IDR picture, the value of
sub_seq_layer_num shall be equal to 0. For a non-paired reference field, the value of sub_seq_layer_num shall be equal
to 0. sub_seq_layer_num shall be in the range of 0 to 255, inclusive.


**sub_seq_id** identifies the sub-sequence within a layer. When the current picture resides in a sub-sequence for which the
first picture in decoding order is an IDR picture, the value of sub_seq_id shall be the same as the value of idr_pic_id of the
IDR picture. sub_seq_id shall be in the range of 0 to 65535, inclusive.


**first_ref_pic_flag** equal to 1 specifies that the current picture is the first reference picture of the sub-sequence in decoding
order. When the current picture is not the first picture of the sub-sequence in decoding order, the first_ref_pic_flag shall
be equal to 0.


**leading_non_ref_pic_flag** equal to 1 specifies that the current picture is a non-reference picture preceding any reference
picture in decoding order within the sub-sequence or that the sub-sequence contains no reference pictures. When the current
picture is a reference picture or the current picture is a non-reference picture succeeding at least one reference picture in
decoding order within the sub-sequence, the leading_non_ref_pic_flag shall be equal to 0.


**last_pic_flag** equal to 1 indicates that the current picture is the last picture of the sub-sequence (in decoding order),
including all reference and non-reference pictures of the sub-sequence. When the current picture is not the last picture of
the sub-sequence (in decoding order), last_pic_flag shall be equal to 0.


The current picture is assigned to a sub-sequence as follows:

- If one or more of the following conditions are true, the current picture is the first picture of a sub-sequence in decoding
order:


   - no earlier picture in decoding order is labelled with the same values of sub_seq_id and sub_seq_layer_num as
the current picture,


   - the value of leading_non_ref_pic_flag is equal to 1 and the value of leading_non_ref_pic_flag is equal to 0 in
the previous picture in decoding order having the same values of sub_seq_id and sub_seq_layer_num as the
current picture,


   - the value of first_ref_pic_flag is equal to 1 and the value of leading_non_ref_pic_flag is equal to 0 in the previous
picture in decoding order having the same values of sub_seq_id and sub_seq_layer_num as the current picture,


   - the value of last_pic_flag is equal to 1 in the previous picture in decoding order having the same values of
sub_seq_id and sub_seq_layer_num as the current picture.

- Otherwise, the current picture belongs to the same sub-sequence as the previous picture in decoding order having the
same values of sub_seq_id and sub_seq_layer_num as the current picture.


**sub_seq_frame_num_flag** equal to 0 specifies that sub_seq_frame_num is not present. sub_seq_frame_num_flag equal
to 1 specifies that sub_seq_frame_num is present.


**sub_seq_frame_num** shall be equal to 0 for the first reference picture of the sub-sequence and for any non-reference
picture preceding the first reference picture of the sub-sequence in decoding order. sub_seq_frame_num is further
constrained as follows:

- If the current picture is not the second field of a complementary field pair, sub_seq_frame_num shall be incremented
by 1, in modulo MaxFrameNum operation, relative to the previous reference picture, in decoding order, that belongs
to the sub-sequence.





- Otherwise (the current picture is the second field of a complementary field pair), the value of sub_seq_frame_num
shall be the same as the value of sub_seq_frame_num for the first field of the complementary field pair.


sub_seq_frame_num shall be in the range of 0 to MaxFrameNum − 1, inclusive.


When the current picture is an IDR picture, it shall start a new sub-sequence in sub-sequence layer 0. Thus, the
sub_seq_layer_num shall be 0, the sub_seq_id shall be different from the previous sub-sequence in sub-sequence layer 0,
first_ref_pic_flag shall be 1, and leading_non_ref_pic_flag shall be equal to 0.


When the sub-sequence information SEI message is present for both coded fields of a complementary field pair, the values
of sub_seq_layer_num, sub_seq_id, leading_non_ref_pic_flag and sub_seq_frame_num, when present, shall be the same
for both of these pictures.


When the sub-sequence information SEI message is present only for one coded field of a complementary field pair, the
values of sub_seq_layer_num, sub_seq_id, leading_non_ref_pic_flag and sub_seq_frame_num, when present, are also
applicable to the other coded field of the complementary field pair.


**D.2.13** **Sub-sequence layer characteristics SEI message semantics**


The sub-sequence layer characteristics SEI message specifies the characteristics of sub-sequence layers.


**num_sub_seq_layers_minus1** plus 1 specifies the number of sub-sequence layers in the sequence.
num_sub_seq_layers_minus1 shall be in the range of 0 to 255, inclusive.


A pair of average_bit_rate and average_frame_rate characterizes each sub-sequence layer. The first pair of
average_bit_rate and average_frame_rate specifies the characteristics of sub-sequence layer 0. When present, the second
pair specifies the characteristics of sub-sequence layers 0 and 1 jointly. Each pair in decoding order specifies the
characteristics for a range of sub-sequence layers from layer number 0 to the layer number specified by the layer loop
counter. The values are in effect from the point they are decoded until an update of the values is decoded.


**accurate_statistics_flag** equal to 1 indicates that the values of average_bit_rate and average_frame_rate are rounded from
statistically correct values. accurate_statistics_flag equal to 0 indicates that the average_bit_rate and the
average_frame_rate are estimates and may deviate somewhat from the correct values.


When accurate_statistics_flag is equal to 0, the quality of the approximation used in the computation of the values of
average_bit_rate and the average_frame_rate is chosen by the encoding process and is not specified by this
Recommendation | International Standard.


**average_bit_rate** indicates the average bit rate in units of 1000 bits per second. All NAL units in the range of sub-sequence
layers specified above are taken into account in the calculation. The average bit rate is derived according to the access unit
removal time specified in Annex C of the Recommendation | International Standard. In the following, bTotal is the number
of bits in all NAL units succeeding a sub-sequence layer characteristics SEI message (including the bits of the NAL units
of the current access unit) and preceding the next access unit (in decoding order) including a sub-sequence layer
characteristics SEI message (if present) or the end of the stream (otherwise). t1 is the removal time (in seconds) of the
current access unit, and t2 is the removal time (in seconds) of the latest access unit (in decoding order) before the next subsequence layer characteristics SEI message (if present) or the end of the stream (otherwise).


When accurate_statistics_flag is equal to 1, the following conditions shall be fulfilled as follows:


- If t1 is not equal to t2, the following condition shall be true:


average_bit_rate = = Round( bTotal ÷ ( ( t2 − t1 ) * 1000 ) ) ) (D-6)


- Otherwise (t1 is equal to t2), the following condition shall be true:


average_bit_rate = = 0 (D-7)


**average_frame_rate** indicates the average frame rate in units of frames/(256 seconds). All NAL units in the range of subsequence layers specified above are taken into account in the calculation. In the following, fTotal is the number of frames,
complementary field pairs and non-paired fields between the current picture (inclusive) and the next sub-sequence layer
characteristics SEI message (if present) or the end of the stream (otherwise). t1 is the removal time (in seconds) of the
current access unit, and t2 is the removal time (in seconds) of the latest access unit (in decoding order) before the next subsequence layer characteristics SEI message (if present) or the end of the stream (otherwise).


When accurate_statistics_flag is equal to 1, the following conditions shall be fulfilled as follows:


- If t1 is not equal to t2, the following condition shall be true:


average_frame_rate = = Round( fTotal * 256 ÷ ( t2 − t1 ) ) (D-8)





- Otherwise (t1 is equal to t2), the following condition shall be true:


average_frame_rate = = 0 (D-9)


**D.2.14** **Sub-sequence characteristics SEI message semantics**


The sub-sequence characteristics SEI message indicates the characteristics of a sub-sequence. It also indicates inter
prediction dependencies between sub-sequences. This message shall be contained in the first access unit in decoding order
of the sub-sequence to which the sub-sequence characteristics SEI message applies. This sub-sequence is herein called the
target sub-sequence.


**sub_seq_layer_num** identifies the sub-sequence layer number of the target sub-sequence. sub_seq_layer_num shall be in
the range of 0 to 255, inclusive.


**sub_seq_id** identifies the target sub-sequence. sub_seq_id shall be in the range of 0 to 65535, inclusive.


**duration_flag** equal to 0 indicates that the duration of the target sub-sequence is not specified.


**sub_seq_duration** specifies the duration of the target sub-sequence in clock ticks of a 90-kHz clock.


**average_rate_flag** equal to 0 indicates that the average bit rate and the average frame rate of the target sub-sequence are
unspecified.


**accurate_statistics_flag** indicates how reliable the values of average_bit_rate and average_frame_rate are.
accurate_statistics_flag equal to 1, indicates that the average_bit_rate and the average_frame_rate are rounded from
statistically correct values. accurate_statistics_flag equal to 0 indicates that the average_bit_rate and the
average_frame_rate are estimates and may deviate from the statistically correct values.


**average_bit_rate** indicates the average bit rate in (1000 bits)/second of the target sub-sequence. All NAL units of the
target sub-sequence are taken into account in the calculation. The average bit rate is derived according to the access unit
removal time specified in clause C.1.2. In the following, nB is the number of bits in all NAL units in the sub-sequence. t1
is the removal time (in seconds) of the first access unit of the sub-sequence (in decoding order), and t2 is the removal time
(in seconds) of the last access unit of the sub-sequence (in decoding order).


When accurate_statistics_flag is equal to 1, the following conditions shall be fulfilled as follows:


- If t1 is not equal to t2, the following condition shall be true:


average_bit_rate = = Round( nB ÷ ( ( t2 − t1 ) * 1000 ) ) (D-10)


- Otherwise (t1 is equal to t2), the following condition shall be true:


average_bit_rate = = 0 (D-11)


**average_frame_rate** indicates the average frame rate in units of frames/(256 seconds) of the target sub-sequence. All
NAL units of the target sub-sequence are taken into account in the calculation. The average frame rate is derived according
to the access unit removal time specified in clause C.1.2. In the following, fC is the number of frames, complementary
field pairs and non-paired fields in the sub-sequence. t1 is the removal time (in seconds) of the first access unit of the subsequence (in decoding order), and t2 is the removal time (in seconds) of the last access unit of the sub-sequence (in decoding
order).


When accurate_statistics_flag is equal to 1, the following conditions shall be fulfilled as follows:


- If t1 is not equal to t2, the following condition shall be true:


average_frame_rate = = Round( fC * 256 ÷ ( t2 − t1 ) ) (D-12)


- Otherwise (t1 is equal to t2), the following condition shall be true:


average_frame_rate = = 0 (D-13)


**num_referenced_subseqs** specifies the number of sub-sequences that contain pictures that are used as reference pictures
for inter prediction in the pictures of the target sub-sequence. num_referenced_subseqs shall be in the range of 0 to 255,
inclusive.


**ref_sub_seq_layer_num,** **ref_sub_seq_id**, and **ref_sub_seq_direction** identify the sub-sequence that contains pictures
that are used as reference pictures for inter prediction in the pictures of the target sub-sequence. Depending on
ref_sub_seq_direction, the following applies:





- If ref_sub_seq_direction is equal to 0, a set of candidate sub-sequences consists of the sub-sequences which have a
value of sub_seq_id equal to ref_sub_seq_id, which reside in the sub-sequence layer having sub_seq_layer_num equal
to ref_sub_seq_layer_num, and for which the first picture in decoding order precedes the first picture of the target
sub-sequence in decoding order.


- Otherwise (ref_sub_seq_direction is equal to 1), a set of candidate sub-sequences consists of the sub-sequences which
have a value of sub_seq_id equal to ref_sub_seq_id, which reside in the sub-sequence layer having
sub_seq_layer_num equal to ref_sub_seq_layer_num, and for which the first picture in decoding order succeeds the
first picture of the target sub-sequence in decoding order.


The sub-sequence used as a reference for the target sub-sequence is the sub-sequence among the set of candidate
sub-sequences for which the first picture is the closest to the first picture of the target sub-sequence in decoding order.


**D.2.15** **Full-frame freeze SEI message semantics**


The full-frame freeze SEI message indicates that the current picture and any subsequent pictures in output order that meet
specified conditions should not affect the content of the display. No more than one full-frame freeze SEI message shall be
present in any access unit.


**full_frame_freeze_repetition_period** specifies the persistence of the full-frame freeze SEI message and may specify a
picture order count interval within which another full-frame freeze SEI message or a full-frame freeze release SEI message
or the end of the coded video sequence shall be present in the bitstream. The value of full_frame_freeze_repetition_period
shall be in the range of 0 to 16 384, inclusive.


full_frame_freeze_repetition_period equal to 0 specifies that the full-frame freeze SEI message applies to the current
decoded picture only.


full_frame_freeze_repetition_period equal to 1 specifies that the full-frame freeze SEI message persists in output order
until any of the following conditions are true:

- A new coded video sequence begins.

- A picture in an access unit containing a full-frame freeze SEI message or a full-frame freeze release SEI message is
output having PicOrderCnt( ) greater than PicOrderCnt( CurrPic ).


full_frame_freeze_repetition_period greater than 1 specifies that the full-frame freeze SEI message persists until any of
the following conditions are true:.

- A new coded video sequence begins.

- A picture in an access unit containing a full-frame freeze SEI message or a full-frame freeze release SEI message is
output having PicOrderCnt( ) greater than PicOrderCnt( CurrPic ) and less than or equal to PicOrderCnt( CurrPic ) +
full_frame_freeze_repetition_period.


full_frame_freeze_repetition_period greater than 1 indicates that another full-frame freeze SEI message or a full-frame
freeze release SEI message shall be present for a picture in an access unit that is output having PicOrderCnt( ) greater than
PicOrderCnt( CurrPic ) and less than or equal to PicOrderCnt( CurrPic ) + full_frame_freeze_repetition_period; unless the
bitstream ends or a new coded video sequence begins without output of such a picture.


**D.2.16** **Full-frame freeze release SEI message semantics**


The full-frame freeze release SEI message cancels the effect of any full-frame freeze SEI message sent with pictures that
precede the current picture in output order. The full-frame freeze release SEI message indicates that the current picture and
subsequent pictures in output order should affect the contents of the display.


No more than one full-frame freeze release SEI message shall be present in any access unit. A full-frame freeze release
SEI message shall not be present in an access unit containing a full-frame freeze SEI message. When a full-frame freeze
SEI message is present in an access unit containing a field of a complementary field pair in which the values of
PicOrderCnt( CurrPic ) for the two fields of the complementary field pair are equal to each other, a full-frame freeze release
SEI message shall not be present in either of the two access units.


**D.2.17** **Full-frame snapshot SEI message semantics**


The full-frame snapshot SEI message indicates that the current frame is labelled for use as determined by the application
as a still-image snapshot of the video content.


**snapshot_id** specifies a snapshot identification number. snapshot_id shall be in the range of 0 to 2 [32] - 2, inclusive.


Values of snapshot_id in the range of 0 to 255, inclusive, and in the range of 512 to 2 [31] - 1, inclusive, may be used as
determined by the application. Values of snapshot_id in the range of 256 to 511, inclusive, and in the range of 2 [31] to 2 [32] - 2,
inclusive, are reserved for future use by ITU-T | ISO/IEC. Decoders encountering a value of snapshot_id in the range of
256 to 511, inclusive, or in the range of 2 [31] to 2 [32] - 2, inclusive, shall ignore (remove from the bitstream and discard) it.





**D.2.18** **Progressive refinement segment start SEI message semantics**


The progressive refinement segment start SEI message specifies the beginning of a set of consecutive coded pictures that
is labelled as the current picture followed by a sequence of one or more pictures of refinement of the quality of the current
picture, rather than as a representation of a continually moving scene.


The tagged set of consecutive coded pictures shall continue until one of the following conditions is true. When a condition
below becomes true, the next slice to be decoded does not belong to the tagged set of consecutive coded pictures:

- The next slice to be decoded belongs to an IDR picture.

- num_refinement_steps_minus1 is greater than 0 and the frame_num of the next slice to be decoded is (currFrameNum
+ num_refinement_steps_minus1 + 1) % MaxFrameNum, where currFrameNum is the value of frame_num of the
picture in the access unit containing the SEI message.

- num_refinement_steps_minus1 is 0 and a progressive refinement segment end SEI message with the same
progressive_refinement_id as the one in this SEI message is decoded.


The decoding order of pictures within the tagged set of consecutive pictures should be the same as their output order.


**progressive_refinement_id** specifies an identification number for the progressive refinement operation.
progressive_refinement_id shall be in the range of 0 to 2 [32] - 2, inclusive.


Values of progressive_refinement_id in the range of 0 to 255, inclusive, and in the range of 512 to 2 [31] - 1, inclusive, may
be used as determined by the application. Values of progressive_refinement_id in the range of 256 to 511, inclusive, and
in the range of 2 [31] to 2 [32] - 2, inclusive, are reserved for future use by ITU-T | ISO/IEC. Decoders encountering a value of
progressive_refinement_id in the range of 256 to 511, inclusive, or in the range of 2 [31] to 2 [32] - 2, inclusive, shall ignore
(remove from the bitstream and discard) it.


**num_refinement_steps_minus1** specifies the number of reference frames in the tagged set of consecutive coded pictures
as follows:

- If num_refinement_steps_minus1 is equal to 0, the number of reference frames in the tagged set of consecutive coded
pictures is unknown.

- Otherwise, the number of reference frames in the tagged set of consecutive coded pictures is equal to
num_refinement_steps_minus1 + 1.


num_refinement_steps_minus1 shall be in the range of 0 to MaxFrameNum − 1, inclusive.


**D.2.19** **Progressive refinement segment end SEI message semantics**


The progressive refinement segment end SEI message specifies the end of a set of consecutive coded pictures that has been
labelled by use of a progressive refinement segment start SEI message as an initial picture followed by a sequence of one
or more pictures of the refinement of the quality of the initial picture, and ending with the current picture.


**progressive_refinement_id** specifies an identification number for the progressive refinement operation.
progressive_refinement_id shall be in the range of 0 to 2 [32] - 2, inclusive.


The progressive refinement segment end SEI message specifies the end of any progressive refinement segment previously
started using a progressive refinement segment start SEI message with the same value of progressive_refinement_id.


Values of progressive_refinement_id in the range of 0 to 255, inclusive, and in the range of 512 to 2 [31] - 1, inclusive, may
be used as determined by the application. Values of progressive_refinement_id in the range of 256 to 511, inclusive, and
in the range of 2 [31] to 2 [32] - 2, inclusive, are reserved for future use by ITU-T | ISO/IEC. Decoders encountering a value of
progressive_refinement_id in the range of 256 to 511, inclusive, or in the range of 2 [31] to 2 [32] - 2, inclusive, shall ignore
(remove from the bitstream and discard) it.


**D.2.20** **Motion-constrained slice group set SEI message semantics**

NOTE 1 – The syntax of the motion-constrained slice group set SEI message is dependent on the content of the picture parameter
set that is active for the primary coded picture associated with the motion-constrained slice group set SEI message. However, the
activation of the associated picture parameter set does not occur until the decoding of the first coded slice NAL unit of the primary
coded picture. Since the coded slice NAL units of the primary coded picture follow the motion-constrained slice group set SEI
message in NAL unit order, it may be necessary for a decoder to store the RBSP containing the motion-constrained slice group set
SEI message until determining the parameters of the picture parameter set that will be active for the primary coded picture, and then
perform the parsing of the motion-constrained slice group set SEI message.


This SEI message indicates that inter prediction over slice group boundaries is constrained as specified below. When
present, the message shall only appear where it is associated, as specified in clause 7.4.1.2.3, with an IDR access unit.


The target picture set for this SEI message contains all consecutive primary coded pictures in decoding order starting with
the associated primary coded IDR picture (inclusive) and ending with the following primary coded IDR picture (exclusive)
or with the very last primary coded picture in the bitstream (inclusive) in decoding order when there is no following primary





coded IDR picture. The slice group set is a collection of one or more slice groups, identified by the slice_group_id[ i ]
syntax element. When separate_colour_plane_flag is equal to 1, the term "primary coded pictures" represents the parts of
the corresponding primary coded pictures that correspond to the NAL units having the same colour_plane_id.


This SEI message indicates that, for each picture in the target picture set, the inter prediction process is constrained as
follows: No sample value outside the slice group set, and no sample value at a fractional sample position that is derived
using one or more sample values outside the slice group set is used for inter prediction of any sample within the slice group
set.


**num_slice_groups_in_set_minus1** plus 1 specifies the number of slice groups in the slice group set. The allowed range
of num_slice_groups_in_set_minus1 is 0 to num_slice_groups_minus1, inclusive. The allowed range of
num_slice_groups_minus1 is specified in Annex A and clauses G.10 and H.10.


**slice_group_id[** i **]** with i = 0.. num_slice_groups_in_set_minus1 identifies the slice group(s) contained within the slice
group set. The allowed range for slice_group_id[ i ] is from 0 to num_slice_groups_minus1, inclusive. The length of the
slice_group_id[ i ] syntax element is Ceil( Log2( num_slice_groups_minus1 + 1 ) ) bits.


When num_slice_groups_minus1 is equal to 0 (i.e., num_slice_groups_in_set_minus1 is equal to 0 and slice_group_id[ 0 ]
is not present), the value of slice_group_id[ 0 ] shall be inferred to be equal to 0.


**exact_sample_value_match_flag** equal to 0 indicates that, within the target picture set, when the macroblocks that do not
belong to the slice group set are not decoded, the value of each sample in the slice group set need not be exactly the same
as the value of the same sample when all the macroblocks are decoded. exact_sample_value_match_flag equal to 1
indicates that, within the target picture set, when the macroblocks that do not belong to the slice group set are not decoded,
the value of each sample in the slice group set shall be exactly the same as the value of the same sample when all the
macroblocks in the target picture set are decoded.

NOTE 2 – When disable_deblocking_filter_idc is equal to 1 or 2 in all slices in the target picture set,
exact_sample_value_match_flag should be 1.


**pan_scan_rect_flag** equal to 0 specifies that pan_scan_rect_id is not present. pan_scan_rect_flag equal to 1 specifies that
pan_scan_rect_id is present.


**pan_scan_rect_id** indicates that the specified slice group set covers at least the pan-scan rectangle identified by
pan_scan_rect_id within the target picture set.

NOTE 3 – Multiple motion_constrained_slice_group_set SEI messages may be associated with the same IDR picture. Consequently,
more than one slice group set may be active within a target picture set.
NOTE 4 – The size, shape, and location of the slice groups in the slice group set may change within the target picture set.


**D.2.21** **Film grain characteristics SEI message semantics**


This SEI message provides the decoder with a parameterized model for film grain synthesis.

NOTE 1 – For example, an encoder could use the film grain characteristics SEI message to characterize film grain that was present
in the original source video material and was removed by pre-processing filtering techniques. Synthesis of simulated film grain on
the decoded images for the display process is optional and does not need to exactly follow the specified semantics of the film grain
characteristics SEI message. When synthesis of simulated film grain on the decoded images for the display process is performed,
there is no requirement that the method by which the synthesis is performed be the same as the parameterized model for the film
grain as provided in the film grain characteristics SEI message.
NOTE 2 – The display process is not specified in this Specification.
NOTE 3 – Society of Motion Picture and Television Engineers document RDD 5 specifies a film grain simulator based on the
information provided in the film grain characteristics SEI message.


The film grain models specified in the film grain characteristics SEI message are expressed for application to decoded
pictures that have 4:4:4 colour format with luma and chroma bit depths corresponding to the luma and chroma bit depths
of the film grain model and use the same colour representation domain as the identified film grain model. When the colour
format of the decoded video is not 4:4:4 or the decoded video uses a different luma or chroma bit depth from that of the
film grain model or uses a different colour representation domain from that of the identified film grain model, an
unspecified conversion process is expected to be applied to convert the decoded pictures to the form that is expressed for
application of the film grain model.

NOTE 4 – Because the use of a specific method is not required for performing the film grain generation function used by the display
process, a decoder could, if desired, down-convert the model information for chroma in order to simulate film grain for other chroma
formats (4:2:0 or 4:2:2) rather than up-converting the decoded video (using a method not specified in this Specification) before
performing film grain generation.


**film_grain_characteristics_cancel_flag** equal to 1 indicates that the SEI message cancels the persistence of any previous
film grain characteristics SEI message in output order. film_grain_characteristics_cancel_flag equal to 0 indicates that film
grain modelling information follows.


**film_grain_model_id** identifies the film grain simulation model as specified in Table D-5. The value of
film_grain_model_id shall be in the range of 0 to 1, inclusive. The values of 2 and 3 for film_grain_model_id are reserved





for future use by ITU-T | ISO/IEC and shall not be present in bitstreams conforming to this version of this Specification.
Decoders shall ignore film grain characteristic SEI messages with film_grain_model_id equal to 2 or 3.


**Table D-5 – film_grain_model_id values**

|Value|Description|
|---|---|
|0|frequency filtering|
|1|auto-regression|
|2|reserved|
|3|reserved|



**separate_colour_description_present_flag** equal to 1 indicates that a distinct combination of luma bit depth, chroma bit
depth, video full range flag, colour primaries, transfer characteristics, and matrix coefficients for the film grain
characteristics specified in the SEI message is present in the film grain characteristics SEI message syntax.
separate_colour_description_present_flag equal to 0 indicates that the combination of luma bit depth, chroma bit depth,
video full range flag, colour primaries, transfer characteristics, and matrix coefficients for the film grain characteristics
specified in the SEI message are the same as indicated in the VUI parameters for the coded video sequence.

NOTE 5 – When separate_colour_description_present_flag is equal to 1, any of the luma bit depth, chroma bit depth, video full
range flag, colour primaries, transfer characteristics, and matrix coefficients specified for the film grain characteristics specified in
the SEI message could differ from those for the pictures in the coded video sequence.


When VUI parameters are not present for the coded video sequence or the value of colour_description_present_flag is
equal to 0, and equivalent information to that conveyed when colour_description_present_flag is equal to 1 is not conveyed
by external means, separate_colour_description_present_flag shall be equal to 1.


The decoded image Idecoded used in the equations in this clause is in the same colour representation domain as the simulated
film grain signal. Therefore, when any of these parameters does differ from that for the pictures in the coded video
sequence, the decoded image Idecoded used in the equations in this clause would be in a different colour representation
domain than that for the pictures in the coded video sequence. For example, when the value of
film_grain_bit_depth_luma_minus8 + 8 is greater than the bit depth of the luma component of the pictures in the coded
video sequence, the bit depth of Idecoded used in the equations in this clause is also greater than the bit depth of the luma
component of the pictures in the coded video sequence. In such a case, the decoded image Idecoded corresponding to an
actual decoded picture would be generated by converting the actual decoded picture to be in the same colour representation
domain as the simulated film grain signal. The process for converting the actual decoded pictures to the 4:4:4 colour format
with the same colour representation domain as the simulated film grain signal is not specified in this Specification.


**film_grain_bit_depth_luma_minus8** plus 8 specifies the bit depth used for the luma component of the film grain
characteristics specified in the SEI message. When film_grain_bit_depth_luma_minus8 is not present in the film grain
characteristics SEI message, the value of film_grain_bit_depth_luma_minus8 is inferred to be equal
to bit_depth_luma_minus8.


The value of filmGrainBitDepth[ 0 ] is derived as follows:


filmGrainBitDepth[ 0 ] = film_grain_bit_depth_luma_minus8 + 8 (D-14)


**film_grain_bit_depth_chroma_minus8** plus 8 specifies the bit depth used for the Cb and Cr components of the film grain
characteristics specified in the SEI message. When film_grain_bit_depth_chroma_minus8 is not present in the film grain
characteristics SEI message, the value of film_grain_bit_depth_chroma_minus8 is inferred to be equal
to bit_depth_chroma_minus8.


The value of filmGrainBitDepth[ c ] for c = 1 and 2 is derived as follows:


filmGrainBitDepth[ c ] = film_grain_bit_depth_chroma_minus8 + 8  with c = 1, 2 (D-15)


**film_grain_full_range_flag** has the same semantics as specified in clause E.2.1 for the video_full_range_flag syntax
element, except as follows:

- film_grain_full_range_flag specifies the video full range flag of the film grain characteristics specified in the SEI
message, rather than the video full range flag used for the coded video sequence.

- When film_grain_full_range_flag is not present in the film grain characteristics SEI message, the value of
film_grain_full_range_flag is inferred to be equal to video_full_range_flag.





**film_grain_colour_primaries** has the same semantics as specified in clause E.2.1 for the colour_primaries syntax
element, except as follows:

- film_grain_colour_primaries specifies the colour primaries of the film grain characteristics specified in the SEI
message, rather than the colour primaries used for the coded video sequence.

- When film_grain_colour_primaries is not present in the film grain characteristics SEI message, the value of
film_grain_colour_primaries is inferred to be equal to colour_primaries.


**film_grain_transfer_characteristics** has the same semantics as specified in clause E.2.1 for the transfer_characteristics
syntax element, except as follows:

- film_grain_transfer_characteristics specifies the transfer characteristics of the film grain characteristics specified in
the SEI message, rather than the transfer characteristics used for the coded video sequence.

- When film_grain_transfer_characteristics is not present in the film grain characteristics SEI message, the value of
film_grain_transfer_characteristics is inferred to be equal to transfer_characteristics.


**film_grain_matrix_coefficients** has the same semantics as specified in clause E.2.1 for the matrix_coefficients syntax
element, except as follows:

- film_grain_matrix_coefficients specifies the matrix coefficients of the film grain characteristics specified in the SEI
message, rather than the matrix coefficients used for the coded video sequence.

- When film_grain_matrix_coefficients is not present in the film grain characteristics SEI message, the value of
film_grain_matrix_coefficients is inferred to be equal to matrix_coefficients.

- The values allowed for film_grain_matrix_coefficients are not constrained by the chroma format of the decoded
pictures that is indicated by the value of chroma_format_idc for the semantics of the VUI parameters.


**blending_mode_id** identifies the blending mode used to blend the simulated film grain with the decoded images as
specified in Table D-6. blending_mode_id shall be in the range of 0 to 1, inclusive. The values of 2 and 3 for
blending_mode_id are reserved for future use by ITU-T | ISO/IEC and shall not be present in bitstreams conforming to
this version of this Specification. Decoders shall ignore film grain characteristic SEI messages with blending_mode_id
equal to 2 or 3.


**Table D-6 – blending_mode_id values**

|Value|Description|
|---|---|
|0|additive|
|1|multiplicative|
|2|reserved|
|3|reserved|



Depending on the value of blending_mode_id, the blending mode is specified as follows:

- If blending_mode_id is equal to 0 the blending mode is additive as specified by


Igrain[ c ][ x ][ y ] = Clip3( 0, ( 1 << filmGrainBitDepth[ c ] ) − 1, Idecoded[ c ][ x ][ y ] + G[ c ][ x ][ y ] ) (D-16)


- Otherwise (blending_mode_id is equal to 1), the blending mode is multiplicative as specified by


Igrain[ c ][ x ][ y ] = Clip3( 0, ( 1 << filmGrainBitDepth[ c ] ) − 1, Idecoded[ c ][ x ][ y ] (D-17)
+ Round( ( Idecoded[ c ][ x ][ y ] * G[ c ][ x ][ y ] ) ÷ ( ( 1 << bitDepth[ c ] ) − 1 ) ) )


where Idecoded[ c ][ x ][ y ] represents the sample value at coordinates ( x, y ) of the colour component c of the decoded
image Idecoded, G[ c ][ x ][ y ] is the simulated film grain value at the same position and colour component, and
filmGrainBitDepth[ c ] is the number of bits used for each sample in a fixed-length unsigned binary representation of the
arrays Igrain[ c ][ x ][ y ], Idecoded[ c ][ x ][ y ], and G[ c ][ x ][ y ], where c = 0..2, x = 0..PicWidthInSamplesL − 1, and
y = 0..PicHeightInSamplesL − 1


**log2_scale_factor** specifies a scale factor used in the film grain characterization equations.


**comp_model_present_flag[** c ] equal to 0 indicates that film grain is not modelled on the c-th colour component, where c
equal to 0 refers to the luma component, c equal to 1 refers to the Cb component, and c equal to 2 refers to the Cr component.
comp_model_present_flag[ c ] equal to 1 indicates that syntax elements specifying modelling of film grain on colour
component c are present in the SEI message.





When separate_colour_description_present_flag is equal to 0 and chroma_format_idc is equal to 0, the value of
comp_model_present_flag[ 1 ] and comp_model_present_flag[ 2 ] shall be equal to 0.


**num_intensity_intervals_minus1** [ c ] plus 1 specifies the number of intensity intervals for which a specific set of model
values has been estimated.

NOTE 6 – The intensity intervals could overlap in order to simulate multi-generational film grain.


**num_model_values_minus1** [ c ] plus 1 specifies the number of model values present for each intensity interval in which
the film grain has been modelled. The value of num_model_values_minus1[ c ] shall be in the range of 0 to 5, inclusive.


**intensity_interval_lower_bound** [ c ][ i ] specifies the lower bound of the i-th intensity interval for which the set of model
values applies.


**intensity_interval_upper_bound** [ c ][ i ] specifies the upper bound of the i-th intensity interval for which the set of model
values applies.


The variable intensityIntervalIdx[ c ][ x ][ y ][ j ] represents the j-th index to the list of intensity intervals selected for the
sample value Idecoded[ c ][ x ][ y ] for c = 0..2, x = 0..PicWidthInSamplesL − 1, y = 0..PicHeightInSamplesL − 1, and
j = 0..numApplicableIntensityIntervals[ c ][ x ][ y ] − 1, where numApplicableIntensityIntervals[ c ][ x ][ y ] is derived
below.


Depending on the value of film_grain_model_id, the selection of one or more intensity intervals for the sample value
Idecoded[ c ][ x ][ y ] is specified as follows:

- The variable numApplicableIntensityIntervals[ c ][ x ][ y ] is initially set equal to 0.

- If film_grain_model_id is equal to 0, the following applies:


   - The top-left sample location ( xB, yB ) of the current 8x8 block b that contains the sample value
Idecoded[ c ][ x ][ y ] is derived as ( xB, yB ) = ( x / 8, y / 8 ).


   - The average value bavg of the current 8x8 block b is derived as follows:


sum8x8 = 0
for( i = 0; i < 8; i++ )
for( j = 0; j < 8; j++ )
sum8x8 += Idecoded[ c][ xB * 8 + i ][ yB * 8 + j ] (D-18)
bavg = Clip3( 0, 255, ( sum8x8 + ( 1 << ( filmGrainBitDepth[ c ] − 3 ) ) ) >> ( filmGrainBitDepth[ c ] − 2 ) )


   - The values of intensityIntervalIdx[ c ][ x ][ y ][ j ] and numApplicableIntensityIntervals[ c ][ x ][ y ] are derived
as follows:


for( i = 0, j = 0; i <= num_intensity_intervals_minus1[ c ]; i++ )
if( bavg >= intensity_interval_lower_bound[ c ][ i ] &&
bavg <= intensity_interval_upper_bound[ c ][ i ] ) {
intensityIntervalIdx[ c ][ x ][ y ][ j ] = i (D-19)
j++
}
numApplicableIntensityIntervals[ c ][ x ][ y ] = j


- Otherwise (film_grain_model_id is equal to 1), the values of intensityIntervalIdx[ c ][ x ][ y ][ j ] and
numApplicableIntensityIntervals[ c ][ x ][ y ] are derived as follows:


I8[ c ][ x ][ y ] = ( filmGrainBitDepth[ c ] = = 8 ) ? ( Idecoded[ c ][ x ][ y ] :
Clip3( 0, 255, ( Idecoded[ c ][ x ][ y ] +
( 1 << ( filmGrainBitDepth[ c ] − 9 ) ) ) >> ( filmGrainBitDepth[ c ] − 8 ) )
for( i = 0, j = 0; i <= num_intensity_intervals_minus1[ c ]; i++ )
if( I8[ c ][ x ][ y ] >= intensity_interval_lower_bound[ c ][ i ] &&
I8[ c ][ x ][ y ] <= intensity_interval_upper_bound[ c ][ i ] ) { (D-20)
intensityIntervalIdx[ c ][ x ][ y ][ j ] = i
j++
}
numApplicableIntensityIntervals[ c ][ x ][ y ] = j


Samples that do not fall into any of the defined intervals (i.e., those samples for which the value of
numApplicableIntensityIntervals[ c ][ x ][ y ] is equal to 0) are not modified by the grain generation function. Samples that
fall into more than one interval (i.e., those samples for which the value of numApplicableIntensityIntervals[ c ][ x ][ y ] is





greater than 1) will originate multi-generation grain. Multi-generation grain results from adding the grain computed
independently for each of the applicable intensity intervals.


In the equations in the remainder of this clause, the variable sj in each instance of the list comp_model_value[ c ][ sj ] is
the value of intensityIntervalIdx[ c ][ x ][ y ][ j ] derived for the sample value Idecoded[ c ][ x ][ y ].


**comp_model_value** [ c ][ i ][ j ] specifies the j-th model value present for the colour component c and the i-th intensity
interval. The set of model values has different meaning depending on the value of film_grain_model_id. The value of
comp_model_value[ c ][ i ][ j ] is constrained as follows, and could be additionally constrained as specified elsewhere in
this clause.

- If film_grain_model_id is equal to 0, comp_model_value[ c ][ i ][ j ] shall be in the range of 0 to 2 [filmGrainBitDepth[ c ]] - 1,
inclusive.

- Otherwise (film_grain_model_id is equal to 1), comp_model_value[ c ][ i ][ j ] shall be in the range of
−2 [( filmGrainBitDepth[ c ] − 1 )] to 2 [( filmGrainBitDepth[ c ] − 1 )]   - 1, inclusive.


Depending on the value of film_grain_model_id, the synthesis of the film grain is modelled as follows:

- If film_grain_model_id is equal to 0, a frequency filtering model enables simulating the original film grain for c = 0..2,
x = 0..PicWidthInSamplesL − 1, and y = 0..PicHeightInSamplesL − 1 as specified by:


G[ c ][ x ][ y ] = ( comp_model_value[ c ][ sj ][ 0 ] * Q[ c ][ x ][ y ] + comp_model_value[ c ][ sj ][ 5 ] *
G[ c − 1 ][ x ][ y ] ) >> log2_scale_factor (D-21)


where Q[ c ] is a two-dimensional random process generated by filtering 16x16 blocks gaussRv with random-variable
elements gaussRvij generated with a normalized Gaussian distribution (independent and identically distributed
Gaussian random variable samples with zero mean and unity variance) and the value of an element G[ c −1 ][ x ][ y ]
used in the right-hand side of the equation is inferred to be equal to 0 when c − 1 is less than 0.

NOTE 7 – A normalized Gaussian random variable can be generated from two independent, uniformly distributed random
variables over the interval from 0 to 1 (and not equal to 0), denoted as uRv0 and uRv1, using the Box-Muller
transformation specified by


gaussRvi,j = Sqrt( −2 * Ln( uRv0 ) ) * Cos( 2 * π * uRv1 ) (D-22)


where  is Archimedes' constant 3.141 592 653 589 793....

The band-pass filtering of blocks gaussRv can be performed in the discrete cosine transform (DCT) domain as follows:


for( y = 0; y < 16; y++ )
for( x = 0; x < 16; x++ )
if( ( x < comp_model_value[ c ][ sj ][ 3 ] && y < comp_model_value[ c ][ sj ][ 4 ] ) | | (D-23)
x > comp_model_value[ c ][ sj ][ 1 ] | | y > comp_model_value[ c ][ sj ][ 2 ] )
gaussRv[ x ][ y ] = 0
filteredRv = IDCT16x16( gaussRv )


where IDCT16x16( z ) refers to a unitary inverse discrete cosine transformation (IDCT) operating on a 16x16 matrix
argument z as specified by


IDCT16x16( z ) = r * z * r [T] (D-24)


where the superscript T indicates a matrix transposition and r is the 16x16 matrix with elements rij specified by



i * ( 2 * j + 1 ) * π



𝑟i,j =



( ( i = = 0 ) ? 1 ∶ Sqrt( 2 )



∗ Cos (
4



) (D-25)
32



where  is Archimedes' constant 3.141 592 653 589 793....

Q[ c ] is formed by the frequency-filtered blocks filteredRv.

NOTE 8 – Coded model values are based on blocks of size 16x16, but a decoder implementation could use other block
sizes. For example, decoders implementing the IDCT on 8x8 blocks could down-convert by a factor of two the set of
coded model values comp_model_value[ c ][ sj ][ i ] for i equal to 1..4.
NOTE 9 – To reduce the degree of visible blocks that can result from mosaicking the frequency-filtered blocks filteredRv,
decoders could apply a low-pass filter to the boundaries between frequency-filtered blocks.





- Otherwise (film_grain_model_id is equal to 1), an auto-regression model enables simulating the original film grain
for c = 0..2, x = 0..PicWidthInSamplesL − 1, and y = 0..PicHeightInSamplesL − 1 as specified by


G[ c ][ x ][ y ] = ( comp_model_value[ c ][ sj ][ 0 ] * n[ c ][ x ][ y ] +

comp_model_value[ c ][ sj ][ 1 ] * ( G[ c ][ x − 1 ][ y ] +
( ( comp_model_value[ c ][ sj ][ 4 ] * G[ c ][ x ][ y − 1 ] ) >> log2_scale_factor ) ) +
comp_model_value[ c ][ sj ][ 3 ] * ( ( ( comp_model_value[ c ][ sj ][ 4 ] * G[ c ][ x − 1 ][ y − 1 ] ) >>
log2_scale_factor ) + G[ c ][ x + 1 ][ y − 1 ] ) +
comp_model_value[ c ][ sj ][ 5 ] * ( G[ c ][ x − 2 ][ y ] +
( ( comp_model_value[ c ][ sj ][ 4 ] * comp_model_value[ c ][ sj ][ 4 ] * G[ c ][ x ][ y − 2 ] ) >>
( 2 * log2_scale_factor ) ) ) +
comp_model_value[ c ][ sj ][ 2 ] * G[ c − 1 ][ x ][ y ] ) >> log2_scale_factor (D-26)


where n[ c ][ x ][ y ] is a random variable with normalized Gaussian distribution (independent and identically
distributed Gaussian random variable samples with zero mean and unity variance for each value of c, x, and y) and
the value of an element G[ c ][ x ][ y ] used in the right-hand side of the equation is inferred to be equal to 0 when any
of the following conditions are true:

   - x is less than 0,

   - y is less than 0,

   - c is less than 0.


comp_model_value[ c ][ i ][ 0 ] provides the first model value for the model as specified by film_grain_model_id.
comp_model_value[ c ][ i ][ 0 ] corresponds to the standard deviation of the Gaussian noise term in the generation
functions specified in Equations D-21 through D-26.


comp_model_value[ c ][ i ][ 1 ] provides the second model value for the model as specified by film_grain_model_id. When
film_grain_model_id is equal to 0, comp_model_value[ c ][ i ][ 1 ] shall be greater than or equal to 0 and less than 16.


When not present in the film grain characteristics SEI message, comp_model_value[ c ][ i ][ 1 ] is inferred as follows:

- If film_grain_model_id is equal to 0, comp_model_value[ c ][ i ][ 1 ] is inferred to be equal to 8.

- Otherwise (film_grain_model_id is equal to 1), comp_model_value[ c ][ i ][ 1 ] is inferred to be equal to 0.


comp_model_value[ c ][ i ][ 1 ] is interpreted as follows:

- If film_grain_model_id is equal to 0, comp_model_value[ c ][ i ][ 1 ] indicates the horizontal high cut frequency to
be used to filter the DCT of a block of 16x16 random variables.

- Otherwise (film_grain_model_id is equal to 1), comp_model_value[ c ][ i ][ 1 ] indicates the first order spatial
correlation for neighbouring samples at positions (x − 1, y) and (x, y − 1).


comp_model_value[ c ][ i ][ 2 ] provides the third model value for the model as specified by film_grain_model_id. When
film_grain_model_id is equal to 0, comp_model_value[ c ][ i ][ 2 ] shall be greater than or equal to 0 and less than 16.


When not present in the film grain characteristics SEI message, comp_model_value[ c ][ i ][ 2 ] is inferred as follows:

- If film_grain_model_id is equal to 0, comp_model_value[ c ][ i ][ 2 ] is inferred to be equal
to comp_model_value[ c ][ i ][ 1 ]

- Otherwise (film_grain_model_id is equal to 1), comp_model_value[ c ][ i ][ 2 ] is inferred to be equal to 0.


comp_model_value[ c ][ i ][ 2 ] is interpreted as follows:

- If film_grain_model_id is equal to 0, comp_model_value[ c ][ i ][ 2 ] indicates the vertical high cut frequency to be
used to filter the DCT of a block of 16x16 random variables.

- Otherwise (film_grain_model_id is equal to 1), comp_model_value[ c ][ i ][ 2 ] indicates the colour correlation
between consecutive colour components.


comp_model_value **[** c ][ i ][ 3 ] provides the fourth model value for the model as specified by film_grain_model_id. When
film_grain_model_id is equal to 0, comp_model_value[ c ][ i ][ 3 ] shall be greater than or equal to 0 and less than or equal
to comp_model_value[ c ][ i ][ 1 ].


When not present in the film grain characteristics SEI message, comp_model_value[ c ][ i ][ 3 ] is inferred to be equal to 0.


comp_model_value[ c ][ i ][ 3 ] is interpreted as follows:

- If film_grain_model_id is equal to 0, comp_model_value[ c ][ i ][ 3 ] indicates the horizontal low cut frequency to be
used to filter the DCT of a block of 16x16 random variables.





- Otherwise (film_grain_model_id is equal to 1), comp_model_value[ c ][ i ][ 3 ] indicates the first order spatial
correlation for neighbouring samples at positions (x − 1, y − 1) and (x + 1, y − 1).


comp_model_value[ c ][ i ][ 4 ] provides the fifth model value for the model as specified by film_grain_model_id. When
film_grain_model_id is equal to 0, comp_model_value[ c ][ i ][ 4] shall be greater than or equal to 0 and less than or equal
to comp_model_value[ c ][ i ][ 2 ].


When not present in the film grain characteristics SEI message, comp_model_value[ c ][ i ][ 4 ] is inferred to be equal to
film_grain_model_id.


comp_model_value[ c ][ i ][ 4 ] is interpreted as follows:

- If film_grain_model_id is equal to 0, comp_model_value[ c ][ i ][ 4 ] indicates the vertical low cut frequency to be
used to filter the DCT of a block of 16x16 random variables.

- Otherwise (film_grain_model_id is equal to 1), comp_model_value[ c ][ i ][ 4 ] indicates the aspect ratio of the
modelled grain.


comp_model_value[ c ][ i ][ 5 ] provides the sixth model value for the model as specified by film_grain_model_id.


When not present in the film grain characteristics SEI message, comp_model_value[ c ][ i ][ 5 ] is inferred to be equal to 0.


comp_model_value[ c ][ i ][ 5 ] is interpreted as follows:

- If film_grain_model_id is equal to 0, comp_model_value[ c ][ i ][ 5 ] indicates the colour correlation between
consecutive colour components.

- Otherwise (film_grain_model_id is equal to 1), comp_model_value[ c ][ i ][ 5 ] indicates the second order spatial
correlation for neighbouring samples at positions (x, y − 2) and (x − 2, y).


**film_grain_characteristics_repetition_period** specifies the persistence of the film grain characteristics SEI message and
may specify a picture order count interval within which another film grain characteristics SEI message or the end of the
coded video sequence shall be present in the bitstream. The value of film_grain_characteristics_repetition_period shall be
in the range 0 to 16 384, inclusive.


film_grain_characteristics_repetition_period equal to 0 specifies that the film grain characteristics SEI message applies to
the current decoded picture only.


film_grain_characteristics_repetition_period equal to 1 specifies that the film grain characteristics SEI message persists in
output order until any of the following conditions are true:

- A new coded video sequence begins.

- A picture in an access unit containing a film grain characteristics SEI message is output having PicOrderCnt( ) greater
than PicOrderCnt( CurrPic ).


film_grain_characteristics_repetition_period greater than 1 specifies that the film grain characteristics SEI message persists
until any of the following conditions are true:

- A new coded video sequence begins.

- A picture in an access unit containing a film grain characteristics SEI message is output having PicOrderCnt( ) greater
than PicOrderCnt( CurrPic ) and less than or equal to PicOrderCnt( CurrPic ) +
film_grain_characteristics_repetition_period.


film_grain_characteristics_repetition_period greater than 1 indicates that another film grain characteristics SEI message
shall be present for a picture in an access unit that is output having PicOrderCnt( ) greater than PicOrderCnt( CurrPic ) and
less than or equal to PicOrderCnt( CurrPic ) + film_grain_characteristics_repetition_period; unless the bitstream ends or a
new coded video sequence begins without output of such a picture.


**D.2.22** **Deblocking filter display preference SEI message semantics**


This SEI message provides the decoder with an indication of whether the display of the cropped result of the deblocking
filter process specified in clause 8.7 or of the cropped result of the picture construction process prior to the deblocking
filter process specified in clause 8.5.14 is preferred by the encoder for the display of each decoded picture that is output.

NOTE 1 – The display process is not specified in this Recommendation | International Standard. The means by which an encoder
determines what to indicate as its preference expressed in a deblocking filter display preference SEI message is also not specified in
this Recommendation | International Standard, and the expression of an expressed preference in a deblocking filter display preference
SEI message does not impose any requirement on the display process.


**deblocking_display_preference_cancel_flag** equal to 1 indicates that the SEI message cancels the persistence of any
previous deblocking filter display preference SEI message in output order. deblocking_display_preference_cancel_flag
equal to 0 indicates that a display_prior_to_deblocking_preferred_flag and
deblocking_display_preference_repetition_period follow.





NOTE 2 – In the absence of the deblocking filter display preference SEI message, or after the receipt of a deblocking filter display
preference SEI message in which deblocking_display_preference_cancel_flag is equal to 1, the decoder should infer that the display
of the cropped result of the deblocking filter process specified in clause 8.7 is preferred over the display of the cropped result of the
picture construction process prior to the deblocking filter process specified in clause 8.5.14 for the display of each decoded picture
that is output.


**display_prior_to_deblocking_preferred_flag** equal to 1 indicates that the encoder preference is for the display process
(which is not specified in this Recommendation | International Standard) to display the cropped result of the picture
construction process prior to the deblocking filter process specified in clause 8.5.14 rather than the cropped result of the
deblocking filter process specified in clause 8.7 for each picture that is cropped and output as specified in Annex C.
display_prior_to_deblocking_preferred_flag equal to 0 indicates that the encoder preference is for the display process
(which is not specified in this Recommendation | International Standard) to display the cropped result of the deblocking
filter process specified in clause 8.7 rather than the cropped result of the picture construction process prior to the
deblocking filter process specified in clause 8.5.14 for each picture that is cropped and output as specified in Annex C.

NOTE 3 – The presence or absence of the deblocking filter display preference SEI message and the value of
display_prior_to_deblocking_preferred_flag does not affect the requirements of the decoding process specified in this
Recommendation | International Standard. Rather, it only provides an indication of when, in addition to fulfilling the requirements
of this Recommendation | International Standard for the decoding process, enhanced visual quality might be obtained by performing
the display process (which is not specified in this Recommendation | International Standard) in an alternative fashion. Encoders that
use the deblocking filter display preference SEI message should be designed with an awareness that unless the encoder restricts its
use of the DPB capacity specified in Annex A and clauses G.10 and H.10 for the profile and level in use, some decoders may not
have sufficient memory capacity for the storage of the result of the picture construction process prior to the deblocking filter process
specified in clause 8.5.14 in addition to the storage of the result of the deblocking filter process specified in clause 8.7 when
reordering and delaying pictures for display, and such decoders would therefore not be able to benefit from the preference indication.
By restricting its use of the DPB capacity, an encoder can be able to use at least half of the DPB capacity specified in Annex A and
clauses G.10 and H.10 while allowing the decoder to use the remaining capacity for storage of unfiltered pictures that have been
indicated as preferable for display until the output time arrives for those pictures.


**dec_frame_buffering_constraint_flag** equal to 1 indicates that the use of the frame buffering capacity of the HRD
decoded picture buffer (DPB) as specified by max_dec_frame_buffering has been constrained such that the coded video
sequence will not require a decoded picture buffer with more than Max( 1, max_dec_frame_buffering ) frame buffers to
enable the output of the decoded filtered or unfiltered pictures, as indicated by the deblocking filter display preference SEI
messages, at the output times specified by the dpb_output_delay of the picture timing SEI messages.
dec_frame_buffering_constraint_flag equal to 0 indicates that the use of the frame buffering capacity in the HRD may or
may not be constrained in the manner that would be indicated by dec_frame_buffering_constraint_flag equal to 1.


For purposes of determining the constraint imposed when dec_frame_buffering_constraint_flag is equal to 1, the quantity
of frame buffering capacity used at any given point in time by each frame buffer of the DPB that contains a picture shall
be derived as follows:

- If both of the following criteria are satisfied for the frame buffer, the frame buffer is considered to use two frame
buffers of capacity for its storage.

   - The frame buffer contains a frame or one or more fields that is marked as "used for reference", and

   - The frame buffer contains a picture for which both of the following criteria are fulfilled:

      - The HRD output time of the picture is greater than the given point in time.

      - It has been indicated in a deblocking filter display preference SEI message that the encoder preference for
the picture is for the display process to display the cropped result of the picture construction process prior
to the deblocking filter process specified in clause 8.5.14 rather than the cropped result of the deblocking
filter process specified in clause 8.7.

- Otherwise, the frame buffer is considered to use one frame buffer of DPB capacity for its storage.


When dec_frame_buffering_constraint_flag is equal to 1, the frame buffering capacity used by all of the frame buffers in
the DPB that contain pictures, as derived in this manner, shall not be greater than Max( 1, max_dec_frame_buffering )
during the operation of the HRD for the coded video sequence.


The value of dec_frame_buffering_constraint_flag shall be the same in all deblocking filter display preference SEI
messages of the coded video sequence.


**deblocking_display_preference_repetition_period** specifies the persistence of the deblocking filter display preference
SEI message and may specify a picture order count interval within which another deblocking filter display preference
message or the end of the coded video sequence shall be present in the bitstream. The value of
deblocking_display_preference_repetition_period shall be in the range 0 to 16 384, inclusive.


deblocking_display_preference_repetition_period equal to 0 specifies that the deblocking filter display preference SEI
message applies to the current decoded picture only.





deblocking_display_preference_repetition_period equal to 1 specifies that the deblocking filter display preference SEI
message persists in output order until any of the following conditions are true:

- A new coded video sequence begins.

- A picture in an access unit containing a deblocking filter display preference SEI message is output having
PicOrderCnt( ) greater than PicOrderCnt( CurrPic ).


deblocking_display_preference_repetition_period greater than 1 specifies that the deblocking filter display preference SEI
message persists until any of the following conditions are true:

- A new coded video sequence begins.

- A picture in an access unit containing a deblocking filter display preference SEI message is output having
PicOrderCnt( ) greater than PicOrderCnt( CurrPic ) and less than or equal to PicOrderCnt( CurrPic ) +
deblocking_display_preference_repetition_period.


deblocking_display_preference_repetition_period greater than 1 indicates that another deblocking filter display
preference SEI message shall be present for a picture in an access unit that is output having PicOrderCnt( ) greater than
PicOrderCnt( CurrPic ) and less than or equal to PicOrderCnt( CurrPic ) +
deblocking_display_preference_repetition_period; unless the bitstream ends or a new coded video sequence begins without
output of such a picture.


**D.2.23** **Stereo video information SEI message semantics**

NOTE 1 – The stereo video information SEI message is included in this Specification primarily for historical reasons. It is now
suggested to use the frame packing arrangement SEI message rather than the stereo video information SEI message to signal stereo
video information.


This SEI message provides the decoder with an indication that the entire coded video sequence consists of pairs of pictures
forming stereo-view content.


The stereo video information SEI message shall not be present in any access unit of a coded video sequence unless a stereo
video information SEI message is present in the first access unit of the coded video sequence.


**field_views_flag** equal to 1 indicates that all pictures in the current coded video sequence are fields and all fields of a
particular parity are considered a left view and all fields of the opposite parity are considered a right view for stereo-view
content. field_views_flag equal to 0 indicates that all pictures in the current coded video sequence are frames and
alternating frames in output order represent a view of a stereo view. The value of field_views_flag shall be the same in all
stereo video information SEI messages within a coded video sequence.


When the stereo video information SEI message is present and field_views_flag is equal to 1, the left view and right view
of a stereo video pair shall be coded as a complementary field pair, the display time of the first field of the field pair in
output order should be delayed to coincide with the display time of the second field of the field pair in output order, and
the spatial locations of the samples in each individual field should be interpreted for display purposes as representing
complete pictures as shown in Figure 6-1 rather than as spatially-distinct fields within a frame as shown in Figure 6-2.

NOTE 2 – The display process is not specified in this Recommendation | International Standard.


**top_field_is_left_view_flag** equal to 1 indicates that the top fields in the coded video sequence represent a left view and
the bottom fields in the coded video sequence represent a right view. top_field_is_left_view_flag equal to 0 indicates that
the bottom fields in the coded video sequence represent a left view and the top fields in the coded video sequence represent
a right view. When present, the value of top_field_is_left_view_flag shall be the same in all stereo video information SEI
messages within a coded video sequence.


**current_frame_is_left_view_flag** equal to 1 indicates that the current picture is the left view of a stereo-view pair.
current_frame_is_left_view_flag equal to 0 indicates that the current picture is the right view of a stereo-view pair.


**next_frame_is_second_view_flag** equal to 1 indicates that the current picture and the next picture in output order form a
stereo-view pair, and the display time of the current picture should be delayed to coincide with the display time of the next
picture in output order. next_frame_is_second_view_flag equal to 0 indicates that the current picture and the previous
picture in output order form a stereo-view pair, and the display time of the current picture should not be delayed for
purposes of stereo-view pairing.





**left_view_self_contained_flag** equal to 1 indicates that no inter prediction operations within the decoding process for the
left-view pictures of the coded video sequence refer to reference pictures that are right-view pictures.
left_view_self_contained_flag equal to 0 indicates that some inter prediction operations within the decoding process for
the left-view pictures of the coded video sequence may or may not refer to reference pictures that are right-view pictures.
Within a coded video sequence, the value of left_view_self_contained_flag in all stereo video information SEI messages
shall be the same.


**right_view_self_contained_flag** equal to 1 indicates that no inter prediction operations within the decoding process for
the right-view pictures of the coded video sequence refer to reference pictures that are left-view pictures.
right_view_self_contained_flag equal to 0 indicates that some inter prediction operations within the decoding process for
the right-view pictures of the coded video sequence may or may not refer to reference pictures that are left-view pictures.
Within a coded video sequence, the value of right_view_self_contained_flag in all stereo video information SEI messages
shall be the same.


**D.2.24** **Post-filter hint SEI message semantics**


This SEI message provides the coefficients of a post-filter or correlation information for the design of a post-filter for
potential use in post-processing of the output decoded pictures to obtain improved displayed quality.


**filter_hint_size_y** specifies the vertical size of the filter coefficient or correlation array. The value of filter_hint_size_y
shall be in the range of 1 to 15, inclusive.


**filter_hint_size_x** specifies the horizontal size of the filter coefficient or correlation array. The value of filter_hint_size_x
shall be in the range of 1 to 15, inclusive.


**filter_hint_type** identifies the type of the transmitted filter hints as specified in Table D-7. The value of filter_hint_type
shall be in the range of 0 to 2, inclusive. Decoders shall ignore post-filter hint SEI messages having filter_hint_type equal
to the reserved value 3.


**Table D-7 – filter_hint_type values**

|Value|Description|
|---|---|
|0|coefficients of a 2D FIRfilter|
|1|coefficients of two 1D FIR filters|
|2|cross-correlation matrix|
|3|Reserved|



**filter_hint[** colour_component **][** cy **][** cx **]** specifies a filter coefficient or an element of a cross-correlation matrix between
original and decoded signal with 16-bit precision. The value of filter_hint[ colour_component ][ cy ][ cx ] shall be in the
range of −2 [31] + 1 to 2 [31] - 1, inclusive. colour_component specifies the related colour component. cy represents a counter
in vertical direction, cx represents a counter in horizontal direction. Depending on filter_hint_type, the following applies:


- If filter_hint_type is equal to 0, the coefficients of a 2-dimensional FIR filter with the size of filter_hint_size_y *
filter_hint_size_x are transmitted.


- Otherwise, if filter_hint_type is equal to 1, the filter coefficients of two 1-dimensional FIR filters are transmitted. In
this case, filter_hint_size_y shall be equal to 2. The index cy = 0 specifies the filter coefficients of the horizontal filter
and cy = 1 specifies the filter coefficients of the vertical filter. In the filtering process, the horizontal filter shall be
applied first and the result shall be filtered by the vertical filter.


- Otherwise (filter_hint_type is equal to 2), the transmitted hints specify a cross-correlation matrix between the original
signal s and the decoded signal s′.


NOTE 1 – The normalized cross-correlation matrix for a related colour component with the size of filter_hint_size_y *
filter_hint_size_x is defined as follows:



1



_w_




- 1



1



1
filter_hint (cy,cx) =



_h_








 - 1



= 8 ~~+~~ _bitDepth_ 2  _s_ ( _m_, _n_ )  _s_  ( _m_ + cy - _offset_ _ _y_, _n_ + cx
(2  - )1  _h_  _w_ _m_ = 0 _n_ = 0



8 ~~+~~ _bitDepth_ 2  _s_ ( _m_, _n_ )  _s_  ( _m_ + cy  - _offset_ _ _y_, _n_ + cx  - _offset_ _ _x_ )
(2 - )1  _h_  _w_ _m_ = 0 _n_ = 0



_bitDepth_ 2  _s_ ( _m_, _n_ )  _s_  ( _m_ + cy - _offset_ _ _y_, _n_ + cx - _offset_ _ _x_

   - )1  _h_  _w_ _m_ = 0 _n_ = 0



(D-27)



_m_



=



0



where _s_ denotes the original frame, _s′_ denotes the decoded frame, _h_ denotes the vertical height of the related colour
component, _w_ denotes the horizontal width of the related colour component, _bitDepth_ denotes the bit depth of the colour
component, _offset_y_ is equal to (filter_hint_size_y >> 1), _offset_x_ is equal to (filter_hint_size_x >> 1), 0 <= cy <
filter_hint_size_y and 0 <= cx < filter_hint_size_x.





NOTE 2 – A decoder can derive a Wiener post-filter from the cross-correlation matrix of original and decoded signal and the
auto-correlation matrix of the decoded signal.


**additional_extension_flag** equal to 0 indicates that no additional data follows within the post-filter hint SEI message. The
value of additional_extension_flag shall be equal to 0. The value of 1 for additional_extension_flag is reserved for future
use by ITU-T | ISO/IEC. Decoders shall ignore all data that follows the value of 1 for additional_extension_flag in a postfilter hint SEI message.


**D.2.25** **Tone mapping information SEI message semantics**


This SEI message provides information to enable remapping of the colour samples of the output decoded pictures for
customization to particular display environments. The remapping process maps coded sample values in the RGB colour
space (specified in Annex E) to target sample values. The mappings are expressed in the luma or RGB colour space domain,
and should be applied to the luma component or to each RGB component produced by colour space conversion of the
decoded image accordingly.


**tone_map_id** contains an identifying number that may be used to identify the purpose of the tone mapping model. The
value of tone_map_id shall be in the range of 0 to 2 [32] - 2, inclusive.


Values of tone_map_id from 0 to 255, inclusive, and from 512 to 2 [31] - 1, inclusive, may be used as determined by the
application. Values of tone_map_id from 256 to 511, inclusive, and from 2 [31] to 2 [32] - 2, inclusive, are reserved for future
use by ITU-T | ISO/IEC. Decoders encountering a value of tone_map_id in the range of 256 to 511, inclusive, or in the
range of 2 [31] to 2 [32] - 2, inclusive, shall ignore it.

NOTE 1 – The tone_map_id can be used to support tone mapping operations that are suitable for different display scenarios. For
example, different values of tone_map_id may correspond to different display bit depths.


**tone_map_cancel_flag** equal to 1 indicates that the SEI message cancels the persistence of any previous tone mapping
information SEI message in output order. tone_map_cancel_flag equal to 0 indicates that tone mapping information
follows.


**tone_map_repetition_period** specifies the persistence of the tone mapping information SEI message and may specify a
picture order count interval within which another tone mapping information SEI message with the same value of
tone_map_id or the end of the coded video sequence shall be present in the bitstream. The value of
tone_map_repetition_period shall be in the range of 0 to 16 384, inclusive.


tone_map_repetition_period equal to 0 specifies that the tone map information applies to the current decoded picture only.


tone_map_repetition_period equal to 1 specifies that the tone map information persists in output order until any of the
following conditions are true:

- A new coded video sequence begins.

- A picture in an access unit containing a tone mapping information SEI message with the same value of tone_map_id
is output having PicOrderCnt( ) greater than PicOrderCnt( CurrPic ).


tone_map_repetition_period equal to 0 or equal to 1 indicates that another tone mapping information SEI message with
the same value of tone_map_id may or may not be present.


tone_map_repetition_period greater than 1 specifies that the tone map information persists until any of the following
conditions are true:

- A new coded video sequence begins.

- A picture in an access unit containing a tone mapping information SEI message with the same value of tone_map_id
is output having PicOrderCnt( ) greater than PicOrderCnt( CurrPic ) and less than or equal to PicOrderCnt( CurrPic )
+ tone_map_repetition_period.


tone_map_repetition_period greater than 1 indicates that another tone mapping information SEI message with the same
value of tone_map_id shall be present for a picture in an access unit that is output having PicOrderCnt( ) greater than
PicOrderCnt( CurrPic ) and less than or equal to PicOrderCnt( CurrPic ) + tone_map_repetition_period; unless the
bitstream ends or a new coded video sequence begins without output of such a picture.


**coded_data_bit_depth** specifies the BitDepthY for interpretation of the luma component of the associated pictures for
purposes of interpretation of the tone mapping information SEI message. If tone mapping information SEI messages are
present that have coded_data_bit_depth that is not equal to BitDepthY, these refer to the hypothetical result of a transcoding
operation performed to convert the coded video to the BitDepthY corresponding to the value of coded_data_bit_depth.


The value of coded_data_bit_depth shall be in the range of 8 to 14, inclusive. Values of coded_data_bit_depth from 0 to 7
and from 15 to 255 are reserved for future use by ITU-T | ISO/IEC. Decoders shall ignore (remove from the bitstream and
discard) all tone mapping SEI messages that contain a coded_data_bit_depth in the range of 0 to 7 or in the range of 15 to
255, and bitstreams shall not contain such values.





**target_bit_depth** specifies the bit depth of the output of the dynamic range mapping function (or tone mapping function)
described by the tone mapping information SEI message. The tone mapping function specified with a particular
target_bit_depth is suggested to be reasonable for all display bit depths that are less than or equal to the target_bit_depth.


The value of target_bit_depth shall be in the range of 1 to 16, inclusive. Values of target_bit_depth equal to 0 and in the
range of 17 to 255 are reserved for future use by ITU-T | ISO/IEC. Decoders shall ignore (remove from the bitstream and
discard) all tone mapping SEI messages that contain a value of target_bit_depth equal to 0 or in the range of 17 to 255, and
bitstreams shall not contain such values.


**tone_map_model_id** specifies the model utilized for mapping the coded data into the target_bit_depth range. Values
greater than 3 are reserved for future use by ITU-T | ISO/IEC. Decoders shall ignore (remove from the bitstream and
discard) all tone mapping SEI messages that contain a value of tone_map_model_id greater than 4, and bitstreams shall
not contain such values. Decoders shall allow reserved values of tone_map_model_id in the range of 5 to 16 384, inclusive,
to be present in the bitstream.

NOTE 2 – A tone_map_model_id value of 0 corresponds to a linear mapping with clipping; a tone_map_model_id value of 1
corresponds to a sigmoidal mapping; a tone_map_model_id value of 2 corresponds to a user-defined table mapping, a
tone_map_model_id value of 3 corresponds to a piece-wise linear mapping, and a tone_map_model_id value of 4 corresponds to
luminance dynamic range information.


**min_value** specifies the RGB sample value that maps to the minimum value in the bit depth indicated by target_bit_depth.
It is used in combination with the max_value parameter. All sample values of the decoded picture that are less than or
equal to min_value after conversion to the RGB domain (when decoded in a different domain) are mapped to this minimum
value in the target_bit_depth representation.


**max_value** specifies the RGB sample value that maps to the maximum value in the bit depth indicated by target_bit_depth.
It is used in combination with the min_value parameter. All sample values of the decoded picture that are greater than or
equal to max_value after conversion to the RGB domain (when decoded in a different domain) are mapped to this
maximum value in the target_bit_depth representation.


max_value shall be greater than or equal to min_value.


**sigmoid_midpoint** specifies the RGB sample value of the coded data that is mapped to the centre point of the
target_bit_depth representation. It is used in combination with the sigmoid_width parameter.


**sigmoid_width** specifies the distance between two coded data values that approximately correspond to the 5% and 95%
values of the target_bit_depth representation, respectively. It is used in combination with the sigmoid_midpoint parameter
and is interpreted according to the following function:








 []





( _i_ ) = _Round_  2 _t_ arg _et_ _ _bit_ _ _depth_ - 1  _for_ _i_ =,0  (,2 _coded_ _ _bit_ _



= _Round_  2 _t_ arg _et_ _ _bit_ _ _depth_ - 1  _for_ _i_ =,0  (,2 _coded_ _ _bit_ _ _depth_



_f_ ( _i_ ) = _Round_   _for_ _i_ =,0  (,2 - )1 (D-28)

            - 6            - ~~(~~ _i_            - _sigmoid_midpoint_ ~~)~~ 



 _for_ _i_ =,0  (,2                     - )1

    - 6    - ~~(~~ _i_    - _sigmoid_midpoint_ ~~)~~ 

1 + exp [] [] []



=,0  (,2 _coded_ _ _bit_ _ _depth_


~~(~~ _i_ - _sigmoid_midpoint_ ~~)~~



 []




    - 6    - ~~(~~ _i_    + exp []



 - 6 - ~~(~~ _i_ - _sigmoid_midpoint_ ~~)~~ 

 []  []
 _sigmoid_width_ 



_for_ _i_








 []





_sigmoid_width_



where _f_ ( i ) denotes the function that maps an RGB sample value i from the coded data to a resulting RGB sample value in
the target_bit_depth representation.


**start_of_coded_interval** [ i ] specifies the beginning point of an interval in the coded data such that all RGB sample values
that are greater than or equal to start_of_coded_interval[ i ] and less than start_of_coded_interval[ i + 1 ] are mapped to i
in the target bit depth representation. The value of start_of_coded_interval[ 2 [target_bit_depth] ] is equal to 2 [coded_bit_depth] . The
number of bits used for the representation of the start_of_coded_interval is ( ( coded_data_bit_depth + 7 ) >> 3 ) << 3.


**num_pivots** specifies the number of pivot points in the piece-wise linear mapping function without counting the two
default end points, (0, 0) and (2 [coded_data_bit_depth] - 1, 2 [target_bit_depth] - 1).


**coded_pivot_value** [ i ] specifies the value in the coded_data_bit_depth corresponding to the i-th pivot point. The number
of bits used for the representation of the coded_pivot_value is ( ( coded_data_bit_depth + 7 ) >> 3 ) << 3.


**target_pivot_value** [ i ] specifies the value in the reference target_bit_depth corresponding to the i-th pivot point. The
number of bits used for the representation of the target_pivot_value is ( ( target_bit_depth + 7 ) >> 3 ) << 3.


**camera_iso_speed_idc** indicates the camera ISO speed for daylight illumination as specified by ISO 12232, interpreted
as specified by Table D-8. When camera_iso_speed_idc indicates Extended_ISO, the ISO speed is represented by
camera_iso_speed_value.


**camera_iso_speed_value** indicates the camera ISO speed for daylight illumination as specified by ISO 12232 when
camera_iso_speed_idc is set to Extended_ISO. The value of camera_iso_speed_value shall not be equal to 0.





**exposure_index_idc** indicates the exposure index setting of the camera as specified by ISO 12232, interpreted as specified
by Table D-8. When exposure_index_idc indicates Extended_ISO, the exposure index is indicated by
exposure_index_value.


The values of camera_iso_speed_idc and exposure_index_idc in the range of 31 to 254, inclusive, are reserved for future
use by ITU-T | ISO/IEC, and shall not be present in bitstreams conforming to this version of this Specification. Decoders
conforming to this version of this Specification shall ignore tone mapping SEI messages that contain these values.


**exposure_index_value** indicates the exposure index setting of the camera as specified by ISO 12232 when
exposure_index_idc indicates Extended_ISO. The value of exposure_index_value shall not be equal to 0.





**Table D-8 – Interpretation of camera_iso_speed_idc and exposure_index_idc**








|camera_iso_speed_idc or<br>exposure_index_idc|Indicated value|
|---|---|
|0|Unspecified|
|1|10|
|2|12|
|3|16|
|4|20|
|5|25|
|6|32|
|7|40|
|8|50|
|9|64|
|10|80|
|11|100|
|12|125|
|13|160|
|14|200|
|15|250|
|16|320|
|17|400|
|18|500|
|19|640|
|20|800|
|21|1000|
|22|1250|
|23|1600|
|24|2000|
|25|2500|
|26|3200|
|27|4000|
|28|5000|
|29|6400|
|30|8000|
|31..254|Reserved|
|255|Extended_ISO|






**exposure_compensation_value_sign_flag**, when applicable as specified below, specifies the sign of the variable
ExposureCompensationValue that indicates the exposure compensation value setting used for the process of image
production.


**exposure_compensation_value_numerator**, when applicable as specified below, specifies the numerator of the variable
ExposureCompensationValue that indicates the exposure compensation value setting used for the process of image
production.


**exposure_compensation_value_denom_idc**, when not equal to 0, specifies the denominator of the variable
ExposureCompensationValue that indicates the exposure compensation value setting used for the process of image
production.


When exposure_compensation_value_denom_idc is present and not equal to 0, the variable ExposureCompensationValue
is derived from exposure_compensation_value_sign_flag, exposure_compensation_value_numerator and
exposure_compensation_value_denom_idc. exposure_compensation_value_sign_flag equal to 0 indicates that the
ExposureCompensationValue is positive. exposure_compensation_value_sign_flag equal to 1 indicates that the
ExposureCompensationValue is negative. When ExposureCompensationValue is positive, the image is indicated to have
been further sensitized through the process of production, relative to the recommended exposure index of the camera as
specified by ISO 12232. When ExposureCompensationValue is negative, the image is indicated to have been further
desensitized through the process of production, relative to the recommended exposure index of the camera as specified by
ISO 12232.


When exposure_compensation_value_denom_idc is present and not equal to 0, the variable ExposureCompensationValue
is derived as follows:


ExposureCompensationValue = ( 1 − 2 * exposure_compensation_value_sign_flag ) *
exposure_compensation_value_numerator ÷
exposure_compensation_value_denom_idc (D-29)


The value of ExposureCompensationValue is interpreted in units of exposure steps such that an increase of 1 in
ExposureCompensationValue corresponds to a doubling of exposure in units of lux-seconds. For example, the exposure
compensation value equal to +1÷2 at the production stage may be indicated by setting
exposure_compensation_value_sign_flag to 0, exposure_compensation_value_numerator to 1, and
exposure_compensation_value_denom_idc to 2.


When exposure_compensation_value_denom_idc is present and equal to 0, the exposure compensation value is indicated
as unknown or unspecified.


**ref_screen_luminance_white** indicates the reference screen brightness setting for the extended range white level used for
image production process in units of candela per square metre.


**extended_range_white_level** indicates the luminance dynamic range for extended dynamic-range display of the
associated pictures, after conversion to the linear light domain for display, expressed as an integer percentage relative to
the nominal white level. When present, the value of extended_range_white_level should be greater than or equal to 100.


**nominal_black_level_luma_code_value** specifies the luma sample value of the associated decoded pictures to which the
nominal black level is assigned. For example, when coded_data_bit_depth is equal to 8, video_full_range_flag is equal to
0, and matrix_coefficients is equal to 1, nominal_black_level_luma_code_value should be equal to 16.


**nominal_white_level_luma_code_value** specifies the luma sample value of the associated decoded pictures to which the
nominal white level is assigned. For example, when coded_data_bit_depth is equal to 8, video_full_range_flag is equal to
0, and matrix_coefficients is equal to 1, nominal_white_level_luma_code_value should be equal to 235. When present, the
value of nominal_white_level_luma_code_value shall be greater than nominal_black_level_luma_code_value.


**extended_white_level_luma_code_value** specifies the luma sample value of the associated decoded pictures to which the
white level associated with an extended dynamic range is assigned. When present, the value of
extended_white_level_luma_code_value shall be greater than or equal to nominal_white_level_luma_code_value.


**D.2.26** **Frame packing arrangement SEI message semantics**


This SEI message informs the decoder that the output cropped decoded picture contains samples of multiple distinct
spatially packed constituent frames that are packed into one frame, or that the output cropped decoded pictures in output
order form a temporal interleaving of alternating first and second constituent frames, using an indicated frame packing
arrangement scheme. This information can be used by the decoder to appropriately rearrange and process the samples of
the constituent frames appropriately for display or other purposes (which are outside the scope of this Specification).





This SEI message may be associated with pictures that are either frames or fields. The frame packing arrangement of the
samples is specified in terms of the sampling structure of a frame in order to define a frame packing arrangement structure
that is invariant with respect to whether a picture is a single field of such a packed frame or is a complete packed frame.


**frame_packing_arrangement_id** contains an identifying number that may be used to identify the usage of the frame
packing arrangement SEI message. The value of frame_packing_arrangement_id shall be in the range of 0 to 2 [32] - 2,
inclusive.


Values of frame_packing_arrangement_id from 0 to 255, inclusive, and from 512 to 2 [31] - 1, inclusive, may be used as
determined by the application. Values of frame_packing_arrangement_id from 256 to 511, inclusive, and from 2 [31] to
2 [32] - 2, inclusive, are reserved for future use by ITU-T | ISO/IEC. Decoders encountering a value of
frame_packing_arrangement_id in the range of 256 to 511, inclusive, or in the range of 2 [31] to 2 [32] - 2, inclusive, shall ignore
it.


**frame_packing_arrangement_cancel_flag** equal to 1 indicates that the SEI message cancels the persistence of any
previous frame packing arrangement SEI message in output order. frame_packing_arrangement_cancel_flag equal to 0
indicates that frame packing arrangement information follows.


**frame_packing_arrangement_type** identifies the indicated interpretation of the sample arrays of the output cropped
decoded picture as specified in Table D-9.


**Table D-9 – Definition of frame_packing_arrangement_type**

|Value|Interpretation|
|---|---|
|0|The frame packing arrangement structure contains a "checkerboard" based interleaving of corresponding<br>planes of two constituent frames as illustrated in Figure D-1.|
|1|The frame packing arrangement structure contains a column based interleaving of corresponding planes of<br>two constituent frames as illustrated in Figure D-2 and Figure D-3.|
|2|The frame packing arrangement structure contains a row based interleaving of corresponding planes of two<br>constituent frames as illustrated in Figure D-4 and Figure D-5.|
|3|The frame packing arrangement structure contains a side-by-side packing arrangement of corresponding<br>planes of two constituent frames as illustrated in Figure D-6, Figure D-7, and Figure D-10.|
|4|The frame packing arrangement structure contains a top-bottom packing arrangement of corresponding<br>planes of two constituent frames as illustrated in Figure D-8 and Figure D-9.|
|5|The component planes of the output cropped decoded pictures in output order form a temporal interleaving<br>of alternating first and second constituent frames as illustrated in Figure D-11.|
|6|The output cropped decoded picture constitutes a complete 2D frame (when field_pic_flag is equal to 0) or<br>field (when field_pic_flag is equal to 1) without any frame packing.|
|7|The frame packing arrangement structure contains a tile format packing arrangement of corresponding<br>planes of two constituent frames as illustrated in Figure D-12.|



NOTE 1 – Figure D-1 to Figure D-10 provide typical examples of rearrangement and upconversion processing for various packing
arrangement schemes. Actual characteristics of the constituent frames are signalled in detail by the subsequent syntax elements of
the frame packing arrangement SEI message. In Figure D-1 to Figure D-10, an upconversion processing is performed on each
constituent frame to produce frames having the same resolution as that of the decoded frame. An example of the upsampling method
to be applied to a quincunx sampled frame as shown in Figure D-1 or Figure D-10 is to fill in missing positions with an average of
the available spatially neighbouring samples (the average of the values of the available samples above, below, to the left and to the
right of each sample to be generated). The actual upconversion process to be performed, if any, is outside the scope of this
Specification.
NOTE 2 – The sample aspect ratio (SAR) indicated in the VUI parameters should indicate the output picture shape for the packed
decoded frame output by a decoder that does not interpret the frame packing arrangement SEI message. In the examples shown in
Figure D-1 to Figure D-10, the SAR produced in each upconverted colour plane would be the same as the SAR indicated in the VUI
parameters, since the illustrated upconversion process produces the same total number of samples from each constituent frame as
existed in the packed decoded frame.
NOTE 3 – When the output time of the samples of constituent frame 0 differs from the output time of the samples of constituent
frame 1 (i.e., when field_views_flag is equal to 1 or frame_packing_arrangement_type is equal to 5) and the display system in use
presents two views simultaneously, the display time for constituent frame 0 should be delayed to coincide with the display time for
constituent frame 1. (The display process is not specified in this Recommendation | International Standard.)





NOTE 4 – When field_views_flag is equal to 1 or frame_packing_arrangement_type is equal to 5, the value 0 for
fixed_frame_rate_flag is not expected to be prevalent in industry use of this SEI message.
NOTE 5 – frame_packing_arrangement_type equal to 5 describes a temporal interleaving process of different views.
NOTE 6 – Figure D-12 provides an illustration of the rearrangement process for the frame packing arrangement scheme for the
frame_packing_arrangement_type value of 7.


All other values of frame_packing_arrangement_type are reserved for future use by ITU-T | ISO/IEC. It is a requirement
of bitstream conformance that the bitstreams shall not contain such other values of frame_packing_arrangement_type.


**quincunx_sampling_flag** equal to 1 indicates that each colour component plane of each constituent frame is quincunx
sampled as illustrated in Figure D-1 or Figure D-10, and quincunx_sampling_flag equal to 0 indicates that the colour
component planes of each constituent frame are not quincunx sampled.


When frame_packing_arrangement_type is equal to 0, it is a requirement of bitstream conformance that quincunx_
sampling_flag shall be equal to 1. When frame_packing_arrangement_type is equal to 5, 6, or 7, it is a requirement of
bitstream conformance that quincunx_sampling_flag shall be equal to 0.

NOTE 7 – For any chroma format (4:2:0, 4:2:2, or 4:4:4), the luma plane and each chroma plane is quincunx sampled as illustrated
in Figure D-1 when quincunx_sampling_flag is equal to 1.


Let CroppedWidth and CroppedHeight be the width and height, respectively, of the cropped frame area output from the
decoder in units of luma samples, derived as follows:


CroppedWidth = PicWidthInSamplesL − CropUnitX * ( frame_crop_left_offset + frame_crop_right_offset )
(D-30)


CroppedHeight = 16 * FrameHeightInMbs − CropUnitY * ( frame_crop_top_offset + frame_crop_bottom_offset )
(D-31)


When frame_packing_arrangement_type is equal to 7, it is a requirement of bitstream conformance that CroppedWidth
and CroppedHeight shall be integer multiples of 3.


Let oneThirdWidth and oneThirdHeight be derived as follows:


oneThirdWidth = CroppedWidth / 3 (D-32)


oneThirdHeight = CroppedHeight / 3 (D-33)


When frame_packing_arrangement_type is equal to 7, the frame packing arrangement is composed of five rectangular
regions identified as R0, R1, R2, R3, and R4 as illustrated in Figure D-12.


The width and height of region R0 are specified in units of frame luma samples as follows:


r0W = 2 * oneThirdWidth (D-34)


r0H = 2* oneThirdHeight (D-35)


The width and height of region R1 are specified in units of frame luma samples as follows:


r1W = oneThirdWidth (D-36)


r1H = 2* oneThirdHeight (D-37)


The width and height of region R2 are specified in units of frame luma samples as follows:


r2W = oneThirdWidth (D-38)


r2H = oneThirdHeight (D-39)


The width and height of region R3 are specified in units of frame luma samples as follows:


r3W = oneThirdWidth (D-40)


r3H = oneThirdHeight (D-41)


The width and height of region R4 are specified in units of frame luma samples as follows:





r4W = oneThirdWidth (D-42)


r4H = oneThirdHeight (D-43)


When frame_packing_arrangement_type is equal to 7, constituent frame 0 is obtained by cropping from the decoded frames
region R0, and constituent frame 1 is obtained by stacking vertically regions R2 and R3 and placing the resulting rectangle
to the right of the region R1. Region R4 is not part of either constituent frame and is discarded.


**content_interpretation_type** indicates the intended interpretation of the constituent frames as specified in Table D-10.
Values of content_interpretation_type that do not appear in Table D-10 are reserved for future specification by ITU-T |
ISO/IEC. When frame_packing_arrangement_type is equal to 6, content_interpretation_type shall be equal to 0.


When frame_packing_arrangement_type is not equal to 6, for each specified frame packing arrangement scheme, there are
two constituent frames that are referred to as frame 0 and frame 1.

|Col1|Table D-10 – Definition of content_interpretation_type|
|---|---|
|**Value**|**Interpretation**|
|0|Unspecified relationship between the frame packed constituent frames|
|1|Indicates that the two constituent frames form the left and right views of a stereo view scene, with frame 0<br>being associated with the left view and frame 1 being associated with the right view|
|2|Indicates that the two constituent frames form the right and left views of a stereo view scene, with frame 0<br>being associated with the right view and frame 1 being associated with the left view|



NOTE 8 – The value 2 for content_interpretation_type is not expected to be prevalent in industry use of this SEI message. However,
the value was specified herein for purposes of completeness.


When frame_packing_arrangement_type is equal to 6, content_interpretation_type, frame0_self_contained_flag,
frame1_self_contained_flag, frame0_grid_position_x, frame0_grid_position_y, frame1_grid_position_x, and
frame1_grid_position_y have no meaning and shall be equal to 0, and decoders shall ignore the values of these syntax
elements. In this case, semantics for other values of these syntax elements are reserved for future specification by ITU-T |
ISO/IEC.


**spatial_flipping_flag** equal to 1, when frame_packing_arrangement_type is equal to 3 or 4, indicates that one of the two
constituent frames is spatially flipped relative to its intended orientation for display or other such purposes.


When frame_packing_arrangement_type is equal to 3 or 4 and spatial_flipping_flag is equal to 1, the type of spatial flipping
that is indicated is as follows:


- If frame_packing_arrangement_type is equal to 3, the indicated spatial flipping is horizontal flipping.


- Otherwise (frame_packing_arrangement_type is equal to 4), the indicated spatial flipping is vertical flipping.


When frame_packing_arrangement_type is not equal to 3 or 4, it is a requirement of bitstream conformance that
spatial_flipping_flag shall be equal to 0. When frame_packing_arrangement_type is not equal to 3 or 4, the value 1 for
spatial_flipping_flag is reserved for future use by ITU-T | ISO/IEC. When frame_packing_arrangement_type is not equal
to 3 or 4, decoders shall ignore the value 1 for spatial_flipping_flag.


**frame0_flipped_flag**, when spatial_flipping_flag is equal to 1, indicates which one of the two constituent frames is flipped.


When spatial_flipping_flag is equal to 1, frame0_flipped_flag equal to 0 indicates that frame 0 is not spatially flipped and
frame 1 is spatially flipped, and frame0_flipped_flag equal to 1 indicates that frame 0 is spatially flipped and frame 1 is
not spatially flipped.


When spatial_flipping_flag is equal to 0, it is a requirement of bitstream conformance that frame0_flipped_flag shall be
equal to 0. When spatial_flipping_flag is equal to 0, the value 1 for spatial_flipping_flag is reserved for future use by ITUT | ISO/IEC. When spatial_flipping_flag is equal to 0, decoders shall ignore the value of frame0_flipped_flag.


**field_views_flag** equal to 1 indicates that all pictures in the current coded video sequence are coded as complementary
field pairs. All fields of a particular parity are considered a first constituent frame and all fields of the opposite parity are
considered a second constituent frame. When frame_packing_arrangement_type is not equal to 2, it is a requirement of
bitstream conformance that the field_views_flag shall be equal to 0. When frame_packing_arrangement_type is not equal
to 2, the value 1 for field_views_flag is reserved for future use by ITU-T | ISO/IEC. When frame_
packing_arrangement_type is not equal to 2, decoders shall ignore the value of field_views_flag.





**current_frame_is_frame0_flag** equal to 1, when frame_packing_arrangement is equal to 5, indicates that the current
decoded frame is constituent frame 0 and the next decoded frame in output order is constituent frame 1, and the display
time of the constituent frame 0 should be delayed to coincide with the display time of constituent frame 1.
current_frame_is_frame0_flag equal to 0, when frame_packing_arrangement is equal to 5, indicates that the current
decoded frame is constituent frame 1 and the previous decoded frame in output order is constituent frame 0, and the display
time of the constituent frame 1 should not be delayed for purposes of stereo-view pairing.


When frame_packing_arrangement_type is not equal to 5, the constituent frame associated with the upper-left sample of
the decoded frame is considered to be constituent frame 0 and the other constituent frame is considered to be constituent
frame 1. When frame_packing_arrangement_type is not equal to 5, it is a requirement of bitstream conformance that
current_frame_is_frame0_flag shall be equal to 0. When frame_packing_arrangement_type is not equal to 5, the value 1
for current_frame_is_frame0_flag is reserved for future use by ITU-T | ISO/IEC. When frame_packing_arrangement_type
is not equal to 5, decoders shall ignore the value of current_frame_is_frame0_flag.


**frame0_self_contained_flag** equal to 1 indicates that no inter prediction operations within the decoding process for the
samples of constituent frame 0 of the coded video sequence refer to samples of any constituent frame 1.
frame0_self_contained_flag equal to 0 indicates that some inter prediction operations within the decoding process for the
samples of constituent frame 0 of the coded video sequence may or may not refer to samples of some constituent frame 1.
When frame_packing_arrangement_type is equal to 0 or 1, it is a requirement of bitstream conformance that
frame0_self_contained_flag shall be equal to 0. When frame_packing_arrangement_type is equal to 0 or 1, the value 1 for
frame0_self_contained_flag is reserved for future use by ITU-T | ISO/IEC. When frame_packing_arrangement_type is
equal to 0 or 1, decoders shall ignore the value of frame0_self_contained_flag. Within a coded video sequence, the value
of frame0_self_contained_flag in all frame packing arrangement SEI messages shall be the same.


**frame1_self_contained_flag** equal to 1 indicates that no inter prediction operations within the decoding process for the
samples of constituent frame 1 of the coded video sequence refer to samples of any constituent frame 0.
frame1_self_contained_flag equal to 0 indicates that some inter prediction operations within the decoding process for the
samples of constituent frame 1 of the coded video sequence may or may not refer to samples of some constituent frame 0.
When frame_packing_arrangement_type is equal to 0 or 1, it is a requirement of bitstream conformance that
frame1_self_contained_flag shall be equal to 0. When frame_packing_arrangement_type is equal to 0 or 1, the value 1 for
frame1_self_contained_flag is reserved for future use by ITU-T | ISO/IEC. When frame_packing_arrangement_type is
equal to 0 or 1, decoders shall ignore the value of frame1_self_contained_flag. Within a coded video sequence, the value
of frame1_self_contained_flag in all frame packing arrangement SEI messages shall be the same.

NOTE 9 – When frame0_self_contained_flag is equal to 1 or frame1_self_contained_flag is equal to 1, and
frame_packing_arrangement_type is equal to 2, it is expected that the decoded frame should not be an MBAFF frame.


When quincunx_sampling_flag is equal to 0 and frame_packing_arrangement_type is not equal to 5, two (x, y) coordinate
pairs are specified to determine the indicated luma sampling grid alignment for constituent frame 0 and constituent frame
1, relative to the upper left corner of the rectangular area represented by the samples of the corresponding constituent
frame.

NOTE 10 – The location of chroma samples relative to luma samples can be indicated by the chroma_sample_loc_type_top_field
and chroma_sample_loc_type_bottom_field syntax elements in the VUI parameters.


**frame0_grid_position_x** (when present) specifies the x component of the (x, y) coordinate pair for constituent frame 0.


**frame0_grid_position_y** (when present) specifies the y component of the (x, y) coordinate pair for constituent frame 0.


**frame1_grid_position_x** (when present) specifies the x component of the (x, y) coordinate pair for constituent frame 1.


**frame1_grid_position_y** (when present) specifies the y component of the (x, y) coordinate pair for constituent frame 1.


When quincunx_sampling_flag is equal to 0 and frame_packing_arrangement_type is not equal to 5 the (x, y) coordinate
pair for each constituent frame is interpreted as follows:


- If the (x, y) coordinate pair for a constituent frame is equal to (0, 0), this indicates a default sampling grid alignment
specified as follows:


   - If frame_packing_arrangement_type is equal to 1 or 3, the indicated position is the same as for the (x, y)

coordinate pair value (4, 8), as illustrated in Figure D-2 and Figure D-6.


   - Otherwise (frame_packing_arrangement_type is equal to 2 or 4), the indicated position is the same as for the

(x, y) coordinate pair value (8, 4), as illustrated in Figure D-4 and Figure D-8.


- Otherwise, if the (x, y) coordinate pair for a constituent frame is equal to (15, 15), this indicates that the sampling
grid alignment is unknown or unspecified or specified by other means not specified in this Recommendation |
International Standard.


- Otherwise, the x and y elements of the (x, y) coordinate pair specify the indicated horizontal and vertical sampling
grid alignment positioning to the right of and below the upper left corner of the rectangular area represented by the





corresponding constituent frame, respectively, in units of one sixteenth of the luma sample grid spacing between the
samples of the columns and rows of the constituent frame that are present in the decoded frame (prior to any
upsampling for display or other purposes).
NOTE 11 – The spatial location reference information frame0_grid_position_x, frame0_grid_position_y, frame1_grid_position_x,
and frame1_grid_position_y is not provided when quincunx_sampling_flag is equal to 1 because the spatial alignment in this case
is assumed to be such that constituent frame 0 and constituent frame 1 cover corresponding spatial areas with interleaved quincunx
sampling patterns as illustrated in Figure D-1 and Figure D-10.
NOTE 12 – When frame_packing_arrangement_type is equal to 2 and field_views_flag is equal to 1, it is suggested that
frame0_grid_position_y should be equal to frame1_grid_position_y.


**frame_packing_arrangement_reserved_byte** is reserved for future use by ITU-T | ISO/IEC. It is a requirement of
bitstream conformance that the value of frame_packing_arrangement_reserved_byte shall be equal to 0. All other values
of frame_packing_arrangement_reserved_byte are reserved for future use by ITU-T | ISO/IEC. Decoders shall ignore
(remove from the bitstream and discard) the value of frame_packing_arrangement_reserved_byte.


**frame_packing_arrangement_repetition_period** specifies the persistence of the frame packing arrangement SEI
message and may specify a frame order count interval within which another frame packing arrangement SEI message with
the same value of frame_packing_arrangement_id or the end of the coded video sequence shall be present in the bitstream.
The value of frame_packing_arrangement_repetition_period shall be in the range of 0 to 16 384, inclusive.


frame_packing_arrangement_repetition_period equal to 0 specifies that the frame packing arrangement SEI message
applies to the current decoded frame only.


frame_packing_arrangement_repetition_period equal to 1 specifies that the frame packing arrangement SEI message
persists in output order until any of the following conditions are true:


- A new coded video sequence begins.

- A frame in an access unit containing a frame packing arrangement SEI message with the same value of
frame_packing_arrangement_id is output having PicOrderCnt( ) greater than PicOrderCnt( CurrPic ).


frame_packing_arrangement_repetition_period equal to 0 or equal to 1 indicates that another frame packing arrangement
SEI message with the same value of frame_packing_arrangement_id may or may not be present.


frame_packing_arrangement_repetition_period greater than 1 specifies that the frame packing arrangement SEI message
persists until any of the following conditions are true:


- A new coded video sequence begins.


- A frame in an access unit containing a frame packing arrangement SEI message with the same value of
frame_packing_arrangement_id is output having PicOrderCnt( ) greater than PicOrderCnt( CurrPic ) and less than or
equal to PicOrderCnt( CurrPic ) + frame_packing_arrangement_repetition_period.


frame_packing_arrangement_repetition_period greater than 1 indicates that another frame packing arrangement SEI
message with the same value of frame_packing_arrangement_frames_id shall be present for a frame in an access unit that
is output having PicOrderCnt( ) greater than PicOrderCnt( CurrPic ) and less than or equal to PicOrderCnt( CurrPic ) +
frame_packing_arrangement_repetition_period; unless the bitstream ends or a new coded video sequence begins without
output of such a frame.


**frame_packing_arrangement_extension_flag** equal to 0 indicates that no additional data follows within the frame
packing arrangement SEI message. It is a requirement of bitstream conformance that the value of frame_packing_
arrangement_extension_flag shall be equal to 0. The value 1 for frame_packing_arrangement_extension_flag is reserved
for future use by ITU-T | ISO/IEC. Decoders shall ignore the value 1 for frame_packing_arrangement_extension_flag in a
frame packing arrangement SEI message and shall ignore all data that follows within a frame packing arrangement SEI
message after the value 1 for frame_packing_arrangement_extension_flag.





**Figure D-1 – Rearrangement and upconversion of checkerboard interleaving**

**(frame_packing_arrangement_type equal to 0)**


**Figure D-2 – Rearrangement and upconversion of column interleaving**
**with frame_packing_arrangement_type equal to 1, quincunx_sampling_flag equal to 0,**

**and (x, y) equal to (0, 0) or (4, 8) for both constituent frames**





**Figure D-3 – Rearrangement and upconversion of column interleaving with**
**frame_packing_arrangement_type equal to 1, quincunx_sampling_flag equal to 0,**
**(x, y) equal to (0, 0) or (4, 8) for constituent frame 0 and (x, y) equal to (12, 8) for constituent frame 1**


**Figure D-4 – Rearrangement and upconversion of row interleaving with**
**frame_packing_arrangement_type equal to 2, quincunx_sampling_flag equal to 0,**

**and (x, y) equal to (0, 0) or (8, 4) for both constituent frames**





**Figure D-5 – Rearrangement and upconversion of row interleaving with**
**frame_packing_arrangement_type equal to 2, quincunx_sampling_flag equal to 0,**
**(x, y) equal to (0, 0) or (8, 4) for constituent frame 0, and (x, y) equal to (8, 12) for constituent frame 1**


**Figure D-6 – Rearrangement and upconversion of side-by-side packing arrangement with**

**frame_packing_arrangement_type equal to 3, quincunx_sampling_flag equal to 0,**

**and (x, y) equal to (0, 0) or (4, 8) for both constituent frames**





**Figure D-7 – Rearrangement and upconversion of side-by-side packing arrangement with**

**frame_packing_arrangement_type equal to 3, quincunx_sampling_flag equal to 0,**
**(x, y) equal to (12, 8) for constituent frame 0, and (x, y) equal to (0, 0) or (4, 8) for constituent frame 1**


**Figure D-8 – Rearrangement and upconversion of top-bottom packing arrangement with**

**frame_packing_arrangement_type equal to 4, quincunx_sampling_flag equal to 0,**

**and (x, y) equal to (0, 0) or (8, 4) for both constituent frames**





**Figure D-9 – Rearrangement and upconversion of top-bottom packing arrangement with**

**frame_packing_arrangement_type equal to 4, quincunx_sampling_flag equal to 0,**
**(x, y) equal to (8, 12) for constituent frame 0, and (x, y) equal to (0, 0) or (8, 4) for constituent frame 1**


**Figure D-** 10 **– Rearrangement and upconversion of side-by-side packing arrangement with quincunx sampling**

**(frame_packing_arrangement_type equal to 3 with quincunx_sampling_flag equal to 1)**





**Figure D-** 11 **– Rearrangement of a temporal interleaving frame arrangement**

**(frame_packing_arrangement_type equal to 5)**


**Figure D-** 12 **– Rearrangement and upconversion of tile format packing arrangement**

**(frame_packing_arrangement_type equal to 7)**


**D.2.27** **Display orientation SEI message semantics**


This SEI message informs the decoder of a transformation that is recommended to be applied to the output decoded and
cropped picture prior to display.


**display_orientation_cancel_flag** equal to 1 indicates that the SEI message cancels the persistence of any previous display
orientation SEI message in output order. display_orientation_cancel_flag equal to 0 indicates that display orientation
information follows.





**hor_flip** equal to 1 indicates that the cropped decoded picture should be flipped horizontally for display. hor_flip equal to
0 indicates that the decoded picture should not be flipped horizontally.


When hor_flip is equal to 1, the cropped decoded picture should be flipped as follows for each colour component Z = L,
Cb, and Cr, letting dZ be the final cropped array of output samples for the component Z:


for( x = 0; x < croppedWidthInSamplesZ; x++ )
for( y = 0; y < croppedHeightInSamplesZ; y++ )
dZ[ x][ y ] = Z[ croppedWidthInSamplesZ − x − 1 ][ y ]


**ver_flip** equal to 1 indicates that the cropped decoded picture should be flipped vertically (in addition to any horizontal
flipping when hor_flip is equal to 1) for display. ver_flip equal to 0 indicates that the decoded picture should not be flipped
vertically.


When ver_flip is equal to 1, the cropped decoded picture should be flipped as follows for each colour component Z = L,
Cb, and Cr, letting dZ be the final cropped array of output samples for the component Z:


for( x = 0; x < croppedWidthInSamplesZ; x++ )
for( y = 0; y < croppedHeightInSamplesZ; y++ )
dZ[ x ][ y ] = Z[ x ][ croppedWidthInSamplesZ − y − 1 ]


**anticlockwise_rotation** specifies the recommended anticlockwise rotation of the decoded picture (after applying
horizontal and/or vertical flipping when hor_flip or ver_flip is set) prior to display. The decoded picture should be rotated
by 360 * anticlockwise_rotation ÷ 2 [16] degrees (2 * π * anticlockwise_rotation ÷ 2 [16] radians, where π is Archimedes'
Constant (3.141 592 653 589 793 …) in the anticlockwise direction prior to display. For example, anticlockwise_rotation
equal to 0 indicates no rotation and anticlockwise_rotation equal to 16 384 indicates 90 degrees (π ÷ 2 radians) rotation in
the anticlockwise direction.

NOTE – It is possible for equivalent transformations to be expressed in multiple ways using these syntax elements. For example,
the combination of having both hor_flip and ver_flip equal to 1 with anticlockwise_rotation equal to 0 can alternatively be expressed
by having both hor_flip and ver_flip equal to 1 with anticlockwise_rotation equal to 0x8000000, and the combination of hor_flip
equal to 1 with ver_flip equal to 0 and anticlockwise_rotation equal to 0 can alternatively be expressed by having hor_flip equal to
0 with ver_flip equal to 1 and anticlockwise_rotation equal to 0x8000000.


**display_orientation_repetition_period** specifies the persistence of the display orientation SEI message and may specify
a picture order count interval within which another display orientation SEI message or the end of the coded video sequence
shall be present in the bitstream. The value of display_orientation_repetition_period shall be in the range 0 to 16 384,
inclusive.


display_orientation_repetition_period equal to 0 specifies that the display orientation SEI message applies to the current
decoded picture only.


display_orientation_repetition_period equal to 1 specifies that the display orientation SEI message persists in output order
until any of the following conditions are true:

- A new coded video sequence begins.

- A picture in an access unit containing a display orientation SEI message is output having PicOrderCnt( ) greater than
PicOrderCnt( CurrPic ).


display_orientation_repetition_period greater than 1 specifies that the display orientation SEI message persists until any
of the following conditions are true:

- A new coded video sequence begins.

- A picture in an access unit containing a display orientation SEI message is output having PicOrderCnt( ) greater than
PicOrderCnt( CurrPic ) and less than or equal to PicOrderCnt( CurrPic ) + display_orientation_repetition_period.


display_orientation_repetition_period greater than 1 indicates that another display orientation SEI message shall be present
for a picture in an access unit that is output having PicOrderCnt( ) greater than PicOrderCnt( CurrPic ) and less than or
equal to PicOrderCnt( CurrPic ) + display_orientation_repetition_period; unless the bitstream ends or a new coded video
sequence begins without output of such a picture.


**display_orientation_extension_flag** equal to 0 indicates that no additional data follows within the display orientation SEI
message. The value of display_orientation_extension_flag shall be equal to 0. The value of 1 for
display_orientation_extension_flag is reserved for future use by ITU-T | ISO/IEC. Decoders shall ignore all data that
follows the value of 1 for display_orientation_extension_flag in a display orientation SEI message.


**D.2.28** **Green metadata SEI message semantics**


The semantics for this SEI message are specified in ISO/IEC 23001-11 (Green metadata), which facilitates reduced power
consumption in decoders, encoders, displays, and in media selection.





**D.2.29** **Mastering display colour volume SEI message semantics**


This SEI message identifies the colour volume (the colour primaries, white point, and luminance range) of a display
considered to be the mastering display for the associated video content – e.g., the colour volume of a display that was used
for viewing while authoring the video content. The described mastering display is a three-colour additive display system
that has been configured to use the indicated mastering colour volume.


This SEI message does not specify the measurement methodologies and procedures used for determining the indicated
values or any description of the mastering environment. It also does not provide information on colour transformations
that would be appropriate to preserve creative intent on displays with colour volumes different from that of the described
mastering display.


The information conveyed in this SEI message is intended to be adequate for purposes corresponding to the use of Society
of Motion Picture and Television Engineers ST 2086.


When a mastering display colour volume SEI message is present in any access unit of a coded video sequence, a mastering
display colour volume SEI message shall be present in the IDR access unit that is the first access unit of the coded video
sequence. All mastering display colour volume SEI messages that apply to the same coded video sequence shall have the
same content.


The mastering display colour volume SEI message persists in decoding order from the current access unit until the end of
the coded video sequence.


**display_primaries_x[** c **]**, when in the range of 5 to 37 000, inclusive, specifies the normalized x chromaticity coordinate
of the colour primary component c of the mastering display, according to the CIE 1931 definition of x as specified in ISO
11664-1 (see also ISO 11664-3 and CIE 15), in increments of 0.00002. When display_primaries_x[ c ] is not in the range
of 5 to 37 000, inclusive, the normalized x chromaticity coordinate of the colour primary component c of the mastering
display is unknown or unspecified or specified by other means not specified in this Specification.


**display_primaries_y[** c **]**, when in the range of 5 to 42 000, inclusive, specifies the normalized y chromaticity coordinate
of the colour primary component c of the mastering display, according to the CIE 1931 definition of y as specified in ISO
11664-1 (see also ISO 11664-3 and CIE 15), in increments of 0.00002. When display_primaries_y[ c ] is not in the range
of 5 to 42 000, inclusive, the normalized y chromaticity coordinate of the colour primary component c of the mastering
display is unknown or unspecified or specified by other means not specified in this Specification.


For describing mastering displays that use red, green, and blue colour primaries, it is suggested that index value c equal to
0 should correspond to the green primary, c equal to 1 should correspond to the blue primary and c equal to 2 should
correspond to the red colour primary (see also Annex E and Table E-3).


**white_point_x**, when in the range of 5 to 37 000, inclusive, specifies the normalized x chromaticity coordinate of the white
point of the mastering display, according to the CIE 1931 definition of x as specified in ISO 11664-1 (see also ISO 116643 and CIE 15), in normalized increments of 0.00002. When white_point_x is not in the range of 5 to 37 000, inclusive, the
normalized x chromaticity coordinate of the white point of the mastering display is indicated to be unknown or unspecified
or specified by other means not specified in this Specification.


**white_point_y**, when in the range of 5 to 42 000, inclusive, specifies the normalized y chromaticity coordinate of the white
point of the mastering display, according to the CIE 1931 definition of y as specified in ISO 11664-1 (see also ISO 116643 and CIE 15), in normalized increments of 0.00002. When white_point_y is not in the range of 5 to 42 000, inclusive, the
normalized y chromaticity coordinate of the white point of the mastering display is indicated to be unknown or unspecified
or specified by other means not specified in this Specification.

NOTE 1 – Society of Motion Picture and Television Engineers ST 2086 (2018) specifies that the normalized x and y chromaticity
coordinate values for the mastering display colour primaries and white point are to be represented with four decimal places. This
would correspond with using values of the syntax elements display_primaries_x[ c ], display_primaries_y[ c ], white_point_x, and
white_point_y, as defined in this Specification, that are multiples of 5.
NOTE 2 – An example of the use of values outside the range for which semantics are specified in this Specification is that
ANSI/CTA 861-G (2016) uses normalized (x, y) chromaticity coordinate values of (0,0) for the white point to indicate that the white
point chromaticity is unknown.


**max_display_mastering_luminance**, when in the range of 50 000 to 100 000 000, specifies the nominal maximum
display luminance of the mastering display in units of 0.0001 candelas per square metre. When
max_display_mastering_luminance is not in the range of 50 000 to 100 000 000, the nominal maximum display luminance
of the mastering display is indicated to be unknown or unspecified or specified by other means not specified in this
Specification.

NOTE 3 – Society of Motion Picture and Television Engineers ST 2086 (2018) specifies that the nominal maximum display
luminance of the mastering display is to be specified as a multiple of 1 candela per square meter. This would correspond with using
values of the syntax element max_display_mastering_luminance, as defined in this Specification, that are a multiple of 10 000.





NOTE 4 – An example of the use of values outside the range for which semantics are specified in this Specification is that
ANSI/CTA 861-G (2016) uses the value 0 for the nominal maximum display luminance of the mastering display to indicate that the
nominal maximum display luminance of the mastering display is unknown.


**min_display_mastering_luminance**, when in the range of 1 to 50 000, specifies the nominal minimum display luminance
of the mastering display in units of 0.0001 candelas per square metre. When min_display_mastering_luminance is not in
the range of 1 to 50 000, the nominal maximum display luminance of the mastering display is unknown or unspecified or
specified by other means not specified in this Specification. When max_display_mastering_luminance is equal to 50 000,
min_display_mastering_luminance shall not be equal to 50 000.

NOTE 5 – Society of Motion Picture and Television Engineers ST 2086 (2018) specifies that the nominal minimum display
luminance of the mastering display is to be specified as a multiple of 0.0001 candelas per square metre, which corresponds to the
semantics specified in this Specification.
NOTE 6 – An example of the use of values outside the range for which semantics are specified in this Specification is that
ANSI/CTA 861-G (2016) uses the value 0 for the nominal minimum display luminance of the mastering display to indicate that the
nominal minimum display luminance of the mastering display is unknown.
NOTE 7 – Another example of the potential use of values outside the range for which semantics are specified in this Specification
is that Society of Motion Picture and Television Engineers ST 2086 (2018) indicates that values outside the specified range could
be used to indicate that the black level and contrast of the mastering display have been adjusted using picture line-up generation
equipment (PLUGE).


At the minimum luminance, the mastering display is considered to have the same nominal chromaticity as the white point.


**D.2.30** **Colour remapping information SEI message semantics**


The colour remapping information SEI message provides information to enable remapping of the reconstructed colour
samples of the output pictures for purposes such as converting the output pictures to a representation that is more suitable
for an alternative display. The colour remapping model used in the colour remapping information SEI message is composed
of a first piece-wise linear function applied to each colour component (specified by the "pre" set of syntax elements herein),
followed by a three-by-three matrix applied to the three resulting colour components, followed by a second piece-wise
linear function applied to each resulting colour component (specified by the "post" set of syntax elements herein).

NOTE 1 – Colour remapping of the output pictures for the display process (which is outside the scope of this Specification) is
optional and does not affect the decoding process specified in this Specification.


Unless indicated otherwise by some means not specified in this Specification, the input to the indicated remapping process
is the set of decoded sample values after applying an (unspecified) upsampling conversion process to the 4:4:4 colour
sampling format as necessary when the colour remapping three-by-three matrix coefficients are present in the SEI message
and chroma_format_idc is equal to 1 (4:2:0 chroma format) or 2 (4:2:2 chroma format). When chroma_format_idc is equal
to 0 (monochrome), the colour remapping information SEI message shall not be present, although decoders shall allow
such messages to be present and shall ignore any such colour remapping information SEI messages that may be present.


**colour_remap_id** contains an identifying number that may be used to identify the purpose of the colour remapping
information. The value of colour_remap_id may be used (in a manner not specified in this Specification) to indicate that
the input to the remapping process is the output of some conversion process that is not specified in this Specification, such
as a conversion of the picture to some alternative colour representation (e.g., conversion from a YCbCr colour
representation to a GBR colour representation). When more than one colour remapping information SEI message is present
with the same value of colour_remap_id, the content of these colour remapping information SEI messages shall be the
same. When colour remapping information SEI messages are present that have more than one value of colour_remap_id,
this may indicate that the remapping processes indicated by the different values of colour_remap_id are alternatives that
are provided for different purposes or that a cascading of remapping processes is to be applied in a sequential order (an
order that is not specified in this Specification). The value of colour_remap_id shall be in the range of 0 to 2 [32] - 2, inclusive.


Values of colour_remap_id from 0 to 255, inclusive, and from 512 to 2 [31] - 1, inclusive, may be used as determined by the
application. Values of colour_remap_id from 256 to 511, inclusive, and from 2 [31] to 2 [32] - 2, inclusive, are reserved for
future use by ITU-T | ISO/IEC. Decoders encountering a value of colour_remap_id in the range of 256 to 511, inclusive,
or in the range of 2 [31] to 2 [32] - 2, inclusive, shall ignore it.

NOTE 2 – The colour_remap_id can be used to support different colour remapping processes that are suitable for different display
scenarios. For example, different values of colour_remap_id may correspond to different remapped colour spaces supported by
displays.


**colour_remap_cancel_flag** equal to 1 indicates that the SEI message cancels the persistence of any previous colour
remapping information SEI message in output order that applies to the current layer. colour_remap_cancel_flag equal to 0
indicates that colour remapping information follows.


**colour_remap_repetition_period** specifies the persistence of the colour remapping information SEI message and may
specify a picture order count interval within which another colour remapping information SEI message with the same value
of colour_remap_id or the end of the coded video sequence shall be present in the bitstream. The value of
colour_remap_repetition_period shall be in the range of 0 to 16 384, inclusive.





colour_remap_repetition_period equal to 0 specifies that the colour remapping information applies to the current decoded
picture only.


colour_remap_repetition_period equal to 1 specifies that the colour remapping information persists in output order until
any of the following conditions are true:

- A new coded video sequence begins.

- A picture in an access unit containing a colour remapping information SEI message with the same value of
colour_remap_id is output having PicOrderCnt( ) greater than PicOrderCnt( CurrPic ).


colour_remap_repetition_period equal to 0 or equal to 1 indicates that another colour remapping information SEI message
with the same value of colour_remap_id may or may not be present.


colour_remap_repetition_period greater than 1 specifies that the colour remapping information persists until any of the
following conditions are true:

- A new coded video sequence begins.

- A picture in an access unit containing a colour remapping information SEI message with the same value of
colour_remap_id is output having PicOrderCnt( ) greater than PicOrderCnt( CurrPic ) and less than or equal to
PicOrderCnt( CurrPic ) + colour_remap_repetition_period.


colour_remap_repetition_period greater than 1 indicates that another colour remapping information SEI message with the
same value of colour_remap_id shall be present for a picture in an access unit that is output having PicOrderCnt( ) greater
than PicOrderCnt( CurrPic ) and less than or equal to PicOrderCnt( CurrPic ) + colour_remap_repetition_period; unless
the bitstream ends or a new coded video sequence begins without output of such a picture.


**colour_remap_video_signal_info_present_flag** equal to 1 specifies that syntax elements colour_remap_full_range_flag,
colour_remap_primaries, colour_remap_transfer_function and colour_remap_matrix_coefficients are present,
colour_remap_video_signal_info_present_flag equal to 0 specifies that syntax elements colour_remap_full_range_flag,
colour_remap_primaries, colour_remap_transfer_function and colour_remap_matrix_coefficients are not present.


**colour_remap_full_range_flag** has the same semantics as specified in clause E.2.1 for the video_full_range_flag syntax
element, except that colour_remap_full_range_flag identifies the colour space of the remapped reconstructed picture, rather
than the colour space used for the coded video sequence. When not present, the value of colour_remap_full_range_flag is
inferred to be equal to the value of video_full_range_flag.


**colour_remap_primaries** has the same semantics as specified in clause E.2.1 for the colour_primaries syntax element,
except that colour_remap_primaries identifies the colour space of the remapped reconstructed picture, rather than the
colour space used for the coded video sequence. When not present, the value of colour_remap_primaries is inferred to be
equal to the value of colour_primaries.


**colour_remap_transfer_function** has the same semantics as specified in clause E.2.1 for the transfer_characteristics
syntax element, except that colour_remap_transfer_function identifies the colour space of the remapped reconstructed
picture, rather than the colour space used for the coded video sequence. When not present, the value of
colour_remap_transfer_function is inferred to be equal to the value of transfer_characteristics.


**colour_remap_matrix_coefficients** has the same semantics as specified in clause E.2.1 for the matrix_coefficients syntax
element, except that colour_remap_matrix_coefficients identifies the colour space of the remapped reconstructed picture,
rather than the colour space used for the coded video sequence. When not present, the value of
colour_remap_matrix_coefficients is inferred to be equal to the value of matrix_coefficients.


**colour_remap_input_bit_depth** specifies the bit depth of the colour components of the associated pictures for purposes
of interpretation of the colour remapping information SEI message. When any colour remapping information SEI messages
is present with the value of colour_remap_input_bit_depth not equal to the bit depth of the decoded colour components,
the SEI message refers to the hypothetical result of a conversion operation performed to convert the decoded colour
component samples to the bit depth equal to colour_remap_input_bit_depth.


The value of colour_remap_input_bit_depth shall be in the range of 8 to 16, inclusive. Values of
colour_remap_input_bit_depth from 0 to 7, inclusive, and from 17 to 255, inclusive, are reserved for future use by ITU-T
| ISO/IEC. Decoders shall ignore all colour remapping SEI messages that contain a colour_remap_input_bit_depth in the
range of 0 to 7, inclusive, or in the range of 17 to 255, inclusive, and bitstreams shall not contain such values.


**colour_remap_output_bit_depth** specifies the bit depth of the output of the colour remapping function described by the
colour remapping information SEI message.


The value of colour_remap_output_bit_depth shall be in the range of 8 to 16, inclusive. Values of
colour_remap_output_bit_depth from 0 to 7, inclusive, and in the range of 17 to 255, inclusive, are reserved for future use
by ITU-T | ISO/IEC. Decoders shall ignore all colour remapping SEI messages that contain a value of





colour_remap_output_bit_depth from 0 to 7, inclusive, or in the range of 17 to 255, inclusive, and bitstreams shall not
contain such values.


**pre_lut_num_val_minus1[** c **]** plus 1 specifies the number of pivot points in the piece-wise linear remapping function for
the c-th component, where c equal to 0 refers to the luma or G component, c equal to 1 refers to the Cb or B component,
and c equal to 2 refers to the Cr or R component. When pre_lut_num_val_minus1[ c ] is equal to 0, the default end points
of the input values are 0 and 2 [colour_remap_input_bit_depth] - 1, and the corresponding default end points of the output values are 0
and 2 [colour_remap_output_bit_depth] - 1, for the c-th component. In bitstreams conforming to this version of this Specification, the
value of pre_lut_num_val_minus1[ c ] shall be in the range of 0 to 32, inclusive.


**pre_lut_coded_value[** c **][** i **]** specifies the value of the i-th pivot point for the c-th component. The number of bits used
to represent pre_lut_coded_value[ c ][ i ] is ( ( colour_remap_input_bit_depth + 7 ) >> 3 ) << 3.


**pre_lut_target_value[** c **][** i **]** specifies the value of the i-th pivot point for the c-th component. The number of bits used
to represent pre_lut_target_value[ c ][ i ] is ( ( colour_remap_output_bit_depth + 7 ) >> 3 ) << 3.


When pre_lut_coded_value[ c ][ 0 ] is greater than 0, an initial linear segment should be inferred that maps input values
ranging from 0 to pre_lut_coded_value[ c ][ 0 ], inclusive, to target values ranging from 0 to pre_lut_target_value[ c ][ 0 ],
inclusive.


When pre_lut_coded_value[ c ][ pre_lut_num_val_minus1[ c ] ] is not equal to 2 [colour_remap_input_bit_depth] - 1, a final linear
segment should be inferred that maps input values ranging from pre_lut_coded_value[ c ][ pre_lut_num_val_minus1[ c ] ]
to 2 [colour_remap_input_bit_depth] - 1, inclusive, to target values ranging from
pre_lut_target_value[ c ][ pre_lut_num_val_minus1[ c ] ] to 2 [colour_remap_output_bit_depth] - 1, inclusive.


**colour_remap_matrix_present_flag** equal to 1 indicates that the syntax elements log2_matrix_denom and
colour_remap_coeffs[ c ][ i ], for c and i in the range of 0 to 2, inclusive, are present. colour_remap_matrix_present_flag
equal to 0 indicates that the syntax elements log2_matrix_denom and colour_remap_coeffs[ c ][ i ], for c and i in the range
of 0 to 2, inclusive, are not present.


**log2_matrix_denom** specifies the base 2 logarithm of the denominator for all matrix coefficients. The value of
log2_matrix_denom shall be in the range of 0 to 15, inclusive. When not present, the value of log2_matrix_denom is
inferred to be equal to 0.


**colour_remap_coeffs[** c **][** i **]** specifies the value of the three-by-three colour remapping matrix coefficients. The value of
colour_remap_coeffs[ c ][ i ] shall be in the range of −2 [15] to 2 [15] - 1, inclusive. When colour_remap_coeffs[ c ][ i ] is not
present, it is inferred to be equal to 1 if c is equal to i, and inferred to be equal to 0 otherwise.

NOTE 3 – When colour_remap_matrix_present_flag is equal to 0, the colour remapping matrix is inferred to be equal to the identity
matrix of size 3x3.


The variable matrixOutput[ c ] for c = 0, 1 and 2 is derived as follows:


roundingOffset = log2_matrix_denom = = 0 ? 0: 1 << ( log2_matrix_denom − 1 )
matrixOutput[ c ] = Clip3( 0, ( 1 << colour_remap_output_bit_depth ) − 1, (D-44)
( colour_remap_coeffs[ c ][ 0 ] * matrixInput[ 0 ] + colour_remap_coeffs[ c ][ 1 ] * matrixInput[ 1 ]
+ colour_remap_coeffs[ c ][ 2 ] * matrixInput[ 2 ] + roundingOffset ) >> log2_matrix_denom )


where matrixInput[ c ] is the input sample value of the c-th colour component,and matrixOutput[ c ] is the output sample
value of the c-th colour component.


**post_lut_num_val_minus1[** c **]** has the same semantics as pre_lut_num_val_minus1[ c ], with pre replaced by post, except
that the default end points of the input values are 0 and 2 [colour_remap_output_bit_depth] - 1 for the c-th colour component. The value
of post_lut_num_val_minus1[ c ] shall be in the range of 0 to 32, inclusive.


**post_lut_coded_value[** c **][** i **]** has the same semantics as pre_lut_coded_value[ c ][ i ], with "pre" replaced by "post",
except that the number of bits used to represent post_lut_coded_value[ c ][ i ] is
( ( colour_remap_output_bit_depth + 7 ) >> 3 ) << 3.


**post_lut_target_value[** c **][** i **]** has the same semantics as pre_lut_target_value[ c ][ i ], with "pre" replaced by "post"
except that colour_remap_input_bit_depth is replaced by colour_remap_output_bit_depth in the semantics.


**D.2.31** **Content light level information SEI message semantics**


This SEI message identifies upper bounds for the nominal target brightness light level of the pictures of the coded video
sequence.


The information conveyed in this SEI message is intended to be adequate for purposes corresponding to the use of the
Consumer Technology Association 861.3 specification.





The semantics of the content light level information SEI message are defined in relation to the values of samples in a 4:4:4
representation of red, green, and blue colour primary intensities in the linear light domain for the pictures of the coded
video sequence, in units of candelas per square metre. However, this SEI message does not, by itself, identify a conversion
process for converting the sample values of a decoded picture to the samples in a 4:4:4 representation of red, green, and
blue colour primary intensities in the linear light domain for the picture.

NOTE 1 – Other syntax elements, such as colour_primaries, transfer_characteristics, matrix_coefficients, and the chroma
resampling filter hint SEI message, when present, may assist in the identification of such a conversion process.


Given the red, green, and blue colour primary intensities in the linear light domain for the location of a luma sample in a
corresponding 4:4:4 representation, denoted as ER, EG, and EB, the maximum component intensity is defined as EMax =
Max( ER, Max( EG, EB ) ). The light level corresponding to the stimulus is then defined as the CIE 1931 luminance
corresponding to equal amplitudes of EMax for all three colour primary intensities for red, green, and blue (with appropriate
scaling to reflect the nominal luminance level associated with peak white – e.g., ordinarily scaling to associate peak white
with 10 000 candelas per square metre when transfer_characteristics is equal to 16).

NOTE 2 – Since the maximum value EMax is used in this definition at each sample location, rather than a direct conversion from ER,
EG, and EB to the corresponding CIE 1931 luminance, the CIE 1931 luminance at a location may in some cases be less than the
indicated light level. This situation would occur, for example, when ER and EG are very small and EB is large, in which case the
indicated light level would be much larger than the true CIE 1931 luminance associated with the ( ER, EG, EB ) triplet.


When a content light level information SEI message is present in any access unit of a coded video sequence, a content light
level information SEI message shall be present in the IDR access unit that is the first access unit of the coded video
sequence. All content light level information SEI messages that apply to the same coded video sequence shall have the
same content.


The content light level information SEI message persists in decoding order from the current access unit until the end of the
coded video sequence.


**max_content_light_level**, when not equal to 0, indicates an upper bound on the maximum light level among all individual
samples in a 4:4:4 representation of red, green, and blue colour primary intensities (in the linear light domain) for the
pictures of the coded video sequence, in units of candelas per square metre. When equal to 0, no such upper bound is
indicated by max_content_light_level.


**max_pic_average_light_level**, when not equal to 0, indicates an upper bound on the maximum average light level among
the samples in a 4:4:4 representation of red, green, and blue colour primary intensities (in the linear light domain) for any
individual picture of the coded video sequence, in units of candelas per square metre. When equal to 0, no such upper
bound is indicated by max_pic_average_light_level.

NOTE 3 – When the visually relevant region does not correspond to the entire cropped decoded picture, such as for "letterbox"
encoding of video content with a wide picture aspect ratio within a taller cropped decoded picture, the indicated average should be
performed only within the visually relevant region.


**D.2.32** **Alternative transfer characteristics SEI message semantics**


The alternative transfer characteristics SEI message provides a preferred alternative value for the transfer_characteristics
syntax element that is indicated by the colour description syntax of VUI parameters of the sequence parameter set. This
SEI message is intended to be used in cases when some value of transfer_characteristics is preferred for interpretation of
the pictures of the coded video sequence although some other value of transfer_characteristics may also be acceptable for
interpretation of the pictures of the coded video sequence and that other value is provided in the colour description syntax
of VUI parameters of the sequence parameter set for interpretation by decoders that do not support interpretation of the
preferred value (e.g., because the preferred value had not yet been defined in a previous version of this Specification).


When an alternative transfer characteristics SEI message is present for any picture of a coded video sequence, an alternative
transfer characteristics SEI message shall be present for the IDR picture of the coded video sequence. The alternative
transfer characteristics SEI message persists in decoding order from the current picture until the end of the coded video
sequence. All alternative transfer characteristics SEI messages that apply to the same coded video sequence shall have the
same content.


**preferred_transfer_characteristics** specifies a preferred alternative value for the transfer_characteristics syntax element
of the colour description syntax of VUI parameters of the sequence parameter set. The semantics for
preferred_transfer_characteristics are otherwise the same as for the transfer_characteristics syntax element specified in the
VUI parameters of the sequence parameter set (see clause E.2.1 and Table E-4). When preferred_transfer_characteristics
is not equal to the value of transfer_characteristics indicated in the VUI parameters of the sequence parameter set, decoders
should ignore the value of transfer_characteristics indicated in the VUI parameters of the sequence parameter set and
instead use the value indicated by preferred_transfer_characteristics.


**D.2.33** **Content colour volume SEI message semantics**


The content colour volume SEI message describes the colour volume characteristics of the associated pictures. These
colour volume characteristics are expressed in terms of a nominal range, although deviations from this range may occur.





The variable transferCharacteristics is specified as follows:


- If an alternative transfer characteristics SEI message is present for the coded video sequence, transferCharacteristics
is set equal to preferred_transfer_characteristics;


- Otherwise, (an alternative transfer characteristics SEI message is not present for the coded video sequence),
transferCharacteristics is set equal to transfer_characteristics.


The content colour volume SEI message shall not be present, and decoders shall ignore it, when any of the following
conditions is true:


- Any of the values of transferCharacteristics, colour_primaries, and matrix_coefficients has a value defined as
unspecified.


- The value of transfer_characteristics is equal to 2, 4, or 5.


- The value of colour_primaries is equal to 2.


The following applies when converting the signal from a non-linear to a linear representation:


- If the value of transferCharacteristics is equal to 1, 6, 7, 14, or 15, the Rec. ITU-R BT.1886-0 reference electro-optical
transfer function should be used to convert the signal to its linear representation, where the value of screen luminance
for white is set equal to 100 cd/m [2], the value of screen luminance for black is set equal to 0 cd/m [2], and the value of
the exponent of the power function is set equal to 2.4.


- Otherwise, if the value of transferCharacteristics is equal to 18, the hybrid log-gamma reference electro-optical
transfer function specified in Rec. ITU-R BT.2100-2 should be used to convert the signal to its linear representation,
where the value of nominal peak luminance of the display is set equal to 1000 cd/m [2], the value of the display
luminance for black is set equal to 0 cd/m [2], and the value of system gamma is set equal to 1.2.


- Otherwise (the value of transferCharacteristics is not equal to 1, 6, 7, 14, 15, or 18) when the content colour volume
SEI message is present, the exact inverse of the transfer function specified in Table E-4 should be used to convert the
non-linear signal to a linear representation.


**ccv_cancel_flag** equal to 1 indicates that the SEI message cancels the persistence of any previous content colour volume
SEI message in output order. ccv_cancel_flag equal to 0 indicates that content colour volume information follows.


**ccv_persistence_flag** specifies the persistence of the content colour volume SEI message.


ccv_persistence_flag equal to 0 specifies that the content colour volume SEI message applies to the current decoded picture
only.


Let picA be the current picture. ccv_persistence_flag equal to 1 specifies that the content colour volume SEI message
persists in output order until one or more of the following conditions are true:

- A new coded video sequence begins.

- The bitstream ends.

- A picture picB in an access unit containing a content colour volume SEI message is output having PicOrderCnt( picB )
greater than PicOrderCnt( picA ).


**ccv_primaries_present_flag** equal to 1 specifies that the syntax elements ccv_primaries_x[ c ] and ccv_primaries_y[ c ]
are present. ccv_primaries_present_flag equal to 0 specifies that the syntax elements ccv_primaries_x[ c ] and
ccv_primaries_y[ c ] are not present.


**ccv_min_luminance_value_present_flag** equal to 1 specifies that the syntax element ccv_min_luminance_value is
present. ccv_min_luminance_value_present_flag equal to 0 specifies that the syntax element ccv_min_luminance_value
is not present.


**ccv_max_luminance_value_present_flag** equal to 1 specifies that the syntax element ccv_max_luminance_value is
present. ccv_max_luminance_value_present_flag equal to 0 specifies that the syntax element ccv_max_luminance_value
is not present.


**ccv_avg_luminance_value_present_flag** equal to 1 specifies that the syntax element ccv_avg_luminance_value is
present. ccv_avg_luminance_value_present_flag equal to 0 specifies that the syntax element ccv_avg_luminance_value is
not present.


It is a requirement of bitstream conformance that the values of ccv_primaries_present_flag,
ccv_min_luminance_value_present_flag, ccv_max_luminance_value_present_flag, and
ccv_avg_luminance_value_present_flag shall not all be equal to 0.





**ccv** _ **reserved_zero_2bits** shall be equal to 0 in bitstreams conforming to this version of this Specification. Other values
for ccv_reserved_zero_2bits are reserved for future use by ITU-T | ISO/IEC. Decoders shall ignore the value of
ccv_reserved_zero_2bits.


**ccv_primaries_x[** c **]** and **ccv_primaries_y[** c **]** specify the normalized x and y chromaticity coordinates, respectively, of
the colour primary component c of the nominal content colour volume in normalized increments of 0.00002, according to
the CIE 1931 definition of x and y as specified in ISO 11664-1 (see also ISO 11664-3 and CIE 15), in normalized
increments of 0.00002. For describing colour volumes that use red, green, and blue colour primaries, it is suggested that
index value c equal to 0 should correspond to the green primary, c equal to 1 should correspond to the blue primary, and c
equal to 2 should correspond to the red colour primary (see also Annex E and Table E-3).


The values of ccv_primaries_x[ c ] and ccv_primaries_y[ c ] shall be in the range of −5 000 000 to 5 000 000, inclusive.


When ccv_primaries_x[ c ] and ccv_primaries_y[ c ] are not present, they are inferred to be equal to the normalized x and
y chromaticity coordinates, respectively, specified by colour_primaries.


**ccv_min_luminance_value** specifies the normalized minimum luminance value, according to CIE 1931, that is expected
to be present in the content, where values are normalized to Lo or Lc as specified in Table E-4 according to the indicated
transfer characteristics of the signal. The values of ccv_min_luminance_value are in normalized increments of 0.0000001.


**ccv_max_luminance_value** specifies the maximum luminance value, according to CIE 1931, that is expected to be present
in the content, where values are normalized to Lo or Lc as specified in Table E-4 according to the transfer characteristics
of the signal. The values of ccv_max_luminance_value are in normalized increments of 0.0000001.


**ccv_avg_luminance_value** specifies the average luminance value, according to CIE 1931, that is expected to be present
in the content, where values are normalized to Lo or Lc as specified in Table E-4 according to the transfer characteristics
of the signal. The values of ccv_avg_luminance_value are in normalized increments of 0.0000001.

NOTE – The resulting domain from this conversion process may or may not represent light in a source or display domain – it is
merely a gamut representation domain rather than necessarily being a representation of actual light in either the scene or display
domain. Therefore, the values corresponding to ccv_min_luminance_value, ccv_max_luminance_value, and
ccv_avg_luminance_value might not necessarily correspond to a true luminance value.


The value of ccv_min_luminance_value, when present, shall be less than or equal to ccv_avg_luminance_value, when
present. The value of ccv_avg_luminance_value, when present, shall be less than or equal to ccv_max_luminance_value,
when present. The value of ccv_min_luminance_value, when present, shall be less than or equal to
ccv_max_luminance_value, when present.


When the visually relevant region does not correspond to the entire cropped decoded picture, such as for "letterbox"
encoding of video content with a wide picture aspect ratio within a taller cropped decoded picture, the indicated
ccv_min_luminance_value, ccv_max_luminance_value, and ccv_avg_luminance_value should correspond only to values
within the visually relevant region.


**D.2.34** **Ambient viewing environment SEI message semantics**


The ambient viewing environment SEI message identifies the characteristics of the nominal ambient viewing environment
for the display of the associated video content. The syntax elements of the ambient viewing environment SEI message may
assist the receiving system in adapting the received video content for local display in viewing environments that may be
similar or may substantially differ from those assumed or intended when mastering the video content.


This SEI message does not provide information on colour transformations that would be appropriate to preserve creative
intent on displays with colour volumes different from that of the described mastering display.


When an ambient viewing environment SEI message is present for any picture of a coded video sequence, an ambient
viewing environment SEI message shall be present in the IDR access unit that is the first access unit of the coded video
sequence. All ambient viewing environment SEI messages that apply to the same coded video sequence shall have the
same content.


**ambient_illuminance** specifies the environmental illluminance of the ambient viewing environment in units of 0.0001
lux. ambient_illuminance shall not be equal to 0.


**ambient_light_x** and **ambient_light_y** specify the normalized x and y chromaticity coordinates, respectively, of the
environmental ambient light in the nominal viewing environment, according to the CIE 1931 definition of x and y as
specified in ISO 11664-1 (see also ISO 11664-3 and CIE 15), in normalized increments of 0.00002. The values of
ambient_light_x and ambient_light_y shall be in the range of 0 to 50 000, inclusive.

NOTE – For example, the conditions identified in Rec. ITU-R BT.2035 can be expressed using ambient_illuminance equal to
100 000 with background chromaticity indicating D65 (ambient_light_x equal to 15 635, ambient_light_y equal to 16 450), or
optionally in some regions, background chromaticity indicating D93 (ambient_light_x equal to 14 155, ambient_light_y equal to
14 855).





**D.2.35** **Semantics of omnidirectional video specific SEI messages**


**D.2.35.1** **Equirectangular projection SEI message semantics**


The equirectangular projection SEI message provides information to enable remapping (through an equirectangular
projection) of the colour samples of the projected pictures onto a sphere coordinate space in sphere coordinates (ϕ, θ) for
use in omnidirectional video applications for which the viewing perspective is from the origin looking outward toward the
inside of the sphere. The sphere coordinates are defined so that ϕ is the azimuth (longitude, increasing eastward) and θ is
the elevation (latitude, increasing northward).


When an equirectangular projection SEI message is present for any picture of a coded video sequence, an equirectangular
projection SEI message shall be present for the first picture of the coded video sequence and no SEI message indicating a
different type of projection shall be present for any picture of the coded video sequence.


When aspect_ratio_idc is present and greater than 1 in the active sequence parameter set, there should be no equirectangular
projection SEI messages applicable for any picture of the coded video sequence.


A frame packing arrangement SEI message for which all the following conditions are true is referred to as an effectively
applicable frame packing arrangement SEI message:

- The value of frame_packing_arrangement_cancel_flag is equal to 0.

- The value of frame_packing_arrangement_type is equal to 3, 4, or 5.

- The value of quincunx_sampling_flag is equal to 0.

- The value of spatial_flipping_flag is equal to 0.

- The value of field_views_flag is equal to 0.

- The value of frame0_grid_position_x is equal to 0.

- The value of frame0_grid_position_y is equal to 0.

- The value of frame1_grid_position_x is equal to 0.

- The value of frame1_grid_position_y is equal to 0.


When an effectively applicable frame packing arrangement SEI message that applies to the picture is not present, an
equirectangular projection SEI message with erp_cancel_flag equal to 0 that applies to the picture shall not be present.
Decoders shall ignore equirectangular projection SEI messages when an effectively applicable frame packing arrangement
SEI message that applies to the picture is not present.


**erp_cancel_flag** equal to 1 indicates that the SEI message cancels the persistence of any previous equirectangular
projection SEI message in output order. erp_cancel_flag equal to 0 indicates that equirectangular projection information
follows.


**erp_persistence_flag** specifies the persistence of the equirectangular projection SEI message.


erp_persistence_flag equal to 0 specifies that the equirectangular projection SEI message applies to the current decoded
picture only.


Let picA be the current picture. erp_persistence_flag equal to 1 specifies that the equirectangular projection SEI message
persists in output order until one or more of the following conditions are true:

- A new coded video sequence begins.

- The bitstream ends.

- A picture picB in an access unit containing an equirectangular projection SEI message is output having
PicOrderCnt( picB ) greater than PicOrderCnt( picA ).


**erp_padding_flag** equal to 1 indicates that the constituent picture contains padded areas for which the sizes are specified
by the syntax elements left_gb_erp_width and right_gb_erp_width. erp_padding_flag equal to 0 indicates that the
constituent picture does not contains padded areas for which the sizes are specified by the syntax elements
left_gb_erp_width and right_gb_erp_width.

**erp** _ **reserved_zero_2bits** shall be equal to 0 in bitstreams conforming to this version of this Specification. Other values
for erp_reserved_zero_2bits are reserved for future use by ITU-T | ISO/IEC. Decoders shall ignore the value of
erp_reserved_zero_2bits.


**gb_erp_type** specifies the type of the guard bands as follows:

- gb_erp_type equal to 0 specifies that the content of the guard band in relation to the content of the constituent picture
is unspecified.





- gb_erp_type equal to 1 specifies that the content of the guard band suffices for interpolation of sample values at subpel sample fractional locations within the constituent picture.

NOTE – gb_erp_type equal to 1 could be used when the boundary samples of a constituent picture have been copied
horizontally to the guard band.

- gb_erp_type equal to 2 specifies that the content of the guard band represents actual picture content at quality that
gradually changes from the picture quality of the constituent picture.

- gb_erp_type equal to 3 specifies that the content of the guard bands represents actual picture content at a similar level
of quality as the constituent picture.

- gb_erp_type values greater than 3 are reserved. Decoders shall ignore the value of gb_erp_type when the value is
greater than 3.


**left_gb_erp_width** specifies the width of the guard band on the left side of the constituent picture in units of luma samples.
When erp_padding_flag is equal to 0, the value of left_gb_erp_width is inferred to be equal to 0. When the decoded picture
has 4:2:0 or 4:2:2 chroma format, left_gb_erp_width shall be an even number.


**right_gb_erp_width** specifies the width of the guard band on the right side of the constituent picture in units of luma
samples. When erp_padding_flag is equal to 0, the value of right_gb_erp_width is inferred to be equal to 0. When the
decoded picture has 4:2:0 or 4:2:2 chroma format, right_gb_erp_width shall be an even number.


**D.2.35.2** **Cubemap projection SEI message semantics**


The cubemap projection SEI message provides information to enable remapping (through a cubemap projection) of the
colour samples of the projected pictures onto a sphere coordinate space in sphere coordinates (ϕ, θ) for use in
omnidirectional video applications for which the viewing perspective is from the origin looking outward toward the inside
of the sphere. The sphere coordinates are defined so that ϕ is the azimuth (longitude, increasing eastward) and θ is the
elevation (latitude, increasing northward).


When a cubemap projection SEI message is present for any picture of a coded video sequence, a cubemap projection SEI
message shall be present for the first picture of the coded video sequence and no SEI message indicating a different type
of projection shall be present for any picture.


When aspect_ratio_idc is present and greater than 1 in the active sequence parameter set, there should be no cubemap
projection SEI messages applicable for any picture of the coded video sequence.


A frame packing arrangement SEI message for which all the following conditions are true is referred to as an effectively
applicable frame packing arrangement SEI message:

- The value of frame_packing_arrangement_cancel_flag is equal to 0.

- The value of frame_packing_arrangement_type is equal to 3, 4, or 5.

- The value of quincunx_sampling_flag is equal to 0.

- The value of spatial_flipping_flag is equal to 0.

- The value of field_views_flag is equal to 0.

- The value of frame0_grid_position_x is equal to 0.

- The value of frame0_grid_position_y is equal to 0.

- The value of frame1_grid_position_x is equal to 0.

- The value of frame1_grid_position_y is equal to 0.


When an effectively applicable frame packing arrangement SEI message that applies to the picture is not present, a
cubemap projection SEI message with cmp_cancel_flag equal to 0 that applies to the picture shall not be present. Decoders
shall ignore cubemap projection SEI messages when an effectively applicable frame packing arrangement SEI message
that applies to the picture is not present.


**cmp_cancel_flag** equal to 1 indicates that the SEI message cancels the persistence of any previous cubemap projection
SEI message in output order. cmp_cancel_flag equal to 0 indicates that cubemap projection information follows.


**cmp_persistence_flag** specifies the persistence of the cubemap projection SEI message.


cmp_persistence_flag equal to 0 specifies that the cubemap projection SEI message applies to the current decoded picture
only.


Let picA be the current picture. cmp_persistence_flag equal to 1 specifies that the cubemap projection SEI message persists
in output order until one or more of the following conditions are true:

- A new coded video sequence begins.





- The bitstream ends.

- A picture picB in an access unit containing a cubemap projection SEI message is output having PicOrderCnt( picB )
greater than PicOrderCnt( picA ).


**D.2.35.3** **Sphere rotation SEI message semantics**


The sphere rotation SEI message provides information on rotation angles yaw (α), pitch (β), and roll (γ) that are used for
conversion between the global coordinate axes and the local coordinate axes.


Relative to an (x, y, z) Cartesian coordinate system, yaw expresses a rotation around the z (vertical, up) axis, pitch rotates
around the y (lateral, side-to-side) axis, and roll rotates around the x (back-to-front) axis. Rotations are extrinsic, i.e.,
around x, y, and z fixed reference axes. The angles increase clockwise when looking from the origin towards the positive
end of an axis.


**sphere_rotation_cancel_flag** equal to 1 indicates that the SEI message cancels the persistence of any previous sphere
rotation SEI message in output order. sphere_rotation_cancel_flag equal to 0 indicates that sphere rotation information
follows.


**sphere_rotation_persistence_flag** specifies the persistence of the sphere rotation SEI message.


sphere_rotation_persistence_flag equal to 0 specifies that the sphere rotation SEI message applies to the current decoded
picture only.


Let picA be the current picture. sphere_rotation_persistence_flag equal to 1 specifies that the sphere rotation SEI message
persists in output order until one or more of the following conditions are true:


- A new coded video sequence begins.


- The bitstream ends.


- A picture picB in an access unit containing a sphere rotation SEI message is output having PicOrderCnt( picB ) greater
than PicOrderCnt( picA ).


When an equirectangular projection SEI message with erp_cancel_flag equal to 0 or a cubemap projection SEI message
with cmp_cancel_flag equal to 0 is not present in the coded video sequence that applies to the current picture and precedes
the sphere rotation SEI message in decoding order, a sphere rotation SEI message with sphere_rotation_cancel_flag equal
to 0 shall not be present in the coded video sequence that applies to the current picture. Decoders shall ignore sphere
rotation SEI messages with sphere_rotation_cancel_flag equal to 0 that do not follow, in decoding order, an equirectangular
projection SEI message with erp_cancel_flag equal to 0 or a cubemap projection SEI message with cmp_cancel_flag equal
to 0 in the coded video sequence that applies to the current picture.


**sphere_rotation_reserved_zero_6bits** shall be equal to 0 in bitstreams conforming to this version of this Specification.
Other values for sphere_rotation_reserved_zero_6bits are reserved for future use by ITU-T | ISO/IEC. Decoders shall
ignore the value of sphere_rotation_reserved_zero_6bits.


**yaw_rotation** specifies the value of the yaw rotation angle, in units of 2 [−16] degrees. The value of yaw_rotation shall be in
the range of −180 * 2 [16] (i.e., −11 796 480) to 180 * 2 [16] - 1 (i.e., 11 796 479), inclusive. When not present, the value of
yaw_rotation is inferred to be equal to 0.


**pitch_rotation** specifies the value of the pitch rotation angle, in units of 2 [−16] degrees. The value of pitch_rotation shall be
in the range of −90 * 2 [16] (i.e., −5 898 240) to 90 * 2 [16] (i.e., 5 898 240), inclusive. When not present, the value of
pitch_rotation is inferred to be equal to 0.


**roll_rotation** specifies the value of the roll rotation angle, in units of 2 [−16] degrees. The value of roll_rotation shall be in
the range of −180 * 2 [16] (i.e., −11 796 480) to 180 * 2 [16] - 1 (i.e., 11 796 479), inclusive. When not present, the value of
roll_rotation is inferred to be equal to 0.


**D.2.35.4** **Region-wise packing SEI message semantics**


The region-wise packing SEI message provides information to enable remapping of the colour samples of the cropped
decoded pictures onto projected pictures as well as information on the location and size of the guard bands, if any.


**rwp_cancel_flag** equal to 1 indicates that the SEI message cancels the persistence of any previous region-wise packing
SEI message in output order. rwp_cancel_flag equal to 0 indicates that region-wise packing information follows.


**rwp_persistence_flag** specifies the persistence of the region-wise packing SEI message.


rwp_persistence_flag equal to 0 specifies that the region-wise packing SEI message applies to the current decoded picture
only.





Let picA be the current picture. rwp_persistence_flag equal to 1 specifies that the region-wise packing SEI message persists
in output order until one or more of the following conditions are true:

- A new coded video sequence begins.

- The bitstream ends.

- A picture picB in an access unit containing a region-wise packing SEI message is output having PicOrderCnt( picB )
greater than PicOrderCnt( picA ).


When an equirectangular projection SEI message with erp_cancel_flag equal to 0 and erp_padding_flag equal to 0 or a
cubemap projection SEI message with cmp_cancel_flag equal to 0 is not present in the coded video sequence that applies
to the current picture and precedes the region-wise packing SEI message in decoding order, a region-wise packing SEI
message with rwp_cancel_flag equal to 0 shall not be present in the coded video sequence that applies to the current picture.
Decoders shall ignore region-wise packing SEI messages with rwp_cancel_flag equal to 0 that do not follow, in decoding
order, an equirectangular projection SEI message with erp_cancel_flag equal to 0 or a cubemap projection SEI message
with cmp_cancel_flag equal to 0 in the coded video sequence that applies to the current picture.


For the frame packing arrangement scheme indicated by a frame packing arrangement SEI message that applies to the
current picture, if a region-wise packing SEI message with rwp_cancel_flag equal to 0 is present that applies to the current
picture, the frame packing arrangement scheme applies to the projected picture, otherwise, the frame packing arrangement
scheme applies to the cropped decoded picture.


If a frame packing arrangement SEI message with frame_packing_arrangement_cancel_flag equal to 0,
frame_packing_arrangement_type equal to 3, 4, or 5, and quincunx_sampling_flag equal to 0 is not present that applies to
the current picture, the variables StereoFlag, TopBottomFlag, SideBySideFlag, and TempInterleavingFlag are all set equal
to 0, the variables HorDiv1 and VerDiv1 are both set equal to 1. Otherwise the following applies:

- StereoFlag is equal to 1.

- When the frame_packing_arrangement_type is equal to 3, SideBySideFlag is set equal to 1, TopBottomFlag and
TempInterleavingFlag are both set equal to 0, HorDiv1 is set equal to 2 and VerDiv1 is set equal to 1.

- When the frame_packing_arrangement_type is equal to 4, TopBottomFlag is set equal to 1, SideBySideFlag and
TempInterleavingFlag are both set equal to 0, HorDiv1 is set equal to 1 and VerDiv1 is set equal to 2.

- When the frame_packing_arrangement_type is equal to 5, TempInterleavingFlag is set equal to 1, TopBottomFlag
and SideBySideFlag are both set equal to 0, HorDiv1 and VerDiv1 are both set equal to 1.


**constituent_picture_matching_flag** equal to 1 specifies that the projected region information, packed region information,
and guard band region information in this SEI message apply individually to each constituent picture and that the packed
picture and the projected picture have the same stereoscopic frame packing format indicated by the frame packing
arrangement SEI message. constituent_picture_matching_flag equal to 0 specifies that the projected region information,
packed region information, and guard band region information in this SEI message apply to the projected picture.


When either of the following two conditions is true, the value of constituent_picture_matching_flag shall be equal to 0:

- StereoFlag is equal to 0.

- StereoFlag is equal to 1 and frame_packing_arrangement_type is equal to 5.


**rwp_reserved_zero_5bits** shall be equal to 0 in bitstreams conforming to this version of this Specification. Other values
for rwp_reserved_zero_56bits[ i ] are reserved for future use by ITU-T | ISO/IEC. Decoders shall ignore the value of
rwp_reserved_zero_5bits[ i ].


**num_packed_regions** specifies the number of packed regions when constituent_picture_matching_flag is equal to 0. The
value of num_packed_regions shall be greater than 0. When constituent_picture_matching_flag is equal to 1, the total
number of packed regions is equal to num_packed_regions * 2, and the information in each entry of the loop of
num_packed_regions entries applies to each constituent picture of the projected picture and the packed picture.


**proj_picture_width** and **proj_picture_height** specify the width and height, respectively, of the projected picture, in
relative projected picture sample units.


The values of proj_picture_width and proj_picture_height shall both be greater than 0.


**packed_picture_width** and **packed_picture_height** specify the width and height, respectively, of the packed picture, in
relative packed picture sample units.


The values of packed_picture_width and packed_picture_height shall both be greater than 0.


It is a requirement of bitstream conformance that packed_picture_width and packed_picture_height shall have such values
that packed_picture_width is an integer multiple of cropPicWidth and packed_picture_height is an integer multiple of
cropPicHeight, where cropPicWidth and cropPicHeight are the width and height, respectively, of the cropped decoded
picture.





**rwp_reserved_zero_4bits[** i **]** shall be equal to 0 in bitstreams conforming to this version of this Specification. Other
values for rwp_reserved_zero_4bits[ i ] are reserved for future use by ITU-T | ISO/IEC. Decoders shall ignore the value of
rwp_reserved_zero_4bits[ i ].


**transform_type[** i **]** specifies the rotation and mirroring to be applied to the i-th packed region to remap to the i-th projected
region. When transform_type[ i ] specifies both rotation and mirroring, rotation applies before mirroring. The values of
transform_type[ i ] are specified in Table D-11:


**Table D-11 – transform_type[ i ] values**

|Value|Description|
|---|---|
|0|no transform|
|1|mirroring horizontally|
|2|rotation by 180 degrees (anticlockwise)|
|3|rotation by 180 degrees (anticlockwise) before mirroring horizontally|
|4|rotation by 90 degrees (anticlockwise) before mirroring horizontally|
|5|rotation by 90 degrees (anticlockwise)|
|6|rotation by 270 degrees (anticlockwise) before mirroring horizontally|
|7|rotation by 270 degrees (anticlockwise)|



**guard_band_flag[** i **]** equal to 0 specifies that the i-th packed region does not have a guard band. guard_band_flag[ i ]
equal to 1 specifies that the i-th packed region has a guard band.


**proj_region_width[** i **]**, **proj_region_height[** i **]**, **proj_region_top[** i **]** and **proj_region_left[** i **]** specify the width,
height, top sample row, and the left-most sample column, respectively, of the i-th projected region, either within the
projected picture (when constituent_picture_matching_flag is equal to 0) or within the constituent picture of the projected
picture (when constituent_picture_matching_flag is equal to 1).


proj_region_width[ i ], proj_region_height[ i ], proj_region_top[ i ], and proj_region_left[ i ] are indicated in relative
projected picture sample units.


NOTE 1 – Two projected regions may partially or entirely overlap with each other.


**packed_region_width[** i **]**, **packed_region_height[** i **]**, **packed_region_top[** i **]**, and **packed_region_left[** i **]** specify the
width, height, the top luma sample row, and the left-most luma sample column, respectively, of the packed region, either
within the region-wise packed picture (when constituent_picture_matching_flag is equal to 0) or within each constituent
picture of the region-wise packed picture (when constituent_picture_matching_flag is equal to 1).


packed_region_width[ i ], packed_region_height[ i ], packed_region_top[ i ], and packed_region_left[ i ] are indicated in
relative region-wise packed picture sample units. packed_region_width[ i ], packed_region_height[ i ],
packed_region_top[ i ], and packed_region_left[ i ] shall represent integer horizontal and vertical coordinates of luma
sample units within the cropped decoded pictures.


NOTE 2 – Two packed regions may partially or entirely overlap with each other.


**left_gb_width[** i **]** specifies the width of the guard band on the left side of the i-th packed region in relative region-wise
packed picture sample units. When the decoded picture has 4:2:0 or 4:2:2 chroma format, left_gb_width[ i ] shall
correspond to an even number of luma samples within the cropped decoded picture.


**right_gb_width[** i **]** specifies the width of the guard band on the right side of the i-th packed region in relative region-wise
packed picture sample units. When the decoded picture has 4:2:0 or 4:2:2 chroma format, right_gb_width[ i ] shall
correspond to an even number of luma samples within the cropped decoded picture.


**top_gb_height[** i **]** specifies the height of the guard band above the i-th packed region in relative region-wise packed
picture sample units. When the decoded picture has 4:2:0 chroma format, top_gb_height[ i ] shall correspond to an even
number of luma samples within the cropped decoded picture.





**bottom_gb_height[** i **]** specifies the height of the guard band below the i-th packed region in relative region-wise packed
picture sample units. When the decoded picture has 4:2:0 chroma format, bottom_gb_height[ i ] shall correspond to an
even number of luma samples within the cropped decoded picture.


When guard_band_flag[ i ] is equal to 1, left_gb_width[ i ], right_gb_width[ i ], top_gb_height[ i ], or
bottom_gb_height[ i ] shall be greater than 0.


The i-th packed region as specified by this SEI message shall not overlap with any other packed region specified by the
same SEI message or any guard band specified by the same SEI message.


The guard bands associated with the i-th packed region, if any, as specified by this SEI message shall not overlap with any
packed region specified by the same SEI message or any other guard bands specified by the same SEI message.


**gb_not_used_for_pred_flag[** i **]** equal to 0 specifies that the guard bands may or may not be used in the inter prediction
process. gb_not_used_for_pred_flag[ i ] equal to 1 specifies that the sample values of the guard bands are not used in the
inter prediction process.


NOTE 3 – When gb_not_used_for_pred_flag[ i ] is equal to 1, the sample values within guard bands in cropped decoded pictures
can be rewritten even if the cropped decoded pictures were used as references for inter prediction of subsequent pictures to be
decoded. For example, the content of a packed region can be seamlessly expanded to its guard band with decoded and re-projected
samples of another packed region.


**gb_type[** i **][** j **]** specifies the type of the guard bands for the i-th packed region as follows, with j equal to 0, 1, 2, or 3
indicating that the semantics below apply to the left, right, top, or bottom edge, respectively, of the packed region:

- gb_type[ i ][ j ] equal to 0 specifies that the content of the guard bands in relation to the content of the packed regions
is unspecified. When gb_not_used_for_pred_flag[ i ] is equal to 0, gb_type[ i ][ j ] shall not be equal to 0.

- gb_type[ i ][ j ] equal to 1 specifies that the content of the guard bands suffices for interpolation of sample values at
sub-pel sample fractional locations within the packed region and less than sample outside of the boundary of the
packed region.


NOTE 4 – gb_type[ i ][ j ] equal to 1 can be used when the boundary samples of a packed region have been copied horizontally
or vertically to the guard band.

- gb_type[ i ][ j ] equal to 2 specifies that the content of the guard bands represents actual picture content that is
spherically adjacent to the content in the packed region and is on the surface of the packed region at quality that
gradually changes from the picture quality of the packed region to that of the spherically adjacent packed region.

- gb_type[ i ][ j ] equal to 3 specifies that the content of the guard bands represents actual picture content that is
spherically adjacent to the content in the packed region and is on the surface of the packed region at the picture quality
of the packed region.

- gb_type[ i ][ j ] values greater than 3 are reserved. Decoders shall ignore the value of gb_type[ i ][ j ] when the value
is greater than 3.


**rwp_gb_reserved_zero_3bits[** i **]** shall be equal to 0 in bitstreams conforming to this version of this Specification. Other
values for rwp_gb_reserved_zero_3bits[ i ] are reserved for future use by ITU-T | ISO/IEC. Decoders shall ignore the value
of rwp_gb_reserved_zero_3bits[ i ].


The variables NumPackedRegions, PackedRegionLeft[ n ], PackedRegionTop[ n ], PackedRegionWidth[ n ],
PackedRegionHeight[ n ], ProjRegionLeft[ n ], ProjRegionTop[ n ], ProjRegionWidth[ n ], ProjRegionHeight[ n ], and
TrasnformType[ n ] are derived as follows:


`-` For n in the range of 0 to num_packed_regions − 1, inclusive, the following applies:


`o` PackedRegionLeft[ n ] is set equal to packed_region_left[ n ].


`o` PackedRegionTop[ n ] is set equal to packed_region_top[ n ].


`o` PackedRegionWidth[ n ] is set equal to packed_region_width[ n ].


`o` PackedRegionHeight[ n ] is set equal to packed_region_height[ n ].


`o` ProjRegionLeft[ n ] is set equal to proj_region_left[ n ].


`o` ProjRegionTop[ n ] is set equal to proj_region_top[ n ].


`o` ProjRegionWidth[ n ] is set equal to proj_region_width[ n ].


`o` ProjRegionHeight[ n ] is set equal to proj_region_height[ n ].


`o` TransformType[ n ] is set equal to transform_type[ n ].


`-` If constituent_picture_matching_flag is equal to 0, the following applies:





`o` NumPackedRegions is set equal to num_packed_regions.


`-` Otherwise (constituent_picture_matching_flag is equal to 1), the following applies:


`o` NumPackedRegions is set equal to 2 * num_packed_regions.


`o` When TopBottomFlag is equal to 1, the following applies:


`o` projLeftOffset and packedLeftOffset are both set equal to 0.


`o` projTopOffset is set equal to proj_picture_height / 2 and packedTopOffset is set equal to
packed_picture_height / 2.


`o` When SideBySideFlag is equal to 1, the following applies:


`o` projLeftOffset is set equal to proj_picture_width / 2 and packedLeftOffset is set equal to
packed_picture_width / 2.


`o` projTopOffset and packedTopOffset are both set equal to 0.


`o` For n in the range of NumPackedRegions / 2 to NumPackedRegions − 1, inclusive, the following
applies:


          - nIdx is set equal to n − NumPackedRegions / 2.


          - PackedRegionLeft[ n ] is set equal to packed_region_left[ nIdx ] + packedLeftOffset.


          - PackedRegionTop[ n ] is set equal to packed_region_top[ nIdx ] + packedTopOffset.


          - PackedRegionWidth[ n ] is set equal to packed_region_width[ nIdx ].


          - PackedRegionHeight[ n ] is set equal to packed_region_height[ nIdx ].


          - ProjRegionLeft[ n ] is set equal to proj_region_left[ nIdx ] + projLeftOffset.


          - ProjRegionTop[ n ] is set equal to proj_region_top[ nIdx ] + projTopOffset.


          - ProjRegionWidth[ n ] is set equal to proj_region_width[ nIdx ].


          - ProjRegionHeight[ n ] is set equal to proj_region_height[ nIdx ].


          - TransformType[ n ] is set equal to transform_type[ nIdx ].


For each value of n in the range of 0 to NumPackedRegions − 1, inclusive, the values of ProjRegionWidth[ n ],
ProjRegionHeight[ n ], ProjRegionTop[ n ], and ProjRegionLeft[ n ] are constrained as follows:


`-` ProjRegionWidth[ n ] shall be in the range of 1 to proj_picture_width, inclusive.


`-` ProjRegionHeight[ n ] shall be in the range of 1 to proj_picture_height, inclusive.


`-` ProjRegionLeft[ n ] shall be in the range of 0 to proj_picture_width − 1, inclusive.


`-` ProjRegionTop[ n ] shall be in the range of 0 to proj_picture_height − 1, inclusive.


`-` If ProjRegionTop[ n ] is less than proj_picture_height / VerDiv1, the sum of ProjRegionTop[ n ] and
ProjRegionHeight[ n ] shall be less than or equal to proj_picture_height / VerDiv1. Otherwise, the sum of
ProjRegionTop[ n ] and ProjRegionHeight[ n ] shall be less than or equal to proj_picture_height / VerDiv1 * 2.


For each value of n in the range of 0 to NumPackedRegions − 1, inclusive, the values of PackedRegionWidth[ n ],
PackedRegionHeight[ n ], PackedRegionTop[ n ], and PackedRegionLeft[ n ] are constrained as follows:


`-` PackedRegionWidth[ n ] shall be in the range of 1 to packed_picture_width, inclusive.


`-` ProjRegionHeight[ n ] shall be in the range of 1 to packed_picture_height, inclusive.


`-` PackedRegionLeft[ n ] shall be in the range of 0 to packed_picture_width − 1, inclusive.


`-` PackedRegionTop[ n ] shall be in the range of 0 to packed_picture_height − 1, inclusive.


`-` If PackedRegionLeft[ n ] is less than packed_picture_width / HorDiv1, the sum of PackedRegionLeft[ n ] and
PackedRegionWidth[ n ] shall be less than or equal to packed_picture_width / HorDiv1. Otherwise, the sum of
PackedRegionLeft[ n ] and PackedRegionWidth[ n ] shall be less than or equal to
packed_picture_width / HorDiv1 * 2.


`-` If PackedRegionTop[ n ] is less than packed_picture_height / VerDiv1, the sum of PackedRegionTop[ n ] and
PackedRegionHeight[ n ] shall be less than or equal to packed_picture_height / VerDiv1. Otherwise, the sum of





PackedRegionTop[ n ] and PackedRegionHeight[ n ] shall be less than or equal to
packed_picture_height / VerDiv1 * 2.


`-` When chroma_format_idc is equal to 1 (4:2:0 chroma format) or 2 (4:2:2 chroma format), PackedRegionLeft[ n ]
shall correspond to an even horizontal coordinate value of luma sample units, and PackedRegionWidth[ n ] shall
correspond to an even number of luma samples, both within the decoded picture.


`-` When chroma_format_idc is equal to 1 (4:2:0 chroma format), PackedRegionTop[ n ] shall correspond to an
even vertical coordinate value of luma sample units, and ProjRegionHeight[ n ] shall correspond to an even
number of luma samples, both within the decoded picture.


**D.2.35.5** **Omnidirectional viewport SEI message semantics**


The omnidirectional viewport SEI message specifies the coordinates of one or more regions of spherical-coordinate
geometry, bounded by four great circles, corresponding to viewports recommended for display when the user does not
have control of the viewing orientation or has released control of the viewing orientation.


When an effectively applicable frame packing arrangement SEI message, as specified in clause D.2.35.1 or D.2.35.2, that
applies to the picture is present, the information indicated by the omnidirectional viewport SEI message applies to both
views.


**omni_viewport_id** contains an identifying number that may be used to identify the purpose of the one or more
recommended viewport regions.


omni_viewport_id equal to 0 indicates that the recommended viewports are per "director’s cut", i.e., a viewport suggested
according to the creative intent of the content author or content provider. omni_viewport_id equal to 1 indicates that the
recommended viewports are selected based on measurements of viewing statistics.


Values of omni_viewport_id from 2 to 511, inclusive, may be used as determined by the application. Values of
omni_viewport_id from 512 to 1023 are reserved for future use by ITU-T | ISO/IEC. Decoders encountering a value of
omni_viewport_id in the range of 512 to 1023, inclusive, shall ignore it.


**omni_viewport_cancel_flag** equal to 1 indicates that the SEI message cancels the persistence of any previous
omnidirectional viewport SEI message in output order. omni_viewport_cancel_flag equal to 0 indicates that
omnidirectional viewport information follows.


**omni_viewport_persistence_flag** specifies the persistence of the omnidirectional viewport SEI message.


omni_viewport_persistence_flag equal to 0 specifies that the omnidirectional viewport SEI message applies to the current
decoded picture only.


Let picA be the current picture. omni_viewport_persistence_flag equal to 1 specifies that the omnidirectional viewport SEI
message persists in output order until one or more of the following conditions are true:


- A new coded video sequence begins.


- The bitstream ends.


- A picture picB in an access unit containing an omnidirectional viewport SEI message is output having
PicOrderCnt( picB ) greater than PicOrderCnt( picA ).


When an equirectangular projection SEI message with erp_cancel_flag equal to 0 or a cubemap projection SEI message
with cmp_cancel_flag equal to 0 is not present in the coded video sequence that applies to the current picture and precedes
the omnidirectional viewport SEI message in decoding order, an omnidirectional viewport SEI message with
omni_viewport_cancel_flag equal to 0 shall not be present in the coded video sequence that applies to the current picture.
Decoders shall ignore omnidirectional viewport SEI messages with omni_viewport_cancel_flag equal to 0 that do not
follow, in decoding order, an equirectangular projection SEI message with erp_cancel_flag equal to 0 or a cubemap
projection SEI message with cmp_cancel_flag equal to 0 in the coded video sequence that applies to the current picture.


**omni_viewport_cnt_minus1** plus 1 specifies the number of recommended viewport regions that are indicated by the SEI
message.


When omni_viewport_cnt_minus1 is greater than 0 and there is no information provided by external means not specified
in this Specification on which recommended viewport is suggested to be displayed, the following applies:


  - When omni_viewport_id is equal to 0 or 1, the 0-th recommended viewport is suggested to be displayed when
the user does not have control of the viewing orientation or has released control of the viewing orientation.


  - When omni_viewport_id is equal to 0, between any two recommended viewports per director's cut, the i-th
recommended viewport has higher priority than the j-th recommended viewport for any values of i and j when i
is less than j. The 0-th recommended viewport per director's cut has the highest priority.





  - When omni_viewport_id is equal to 1, between any two recommended viewports, the i-th recommended viewport
has higher popularity, among some selection of candidate viewports, than the j-th recommended viewport for any
values of i and j when i is less than j. The 0-th most-viewed recommended viewport has the highest popularity.
The selection of the candidate viewports is outside the scope of this Specification.


**omni_viewport_azimuth_centre[** i **]** and **omni_viewport_elevation_centre[** i **]** indicate the centre of the i-th
recommended viewport region, in units of 2 [−16] degrees relative to the global coordinate axes. The value of
omni_viewport_azimuth_centre[ i ] shall be in the range of −180 * 2 [16] (i.e., −11 796 480) to 180 * 2 [16] - 1
(i.e., 11 796 479), inclusive. The value of omni_viewport_elevation_centre[ i ] shall be in the range of −90 * 2 [16]
(i.e., −5 898 240) to 90 * 2 [16] (i.e., 5 898 240), inclusive.


**omni_viewport_tilt_centre[** i **]** indicates the tilt angle of the i-th recommended viewport region, in units of 2 [−16] degrees.
The value of omni_viewport_tilt_centre[ i ] shall be in the range of −180 * 2 [16] (i.e., −11 796 480) to 2 [16] - 1
(i.e., 11 796 479), inclusive.


**omni_viewport_hor_range[** i **]** indicates the azimuth range of the i-th recommended viewport region, in units of 2 [−16]
degrees. The value of omni_viewport_hor_range[ i ] shall be in the range of 1 to 360 * 2 [16] (i.e., 23 592 960), inclusive.


**omni_viewport_ver_range[** i **]** indicates the elevation range of the i-th recommended viewport region, in units of 2 [−16 ]
degrees. The value of omni_viewport_ver_range[ i ] shall be in the range of 1 to 180 * 2 [16] (i.e., 11 796 480), inclusive.


**D.2.35.6** **Sample location remapping process**


**D.2.35.6.1General**


To remap colour sample locations of a region-wise packed picture to a unit sphere, the following ordered steps are applied:


- A region-wise packed picture is obtained as the cropped decoded picture by decoding a coded picture. For purposes
of interpretation of chroma samples, the input to the indicated remapping process is the set of decoded sample values
after applying an (unspecified) upsampling conversion process to the 4:4:4 colour sampling format as necessary when
chroma_format_idc is equal to 1 (4:2:0 chroma format) or 2 (4:2:2 chroma format). This (unspecified) upsampling
process should account for the relative positioning relationship between the luma and chroma samples as indicated
by chroma_sample_loc_type_top_field and chroma_sample_loc_type_bottom_field, when present.


- If region-wise packing is indicated, the sample locations of the region-wise packed picture are converted to sample
locations of the respective projected picture as specified in clause D.2.35.6.4. Otherwise, the projected picture is
identical to the region-wise packed picture.


- If frame packing is indicated, the sample locations of the projected picture are converted to sample locations of the
respective constituent picture of the projected picture, as specified in clause D.2.35.6.6. Otherwise, the constituent
picture of the projected picture is identical to the projected picture.


- The sample locations of a constituent picture of the projected picture are converted to sphere coordinates relative to
the local coordinate axes, as specified in clause D.2.35.6.2.


- If rotation is indicated, the sphere coordinates relative to the local coordinate axes are converted to sphere coordinates
relative to the global coordinate axes, as specified in clause D.2.35.6.3. Otherwise, the global coordinate axes are
identical to the local coordinate axes.


The overall process for mapping of luma sample locations within a region-wise packed picture to sphere coordinates
relative to the global coordinate axes is normatively specified in clause D.2.35.6.5.


For each region-wise packed picture corresponding to a decoded picture, the following applies:


- When an equirectangular projection SEI message with erp_cancel_flag equal to 0 that applies to the picture is present,
ErpFlag is set equal to 1, and CmpFlag is set equal to 0.


- When a cubemap projection SEI message with cmp_cancel_flag equal to 0 that applies to the picture is present,
CmpFlag is set equal to 1, and ErpFlag is set equal to 0.


- If a sphere rotation SEI message with sphere_rotation_cancel_flag equal to 0 that applies to the picture is present,
RotationFlag is set equal to 1, and RotationYaw, RotationPitch, and RotationRoll are set equal to yaw_rotation ÷ 2 [16],
pitch_rotation ÷ 2 [16], and roll_rotation ÷ 2 [16], respectively.


- Otherwise, RotationFlag is set equal to 0.


- If a frame packing arrangement SEI message with frame_packing_arrangement_cancel_flag equal to 0 that applies to
the picture is not present, StereoFlag, TopBottomFlag, and SideBySideFlag are all set equal to 0, HorDiv1 is set equal
to 1, and VerDiv1 is set equal to 1.


- Otherwise, the following applies:





   - StereoFlag is set equal to 1.


   - If the value of frame_packing_arrangement_type of the frame packing arrangement SEI message is equal to 3,

TopBottomFlag is set equal to 0, SideBySideFlag is set equal to 1, HorDiv1 is set equal to 2 and VerDiv1 is set
equal to 1.


   - Otherwise, if the value of frame_packing_arrangement_type of the frame packing arrangement SEI message is

equal to 4, TopBottomFlag is set equal to 1, SideBySideFlag is set equal to 0, HorDiv1 is set equal to 1, and
VerDiv1 is set equal to 2.


   - Otherwise, TopBottomFlag is set equal to 0, SideBySideFlag is set equal to 0, HorDiv1 is set equal to 1, and

VerDiv1 is set equal to 1.


- If a region-wise packing SEI message with rwp_cancel_flag equal to 0 that applies to the picture is not present,
RegionWisePackingFlag is set equal to 0, and ConstituentPicWidth and ConstituentPicHeight are set to be equal to
cropPicWidth / HorDiv1 and cropPicHeight / VerDiv1, respectively, where cropPicWidth and cropPicHeight are the
width and height, respectively, of the cropped decoded picture.


- Otherwise, RegionWisePackingFlag is set equal to 1, and ConstituentPicWidth and ConstituentPicHeight are set equal
to proj_picture_width / HorDiv1 and proj_picture_height / VerDiv1, respectively.


**D.2.35.6.2Projection for a sample location**


Inputs to this clause are:


- pictureWidth and pictureHeight, which are the width and height, respectively, of a monoscopic projected luma picture,
in relative projected picture sample units, and


- the centre point of a sample location (hPos, vPos) along the horizontal and vertical axes, respectively, in relative
projected picture sample units, where hPos and vPos may have non-integer real values.


Outputs of this clause are:


- sphere coordinates (ϕ, θ) for the sample location in degrees relative to the coordinate axes specified in clause
D.2.35.6.1.


The projection for a sample location is derived as follows:


- If ErpFlag is equal to 1, the following applies:


   - If RegionWisePackingFlag is equal to 0 and erp_padding_flag is equal to 1, the following applies:


hPos′ = hPos − left_gb_erp_width (D-45)
pictureWidth = pictureWidth − left_gb_erp_width − right_gb_erp_width


   - Otherwise, the following applies:


hPos′ = hPos (D-46)


   - The following applies:


ϕ = 180 − hPos′ * ( 360 ÷ pictureWidth ) (D-47)
θ = 90 − vPos * ( 180 ÷ pictureHeight )


- Otherwise (CmpFlag is equal to 1), it is a requirement of bitstream conformance that pictureWidth shall be a multiple
of 3 and pictureHeight shall be a multiple of 2, and that pictureWidth / 3 shall be equal to pictureHeight / 2, and the
following applies:


lw = pictureWidth / 3
lh = pictureHeight / 2
w = Floor( hPos ÷ lw )
h = Floor( vPos ÷ lh )
tmpHorVal = hPos − w * lw
tmpVerVal = vPos − h * lh
hPos′ = −( 2 * tmpHorVal ÷ lw ) + 1
vPos′ = −( 2 * tmpVerVal ÷ lh ) + 1
if( w = = 1 && h = = 0 ) { /* positive x front face */
x = 1.0
y = hPos′
z = vPos′
} else if( w = = 1 && h = = 1 ) { /* negative x back face */
x = −1.0





y = −vPos′
z = −hPos′
} else if( w = = 2 && h = = 1 ) { /* positive z top face */ (D-48)
x = −hPos′
y = −vPos′
z = 1.0
} else if( w = = 0 && h = = 1 ) { /* negative z bottom face */
x = hPos′
y = −vPos′
z = −1.0
} else if( w = = 0 && h = = 0 ) { /* positive y left face */
x = −hPos′
y = 1.0
z = vPos′
} else { /* ( w = = 2 && h = = 0 ), negative y right face */
x = hPos′
y = −1.0
z = vPos′
}
ϕ = Atan2( y, x ) * 180 ÷ π
θ = Asin( z ÷ Sqrt( x [2] + y [2] + z [2] ) ) * 180 ÷ π


**D.2.35.6.3Conversion from the local coordinate axes to the global coordinate axes**


Inputs to this clause are:


- rotation_yaw (αd), rotation_pitch (βd), rotation_roll (γd), all in units of degrees, and


- sphere coordinates (ϕd, θd) relative to the local coordinate axes.


Outputs of this clause are:


- sphere coordinates (ϕ′, θ′) relative to the global coordinate axes.


The outputs are derived as follows:


ϕ = ϕd * π ÷ 180
θ = θd * π ÷ 180
α = αd * π ÷ 180
β = βd * π ÷ 180
γ = γd * π ÷ 180
x1 = Cos( ϕ ) * Cos( θ )
y1 = Sin( ϕ ) * Cos( θ )
z1 = Sin( θ )
x2 = Cos( β ) * Cos ( γ ) * x1 − Cos( β ) * Sin( γ ) * y1 + Sin( β ) * z1 (D-49)
y2 = ( Cos( α ) * Sin( γ ) + Sin( α ) * Sin( β ) * Cos( γ ) ) * x1 +
( Cos( α ) * Cos( γ ) − Sin( α ) * Sin( β ) * Sin( γ ) ) * y1 −
Sin( α ) * Cos( β ) * z1
z2 = ( Sin( α ) * Sin( γ ) − Cos( α ) * Sin( β ) * Cos( γ ) ) * x1 +
( Sin( α ) * Cos( γ ) + Cos( α ) * Sin( β ) * Sin( γ ) ) * y1 +
Cos( α ) * Cos( β ) * z1
ϕ′ = Atan2( y2, x2 ) * 180 ÷ π
θ′ = Asin( z2 ) * 180 ÷ π


**D.2.35.6.4Conversion of sample locations for rectangular region-wise packing**


Inputs to this clause are:


- sample location (x, y) within the packed region, where x and y are in relative packed picture sample units, while the
sample location is at an integer sample location within the packed picture,


- the width and the height (projRegWidth, projRegHeight) of the projected region, in relative projected picture sample
units,


- the width and the height (packedRegWidth, packedRegHeight) of the packed region, in relative packed picture sample
units,


- transform type (transformType), and





- offset values for the sampling position (offsetX, offsetY) in the range of 0, inclusive, to 1, exclusive, in horizontal
and vertical relative packed picture sample units, respectively.


NOTE: offsetX and offsetY both equal to 0.5 indicate a sampling position that is in the centre point of a sample in packed
picture sample units.


Outputs of this clause are:


- the centre point of the sample location (hPos, vPos) within the projected region in relative projected picture sample
units, where hPos and vPos may have non-integer real values.


The outputs are derived as follows:


if( transformType = = 0 | | transformType = = 1 | | transformType = = 2 | | transformType = = 3 ) {
horRatio = projRegWidth ÷ packedRegWidth
verRatio = projRegHeight ÷ packedRegHeight
} else if( transformType = = 4 | | transformType = = 5 | | transformType = = 6 | |
transformType = = 7 ) {
horRatio = projRegWidth ÷ packedRegHeight
verRatio = projRegHeight ÷ packedRegWidth
}
if( transformType = = 0 ) {
hPos = horRatio * ( x + offsetX )
vPos = verRatio * ( y + offsetY )
} else if( transformType = = 1 ) {
hPos = horRatio * ( packedRegWidth − x − offsetX )
vPos = verRatio * ( y + offsetY )
} else if( transformType = = 2 ) {
hPos = horRatio * ( packedRegWidth − x − offsetX )
vPos = verRatio * ( packedRegHeight − y − offsetY ) (D-50)
} else if( transformType = = 3 ) {
hPos = horRatio * ( x + offsetX )
vPos = verRatio * ( packedRegHeight − y − offsetY )
} else if( transformType = = 4 ) {
hPos = horRatio * ( y + offsetY )
vPos = verRatio * ( x + offsetX )
} else if( transformType = = 5 ) {
hPos = horRatio * ( y + offsetY )
vPos = verRatio * ( packedRegWidth − x − offsetX )
} else if( transformType = = 6 ) {
hPos = horRatio * ( packedRegHeight − y − offsetY )
vPos = verRatio * ( packedRegWidth − x − offsetX )
} else if( transformType = = 7 ) {
hPos = horRatio * ( packedRegHeight − y − offsetY )
vPos = verRatio * ( x+ offsetX )
}


**D.2.35.6.5Mapping of luma sample locations within a cropped decoded picture to sphere coordinates relative to the**

**global coordinate axes**


This clause specifies the semantics of luma sample locations within a cropped decoded picture to sphere coordinates
relative to the global coordinate axes.


offsetX is set equal to 0.5 and offsetY is set equal to 0.5.


If RegionWisePackingFlag is equal to 1, the following applies for each packed region n in the range of 0 to
NumPackedRegions − 1, inclusive:


- For each sample location (xPackedPicture, yPackedPicture) belonging to the n-th packed region, the following
applies:


   - The corresponding sample location (xProjPicture, yProjPicture) of the projected picture is derived as follows:


      - x is set equal to xPackedPicture − PackedRegionLeft[ n ].


      - y is set equal to yPackedPicture − PackedRegionTop[ n ].





      - Clause D.2.35.6.4 is invoked with x, y, PackedRegionWidth[ n ], PackedRegionHeight[ n ],
ProjRegionWidth[ n ], ProjRegionHeight[ n ], TransformType[ n ], offsetX, and offsetY as inputs, and the
output is assigned to sample location (hPos, vPos).


      - xProjPicture is set equal to ProjRegionLeft[ n ] + hPos.


      - When StereoFlag is equal to 0 or TopBottomFlag is equal to 1, and when xProjPicture is greater than or
equal to proj_picture_width, xProjPicture is set equal to xProjPicture − proj_picture_width.


      - When SideBySideFlag is equal to 1, the following applies:


        - When ProjRegionLeft[ n ] is less than proj_picture_width / 2 and xProjPicture is greater than or equal
to proj_picture_width / 2, xProjPicture is set equal to xProjPicture − proj_picture_width / 2.


        - When ProjRegionLeft[ n ] is greater than or equal to proj_picture_width / 2 and xProjPicture is greater
than or equal to proj_picture_width, xProjPicture is set equal to xProjPicture − proj_picture_width / 2.


      - yProjPicture is set equal to ProjRegionTop[ n ] + vPos.


   - Clause D.2.35.6.6 is invoked with xProjPicture, yProjPicture, ConstituentPicWidth, and ConstituentPicHeight

as inputs, and the outputs indicating the sphere coordinates and the constituent picture index (for frame-packed
stereoscopic video) for the luma sample location (xPackedPicture, yPackedPicture) belonging to the n-th packed
region in the decoded picture.


Otherwise (RegionWisePackingFlag is equal 0), the following applies for each sample location (x, y) that is not an
equirectangular projection padded sample within the cropped decoded picture, where a sample location (x, y) is an
equirectangular projection padded sample when and only when ErpFlag is equal to 1, x is in the range of 0 to
left_gb_erp_width − 1, inclusive, or ConstituentPicWidth − right_gb_erp_width to ConstituentPicWidth − 1, inclusive,
and y is in the range of 0 to ConstituentPicHeight − 1, inclusive:


- xProjPicture is set equal to x + offsetX.


- yProjPicture is set equal to y + offsetY.


- If ErpFlag is equal to 0, projPicWidth is set equal to ConstituentPicWidth. Otherwise (ErpFlag is equal to 1),
projPicWidth is set equal to ConstituentPicWidth − ( left_gb_erp_width + right_gb_erp_width ).


- Clause D.2.35.6.6 is invoked with xProjPicture, yProjPicture, projPicWidth, and ConstituentPicHeight as inputs, and
the outputs indicating the sphere coordinates and the constituent picture index (for frame-packed stereoscopic video)
for the sample location (x, y) within the region-wise packed picture.


**D.2.35.6.6Conversion from a sample location in a projected picture to sphere coordinates relative to the global**

**coordinate axes**


Inputs to this clause are:


- the centre point of a sample location (xProjPicture, yProjPicture) within a projected picture, where xProjPicture and
yProjPicture are in relative projected picture sample units and may have non-integer real values, and


- pictureWidth and pictureHeight, which are the width and height, respectively, of a monoscopic projected luma picture,
in relative projected picture sample units.


Outputs of this clause are:


- sphere coordinates (azimuthGlobal, elevationGlobal), in units of degrees relative to the global coordinate axes, and


- when StereoFlag is equal to 1, the index of the constituent picture (constituentPicture) equal to 0 or 1.


The outputs are derived with the following ordered steps:


- If xProjPicture is greater than or equal to pictureWidth or yProjPicture is greater than or equal to pictureHeight, the
following applies:


   - constituentPicture is set equal to 1.


   - If xProjPicture is greater than or equal to pictureWidth, xProjPicture is set to xProjPicture − pictureWidth.


   - If yProjPicture is greater than or equal to pictureHeight, yProjPicture is set to yProjPicture − pictureHeight.


- Otherwise, constituentPicture is set equal to 0.


- Clause D.2.35.6.2 is invoked with pictureWidth, pictureHeight, xProjPicture, and yProjPicture as inputs, and the
output is assigned to azimuthLocal, elevationLocal.





- If RotationFlag is equal to 1, clause D.2.35.6.3 is invoked with azimuthLocal, elevationLocal, RotationYaw,
RotationPitch, and RotationRoll as inputs, and the output is assigned to azimuthGlobal and elevationGlobal.


- Otherwise, azimuthGlobal is set equal to azimuthLocal and elevationGlobal is set equal to elevationLocal.


**D.2.36** **SEI manifest SEI message semantics**


The SEI manifest SEI message conveys information on SEI messages that are indicated as expected (i.e., likely) to be
present or not present. Such information may include:


1) The indication that certain types of SEI messages are expected (i.e., likely) to be present (although not guaranteed

to be present) in the coded video sequence.


2) For each type of SEI message that is indicated as expected (i.e., likely) to be present in the coded video sequence,

the degree of expressed necessity of interpretation of the SEI messages of this type.


The degree of necessity of interpretation of an SEI message type may be indicated as "necessary", "unnecessary",
or "undetermined".


An SEI message is indicated by the encoder (i.e., the content producer) as being "necessary" when the information
conveyed by the SEI message is considered as necessary for interpretation by the decoder or receiving system in
order to properly process the content and enable an adequate user experience; it does not mean that the bitstream
is required to contain the SEI message in order to be a conforming bitstream. It is at the discretion of the encoder
to determine which SEI messages are to be considered as necessary in a particular coded video sequence.
However, it is suggested that some SEI messages, such as the frame packing arrangement, segmented rectangular
frame packing arrangement, and omnidirectional projection indication SEI messages, should typically be
considered as necessary.


3) The indication that certain types of SEI messages are expected (i.e., likely) not to be present (although not

guaranteed not to be present) in the coded video sequence.

NOTE – An example of such a usage of an SEI manifest SEI message is to express the expectation that there are no frame
packing arrangement SEI messages, segmented rectangular frame packing arrangement SEI messages, display orientation
SEI messages, or omnidirectional projection indication SEI messages in the coded video sequence, and therefore that the
rendering of the decoded video pictures for display purposes would not need any of the additional post-processing that is
commonly associated with the interpretation of these SEI messages.


The content of an SEI manifest SEI message may, for example, be used by transport-layer or systems-layer processing
elements to determine whether the coded video sequence is suitable for delivery to a receiving and decoding system, based
on whether the receiving system can properly process the coded video sequence to enable an adequate user experience or
whether the coded video sequence satisfies the application needs.


When an SEI manifest SEI message is present in any access unit of a coded video sequence, an SEI manifest SEI message
shall be present in the first access unit of the coded video sequence. The SEI manifest SEI message persists in decoding
order from the current access unit until the end of the coded video sequence. When there are multiple SEI manifest SEI
messages present in a coded video sequence, they shall have the same content.


An SEI NAL unit containing an SEI manifest SEI message shall not contain any other SEI messages other than SEI prefix
indication SEI messages. When present in an SEI NAL unit, the SEI manifest SEI message shall be the first SEI message
in the SEI NAL unit.


**manifest_num_sei_msg_types** specifies the number of types of SEI messages for which information is provided in the
SEI manifest SEI message.


**manifest_sei_payload_type[** i **]** indicates the payloadType value of the i-th type of SEI message for which information is
provided in the SEI manifest SEI message. The values of manifest_sei_payload_type[ m ] and
manifest_sei_payload_type[ n ] shall not be identical when m is not equal to n.


**manifest_sei_description[** i **]** provides information on SEI messages with payloadType equal to
manifest_sei_payload_type[ i ] as specified in Table D-12.





**Table D-12– manifest_sei_description[ i ] values**







|Value|Description|
|---|---|
|0|Indicates that there is no SEI message with payloadType equal to<br>manifest_sei_payload_type[ i ] expected to be present in the coded video<br>sequence.|
|1|Indicates that there are SEI messages with payloadType equal to<br>manifest_sei_payload_type[ i ] expected to be present in the coded video<br>sequence, and these SEI messages are considered as necessary.|
|2|Indicates that there are SEI messages with payloadType equal to<br>manifest_sei_payload_type[ i ] expected to be present in the coded video<br>sequence, and these SEI messages are considered as unnecessary.|
|3|Indicates that there are SEI messages with payloadType equal to<br>manifest_sei_payload_type[ i ] expected to be present in the coded video<br>sequence, and the necessity of these SEI messages is undetermined.|
|4-255|Reserved|


The value of manifest_sei_description[ i ] shall be in the range of 0 to 3, inclusive, in bitstreams conforming to this version
of this Specification. Other values for manifest_sei_description[ i ] are reserved for future use by ITU-T | ISO/IEC.
Decoders shall allow the value of manifest_sei_description[ i ] greater than or equal to 4 to appear in the syntax and shall
ignore all information for payloadType equal to manifest_sei_payload_type[ i ] signalled in the SEI manifest SEI message
and shall ignore all SEI prefix indication SEI messages with prefix_sei_payload_type equal to
manifest_sei_payload_type[ i ] when manifest_sei_description[ i ] is greater than or equal to 4.


**D.2.37** **SEI prefix indication SEI message semantics**


The SEI prefix indication SEI message carries one or more SEI prefix indications for SEI messages of a particular value
of payloadType. Each SEI prefix indication is a bit string that follows the SEI payload syntax of that value of payloadType
and contains a number of complete syntax elements starting from the first syntax element in the SEI payload.


Each SEI prefix indication for an SEI message of a particular value of payloadType indicates that one or more SEI messages
of this value of payloadType are expected (i.e., likely) to be present in the coded video sequence and to start with the
provided bit string. A starting bit string would typically contain only a true subset of an SEI payload of the type of SEI
message indicated by the payloadType, may contain a complete SEI payload, and shall not contain more than a complete
SEI payload. It is not prohibited for SEI messages of the indicated value of payloadType to be present that do not start
with any of the indicated bit strings.


These SEI prefix indications should provide sufficient information for indicating what type of processing is needed or what
type of content is included. The former (type of processing) indicates decoder-side processing capability, e.g., whether
some type of frame unpacking is needed. The latter (type of content) indicates, for example, whether the bitstream contains
subtitle captions in a particular language.


The content of an SEI prefix indication SEI message may, for example, be used by transport-layer or systems-layer
processing elements to determine whether the coded video sequence is suitable for delivery to a receiving and decoding
system, based on whether the receiving system can properly process the coded video sequence to enable an adequate user
experience or whether the coded video sequence satisfies the application needs (as determined in some manner by external
means outside the scope of this Specification).


In one example, when the payloadType indicates the frame packing arrangement SEI message, an SEI prefix indication
should include up to at least the syntax element frame_packing_arrangement_type; and when the payloadType indicates
the omnidirectional projection indication SEI message, an SEI prefix indication should include up to at least the syntax
element projection_type.


In another example, for user data registered SEI messages that are used to carry captioning information, an SEI prefix
indication should include up to at least the language code; and for user data unregistered SEI messages extended for private
use, an SEI prefix indication should include up to at least the UUID.


When an SEI prefix indication SEI message is present in any access unit of a coded video sequence, an SEI prefix indication
SEI message shall be present in the first access unit of the coded video sequence. The SEI prefix indication SEI message
persists in decoding order from the current access unit until the end of the coded video sequence. When there are multiple





SEI prefix indication SEI messages present in a coded video sequence for a particular value of payloadType, they shall
have the same content.


An SEI NAL unit containing an SEI prefix indication SEI message for a particular value of payloadType shall not contain
any other SEI messages other than an SEI manifest SEI message and SEI prefix indication SEI messages for other values
of payloadType.


**prefix_sei_payload_type** indicates the payloadType value of the SEI messages for which one or more SEI prefix
indications are provided in the SEI prefix indication SEI message. When an SEI manifest SEI message is also present for
the coded video sequence, the value of prefix_sei_payload_type shall be equal to one of the
manifest_sei_payload_type[ m ] values for which manifest_sei_description[ m ] is equal to 1 to 3, inclusive, as indicated
by an SEI manifest SEI message that applies to the coded video sequence.


**num_sei_prefix_indications_minus1** plus 1 specifies the number of SEI prefix indications.


**num_bits_in_prefix_indication_minus1[** i **]** plus 1 specifies the number of bits in the i-th SEI prefix indication.


**sei_prefix_data_bit[** i **][** j **]** specifies the j-th bit of the i-th SEI prefix indication.


The bits sei_prefix_data_bit[ i ][ j ] for j ranging from 0 to num_bits_in_prefix_indication_minus1[ i ], inclusive, follow
the syntax of the SEI payload with payloadType equal to prefix_sei_payload_type, and contain a number of complete
syntax elements starting from the first syntax element in the SEI payload syntax, and may or may not contain all the syntax
elements in the SEI payload syntax. The last bit of these bits (i.e., the bit
sei_prefix_data_bit[ i ][ num_bits_in_prefix_indication_minus1[ i ] ]) shall be the last bit of a syntax element in the SEI
payload syntax, unless it is a bit within an itu_t_t35_payload_byte or user_data_payload_byte.

NOTE – The exception for itu_t_t35_payload_byte and user_data_payload_byte is provided because these syntax elements may
contain externally-specified syntax elements, and the determination of the boundaries of such externally-specified syntax elements
is a matter outside the scope of this Specification.


**D.2.38** **Annotated regions SEI message semantics**


The annotated regions SEI message carries parameters that identify annotated regions using bounding boxes representing
the size and location of identified objects.


The following variables are specified for the use of this SEI message:


- CroppedWidth and CroppedHeight, set as specified by Equations D-30 and D-31, respectively.


- A conformance cropping window left offset, ConfWinLeftOffset, set equal to CropUnitX * frame_crop_left_offset.


- A conformance cropping window top offset, ConfWinTopOffset, set equal to CropUnitY * frame_crop_top_offset.


**ar_cancel_flag** equal to 1 indicates that the SEI message cancels the persistence of any previous annotated regions SEI
message that is associated with one or more layers to which the annotated regions SEI message applies. ar_cancel_flag
equal to 0 indicates that annotated regions information follows.


When ar_cancel_flag equal to 1 or a new coded video sequence of the current layer begins, the variables LabelAssigned[ i ],
ObjectTracked[ i ], and ObjectBoundingBoxAvail are set equal to 0 for i in the range of 0 to 255, inclusive.


**ar_not_optimized_for_viewing_flag** equal to 1 indicates that the decoded pictures that the annotated regions SEI message
applies to are not optimized for user viewing, but rather are optimized for some other purpose such as algorithmic object
classification performance. ar_not_optimized_for_viewing_flag equal to 0 indicates that the decoded pictures that the
annotated regions SEI message applies to may or may not be optimized for user viewing.


**ar_true_motion_flag** equal to 1 indicates that the motion information in the coded pictures that the annotated regions SEI
message applies to was selected with a goal of accurately representing object motion for objects in the annotated regions.
ar_true_motion_flag equal to 0 indicates that the motion information in the coded pictures that the annotated regions SEI
message applies to may or may not be selected with a goal of accurately representing object motion for objects in the
annotated regions.


**ar_occluded_object_flag** equal to 1 indicates that the ar_bounding_box_top[ ar_object_idx[ i ] ], ar_bounding_box_
left[ ar_object_idx[ i ] ], ar_bounding_box_width[ ar_object_idx[ i ] ], and ar_bounding_box_height[ ar_object_idx[ i ] ]
syntax elements represent the size and location of an object or a portion of an object that may not be visible or may be only
partially visible within the cropped decoded picture. ar_occluded_object_flag equal to 0 indicates that the
ar_bounding_box_top[ ar_object_idx[ i ] ], ar_bounding_box_left[ ar_object_idx[ i ] ], ar_bounding_box_
width[ ar_object_idx[ i ] ], and ar_bounding_box_height[ ar_object_idx[ i ] ] syntax elements represent the size and
location of an object that is entirely visible within the cropped decoded picture. It is a requirement of bitstream conformance
that the value of ar_occluded_object_flag shall be the same for all annotated_regions( ) syntax structures within a coded
video sequence.





**ar_partial_object_flag_present_flag** equal to 1 indicates that ar_partial_object_flag[ ar_object_idx[ i ] ] syntax elements
are present. ar_partial_object_flag_present_flag equal to 0 indicates that ar_partial_object_flag[ ar_object_idx[ i ] ] syntax
elements are not present. It is a requirement of bitstream conformance that the value of ar_partial_object_flag_present_flag
shall be the same for all annotated_regions( ) syntax structures within a coded video sequence.


**ar_object_label_present_flag** equal to 1 indicates that label information corresponding to objects in the annotated regions
is present. ar_object_label_present_flag equal to 0 indicates that label information corresponding to the objects in the
annotated regions is not present.


**ar_object_confidence_info_present_flag** equal to 1 indicates that ar_object_confidence[ ar_object_idx[ i ] ] syntax
elements are present. ar_object_confidence_info_present_flag equal to 0 indicates that ar_object_
confidence[ ar_object_idx[ i ] ] syntax elements are not present. It is a requirement of bitstream conformance that the value
of ar_object_confidence_present_flag shall be the same for all annotated_regions( ) syntax structures within a coded video
sequence.


**ar_object_confidence_length_minus1** plus 1 specifies the length, in bits, of the ar_object_confidence[ ar_object_idx[ i ] ]
syntax elements. It is a requirement of bitstream conformance that the value of ar_object_confidence_length_minus1 shall
be the same for all annotated_regions( ) syntax structures within a coded video sequence.


**ar_object_label_language_present_flag** equal to 1 indicates that the ar_object_label_language syntax element is present.
ar_object_label_language_present_flag equal to 0 indicates that the ar_object_label_language syntax element is not
present.


**ar_bit_equal_to_zero** shall be equal to zero.


**ar_object_label_language** contains a language tag as specified by IETF RFC 5646 followed by a null termination byte
equal to 0x00. The length of the ar_object_label_language syntax element shall be less than or equal to 255 bytes, not
including the null termination byte. When not present, the language of the label is unspecified.


**ar_num_label_updates** indicates the total number of labels associated with the annotated regions that are signalled. The
value of ar_num_label_updates shall be in the range of 0 to 255, inclusive.


**ar_label_idx** [ i ] indicates the index of the signalled label. The value of ar_label_idx[ i ] shall be in the range of 0 to 255,
inclusive.


**ar_label_cancel_flag** equal to 1 cancels the persistence scope of the ar_label_idx[ i ]-th label. ar_label_cancel_flag equal
to 0 indicates that the ar_label_idx[ i ]-th label is assigned a signalled value.


LabelAssigned[ ar_label_idx[ i ] ] equal to 1 indicates that the ar_label_idx[ i ]-th label is assigned.
LabelAssigned[ ar_label_idx[ i ] ] equal to 0 indicates that the ar_label_idx[ i ]-th label is not assigned.


**ar_label** [ ar_label_idx[ i ] ] specifies the contents of the ar_label_idx[ i ] –th label. The length of the
ar_label[ ar_label_idx[ i ] ] syntax element shall be less than or equal to 255 bytes, not including the null termination byte.


**ar_num_object_updates** indicates the number of object updates to be signalled. ar_num_object_updates shall be in the
range of 0 to 255, inclusive.


**ar_object_idx** [ i ] is the index of the object parameters to be signalled. ar_object_idx[ i ] shall be in the range of 0 to 255,
inclusive.


**ar_object_cancel_flag** equal to 1 cancels the persistence scope of the ar_object_idx[ i ]-th object. ar_object_cancel_flag
equal to 0 indicates that parameters associated with the ar_object_idx[ i ]-th object tracked object are signalled.


ObjectTracked[ ar_object_idx[ i ] ] equal to 1 indicates that the object_idx[ i ]-th object is tracked.
ObjectTracked[ ar_object_idx[ i ] ] equal to 0 indicates that the object_idx[ i ]-th object is not tracked.


**ar_object_label_update_flag** equal to 1 indicates that an object label is signalled. ar_object_label_update_flag equal to 0
indicates that an object label is not signalled.


**ar_object_label_idx** [ ar_object_idx[ i ] ] indicates the index of the label corresponding to the ar_object_idx[ i ]-th object.
When ar_object_label_idx[ ar_object_idx[ i ] ] is not present, its value is inferred from a previous annotated regions SEI
message in output order in the same coded video sequence, if any. The value of ar_object_label_idx[ ar_object_idx[ i ] ]
shall be in the range of 0 to 255, inclusive.


**ar_bounding_box_update_flag** equal to 1 indicates that object bounding box parameters are signalled.
ar_bounding_box_update_flag equal to 0 indicates that object bounding box parameters are not signalled.


**ar_bounding_box_cancel_flag** equal to 1 cancels the persistence scope of the ar_bounding_box_top[ ar_object_idx[ i ] ],
ar_bounding_box_left[ ar_object_idx[ i ] ], ar_bounding_box_width[ ar_object_idx[ i ] ], ar_bounding_box_
height[ ar_object_idx[ i ] ]. ar_partial_object_flag[ ar_object_idx[ i ] ], and ar_object_confidence[ ar_object_idx[ i ] ].
ar_bounding_box_cancel_flag equal to 0 indicates that ar_bounding_box_top[ ar_object_idx[ i ] ], ar_bounding_box_





left[ ar_object_idx[ i ] ], ar_bounding_box_width[ ar_object_idx[ i ] ] ar_bounding_box_height[ ar_object_idx[ i ] ]
ar_partial_object_flag[ ar_object_idx[ i ] ], and ar_object_confidence[ ar_object_idx[ i ] ] syntax elements are signalled.


ObjectBoundingBoxAvail[ ar_object_idx[ i ] ] equal to 1 indicates that the bounding box information of the object_idx[ i ]th object is signalled. ObjectBoundingBoxAvail[ ar_object_idx[ i ] ] equal to 0 indicates that the bounding box information
of the object_idx[ i ]-th object is not signalled.


**ar_bounding_box_top** [ ar_object_idx[ i ] ], **ar_bounding_box_left** [ ar_object_idx[ i ] ], **ar_bounding_box_width** [ ar_
object_idx[ i ] ], and **ar_bounding_box_height** [ ar_object_idx[ i ] ] specify the coordinates of the top-left corner and the
width and height, respectively, of the bounding box of the ar_object_idx[ i ]-th object in the cropped decoded picture,
relative to the conformance cropping window specified by the active SPS.


The value of ar_bounding_box_left[ ar_object_idx[ i ] ] shall be in the range of 0 to CroppedWidth / SubWidthC − 1,
inclusive.


The value of ar_bounding_box_top[ ar_object_idx[ i ] ] shall be in the range of 0 to CroppedHeight / SubHeightC − 1,
inclusive.


The value of ar_bounding_box_width[ ar_object_idx[ i ] ] shall be in the range of 0 to CroppedWidth / SubWidthC −
ar_bounding_box_left[ ar_object_idx[ i ] ], inclusive.


The value of ar_bounding_box_height[ ar_object_idx[ i ] ] shall be in the range of 0 to CroppedHeight / SubHeightC −
ar_bounding_box_top[ ar_object_idx[ i ] ], inclusive.


The identified object rectangle contains the luma samples with horizontal picture coordinates from SubWidthC *
( ConfWinLeftOffset + ar_bounding_box_left[ ar_object_idx[ i ] ] ) to SubWidthC * ( ConfWinLeftOffset + ar_bounding
_box_left[ ar_object_idx[ i ] ] + ar_bounding_box_width[ ar_object_idx[ i ] ] ) − 1, inclusive, and vertical picture
coordinates from SubHeightC * ( ConfWinTopOffset + ar_bounding_box_top[ ar_object_idx[ i ] ] ) to SubHeightC *
( ConfWinTopOffset + ar_bounding_box_top[ ar_object_idx[ i ] ] + ar_bounding_box_height[ ar_object_idx[ i ] ] ) − 1,
inclusive.


When ChromaArrayType is not equal to 0, the corresponding specified samples of the two chroma arrays are the samples
having picture coordinates ( x / SubWidthC, y / SubHeightC ), where ( x, y ) are the picture coordinates of the specified
luma samples.


The values of ar_bounding_box_top[ ar_object_idx[ i ] ], ar_bounding_box_left[ ar_object_idx[ i ] ], ar_bounding_box_
width[ ar_object_idx[ i ] ] and ar_bounding_box_height[ ar_object_idx[ i ] ] persist in output order within the coded video
sequence for each value of ar_object_idx[ i ]. When not present, the values of ar_bounding_box_top[ ar_object_idx[ i ] ],
ar_bounding_box_left[ ar_object_idx[ i ] ], ar_bounding_box_width[ ar_object_idx[ i ] ] or ar_bounding_box_
height[ ar_object_idx[ i ] ] are inferred from a previous annotated regions SEI message in output order in the coded video
sequence, if any.


**ar_partial_object_flag** [ ar_object_idx[ i ] ] equal to 1 indicates that the ar_bounding_box_top[ ar_object_idx[ i ] ],
ar_bounding_box_left[ ar_object_idx[ i ] ], ar_bounding_box_width[ ar_object_idx[ i ] ] and ar_bounding_box_
height[ ar_object_idx[ i ] ] syntax elements represent the size and location of an object that is only partially visible within
the cropped decoded picture. ar_partial_object_flag[ ar_object_idx[ i ] ] equal to 0 indicates that the ar_bounding_box_
top[ ar_object_idx[ i ] ], ar_bounding_box_left[ ar_object_idx[ i ] ], ar_bounding_box_width[ ar_object_idx[ i ] ] and
ar_bounding_box_height[ ar_object_idx[ i ] ] syntax elements represent the size and location of an object that may or may
not be only partially visible within the cropped decoded picture. When not present, the value of
ar_partial_object_flag[ ar_object_idx[ i ] ] is inferred from a previous annotated regions SEI message in output order in
the coded video sequence, if any.


**ar_object_confidence** [ ar_object_idx[ i ] ] indicates the degree of confidence associated with the ar_object_idx[ i ]-th
object, in units of 2 [−( ar_object_confidence_length_minus1 + 1 )], such that a higher value of ar_object_confidence[ ar_object_idx[ i ] ]
indicates a higher degree of confidence. The length of the ar_object_confidence[ ar_object_idx[ i ] ] syntax element is
ar_object_confidence_length_minus1 + 1 bits. When not present, the value of_object_confidence[ ar_object_idx[ i ] ] is
inferred from a previous annotated regions SEI message in output order in the coded video sequence, if any.


**D.2.39** **Shutter interval information SEI message semantics**


The shutter interval information SEI message indicates the shutter interval for the associated video source pictures prior to
encoding, e.g., for camera-captured content, the shutter interval is the amount of time that an image sensor is exposed to
produce each source picture.


**sii_sub_layer_idx** specifies the shutter interval temporal sub-layer index of the current picture. The value of
sii_sub_layer_idx shall be equal to 0 when the current access unit is the first access unit of the coded video sequence. When
fixed_shutter_interval_within_cvs_flag is equal to 1, the value of sii_sub_layer_idx shall be equal to 0. Otherwise,
fixed_shutter_interval_within_cvs_flag is equal to 0, the value of sii_sub_layer_idx shall be less than or equal to the value
of sii_max_sub_layers_minus1.





**shutter_interval_info_present_flag** equal to 1 indicates that the syntax elements sii_time_scale,
fixed_shutter_interval_within_cvs_flag, and either sii_num_units_in_shutter_interval or sii_max_sub_layers_minus1 and
sub_layer_num_units_in_shutter_interval[ i ] are present. shutter_interval_info_present_flag equal to 0 indicates that the
syntax elements sii_time_scale, fixed_shutter_interval_within_cvs_flag, sii_num_units_in_shutter_interval,
sii_max_sub_layers_minus1, and sub_layer_num_units_in_shutter_interval[ i ] are not present. The value of
shutter_interval_info_present_flag shall be equal to 1 when the current access unit is the first access unit of the coded video
sequence. Otherwise, the current access unit is not the first access unit of the coded video sequence, the value of
shutter_interval_info_present_flag shall be equal to 0.


**sii_time_scale** specifies the number of time units that pass in one second. The value of sii_time_scale shall be greater
than 0. For example, a time coordinate system that measures time using a 27 MHz clock has an sii_time_scale of
27 000 000.


**fixed_shutter_interval_within_cvs_flag** equal to 1 specifies that the indicated shutter interval is the same for all pictures
in the coded video sequence. fixed_shutter_interval_within_cvs_flag equal to 0 specifies that the indicated shutter interval
may not be the same for all pictures in the coded video sequence.


**sii_num_units_in_shutter_interval**, when fixed_shutter_interval_within_cvs_flag is equal to 1, specifies the number of
time units of a clock operating at the frequency sii_time_scale Hz that corresponds to the indicated shutter interval of each
picture in the coded video sequence. The value 0 may be used to indicate that the associated video content contains screen
capture content, computer generated content, or other non-camera-captured content.


The indicated shutter interval, denoted by the variable shutterInterval, in units of seconds, is equal to the quotient of
sii_num_units_in_shutter_interval divided by sii_time_scale. For example, to represent a shutter interval equal to 0.04
seconds, sii_time_scale may be equal to 27 000 000 and sii_num_units_in_shutter_interval may be equal to 1 080 000.


**sii_max_sub_layers_minus1** plus 1 specifies the maximum number of shutter interval temporal sub-layers indexes that
may be present in the coded video sequence.


**sub_layer_num_units_in_shutter_interval[** i **]**, when present, specifies the number of time units of a clock operating at
the frequency sii_time_scale Hz that corresponds to the shutter interval of each picture in the coded video sequence for
which the value of sii_sub_layer_idx is equal to i. The sub-layer shutter interval for each picture for which the value of
sii_sub_layer_idx is equal to i, denoted by the variable subLayerShutterInterval[ i ], in units of seconds, is equal to the
quotient of sub_layer_num_units_in_shutter_interval[ i ] divided by sii_time_scale.


The variable subLayerShutterInterval[ i ], corresponding to the indicated shutter interval of each picture in the coded video
sequence for which the value of sii_sub_layer_idx is equal to i, is thus derived as follows:


if( fixed_shutter_interval_within_cvs_flag )
subLayerShutterInterval[ i ] = sii_num_units_in_shutter_interval ÷ sii_time_scale (D-51)
else
subLayerShutterInterval[ i ] = sub_layer_num_units_in_shutter_interval[ i ] ÷ sii_time_scale


When a shutter interval information SEI message is present for any access unit in a coded video sequence, a shutter interval
information SEI message shall be present for the IDR access unit that is the first access unit of the coded video sequence.
All shutter interval information SEI messages that apply to the same access unit shall have the same content.


sii_time_scale and fixed_shutter_interval_within_cvs_flag persist from the first access unit of the coded video sequence
until a new coded video sequence begins or the bitstream ends.


When the value of fixed_shutter_interval_within_cvs_flag is equal to 0, a shutter interval information SEI message shall
be present for every picture in the coded video sequence. When present, sii_num_units_in_shutter_interval,
sii_max_sub_layers_minus1, and sub_layer_num_units_in_shutter_interval[ i ], persist from the first access unit of the
coded video sequence until a new coded video sequence begins or the bitstream ends.


**D.2.40** **Reserved SEI message semantics**


This message consists of data reserved for future backward-compatible use by ITU-T | ISO/IEC. Encoders conforming to
this Recommendation | International Standard shall not send reserved SEI messages until and unless the use of such
messages has been specified by ITU-T | ISO/IEC. Decoders that encounter reserved SEI messages shall discard their
content without effect on the decoding process, except as specified in future Recommendations | International Standards
specified by ITU-T | ISO/IEC.


**reserved_sei_message_payload_byte** is a byte reserved for future use by ITU-T | ISO/IEC.
