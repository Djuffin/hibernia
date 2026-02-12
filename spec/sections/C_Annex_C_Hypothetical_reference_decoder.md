**Annex C**


**Hypothetical reference decoder**


(This annex forms an integral part of this Recommendation | International Standard.)


This annex specifies the hypothetical reference decoder (HRD) and its use to check bitstream and decoder conformance.


Two types of bitstreams are subject to HRD conformance checking for this Recommendation | International Standard. The
first such type of bitstream, called Type I bitstream, is a NAL unit stream containing only the VCL NAL units and filler
data NAL units for all access units in the bitstream. The second type of bitstream, called a Type II bitstream, contains, in
addition to the VCL NAL units and filler data NAL units for all access units in the bitstream, at least one of the following:

- additional non-VCL NAL units other than filler data NAL units,

- all leading_zero_8bits, zero_byte, start_code_prefix_one_3bytes, and trailing_zero_8bits syntax elements that form a
byte stream from the NAL unit stream (as specified in Annex B).


Figure C-1 shows the types of bitstream conformance points checked by the HRD.


Non-VCL NAL units other



VCL NAL units



than filter data NAL units







H.264(09)_FC-1


**Figure C-** 1 **– Structure of byte streams and NAL unit streams for HRD conformance checks**


The syntax elements of non-VCL NAL units (or their default values for some of the syntax elements), required for the
HRD, are specified in the semantics subclauses of clause 7, Annexes D and E, and clauses G.7, G.13, G.14, H.7, H.13,
H.14, I.7, I.13, and I.14.


Two types of HRD parameter sets (NAL HRD parameters and VCL HRD parameters) are used. The HRD parameter sets
are signalled as follows:

- When the coded video sequence conforms to one or more of the profiles specified in Annex A and the decoding
process specified in clauses 2 to 9 is applied, the HRD parameter sets are signalled through video usability information
as specified in clauses E.1 and E.2, which is part of the sequence parameter set syntax structure.

- When the coded video sequence conforms to one or more of the profiles specified in Annex G and the decoding
process specified in Annex G is applied, the HRD parameter sets are signalled through the SVC video usability
information extension as specified in clauses G.14.1 and G.14.2, which is part of the subset sequence parameter set
syntax structure.
NOTE 1 – For coded video sequences that conform to both, one or more of the profiles specified in Annex A and one or more of the
profiles specified in Annex G, the signalling of the applicable HRD parameter sets is depending on whether the decoding process
specified in clauses 2 to 9 or the decoding process specified in Annex G is applied.

- When the coded video sequence conforms to one or more of the profiles specified in Annex H and the decoding
process specified in Annex H is applied, the HRD parameter sets are signalled through the MVC video usability
information extension as specified in clauses H.14.1 and H.14.2, which is part of the subset sequence parameter set
syntax structure.





NOTE 2 – For coded video sequences that conform to both, one or more of the profiles specified in Annex A and one or more of the
profiles specified in Annex H, the signalling of the applicable HRD parameter sets is depending on whether the decoding process
specified in clauses 2 to 9 or the decoding process specified in Annex H is applied.

- When the coded video sequence conforms to one or more of the profiles specified in Annex I and the decoding process
specified in Annex I is applied, the HRD parameter sets are signalled through the MVC video usability information
extension as specified in clause I.14, which is part of the subset sequence parameter set syntax structure.
NOTE 3 – For coded video sequences that conform to one or more of the profiles specified in Annex A, one or more of the profiles
specified in Annex H and one or more of the profiles specified in Annex I, the signalling of the applicable HRD parameter sets is
dependent on whether the decoding process specified in clauses 2-9, the decoding process specified in Annex H, or the decoding
process specified in Annex I is applied.


All sequence parameter sets and picture parameter sets referred to in the VCL NAL units, and corresponding buffering
period and picture timing SEI messages shall be conveyed to the HRD, in a timely manner, either in the bitstream (by nonVCL NAL units), or by other means not specified in this Recommendation | International Standard.


In Annexes C, D, and E and clauses G.12, G.13, G.14, H.12, H.13, H.14, I.13 and I.14 the specification for "presence" of
non-VCL NAL units is also satisfied when those NAL units (or just some of them) are conveyed to decoders (or to the
HRD) by other means not specified by this Recommendation | International Standard. For the purpose of counting bits,
only the appropriate bits that are actually present in the bitstream are counted.

NOTE 4 – As an example, synchronization of a non-VCL NAL unit, conveyed by means other than presence in the bitstream, with
the NAL units that are present in the bitstream, can be achieved by indicating two points in the bitstream, between which the
non-VCL NAL unit would have been present in the bitstream, had the encoder decided to convey it in the bitstream.


