**H.7** **Syntax and semantics**


This clause specifies syntax and semantics for coded video sequences that conform to one or more of the profiles specified
in this annex.


**H.7.1** **Method of specifying syntax in tabular form**


The specifications in clause 7.1 apply.


**H.7.2** **Specification of syntax functions, categories, and descriptors**


The specifications in clause 7.2 apply.


**H.7.3** **Syntax in tabular form**


**H.7.3.1** **NAL unit syntax**


The syntax table is specified in clause 7.3.1.


**H.7.3.1.1** **NAL unit header MVC extension syntax**

|nal_unit_header_mvc_extension( ) {|C|Descriptor|
|---|---|---|
|<br>**non_idr_flag**|All|u(1)|
|<br>**priority_id**|All|u(6)|
|<br>**view_id**|All|u(10)|
|<br>**temporal_id**|All|u(3)|
|<br>**anchor_pic_flag**|All|u(1)|
|<br>**inter_view_flag**|All|u(1)|
|<br>**reserved_one_bit**|All|u(1)|
|}|||



**H.7.3.2** **Raw byte sequence payloads and RBSP trailing bits syntax**


**H.7.3.2.1** **Sequence parameter set RBSP syntax**


The syntax table is specified in clause 7.3.2.1.


**H.7.3.2.1.1** **Sequence parameter set data syntax**


The syntax table is specified in clause 7.3.2.1.1.


**H.7.3.2.1.1.1** **Scaling list syntax**


The syntax table is specified in clause 7.3.2.1.1.1.


**H.7.3.2.1.2** **Sequence parameter set extension RBSP syntax**


The syntax table is specified in clause 7.3.2.1.2.


**H.7.3.2.1.3** **Subset sequence parameter set RBSP syntax**


The syntax table is specified in clause 7.3.2.1.3.


**H.7.3.2.1.4** **Sequence parameter set MVC extension syntax**





|seq_parameter_set_mvc_extension( ) {|C|Descriptor|
|---|---|---|
|<br>**num_views_minus1**|0|ue(v)|
|<br>for( i = 0; i <= num_views_minus1; i++ )|||
|<br> <br>**view_id[**i **] **|0|ue(v)|
|<br>for( i = 1; i <= num_views_minus1; i++ ) {|||
|<br> <br>**num_anchor_refs_l0[** i**]**|0|ue(v)|
|<br> <br>for( j = 0; j < num_anchor_refs_l0[ i ]; j++ )|||
|<br> <br> <br>**anchor_ref_l0[** i**][** j**] **|0|ue(v)|
|<br> <br>**num_anchor_refs_l1[** i**]**|0|ue(v)|
|<br> <br>for( j = 0; j < num_anchor_refs_l1[ i ]; j++ )|||
|<br> <br> <br>**anchor_ref_l1[** i**][** j**] **|0|ue(v)|
|<br>}|||
|<br>for( i = 1; i <= num_views_minus1; i++ ) {|||
|<br> <br>**num_non_anchor_refs_l0[** i**]**|0|ue(v)|
|<br> <br>for( j = 0; j < num_non_anchor_refs_l0[ i ]; j++ )|||
|<br> <br> <br>**non_anchor_ref_l0[**i** ][**j** ]**|0|ue(v)|
|<br> <br>**num_non_anchor_refs_l1[**i **]**|0|ue(v)|
|<br> <br>for( j = 0; j < num_non_anchor_refs_l1[ i ]; j++ )|||
|<br> <br> <br>**non_anchor_ref_l1[**i **][**j **] **|0|ue(v)|
|<br>}|||
|<br>**num_level_values_signalled_minus1**|0|ue(v)|
|<br>for( i = 0; i <= num_level_values_signalled_minus1; i++ ) {|||
|<br> <br>**level_idc[** i**]**|0|u(8)|
|<br> <br>**num_applicable_ops_minus1[** i**]**|0|ue(v)|
|<br> <br>for( j = 0; j <= num_applicable_ops_minus1[ i ]; j++ ) {|||
|<br> <br> <br>**applicable_op_temporal_id[** i**][** j**]**|0|u(3)|
|<br> <br> <br>**applicable_op_num_target_views_minus1[** i**][** j**]**|0|ue(v)|
|<br> <br> <br>for( k = 0; k <= applicable_op_num_target_views_minus1[ i ][ j ]; k++ )|||
|<br> <br> <br> <br>**applicable_op_target_view_id[** i**][** j**][** k**]**|0|ue(v)|
|<br> <br> <br>**applicable_op_num_views_minus1[** i**][** j**]**|0|ue(v)|
|<br> <br>}|||
|<br>}|||
|<br>if( profile_idc = = 134 ) {|||
|<br> <br>**mfc_format_idc**|0|u(6)|
|<br> <br>if( mfc_format_idc = = 0 | | mfc_format_idc = = 1 ) {|||
|<br> <br> <br>**default_grid_position_flag**|0|u(1)|
|<br> <br> <br>if( !default_grid_position_flag ) {|||
|<br> <br> <br> <br>**view0_grid_position_x**|0|u(4)|
|<br> <br> <br> <br>**view0_grid_position_y**|0|u(4)|
|<br> <br> <br> <br>**view1_grid_position_x**|0|u(4)|
|<br> <br> <br> <br>**view1_grid_position_y**|0|u(4)|
|<br> <br> <br>}|||
|<br> <br>}|||
|<br> <br>**rpu_filter_enabled_flag**|0|u(1)|
|<br> <br>if( !frame_mbs_only_flag )|||
|<br> <br> <br>**rpu_field_processing_flag**|0|u(1)|
|<br>}|||
|}|||





**H.7.3.2.2** **Picture parameter set RBSP syntax**


The syntax table is specified in clause 7.3.2.2.


**H.7.3.2.3** **Supplemental enhancement information RBSP syntax**


The syntax table is specified in clause 7.3.2.3.


**H.7.3.2.3.1** **Supplemental enhancement information message syntax**


The syntax table is specified in clause 7.3.2.3.1.


**H.7.3.2.4** **Access unit delimiter RBSP syntax**


The syntax table is specified in clause 7.3.2.4.


**H.7.3.2.5** **End of sequence RBSP syntax**


The syntax table is specified in clause 7.3.2.5.


**H.7.3.2.6** **End of stream RBSP syntax**


The syntax table is specified in clause 7.3.2.6.


**H.7.3.2.7** **Filler data RBSP syntax**


The syntax table is specified in clause 7.3.2.7.


**H.7.3.2.8** **Slice layer without partitioning RBSP syntax**


The syntax table is specified in clause 7.3.2.8.


**H.7.3.2.9** **Slice data partition RBSP syntax**


Slice data partition syntax is not present in coded video sequences conforming to one or more of the profiles specified in
this annex.


**H.7.3.2.10RBSP slice trailing bits syntax**


The syntax table is specified in clause 7.3.2.10.


**H.7.3.2.11RBSP trailing bits syntax**


The syntax table is specified in clause 7.3.2.11.


**H.7.3.2.12Prefix NAL unit RBSP syntax**


The syntax table is specified in clause 7.3.2.12.


**H.7.3.2.13Slice layer extension RBSP syntax**


The syntax table is specified in clause 7.3.2.13.


**H.7.3.3** **Slice header syntax**


The syntax table is specified in clause 7.3.3.


**H.7.3.3.1** **Reference picture list modification syntax**


The syntax table is specified in clause 7.3.3.1.





**H.7.3.3.1.1** **Reference picture list MVC modification syntax**

|ref_pic_list_mvc_modification( ) {|C|Descriptor|
|---|---|---|
|<br>if( slice_type % 5 != 2 && slice_type % 5 != 4 ) {|||
|<br> <br>**ref_pic_list_modification_flag_l0**|2|u(1)|
|<br> <br>if( ref_pic_list_modification_flag_l0 )|||
|<br> <br> <br>do {|||
|<br> <br> <br> <br>**modification_of_pic_nums_idc**|2|ue(v)|
|<br> <br> <br> <br>if( modification_of_pic_nums_idc = = 0 | |<br> <br> <br> <br> <br> <br>modification_of_pic_nums_idc = = 1 )|||
|<br> <br> <br> <br> <br>**abs_diff_pic_num_minus1**|2|ue(v)|
|<br> <br> <br> <br>else if( modification_of_pic_nums_idc = = 2 )|||
|<br> <br> <br> <br> <br>**long_term_pic_num**|2|ue(v)|
|<br> <br> <br> <br>else if( modification_of_pic_nums_idc = = 4 | |<br> <br> <br> <br> <br> <br> <br> <br>modification_of_pic_nums_idc = = 5 )|||
|<br> <br> <br> <br> <br> <br>**abs_diff_view_idx_minus1**|2|ue(v)|
|<br> <br> <br>} while( modification_of_pic_nums_idc != 3 )|||
|<br>}|||
|<br>if( slice_type % 5 = = 1 ) {|||
|<br> <br>**ref_pic_list_modification_flag_l1**|2|u(1)|
|<br> <br>if( ref_pic_list_modification_flag_l1 )|||
|<br> <br> <br>do {|||
|<br> <br> <br> <br>**modification_of_pic_nums_idc**|2|ue(v)|
|<br> <br> <br> <br>if( modification_of_pic_nums_idc = = 0 | |<br> <br> <br> <br> <br> <br>modification_of_pic_nums_idc = = 1 )|||
|<br> <br> <br> <br> <br>**abs_diff_pic_num_minus1**|2|ue(v)|
|<br> <br> <br> <br>else if( modification_of_pic_nums_idc = = 2 )|||
|<br> <br> <br> <br> <br>**long_term_pic_num**|2|ue(v)|
|<br> <br> <br> <br>else if( modification_of_pic_nums_idc = = 4 | |<br> <br> <br> <br> <br> <br> <br> <br>modification_of_pic_nums_idc = = 5 )|||
|<br> <br> <br> <br> <br>**abs_diff_view_idx_minus1**|2|ue(v)|
|<br> <br> <br>} while( modification_of_pic_nums_idc != 3 )|||
|<br>}|||
|}|||



