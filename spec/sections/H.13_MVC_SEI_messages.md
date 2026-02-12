**H.13** **MVC SEI messages**


The specifications in Annex D together with the extensions and modifications specified in this clause apply.


**H.13.1** **SEI message syntax**


**H.13.1.1Parallel decoding information SEI message syntax**

|parallel_decoding_info( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**seq_parameter_set_id**|5|ue(v)|
|<br>for( i = 1; i <= num_views_minus1; i++ ) {|||
|<br> <br>if( anchor_pic_flag ) {|||
|<br> <br> <br>for( j = 0; j <= num_anchor_refs_l0[ i ]; j++ )|||
|<br> <br> <br> <br>**pdi_init_delay_anchor_minus2_l0[**i** ][**j** ]**|5|ue(v)|
|<br> <br> <br>for( j = 0; j <= num_anchor_refs_l1[ i ]; j++ )|||
|<br> <br> <br> <br>**pdi_init_delay_anchor_minus2_l1[**i** ][**j** ]**|5|ue(v)|
|<br> <br>}|||
|<br> <br>else {|||
|<br> <br> <br>for( j = 0; j <= num_non_anchor_refs_l0[ i ]; j++ )|||
|<br> <br> <br> <br>**pdi_init_delay_non_anchor_minus2_l0[**i** ][**j** ]**|5|ue(v)|
|<br> <br> <br>for( j = 0; j <= num_non_anchor_refs_l1[ i ]; j++ )|||
|<br> <br> <br> <br>**pdi_init_delay_non_anchor_minus2_l1[**i** ][**j** ]**|5|ue(v)|
|<br> <br>}|||
|<br>}|||
|}|||



**H.13.1.2MVC scalable nesting SEI message syntax**

|mvc_scalable_nesting( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**operation_point_flag**|5|u(1)|
|<br>if( !operation_point_flag ) {|||
|<br> <br>**all_view_components_in_au_flag**|5|u(1)|
|<br> <br>if( !all_view_components_in_au_flag ) {|||
|<br> <br> <br>**num_view_components_minus1**|5|ue(v)|
|<br> <br> <br>for( i = 0; i <= num_view_components_minus1; i++ )|||
|<br> <br> <br> <br>**sei_view_id[** i**] **|5|u(10)|
|<br> <br>}|||
|<br>} else {|||
|<br> <br>**num_view_components_op_minus1**|5|ue(v)|
|<br> <br>for( i = 0; i <= num_view_components_op_minus1; i++ )|||
|<br> <br> <br>**sei_op_view_id[** i**] **|5|u(10)|
|<br> <br>**sei_op_temporal_id**|5|u(3)|
|<br>}|||
|<br>while( !byte_aligned( ) )|||






|sei_nesting_zero_bit /* equal to 0 */|5|f(1)|
|---|---|---|
|<br>sei_message( )|5||
|}|||


**H.13.1.3** **View scalability information SEI message syntax**

|view_scalability_info( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**num_operation_points_minus1**|5|ue(v)|
|<br>for( i = 0; i <= num_operation_points_minus1; i++ ) {|||
|<br> <br>**operation_point_id[** i**] **|5|ue(v)|
|<br> <br>**priority_id[**i** ]**|5|u(5)|
|<br> <br>**temporal_id[** i**] **|5|u(3)|
|<br> <br>**num_target_output_views_minus1[** i**] **|5|ue(v)|
|<br> <br>for( j = 0; j <= num_target_output_views_minus1[ i ]; j++ )|||
|<br> <br> <br>**view_id[** i**][** j**] **|5|ue(v)|
|<br> <br>**profile_level_info_present_flag[** i**]**|5|u(1)|
|<br> <br>**bitrate_info_present_flag[** i**]**|5|u(1)|
|<br> <br>**frm_rate_info_present_flag[** i**]**|5|u(1)|
|<br> <br>if( !num_target_output_views_minus1[ i ] )|||
|<br> <br> <br>**view_dependency_info_present_flag[**i** ]**|5|u(1)|
|<br> <br>**parameter_sets_info_present_flag[** i**]**|5|u(1)|
|<br> <br>**bitstream_restriction_info_present_flag**[ i ]|5|u(1)|
|<br> <br>if( profile_level_info_present_flag[ i ] )|||
|<br> <br> <br>**op_profile_level_idc[** i**]**|5|u(24)|
|<br> <br>if( bitrate_info_present_flag[ i ] ) {|||
|<br> <br> <br>**avg_bitrate[** i**] **|5|u(16)|
|<br> <br> <br>**max_bitrate[** i**] **|5|u(16)|
|<br> <br> <br>**max_bitrate_calc_window[** i**]**|5|u(16)|
|<br> <br>}|||
|<br> <br>if( frm_rate_info_present_flag[ i ] ) {|||
|<br> <br> <br>**constant_frm_rate_idc[** i**] **|5|u(2)|
|<br> <br> <br>**avg_frm_rate[** i**] **|5|u(16)|
|<br> <br>}|||
|<br> <br>if( view_dependency_info_present_flag[ i ] ) {|||
|<br> <br> <br>**num_directly_dependent_views[**i** ]**|5|ue(v)|
|<br> <br> <br>for( j = 0; j < num_directly_dependent_views[ i ]; j++ )|||
|<br> <br> <br> <br>**directly_dependent_view_id[**i** ][**j** ]**|5|ue(v)|
|<br> <br>} else|||
|<br> <br> <br>**view_dependency_info_src_op_id[** i**]**|5|ue(v)|
|<br> <br>if( parameter_sets_info_present_flag[ i ] ) {|||
|<br> <br> <br>**num_seq_parameter_sets[** i**] **|5|ue(v)|
|<br> <br> <br>for( j = 0; j < num_seq_parameter_sets[ i ]; j++ )|||
|<br> <br> <br> <br>**seq_parameter_set_id_delta[** i**][** j**]**|5|ue(v)|
|<br> <br> <br>**num_subset_seq_parameter_sets[** i**] **|5|ue(v)|
|<br> <br> <br>for( j = 0; j < num_subset_seq_parameter_sets[ i ]; j++ )|||
|<br> <br> <br> <br>**subset_seq_parameter_set_id_delta[** i**][** j**]**|5|ue(v)|
|<br> <br> <br>**num_pic_parameter_sets_minus1[** i**] **|5|ue(v)|






|for( j = 0; j <= num_pic_parameter_sets_minus1[ i ]; j++ )|Col2|Col3|
|---|---|---|
|<br> <br> <br> <br>**pic_parameter_set_id_delta[** i**][** j**]**|5|ue(v)|
|<br> <br>} else|||
|<br> <br> <br>**parameter_sets_info_src_op_id[** i**]**|5|ue(v)|
|<br> <br>if( bitstream_restriction_info_present_flag[ i ] ) {|||
|<br> <br> <br>**motion_vectors_over_pic_boundaries_flag[** i**]**|5|u(1)|
|<br> <br> <br>**max_bytes_per_pic_denom[** i**]**|5|ue(v)|
|<br> <br> <br>**max_bits_per_mb_denom[** i**]**|5|ue(v)|
|<br> <br> <br>**log2_max_mv_length_horizontal[** i**]**|5|ue(v)|
|<br> <br> <br>**log2_max_mv_length_vertical[** i**]**|5|ue(v)|
|<br> <br> <br>**max_num_reorder_frames[** i**]**|5|ue(v)|
|<br> <br> <br>**max_dec_frame_buffering[** i**]**|5|ue(v)|
|<br> <br>}|||
|<br>}|||
|}|||


**H.13.1.4** **Multiview scene information SEI message syntax**

|multiview_scene_info( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**max_disparity**|5|ue(v)|
|}|||



**H.13.1.5** **Multiview acquisition information SEI message syntax**

