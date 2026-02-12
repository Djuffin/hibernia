**I.7** **Syntax and semantics**


This clause specifies syntax and semantics for coded video sequences that conform to one or more of the profiles specified
in this annex.


**I.7.1** **Method of specifying syntax in tabular form**


The specifications in clause H.7.1 apply.


**I.7.2** **Specification of syntax functions, categories, and descriptors**


The specifications in clause H.7.2 apply.


**I.7.3** **Syntax in tabular form**


**I.7.3.1** **NAL unit syntax**


The syntax table is specified in clause H.7.3.1.


**I.7.3.1.1** **NAL unit header MVC extension syntax**


The syntax table is specified in clause H.7.3.1.1.


**I.7.3.2** **Raw byte sequence payloads and RBSP trailing bits syntax**


**I.7.3.2.1** **Sequence parameter set RBSP syntax**


The syntax table is specified in clause H.7.3.2.1.


**I.7.3.2.1.1** **Sequence parameter set data syntax**


The syntax table is specified in clause H.7.3.2.1.1.


**I.7.3.2.1.1.1** **Scaling list syntax**


The syntax table is specified in clause H.7.3.2.1.1.1.


**I.7.3.2.1.2** **Sequence parameter set extension RBSP syntax**


The syntax table is specified in clause H.7.3.2.1.2.


**I.7.3.2.1.3** **Subset sequence parameter set RBSP syntax**


The syntax table is specified in clause H.7.3.2.1.3.


**I.7.3.2.1.4** **Sequence parameter set MVC extension syntax**


The syntax table is specified in clause H.7.3.2.1.4.





|1.5 Sequence parameter set MVCD extension syntax|Col2|Col3|
|---|---|---|
|seq_parameter_set_mvcd_extension( ) {|**C **|**Descriptor**|
|<br>**num_views_minus1**|0|ue(v)|
|<br>for( i = 0, NumDepthViews = 0; i <= num_views_minus1; i++ ) {|||
|<br> <br>**view_id[**i** ]**|0|ue(v)|
|<br> <br>**depth_view_present_flag[**i** ]**|0|u(1)|
|<br> <br>DepthViewId[ NumDepthViews ] = view_id[ i ]|||
|<br> <br>NumDepthViews += depth_view_present_flag[ i ]|||
|<br> <br>**texture_view_present_flag[**i** ]**|0|u(1)|
|<br>}|||
|<br>for( i = 1; i <= num_views_minus1; i++ )|||
|<br> <br>if( depth_view_present_flag[ i ] ) {|||
|<br> <br> <br>**num_anchor_refs_l0[**i** ]**|0|ue(v)|
|<br> <br> <br>for( j = 0; j < num_anchor_refs_l0[ i ]; j++ )|||
|<br> <br> <br>** anchor_ref_l0[**i** ][**j** ]**|0|ue(v)|
|<br> <br> <br>**num_anchor_refs_l1[**i** ]**|0|ue(v)|
|<br> <br> <br>for( j = 0; j < num_anchor_refs_l1[ i ]; j++ )|||
|<br> <br> <br>** anchor_ref_l1[**i** ][**j** ]**|0|ue(v)|
|<br> <br>}|||
|<br>for( i = 1; i <= num_views_minus1; i++ )|||
|<br> <br>if( depth_view_present_flag[ i ] ) {|||
|<br> <br> <br>**num_non_anchor_refs_l0[**i** ]**|0|ue(v)|
|<br> <br> <br>for( j = 0; j < num_non_anchor_refs_l0[ i ]; j++ )|||
|<br> <br> <br>** non_anchor_ref_l0[**i** ][**j** ]**|0|ue(v)|
|<br> <br> <br>**num_non_anchor_refs_l1[**i** ]**|0|ue(v)|
|<br> <br> <br>for( j = 0; j < num_non_anchor_refs_l1[ i ]; j++ )|||
|<br> <br> <br>** non_anchor_ref_l1[**i** ][**j** ]**|0|ue(v)|
|<br> <br>}|||
|<br>**num_level_values_signalled_minus1**|0|ue(v)|
|<br>for( i = 0; i <= num_level_values_signalled_minus1; i++ ) {|||
|<br> <br>**level_idc[**i** ]**|0|u(8)|
|<br> <br>**num_applicable_ops_minus1[**i** ]**|0|ue(v)|
|<br> <br>for( j = 0; j <= num_applicable_ops_minus1[ i ]; j++ ) {|||
|<br> <br> <br>**applicable_op_temporal_id[**i** ][**j** ]**|0|u(3)|
|<br> <br> <br>**applicable_op_num_target_views_minus1[**i** ][**j** ]**|0|ue(v)|
|<br> <br> <br>for( k = 0; k <= applicable_op_num_target_views_minus1[ i ][ j ];<br> <br> <br> <br> k++ ) {|||
|<br> <br> <br>** applicable_op_target_view_id[**i** ][**j** ][**k** ]**|0|ue(v)|
|<br> <br> <br>** applicable_op_depth_flag[**i** ][**j** ][**k** ]**|0|u(1)|
|<br> <br> <br>** applicable_op_texture_flag[**i** ][**j** ][**k** ]**|0|u(1)|
|<br> <br> <br>}|||
|<br> <br> <br>**applicable_op_num_texture_views_minus1[**i** ][**j** ]**|0|ue(v)|
|<br> <br> <br>**applicable_op_num_depth_views[**i** ][**j** ]**|0|ue(v)|
|<br> <br>}|||
|<br>}|||
|<br>**mvcd_vui_parameters_present_flag**|0|u(1)|
|<br>if( mvcd_vui_parameters_present_flag = = 1 )|||
|<br> <br>mvcd_vui_parameters_extension( )|||
|<br>**texture_vui_parameters_present_flag**|0|u(1)|
|<br>if( texture_vui_parameters_present_flag = = 1 )|||
|<br> <br>mvc_vui_parameters_extension( )|0||
|}|||





**I.7.3.2.2** **Picture parameter set RBSP syntax**


The syntax table is specified in clause H.7.3.2.2.


**I.7.3.2.3** **Supplemental enhancement information RBSP syntax**


The syntax table is specified in clause H.7.3.2.3.


**I.7.3.2.3.1** **Supplemental enhancement information message syntax**


The syntax table is specified in clause H.7.3.2.3.1.


**I.7.3.2.4** **Access unit delimiter RBSP syntax**


The syntax table is specified in clause H.7.3.2.4.


**I.7.3.2.5** **End of sequence RBSP syntax**


The syntax table is specified in clause H.7.3.2.5.


**I.7.3.2.6** **End of stream RBSP syntax**


The syntax table is specified in clause H.7.3.2.6.


**I.7.3.2.7** **Filler data RBSP syntax**


The syntax table is specified in clause H.7.3.2.7.


**I.7.3.2.8** **Slice layer without partitioning RBSP syntax**