**H.7.3.3.2** **Prediction weight table syntax**


The syntax table is specified in clause 7.3.3.2.


**H.7.3.3.3** **Decoded reference picture marking syntax**


The syntax table is specified in clause 7.3.3.3.


**H.7.3.4** **Slice data syntax**


The syntax table is specified in clause 7.3.4.


**H.7.3.5** **Macroblock layer syntax**


The syntax table is specified in clause 7.3.5.


**H.7.3.5.1** **Macroblock prediction syntax**


The syntax table is specified in clause 7.3.5.1.


**H.7.3.5.2** **Sub-macroblock prediction syntax**


The syntax table is specified in clause 7.3.5.2.





**H.7.3.5.3** **Residual data syntax**


The syntax table is specified in clause 7.3.5.3.


**H.7.3.5.3.1** **Residual luma syntax**


The syntax table is specified in clause 7.3.5.3.1.


**H.7.3.5.3.2** **Residual block CAVLC syntax**


The syntax table is specified in clause 7.3.5.3.2.


**H.7.3.5.3.3** **Residual block CABAC syntax**


The syntax table is specified in clause 7.3.5.3.3.


**H.7.4** **Semantics**


Semantics associated with the syntax structures and syntax elements within these structures (in clause H.7.3 and in
clause 7.3 by reference in clause H.7.3) are specified in this clause and by reference to clause 7.4. When the semantics of
a syntax element are specified using a table or a set of tables, any values that are not specified in the table(s) shall not be
present in the bitstream unless otherwise specified in this Recommendation | International Standard.


**H.7.4.1** **NAL unit semantics**


The semantics for the syntax elements in clause H.7.3.1 are specified in clause 7.4.1. The following specifications
additionally apply.


For NAL units with nal_unit_type equal to 14, nal_ref_idc shall be identical to the value of nal_ref_idc for the associated
NAL unit, which follows the NAL unit with nal_unit_type equal to 14 in decoding order.


The value of nal_ref_idc shall be identical for all VCL NAL units of a view component.


**H.7.4.1.1** **NAL unit header MVC extension semantics**


The syntax elements non_idr_flag, priority_id, view_id, temporal_id, anchor_pic_flag, and inter_view_flag, when present
in a prefix NAL unit, are considered to apply to the associated NAL unit.


**non_idr_flag** equal to 0 specifies that the current access unit is an IDR access unit.


The value of non_idr_flag shall be the same for all VCL NAL units of an access unit. When non_idr_flag is equal to 0 for
a prefix NAL unit, the associated NAL unit shall have nal_unit_type equal to 5. When non_idr_flag is equal to 1 for a
prefix NAL unit, the associated NAL unit shall have nal_unit_type equal to 1.


When nal_unit_type is equal to 1 and the NAL unit is not immediately preceded by a NAL unit with nal_unit_type equal
to 14, non_idr_flag shall be inferred to be equal to 1. When nal_unit_type is equal to 5 and the NAL unit is not immediately
preceded by a NAL unit with nal_unit_type equal to 14, non_idr_flag shall be inferred to be equal to 0.


When nal_ref_idc is equal to 0, the value of non_idr_flag shall be equal to 1.


For NAL units in which non_idr_flag is present, the variable IdrPicFlag derived in clause 7.4.1 is modified by setting it
equal to 1 when non_idr_flag is equal to 0, and setting it equal to 0 when non_idr_flag is equal to 1.


**priority_id** specifies a priority identifier for the NAL unit. A lower value of priority_id specifies a higher priority. The
assignment of values to priority_id is constrained by the sub-bitstream extraction process as specified in clause H.8.5.3.


When nal_unit_type is equal to 1 or 5 and the NAL unit is not immediately preceded by a NAL unit with nal_unit_type
equal to 14, priority_id shall be inferred to be equal to 0.


NOTE 1 – The syntax element priority_id is not used by the decoding process specified in this Recommendation | International
Standard. The syntax element priority_id may be used as determined by the application within the specified constraints.


**view_id** specifies a view identifier for the NAL unit. NAL units with the same value of view_id belong to the same view.
The assignment of values to view_id is constrained by the sub-bitstream extraction process as specified in clause H.8.5.3.


When nal_unit_type is equal to 1 or 5 and the NAL unit is not immediately preceded by a NAL unit with nal_unit_type
equal to 14, the value of view_id shall be inferred to be equal to 0. When the bitstream does contain NAL units with
nal_unit_type equal to 1 or 5 that are not immediately preceded by a NAL unit with nal_unit_type equal to 14, it shall not
contain data that result in a value of view_id for a view component of any non-base view that is equal to 0.


The variable VOIdx, representing the view order index of the view identified by view_id, is set equal to the value of i for
which the syntax element view_id[ i ] included in the referred subset sequence parameter set is equal to view_id.


**temporal_id** specifies a temporal identifier for the NAL unit.





When nal_unit_type is equal to 1 or 5 and the NAL unit is not immediately preceded by a NAL unit with nal_unit_type
equal to 14, temporal_id shall be inferred to be equal to the value of temporal_id for the non-base views in the same access
unit.


The value of temporal_id shall be the same for all prefix and coded slice MVC extension NAL units of an access unit.
When an access unit contains any NAL unit with nal_unit_type equal to 5 or non_idr_flag equal to 0, temporal_id shall be
equal to 0.


The assignment of values to temporal_id is further constrained by the sub-bitstream extraction process as specified in
clause H.8.5.3.


**anchor_pic_flag** equal to 1 specifies that the current access unit is an anchor access unit.


When nal_unit_type is equal to 1 or 5 and the NAL unit is not immediately preceded by a NAL unit with nal_unit_type
equal to 14, anchor_pic_flag shall be inferred to be equal to the value of anchor_pic_flag for the non-base views in the
same access unit.


When non_idr_flag is equal to 0, anchor_pic_flag shall be equal to 1.


When nal_ref_idc is equal to 0, the value of anchor_pic_flag shall be equal to 0.


The value of anchor_pic_flag shall be the same for all VCL NAL units of an access unit.


**inter_view_flag** equal to 0 specifies that the current view component is not used for inter-view prediction by any other
view component in the current access unit. inter_view_flag equal to 1 specifies that the current view component may be
used for inter-view prediction by other view components in the current access unit.


When nal_unit_type is equal to 1 or 5 and the NAL unit is not immediately preceded by a NAL unit with nal_unit_type
equal to 14, inter_view_flag shall be inferred to be equal to 1.


The value of inter_view_flag shall be the same for all VCL NAL units of a view component.


**reserved_one_bit** shall be equal to 1. The value 0 for reserved_one_bit may be specified by future extension of this
Recommendation | International Standard. Decoders shall ignore the value of reserved_one_bit.


**H.7.4.1.2** **Order of NAL units and association to coded pictures, access units, and video sequences**


This clause specifies constraints on the order of NAL units in the bitstream. Any order of NAL units in the bitstream
obeying these constraints is referred to in the text as the decoding order of NAL units. Within a NAL unit, the syntax in
clauses 7.3, D.1, E.1, H.7.3, H.13.1, and H.14.1 specifies the decoding order of syntax elements. Decoders shall be capable
of receiving NAL units and their syntax elements in decoding order.


**H.7.4.1.2.1** **Order of MVC sequence parameter set RBSPs and picture parameter set RBSPs and their activation**


NOTE 1 – The sequence and picture parameter set mechanism decouples the transmission of infrequently changing information
from the transmission of coded macroblock data. Sequence and picture parameter sets may, in some applications, be conveyed "outof-band" using a reliable transport mechanism.


A picture parameter set RBSP includes parameters that can be referred to by the coded slice NAL units of one or more
view components of one or more coded pictures.


Each picture parameter set RBSP is initially considered not active at the start of the operation of the decoding process. At
most one picture parameter set RBSP is considered as the active picture parameter set RBSP at any given moment during
the operation of the decoding process, and when any particular picture parameter set RBSP becomes the active picture
parameter set RBSP, the previously-active picture parameter set RBSP (if any) is deactivated.


In addition to the active picture parameter set RBSP, zero or more picture parameter set RBSPs may be specifically active
for view components (with a particular value of VOIdx less than VOIdxMax) that may be referred to through inter-view
prediction in decoding the view component with VOIdx equal to VOIdxMax. Such a picture parameter set RBSP is referred
to as active view picture parameter set RBSP for the particular value of VOIdx. The restrictions on active picture parameter
set RBSPs also apply to active view picture parameter set RBSPs for a particular value of VOIdx less than VOIdxMax.