|multiview_acquisition_info( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**num_views_minus1**||ue(v)|
|<br>**intrinsic_param_flag**|5|u(1)|
|<br>**extrinsic_param_flag**|5|u(1)|
|<br>if( instrinsic_param_flag ) {|||
|<br> <br>**intrinsic_params_equal_flag**|5|u(1)|
|<br> <br>**prec_focal_length**|5|ue(v)|
|<br> <br>**prec_principal_point**|5|ue(v)|
|<br> <br>**prec_skew_factor**|5|ue(v)|
|<br> <br>for( i = 0; i <= intrinsic_params_equal_flag ? 0 : num_views_minus1;<br> <br> <br> <br>i++ ) {|||
|<br> <br> <br>**sign_focal_length_x[** i**] **|5|u(1)|
|<br> <br> <br>**exponent_focal_length_x[** i**] **|5|u(6)|
|<br> <br> <br>**mantissa_focal_length_x[** i**]**|5|u(v)|
|<br> <br> <br>**sign_focal_length_y[** i**] **|5|u(1)|
|<br> <br> <br>**exponent_focal_length_y[** i**] **|5|u(6)|
|<br> <br> <br>**mantissa_focal_length_y[** i**] **|5|u(v)|
|<br> <br> <br>**sign_principal_point_x[** i**] **|5|u(1)|
|<br> <br> <br>**exponent_principal_point_x[** i**] **|5|u(6)|
|<br> <br> <br>**mantissa_principal_point_x[** i**] **|5|u(v)|
|<br> <br> <br>**sign_principal_point_y[** i**] **|5|u(1)|
|<br> <br> <br>**exponent_principal_point_y[** i**] **|5|u(6)|
|<br> <br> <br>**mantissa_principal_point_y[** i**]**|5|u(v)|






|sign_skew_factor[ i ]|5|u(1)|
|---|---|---|
|<br> <br> <br>**exponent_skew_factor[** i**] **|5|u(6)|
|<br> <br> <br>**mantissa_skew_factor[** i**] **|5|u(v)|
|<br> <br>}|||
|<br>}|||
|<br>if( extrinsic_param_flag ) {|||
|<br> <br>**prec_rotation_param**|5|ue(v)|
|<br> <br>**prec_translation_param**|5|ue(v)|
|<br> <br>for( i = 0; i <= num_views_minus1; i++ ) {|||
|<br> <br> <br>for( j = 1; j <= 3; j++ ) { /* row */|||
|<br> <br> <br> <br>for( k = 1; k <= 3; k++ ) { /* column */|||
|<br> <br> <br> <br> <br>**sign_r[** i**][** j**][** k**] **|5|u(1)|
|<br> <br> <br> <br> <br>**exponent_r[** i**][** j**][** k**]**|5|u(6)|
|<br> <br> <br> <br> <br>**mantissa_r[** i**][** j**][** k**]**|5|u(v)|
|<br> <br> <br> <br>}|||
|<br> <br> <br> <br>**sign_t[**i **][**j **]**|5|u(1)|
|<br> <br> <br> <br>**exponent_t[**i **][**j **]**|5|u(6)|
|<br> <br> <br> <br>**mantissa_t[**i **][**j **]**|5|u(v)|
|<br> <br> <br>}|||
|<br> <br>}|||
|<br>}|||
|}|||


**H.13.1.6Non-required view component SEI message syntax**

|non_required_view_component( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**num_info_entries_minus1**|5|ue(v)|
|<br>for( i = 0; i <= num_info_entries_minus1; i++ ) {|||
|<br> <br>**view_order_index[** i**] **|5|ue(v)|
|<br> <br>**num_non_required_view_components_minus1[** i**] **|5|ue(v)|
|<br> <br>for( j = 0; j <= num_non_required_view_components_minus1[ i ]; j++ )|||
|<br> <br> <br>**index_delta_minus1[** i**][** j**]**|5|ue(v)|
|<br>}|||
|}|||






**H.13.1.7View dependency change SEI message syntax**

|view_dependency_change( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**seq_parameter_set_id**|5|ue(v)|
|<br>**anchor_update_flag**|5|u(1)|
|<br>**non_anchor_update_flag**|5|u(1)|
|<br>if( anchor_update_flag )|||
|<br> <br>for( i = 1; i <= num_views_minus1; i++ ) {|||
|<br> <br> <br>for( j = 0; j < num_anchor_refs_l0[ i ]; j++ )|||
|<br> <br> <br> <br>**anchor_ref_l0_flag[** i**][** j**]**|5|u(1)|
|<br> <br> <br>for( j = 0; j < num_anchor_refs_l1[ i ]; j++ )|||
|<br> <br> <br> <br>**anchor_ref_l1_flag[** i**][** j**]**|5|u(1)|
|<br> <br>}|||
|<br>if( non_anchor_update_flag )|||
|<br> <br>for( i = 1; i <= num_views_minus1; i++ ) {|||
|<br> <br> <br>for( j = 0; j < num_non_anchor_refs_l0[ i ]; j++ )|||
|<br> <br> <br> <br>**non_anchor_ref_l0_flag[** i**][** j**]**|5|u(1)|
|<br> <br> <br>for( j = 0; j < num_non_anchor_refs_l1[ i ]; j++ )|||
|<br> <br> <br> <br>**non_anchor_ref_l1_flag[** i**][** j**]**|5|u(1)|
|<br> <br>}|||
|}|||



**H.13.1.8Operation point not present SEI message syntax**

|operation_point_not_present( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**num_operation_points**|5|ue(v)|
|<br>for( k = 0; k < num_operation_points; k++ )|||
|<br> <br>**operation_point_not_present_id[** k**] **|5|ue(v)|
|}|||



**H.13.1.9Base view temporal HRD SEI message syntax**

|base_view_temporal_hrd( payloadSize ) {|C|Descriptor|
|---|---|---|
|**num_of_temporal_layers_in_base_view_minus1**|5|ue(v)|
|for( i = 0; i <= num_of_temporal_layers_in_base_view_minus1; i++ ) {|||
|<br> <br>**sei_mvc_temporal_id[**i** ]**|5|u(3)|
|<br> **sei_mvc_timing_info_present_flag[**i** ]**|5|u(1)|
|<br> <br>if( sei_mvc_timing_info_present_flag[ i ] ) {|||
|<br> <br> <br>**sei_mvc_num_units_in_tick[**i** ]**|5|u(32)|
|<br> <br> <br>**sei_mvc_time_scale[**i** ]**|5|u(32)|
|<br> <br> <br>**sei_mvc_fixed_frame_rate_flag[**i** ]**|5|u(1)|
|<br> <br>}|||
|<br> <br>**sei_mvc_nal_hrd_parameters_present_flag[**i** ]**|5|u(1)|
|<br> <br>if( sei_mvc_nal_hrd_parameters_present_flag[ i ] )|||
|<br> <br> <br>hrd_parameters( )|5||
|<br> <br>**sei_mvc_vcl_hrd_parameters_present_flag[**i** ]**|5|u(1)|
|<br> <br>if( sei_mvc_vcl_hrd_parameters_present_flag[ i ] )|||






|hrd_parameters( )|5|Col3|
|---|---|---|
|<br> <br>if( sei_mvc_nal_hrd_parameters_present_flag[ i ] | |<br> <br> <br> <br>sei_mvc_vcl_hrd_parameters_present_flag[ i ] )|||
|<br> <br> <br>**sei_mvc_low_delay_hrd_flag[**i** ]**|5|u(1)|
|<br> <br>**sei_mvc_pic_struct_present_flag[**i** ]**|5|u(1)|
|<br>}|||
|}|||


**H.13.1.10** **Multiview view position SEI message syntax**

|multiview_view_position( payloadSize ) {|C|Descriptor|
|---|---|---|
|**num_views_minus1**|5|ue(v)|
|<br>for( i = 0; i <= num_views_minus1; i++ )|||
|<br> **view_position[**i** ]**|5|ue(v)|
|<br>**multiview_view_position_extension_flag**|5|u(1)|
|}|||



**H.13.2** **SEI message semantics**


Depending on payloadType, the corresponding SEI message semantics are extended as follows:


- If payloadType is equal to 2, 3, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 45, 47, 137, 142, 144,
147, 148, 149, 150, 151, 154, 155, 156, 200, 201, 202, or 205, the following applies:


   - If the SEI message is not included in an MVC scalable nesting SEI message, it applies to the view component of
the current access unit with VOIdx equal to VOIdxMin.


   - Otherwise (the SEI message is included in an MVC scalable nesting SEI message), it applies to all view
components of the current access unit when all_view_components_in_au_flag is equal to 1, or it applies to all
view components of the current access unit with view_id equal to sei_view_id[ i ] for any i in the range of 0 to
num_view_components_minus1, inclusive, when all_view_components_in_au_flag is equal to 0. When
payloadType is equal to 10 for the SEI message that is included in an MVC scalable nesting SEI message, the
semantics for sub_seq_layer_num of the sub-sequence information SEI message is modified as follows:


**sub_seq_layer_num** specifies the sub-sequence layer number of the current picture. When the current
picture resides in a sub-sequence for which the first picture in decoding order is an IDR picture, the value
of sub_seq_layer_num shall be equal to 0. For a non-paired reference field, the value of sub_seq_layer_num
shall be equal to 0. sub_seq_layer_num shall be in the range of 0 to 255, inclusive.


- Otherwise, if payloadType is equal to 0 or 1, the following applies:


   - If the SEI message is not included in an MVC scalable nesting SEI message, the following applies. When the
SEI message and all other SEI messages with payloadType equal to 0 or 1 not included in an MVC scalable
nesting SEI message are used as the buffering period and picture timing SEI messages for checking the bitstream
conformance according to Annex C and the decoding process specified in clauses 2 to 9 is used, the bitstream
shall be conforming to this Recommendation | International Standard.


   - Otherwise (the SEI message is included in an MVC scalable nesting SEI message), the following applies. When
the SEI message and all other SEI messages with payloadType equal to 0 or 1 included in an MVC scalable
nesting SEI message with identical values of sei_op_temporal_id and sei_op_view_id[ i ] for all i in the range
of 0 to num_view_components_op_minus1, inclusive, are used as the buffering period and picture timing SEI
messages for checking the bitstream conformance according to Annex C, the bitstream that would be obtained
by invoking the bitstream extraction process as specified in clause H.8.3 with tIdTarget equal to
sei_op_temporal_id and viewIdTargetList equal to sei_op_view_id[ i ] for all i in the range of 0 to
num_view_components_op_minus1, inclusive, shall be conforming to this Recommendation |
International Standard.


In the semantics of clauses D.2.1 and D.2.3, the syntax elements num_units_in_tick, time_scale,
fixed_frame_rate_flag, nal_hrd_parameters_present_flag, vcl_hrd_parameters_present_flag,
low_delay_hrd_flag, and pic_struct_present_flag and the derived variables NalHrdBpPresentFlag,