The syntax table is specified in clause H.7.3.2.8.


**I.7.3.2.9** **Slice data partition RBSP syntax**


Slice data partition syntax is not present in coded video sequences conforming to one or more of the profiles specified in
this annex.


**I.7.3.2.10** **RBSP slice trailing bits syntax**


The syntax table is specified in clause H.7.3.2.10.


**I.7.3.2.11** **RBSP trailing bits syntax**


The syntax table is specified in clause H.7.3.2.11.


**I.7.3.2.12** **Prefix NAL unit RBSP syntax**


The syntax table is specified in clause H.7.3.2.12.


**I.7.3.2.13** **Slice layer extension RBSP syntax**


The syntax table is specified in clause H.7.3.2.13.


**I.7.3.3** **Slice header syntax**


The syntax table is specified in clause H.7.3.3.


**I.7.3.3.1** **Reference picture list modification syntax**


The syntax table is specified in clause H.7.3.3.1.


**I.7.3.3.1.1** **Reference picture list MVC modification syntax**


The syntax table is specified in clause H.7.3.3.1.1


**I.7.3.3.2** **Prediction weight table syntax**


The syntax table is specified in clause H.7.3.3.2.


**I.7.3.3.3** **Decoded reference picture marking syntax**


The syntax table is specified in clause H.7.3.3.3.


**I.7.3.4** **Slice data syntax**


The syntax table is specified in clause H.7.3.4.


**I.7.3.5** **Macroblock layer syntax**


The syntax table is specified in clause H.7.3.5.





**I.7.3.5.1** **Macroblock prediction syntax**


The syntax table is specified in clause H.7.3.5.1.


**I.7.3.5.2** **Sub-macroblock prediction syntax**


The syntax table is specified in clause H.7.3.5.2.


**I.7.3.5.3** **Residual data syntax**


The syntax table is specified in clause H.7.3.5.3.


**I.7.3.5.3.1** **Residual luma syntax**


The syntax table is specified in clause H.7.3.5.3.1.


**I.7.3.5.3.2** **Residual block CAVLC syntax**


The syntax table is specified in clause H.7.3.5.3.2.


**I.7.3.5.3.3** **Residual block CABAC syntax**


The syntax table is specified in clause H.7.3.5.3.3.


**I.7.4** **Semantics**


Semantics associated with the syntax structures and syntax elements within these structures (in clause I.7.3 and in
clause H.7.3 by reference in clause I.7.3) are specified in this clause and by reference to clause I.7.4. When the semantics
of a syntax element are specified using a table or a set of tables, any values that are not specified in the table(s) shall not
be present in the bitstream unless otherwise specified in this Recommendation | International Standard.


**I.7.4.1** **NAL unit semantics**


The semantics for the syntax elements in clause I.7.3.1 are specified in clause H.7.4.1.


**I.7.4.1.1** **NAL unit header MVC extension semantics**


The semantics for the syntax elements in clause I.7.3.1.1 are specified in clause H.7.4.1.1.


**I.7.4.1.2** **Order of NAL units and association to coded pictures, access units, and video sequences**


This clause specifies constraints on the order of NAL units in the bitstream. Any order of NAL units in the bitstream
obeying these constraints is referred to in the text as the decoding order of NAL units. Within a NAL unit, the syntax in
clauses 7.3, D.1, E.1, H.7.3, H.13.1, H.14.1, I.13.1 and I.14.1 specifies the decoding order of syntax elements. Decoders
shall be capable of receiving NAL units and their syntax elements in decoding order.


**I.7.4.1.2.1** **Order of MVCD sequence parameter set RBSPs and picture parameter set RBSPs and their activation**


NOTE 1 – The sequence and picture parameter set mechanism decouples the transmission of infrequently changing information
from the transmission of coded macroblock data. Sequence and picture parameter sets may, in some applications, be conveyed "outof-band" using a reliable transport mechanism.


A picture parameter set RBSP includes parameters that can be referred to by the coded slice NAL units of one or more
texture view or depth view components of one or more coded pictures.


Each picture parameter set RBSP is initially considered not active at the start of the operation of the decoding process. At
most one picture parameter set RBSP is considered as the active picture parameter set RBSP at any given moment during
the operation of the decoding process, and when any particular picture parameter set RBSP becomes the active picture
parameter set RBSP, the previously-active picture parameter set RBSP (if any) is deactivated.


In addition to the active picture parameter set RBSP, zero or more picture parameter set RBSPs may be specifically active
for texture view components (with a particular value of VOIdx less than or equal to VOIdxMax) that belong to the target
output views or that may be referred to through inter-view prediction in decoding texture view components belonging to
the target output views. Such a picture parameter set RBSP is referred to as the active texture picture parameter set RBSP
for the particular value of VOIdx. The restrictions on active picture parameter set RBSPs also apply to active texture picture
parameter set RBSPs for a particular value of VOIdx.


Furthermore, zero or more picture parameter set RBSPs may be specifically active for depth view components (with a
particular value of VOIdx less than VOIdxMax) that belong to the target output views or that may be referred to through
inter-view prediction in decoding depth view components belonging to the target output views. Such a picture parameter
set RBSP is referred to as the active depth picture parameter set RBSP for the particular value of VOIdx. The restrictions
on active picture parameter set RBSPs also apply to active depth picture parameter set RBSPs for a particular value of
VOIdx less than VOIdxMax.





When a picture parameter set RBSP (with a particular value of pic_parameter_set_id) is not the active picture parameter
set RBSP and it is referred to by a coded slice NAL unit belonging to a depth view component (i.e., with nal_unit_type
equal to 21) and with VOIdx equal to VOIdxMax (using that value of pic_parameter_set_id), it is activated. This picture
parameter set RBSP is called the active picture parameter set RBSP until it is deactivated when another picture parameter
set RBSP becomes the active picture parameter set RBSP. A picture parameter set RBSP, with that particular value of
pic_parameter_set_id, shall be available to the decoding process prior to its activation.


When a picture parameter set RBSP (with a particular value of pic_parameter_set_id) is not the active depth picture
parameter set for a particular value of VOIdx less than VOIdxMax and it is referred to by a coded slice NAL unit belonging
to a depth view component (i.e., with nal_unit_type equal to 21) and with the particular value of VOIdx (using that value
of pic_parameter_set_id), it is activated for view components with the particular value of VOIdx. This picture parameter
set RBSP is called the active depth picture parameter set RBSP for the particular value of VOIdx until it is deactivated
when another picture parameter set RBSP becomes the active depth picture parameter set RBSP for the particular value of
VOIdx. A picture parameter set RBSP, with that particular value of pic_parameter_set_id, shall be available to the decoding
process prior to its activation.