When a picture parameter set RBSP (with a particular value of pic_parameter_set_id) is not the active picture parameter
set RBSP and it is referred to by a coded slice NAL unit with VOIdx equal to VOIdxMax (using that value of
pic_parameter_set_id), it is activated. This picture parameter set RBSP is called the active picture parameter set RBSP
until it is deactivated when another picture parameter set RBSP becomes the active picture parameter set RBSP. A picture
parameter set RBSP, with that particular value of pic_parameter_set_id, shall be available to the decoding process prior to
its activation.


When a picture parameter set RBSP (with a particular value of pic_parameter_set_id) is not the active view picture
parameter set for a particular value of VOIdx less than VOIdxMax and it is referred to by a coded slice NAL unit with the
particular value of VOIdx (using that value of pic_parameter_set_id), it is activated for view components with the particular





value of VOIdx. This picture parameter set RBSP is called the active view picture parameter set RBSP for the particular
value of VOIdx until it is deactivated when another picture parameter set RBSP becomes the active view picture parameter
set RBSP for the particular value of VOIdx. A picture parameter set RBSP, with that particular value of
pic_parameter_set_id, shall be available to the decoding process prior to its activation.


Any picture parameter set NAL unit containing the value of pic_parameter_set_id for the active picture parameter set
RBSP for a coded picture shall have the same content as that of the active picture parameter set RBSP for this coded picture
unless it follows the last VCL NAL unit of this coded picture and precedes the first VCL NAL unit of another coded
picture. Any picture parameter set NAL unit containing the value of pic_parameter_set_id for the active view picture
parameter set RBSP for a particular value of VOIdx less than VOIdxMax for a coded picture shall have the same content
as that of the active view picture parameter set RBSP for the particular value of VOIdx for this coded picture unless it
follows the last VCL NAL unit of this coded picture and precedes the first VCL NAL unit of another coded picture.


When a picture parameter set NAL unit with a particular value of pic_parameter_set_id is received, its content replaces
the content of the previous picture parameter set NAL unit, in decoding order, with the same value of pic_parameter_set_id
(when a previous picture parameter set NAL unit with the same value of pic_parameter_set_id was present in the
bitstream).

NOTE 2 – A decoder must be capable of simultaneously storing the contents of the picture parameter sets for all values of
pic_parameter_set_id. The content of the picture parameter set with a particular value of pic_parameter_set_id is overwritten when
a new picture parameter set NAL unit with the same value of pic_parameter_set_id is received.


An MVC sequence parameter set RBSP includes parameters that can be referred to by one or more picture parameter set
RBSPs or one or more buffering period SEI messages.


Each MVC sequence parameter set RBSP is initially considered not active at the start of the operation of the decoding
process. At most one MVC sequence parameter set RBSP is considered as the active MVC sequence parameter set RBSP
at any given moment during the operation of the decoding process, and when any particular MVC sequence parameter set
RBSP becomes the active MVC sequence parameter set RBSP, the previously-active MVC sequence parameter set RBSP
(if any) is deactivated.


In addition to the active MVC sequence parameter set RBSP, zero or more MVC sequence parameter set RBSPs may be
specifically active for view components (with a particular value of VOIdx less than VOIdxMax) that may be referred to
through inter-view prediction in decoding the view component with VOIdx equal to VOIdxMax. Such an MVC sequence
parameter set RBSP is referred to as the active view MVC sequence parameter set RBSP for the particular value of VOIdx.
The restrictions on active MVC sequence parameter set RBSPs also apply to active view MVC sequence parameter set
RBSPs for a particular value of VOIdx less than VOIdxMax.


For the following specification, the activating buffering period SEI message is specified as follows:


- If VOIdxMax is equal to VOIdxMin and the access unit contains a buffering period SEI message not included in an
MVC scalable nesting SEI message, this buffering period SEI message is the activating buffering period SEI message.


- Otherwise if VOIdxMax is not equal to VOIdxMin and the access unit contains a buffering period SEI message
included in an MVC scalable nesting SEI message and associated with the operation point being decoded, this
buffering period SEI message is the activating buffering period SEI message.


- Otherwise, the access unit does not contain an activating buffering period SEI message.


When a sequence parameter set RBSP (nal_unit_type is equal to 7) with a particular value of seq_parameter_set_id is not
already the active MVC sequence parameter set RBSP and it is referred to by activation of a picture parameter set RBSP
(using that value of seq_parameter_set_id) and the picture parameter set RBSP is activated by a coded slice NAL unit with
nal_unit_type equal to 1 or 5 (the picture parameter set RBSP becomes the active picture parameter set RBSP and
VOIdxMax is equal to VOIdxMin) and the access unit does not contain an activating buffering period SEI message, it is
activated. This sequence parameter set RBSP is called the active MVC sequence parameter set RBSP until it is deactivated
when another MVC sequence parameter set RBSP becomes the active MVC sequence parameter set RBSP. A sequence
parameter set RBSP, with that particular value of seq_parameter_set_id, shall be available to the decoding process prior to
its activation.


When a sequence parameter set RBSP (nal_unit_type is equal to 7) with a particular value of seq_parameter_set_id is not
already the active MVC sequence parameter set RBSP and it is referred to by an activating buffering period SEI message
(using that value of seq_parameter_set_id) that is not included in an MVC scalable nesting SEI message and VOIdxMax
is equal to VOIdxMin, it is activated. This sequence parameter set RBSP is called the active MVC sequence parameter set
RBSP until it is deactivated when another MVC sequence parameter set RBSP becomes the active MVC sequence
parameter set RBSP. A sequence parameter set RBSP, with that particular value of seq_parameter_set_id, shall be available
to the decoding process prior to its activation.


When a subset sequence parameter set RBSP (nal_unit_type is equal to 15) with a particular value of seq_parameter_set_id
is not already the active MVC sequence parameter set RBSP and it is referred to by activation of a picture parameter set





RBSP (using that value of seq_parameter_set_id) and the picture parameter set RBSP is activated by a coded slice MVC
extension NAL unit (nal_unit_type is equal to 20) with VOIdx equal to VOIdxMax (the picture parameter set RBSP
becomes the active picture parameter set RBSP) and the access unit does not contain an activating buffering period SEI
message, it is activated. This subset sequence parameter set RBSP is called the active MVC sequence parameter set RBSP
until it is deactivated when another MVC sequence parameter set RBSP becomes the active MVC sequence parameter set
RBSP. A subset sequence parameter set RBSP, with that particular value of seq_parameter_set_id, shall be available to the
decoding process prior to its activation.


When a subset sequence parameter set RBSP (nal_unit_type is equal to 15) with a particular value of seq_parameter_set_id
is not already the active MVC sequence parameter set RBSP and it is referred to by an activating buffering period SEI
message (using that value of seq_parameter_set_id) that is included in an MVC scalable nesting SEI message, it is
activated. This subset sequence parameter set RBSP is called the active MVC sequence parameter set RBSP until it is
deactivated when another MVC sequence parameter set RBSP becomes the active MVC sequence parameter set RBSP. A
subset sequence parameter set RBSP, with that particular value of seq_parameter_set_id, shall be available to the decoding
process prior to its activation.


NOTE 3 – The active MVC sequence parameter set RBSP is either a sequence parameter set RBSP or a subset sequence parameter
set RBSP. Sequence parameter set RBSPs are activated by coded slice NAL units with nal_unit_type equal to 1 or 5 or buffering
period SEI messages that are not included in an MVC scalable nesting SEI message. Subset sequence parameter sets are activated
by coded slice MVC extension NAL units (nal_unit_type equal to 20) or buffering period SEI messages that are included in an MVC
scalable nesting SEI message. A sequence parameter set RBSP and a subset sequence parameter set RBSP may have the same value
of seq_parameter_set_id.


For the following specification, the activating view buffering period SEI message for a particular value of VOIdx is
specified as follows:


- If the access unit contains one or more than one buffering period SEI message included in an MVC scalable nesting
SEI message and associated with an operation point for which the greatest VOIdx in the associated bitstream subset
is equal to the particular value of VOIdx, the first of these buffering period SEI messages, in decoding order, is the
activating view buffering period SEI message for the particular value of VOIdx.


- Otherwise, if the access unit contains a buffering period SEI message not included in an MVC scalable nesting SEI
message, this buffering period SEI message is the activating view buffering period SEI message for the particular
value of VOIdx equal to VOIdxMin.


- Otherwise, the access unit does not contain an activating buffering period SEI message for the particular value of
VOIdx.


When a sequence parameter set RBSP (nal_unit_type is equal to 7) with a particular value of seq_parameter_set_id is not
already the active view MVC sequence parameter set RBSP for VOIdx equal to VOIdxMin and VOIdxMax is greater than
VOIdxMin and it is referred to by activation of a picture parameter set RBSP (using that value of seq_parameter_set_id)
and the picture parameter set RBSP is activated by a coded slice NAL unit with nal_unit_type equal to 1 or 5 (the picture
parameter set RBSP becomes the active view picture parameter set RBSP for VOIdx equal to VOIdxMin), it is activated
for view components with VOIdx equal to VOIdxMin. This sequence parameter set RBSP is called the active view MVC
sequence parameter set RBSP for VOIdx equal to VOIdxMin until it is deactivated when another MVC sequence parameter
set RBSP becomes the active view MVC sequence parameter set RBSP for VOIdx equal to VOIdxMin or when decoding
an access unit with VOIdxMax equal to VOIdxMin, whichever is earlier. A sequence parameter set RBSP, with that
particular value of seq_parameter_set_id, shall be available to the decoding process prior to its activation.