When the content of a non-VCL NAL unit is conveyed for the application by some means other than presence within the
bitstream, the representation of the content of the non-VCL NAL unit is not required to use the same syntax specified in
this annex.

NOTE 5 – When HRD information is contained within the bitstream, it is possible to verify the conformance of a bitstream to the
requirements of this clause based solely on information contained in the bitstream. When the HRD information is not present in the
bitstream, as is the case for all "stand-alone" Type I bitstreams, conformance can only be verified when the HRD data is supplied
by some other means not specified in this Recommendation | International Standard.


The HRD contains a coded picture buffer (CPB), an instantaneous decoding process, a decoded picture buffer (DPB), and
output cropping as shown in Figure C-2.





Reference
fields or frames









H.264(13)_FC-2


**Figure C-2 – HRD buffer model**


The CPB size (number of bits) is CpbSize[ SchedSelIdx ]. The DPB size (number of frame buffers) is
Max( 1, max_dec_frame_buffering ). When the coded video sequence conforms to one or more of the profiles specified in
Annex H and the decoding process specified in Annex H is applied, the DPB size is specified in units of view components.
When the coded video sequence conforms to one or more of the profiles specified in Annex I and the decoding process
specified in Annex I is applied, the DPB is operated separately for texture view components and depth view components
and the terms texture DPB and depth DPB are used, respectively. The texture DPB size is specified in units of texture view
components and the depth DPB size is specified in units of depth view components.


The HRD operates as follows. Data associated with access units that flow into the CPB according to a specified arrival
schedule are delivered by the HSS. The data associated with each access unit are removed and decoded instantaneously by
the instantaneous decoding process at CPB removal times. Each decoded picture is placed in the DPB at its CPB removal
time unless it is output at its CPB removal time and is a non-reference picture. When a picture is placed in the DPB it is
removed from the DPB at the later of the DPB output time or the time that it is marked as "unused for reference".


For each picture in the bitstream, the variable OutputFlag for the decoded picture and, when applicable, the reference base
picture, is set as follows:

- If the coded video sequence containing the picture conforms to one or more of the profiles specified in Annex A and
the decoding process specified in clauses 2 to 9 is applied, OutputFlag is set equal to 1.

- Otherwise, if the coded video sequence containing the picture conforms to one or more of the profiles specified in
Annex G and the decoding process specified in Annex G is applied, the following applies:

   - For a reference base picture, OutputFlag is set equal to 0.

   - For a decoded picture, OutputFlag is set equal to the value of the output_flag syntax element of the target layer
representation.

- Otherwise, if the coded video sequence containing the picture conforms to one or more of the profiles specified in
Annex H and the decoding process specified in Annex H is applied, the following applies:

   - For the decoded view components of the target output views, OutputFlag is set equal to 1.

   - For the decoded view components of other views, OutputFlag is set equal to 0.

- Otherwise (the coded video sequence containing the picture conforms to one or more of the profiles specified in
Annex I and the decoding process specified in Annex I is applied), the following applies:





   - For the decoded texture view components and corresponding depth view components with the same VOIdx as
the target output views, OutputFlag is set equal to 1.

   - For the decoded texture view components and corresponding depth view components with the same VOIdx as
other views, OutputFlag is set equal to 0.


The operation of the CPB is specified in clause C.1. The instantaneous decoder operation is specified in clauses 2 to 9 (for
coded video sequences conforming to one or more of the profiles specified in Annex A) and in Annex G (for coded video
sequences conforming to one or more of the profiles specified in Annex G) and in Annex H (for coded video sequences
conforming to one or more of the profiles specified in Annex H) and in Annex I (for coded video sequences conforming
to one or more of the profiles specified in Annex I). The operation of the DPB is specified in clause C.2. The output
cropping is specified in clause C.2.2.

NOTE 6 – Coded video sequences that conform to one or more of the profiles specified in Annex A, and at the same time, one or
more of the profiles specified in Annex G can be decoded either by the decoding process specified in clauses 2 to 9 or by the decoding
process specified in Annex G. The decoding result and the HRD operation may be dependent on which of the decoding processes is
applied.
NOTE 7 – Coded video sequences that conform to one or more of the profiles specified in Annex A, and at the same time, one or
more of the profiles specified in Annex H can be decoded either by the decoding process specified in clauses 2 to 9 or by the decoding
process specified in Annex H. The decoding result and the HRD operation may be dependent on which of the decoding processes is
applied.
NOTE 8 – Coded video sequences that conform to one or more of the profiles specified in Annex A, one or more of the profiles
specified in Annex H, and one or more of the profiles specified in Annex I, can be decoded either by the decoding process specified
in clauses 2 to 9, by the decoding process specified in Annex H or by the decoding process specified in Annex I. The decoding result
and the HRD operation may be dependent on which of the decoding processes is applied.