VclHrdBpPresentFlag, and CpbDpbDelaysPresentFlag are substituted with the syntax elements
vui_mvc_num_units_in_tick[ i ], vui_mvc_time_scale[ i ], vui_mvc_fixed_frame_rate_flag[ i ],
vui_mvc_nal_hrd_parameters_present_flag[ i ], vui_mvc_vcl_hrd_parameters_present_flag[ i ],
vui_mvc_low_delay_hrd_flag[ i ], and vui_mvc_pic_struct_present_flag[ i ] and the derived variables
VuiMvcNalHrdBpPresentFlag[ i ], VuiMvcVclHrdBpPresentFlag[ i ], and
VuiMvcCpbDpbDelaysPresentFlag[ i ].


The values of seq_parameter_set_id's in all buffering period SEI messages included in MVC scalable nesting
SEI messages and associated with operation points for which the greatest VOIdx values in the associated
bitstream subsets are identical shall be identical.


- Otherwise (all remaining payloadType values), the corresponding SEI message semantics are not extended.


When an SEI message having a particular value of payloadType equal to 137 or 144, contained in an MVC scalable nesting
SEI message, and applying to a particular combination of a list of view_id and a temporal_id is present in an access unit,
the SEI message with the particular value of payloadType applying to the particular combination of a list of view_id and
a temporal_id shall be present an MVC scalable nesting SEI message in the IDR access unit that is the first access unit of
the coded video sequence.


All SEI messages having a particular value of payloadType equal to 137 or 144, contained in MVC scalable nesting SEI
messages, and applying to a particular combination of a list of view_id and a temporal_id present in a coded video sequence
shall have the same content.


For the semantics of SEI messages with payloadType in the range of 0 to 23, inclusive, or equal to 45, 47, 137, 142, 144,
147, 148, 149, 150, 151, 154, 155, 156, 200, 201, 202, or 205, which are specified in clause D.2, MVC sequence parameter
set is substituted for sequence parameter set; the parameters of MVC sequence parameter set RBSP and picture parameter
set RBSP that are in effect are specified in clauses H.7.4.2.1 and H.7.4.2.2, respectively.


Coded video sequences conforming to one or more of the profiles specified in Annex H shall not include SEI NAL units
that contain SEI messages with payloadType in the range of 24 to 35, inclusive, which are specified in clause G.13.


When an SEI NAL unit contains an SEI message with payloadType in the range of 36 to 44, inclusive, or equal to 46,
which are specified in clause H.13, it shall not contain any SEI messages with payloadType less than 36 or equal to 45, 47,
137, 142, 144, 147, 148, 149, 150, 151, 154, 155, 156, 200, 201, 202, or 205, and the first SEI message in the SEI NAL
unit shall have payloadType in the range of 36 to 44, inclusive, or equal to 46.


When an MVC scalable nesting SEI message (payloadType equal to 37) or a view scalability information SEI message
(payloadType equal to 38) or an operation point not present SEI message (payloadType equal to 43) is present in an SEI
NAL unit, it shall be the only SEI message in the SEI NAL unit.


**H.13.2.1Parallel decoding information SEI message semantics**


The parallel decoding information SEI message may be associated with any access unit. The information signalled in the
SEI message applies to all the access units from the access unit the SEI message is associated with to the next access unit,
in decoding order, containing an SEI message of the same type, exclusively, or to the end of the coded video sequence,
whichever is earlier in decoding order.


Some view components for which the parallel decoding information is signalled in a parallel decoding information SEI
message may not be present in the coded video sequence.


**seq_parameter_set_id** specifies a subset sequence parameter set that contains the inter-view dependency relationship
information. The value of seq_parameter_set_id shall be equal to the value of seq_parameter_set_id in the picture
parameter set referenced by a view component of the primary coded picture of the access unit containing the parallel
decoding information SEI message. The value of seq_parameter_set_id shall be in the range of 0 to 31, inclusive.


NOTE 1 – The inter-view dependency relationship is signalled in the sequence parameter set MVC extension, which is identical
for all subset sequence parameter sets that may be activated during the decoding process for the coded video sequence.


**pdi_init_delay_anchor_minus2_l0[** i **][** j **]** specifies the unavailable reference area in the view component with view_id
equal to anchor_ref_l0[ i ][ j ] that shall not be used for inter-view reference by the coded anchor view component with
view_id equal to view_id[ i ], where anchor_ref_l0[ i ][ j ] and view_id[ i ] are both from the MVC sequence parameter set
that has a sequence parameter set identifier equal to the syntax element seq_parameter_set_id contained in the current SEI
message. The unavailable reference area is a rectangular area with coordinates
( 0, (CurrMbAddr / PicWidthInMbs + pdi_init_delay_anchor_minus2_l0[ i ][ j ] + 2 ) * 16 ) as the top left corner and
( PicWidthInSamples, PicHeightInSamples ) as the bottom right corner. When decoding the coded view component with
view_id equal to view_id[ i ], samples from the unavailable reference area from the view component with view_id equal
to anchor_ref_l0[ i ][ j ] shall not be referred to by the inter-view prediction process. The value of
pdi_init_delay_anchor_minus2_l0[ i ][ j ] shall be in the range of 0 to PicHeightInMbs − 2, inclusive.





**pdi_init_delay_anchor_minus2_l1[** i **][** j **]** specifies the unavailable reference area in the view component with view_id
equal to anchor_ref_l1[ i ][ j ] that shall not be used for inter-view reference by the coded anchor view component with
view_id equal to view_id[ i ], where anchor_ref_lX[ i ][ j ] and view_id[ i ] are both from the MVC sequence parameter
set that has a sequence parameter set identifier equal to the syntax element seq_parameter_set_id contained in the current
SEI message. The unavailable reference area is a rectangular area with coordinates
( 0, ( CurrMbAddr / PicWidthInMbs + pdi_init_delay_anchor_minus2_l1[i ][ j ] + 2) * 16 ) as the top left corner and
( PicWidthInSamples, PicHeightInSamples ) as the bottom right corner. When decoding the coded view component with
view_id equal to view_id[ i ], samples from the unavailable reference area from the view component with view_id equal
to anchor_ref_l1[ i ][ j ] shall not be referred to by the inter-view prediction process. The value of
pdi_init_delay_anchor_minus2_l1[ i ][ j ] shall be in the range of 0 to PicHeightInMbs − 2, inclusive.


**pdi_init_delay_non_anchor_minus2_l0[** i **][** j **]** specifies the unavailable reference area in the view component with
view_id equal to non_anchor_ref_l0[ i ][ j ] that shall not be used for inter-view reference by the coded non-anchor view
component with view_id equal to view_id[ i ], where non_anchor_ref_l0[ i ][ j ] and view_id[ i ] are both from the MVC
sequence parameter set that has a sequence parameter set identifier equal to the syntax element seq_parameter_set_id
contained in the current SEI message. The unavailable reference area is a rectangular area with coordinates
( 0, (CurrMbAddr / PicWidthInMbs + pdi_init_delay_non_anchor_minus2_l0[ i ][ j ] + 2) * 16 ) as the top left corner and
( PicWidthInSamples, PicHeightInSamples ) as the bottom right corner. When decoding the coded view component with
view_id equal to view_id[ i ], samples from the unavailable reference area from the view component with view_id equal
to non_anchor_ref_l0[ i ][ j ] shall not be referred to by the inter-view prediction process. The value of
pdi_init_delay_non_anchor_minus2_l0[ i ][ j ] shall be in the range of 0 to PicHeightInMbs − 2, inclusive.


**pdi_init_delay_non_anchor_minus2_l1[** i **][** j **]** specifies the unavailable reference area in the view component with
view_id equal to non_anchor_ref_l1[ i ][ j ] that shall not be used for inter-view reference by the coded anchor view
component with view_id equal to view_id[ i ], where non_anchor_ref_lX[ i ][ j ] and view_id[ i ] are both from the MVC
sequence parameter set that has a sequence parameter set identifier equal to the syntax element seq_parameter_set_id
contained in the current SEI message. The unavailable reference area is a rectangular area with coordinates
( 0, (CurrMbAddr / PicWidthInMbs + pdi_init_delay_non_anchor_minus2_l1[i ][ j ] + 2) * 16 ) as the top left corner and
( PicWidthInSamples, PicHeightInSamples ) as the bottom right corner. When decoding the coded view component with
view_id equal to view_id[ i ], samples from the unavailable reference area from the view component with view_id equal
to non_anchor_ref_l1[ i ][ j ] shall not be referred to by the inter-view prediction process. The value of
pdi_init_delay_non_anchor_minus2_l1[ i ][ j ] shall be in the range of 0 to PicHeightInMbs − 2, inclusive.


**H.13.2.2MVC scalable nesting SEI message semantics**


An MVC nesting SEI message shall contain one and only one SEI message of payloadType less than or equal to 23, which
is referred to as the nested SEI message. The scope to which the nested SEI message applies is indicated by the syntax
elements operation_point_flag, all_view_components_in_au_flag, num_view_components_minus1, sei_view_id[ i ] for
all i, num_view_components_op_minus1, sei_op_view_id[ i ] for all i, and sei_op_temporal_id.


Some view components to which the nested SEI message applies may not be present in the access unit containing the MVC
scalable nesting SEI message.