When a sequence parameter set RBSP (nal_unit_type is equal to 7) with a particular value of seq_parameter_set_id is not
already the active view MVC sequence parameter set RBSP for VOIdx equal to VOIdxMin and VOIdxMax is greater than
VOIdxMin and it is referred to by an activating view buffering period SEI message (using that value of
seq_parameter_set_id) that is not included in an MVC scalable nesting SEI message, the sequence parameter set RBSP is
activated for view components with VOIdx equal to VOIdxMin. This sequence parameter set RBSP is called the active
view MVC sequence parameter set RBSP for VOIdx equal to VOIdxMin until it is deactivated when another MVC
sequence parameter set RBSP becomes the active view MVC sequence parameter set RBSP for VOIdx equal to VOIdxMin
or when decoding an access unit with VOIdxMax equal to VOIdxMin. A sequence parameter set RBSP, with that particular
value of seq_parameter_set_id, shall be available to the decoding process prior to its activation.


When a subset sequence parameter set RBSP (nal_unit_type is equal to 15) with a particular value of seq_parameter_set_id
is not already the active view MVC sequence parameter set RBSP for a particular value of VOIdx less than VOIdxMax
and it is referred to by activation of a picture parameter set RBSP (using that value of seq_parameter_set_id) and the picture
parameter set RBSP is activated by a coded slice MVC extension NAL unit (nal_unit_type equal to 20) with the particular
value of VOIdx (the picture parameter set RBSP becomes the active view picture parameter set RBSP for the particular
value of VOIdx), it is activated for view components with the particular value of VOIdx. This subset sequence parameter
set RBSP is called the active view MVC sequence parameter set RBSP for the particular value of VOIdx until it is
deactivated when another MVC sequence parameter set RBSP becomes the active view MVC sequence parameter set
RBSP for the particular value of VOIdx or when decoding an access unit with VOIdxMax less than or equal to the particular





value of VOIdx. A subset sequence parameter set RBSP, with that particular value of seq_parameter_set_id, shall be
available to the decoding process prior to its activation.


When a subset sequence parameter set RBSP (nal_unit_type is equal to 15) with a particular value of seq_parameter_set_id
is not already the active view MVC sequence parameter set RBSP for a particular value of VOIdx less than VOIdxMax
and it is referred to by an activating view buffering period SEI message (using that value of seq_parameter_set_id) that is
included in an MVC scalable nesting SEI message and associated with the particular value of VOIdx, this subset sequence
parameter set RBSP is activated for view components with the particular value of VOIdx. This subset sequence parameter
set RBSP is called the active view MVC sequence parameter set RBSP for the particular value of VOIdx until it is
deactivated when another MVC sequence parameter set RBSP becomes the active view MVC sequence parameter set
RBSP for the particular value of VOIdx or when decoding an access unit with VOIdxMax less than or equal to the particular
value of VOIdx. A subset sequence parameter set RBSP, with that particular value of seq_parameter_set_id, shall be
available to the decoding process prior to its activation.


An MVC sequence parameter set RBSP that includes a value of profile_idc not specified in Annex A or Annex H shall not
be referred to by activation of a picture parameter set RBSP as the active picture parameter set RBSP or as active view
picture parameter set RBSP (using that value of seq_parameter_set_id) or referred to by a buffering period SEI message
(using that value of seq_parameter_set_id). An MVC sequence parameter set RBSP including a value of profile_idc not
specified in Annex A or Annex H is ignored in the decoding for profiles specified in Annex A or Annex H.


It is a requirement of bitstream conformance that the following constraints are obeyed:


- For each particular value of VOIdx, all coded slice NAL units of a coded video sequence shall refer to the same value
of seq_parameter_set_id (via the picture parameter set RBSP that is referred to by the value of pic_parameter_set_id).


- The value of seq_parameter_set_id in a buffering period SEI message that is not included in an MVC scalable nesting
SEI message shall be identical to the value of seq_parameter_set_id in the picture parameter set RBSP that is referred
to by coded slice NAL units (with nal_unit_type equal to 1 or 5) (via the value of pic_parameter_set_id) in the same
access unit.


- The value of seq_parameter_set_id in a buffering period SEI message that is included in an MVC scalable nesting
SEI message and is associated with a particular value of VOIdx shall be identical to the value of seq_parameter_set_id
in the picture parameter set RBSP that is referred to by coded slice NAL units with the particular value of VOIdx (via
the value of pic_parameter_set_id) in the same access unit.


The active view MVC sequence parameter set RBSPs for different values of VOIdx may be the same MVC sequence
parameter set RBSP. The active MVC sequence parameter set RBSP and an active view MVC sequence parameter set
RBSP for a particular value of VOIdx may be the same MVC sequence parameter set RBSP.


When the active MVC sequence parameter set RBSP for a coded picture is a sequence parameter set RBSP, any sequence
parameter set RBSP in the coded video sequence containing this coded picture and with the value of seq_parameter_set_id
for the active MVC sequence parameter set RBSP shall have the same content as that of the active MVC sequence
parameter set RBSP.


When the active MVC sequence parameter set RBSP for a coded picture is a subset sequence parameter set RBSP, any
subset sequence parameter set RBSP in the coded video sequence containing this coded picture and with the value of
seq_parameter_set_id for the active MVC sequence parameter set RBSP shall have the same content as that of the active
MVC sequence parameter set RBSP.


For each particular value of VOIdx, the following applies:


- When the active view MVC sequence parameter set RBSP for a coded picture is a sequence parameter set RBSP, any
sequence parameter set RBSP in the coded video sequence containing this coded picture and with the value of
seq_parameter_set_id for the active view MVC sequence parameter set RBSP shall have the same content as that of
the active view MVC sequence parameter set RBSP.


- When the active view MVC sequence parameter set RBSP for a coded picture is a subset sequence parameter set
RBSP, any subset sequence parameter set RBSP in the coded video sequence containing this coded picture and with
the value of seq_parameter_set_id for the active view MVC sequence parameter set RBSP shall have the same content
as that of the active view MVC sequence parameter set RBSP.


NOTE 4 – If picture parameter set RBSPs or MVC sequence parameter set RBSPs are conveyed within the bitstream, these
constraints impose an order constraint on the NAL units that contain the picture parameter set RBSPs or MVC sequence parameter
set RBSPs, respectively. Otherwise (picture parameter set RBSPs or MVC sequence parameter set RBSPs are conveyed by other
means not specified in this Recommendation | International Standard), they must be available to the decoding process in a timely
fashion such that these constraints are obeyed.


When a sequence parameter set NAL unit with a particular value of seq_parameter_set_id is received, its content replaces
the content of the previous sequence parameter set NAL unit, in decoding order, with the same value of
seq_parameter_set_id (when a previous sequence parameter set NAL unit with the same value of seq_parameter_set_id





was present in the bitstream). When a subset sequence parameter set NAL unit with a particular value of
seq_parameter_set_id is received, its content replaces the content of the previous subset sequence parameter set NAL unit,
in decoding order, with the same value of seq_parameter_set_id (when a previous subset sequence parameter set NAL unit
with the same value of seq_parameter_set_id was present in the bitstream).

NOTE 5 – A decoder must be capable of simultaneously storing the contents of the sequence parameter sets and subset sequence
parameter sets for all values of seq_parameter_set_id. The content of the sequence parameter set with a particular value of
seq_parameter_set_id is overwritten when a new sequence parameter set NAL unit with the same value of seq_parameter_set_id is
received, and the content of the subset sequence parameter set with a particular value of seq_parameter_set_id is overwritten when
a new subset sequence parameter set NAL unit with the same value of seq_parameter_set_id is received.


When present, a sequence parameter set extension RBSP includes parameters having a similar function to those of a
sequence parameter set RBSP. For purposes of establishing constraints on the syntax elements of the sequence parameter
set extension RBSP and for purposes of determining activation of a sequence parameter set extension RBSP, the sequence
parameter set extension RBSP shall be considered part of the preceding sequence parameter set RBSP with the same value
of seq_parameter_set_id. When a sequence parameter set RBSP is present that is not followed by a sequence parameter
set extension RBSP with the same value of seq_parameter_set_id prior to the activation of the sequence parameter set
RBSP, the sequence parameter set extension RBSP and its syntax elements shall be considered not present for the active
MVC sequence parameter set RBSP. The contents of sequence parameter set extension RBSPs only apply when the base
view, which conforms to one or more of the profiles specified in Annex A, of a coded video sequence conforming to one
or more profiles specified in Annex H is decoded. Subset sequence parameter set RBSPs shall not be followed by a
sequence parameter set extension RBSP.


NOTE 6 – Sequence parameter sets extension RBSPs are not considered to be part of a subset sequence parameter set RBSP and
subset sequence parameter set RBSPs must not be followed by a sequence parameter set extension RBSP.