When a picture parameter set RBSP (with a particular value of pic_parameter_set_id) is not the active texture picture
parameter set for a particular value of VOIdx less than or equal to VOIdxMax and it is referred to by a coded slice NAL
unit belonging to a texture view component (i.e., with nal_unit_type equal to 1, 5 or 20) and with the particular value of
VOIdx (using that value of pic_parameter_set_id), it is activated for texture view components with the particular value of
VOIdx. This picture parameter set RBSP is called the active texture picture parameter set RBSP for the particular value of
VOIdx until it is deactivated when another picture parameter set RBSP becomes the active texture picture parameter set
RBSP for the particular value of VOIdx. A picture parameter set RBSP, with that particular value of pic_parameter_set_id,
shall be available to the decoding process prior to its activation.


Any picture parameter set NAL unit containing the value of pic_parameter_set_id for the active picture parameter set
RBSP for a coded picture shall have the same content as that of the active picture parameter set RBSP for this coded picture
unless it follows the last VCL NAL unit of this coded picture and precedes the first VCL NAL unit of another coded
picture. Any picture parameter set NAL unit containing the value of pic_parameter_set_id for the active depth picture
parameter set RBSP for a particular value of VOIdx less than VOIdxMax for a coded picture shall have the same content
as that of the active view picture parameter set RBSP for the particular value of VOIdx for this coded picture unless it
follows the last VCL NAL unit of this coded picture and precedes the first VCL NAL unit of another coded picture. Any
picture parameter set NAL unit containing the value of pic_parameter_set_id for the active texture picture parameter set
RBSP for a particular value of VOIdx for a coded picture shall have the same content as that of the active texture picture
parameter set RBSP for the particular value of VOIdx for this coded picture unless it follows the last VCL NAL unit of
this coded picture and precedes the first VCL NAL unit of another coded picture.


An MVCD sequence parameter set RBSP includes parameters that can be referred to by one or more picture parameter set
RBSPs or one or more buffering period SEI messages.


Each MVCD sequence parameter set RBSP is initially considered not active at the start of the operation of the decoding
process. At most one MVCD sequence parameter set RBSP is considered as the active MVCD sequence parameter set
RBSP at any given moment during the operation of the decoding process, and when any particular MVCD sequence
parameter set RBSP becomes the active MVCD sequence parameter set RBSP, the previously-active MVCD sequence
parameter set RBSP (if any) is deactivated.


In addition to the active MVCD sequence parameter set RBSP, zero or more MVCD sequence parameter set RBSPs may
be specifically active for view components (with a particular value of VOIdx less than VOIdxMax) that belong to the target
output views or that may be referred to through inter-view prediction in decoding view components belonging to the target
output views. Such an MVCD sequence parameter set RBSP is referred to as the active view MVCD sequence parameter
set RBSP for the particular value of VOIdx. The restrictions on active MVCD sequence parameter set RBSPs also apply
to active view MVCD sequence parameter set RBSPs for a particular value of VOIdx less than VOIdxMax.


Furthermore, zero or more MVCD sequence parameter set RBSPs may be specifically active for texture view components
(with a particular value of VOIdx less than or equal to VOIdxMax) that belong to the target output views or that may be
referred to through inter-view prediction in decoding texture view components belonging to the target output views. Such
an MVCD sequence parameter set RBSP is referred to as the active texture MVCD sequence parameter set RBSP for the
particular value of VOIdx. The restrictions on active MVCD sequence parameter set RBSPs also apply to active texture
MVCD sequence parameter set RBSPs for a particular value of VOIdx.


For the following specification, the activating buffering period SEI message is specified as follows:


- If VOIdxMax is equal to VOIdxMin and the access unit contains a buffering period SEI message not included in an
MVC scalable nesting SEI message and not included in an MVCD scalable nesting SEI message, this buffering period
SEI message is the activating buffering period SEI message.





- Otherwise if VOIdxMax is not equal to VOIdxMin and the access unit contains a buffering period SEI message
included in an MVCD scalable nesting SEI message and associated with the operation point being decoded, this
buffering period SEI message is the activating buffering period SEI message.


- Otherwise, the access unit does not contain an activating buffering period SEI message.


When a sequence parameter set RBSP (nal_unit_type is equal to 7) with a particular value of seq_parameter_set_id is not
already the active MVCD sequence parameter set RBSP and it is referred to by activation of a picture parameter set RBSP
(using that value of seq_parameter_set_id) and the picture parameter set RBSP is activated by a coded slice NAL unit with
nal_unit_type equal to 1 or 5 (the picture parameter set RBSP becomes the active picture parameter set RBSP and
VOIdxMax is equal to VOIdxMin and there is no depth view component in any access unit) and the access unit does not
contain an activating buffering period SEI message, it is activated. This sequence parameter set RBSP is called the active
MVCD sequence parameter set RBSP until it is deactivated when another MVCD sequence parameter set RBSP becomes
the active MVCD sequence parameter set RBSP. A sequence parameter set RBSP, with that particular value of
seq_parameter_set_id, shall be available to the decoding process prior to its activation.


When a sequence parameter set RBSP (nal_unit_type is equal to 7) with a particular value of seq_parameter_set_id is not
already the active MVCD sequence parameter set RBSP and it is referred to by an activating buffering period SEI message
(using that value of seq_parameter_set_id) that is not included in an MVCD scalable nesting SEI message and VOIdxMax
is equal to VOIdxMin and there is no depth view component in the access unit, it is activated. This sequence parameter set
RBSP is called the active MVCD sequence parameter set RBSP until it is deactivated when another MVCD sequence
parameter set RBSP becomes the active MVCD sequence parameter set RBSP. A sequence parameter set RBSP, with that
particular value of seq_parameter_set_id, shall be available to the decoding process prior to its activation.


When a subset sequence parameter set RBSP (nal_unit_type is equal to 15) with a particular value of seq_parameter_set_id
is not already the active MVCD sequence parameter set RBSP and it is referred to by activation of a picture parameter set
RBSP (using that value of seq_parameter_set_id) and the picture parameter set RBSP is activated by a coded slice depth
extension NAL unit with nal_unit_type equal to 21 and with VOIdx equal to VOIdxMax (the picture parameter set RBSP
becomes the active picture parameter set RBSP) and the access unit does not contain an activating buffering period SEI
message, it is activated. This subset sequence parameter set RBSP is called the active MVCD sequence parameter set
RBSP until it is deactivated when another MVCD sequence parameter set RBSP becomes the active MVCD sequence
parameter set RBSP. A subset sequence parameter set RBSP, with that particular value of seq_parameter_set_id, shall be
available to the decoding process prior to its activation.


When a subset sequence parameter set RBSP (nal_unit_type is equal to 15) with a particular value of seq_parameter_set_id
is not already the active MVCD sequence parameter set RBSP and it is referred to by an activating buffering period SEI
message (using that value of seq_parameter_set_id) that is included in an MVCD scalable nesting SEI message, it is
activated. This subset sequence parameter set RBSP is called the active MVCD sequence parameter set RBSP until it is
deactivated when another MVCD sequence parameter set RBSP becomes the active MVCD sequence parameter set RBSP.
A subset sequence parameter set RBSP, with that particular value of seq_parameter_set_id, shall be available to the
decoding process prior to its activation.