**operation_point_flag** equal to 1 specifies that the nested SEI message applies to the current access unit when the
associated operation point identified by sei_op_temporal_id and sei_op_view_id[ i ] for all i in the range of 0 to
num_view_components_op_minus1, inclusive, is decoded. operation_point_flag equal to 0 specifies that the nested SEI
message applies to the view components identified by all_view_components_in_au_flag, num_view_components_minus1,
and sei_view_id[ i ] for all i in the range of 0 to num_view_components_minus1, inclusive, regardless of which operation
point is decoded.


If the nested SEI message has payloadType equal to 0 or 1, operation_point_flag shall be equal to 1. Otherwise (the nested
SEI message has payloadType not equal to 0 or 1), operation_point_flag shall be equal to 0.


**all_view_components_in_au_flag** equal to 1 specifies that the nested SEI message applies to all view components of the
access unit. all_view_components_in_au_flag equal to 0 specifies that the applicable scope of the nested SEI message is
signalled by the syntax elements num_view_components_minus1 and sei_view_id[ i ] for all i in the range of 0 to
num_view_components_minus1, inclusive.


**num_view_components_minus1** plus 1 specifies the number of view components to which the nested SEI message
applies when operation_point_flag is equal to 0 and all_view_components_in_au_flag is equal to 0. The value of
num_view_components_minus1 shall be in the range of 0 to 1023, inclusive.


**sei_view_id** [ i ] specifies the view_id of the i-th view component to which the nested SEI message applies when
operation_point_flag is equal to 0 and all_view_components_in_au_flag is equal to 0.


**num_view_components_op_minus1** plus 1 specifies the number of view components of the operation point to which the
nested SEI message applies when operation_point_flag is equal to 1. The value of num_view_components_op_minus1
shall be in the range of 0 to 1023, inclusive.





**sei_op_view_id** [ i ] specifies the view_id of the i-th view component to which the nested SEI message applies when
operation_point_flag is equal to 1.


**sei_op_temporal_id** specifies the maximum temporal_id of the bitstream subset to which the nested SEI message applies
when operation_point_flag is equal to 1.


**sei_nesting_zero_bit** is equal to 0.


**H.13.2.3View scalability information SEI message semantics**


When present, this SEI message shall be associated with an IDR access unit. The semantics of the message are valid for
the current coded video sequence. A view scalability information SEI message contains view and scalability information
for a subset of the operation points in the coded video sequence. Each operation point is associated with an operation point
identifier. The sub-bitstream for an operation point is referred to as the operation point representation or the representation
of the operation point. Information such as bit rate and frame rate, among others, are signalled for the representations of
the subset of the operation points.


NOTE 1 – Any operation point for which view and scalability information is signalled in a view scalability information SEI message
(i.e. identified by a value of operation_point_id[ i ]) must be present in the coded video sequence. When an application keeps a
view scalability information SEI message in a sub-bitstream extracted according to the process specified in clause H.8.5.3, and
after the extraction any operation point for which view and scalability information is signalled in the original SEI message becomes
not present in the coded video sequence, the application must change the content of the view scalability information SEI message
to fulfil the condition stated by the first sentence in this note.


**num_operation_points_minus1** plus 1 specifies the number of operation points that are present in the coded video
sequence and for which the view scalability information is signalled by the following syntax elements. The value of
num_operation_points_minus1 shall be in the range of 0 to 1023, inclusive.


The bitstream subset corresponding to an operation point is defined as the operation point representation or the
representation of the operation point. The representation of the operation point identified by operation_point_id[ i ] is the
output of the sub-bitstream extraction process specified in clause H.8.5.3 with tIdTarget equal to temporal_id[ i ] and
viewIdTargetList consisting of view_id[ i ][ j ] for all j in the range of 0 to num_target_output_views_minus1[ i ],
inclusive, as the inputs.


**operation_point_id[** i **]** specifies the identifier of the operation point. Each operation point is associated with a unique
operation point identifier. The value of operation_point_id[ i ] shall be in the range of 0 to 65535, inclusive.


In the following semantics in this clause, the operation point with identifier equal to operation_point_id[ i ] is referred to
as the current operation point.


**priority_id[** i **]** and **temporal_id[** i **]** specify the maximum value of priority_id and temporal_id, respectively, of the NAL
units in the representation of the current operation point.


**num_target_output_views_minus1[** i **]** plus 1 specifies the number of target output views for the current operation point.
The value of num_target_output_views_minus1[ i ] shall be in the range of 0 to 1023, inclusive.


**view_id[** i **][** j **]** specifies the identifier of the j-th target output view for the current operation point. The value of
view_id[ i ][ j ] shall be in the range of 0 to 1023, inclusive.


**profile_level_info_present_flag[** i **]** equal to 1 specifies that the profile and level information for the representation of the
current operation point is present in the SEI message. profile_level_info_present_flag[ i ] equal to 0 specifies that the
profile and level information for the current operation point is not present in the SEI message.


**bitrate_info_present_flag[** i **]** equal to 1 specifies that the bitrate information for the current operation point is present in
the SEI message. bitrate_info_present_flag[ i ] equal to 0 specifies that the bitrate information for the current operation
point is not present in the SEI message.


**frm_rate_info_present_flag[** i **]** equal to 1 specifies that the frame rate information for the current operation point is
present in the SEI message. frm_rate_info_present_flag[ i ] equal to 0 specifies that the frame rate information for the
current operation point is not present in the SEI message.


**view_dependency_info_present_flag[** i **]** equal to 1 specifies that information on the views the target output view of the
current operation point directly depends on is present in the SEI message. View A is directly dependent on view point B if
there is at least one view component of view A using a view component of view B for inter-view prediction reference.
view_dependency_info_present_flag[ i ] equal to 0 specifies that view_dependency_info_src_op_id[ i ] is present in the
SEI message. When not present, view_dependency_info_present_flag[ i ] shall be inferred to be equal to 0.





NOTE 2 – The inter-view dependency relationship signalled in sequence parameter set MVC extension is an upper bound, in the
sense that whenever view A may depend on view B at any access unit, it is specified as view A depends on view B. Therefore, the
dependency relationship is indicated by sequence parameter set MVC extension when view A depends on view B at only one of all
access units in the coded video sequence, or even when view A actually does not depend on view B at any access unit but when
generating the sequence parameter set MVC extension the encoder thought view A might depend on view B. The dependency
relationship signalled here can be more refined. For example, when view A depends on view B at access units with temporal_id
equal to 0 but not at other access units, this can be indicated through the view dependency information signalled in this SEI message
for operation points with view A as the target output view and with different maximum values of temporal_id.


**parameter_sets_info_present_flag[** i **]** equal to 1 specifies that the values of seq_parameter_set_id of the sequence
parameter sets and subset sequence parameter sets and the values of pic_parameter_set_id of the picture parameter sets
that are referred to by the VCL NAL units of the representation of the current operation point are present in the SEI
message. parameter_sets_info_present_flag[ i ] equal to 0 specifies that parameter_sets_info_src_op_id[ i ] is present in
the SEI message.


**bitstream_restriction_info_present_flag[** i **]** equal to 1 specifies that the bitstream restriction information for the
representation of the current operation point is present in the SEI message. bitstream_restriction_info_present_flag[ i ]
equal to 0 specifies that the bitstream restriction information for the representation of the current operation point is not
present in the SEI message.


**op_profile_level_idc[** i **]** specifies the profile and level compliancy of the representation of the current operation point.
op_profile_level_idc[ i ] is the exact copy of the three bytes comprised of profile_idc, constraint_set0_flag,
constraint_set1_flag, constraint_set2_flag, constraint_set3_flag, constraint_set4_flag, constraint_set5_flag,
reserved_zero_2bits, and level_idc, if these syntax elements were used to specify the profile and level compliancy of the
representation of the current operation point as specified in Annexes A and H.


**avg_bitrate[** i **]** specifies the average bit rate of the representation of the current operation point. The average bit rate for
the representation of the current operation point in bits per second is given by BitRateBPS( avg_bitrate[ i ] ) with the
function BitRateBPS( ) being specified by the following equation.


BitRateBPS( x ) = ( x & ( 2 [14]        - 1 ) ) * 10 [( 2 + ( x >> 14 ) )] (H-82)


All NAL units of the representation of the current operation point are taken into account in the calculation. The average
bit rate is derived according to the access unit removal time specified in Annex C. In the following, bTotal is the number
of bits in all NAL units of the representation of the current operation point in the current coded video sequence. t1 is the
removal time (in seconds) of the current access unit, and t2 is the removal time (in seconds) of the last access unit (in
decoding order) of the current coded video sequence.


With x specifying the value of avg_bitrate[ i ], the following applies:


- If t1 is not equal to t2, the following condition shall be true.


( x & ( 2 [14]         - 1 ) ) = = Round( bTotal ÷ ( ( t2 − t1 ) * 10 [( 2 + ( x >> 14 ) )] ) ) (H-83)


- Otherwise (t1 is equal to t2), the following condition shall be true.


( x & ( 2 [14]         - 1 ) ) = = 0 (H-84)


**max_bitrate[** i **]** specifies the maximum bit rate of the representation of the current operation point, given by
BitRateBPS( max_bitrate_layer_representation[ i ] ), in bits per second, with the function BitRateBPS( ) being specified
in Equation H-82. The maximum bit rate of the representation of the current operation point is calculated based on a time
window specified by max_bitrate_calc_window[ i ].