HSS and HRD information concerning the number of enumerated delivery schedules and their associated bit rates and
buffer sizes is specified in clauses E.1.1, E.1.2, E.2.1, E.2.2, G.14.1, G.14.2, H.14.1, H.14.2 and I.14. The HRD is initialized
as specified by the buffering period SEI message as specified in clauses D.1.2 and D.2.1. The removal timing of access
units from the CPB and output timing from the DPB are specified in the picture timing SEI message as specified in
clauses D.1.3 and D.2.3. All timing information relating to a specific access unit shall arrive prior to the CPB removal time
of the access unit.


When the coded video sequence conforms to one or more of the profiles specified in Annex G and the decoding process
specified in Annex G is applied, the following is specified:


(a) When an access unit contains one or more buffering period SEI messages that are included in scalable nesting

SEI messages and are associated with values of DQId in the range of ( ( DQIdMax >> 4) << 4 ) to
( ( ( DQIdMax >> 4 ) << 4 ) + 15 ), inclusive, the last of these buffering period SEI messages in decoding order
is the buffering period SEI message that initializes the HRD. Let hrdDQId be the largest value of
16 * sei_dependency_id[ i ] + sei_quality_id[ i ] that is associated with the scalable nesting SEI message
containing the buffering period SEI message that initializes the HRD, let hrdDId and hrdQId be equal to
hrdDQId >> 4 and hrdDQId & 15, respectively, and let hrdTId be the value of sei_temporal_id that is associated
with the scalable nesting SEI message containing the buffering period SEI message that initializes the HRD.


(b) The picture timing SEI messages that specify the removal timing of access units from the CPB and output timing

from the DPB are the picture timing SEI messages that are included in scalable nesting SEI messages associated
with values of sei_dependency_id[ i ], sei_quality_id[ i ], and sei_temporal_id equal to hrdDId, hrdQId, and
hrdTId, respectively.


(c) The HRD parameters that are used for conformance checking are the HRD parameters included in the SVC video

usability information extension of the active SVC sequence parameter set that are associated with values of
vui_ext_dependency_id[ i ], vui_ext_quality_id[ i ], and vui_ext_temporal_id[ i ] equal to hrdDId, hrdQId, and
hrdTId, respectively. For the specification in this annex, num_units_in_tick, time_scale, fixed_frame_rate_flag,
nal_hrd_parameters_present_flag, vcl_hrd_parameters_present_flag, low_delay_hrd_flag, and
pic_struct_present_flag are substituted with the values of vui_ext_num_units_in_tick[ i ],
vui_ext_time_scale[ i ], vui_ext_fixed_frame_rate_flag[ i ], vui_ext_nal_hrd_parameters_present_flag[ i ],
vui_ext_vcl_hrd_parameters_present_flag[ i ], vui_ext_low_delay_hrd_flag[ i ], and
vui_ext_pic_struct_present_flag[ i ], respectively, with i being the value for which vui_ext_dependency_id[ i ],
vui_ext_quality_id[ i ], and vui_ext_temporal_id[ i ] are equal to hrdDId, hrdQId, and hrdTId, respectively.


When the coded video sequence conforms to one or more of the profiles specified in Annex H and the decoding process
specified in Annex H is applied, the following is specified:


(a) When an access unit contains one or more buffering period SEI messages that are included in MVC scalable

nesting SEI messages, the buffering period SEI message that is associated with the operation point being decoded
is the buffering period SEI message that initializes the HRD. Let hrdVId[ i ] be equal to sei_op_view_id[ i ] for
all i in the range of 0 to num_view_components_op_minus1, inclusive, and let hrdTId be the value of





sei_op_temporal_id, that are associated with the MVC scalable nesting SEI message containing the buffering
period SEI message that initializes the HRD.


(b) The picture timing SEI messages that specify the removal timing of access units from the CPB and output timing

from the DPB are the picture timing SEI messages that are included in MVC scalable nesting SEI messages
associated with values of sei_op_view_id[ i ] equal to hrdVId[ i ] for all i in the range of 0 to
num_view_components_op_minus1, inclusive, and sei_temporal_id equal to hrdTId.