NOTE 2 – The active MVCD sequence parameter set RBSP is either a sequence parameter set RBSP or a subset sequence parameter
set RBSP. Sequence parameter set RBSPs are activated by coded slice NAL units with nal_unit_type equal to 1 or 5 or buffering
period SEI messages that are not included in an MVC scalable nesting SEI message or an MVCD scalable nesting SEI message.
Subset sequence parameter sets are activated by coded slice depth extension NAL units (nal_unit_type equal to 21) or buffering
period SEI messages that are included in an MVCD scalable nesting SEI message. A sequence parameter set RBSP and a subset
sequence parameter set RBSP may have the same value of seq_parameter_set_id.


For the following specification, the activating texture buffering period SEI message for a particular value of VOIdx is
specified as follows:


- If the access unit contains one or more than one buffering period SEI message included in an MVC scalable nesting
SEI message and associated with an operation point for which the greatest VOIdx in the associated bitstream subset
is equal to the particular value of VOIdx, the first of these buffering period SEI messages, in decoding order, is the
activating texture buffering period SEI message for the particular value of VOIdx.


- Otherwise, if the access unit contains a buffering period SEI message not included in an MVC scalable nesting SEI
message or an MVCD scalable nesting SEI message, this buffering period SEI message is the activating texture
buffering period SEI message for the particular value of VOIdx equal to VOIdxMin.


- Otherwise, the access unit does not contain an activating texture buffering period SEI message for the particular value
of VOIdx.


When a sequence parameter set RBSP (nal_unit_type is equal to 7) with a particular value of seq_parameter_set_id is not
already the active texture MVCD sequence parameter set RBSP for VOIdx equal to VOIdxMin and it is referred to by
activation of a picture parameter set RBSP (using that value of seq_parameter_set_id) and the picture parameter set RBSP
is activated by a coded slice NAL unit with nal_unit_type equal to 1 or 5 (the picture parameter set RBSP becomes the
active texture picture parameter set RBSP for VOIdx equal to VOIdxMin), it is activated for texture view components with





VOIdx equal to VOIdxMin. This sequence parameter set RBSP is called the active texture MVCD sequence parameter set
RBSP for VOIdx equal to VOIdxMin until it is deactivated when another MVCD sequence parameter set RBSP becomes
the active texture MVCD sequence parameter set RBSP for VOIdx equal to VOIdxMin. A sequence parameter set RBSP,
with that particular value of seq_parameter_set_id, shall be available to the decoding process prior to its activation.


When a sequence parameter set RBSP (nal_unit_type is equal to 7) with a particular value of seq_parameter_set_id is not
already the active texture MVCD sequence parameter set RBSP for VOIdx equal to VOIdxMin and it is referred to by an
activating texture buffering period SEI message (using that value of seq_parameter_set_id) that is not included in an MVC
scalable nesting SEI message or an MVCD scalable nesting SEI message, the sequence parameter set RBSP is activated
for texture view components with VOIdx equal to VOIdxMin. This sequence parameter set RBSP is called the active
texture MVCD sequence parameter set RBSP for VOIdx equal to VOIdxMin until it is deactivated when another MVCD
sequence parameter set RBSP becomes the active texture MVCD sequence parameter set RBSP for VOIdx equal to. A
sequence parameter set RBSP, with that particular value of seq_parameter_set_id, shall be available to the decoding
process prior to its activation.


When a subset sequence parameter set RBSP (nal_unit_type is equal to 15) with a particular value of seq_parameter_set_id
is not already the active texture MVCD sequence parameter set RBSP for a particular value of VOIdx less than or equal to
VOIdxMax and it is referred to by activation of a picture parameter set RBSP (using that value of seq_parameter_set_id)
and the picture parameter set RBSP is activated by a coded slice MVC extension NAL unit (nal_unit_type equal to 20)
with the particular value of VOIdx (the picture parameter set RBSP becomes the active texture picture parameter set RBSP
for the particular value of VOIdx), it is activated for texture view components with the particular value of VOIdx. This
subset sequence parameter set RBSP is called the active texture MVCD sequence parameter set RBSP for the particular
value of VOIdx until it is deactivated when another MVCD sequence parameter set RBSP becomes the active texture
MVCD sequence parameter set RBSP for the particular value of VOIdx. A subset sequence parameter set RBSP, with that
particular value of seq_parameter_set_id, shall be available to the decoding process prior to its activation.


When a subset sequence parameter set RBSP (nal_unit_type is equal to 15) with a particular value of seq_parameter_set_id
is not already the active texture MVCD sequence parameter set RBSP for a particular value of VOIdx less than or equal to
VOIdxMax and it is referred to by an activating texture buffering period SEI message (using that value of
seq_parameter_set_id) that is included in an MVC scalable nesting SEI message and associated with the particular value
of VOIdx, this subset sequence parameter set RBSP is activated for texture view components with the particular value of
VOIdx. This subset sequence parameter set RBSP is called the active texture MVCD sequence parameter set RBSP for the
particular value of VOIdx until it is deactivated when another MVCD sequence parameter set RBSP becomes the active
texture MVCD sequence parameter set RBSP for the particular value of VOIdx. A subset sequence parameter set RBSP,
with that particular value of seq_parameter_set_id, shall be available to the decoding process prior to its activation.


For the following specification, the activating view buffering period SEI message for a particular value of VOIdx is
specified as follows:


- If the access unit contains one or more than one buffering period SEI message included in an MVCD scalable nesting
SEI message and associated with an operation point for which the greatest VOIdx in the associated bitstream subset
is equal to the particular value of VOIdx, the first of these buffering period SEI messages, in decoding order, is the
activating view buffering period SEI message for the particular value of VOIdx.


- Otherwise, the access unit does not contain an activating view buffering period SEI message for the particular value
of VOIdx.


When a subset sequence parameter set RBSP (nal_unit_type is equal to 15) with a particular value of seq_parameter_set_id
is not already the active view MVCD sequence parameter set RBSP for a particular value of VOIdx less than VOIdxMax
and it is referred to by activation of a picture parameter set RBSP (using that value of seq_parameter_set_id) and the picture
parameter set RBSP is activated by a coded slice NAL unit with nal_unit_type equal to 21 and with the particular value of
VOIdx (the picture parameter set RBSP becomes the active view picture parameter set RBSP for the particular value of
VOIdx), it is activated for view components with the particular value of VOIdx. This subset sequence parameter set RBSP
is called the active view MVCD sequence parameter set RBSP for the particular value of VOIdx until it is deactivated
when another MVCD sequence parameter set RBSP becomes the active view MVCD sequence parameter set RBSP for
the particular value of VOIdx or when decoding an access unit with VOIdxMax less than or equal to the particular value
of VOIdx. A subset sequence parameter set RBSP, with that particular value of seq_parameter_set_id, shall be available
to the decoding process prior to its activation.