**max_bitrate_calc_window[** i **]** specifies the length of the time window, in units of 1/100 second, based on which
max_bitrate[ i ] is calculated.


**constant_frm_rate_idc[** i **]** specifies whether the frame rate of the representation of the current operation point is constant.
If the value of avg_frm_rate as specified below is constant whichever a temporal section of the operation point
representation is used for the calculation, the frame rate is constant, otherwise the frame rate is non-constant.
constant_frm_rate_idc[ i ] equal to 0 specifies that the frame rate is not constant, constant_frm_rate_idc[ i ] equal to 1
specifies that the frame rate is constant, and constant_frm_rate_idc[ i ] equal to 2 specifies that the frame rate may be or
may not be constant. The value of constant_frm_rate_idc[ i ] shall be in the range of 0 to 2, inclusive.


**avg_frm_rate[** i **]** specifies the average frame rate, in units of frames per 256 seconds, of the representation of the current
operation point. The semantics of avg_frm_rate[ i ] is identical to the semantics of average_frame_rate in sub-sequence
layer characteristics SEI message when accurate_statistics_flag is equal to 1, except that herein the set of NAL units in the
range of sub-sequence layers is replaced by the set of NAL units of the representation of the current operation point.





**num_directly_dependent_views[** i **]** specifies the number of views that the target output view of the current operation
point is directly dependent on within the representation of the current operation point. The value of
num_directly_dependent_views[ i ] shall be in the range of 0 to 16, inclusive.


**directly_dependent_view_id[** i **][** j **]** specifies the view_id of the j-th view that the target output view of the current
operation point is directly dependent on within the representation of the current operation point. The value of
directly_dependent_view_id[ i ][ j ] shall be in the range of 0 to 1023, inclusive.


**view_dependency_info_src_op_id[** i **]** specifies that the views the target output view of the current operation point
directly depends on within the representation of the current operation point are the same as the views the target output view
of the operation point with identifier equal to view_dependency_info_src_op_id[ i ] directly depends on within the
representation of the operation point with identifier equal to view_dependency_info_src_op_id[ i ], if
view_dependency_info_src_op_id[ i ] is not equal to operation_point_id[ i ]. Otherwise
(view_dependency_info_src_op_id[ i ] is equal to operation_point_id[ i ]), information on the views the target output view
of the current operation point directly depends on is not present in the SEI message. The value of
view_dependency_info_src_op_id[ i ] shall be in the range of 0 to 65535, inclusive.


**num_seq_parameter_sets[** i **]** specifies the number of different sequence parameter sets that are referred to by the VCL
NAL units of the representation of the current operation point. The value of num_seq_parameter_sets[ i ] shall be in the
range of 0 to 32, inclusive.


**seq_parameter_set_id_delta[** i **][** j **]** specifies the smallest value of the seq_parameter_set_id of all sequence parameter
sets required for decoding the representation of the current operation point, if j is equal to 0. Otherwise (j is greater than 0),
seq_parameter_set_id_delta[ i ][ j ] specifies the difference between the value of the seq_parameter_set_id of the j-th
required sequence parameter set and the value of the seq_parameter_set_id of the (j−1)-th required sequence parameter set
for decoding the representation of the current operation point. The sequence parameter sets are logically ordered in
ascending order of the value of seq_parameter_set_id. The value of seq_parameter_set_id_delta[ i ][ j ] shall be in the
range of 0 to 31, inclusive.


**num_subset_seq_parameter_sets[** i **]** specifies the number of different subset sequence parameter sets that are referred
to by the VCL NAL units of the representation of the current operation point. The value of
num_subset_seq_parameter_sets[ i ] shall be in the range of 0 to 32, inclusive.


**subset_seq_parameter_set_id_delta[** i **][** j **]** specifies the smallest value of the seq_parameter_set_id of all subset
sequence parameter sets required for decoding the representation of the current operation point, if j is equal to 0. Otherwise
(j is greater than 0), subset_seq_parameter_set_id_delta[ i ][ j ] specifies the difference between the value of the
seq_parameter_set_id of the j-th required subset sequence parameter set and the value of the seq_parameter_set_id of the
(j−1)-th required subset sequence parameter set for decoding the representation of the current operation point. The subset
sequence parameter sets are logically ordered in ascending order of the value of seq_parameter_set_id. The value of
subset_seq_parameter_set_id_delta[ i ][ j ] shall be in the range of 0 to 31, inclusive.


**num_pic_parameter_sets_minus1[** i **]** plus 1 specifies the number of different picture parameter sets that are referred to
by the VCL NAL units of the representation of the current operation point. The value of
num_pic_parameter_sets_minus1[ i ] shall be in the range of 0 to 255, inclusive.


**pic_parameter_set_id_delta[** i **][** j **]** specifies the smallest value of the pic_parameter_set_id of all picture parameter sets
required for decoding the representation of the current operation point, if j is equal to 0. Otherwise (j is greater than 0),
pic_parameter_set_id_delta[ i ][ j ] specifies the difference between the value of the pic_parameter_set_id of the j-th
required picture parameter set and the value of the pic_parameter_set_id of the (j−1)-th required picture parameter set for
decoding the representation of the current operation point. The picture parameter sets are logically ordered in ascending
order of the value of pic_parameter_set_id. The value of pic_parameter_set_id_delta[ i ][ j ] shall be in the range of 0
to 255, inclusive.


**parameter_sets_info_src_op_id[** i **]** specifies that the values of seq_parameter_set_id of the sequence parameter sets and
subset sequence parameter sets and the values of pic_parameter_set_id of the picture parameter sets that are referred to by
the VCL NAL units of the representation of the current operation point are the same as those for the representation of the
operation point with identifier equal to parameter_sets_info_src_op_id[ i ], if parameter_sets_info_src_op_id[ i ] is not
equal to operation_point_id[ i ]. Otherwise (parameter_sets_info_src_op_id[ i ] is equal to operation_point_id[ i ]),
parameter_sets_info_src_op_id[ i ] specifies that the values of seq_parameter_set_id of the sequence parameter sets and
subset sequence parameter sets and the values of pic_parameter_set_id of the picture parameter sets that are referred to by
the VCL NAL units of the representation of the current operation point are not present in the SEI message. The value of
parameter_sets_info_src_op_id[ i ] shall be in the range of 0 to 65535, inclusive.


**motion_vectors_over_pic_boundaries_flag[** i **]** specifies the value of motion_vectors_over_pic_boundaries_flag, as
specified in clause E.2.1, for the current operation point representation. When the
motion_vectors_over_pic_boundaries_flag[ i ] syntax element is not present, motion_vectors_over_pic_boundaries_flag
value for the current operation point representation shall be inferred to be equal to 1.





**max_bytes_per_pic_denom[** i **]** specifies the max_bytes_per_pic_denom value, as specified in clause E.2.1, for the
current operation point representation. When the max_bytes_per_pic_denom[ i ] syntax element is not present, the value
of max_bytes_per_pic_denom for the current operation point representation shall be inferred to be equal to 2. The value
of max_bytes_per_pic_denom[ i ] shall be in the range of 0 to 16, inclusive.


**max_bits_per_mb_denom[** i **]** specifies the max_bits_per_mb_denom value, as specified in clause E.2.1, for the current
operation point representation. When the max_bits_per_mb_denom[ i ] is not present, the value of
max_bits_per_mb_denom for the current operation point representation shall be inferred to be equal to 1. The value of
max_bits_per_mb_denom[ i ] shall be in the range of 0 to 16, inclusive.


**log2_max_mv_length_horizontal[** i **]** and **log2_max_mv_length_vertical[** i **]** specify the values of
log2_max_mv_length_horizontal and log2_max_mv_length_vertical, as specified in clause E.2.1, for the current operation
point representation. When log2_max_mv_length_horizontal[ i ] is not present, the values of
log2_max_mv_length_horizontal and log2_max_mv_length_vertical for the current operation point representation shall
be inferred to be equal to 16. The value of log2_max_mv_length_horizontal[ i ] shall be in the range of 0 to 16, inclusive.
The value of log2_max_mv_length_vertical[ i ] shall be in the range of 0 to 16, inclusive.


NOTE 3 – The maximum absolute value of a decoded vertical or horizontal motion vector component is also constrained by profile
and level limits as specified in Annex A or clause H.10.2.


**max_num_reorder_frames[** i **]** specifies the value of max_num_reorder_frames, as specified in clause E.2.1, for the
current operation point representation. The value of max_num_reorder_frames[ i ] shall be in the range of 0 to 16,
inclusive. When the max_num_reorder_frames[ i ] syntax element is not present, the value of max_num_reorder_frames
for the current operation point representation shall be inferred to be equal to 16.


**max_dec_frame_buffering[** i **]** specifies the value of max_dec_frame_buffering, as specified in clause E.2.1, for the
current operation point representation. The value of max_dec_frame_buffering[ i ] shall be in the range of 0 to
MaxDpbFrames (as specified in clauses A.3.1, A.3.2, or H.10.2), inclusive. When the max_dec_frame_buffering[ i ]
syntax element is not present, the value of max_dec_frame_buffering for the current operation point representation shall
be inferred to be equal to MaxDpbFrames.


**H.13.2.4Multiview scene information SEI message semantics**