For view components with VOIdx equal to VOIdxMax, all constraints that are expressed on the relationship between the
values of the syntax elements (and the values of variables derived from those syntax elements) in MVC sequence parameter
sets and picture parameter sets and other syntax elements are expressions of constraints that apply only to the active MVC
sequence parameter set and the active picture parameter set. For view components with a particular value of VOIdx less
than VOIdxMax, all constraints that are expressed on the relationship between the values of the syntax elements (and the
values of variables derived from those syntax elements) in MVC sequence parameter sets and picture parameter sets and
other syntax elements are expressions of constraints that apply only to the active view MVC sequence parameter set and
the active view picture parameter set for the particular value of VOIdx. If any MVC sequence parameter set RBSP having
profile_idc equal to the value of one of the profile_idc values specified in Annex A or Annex H is present that is never
activated in the bitstream (i.e., it never becomes the active MVC sequence parameter set or an active view MVC sequence
parameter set), its syntax elements shall have values that would conform to the specified constraints if it were activated by
reference in an otherwise-conforming bitstream. If any picture parameter set RBSP is present that is never activated in the
bitstream (i.e., it never becomes the active picture parameter set or an active view picture parameter set), its syntax elements
shall have values that would conform to the specified constraints if it were activated by reference in an otherwiseconforming bitstream.


During operation of the decoding process (see clause H.8), for view components with VOIdx equal to VOIdxMax, the
values of parameters of the active picture parameter set and the active MVC sequence parameter set shall be considered in
effect. For view components with a particular value of VOIdx less than VOIdxMax, the values of the parameters of the
active view picture parameter set and the active view MVC sequence parameter set for the particular value of VOIdx shall
be considered in effect. For interpretation of SEI messages that apply to the entire access unit or the view component with
VOIdx equal to VOIdxMax, the values of the parameters of the active picture parameter set and the active MVC sequence
parameter set for the same access unit shall be considered in effect unless otherwise specified in the SEI message semantics.
For interpretation of SEI messages that apply to view components with a particular value of VOIdx less than VOIdxMax,
the values of the parameters of the active view picture parameter set and the active view MVC sequence parameter set for
the particular value of VOIdx for the same access unit shall be considered in effect unless otherwise specified in the SEI
message semantics.


**H.7.4.1.2.2** **Order of access units and association to coded video sequences**


The specification of clause 7.4.1.2.2 applies with the following modifications.


The first access unit of the bitstream shall only contain coded slice NAL units with nal_unit_type equal to 5 or non_idr_flag
equal to 0.


The order of NAL units and coded pictures and their association to access units is described in clause H.7.4.1.2.3.


**H.7.4.1.2.3** **Order of NAL units and coded pictures and association to access units**


The specification of clause 7.4.1.2.3 applies with the following modifications.

NOTE – Some bitstreams that conform to one or more profiles specified in this annex do not conform to any profile specified in
Annex A (prior to operation of the base view extraction process specified in clause H.8.5.4). As specified in clauses 7.4.1
and 7.4.1.2.3, for the profiles specified in Annex A, NAL units with nal_unit_type equal to 20 are classified as non-VCL NAL units





that must be preceded within each access unit by at least one NAL unit with nal_unit_type in the range of 1 to 5, inclusive. For this
reason, any bitstream that conforms to one or more profiles specified in this annex does not conform to any profile specified in
Annex A when it contains any of the following:

  - any access unit that does not contain any NAL units with nal_unit_type equal to 1 or 5, but contains one or more NAL units
with nal_unit_type equal to 6, 7, 8, 9, or 15;

  - any access unit in which one or more NAL units with nal_unit_type equal to 7, 8, or 15 is present after the last NAL unit in the
access unit with nal_unit_type equal to 1 or 5.


The association of VCL NAL units to primary or redundant coded pictures is specified in clause H.7.4.1.2.5.


The constraints for the detection of the first VCL NAL unit of a primary coded picture are specified in clause H.7.4.1.2.4.


The constraint expressed in clause 7.4.1.2.3 on the order of a buffering period SEI message is replaced by the following
constraints.


- When an SEI NAL unit containing a buffering period SEI message is present, the following applies:


   - If the buffering period SEI message is the only buffering period SEI message in the access unit and it is not
included in an MVC scalable nesting SEI message, the buffering period SEI message shall be the first SEI
message payload of the first SEI NAL unit in the access unit.


   - Otherwise (the buffering period SEI message is not the only buffering period SEI message in the access unit or
it is included in an MVC scalable nesting SEI message), the following constraints are specified:


      - When a buffering period SEI message that is not included in an MVC scalable nesting SEI message is
present, this buffering period SEI message shall be the only SEI message payload of the first SEI NAL unit
in the access unit.


      - An MVC scalable nesting SEI message that includes a buffering period SEI message shall not include any
other SEI messages and shall be the only SEI message inside the SEI NAL unit.


      - All SEI NAL units that precede an SEI NAL unit that contains an MVC scalable nesting SEI message with
a buffering period SEI message as payload in an access unit shall only contain buffering period SEI
messages or MVC scalable nesting SEI messages with a buffering period SEI message as payload.


Each prefix NAL unit shall be immediately followed by a NAL unit with nal_unit_type equal to 1 or 5.


**H.7.4.1.2.4** **Detection of the first VCL NAL unit of a primary coded picture**


This clause specifies constraints on VCL NAL unit syntax that are sufficient to enable the detection of the first VCL NAL
unit of each primary coded picture.


The first VCL NAL unit of the primary coded picture of the current access unit, in decoding order, shall be different from
the last VCL NAL unit of the primary coded picture of the previous access unit, in decoding order, in one or more of the
following ways:


- view_id of the first VCL NAL unit of the primary coded picture of the current access unit is different from view_id
of the last VCL NAL unit of the primary coded picture of the previous access unit, and VOIdx of the first VCL NAL
unit of the primary coded picture of the current access unit is less than VOIdx of the last VCL NAL unit of the primary
coded picture of the previous access unit.


- view_id of the first VCL NAL unit of the primary coded picture of the current access unit is equal to view_id of the
last VCL NAL unit of the primary coded picture of the previous access unit, and any of the conditions specified in
clause 7.4.1.2.4 is fulfilled.


**H.7.4.1.2.5** **Order of VCL NAL units and association to coded pictures**


Each VCL NAL unit is part of a coded picture.


Let voIdx be the value of VOIdx of any particular VCL NAL unit. The order of the VCL NAL units within a coded picture
is constrained as follows:


- For all VCL NAL units following this particular VCL NAL unit, the value of VOIdx shall be greater than or equal
to voIdx.


For each set of VCL NAL units within a view component, the following applies:


- If arbitrary slice order, as specified in Annex A or clause H.10, is allowed, coded slice NAL units of a view component
may have any order relative to each other.


- Otherwise (arbitrary slice order is not allowed), coded slice NAL units of a slice group shall not be interleaved with
coded slice NAL units of another slice group and the order of coded slice NAL units within a slice group shall be in





the order of increasing macroblock address for the first macroblock of each coded slice NAL unit of the same slice
group.


NAL units having nal_unit_type equal to 12 may be present in the access unit but shall not precede the first VCL NAL
unit of the primary coded picture within the access unit.


NAL units having nal_unit_type equal to 0 or in the range of 24 to 31, inclusive, which are unspecified, may be present in
the access unit but shall not precede the first VCL NAL unit of the primary coded picture within the access unit.


NAL units having nal_unit_type in the range of 21 to 23, inclusive, which are reserved, shall not precede the first VCL
NAL unit of the primary coded picture within the access unit (when specified in the future by ITU-T | ISO/IEC).


**H.7.4.2** **Raw byte sequence payloads and RBSP trailing bits semantics**


**H.7.4.2.1** **Sequence parameter set RBSP semantics**


The semantics specified in clause 7.4.2.1 apply.


**H.7.4.2.1.1** **Sequence parameter set data semantics**


For all syntax elements other than max_num_ref_frames, the semantics specified in clause 7.4.2.1.1 apply with the
substitution of MVC sequence parameter set for sequence parameter set. All constraints specified in clause 7.4.2.1.1 apply
only to the view components for which the MVC sequence parameter set is the active MVC sequence parameter set or the
active view MVC sequence parameter set as specified in clause H.7.4.1.2.1.


For each coded video sequence, the active MVC sequence parameter set and all active view MVC sequence parameter sets
(if any) shall have the same values of pic_width_in_mbs_minus1, pic_height_in_map_units_minus1, and
frame_mbs_only_flag.


When the seq_parameter_set_data( ) syntax structure is present in a subset sequence parameter set RBSP and
vui_parameters_present_flag is equal to 1, timing_info_present_flag shall be equal to 0, nal_hrd_parameters_present_flag
shall be equal to 0, vcl_hrd_parameters_present_flag shall be equal to 0, and pic_struct_present_flag shall be equal to 0.
The value of 1 for timing_info_present_flag, nal_hrd_parameters_present_flag, vcl_hrd_parameters_present_flag, and
pic_struct_present_flag for subset sequence parameter set RBSPs is reserved for future use by ITU-T | ISO/IEC. When
timing_info_present_flag is equal to 1, decoders shall ignore the values of the directly following num_units_in_tick,
time_scale, fixed_frame_rate_flag syntax elements. When nal_hrd_parameters_present_flag is equal to 1, decoders shall
ignore the value of the syntax elements in the directly following hrd_parameters( ) syntax structure. When
vcl_hrd_parameters_present_flag is equal to 1, decoders shall ignore the value of the syntax elements in the directly
following hrd_parameters( ) syntax structure.