When a subset sequence parameter set RBSP (nal_unit_type is equal to 15) with a particular value of seq_parameter_set_id
is not already the active view MVCD sequence parameter set RBSP for a particular value of VOIdx less than VOIdxMax
and it is referred to by an activating view buffering period SEI message (using that value of seq_parameter_set_id) that is
included in an MVCD scalable nesting SEI message and associated with the particular value of VOIdx, this subset sequence
parameter set RBSP is activated for view components with the particular value of VOIdx. This subset sequence parameter
set RBSP is called the active view MVCD sequence parameter set RBSP for the particular value of VOIdx until it is
deactivated when another MVCD sequence parameter set RBSP becomes the active view MVCD sequence parameter set





RBSP for the particular value of VOIdx or when decoding an access unit with VOIdxMax less than or equal to the particular
value of VOIdx. A subset sequence parameter set RBSP, with that particular value of seq_parameter_set_id, shall be
available to the decoding process prior to its activation.


An MVCD sequence parameter set RBSP that includes a value of profile_idc not specified in Annex A or Annex H or
Annex I shall not be referred to by activation of a picture parameter set RBSP as the active picture parameter set RBSP or
as active view picture parameter set RBSP or as active texture picture parameter set RBSP (using that value of
seq_parameter_set_id) or referred to by a buffering period SEI message (using that value of seq_parameter_set_id). An
MVCD sequence parameter set RBSP including a value of profile_idc not specified in Annex A or Annex H or Annex I is
ignored in the decoding for profiles specified in Annex A or Annex H or Annex I.


It is a requirement of bitstream conformance that the following constraints are obeyed:


- For each particular value of VOIdx, all coded slice NAL units (with nal_unit_type equal to 1, 5, 20, or 21) of a coded
video sequence shall refer to the same value of seq_parameter_set_id (via the picture parameter set RBSP that is
referred to by the value of pic_parameter_set_id).


- The value of seq_parameter_set_id in a buffering period SEI message that is not included in an MVC scalable nesting
SEI message shall be identical to the value of seq_parameter_set_id in the picture parameter set RBSP that is referred
to by coded slice NAL units with nal_unit_type equal to 1 or 5 (via the value of pic_parameter_set_id) in the same
access unit.


- The value of seq_parameter_set_id in a buffering period SEI message that is included in an MVC scalable nesting
SEI message and is associated with a particular value of VOIdx shall be identical to the value of seq_parameter_set_id
in the picture parameter set RBSP that is referred to by coded slice NAL units with nal_unit_type equal to 1, 5 or 20
with the particular value of VOIdx (via the value of pic_parameter_set_id) in the same access unit.


- The value of seq_parameter_set_id in a buffering period SEI message that is included in an MVCD scalable nesting
SEI message and is associated with a particular value of VOIdx shall be identical to the value of seq_parameter_set_id
in the picture parameter set RBSP that is referred to by coded slice NAL units with nal_unit_type equal to 21 with the
particular value of VOIdx (via the value of pic_parameter_set_id) in the same access unit.


The active view MVCD sequence parameter set RBSPs for different values of VOIdx may be the same MVCD sequence
parameter set RBSP. The active MVCD sequence parameter set RBSP and an active view MVCD sequence parameter set
RBSP for a particular value of VOIdx may be the same MVCD sequence parameter set RBSP.


The active texture MVCD sequence parameter set RBSPs for different values of VOIdx may be the same MVCD sequence
parameter set RBSP. The active MVCD sequence parameter set RBSP and an active texture MVCD sequence parameter
set RBSP for a particular value of VOIdx may be the same MVCD sequence parameter set RBSP.


When the active MVCD sequence parameter set RBSP for a coded picture is a sequence parameter set RBSP, any sequence
parameter set RBSP in the coded video sequence containing this coded picture and with the value of seq_parameter_set_id
for the active MVCD sequence parameter set RBSP shall have the same content as that of the active MVCD sequence
parameter set RBSP.


When the active MVCD sequence parameter set RBSP for a coded picture is a subset sequence parameter set RBSP, any
subset sequence parameter set RBSP in the coded video sequence containing this coded picture and with the value of
seq_parameter_set_id for the active MVCD sequence parameter set RBSP shall have the same content as that of the active
MVCD sequence parameter set RBSP.


For each particular value of VOIdx, the following applies:


- When the active texture MVCD sequence parameter set RBSP for a coded picture is a sequence parameter set RBSP,
any sequence parameter set RBSP in the coded video sequence containing this coded picture and with the value of
seq_parameter_set_id for the active texture MVCD sequence parameter set RBSP shall have the same content as that
of the active texture MVCD sequence parameter set RBSP.


- When the active texture MVCD sequence parameter set RBSP for a coded picture is a subset sequence parameter set
RBSP, any subset sequence parameter set RBSP in the coded video sequence containing this coded picture and with
the value of seq_parameter_set_id for the active texture MVCD sequence parameter set RBSP shall have the same
content as that of the active texture MVCD sequence parameter set RBSP.


- The active view MVCD sequence parameter set RBSP for a coded picture is a subset sequence parameter set RBSP,
and any subset sequence parameter set RBSP in the coded video sequence containing this coded picture and with the
value of seq_parameter_set_id for the active view MVCD sequence parameter set RBSP shall have the same content
as that of the active view MVCD sequence parameter set RBSP.


NOTE 3 – If picture parameter set RBSPs or MVCD sequence parameter set RBSPs are conveyed within the bitstream, these
constraints impose an order constraint on the NAL units that contain the picture parameter set RBSPs or MVCD sequence parameter
set RBSPs, respectively. Otherwise (picture parameter set RBSPs or MVCD sequence parameter set RBSPs are conveyed by other





means not specified in this Recommendation | International Standard), they must be available to the decoding process in a timely
fashion such that these constraints are obeyed.


When present, a sequence parameter set extension RBSP includes parameters having a similar function to those of a
sequence parameter set RBSP. For purposes of establishing constraints on the syntax elements of the sequence parameter
set extension RBSP and for purposes of determining activation of a sequence parameter set extension RBSP, the sequence
parameter set extension RBSP shall be considered part of the preceding sequence parameter set RBSP with the same value
of seq_parameter_set_id. When a sequence parameter set RBSP is present that is not followed by a sequence parameter
set extension RBSP with the same value of seq_parameter_set_id prior to the activation of the sequence parameter set
RBSP, the sequence parameter set extension RBSP and its syntax elements shall be considered not present for the active
MVCD sequence parameter set RBSP. The contents of sequence parameter set extension RBSPs only apply when the base
texture view, which conforms to one or more of the profiles specified in Annex A, of a coded video sequence conforming
to one or more profiles specified in Annex I is decoded. Subset sequence parameter set RBSPs shall not be followed by a
sequence parameter set extension RBSP.