The multiview scene information SEI message indicates the maximum disparity among multiple view components in an
access unit. The maximum disparity could be used for processing the decoded view components prior to rendering on a
3D display. When present, the multiview scene information SEI message shall be associated with an IDR access unit. The
information signalled in the SEI message applies to the coded video sequence.


The actual maximum disparity value may be less than the one signalled in the multiview scene information SEI message,
due to that some views in the coded video sequence may have been removed from the original bitstream to produce an
extracted sub-bitstream according to the process specified in clause H.8.5.3.


**max_disparity** specifies the maximum disparity, in units of luma samples, between spatially adjacent view components
among the total set of view components in an access unit. The value of max_disparity shall be in the range of 0 to 1023,
inclusive.


NOTE – The maximum disparity depends on the baseline distance between spatially adjacent views and the spatial resolution of
each view. Therefore, if either the number of views or spatial resolution is changed, the maximum disparity should also be changed
accordingly.


**H.13.2.5Multiview acquisition information SEI message semantics**


The multiview acquisition information SEI message specifies various parameters of the acquisition environment.
Specifically, intrinsic and extrinsic camera parameters are specified. These parameters could be used for processing the
decoded view components prior to rendering on a 3D display. When present as a non-nested SEI message, the multiview
acquisition information SEI message shall be associated with an IDR access unit. The information signalled in the
multiview acquisition information SEI message applies to the coded video sequence.


The multiview acquisition information SEI message may be nested in an MVCD scalable nesting SEI message to indicate
parameters of the acquisition environment of texture and depth views. When present as a nested SEI message, the multiview
acquisition information SEI message is recommended be associated with an IDR access unit and may be associated with
any access unit. When present as a nested SEI message, the information indicated in the SEI message applies from the
access unit associated with the SEI message to the next access unit, in decoding order, containing an SEI message of the
same type, exclusive, or to the end of the coded video sequence, whichever is earlier in decoding order.


Some of the views for which the multiview acquisition information is included in a multiview acquisition information SEI
message may not be present in the coded video sequence.


The extrinsic camera parameters are specified according to a right-handed coordinate system, where the upper left corner
of the image is the origin, i.e., the (0, 0) coordinate, with the other corners of the image having non-negative coordinates.





With these specifications, a 3-dimensional world point, wP=[x y z] is mapped to a 2-dimensional camera point, cP[ i ] =

[u v 1], for the i-th camera according to:


s * cP[ i ] = A[ i ] * R [−1] [ i ] * ( wP − T[ i ] ) (H-85)


where A[ i ] denotes the intrinsic camera parameter matrix, R [−1] [ i ] denotes the inverse of the rotation matrix R[ i ], T[ i ]
denotes the translation vector, and s (a scalar value) is an arbitrary scale factor chosen to make the third coordinate of
cP[ i ] equal to 1. The elements of A[ i ], R[ i ], T[ i ] are determined according to the syntax elements signalled in this SEI
message and as specified below.


**num_views_minus1** shall be equal to the value of the syntax element num_views_minus1 in the active MVC sequence
parameter set for the coded video sequence when the SEI message is not nested. When the SEI message is nested in an
MVCD scalable nesting SEI message, num_views_minus1 shall be equal to the value of num_view_components_minus1
of the containing MVCD scalable nesting SEI message. The value of num_views_minus1 shall be in the range of 0 to 1023,
inclusive.


When the SEI message is not nested, the loop index i in the subsequent syntax elements indicates the view order index
derived from the active MVC sequence parameter set. When the SEI message is nested in an MVCD scalable nesting SEI
message, the loop index i in the subsequent syntax elements indicates the view with view_id equal to sei_view_id[ i ] of
the containing MVCD scalable nesting SEI message.


**intrinsic_param_flag** equal to 1 indicates the presence of intrinsic camera parameters. intrinsic_param_flag equal to 0
indicates the absence of intrinsic camera parameters.


**extrinsic_param_flag** equal to 1 indicates the presence of extrinsic camera parameters. extrinsic_param_flag equal to 0
indicates the absence of extrinsic camera parameters.


**intrinsic_params_equal_flag** equal to 1 indicates that the intrinsic camera parameters are equal for all cameras and only
one set of intrinsic camera parameters is present. intrinsic_params_equal_flag equal to 0 indicates that the intrinsic camera
parameters are different for each camera and that a set of intrinsic camera parameters is present for each camera.


**prec_focal_length** specifies the exponent of the maximum allowable truncation error for focal_length_x[ i ] and
focal_length_y[ i ] as given by 2 [−prec_focal_length] . The value of prec_focal_length shall be in the range of 0 to 31, inclusive.


**prec_principal_point** specifies the exponent of the maximum allowable truncation error for principal_point_x[ i ] and
principal_point_y[ i ] as given by 2 [−prec_principal_point] . The value of prec_principal_point shall be in the range of 0 to 31,
inclusive.


**prec_skew_factor** specifies the exponent of the maximum allowable truncation error for skew factor as given by
2 [-prec_skew_factor] . The value of prec_skew_factor shall be in the range of 0 to 31, inclusive.


**sign_focal_length_x[** i **]** equal to 0 indicates that the sign of the focal length of the i-th camera in the horizontal direction
is positive. sign_focal_length_x[ i ] equal to 1 indicates that the sign is negative.


**exponent_focal_length_x[** i **]** specifies the exponent part of the focal length of the i-th camera in the horizontal direction.
The value of exponent_focal_length_x[ i ] shall be in the range of 0 to 62, inclusive. The value 63 is reserved for future
use by ITU-T | ISO/IEC. Decoders shall treat the value 63 as indicating an unspecified focal length.


**mantissa_focal_length_x[** i **]** specifies the mantissa part of the focal length of the i-th camera in the horizontal direction.
The length of the mantissa_focal_length_x[ i ] syntax element in units of bits is variable and determined as follows:


- If exponent_focal_length_x[ i ] = = 0, the length is Max( 0, prec_focal_length − 30 ).


- Otherwise (0 < exponent_focal_length_x[ i ] < 63), the length is Max( 0, exponent_focal_length_x[ i ] +
prec_focal_length − 31 ).


**sign_focal_length_y[** i **]** equal to 0 indicates that the sign of the focal length of the i-th camera in the vertical direction is
positive. sign_focal_length_y[ i ] equal to 1 indicates that the sign is negative.


**exponent_focal_length_y[** i **]** specifies the exponent part of the focal length of the i-th camera in the vertical direction.
The value of exponent_focal_length_y[ i ] shall be in the range of 0 to 62, inclusive. The value 63 is reserved for future
use by ITU-T | ISO/IEC. Decoders shall treat the value 63 as indicating an unspecified focal length.


**mantissa_focal_length_y[** i **]** specifies the mantissa part of the focal length of the i-th camera in the vertical direction. The
length of the mantissa_focal_length_y[ i ] syntax element in units of bits is variable and determined as follows:


- If exponent_focal_length_y[ i ] = = 0, the length is Max( 0, prec_focal_length − 30 ).


- Otherwise (0 < exponent_focal_length_y[ i ] < 63), the length is Max( 0, exponent_focal_length_y[ i ] +
prec_focal_length − 31 ).


**sign_principal_point_x[** i **]** equal to 0 indicates that the sign of the principal point of the i-th camera in the horizontal





direction is positive. sign_principal_point_x[ i ] equal to 1 indicates that the sign is negative.


**exponent_principal_point_x[** i **]** specifies the exponent part of the principal point of the i-th camera in the horizontal
direction. The value of exponent_principal_point_x[ i ] shall be in the range of 0 to 62, inclusive. The value 63 is reserved
for future use by ITU-T | ISO/IEC. Decoders shall treat the value 63 as indicating an unspecified principal point.


**mantissa_principal_point_x[** i **]** specifies the mantissa part of the principal point of the i-th camera in the horizontal
direction. The length of the mantissa_principal_point_x[ i ] syntax element in units of bits is variable and is determined as
follows:


- If exponent_principal_point_x[ i ] = = 0, the length is Max( 0, prec_principal_point − 30 ).


- Otherwise (0 < exponent_principal_point_x[ i ] < 63), the length is Max( 0, exponent_principal_point_x[ i ] +
prec_principal_point − 31 ).


**sign_principal_point_y[** i **]** equal to 0 indicates that the sign of the principal point of the i-th camera in the vertical
direction is positive. sign_principal_point_y[ i ] equal to 1 indicates that the sign is negative.


**exponent_principal_point_y[** i **]** specifies the exponent part of the principal point of the i-th camera in the vertical
direction. The value of exponent_principal_point_y[ i ] shall be in the range of 0 to 62, inclusive. The value 63 is reserved
for future use by ITU-T | ISO/IEC. Decoders shall treat the value 63 as indicating an unspecified principal point.


**mantissa_principal_point_y[** i **]** specifies the mantissa part of the principal point of the i-th camera in the vertical
direction. The length of the mantissa_principal_point_y[ i ] syntax element in units of bits is variable and is determined as
follows:


- If exponent_principal_point_y[ i ] = = 0, the length is Max( 0, prec_principal_point − 30 ).


- Otherwise (0 < exponent_principal_point_y[ i ] < 63), the length is Max( 0, exponent_principal_point_y[ i ] +
prec_principal_point − 31 ).


**sign_skew_factor[** i **]** equal to 0 indicates that the sign of the skew factor of the i-th camera is positive.
sign_skew_factor[ i ] equal to 1 indicates that the sign is negative.