If max_num_ref_frames is included in a sequence parameter set, the semantics specified in clause 7.4.2.1.1 apply.
Otherwise (max_num_ref_frames is included in a subset sequence parameter set), the following is specified:


**max_num_ref_frames** specifies the maximum number of short-term and long-term reference frames, complementary
reference field pairs, and non-paired reference fields that may be used by the decoding process for inter prediction of any
view component in the coded video sequence. max_num_ref_frames also determines the sliding window size of the sliding
window operation as specified in clause H.8.3. The value of max_num_ref_frames shall be in the range of 0 to 16,
inclusive.


**H.7.4.2.1.1.1** **Scaling list semantics**


The semantics specified in clause 7.4.2.1.1.1 apply.


**H.7.4.2.1.2** **Sequence parameter set extension RBSP semantics**


The semantics specified in clause 7.4.2.1.2 apply. Additionally, the following applies.


Sequence parameter set extension RBSPs can only follow sequence parameter set RBSPs in decoding order. Subset
sequence parameter set RBSPs shall not be followed by a sequence parameter set extension RBSP. The contents of
sequence parameter set extension RBSPs only apply when the base view, which conforms to one or more of the profiles
specified in Annex A, of a coded video sequence conforming to one or more profiles specified in Annex H is decoded.


**H.7.4.2.1.3** **Subset sequence parameter set RBSP semantics**


The semantics specified in clause 7.4.2.1.3 apply.


**H.7.4.2.1.4** **Sequence parameter set MVC extension semantics**


The sequence parameter set MVC extension specifies inter-view dependency relationships for the coded video sequence.
The sequence parameter set MVC extension also specifies level values for a subset of the operation points for the coded
video sequence. All sequence parameter set MVC extensions that are referred to by a coded video sequence shall be
identical.





Some views identified by view_id[ i ] may not be present in the coded video sequence.


NOTE 1 – Some views or temporal subsets described by the sequence parameter set MVC extension may have been removed from
the original coded video sequence, hence may not be present in the coded video sequence. However, the information in the sequence
parameter set MVC extension always applies to the remaining views and temporal subsets.


**num_views_minus1** plus 1 specifies the maximum number of coded views in the coded video sequence. The value of
num_view_minus1 shall be in the range of 0 to 1023, inclusive.


NOTE 2 – The actual number of views in the coded video sequence may be less than num_views_minus1 plus 1.


**view_id[** i **]** specifies the view_id of the view with VOIdx equal to i. The value of view_id[ i ] shall be in the range of 0 to
1023, inclusive.


**num_anchor_refs_l0[** i **]** specifies the number of view components for inter-view prediction in the initial reference picture
list RefPicList0 (which is derived as specified in clause H.8.2.1) in decoding anchor view components with VOIdx equal
to i. The value of num_anchor_refs_l0[ i ] shall not be greater than Min( 15, num_views_minus1 ). The value of
num_anchor_refs_l0[ 0 ] shall be equal to 0.


**anchor_ref_l0[** i **][** j **]** specifies the view_id of the j-th view component for inter-view prediction in the initial reference
picture list RefPicList0 (which is derived as specified in clause H.8.2.1) in decoding anchor view components with VOIdx
equal to i. The value of anchor_ref_l0[ i ][ j ] shall be in the range of 0 to 1023, inclusive.


**num_anchor_refs_l1[** i **]** specifies the number of view components for inter-view prediction in the initial reference picture
list RefPicList1 (which is derived as specified in clause H.8.2.1) in decoding anchor view components with VOIdx equal
to i. The value of num_anchor_refs_l1[ i ] shall not be greater than Min( 15, num_views_minus1 ). The value of
num_anchor_refs_l1[ 0 ] shall be equal to 0.


**anchor_ref_l1[** i **][** j **]** specifies the view_id of the j-th view component for inter-view prediction in the initial reference
picture list RefPicList1 (which is derived as specified in clause H.8.2.1) in decoding an anchor view component with
VOIdx equal to i. The value of anchor_ref_l1[ i ][ j ] shall be in the range of 0 to 1023, inclusive.


**num_non_anchor_refs_l0[** i **]** specifies the number of view components for inter-view prediction in the initial reference
picture list RefPicList0 (which is derived as specified in clause H.8.2.1) in decoding non-anchor view components with
VOIdx equal to i. The value of num_non_anchor_refs_l0[ i ] shall not be greater than Min( 15, num_views_minus1 ). The
value of num_non_anchor_refs_l0[ 0 ] shall be equal to 0.


**non_anchor_ref_l0[** i **][** j **]** specifies the view_id of the j-th view component for inter-view prediction in the initial
reference picture list RefPicList0 (which is derived as specified in clause H.8.2.1) in decoding non-anchor view
components with VOIdx equal to i. The value of non_anchor_ref_l0[ i ][ j ] shall be in the range of 0 to 1023, inclusive.


**num_non_anchor_refs_l1[** i **]** specifies the number of view components for inter-view prediction in the initial reference
picture list RefPicList1 (which is derived as specified in clause H.8.2.1) in decoding non-anchor view components with
VOIdx equal to i. The value of num_non_anchor_refs_l1[ i ] shall not be greater than Min( 15, num_views_minus1 ). The
value of num_non_anchor_refs_l1[ 0 ] shall be equal to 0.


**non_anchor_ref_l1[** i **][** j **]** specifies the view_id of the j-th view component for inter-view prediction in the initial
reference picture list RefPicList1 (which is derived as specified in clause H.8.2.1) in decoding non-anchor view
components with VOIdx equal to i. The value of non_anchor_ref_l1[ i ][ j ] shall be in the range of 0 to 1023, inclusive.


For any particular view with view_id equal to vId1 and VOIdx equal to vOIdx1 and another view with view_id equal to
vId2 and VOIdx equal to vOIdx2, when vId2 is equal to the value of one of non_anchor_ref_l0[ vOIdx1 ][ j ] for all j in
the range of 0 to num_non_anchor_refs_l0[ vOIdx1 ], exclusive, or one of non_anchor_ref_l1[ vOIdx1 ][ j ] for all j in the
range of 0 to num_non_anchor_refs_l1[ vOIdx1 ], exclusive, vId2 shall also be equal to the value of one of
anchor_ref_l0[ vOIdx1 ][ j ] for all j in the range of 0 to num_anchor_refs_l0[ vOIdx1 ], exclusive, or one of
anchor_ref_l1[ vOIdx1 ][ j ] for all j in the range of 0 to num_anchor_refs_l1[ vOIdx1 ], exclusive.


NOTE 3 – The inter-view dependency for non-anchor view components is a subset of that for anchor view components.


**num_level_values_signalled_minus1** plus 1 specifies the number of level values signalled for the coded video sequence.
The value of num_level_values_signalled_minus1 shall be in the range of 0 to 63, inclusive.


**level_idc[** i **]** specifies the i-th level value signalled for the coded video sequence.


**num_applicable_ops_minus1[** i **]** plus 1 specifies the number of operation points to which the level indicated by
level_idc[ i ] applies. The value of num_applicable_ops_minus1[ i ] shall be in the range of 0 to 1023, inclusive.


**applicable_op_temporal_id[** i **][** j **]** specifies the temporal_id of the j-th operation point to which the level indicated by
level_idc[ i ] applies.


**applicable_op_num_target_views_minus1[** i **][** j **]** plus 1 specifies the number of target output views for the j-th
operation point to which the level indicated by level_idc[ i ] applies. The value of





applicable_op_num_target_views_minus1[ i ][ j ] shall be in the range of 0 to 1023, inclusive.


**applicable_op_target_view_id[** i **][** j **][** k **]** specifies the k-th target output view for the j-th operation point to which the
level indicated by level_idc[ i ] applies. The value of applicable_op_target_view_id[ i ][ j ][ k ] shall be in the range of 0
to 1023, inclusive.


Let maxTId be the greatest temporal_id of all NAL units in the coded video sequence, and vId be view_id of any view in
the coded video sequence. There shall be one set of applicable_op_temporal_id[ i ][ j ],
applicable_op_num_target_views_minus1[ i ][ j ], and applicable_op_target_view_id[ i ][ j ][ k ], for any i and j and all k
for the i and j, in which applicable_op_temporal_id[ i ][ j ] is equal to maxTId,
applicable_op_num_target_views_minus1[ i ][ j ] is equal to 0, and applicable_op_target_view_id[ i ][ j ][ k ] is equal to
vId.


NOTE 4 – The above constraint ensures that the level that applies to each operation point consisting of only one target output view
with the greatest highest temporal_id in the coded video sequence is signalled by one of the level_idc[ i ] for all i.


NOTE 5 – Some operation points identified by applicable_op_temporal_id[ i ][ j ],
applicable_op_num_target_views_minus1[ i ][ j ], and applicable_op_target_view_id[ i ][ j ][ k ], for all i, j, and k, may not be
present in the coded video sequence.