NOTE 4 – Sequence parameter sets extension RBSPs are not considered to be part of a subset sequence parameter set RBSP and
subset sequence parameter set RBSPs must not be followed by a sequence parameter set extension RBSP.


For view components with VOIdx equal to VOIdxMax, all constraints that are expressed on the relationship between the
values of the syntax elements (and the values of variables derived from those syntax elements) in MVCD sequence
parameter sets and picture parameter sets and other syntax elements are expressions of constraints that apply only to the
active MVCD sequence parameter set and the active picture parameter set. For view components with a particular value
of VOIdx less than VOIdxMax, all constraints that are expressed on the relationship between the values of the syntax
elements (and the values of variables derived from those syntax elements) in MVCD sequence parameter sets and picture
parameter sets and other syntax elements are expressions of constraints that apply only to the active view MVCD sequence
parameter set and the active view picture parameter set for the particular value of VOIdx. If any MVCD sequence parameter
set RBSP having profile_idc equal to the value of one of the profile_idc values specified in Annex A or Annex H or Annex I
is present that is never activated in the bitstream (i.e., it never becomes the active MVCD sequence parameter set or an
active view MVCD sequence parameter set), its syntax elements shall have values that would conform to the specified
constraints if it were activated by reference in an otherwise-conforming bitstream. If any picture parameter set RBSP is
present that is never activated in the bitstream (i.e., it never becomes the active picture parameter set or an active view
picture parameter set), its syntax elements shall have values that would conform to the specified constraints if it were
activated by reference in an otherwise-conforming bitstream.


During operation of the decoding process (see clause I.8), for view components with VOIdx equal to VOIdxMax, the
values of parameters of the active picture parameter set and the active MVCD sequence parameter set shall be considered
in effect. For view components with a particular value of VOIdx less than VOIdxMax, the values of the parameters of the
active view picture parameter set and the active view MVCD sequence parameter set for the particular value of VOIdx
shall be considered in effect. For interpretation of SEI messages that apply to the entire access unit or the view component
with VOIdx equal to VOIdxMax, the values of the parameters of the active picture parameter set and the active MVCD
sequence parameter set for the same access unit shall be considered in effect unless otherwise specified in the SEI message
semantics. For interpretation of SEI messages that apply to view components with a particular value of VOIdx less than
VOIdxMax, the values of the parameters of the active view picture parameter set and the active view MVCD sequence
parameter set for the particular value of VOIdx for the same access unit shall be considered in effect unless otherwise
specified in the SEI message semantics.


For any active MVCD sequence parameter set or active view MVCD sequence parameter set, part of the syntax elements
in the MVC sequence parameter set extension applies only to the depth views referring to this sequence parameter set,
while the some other parts of the syntax elements in the MVCD sequence parameter set extension collectively apply to
both the depth views referring to this sequence parameter set and the corresponding texture views. More specifically, the
view dependency information of the MVCD sequence parameter set extension applies only to the depth views, and the
level definitions collectively apply to operation points, each of which contains both depth views and their corresponding
texture views. Moreover, the mvcd_vui_parameters_extension( ) applies collectively to both the depth views referring to
this MVCD sequence parameter set and the corresponding texture views. The vui_parameters( ) included in the sequence
parameter set data syntax structure, if present, apply collectively to both the depth views referring to this sequence
parameter set and the corresponding texture views, except for the aspect ratio information and the bitstream restriction
information, if present, which apply only to the depth views referring to this MVCD sequence parameter set. The aspect
ratio information and the bitstream restriction information for the texture views may be present in the vui_parameters( )
syntax structure included in an MVC sequence parameter set.


**I.7.4.1.2.2** **Order of access units and association to coded video sequences**


The specification of clause H.7.4.1.2.2 apply.


**I.7.4.1.2.3** **Order of NAL units and coded pictures and association to access units**


The specification of clause H.7.4.1.2.3 applies with the following modifications.





NOTE – Some bitstreams that conform to one or more profiles specified in this annex do not conform to any profile specified in
Annex A (prior to operation of the base view extraction process specified in clause I.8.5.4). As specified in clauses 7.4.1
and 7.4.1.2.3 for the profiles specified in Annex A, NAL units with nal_unit_type equal to 21 are classified as non-VCL NAL units
that must be preceded within each access unit by at least one NAL unit with nal_unit_type in the range of 1 to 5, inclusive. For this
reason, any bitstream that conforms to one or more profiles specified in this annex does not conform to any profile specified in
Annex A when it contains any of the following:

  - any access unit that does not contain any NAL units with nal_unit_type equal to 1 or 5, but contains one or more NAL units
with nal_unit_type equal to 6, 7, 8, 9, or 15

  - any access unit in which one or more NAL units with nal_unit_type equal to 7, 8, or 15 is present after the last NAL unit in the
access unit with nal_unit_type equal to 1 or 5.


The association of VCL NAL units to primary or redundant coded pictures is specified in clause I.7.4.1.2.5.


The constraints for the detection of the first VCL NAL unit of a primary coded picture are specified in clause I.7.4.1.2.4.


The constraint expressed in clause H.7.4.1.2.3 on the order of a buffering period SEI message is replaced by the following
constraints.


- When an SEI NAL unit containing a buffering period SEI message is present, the following applies:


   - If the buffering period SEI message is the only buffering period SEI message in the access unit and it is not
included in an MVC scalable nesting SEI message or an MVCD scalable nesting SEI message, the buffering
period SEI message shall be the first SEI message payload of the first SEI NAL unit in the access unit.


   - Otherwise (the buffering period SEI message is not the only buffering period SEI message in the access unit or
it is included in an MVC scalable nesting SEI message or it is included in an MVCD scalable nesting SEI
message), the following constraints are specified:


      - When a buffering period SEI message that is not included in either an MVC scalable nesting SEI message
or an MVCD scalable nesting SEI message is present, this buffering period SEI message shall be the only
SEI message payload of the first SEI NAL unit in the access unit.


      - An MVC scalable nesting SEI message that includes a buffering period SEI message shall not include any
other SEI messages and shall be the only SEI message inside the SEI NAL unit.


      - An MVCD scalable nesting SEI message that includes a buffering period SEI message shall not include any
other SEI messages and shall be the only SEI message inside the SEI NAL unit.


      - All SEI NAL units that precede an SEI NAL unit that contains an MVC scalable nesting SEI message with
a buffering period SEI message as payload, or an MVCD scalable nesting SEI message with a buffering
period SEI message as payload in an access unit shall only contain buffering period SEI messages or MVC
scalable nesting SEI messages with a buffering period SEI message as payload, or MVCD scalable nesting
SEI messages with a buffering period SEI message.


**I.7.4.1.2.4** **Detection of the first VCL NAL unit of a primary coded picture**


The specification of clause H.7.4.1.2.4 applies.