**exponent_skew_factor[** i **]** specifies the exponent part of the skew factor of the i-th camera. The value of
exponent_skew_factor[ i ] shall be in the range of 0 to 62, inclusive. The value 63 is reserved for future use by
ITU-T | ISO/IEC. Decoders shall treat the value 63 as indicating an unspecified skew factor.


**mantissa_skew_factor[** i **]** specifies the mantissa part of the skew factor of the i-th camera. The length of the
mantissa_skew_factor[ i ] syntax element in units of bits is variable and determined as follows:


- If exponent_skew_factor[ i ] = = 0, the length is Max( 0, prec_skew_factor − 30 ).


- Otherwise (0 < exponent_skew_factor[ i ] < 63), the length is Max( 0, exponent_skew_factor[ i ] +
prec_skew_factor − 31 ).


The intrinsic matrix A[ i ] for i-th camera is represented by













focalLengthX[ i ] skewFactori[ ] principalPointX[ i ]



0 focalLengthY[i ] principalPointY[ i ]



(H-86)



0 0 1













**prec_rotation_param** specifies the exponent of the maximum allowable truncation error for r[ i ][ j ][ k ] as given by
2 [−prec_rotation_param] . The value of prec_rotation_param shall be in the range of 0 to 31, inclusive.


**prec_translation_param** specifies the exponent of the maximum allowable truncation error for t[ i ][ j ] as given by
2 [−prec_translation_param] . The value of prec_translation_param shall be in the range of 0 to 31, inclusive.


**sign_r[** i **][** j **][** k **]** equal to 0 indicates that the sign of (j, k) component of the rotation matrix for the i-th camera is positive.
sign_r[ i ][ j ][ k ] equal to 1 indicates that the sign is negative.


**exponent_r[** i **][** j **][** k **]** specifies the exponent part of (j, k) component of the rotation matrix for the i-th camera. The value
of exponent_r[ i ][ j ][ k ] shall be in the range of 0 to 62, inclusive. The value 63 is reserved for future use by
ITU-T | ISO/IEC. Decoders shall treat the value 63 as indicating an unspecified rotation matrix.


**mantissa_r[** i **][** j **][** k **]** specifies the mantissa part of (j, k) component of the rotation matrix for the i-th camera. The length
of the mantissa_r[ i ][ j ][ k ] syntax element in units of bits is variable and determined as follows:


- If exponent_r[ i ] = = 0, the length is Max( 0, prec_rotation_param − 30 ).


- Otherwise (0 < exponent_r[ i ] < 63), the length is Max( 0, exponent_r[ i ] + prec_rotation_param − 31 ).





The rotation matrix R[ i ] for i-th camera is represented as follows:























rE[ i ][ 0 ][ 0 ] rE[ i ][ 0 ][1] rE[ i ][ 0 ][ 2 ]

rE[ i ][ 1 ][] 0 rE[ i ][1][1] rE[ i ][1][ 2 ]

rE[ i ][ 2 ][] 0 rE[ i ][ 2 ][1] rE[ i ][ 2 ][ 2 ]



(H-87)



**sign_t[** i **][** j **]** equal to 0 indicates that the sign of the j-th component of the translation vector for the i-th camera is positive.
sign_t[ i ][ j ] equal to 1 indicates that the sign is negative.


**exponent_t[** i **][** j **]** specifies the exponent part of the j-th component of the translation vector for the i-th camera. The value
of exponent_t[ i ][ j ] shall be in the range of 0 to 62, inclusive. The value 63 is reserved for future use by ITU-T | ISO/IEC.
Decoders shall treat the value 63 as indicating an unspecified translation vector.


**mantissa_t[** i **][** j **]** specifies the mantissa part of the j-th component of the translation vector for the i-th camera. The length
v of the mantissa_t[ i ][ j ] syntax element in units of bits is variable and is determined as follows:


- If exponent_t[ i ] = = 0, the length v = Max( 0, prec_translation_param − 30 ).


- Otherwise (0 < exponent_t[ i ] < 63), the length v = Max( 0, exponent_t[ i ] + prec_translation_param − 31 ).


The translation vector T[ i ] for the i-th camera is represented by:























tE[ i ][] 0

tE[ i ][] 1

tE[ i ][] 2



(H-88)



The association between the camera parameter variables and corresponding syntax elements is specified by Table H-3.
Each component of the intrinsic and rotation matrices and the translation vector is obtained from the variables specified in
Table H-3 as the variable x computed as follows:


- If 0 < e < 63, x = ( −1 ) [s] - 2 [( e − 31 )] - ( 1 + n ÷ 2 [v ] ).


- Otherwise (e is equal to 0), x = ( −1 ) [s] - 2 [−( 30 + v )] - n.


NOTE – The above specification is similar to that found in IEC 60559:1989, _Binary floating-point arithmetic for microprocessor_
_systems_ .


**Table H-3 – Association between camera parameter variables and syntax elements.**

|x|s|e|n|
|---|---|---|---|
|**focalLengthX[** i**] **|sign_focal_length_x[ i ]|exponent_focal_length_x[ i ]|mantissa_focal_length_x[ i ]|
|**focalLengthY[** i**] **|sign_focal_length_y[ i ]|exponent_focal_length_y[ i ]|mantissa_focal_length_y[ i ]|
|**principalPointX[** i**] **|sign_principal_point_x[ i ]|exponent_principal_point_x[ i ]|mantissa_principal_point_x[ i ]|
|**principalPointY[** i**] **|sign_principal_point_y[ i ]|exponent_principal_point_y[ i ]|mantissa_principal_point_y[ i ]|
|**skewFactor[** i**] **|sign_skew_factor[ i ]|exponent_skew_factor[ i ]|mantissa_skew_factor[ i ]|
|**rE[** i**][**j **][**k **] **|sign_r[ i ][ j ][ k ]|exponent_r[ i ][ j ][ k ]|mantissa_r[ i ][ j ][ k ]|
|**tE[** i**][**j **] **|sign_t[ i ][ j ]|exponent_t[ i ][ j ]|mantissa_t[ i ][ j ]|



**H.13.2.6Non-required view component SEI message semantics**


This SEI message indicates non-required view components within the associated access unit. A view component is a nonrequired view component for a target view component if it is not needed for decoding the target view component and
subsequent view components with the same view_id in decoding order within the coded video sequence.


Some of the view components indicated by view_order_index[ i ] or index_delta_minus1[ i ][ j ] may not be present in the
associated access unit.


**num_info_entries_minus1** plus 1 specifies the number of target view components for which non-required view
components are indicated. The value of num_info_entries_minus1 shall be in the range of 0 to num_views_minus1 − 1,
inclusive.


**view_order_index[** i **]** specifies the view order index of the i-th target view component for which non-required view
components are indicated. The i-th target view component has view_id equal to view_id[ view_order_index[ i ] ]. The
value of view_order_index[ i ] shall be in the range of 1 to num_views_minus1, inclusive.


**num_non_required_view_components_minus1** [ i ] plus 1 specifies the number of non-required view components for





the i-th target view component. The value of num_non_required_view_components_minus1[ i ] shall be in the range of 0
to view_order_index[ i ] − 1, inclusive.


**index_delta_minus1[** i **][** j **]** plus 1 specifies the difference between the view order index of the i-th target view component
and the view order index of the j-th non-required view component for the i-th target view component. The view order index
of the j-th non-required view component for the i-th target view component is
view_order_index[ i ] − index_delta_minus1[ i ][ j ] − 1. The value of index_delta_minus1[ i ][ j ] shall be in the range
of 0 to view_order_index[ i ] − 1, inclusive.


**H.13.2.7View dependency change SEI message semantics**


This SEI message indicates that the view dependency information changes starting with the current access unit containing
the SEI message and is always interpreted with respect to the active MVC sequence parameter set. When present, the view
dependency change SEI message applies to the target access unit set that consists of the current access unit and all the
subsequent access units, in decoding order, until the next view dependency change SEI message or the end of the coded
video sequence, whichever is earlier in decoding order.


If, according to the view dependency information indicated in the active MVC sequence parameter set, view component
A does not directly or indirectly depend on view component B and vice versa, the view dependency change SEI message
shall not specify view dependency relationship between view components A and B.


NOTE 1 – The dependent views for any view are always a subset of those indicated by the active MVC sequence parameter set.
NOTE 2 – View dependency change SEI messages do not have a cumulative effect.


Some of the views indicated by the following syntax elements may not be present in the target access unit set.


**seq_parameter_set_id** specifies a subset sequence parameter set that contains the inter-view dependency relationship
information. The value of seq_parameter_set_id shall be equal to the value of seq_parameter_set_id in the picture
parameter set referenced by a view component of the primary coded picture of the access unit containing the view
dependency change SEI message. The value of seq_parameter_set_id shall be in the range of 0 to 31, inclusive.


**anchor_update_flag** equal to 1 indicates that there are updates for the dependencies for anchor view components relative
to the dependencies defined in the active MVC sequence parameter set. anchor_update_flag equal to 0 indicates that there
is no change for the dependencies for anchor view components relative to the dependencies defined in the active MVC
sequence parameter set.


**non_anchor_update_flag** equal to 1 indicates that there are updates for the dependencies for non-anchor view components
relative to the dependencies defined in the active MVC sequence parameter set. non_anchor_update_flag equal to 0
indicates that there is no change for the dependencies for non-anchor view components relative to the dependencies defined
in the active MVC sequence parameter set.