**applicable_op_num_views_minus1[** i **][** j **]** plus 1 specifies the number of views required for decoding the target output
views corresponding to the j-th operation point to which the level indicated by level_idc[ i ] applies. The number of views
specified by applicable_op_num_views_minus1 includes the target output views and the views that the target output views
depend on as specified by the sub-bitstream extraction process in clause H.8.5 with tIdTarget equal to
applicable_op_temporal_id[ i ][ j ] and viewIdTargetList equal to the list of applicable_op_target_view_id[ i ][ j ][ k ] for
all k in the range of 0 to applicable_op_num_target_views_minus1[ i ][ j ], inclusive, as inputs. The value of
applicable_op_num_views_minus1[ i ][ j ] shall be in the range of 0 to 1023, inclusive.


**mfc_format_idc** specifies the frame packing arrangement type for view components of the base view and the
corresponding frame packing arrangement type for view components in the non-base view. The semantics of
mfc_format_idc equal to 0 and 1 are specified by Table H-1.


In bitstreams conforming to this version of this Specification, the value of mfc_format_idc shall be equal to 0 or 1. Values
of mfc_format_idc in the range of 2..63 are reserved for future use by ITU-T | ISO/IEC. Decoders shall ignore the coded
video sequence when the value of mfc_format_idc is greater than 1.


**Table H-1 – Association between frame packing arrangement type and syntax elements**










|mfc_format_idc|Constraints on the frame packing arrangement<br>SEI message syntax for view components of the<br>base view|Corresponding frame packing arrangement<br>type inferred for view components of the non-<br>base view|
|---|---|---|
|0|frame_packing_arrangement_type shall be equal to<br>3 (side-by-side)|frame_packing_arrangement_type equal to 4 (top-<br>bottom)|
|1|frame_packing_arrangement_type shall be equal to<br>4 (top-bottom)|frame_packing_arrangement_type equal to 3<br>(side-by-side)|



**default_grid_position_flag** equal to 0 specifies that the syntax elements view0_grid_position_x, view0_grid_position_y,
view1_grid_position_x, and view1_grid_position_y are present. default_grid_position_flag equal to 1 specifies that
view0_grid_position_x, view0_grid_position_y, view1_grid_position_x, and view1_grid_position_y are not present.


**view0_grid_position_x** has the same semantics as specified in clause D.2.26 for the frame0_grid_position_x syntax
element. The value of view0_grid_position_x shall be equal to 4, 8 or 12.


**view0_grid_position_y** has the same semantics as specified in clause D.2.26 for the frame0_grid_position_y syntax
element. The value of view0_grid_position_y shall be equal to 4, 8 or 12.


**view1_grid_position_x** has the same semantics as specified in clause D.2.26 for the frame1_grid_position_x syntax
element. The value of view1_grid_position_x shall be equal to 4, 8 or 12.


**view1_grid_position_y** has the same semantics as specified in clause D.2.26 for the frame1_grid_position_y syntax
element. The value of view1_grid_position_y shall be equal to 4, 8 or 12.


When default_grid_position_flag is equal to 1, the values of view0_grid_position_x, view0_grid_position_y,
view1_grid_position_x, and view1_grid_position_y are inferred as follows:

- If mfc_format_idc is equal to 0, the following applies:

  - view0_grid_position_x is inferred to be equal to 4.

  - view0_grid_position_y is inferred to be equal to 8.

  - view1_grid_position_x is inferred to be equal to 12.





  - view1_grid_position_y is inferred to be equal to 8.

- Otherwise (mfc_format_idc is equal to 1), the following applies:

  - view0_grid_position_x is inferred to be equal to 8.

  - view0_grid_position_y is inferred to be equal to 4.

  - view1_grid_position_x is inferred to be equal to 8.

  - view1_grid_position_y is inferred to be equal to 12.


When mfc_format_idc is present, the following applies:

- It is a requirement of bitstream conformance that each coded view component of the base view shall be associated

with a frame packing arrangement SEI message for which all of the following constraints apply:

  - frame_packing_arrangement_type is equal to the value specified by Table H-1for view components of the base
view.

  - quincunx_sampling_flag is equal to 0.

  - content_interpretation_type is equal to 1.

  - spatial_flipping_flag is equal to 0.

  - frame0_grid_position_x, frame0_grid_position_y, frame1_grid_position_x, and frame1_grid_position_y are
equal to view0_grid_position_x, view0_grid_position_y, view1_grid_position_x, and view1_grid_position_y,
respectively.

- It is a requirement of bitstream conformance that no frame packing arrangement SEI message shall be associated with

any view component of a non-base view. For each view component of a non-base view, a frame packing arrangement
is inferred as follows:

  - frame_packing_arrangement_type is equal to the value specified by Table H-1for the view components of the
non-base view.

  - quincunx_sampling_flag is equal to 0.

  - content_interpretation_type is equal to 1.

  - spatial_flipping_flag is equal to 0.


NOTE 6 – These constraints also apply to cases where a coded view component would be associated with a frame packing
arrangement SEI message that is present in an access unit that is earlier in decoding order than the access unit containing the coded
view component.


**rpu_filter_enabled_flag** equal to 1 specifies that a downsampling filter process and an upsampling filter process are used
to generate each colour component of an inter-view prediction reference. rpu_filter_enabled_flag equal to 0 specifies that
all sample values for each colour component of an inter-view prediction reference are set equal to 128.


**rpu_field_processing_flag** equal to 0 specifies that each inter-view prediction reference with field_pic_flag equal to 0 is
processed as a frame when processed by the RPU. rpu_field_processing_flag equal to 1 specifies that each inter-view
prediction reference with field_pic_flag equal to 0 is processed as two fields when processed by the RPU. When not
present, the value of rpu_field_processing_flag is inferred to be equal to 0.


**H.7.4.2.2** **Picture parameter set RBSP semantics**


The semantics specified in clause 7.4.2.2 apply with substituting MVC sequence parameter set for sequence parameter set.
All constraints specified in clause 7.4.2.2 apply only to the view components for which the picture parameter set is the
active picture parameter set or the active view picture parameter set as specified in clause H.7.4.1.2.1.


**weighted_bipred_idc** has the same semantics as specified in clause 7.4.2.2 with the following modification.


When there is at least one inter-view prediction reference, which belongs to the same access unit as the current view
component, in RefPicList0 or RefPicList1, weighted_bipred_idc shall not be equal to 2.


**H.7.4.2.3** **Supplemental enhancement information RBSP semantics**


The semantics specified in clause 7.4.2.3 apply.


**H.7.4.2.3.1** **Supplemental enhancement information message semantics**


The semantics specified in clause 7.4.2.3.1 apply.


**H.7.4.2.4** **Access unit delimiter RBSP semantics**


The semantics specified in clause 7.4.2.4 apply.





NOTE – The value of primary_pic_type applies to the slice_type values in all slice headers of the primary coded picture, including
the slice_type syntax elements in all NAL units with nal_unit_type equal to 1, 5, or 20. NAL units with nal_unit_type equal to 2 are
not present in bitstreams conforming to any of the profiles specified in this annex.


**H.7.4.2.5** **End of sequence RBSP semantics**


The semantics specified in clause 7.4.2.5 apply.


**H.7.4.2.6** **End of stream RBSP semantics**


The semantics specified in clause 7.4.2.6 apply.


**H.7.4.2.7** **Filler data RBSP semantics**


The semantics specified in clause 7.4.2.7 apply with the following addition.


Filler data NAL units shall be considered to contain the syntax elements priority_id, view_id, and temporal_id with values
that are inferred as follows:


1. Let prevMvcNalUnit be the most recent NAL unit in decoding order that has nal_unit_type equal to 14 or 20.

NOTE – The most recent NAL unit in decoding order with nal_unit_type equal to 14 or 20 always belongs to the same access
unit as the filler data NAL unit.


2. The values of priority_id, view_id, and temporal_id for the filler data NAL unit are inferred to be equal to the

values of priority_id, view_id, and temporal_id, respectively, of the NAL unit prevMvcNalUnit.


**H.7.4.2.8** **Slice layer without partitioning RBSP semantics**


The semantics specified in clause 7.4.2.8 apply.


**H.7.4.2.9** **Slice data partition RBSP semantics**


Slice data partition syntax is not present in bitstreams conforming to one or more of the profiles specified in Annex H.


**H.7.4.2.10RBSP slice trailing bits semantics**


The semantics specified in clause 7.4.2.10 apply with the following modifications.


Let NumBytesInVclNALunits be the sum of the values of NumBytesInNALunit for all VCL NAL units of a view
component and let BinCountsInNALunits be the number of times that the parsing process function DecodeBin( ), specified
in clause 9.3.3.2, is invoked to decode the contents of all VCL NAL units of the view component. When
entropy_coding_mode_flag is equal to 1, it is a requirement of bitstream conformance that BinCountsInNALunits shall
not exceed ( 32 ÷ 3 ) * NumBytesInVclNALunits + ( RawMbBits * PicSizeInMbs ) ÷ 32.

NOTE – The constraint on the maximum number of bins resulting from decoding the contents of the slice layer NAL units of a view
component can be met by inserting a number of cabac_zero_word syntax elements to increase the value of
NumBytesInVclNALunits. Each cabac_zero_word is represented in a NAL unit by the three-byte sequence 0x000003 (as a result of
the constraints on NAL unit contents that result in requiring inclusion of an emulation_prevention_three_byte for each
cabac_zero_word).