**I.7.4.1.2.5** **Order of VCL NAL units and association to coded pictures**


The specification of clause H.7.4.1.2.5 applies with following modifications.


Each VCL NAL unit is part of a coded picture.


Let voIdx be the value of VOIdx of any particular VCL NAL unit. The order of the VCL NAL units within a coded picture
is constrained as follows:


- For all VCL NAL units following this particular VCL NAL unit, the value of VOIdx shall be greater than or equal
to voIdx.


- All VCL NAL units for a depth view component, if present, shall follow any VCL NAL unit of a texture view
component with a same value of VOIdx.


For each set of VCL NAL units within a texture or depth view component, the following applies:


- If arbitrary slice order, as specified in Annex A, clause H.10 or clause I.10, is allowed, coded slice NAL units of a
view component may have any order relative to each other.


- Otherwise (arbitrary slice order is not allowed), coded slice NAL units of a slice group shall not be interleaved with
coded slice NAL units of another slice group and the order of coded slice NAL units within a slice group shall be in
the order of increasing macroblock address for the first macroblock of each coded slice NAL unit of the same slice
group.





The following applies:


- If a coded texture view component with a particular view_id is the first field view component of a complementary
field pair, the depth view component with the same view_id value, if present in the access unit, shall be a coded frame
view component or the first field view component of a complementary field pair.


- Otherwise, if a coded texture view component with a particular view_id is the second field view component of a
complementary field pair, the depth view component with the same view_id value, if present in the access unit, shall
be the second field view component of a complementary field pair.


- Otherwise, if a coded texture view component with a particular view_id is a non-paired field, the depth view
component with the same view_id value, if present in the access unit, shall be a coded frame view component or a
non-paired field.


- Otherwise (a coded texture view component with a particular view_id is a coded frame), the depth view component
with the same view_id value, if present in the access unit, shall be a coded frame view component.


NAL units having nal_unit_type equal to 12 may be present in the access unit but shall not precede the first VCL NAL
unit of the primary coded picture within the access unit.


NAL units having nal_unit_type equal to 0 or in the range of 24 to 31, inclusive, which are unspecified, may be present in
the access unit but shall not precede the first VCL NAL unit of the primary coded picture within the access unit.


NAL units having nal_unit_type in the range of 22 to 23, inclusive, which are reserved, shall not precede the first VCL
NAL unit of the primary coded picture within the access unit (when specified in the future by ITU-T | ISO/IEC).


**I.7.4.2** **Raw byte sequence payloads and RBSP trailing bits semantics**


**I.7.4.2.1** **Sequence parameter set RBSP semantics**


The semantics specified in clause 7.4.2.1 apply.


**I.7.4.2.1.1** **Sequence parameter set data semantics**


The semantics specified in clause H.7.4.2.1.1 apply with the substitution of MVCD sequence parameter set for MVC
sequence parameter set. All constraints specified in clause H.7.4.2.1.1 apply only to the texture view components for which
the MVCD sequence parameter set is the active texture MVC sequence parameter set or to the depth view components for
which the MVCD sequence parameter set is the active view MVC sequence parameter set as specified in clause I.7.4.1.2.1.


**I.7.4.2.1.1.1** **Scaling list semantics**


The semantics specified in clause H.7.4.2.1.1.1 apply.


**I.7.4.2.1.2** **Sequence parameter set extension RBSP semantics**


The semantics specified in clause 7.4.2.1.2 apply. Additionally, the following applies.


Sequence parameter set extension RBSPs can only follow sequence parameter set RBSPs in decoding order. Subset
sequence parameter set RBSPs shall not be followed by a sequence parameter set extension RBSP. The contents of
sequence parameter set extension RBSPs only apply when the base view, which conforms to one or more of the profiles
specified in Annex A, of a coded video sequence conforming to one or more profiles specified in Annex I is decoded.


**I.7.4.2.1.3** **Subset sequence parameter set RBSP semantics**


The semantics specified in clause 7.4.2.1.3 apply with the following additions.


**mvcd_vui_parameters_present_flag** equal to 0 specifies that the syntax structure mvc_vui_parameters_extension( )
corresponding to MVCD VUI parameters extension is not present. mvcd_vui_parameters_present_flag equal to 1 specifies
that the syntax structure mvc_vui_parameters_extension( ) is present and referred to as MVCD VUI parameters extension.


**texture_vui_parameters_present_flag** equal to 0 specifies that the syntax structure mvc_vui_parameters_extension( )
corresponding to MVCD texture sub-bitstream VUI parameters extension is not present.
texture_vui_parameters_present_flag equal to 1 specifies that the syntax structure mvc_vui_parameters_extension( ) is
present and referred to as MVCD texture sub-bitstream VUI parameters extension.


**I.7.4.2.1.4** **Sequence parameter set MVCD extension semantics**


The semantics specified in clause H.7.4.2.1.4 apply with the substitution of texture view component or depth view
component for view component and with the following additions:


**depth_view_present_flag[** i **]** equal to 0 specifies that there is no depth view having a view_id equal to view_id[ i ] and
VOIdx equal to i. depth_view_present_flag[ i ] equal to 1 specifies that there is a depth view having a view_id equal to
view_id[ i ].





**texture_view_present_flag[** i **]** equal to 0 specifies that there is no texture view having a view_id equal to view_id[ i ]
and VOIdx equal to i. texture_view_present_flag[ i ] equal to 1 specifies that there is a texture view having a view_id equal
to view_id[ i ] and VOIdx equal to i. When depth_view_present_flag[ i ] is equal to 0, texture_view_present_flag[ i ] shall
be equal to 1.


num_anchor_refs_l0[ i ], anchor_ref_l0[ i ][ j ], num_anchor_refs_l1[ i ], anchor_ref_l1[ i ][ j ],
num_non_anchor_refs_l0[ i ], non_anchor_ref_l0[ i ][ j ], num_non_anchor_refs_l1[ i ], and non_anchor_ref_l1[ i ][ j ]
apply to depth view components.


**applicable_op_depth_flag[** i **][** j **][** k **]** equal to 0 indicates that the depth view with view_id equal to
applicable_op_target_view_id[ i ][ j ][ k ] is not included in the j-th operation point. applicable_op_depth_flag[ i ][ j ][ k ]
equal to 1 indicates that the depth view with view_id equal to applicable_op_target_view_id[ i ][ j ][ k ] is included in the
j-th operation point.


**applicable_op_texture_flag[** i **][** j **][** k **]** equal to 0 indicates that the texture view with view_id equal to
applicable_op_target_view_id[ i ][ j ][ k ] is not included in the j-th operation point.
applicable_op_texture_flag[ i ][ j ][ k ] equal to 1 indicates that the texture view with view_id equal to
applicable_op_target_view_id[ i ][ j ][ k ] is included in the j-th operation point. When
applicable_op_depth_flag[ i ][ j ][ k ] is equal to 0, applicable_op_texture_flag[ i ][ j ][ k ] shall be equal to 1.