**anchor_ref_l0_flag[** i **][** j **]** equal to 0 indicates that the j-th inter-view prediction reference in the initial reference picture
list RefPicList0 (which is derived as specified in clause H.8.2.1) for any anchor view component with view order index
equal to i will not be present in the final RefPicList0 after reference picture list modification for the anchor view
component. anchor_ref_l0_flag[ i ][ j ] equal to 1 indicates that the j-th inter-view prediction reference in the initial
reference picture list RefPicList0 for at least one anchor view component with view order index equal to i will be present
in the final RefPicList0 after reference picture list modification for the anchor view component.


**anchor_ref_l1_flag[** i **][** j **]** equal to 0 indicates that the j-th inter-view prediction reference in the initial reference picture
list RefPicList1 (which is derived as specified in clause H.8.2.1) for any anchor view component with view order index
equal to i will not be present in the final RefPicList1 after reference picture list modification for the anchor view
component. anchor_ref_l1_flag[ i ][ j ] equal to 1 indicates that the j-th inter-view prediction reference in the initial
reference picture list RefPicList1 for at least one anchor view component with view order index equal to i will be present
in the final RefPicList1 after reference picture list modification for the anchor view component.


**non_anchor_ref_l0_flag[** i **][** j **]** equal to 0 indicates that the j-th inter-view prediction reference in the initial reference
picture list RefPicList0 (which is derived as specified in clause H.8.2.1) for any non-anchor view component with view
order index equal to i will not be present in the final RefPicList0 after reference picture list modification for the non-anchor
view component. non-anchor_ref_l0_flag[ i ][ j ] equal to 1 indicates that the j-th inter-view prediction reference in the
initial reference picture list RefPicList0 for at least one non-anchor view component with view order index equal to i will
be present in the final RefPicList0 after reference picture list modification for the non-anchor view component.


**non_anchor_ref_l1_flag[** i **][** j **]** equal to 0 indicates that the j-th inter-view prediction reference in the initial reference
picture list RefPicList1 (which is derived as specified in clause H.8.2.1) for any non-anchor view component with view
order index equal to i will not be present in the final RefPicList1 after reference picture list modification for the non-anchor
view component. non-anchor_ref_l1_flag[ i ][ j ] equal to 1 indicates that the j-th inter-view prediction reference in the
initial reference picture list RefPicList1 for at least one non-anchor view component with view order index equal to i will
be present in the final RefPicList1 after reference picture list modification for the non-anchor view component.





**H.13.2.8Operation point not present SEI message semantics**


This SEI message indicates operation points that are not present in the bitstream starting with the current access unit, and
is interpreted with respect to the previous view scalability information SEI message in decoding order. The message
remains effective until the next SEI message of the same type or the end of the coded video sequence, whichever is earlier
in decoding order.


NOTE 1– Operation point not present SEI messages do not have a cumulative effect.


NOTE 2 – Any operation point identified by a value of operation_point_id[ i ] in the previous view scalability information SEI
message, in decoding order, and not identified by a value of operation_point_not_present_id[ k ] must be present in the coded video
sequence. Therefore, when an application keeps an operation point not present SEI message in a sub-bitstream extracted according
to the process specified in clause H.8.5.3, the application may need to change the content of the operation point not present SEI
message according to the semantics.


**num_operation_points** specifies the number of operation points that are indicated not to be present by the SEI message.
num_operation_points equal to 0 indicates that all operation points indicated by the view scalability information SEI
message are present. The value of num_operation_points shall be in the range of 0 to the value of
num_operation_points_minus1 in the previous view scalability information SEI message in decoding order, inclusive.


**operation_point_not_present_id[** k **]** identifies an operation point that is not present. operation_point_not_present_id[ k ]
shall be equal to the value of one of the operation_point_id[ i ] syntax elements of the previous view scalability information
SEI message in decoding order. The value of operation_point_not_present_id[ k ] shall be in the range of 0 to 65535,
inclusive.


**H.13.2.9Base view temporal HRD SEI message semantics**


When present, this SEI message shall be associated with an IDR access unit. The SEI message applies to the coded video
sequence. Some temporal subsets identified by sei_mvc_temporal_id[ i ] may not be present in the coded video sequence.


**num_of_temporal_layers_in_base_view_minus1** plus 1 specifies the number of temporal bitstream subsets in the coded
video sequence for which the following syntax elements apply. The value of
num_of_temporal_layers_in_base_view_minus1 shall be in the range of 0 to 7, inclusive.


**sei_mvc_temporal_id[** i **]** specifies the temporal_id value of the i-th temporal bitstream subset.


Let the i-th bitstream subset for the coded video sequence that is obtained by invoking the sub-bitstream extraction process
as specified in clause H.8.5.3 with tIdTarget equal to sei_mvc_temporal_id[ i ] as input.


**sei_mvc_timing_info_present_flag[** i **]** equal to 1 specifies that sei_mvc_num_units_in_tick[ i ], sei_mvc_time_scale[ i ],
and sei_mvc_fixed_frame_rate_flag[ i ] are present in the base view temporal HRD SEI message.
sei_mvc_timing_info_present_flag[ i ] equal to 0 specifies that sei_mvc_num_units_in_tick[ i ], sei_mvc_time_scale[ i ],
and sei_mvc_fixed_frame_rate_flag[ i ] are not present in the base view temporal HRD SEI message.


The following syntax elements for the i-th bitstream subset are specified using references to Annex E. For these syntax
elements the same semantics and constraints as the ones specified in Annex E apply, as if these syntax elements
sei_mvc_num_units_in_tick[ i ], sei_mvc_time_scale[ i ], sei_mvc_fixed_frame_rate_flag[ i ],
sei_mvc_nal_hrd_parameters_present_flag[ i ], sei_mvc_vcl_hrd_parameters_present_flag[ i ],
sei_mvc_low_delay_hrd_flag[ i ], and sei_mvc_pic_struct_present_flag[ i ] were present as num_units_in_tick,
time_scale, fixed_frame_rate_flag, nal_hrd_parameters_present_flag, vcl_hrd_parameters_present_flag,
low_delay_hrd_flag, and pic_struct_present_flag, respectively, in the VUI parameters of the active MVC sequence
parameter sets for the i-th bitstream subset.


The parameters for the i-th bitstream subset that are coded in the base view temporal HRD SEI message shall be correct,
as if these parameters are used for conformance checking (as specified in Annex C) of the i-th bitstream subset.


**sei_mvc_num_units_in_tick[** i **]** indicates the value of num_units_in_tick, as specified in clause E.2.1, that applies to the
i-th bitstream subset.


**sei_mvc_time_scale[** i **]** indicates the value of time_scale, as specified in clause E.2.1, that applies to the i-th bitstream
subset.


**sei_mvc_fixed_frame_rate_flag[** i **]** indicates the value of fixed_frame_rate_flag, as specified in clause E.2.1, that applies
to the i-th bitstream subset.


**sei_mvc_nal_hrd_parameters_present_flag[** i **]** indicates the value of nal_hrd_parameters_present_flag, as specified in
clause E.2.1, that applies to the i-th bitstream subset. When sei_mvc_nal_hrd_parameters_present_flag[ i ] is equal to 1,
the NAL HRD parameters that apply to the i-th bitstream subset immediately follow the
sei_mvc_nal_hrd_parameters_present_flag[ i ].


**sei_mvc_vcl_hrd_parameters_present_flag[** i **]** indicates the value of vcl_hrd_parameters_present_flag, as specified in
clause E.2.1, that applies to the i-th bitstream subset. When sei_mvc_vcl_hrd_parameters_present_flag[ i ] is equal to 1,





the VCL HRD parameters that apply to the i-th bitstream subset immediately follow the
sei_mvc_vcl_hrd_parameters_present_flag[ i ].


**sei_mvc_low_delay_hrd_flag[** i **]** indicates the value of low_delay_hrd_flag, as specified in clause E.2.1, that applies to
the i-th bitstream subset.


**sei_mvc_pic_struct_present_flag[** i **]** indicates the value of pic_struct_present_flag, as specified in clause E.2.1, that
applies to the i-th bitstream subset.


**H.13.2.10** **Multiview view position SEI message semantics**


The multiview view position SEI message specifies the relative view position along a single horizontal axis of view
components within a coded video sequence. When present, the multiview view position SEI message shall be associated
with an IDR access unit. The information signalled in this SEI message applies to the entire coded video sequence.


**num_views_minus1** shall be equal to the value of the syntax element num_views_minus1 in the active MVC sequence
parameter set for the coded video sequence. The value of num_views_minus1 shall be in the range of 0 to 1023, inclusive.


**view_position[** i **]** indicates the order of the view with VOIdx equal to i among all the views from left to right for the
purpose of display, with the order for the left-most view being equal to 0 and the value of the order increasing by 1 for
next view from left to right. The value of view_position[ i ] shall be in the range of 0 to 1023, inclusive.


**multiview_view_position_extension_flag** equal to 0 indicates that no additional data follows within the multiview view
position SEI message. The value of multiview_view_position_extension_flag shall be equal to 0. The value of 1 for
multiview_view_position_extension_flag is reserved for future use by ITU-T | ISO/IEC. Decoders shall ignore all data that
follows the value of 1 for multiview_view_position_extension_flag in a multiview view position SEI message.