**H.7.4.2.11RBSP trailing bits semantics**


The semantics specified in clause 7.4.2.11 apply.


**H.7.4.2.12Prefix NAL unit RBSP semantics**


The semantics specified in clause 7.4.2.12 apply.


**H.7.4.2.13Slice layer extension RBSP semantics**


The semantics specified in clause 7.4.2.13 apply.


**H.7.4.3** **Slice header semantics**


The semantics specified in clause 7.4.3 apply with the following modifications.


All constraints specified in clause 7.4.3 apply only to the view components with the same value of VOIdx.


The value of the following MVC sequence parameter set syntax elements shall be the same across all coded slice NAL
units of an access unit: chroma_format_idc.


The value of the following slice header syntax elements shall be the same across all coded slice NAL units of an access
unit: field_pic_flag and bottom_field_flag.


**frame_num** is used as an identifier for view components and is represented by log2_max_frame_num_minus4 + 4 bits in
the bitstream.





frame_num is constrained as specified in clause 7.4.3 where this constraint applies to view components with view_id equal
to the current value of view_id.


**direct_spatial_mv_pred_flag** has the same semantics as specified in clause 7.4.3 with the following modification.


When RefPicList1[ 0 ] is an inter-view reference component or an inter-view only reference component, which belongs to
the same access unit as the current view component, direct_spatial_mv_pred_flag shall be equal to 1.


**num_ref_idx_l0_active_minus1** has the same semantics as specified in clause 7.4.3 with the following modification.


The range of num_ref_idx_l0_active_minus1 is specified as follows:


- If num_views_minus1 is equal to 1, the following applies:


   - If field_pic_flag is equal to 0, num_ref_idx_l0_active_minus1 shall be in the range of 0 to 7, inclusive. When
MbaffFrameFlag is equal to 1, num_ref_idx_l0_active_minus1 is the maximum index value for the decoding of
frame macroblocks and 2 * num_ref_idx_l0_active_minus1 + 1 is the maximum index value for the decoding of
field macroblocks.


   - Otherwise (field_pic_flag is equal to 1), num_ref_idx_l0_active_minus1 shall be in the range of 0 to 15, inclusive.


- Otherwise (num_views_minus1 is greater than 1), the following applies:


   - If field_pic_flag is equal to 0, num_ref_idx_l0_active_minus1 shall be in the range of 0 to 15, inclusive. When
MbaffFrameFlag is equal to 1, num_ref_idx_l0_active_minus1 is the maximum index value for the decoding of
frame macroblocks and 2 * num_ref_idx_l0_active_minus1 + 1 is the maximum index value for the decoding of
field macroblocks.


   - Otherwise (field_pic_flag is equal to 1), num_ref_idx_l0_active_minus1 shall be in the range of 0 to 31, inclusive.


**num_ref_idx_l1_active_minus1** has the same semantics as specified in clause 7.4.3 with the following modification.


The range of num_ref_idx_l1_active_minus1 is constrained as specified in the semantics for
num_ref_idx_l0_active_minus1 in this clause with l0 and list 0 replaced by l1 and list 1, respectively.


**H.7.4.3.1** **Reference picture list modification semantics**


The semantics specified in clause 7.4.3.1 apply with the following modifications.


**ref_pic_list_modification_flag_l0** equal to 1 specifies that the syntax element modification_of_pic_nums_idc is present
for specifying reference picture list 0. ref_pic_list_modification_flag_l0 equal to 0 specifies that this syntax element is not
present.


When ref_pic_list_modification_flag_l0 is equal to 1, the number of times that modification_of_pic_nums_idc is not equal
to 3 following ref_pic_list_modification_flag_l0 shall not exceed num_ref_idx_l0_active_minus1 + 1.


When RefPicList0[ num_ref_idx_l0_active_minus1 ] in the initial reference picture list produced as specified in
clause H.8.2.1 is equal to "no reference picture", ref_pic_list_modification_flag_l0 shall be equal to 1 and
modification_of_pic_nums_idc shall not be equal to 3 until RefPicList0[ num_ref_idx_l0_active_minus1 ] in the modified
list produced as specified in clause H.8.2.2 is not equal to "no reference picture".


**ref_pic_list_modification_flag_l1** equal to 1 specifies that the syntax element modification_of_pic_nums_idc is present
for specifying reference picture list 1. ref_pic_list_modification_flag_l1 equal to 0 specifies that this syntax element is not
present.


When ref_pic_list_modification_flag_l1 is equal to 1, the number of times that modification_of_pic_nums_idc is not equal
to 3 following ref_pic_list_modification_flag_l1 shall not exceed num_ref_idx_l1_active_minus1 + 1.


When decoding a slice with slice_type equal to 1 or 6 and RefPicList1[ num_ref_idx_l1_active_minus1 ] in the initial
reference picture list produced as specified in clause H.8.2.1 is equal to "no reference picture",
ref_pic_list_modification_flag_l1 shall be equal to 1 and modification_of_pic_nums_idc shall not be equal to 3 until
RefPicList1[ num_ref_idx_l1_active_minus1 ] in the modified list produced as specified in clause H.8.2.2 is not equal to
"no reference picture".


**H.7.4.3.1.1** **Reference picture list MVC modification semantics**


The semantics specified in clause 7.4.3.1 apply with the following modified semantics of modification_of_pic_nums_idc.
In addition, the semantics of abs_diff_view_idx_minus1 specified below apply.


**modification_of_pic_nums_idc** together with abs_diff_pic_num_minus1, long_term_pic_num, or
abs_diff_view_idx_minus1 specifies which of the reference pictures or inter-view only reference components are
re-mapped. The values of modification_of_pic_nums_idc are specified in Table H-2. The value of the first
modification_of_pic_nums_idc that follows immediately after ref_pic_list_modification_flag_l0 or





ref_pic_list_modification_flag_l1 shall not be equal to 3.


**Table H-2 – modification_of_pic_nums_idc operations for modification of reference picture lists**

|modification_of_pic_nums_idc|Modification specified|
|---|---|
|0|abs_diff_pic_num_minus1 is present and corresponds to a difference to subtract from<br>a picture number prediction value|
|1|abs_diff_pic_num_minus1 is present and corresponds to a difference to add to a picture<br>number prediction value|
|2|long_term_pic_num is present and specifies the long-term picture number for a<br>reference picture|
|3|End loop for modification of the initial reference picture list|
|4|abs_diff_view_idx_minus1 is present and corresponds to a difference to subtract from<br>a prediction value of the reference view index|
|5|abs_diff_view_idx_minus1 is present and corresponds to a difference to add to a<br>prediction value of the reference view index|



**abs_diff_view_idx_minus1** plus 1 specifies the absolute difference between the reference view index to put to the current
index in the reference picture list and the prediction value of the reference view index.


Let currVOIdx be the VOIdx of the current view component, and let intViewIdx be the reference view index of the target
inter-view prediction reference to put to the current index in RefPicListX (X is 0 or 1). Depending on whether the current
view component is an anchor view component, the following applies:


- If the current view component is an anchor view component, the view_id of the target inter-view prediction reference
is equal to anchor_ref_lX[ currVOIdx ][ intViewIdx ]. For anchor view components with VOIdx equal to currVOIdx,
abs_diff_view_idx_minus1 shall be in the range of 0 to Max(0, num_anchor_refs_lX[ currVOIdx ] − 1 ), inclusive.


- Otherwise (the current view component is not an anchor view component), the view_id of the target inter-view
prediction reference is equal to non_anchor_ref_lX[ currVOIdx ][ intViewIdx ]. For non-anchor view components
with VOIdx equal to currVOIdx, abs_diff_view_idx_minus1 shall be in the range of 0 to Max(0,
num_non_anchor_refs_lX[ currVOIdx ] − 1), inclusive.


The allowed values of abs_diff_view_idx_minus1 are further restricted as specified in clause H.8.2.2.3.


**H.7.4.3.2** **Prediction weight table semantics**


The semantics specified in clause 7.4.3.2 apply.


**H.7.4.3.3** **Decoded reference picture marking semantics**


The semantics specified in clause 7.4.3.3 apply to each view independently, with "sequence parameter set" being replaced
by "MVC sequence parameter set", and "primary coded picture" being replaced by "view component of the primary coded
picture".


**H.7.4.4** **Slice data semantics**


The semantics specified in clause 7.4.4 apply.


**H.7.4.5** **Macroblock layer semantics**


The semantics specified in clause 7.4.5 apply.


**H.7.4.5.1** **Macroblock prediction semantics**


The semantics specified in clause 7.4.5.1 apply.


**H.7.4.5.2** **Sub-macroblock prediction semantics**


The semantics specified in clause 7.4.5.2 apply.


**H.7.4.5.3** **Residual data semantics**


The semantics specified in clause 7.4.5.3 apply.





**H.7.4.5.3.1** **Residual luma semantics**


The semantics specified in clause 7.4.5.3.1 apply.


**H.7.4.5.3.2** **Residual block CAVLC semantics**


The semantics specified in clause 7.4.5.3.2 apply.


**H.7.4.5.3.3** **Residual block CABAC semantics**


The semantics specified in clause 7.4.5.3.3 apply.