**applicable_op_num_texture_views_minus1[** i **][** j **]** plus 1 specifies the number of texture views required for decoding
the target output views corresponding to the j-th operation point to which the level indicated by level_idc[ i ] applies. The
number of texture views specified by applicable_op_num_views_minus1 includes the texture views of the target output
views and the texture views that the target output views depend on. The value of
applicable_op_num_texture_views_minus1[ i ][ j ] shall be in the range of 0 to 1023, inclusive.


**applicable_op_num_depth_views[** i **][** j **]** specifies the number of depth views required for decoding the target output
views corresponding to the j-th operation point to which the level indicated by level_idc[ i ] applies. The number of depth
views specified by applicable_op_num_depth_views_minus1 includes the depth views of the target output views and the
depth views that the depth views of the target output views depend on. The value of
applicable_op_num_depth_views_minus1[ i ][ j ] shall be in the range of 0 to 1023, inclusive.


All sequence parameter set MVCD extensions that are included in the active view MVCD sequence parameter set RBSPs
of one coded video sequence shall be identical.


**I.7.4.2.2** **Picture parameter set RBSP semantics**


The semantics specified in clause H.7.4.2.2 apply with substituting MVCD sequence parameter set for MVC sequence
parameter set. All constraints specified in clause H.7.4.2.2 apply only to the texture or depth view components for which
the picture parameter set is the active picture parameter set or the active view picture parameter set or the active texture
picture parameter set as specified in clause I.7.4.1.2.1.


**I.7.4.2.3** **Supplemental enhancement information RBSP semantics**


The semantics specified in clause H.7.4.2.3 apply.


**I.7.4.2.3.1** **Supplemental enhancement information message semantics**


The semantics specified in clause H.7.4.2.3.1 apply.


**I.7.4.2.4** **Access unit delimiter RBSP semantics**


The semantics specified in clause H.7.4.2.4 apply.

NOTE – The value of primary_pic_type applies to the slice_type values in all slice headers of the primary coded picture, including
the slice_type syntax elements in all NAL units with nal_unit_type equal to 1, 5, 20 or 21. NAL units with nal_unit_type equal to 2
are not present in bitstreams conforming to any of the profiles specified in this annex.


**I.7.4.2.5** **End of sequence RBSP semantics**


The semantics specified in clause H.7.4.2.5 apply.


**I.7.4.2.6** **End of stream RBSP semantics**


The semantics specified in clause H.7.4.2.6 apply.


**I.7.4.2.7** **Filler data RBSP semantics**


The semantics specified in clause H.7.4.2.7 apply with the following modifications.


Filler data NAL units shall be considered to contain the syntax elements priority_id, view_id, and temporal_id with values
that are inferred as follows:


1. Let prevMvcNalUnit be the most recent NAL unit in decoding order that has nal_unit_type equal to 14, 20 or 21.





NOTE – The most recent NAL unit in decoding order with nal_unit_type equal to 14, 20 or 21 always belongs to the same
access unit as the filler data NAL unit.


2. The values of priority_id, view_id, and temporal_id for the filler data NAL unit are inferred to be equal to the

values of priority_id, view_id, and temporal_id, respectively, of the NAL unit prevMvcNalUnit.


**I.7.4.2.8** **Slice layer without partitioning RBSP semantics**


The semantics specified in clause H.7.4.2.8 apply.


**I.7.4.2.9** **Slice data partition RBSP semantics**


Slice data partition syntax is not present in bitstreams conforming to one or more of the profiles specified in Annex I.


**I.7.4.2.10** **RBSP slice trailing bits semantics**


The semantics specified in H.7.4.2.10 apply.


**I.7.4.2.11** **RBSP trailing bits semantics**


The semantics specified in clause H.7.4.2.11 apply.


**I.7.4.2.12** **Prefix NAL unit RBSP semantics**


The semantics specified in clause H.7.4.2.12 apply.


**I.7.4.2.13** **Slice layer extension RBSP semantics**


The semantics specified in clause H.7.4.2.13 apply.


**I.7.4.3** **Slice header semantics**


The semantics specified in clause H.7.4.3 apply with the substitution of texture view component (for nal_unit_type equal
to 1, 5, and 20) or depth view component (for nal_unit_type equal to 21 and avc_3d_extension_flag equal to 0) for view
component and with the following modifications.


When nal_unit_type is equal to 1, 5, or 20, all constraints specified in clause H.7.4.3 apply only to the texture view
components with the same value of VOIdx. When nal_unit_type is equal to 21 and avc_3d_extension_flag is equal to 0,
all constraints specified in clause H.7.4.3 apply only to the depth view components with the same value of VOIdx.


The value of the following MVCD sequence parameter set syntax elements shall be the same across all coded slice NAL
units of nal_unit_type equal to 1, 5, and 20 of an access unit: chroma_format_idc.


The value of the following slice header syntax elements shall be the same across all coded slice NAL units of nal_unit_type
equal to 1, 5, and 20 of an access unit: field_pic_flag and bottom_field_flag.


The value of the following slice header syntax elements shall be the same across all coded slice NAL units of nal_unit_type
equal to 21 of an access unit: field_pic_flag and bottom_field_flag.


**I.7.4.3.1** **Reference picture list modification semantics**


The semantics specified in clause H.7.4.3.1 apply.


**I.7.4.3.1.1** **Reference picture list MVC modification semantics**


The semantics specified in clause H.7.4.3.1.1 apply.


**I.7.4.3.2** **Prediction weight table semantics**


The semantics specified in clause H.7.4.3.2 apply.


**I.7.4.3.3** **Decoded reference picture marking semantics**


The semantics specified in clause 7.4.3.3 apply to each view independently, with "sequence parameter set" being replaced
by "MVCD sequence parameter set", and "primary coded picture" being replaced by "texture view component" for
nal_unit_type equal to 1, 5, and 20, and by "depth view component" for nal_unit_type equal to 21.


**I.7.4.4** **Slice data semantics**


The semantics specified in clause H.7.4.4 apply.


**I.7.4.5** **Macroblock layer semantics**


The semantics specified in clause H.7.4.5 apply.





**I.7.4.5.1** **Macroblock prediction semantics**


The semantics specified in clause H.7.4.5.1 apply.


**I.7.4.5.2** **Sub-macroblock prediction semantics**


The semantics specified in clause H.7.4.5.2 apply.


**I.7.4.5.3** **Residual data semantics**


The semantics specified in clause H.7.4.5.3 apply.


**I.7.4.5.3.1** **Residual luma semantics**


The semantics specified in clause H.7.4.5.3.1 apply.


**I.7.4.5.3.2** **Residual block CAVLC semantics**


The semantics specified in clause H.7.4.5.3.2 apply.


**I.7.4.5.3.3** **Residual block CABAC semantics**


The semantics specified in clause H.7.4.5.3.3 apply.