(c) The HRD parameters that are used for conformance checking are the HRD parameters included in the MVC

video usability information extension of the active MVC sequence parameter set that are associated with values
of vui_mvc_view_id[ i ][ j ] for all j in the range of 0 to vui_mvc_num_target_output_views_minus1[ i ],
inclusive, equal to hrdVId[ j ], and the value of vui_mvc_temporal_id[ i ] equal to hrdTId. For the specification
in this annex, num_units_in_tick, time_scale, fixed_frame_rate_flag, nal_hrd_parameters_present_flag,
vcl_hrd_parameters_present_flag, low_delay_hrd_flag, and pic_struct_present_flag are substituted with the
values of vui_mvc_num_units_in_tick[ i ], vui_mvc_time_scale[ i ], vui_mvc_fixed_frame_rate_flag[ i ],
vui_mvc_nal_hrd_parameters_present_flag[ i ], vui_mvc_vcl_hrd_parameters_present_flag[ i ],
vui_mvc_low_delay_hrd_flag[ i ], and vui_mvc_pic_struct_present_flag[ i ], respectively, with i being the value
for which vui_mvc_view_id[ i ] is equal to hrdVId[ j ] for all j in the range of 0 to
vui_mvc_num_traget_output_views_minus1[ i ], inclusive, and vui_mvc_temporal_id[ i ] equal to hrdTId.


When the coded video sequence conforms to one or more of the profiles specified in Annex I and the decoding process
specified in Annex I is applied, the following is specified:


(a) When an access unit contains one or more buffering period SEI messages that are included in MVCD scalable

nesting SEI messages, the buffering period SEI message that is associated with the operation point being decoded
is the buffering period SEI message that initializes the HRD. Let hrdVId[ i ] be equal to sei_op_view_id[ i ] for
all i in the range of 0 to num_view_components_op_minus1, inclusive, and let hrdTId be the value of
sei_op_temporal_id, that are associated with the MVCD scalable nesting SEI message containing the buffering
period SEI message that initializes the HRD.


(b) The picture timing SEI messages that specify the removal timing of access units from the CPB and output timing

from the DPB are the picture timing SEI messages that are included in MVCD scalable nesting SEI messages
associated with values of sei_op_view_id[ i ] equal to hrdVId[ i ] for all i in the range of 0 to
num_view_components_op_minus1, inclusive, and sei_temporal_id equal to hrdTId.


(c) The HRD parameter sets that are used for conformance checking are the HRD parameter sets, included in the

MVC video usability information extension of the active MVCD sequence parameter set, that are associated with
values of vui_mvc_view_id[ i ][ j ] for all j in the range of 0 to vui_mvc_num_target_output_views_minus1[ i ],
inclusive, equal to hrdVId[ j ], and the value of vui_mvc_temporal_id[ i ] equal to hrdTId. For the specification
in this annex, num_units_in_tick, time_scale, fixed_frame_rate_flag, nal_hrd_parameters_present_flag,
vcl_hrd_parameters_present_flag, low_delay_hrd_flag, and pic_struct_present_flag are substituted with the
values of vui_mvc_num_units_in_tick[ i ], vui_mvc_time_scale[ i ], vui_mvc_fixed_frame_rate_flag[ i ],
vui_mvc_nal_hrd_parameters_present_flag[ i ], vui_mvc_vcl_hrd_parameters_present_flag[ i ],
vui_mvc_low_delay_hrd_flag[ i ], and vui_mvc_pic_struct_present_flag[ i ], respectively, with i being the value
for which vui_mvc_view_id[ i ] is equal to hrdVId[ j ] for all j in the range of 0 to
vui_mvc_num_traget_output_views_minus1[ i ], inclusive, and vui_mvc_temporal_id[ i ] equal to hrdTId.


The HRD is used to check conformance of bitstreams and decoders as specified in clauses C.3 and C.4, respectively.

NOTE 9 – While conformance is guaranteed under the assumption that all frame-rates and clocks used to generate the bitstream
match exactly the values signalled in the bitstream, in a real system each of these may vary from the signalled or specified value.


All the arithmetic in this annex is done with real values, so that no rounding errors can propagate. For example, the number
of bits in a CPB just prior to or after removal of an access unit is not necessarily an integer.


The variable tc is derived as follows and is called a clock tick:


tc = num_units_in_tick ÷ time_scale (C-1)


The following is specified for expressing the constraints in this annex:


- Let access unit n be the n-th access unit in decoding order with the first access unit being access unit 0.


- Let picture n be the primary coded picture or the decoded primary picture of access unit n.
