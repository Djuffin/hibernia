**I.13** **MVCD SEI messages**


The specifications in Annex D together with the extensions and modifications specified in this clause apply.


**I.13.1** **SEI message syntax**


**I.13.1.1** **MVCD view scalability information SEI message syntax**

|mvcd_view_scalability_info( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**num_operation_points_minus1**|5|ue(v)|
|<br>for( i = 0; i <= num_operation_points_minus1; i++ ) {|||
|<br> <br>**operation_point_id[** i**] **|5|ue(v)|
|<br> <br>**priority_id[**i** ]**|5|u(5)|






|temporal_id[ i ]|5|u(3)|
|---|---|---|
|<br> <br>**num_target_output_views_minus1[** i**] **|5|ue(v)|
|<br> <br>for( j = 0; j <= num_target_output_views_minus1[ i ]; j++ ) {|||
|<br> <br> <br>**view_id[** i**][** j**] **|5|ue(v)|
|<br> <br> <br>mvcd_op_view_info( )|||
|<br> <br>}|||
|<br> <br>**profile_level_info_present_flag[** i**]**|5|u(1)|
|<br> <br>**bitrate_info_present_flag[** i**]**|5|u(1)|
|<br> <br>**frm_rate_info_present_flag[** i**]**|5|u(1)|
|<br> <br>if( !num_target_output_views_minus1[ i ] )|||
|<br> <br> <br>**view_dependency_info_present_flag[**i** ]**|5|u(1)|
|<br> <br>**parameter_sets_info_present_flag[** i**]**|5|u(1)|
|<br> <br>**bitstream_restriction_info_present_flag[** i**]**|5|u(1)|
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
|<br> <br> <br>for( j = 0; j < num_directly_dependent_views[ i ]; j++ ) {|||
|<br> <br> <br> <br>**directly_dependent_view_id[**i** ][**j** ]**|5|ue(v)|
|<br> <br> <br> <br>mvcd_op_view_info( )|||
|<br> <br> <br>}|||
|<br> <br>} else|||
|<br> <br> <br>**view_dependency_info_src_op_id[** i**]**|5|ue(v)|
|<br> <br>if( parameter_sets_info_present_flag[ i ] ) {|||
|<br> <br> <br>**num_seq_parameter_set_minus1[** i**] **|5|ue(v)|
|<br> <br> <br>for( j = 0; j <= num_seq_parameter_set_minus1[ i ]; j++ )|||
|<br> <br> <br> <br>**seq_parameter_set_id_delta[** i**][** j**]**|5|ue(v)|
|<br> <br> <br>**num_subset_seq_parameter_set_minus1[** i**] **|5|ue(v)|
|<br> <br> <br>for( j = 0; j <= num_subset_seq_parameter_set_minus1[ i ]; j++ )|||
|<br> <br> <br> <br>**subset_seq_parameter_set_id_delta[** i**][** j**]**|5|ue(v)|
|<br> <br> <br>**num_pic_parameter_set_minus1[** i**] **|5|ue(v)|
|<br> <br> <br>for( j = 0; j <= num_init_pic_parameter_set_minus1[ i ]; j++ )|||
|<br> <br> <br> <br>**pic_parameter_set_id_delta[** i**][** j**]**|5|ue(v)|
|<br> <br>} else|||
|<br> <br> <br>**parameter_sets_info_src_op_id[** i**]**|5|ue(v)|





|if( bitstream_restriction_info_present_flag[ i ] ) {|Col2|Col3|
|---|---|---|
|<br> <br> <br>**motion_vectors_over_pic_boundaries_flag[** i**]**|5|u(1)|
|<br> <br> <br>**max_bytes_per_pic_denom[** i**]**|5|ue(v)|
|<br> <br> <br>**max_bits_per_mb_denom[** i**]**|5|ue(v)|
|<br> <br> <br>**log2_max_mv_length_horizontal[** i**]**|5|ue(v)|
|<br> <br> <br>**log2_max_mv_length_vertical[** i**]**|5|ue(v)|
|<br> <br> <br>**num_reorder_frames[** i**]**|5|ue(v)|
|<br> <br> <br>**max_dec_frame_buffering[** i**]**|5|ue(v)|
|<br> <br>}|||
|<br>}|||
|}|||


**I.13.1.1.1** **MVCD operation point view information syntax**

|mvcd_op_view_info( ) {|C|Descriptor|
|---|---|---|
|<br>**view_info_depth_view_present_flag**|5|u(1)|
|<br>if( view_info_depth_view_present_flag )|||
|<br> <br>**mvcd_depth_view_flag**|5|u(1)|
|<br>**view_info_texture_view_present_flag**|5|u(1)|
|<br>if( view_info_texture_view_present_flag )|||
|<br> <br>**mvcd_texture_view_flag**|5|u(1)|
|}|||



**I.13.1.2** **MVCD scalable nesting SEI message syntax**

|mvcd_scalable_nesting( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**operation_point_flag**|5|u(1)|
|<br>if( !operation_point_flag ) {|||
|<br> <br>**all_view_components_in_au_flag**|5|u(1)|
|<br> <br>if( !all_view_components_in_au_flag ) {|||
|<br> <br> <br>**num_view_components_minus1**|5|ue(v)|
|<br> <br> <br>for( i = 0; i <= num_view_components_minus1; i++ ) {|||
|<br> <br> <br> <br>**sei_view_id[** i**] **|5|u(10)|
|<br> <br> <br> <br>**sei_view_applicability_flag[** i**] **|5|u(1)|
|<br> <br> <br>}|||
|<br> <br>}|||
|<br>} else {|||
|<br> <br>**sei_op_texture_only_flag**|5|u(1)|
|<br> <br>**num_view_components_op_minus1**|5|ue(v)|
|<br> <br>for( i = 0; i <= num_view_components_op_minus1; i++ ) {|||
|<br> <br> <br>**sei_op_view_id[** i**] **|5|u(10)|
|<br> <br> <br>if( !sei_op_texture_only_flag ) {|||
|<br> <br> <br> <br>**sei_op_depth_flag[** i**]**|||
|<br> <br> <br> <br>**sei_op_texture_flag[** i**]**|||
|<br> <br> <br>}|||
|<br> <br>}|||
|<br> <br>**sei_op_temporal_id**|5|u(3)|
|<br>}|||
|<br>while( !byte_aligned( ) )|||
|<br> <br>**sei_nesting_zero_bit**/* equal to 0 */|5|f(1)|
|<br>sei_message( )|5||
|}|||






**I.13.1.3** **Depth representation information SEI message syntax**

|depth_representation_info( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**all_views_equal_flag**|5|u(1)|
|<br>if( all_views_equal_flag = = 0 ) {|||
|<br> <br>**num_views_minus1**|5|ue(v)|
|<br> <br>numViews = num_views_minus1 + 1|||
|<br>} else|||
|<br> <br>numViews = 1|||
|<br>**z_near_flag**|5|u(1)|
|<br>**z_far_flag**|5|u(1)|
|<br>if( z_near_flag | | z_far_flag ) {|||
|<br> <br>**z_axis_equal_flag**|5|u(1)|
|<br> <br>if( z_axis_equal_flag )|||
|<br> <br> <br>**common_z_axis_reference_view**|5|ue(v)|
|<br>}|||
|<br>**d_min_flag**|5|u(1)|
|<br>**d_max_flag**|5|u(1)|
|<br>**depth_representation_type**|5|ue(v)|
|<br>for( i = 0; i < numViews; i++ ) {|||
|<br> <br>**depth_info_view_id[** i**]**|5|ue(v)|
|<br> <br>if( ( z_near_flag | | z_far_flag ) && ( z_axis_equal_flag = = 0 ) )|||
|<br> <br> <br>**z_axis_reference_view[** i**]**|5|ue(v)|
|<br> <br>if( d_min_flag | | d_max_flag )|||
|<br> <br> <br>**disparity_reference_view[** i**]**|5|ue(v)|
|<br> <br>if( z_near_flag )|||
|<br> <br> <br>depth_representation_sei_element( ZNearSign, ZNearExp,<br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br>ZNearMantissa, ZNearManLen )|||
|<br> <br>if( z_far_flag )|||
|<br> <br> <br>depth_representation_sei_element( ZFarSign, ZFarExp,<br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br>ZFarMantissa, ZFarManLen )|||
|<br> <br>if( d_min_flag )|||
|<br> <br> <br>depth_representation_sei_element( DMinSign, DMinExp,<br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br>DMinMantissa, DMinManLen )|||
|<br> <br>if( d_max_flag )|||
|<br> <br> <br>depth_representation_sei_element( DMaxSign, DMaxExp,<br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br>DMaxMantissa, DMaxManLen )|||
|<br>}|||
|<br>if( depth_representation_type = = 3 ) {|||
|<br> <br>**depth_nonlinear_representation_num_minus1**|5|ue(v)|
|<br> <br>for( i = 1; i <= depth_nonlinear_representation_num_minus1 + 1; i++ )|||
|<br> <br> <br>**depth_nonlinear_representation_model[** i**]**|5|ue(v)|
|<br>}|||
|}|||






**I.13.1.3.1** **Depth representation SEI element syntax**





|depth_representation_sei_element( outSign, outExp, outMantissa,<br>outManLen ) {|C|Descriptor|
|---|---|---|
|<br>**da_sign_flag**|5|u(1)|
|<br>**da_exponent**|5|u(7)|
|<br>**da_mantissa_len_minus1**|5|u(5)|
|<br>**da_mantissa**|5|u(v)|
|}|||


**I.13.1.4** **3D reference displays information SEI message syntax**

|three_dimensional_reference_displays_info( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**prec_ref_baseline**|5|ue(v)|
|<br>**prec_ref_display_width**|5|ue(v)|
|<br>**ref_viewing_distance_flag**|5|u(1)|
|<br>if( ref_viewing_distance_flag )|||
|<br> <br>**prec_ref_viewing_dist**|5|ue(v)|
|<br>**num_ref_displays_minus1**|5|ue(v)|
|<br>numRefDisplays = num_ref_displays_minus1 + 1|||
|<br>for( i = 0; i < numRefDisplays; i++ ) {|||
|<br> <br>**exponent_ref_baseline[**i** ]**|5|u(6)|
|<br> <br>**mantissa_ref_baseline[**i** ]**|5|u(v)|
|<br> <br>**exponent_ref_display_width[**i** ]**|5|u(6)|
|<br> <br>**mantissa_ref_display_width[**i** ]**|5|u(v)|
|<br> <br>if( ref_viewing_distance_flag ) {|||
|<br> <br> <br>**exponent_ref_viewing_distance[**i** ]**|5|u(6)|
|<br> <br> <br>**mantissa_ref_viewing_distance[**i** ]**|5|u(v)|
|<br> <br>}|||
|<br> <br>**additional_shift_present_flag[** i**] **|5|u(1)|
|<br> <br>if( additional_shift_present[ i ] )|||
|<br> <br> <br>**num_sample_shift_plus512[** i**] **|5|u(10)|
|<br>}|||
|<br>**three_dimensional_reference_displays_extension_flag**|5|u(1)|
|}|||



**I.13.1.5** **Depth timing SEI message syntax**

|depth_timing( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**per_view_depth_timing_flag**|5|u(1)|
|<br>if( per_view_depth_timing_flag )|||
|<br> <br>for( i = 0; i < NumDepthViews; i++ )|||
|<br> <br> <br>depth_timing_offset( )|||
|<br>else|||
|<br> <br>depth_timing_offset( )|||
|}|||






**I.13.1.5.1** **Depth timing offset syntax**

|depth_timing_offset( ) {|C|Descriptor|
|---|---|---|
|<br>**offset_len_minus1**|5|u(5)|
|<br>**depth_disp_delay_offset_fp**|5|u(v)|
|<br>**depth_disp_delay_offset_dp**|5|u(6)|
|}|||



**I.13.1.6** **Alternative depth information SEI message syntax**

|alternative_depth_info( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**depth_type**|5|ue(v)|
|<br>if( depth_type = = 0 ) {|||
|<br> <br>**num_constituent_views_gvd_minus1**|5|ue(v)|
|<br> <br>**depth_present_gvd_flag**|5|u(1)|
|<br> <br>**z_gvd_flag**|5|u(1)|
|<br> <br>**intrinsic_param_gvd_flag**|5|u(1)|
|<br> <br>**rotation_gvd_flag**|5|u(1)|
|<br> <br>**translation_gvd_flag**|5|u(1)|
|<br> <br>if( z_gvd_flag )|||
|<br> <br> <br>for( i = 0; i <= num_constituent_views_gvd_minus1 + 1; i++ ) {|||
|<br> <br> <br> **sign_gvd_z_near_flag[**i** ]**|5|u(1)|
|<br> <br> <br>** exp_gvd_z_near[**i** ]**|5|u(7)|
|<br> <br> <br>** man_len_gvd_z_near_minus1[**i** ]**|5|u(5)|
|<br> <br> <br>** man_gvd_z_near[**i** ]**|5|u(v)|
|<br> <br> <br>** sign_gvd_z_far_flag[**i** ]**|5|u(1)|
|<br> <br> <br>** exp_gvd_z_far[**i** ]**|5|u(7)|
|<br> <br> <br>** man_len_gvd_z_far_minus1[**i** ]**|5|u(5)|
|<br> <br> <br>** man_gvd_z_far[**i** ]**|5|u(v)|
|<br> <br> <br>}|||
|<br> <br>if( intrinsic_param_gvd_flag ) {|||
|<br> <br> <br>**prec_gvd_focal_length**|5|ue(v)|
|<br> <br> <br>**prec_gvd_principal_point**|5|ue(v)|
|<br> <br>}|||
|<br> <br>if( rotation_gvd_flag )|||
|<br> <br> <br>**prec_gvd_rotation_param**|5|ue(v)|
|<br> <br>if( translation_gvd_flag )|||
|<br> <br> <br>**prec_gvd_translation_param**|5|ue(v)|
|<br> <br>for( i = 0; i <= num_constituent_views_gvd_minus1 + 1; i++ ) {|||
|<br> <br> <br>if( intrinsic_param_gvd_flag ) {|||
|<br> <br> <br>** sign_gvd_focal_length_x[** i**]**|5|u(1)|
|<br> <br> <br>** exp_gvd_focal_length_x[** i**]**|5|u(6)|
|<br> <br> <br>** man_gvd_focal_length_x[** i**]**|5|u(v)|
|<br> <br> <br>** sign_gvd_focal_length_y[** i**]**|5|u(1)|
|<br> <br> <br>** exp_gvd_focal_length_y[** i**]**|5|u(6)|
|<br> <br> <br>** man_gvd_focal_length_y[** i**]**|5|u(v)|
|<br> <br> <br>** sign_gvd_principal_point_x[** i**]**|5|u(1)|
|<br> <br> <br>** exp_gvd_principal_point_x[** i**]**|5|u(6)|






|man_gvd_principal_point_x[ i ]|5|u(v)|
|---|---|---|
|<br> <br> <br>** sign_gvd_principal_point_y[** i**]**|5|u(1)|
|<br> <br> <br>** exp_gvd_principal_point_y[** i**]**|5|u(6)|
|<br> <br> <br>** man_gvd_principal_point_y[** i**]**|5|u(v)|
|<br> <br> <br>}|||
|<br> <br> <br>if( rotation_gvd_flag )|||
|<br> <br> <br> for( j = 0; j < 3; j++ ) /* row */|||
|<br> <br> <br> for( k = 0; k < 3; k++ ) { /* column */|||
|<br> <br> <br> <br>**sign_gvd_r[** i**][** j**][** k**]**|5|u(1)|
|<br> <br> <br> <br>**exp_gvd_r[** i**][** j**][** k**] **|5|u(6)|
|<br> <br> <br> <br>**man_gvd_r[** i**][** j**][** k**] **|5|u(v)|
|<br> <br> <br> }|||
|<br> <br> <br>if( translation_gvd_flag ) {|||
|<br> <br> <br> **sign_gvd_t_x[** i**]**|5|u(1)|
|<br> <br> <br> **exp_gvd_t_x[** i**]**|5|u(6)|
|<br> <br> <br> **man_gvd_t_x[** i**]**|5|u(v)|
|<br> <br> <br>}|||
|<br> <br>}|||
|<br>}|||
|}|||


**I.13.1.7** **Depth sampling information SEI message syntax**

|depth_sampling_info( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**dttsr_x_mul**|5|u(16)|
|<br>**dttsr_x_dp**|5|u(4)|
|<br>**dttsr_y_mul**|5|u(16)|
|<br>**dttsr_y_dp**|5|u(4)|
|<br>**per_view_depth_grid_pos_flag**|5|u(1)|
|<br>if( per_view_depth_grid_pos_flag ) {|||
|<br> <br>**num_video_plus_depth_views_minus1**|5|ue(v)|
|<br> <br>for( i = 0; i <= num_video_plus_depth_views_minus1; i++ ) {|||
|<br> <br> <br>**depth_grid_view_id[** i**]**|5|ue(v)|
|<br> <br> <br>depth_grid_position( )|||
|<br> <br>}|||
|<br>} else|||
|<br> <br>depth_grid_position( )|||
|}|||






**I.13.1.7.1** **Depth grid position syntax**

|depth_grid_position( ) {|C|Descriptor|
|---|---|---|
|<br>**depth_grid_pos_x_fp**|5|u(20)|
|<br>**depth_grid_pos_x_dp**|5|u(4)|
|<br>**depth_grid_pos_x_sign_flag**|5|u(1)|
|<br>**depth_grid_pos_y_fp**|5|u(20)|
|<br>**depth_grid_pos_y_dp**|5|u(4)|
|<br>**depth_grid_pos_y_sign_flag**|5|u(1)|
|}|||



**I.13.2** **SEI message semantics**


Depending on payloadType, the corresponding SEI message semantics are extended as follows:


- If payloadType is equal to 2, 3, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 45, 47, 137, 142, 144,
147, 148, 149, 150, 151, 154, 155, 156, 200, 201, 202, or 205, the following applies:


   - If the SEI message is not included in an MVC scalable nesting SEI message or an MVCD scalable nesting SEI
message, it applies to the texture view component of the current access unit with VOIdx equal to VOIdxMin.


   - Otherwise, if included in an MVC scalable nesting SEI message and not included in an MVCD scalable nesting
SEI message, it applies to all texture view components of the current access unit when
all_view_components_in_au_flag is equal to 1, or it applies to all texture view components of the current access
unit with view_id equal to sei_view_id[ i ] for any i in the range of 0 to num_view_components_minus1,
inclusive, when all_view_components_in_au_flag is equal to 0. When payloadType is equal to 10 for the SEI
message that is included in an MVC scalable nesting SEI message, the semantics for sub_seq_layer_num of the
sub-sequence information SEI message is modified as follows:


**sub_seq_layer_num** specifies the sub-sequence layer number of the current picture. When the current
picture resides in a sub-sequence for which the first picture in decoding order is an IDR picture, the value
of sub_seq_layer_num shall be equal to 0. For a non-paired reference field, the value of sub_seq_layer_num
shall be equal to 0. sub_seq_layer_num shall be in the range of 0 to 255, inclusive.


   - Otherwise, if not included in an MVC scalable nesting SEI message and included in an MVCD scalable nesting
SEI message, it applies to all depth view components or view component pairs of the current access unit when
all_view_components_in_au_flag is equal to 1, or it applies to all depth view components or view component
pairs of the current access unit with view_id equal to sei_view_id[ i ] for any i in the range of 0 to
num_view_components_minus1, inclusive, when all_view_components_in_au_flag is equal to 0. When
payloadType is equal to 10 for the SEI message that is included in an MVCD scalable nesting SEI message, the
semantics for sub_seq_layer_num of the sub-sequence information SEI message is modified as follows:


**sub_seq_layer_num** specifies the sub-sequence layer number of the current picture. When the current
picture resides in a sub-sequence for which the first picture in decoding order is an IDR picture, the value
of sub_seq_layer_num shall be equal to 0. For a non-paired reference field, the value of sub_seq_layer_num
shall be equal to 0. sub_seq_layer_num shall be in the range of 0 to 255, inclusive.


- Otherwise, if payloadType is equal to 41, 42 or 43, the following applies:


   - If the SEI message is not included in MVCD scalable nesting SEI message, it applies to texture views only and
NAL units having nal_unit_type equal to 21 are non-VCL NAL units.


   - Otherwise (the SEI message is included in MVCD scalable nesting SEI message), the SEI message applies to
depth views, to texture views or both texture views all depth views, depending on the values of the syntax
elements of the MVCD scalable nesting SEI message.


- Otherwise, if payloadType is equal to 0 or 1, the following applies:


   - If the SEI message is not included in an MVC scalable nesting SEI message or an MVCD scalable nesting SEI
message or an MVCD texture sub-bitstream HRD nesting SEI message, the following applies. When the SEI
message and all other SEI messages with payloadType equal to 0 or 1 not included in an MVC scalable nesting
SEI message or an MVCD scalable nesting SEI message or an MVCD texture sub-bitstream HRD nesting SEI
message are used as the buffering period and picture timing SEI messages for checking the bitstream





conformance according to Annex C and the decoding process specified in clauses 2 to 9 is used, the bitstream
shall be conforming to this Recommendation | International Standard.


- Otherwise, if the SEI message is included in an MVC scalable nesting SEI message and not included in an
MVCD scalable nesting SEI message or a MVCD texture sub-bitstream HRD nesting SEI message, the following
applies. When the SEI message and all other SEI messages with payloadType equal to 0 or 1 included in an
MVC scalable nesting SEI message with identical values of sei_op_temporal_id and sei_op_view_id[ i ] for all
i in the range of 0 to num_view_components_op_minus1, inclusive, are used as the buffering period and picture
timing SEI messages for checking the bitstream conformance according to Annex C, the bitstream that would be
obtained by invoking the bitstream extraction process as specified in clause H.8.3 with tIdTarget equal to
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


- Otherwise, if the SEI message is included in a MVCD scalable nesting SEI message and not included in an MVC
scalable nesting SEI message or an MVCD texture sub-bitstream HRD nesting SEI message, the following
applies. When the SEI message and all other SEI messages with payloadType equal to 0 or 1 included in an
MVCD scalable nesting SEI message with identical values of sei_op_temporal_id and sei_op_view_id[ i ] for
all i in the range of 0 to num_view_components_op_minus1, inclusive, are used as the buffering period and
picture timing SEI messages for checking the bitstream conformance according to Annex C, the bitstream that
would be obtained by invoking the bitstream extraction process as specified in clause I.8.5 with
depthPresentTargetFlag equal to 1, tIdTarget equal to sei_op_temporal_id and viewIdTargetList equal to
sei_op_view_id[ i ] for all i in the range of 0 to num_view_components_op_minus1, inclusive, shall be
conforming to this Recommendation | International Standard.


In the semantics of clauses D.2.1 and D.2.3, the syntax elements num_units_in_tick, time_scale,
fixed_frame_rate_flag, nal_hrd_parameters_present_flag, vcl_hrd_parameters_present_flag,
low_delay_hrd_flag, and pic_struct_present_flag and the derived variables NalHrdBpPresentFlag,
VclHrdBpPresentFlag, and CpbDpbDelaysPresentFlag are substituted with the syntax elements
vui_mvc_num_units_in_tick[ i ], vui_mvc_time_scale[ i ], vui_mvc_fixed_frame_rate_flag[ i ],
vui_mvc_nal_hrd_parameters_present_flag[ i ], vui_mvc_vcl_hrd_parameters_present_flag[ i ],
vui_mvc_low_delay_hrd_flag[ i ], and vui_mvc_pic_struct_present_flag[ i ] and the derived variables
VuiMvcNalHrdBpPresentFlag[ i ], VuiMvcVclHrdBpPresentFlag[ i ], and
VuiMvcCpbDpbDelaysPresentFlag[ i ] for the MVCD VUI parameters extension.


The values of seq_parameter_set_id's in all buffering period SEI messages included in MVCD scalable nesting
SEI messages and not included in either MVC scalable nesting SEI messages or MVCD texture sub-bitstream
HRD nesting SEI messages and associated with operation points for which the greatest VOIdx values in the
associated bitstream subsets are identical shall be identical.


- Otherwise, if the SEI message is included in an MVCD texture sub-bitstream HRD nesting SEI message, the
following applies. When the SEI message and all other SEI messages included in an MVCD texture sub-bitstream
HRD nesting SEI message with identical values of texture_subbitstream_temporal_id and
texture_subbitstream_view_id[ i ] for all i in the range of 0 to
num_texture_subbitstream_view_components_minus1, inclusive, are used as the buffering period and picture
timing SEI messages for checking the bitstream conformance according to Annex C, the bitstream that would be
obtained by invoking the bitstream extraction process as specified in clause I.8.5 with depthPresentTargetFlag
equal to 0, tIdTarget equal to texture_subbitstream_temporal_id and viewIdTargetList equal to
texture_subbitstream_view_id[ i ] for all i in the range of 0 to
num_texture_subbitstream_view_components_minus1, inclusive, shall be conforming to this Recommendation |
International Standard.





In the semantics of clauses D.2.1 and D.2.3, the syntax elements num_units_in_tick, time_scale,
fixed_frame_rate_flag, nal_hrd_parameters_present_flag, vcl_hrd_parameters_present_flag,
low_delay_hrd_flag, and pic_struct_present_flag and the derived variables NalHrdBpPresentFlag,
VclHrdBpPresentFlag, and CpbDpbDelaysPresentFlag are substituted with the syntax elements
vui_mvc_num_units_in_tick[ i ], vui_mvc_time_scale[ i ], vui_mvc_fixed_frame_rate_flag[ i ],
vui_mvc_nal_hrd_parameters_present_flag[ i ], vui_mvc_vcl_hrd_parameters_present_flag[ i ],
vui_mvc_low_delay_hrd_flag[ i ], and vui_mvc_pic_struct_present_flag[ i ] and the derived variables
VuiMvcNalHrdBpPresentFlag[ i ], VuiMvcVclHrdBpPresentFlag[ i ], and
VuiMvcCpbDpbDelaysPresentFlag[ i ] for the MVCD texture sub-bitstream VUI parameters extension.


The values of seq_parameter_set_id's in all buffering period SEI messages included in MVCD texture subbitstream HRD nesting SEI messages and associated with operation points for which the greatest VOIdx values
in the associated bitstream subsets are identical shall be identical.


- Otherwise (all remaining payloadType values), the corresponding SEI message semantics are not extended.


When an SEI message having a particular value of payloadType equal to 137 or 144, contained in an MVCD scalable
nesting SEI message, and applying to a particular combination of a list of view_id and a temporal_id is present in an access
unit, the SEI message with the particular value of payloadType applying to the particular combination of a list of view_id
and a temporal_id shall be present an MVCD scalable nesting SEI message in the IDR access unit that is the first access
unit of the coded video sequence.


All SEI messages having a particular value of payloadType equal to 137 or 144, contained in MVCD scalable nesting SEI
messages, and applying to a particular combination of a list of view_id and a temporal_id present in a coded video sequence
shall have the same content.


For the semantics of SEI messages with payloadType in the range of 0 to 23, inclusive, or equal to 45, 47, 137, 142, 144,
147, 148, 149, 150, 151, 154, 155, 156, 200, 201, 202, or 205, which are specified in clause D.2, MVCD sequence
parameter set is substituted for sequence parameter set; the parameters of MVCD sequence parameter set RBSP and picture
parameter set RBSP that are in effect are specified in clauses I.7.4.2.1 and I.7.4.2.2, respectively.


Coded video sequences conforming to one or more of the profiles specified in Annex I shall not include SEI NAL units
that contain SEI messages with payloadType in the range of 24 to 35, inclusive.


When an SEI NAL unit contains an SEI message with payloadType in the range of 36 to 44, inclusive, or equal to 46, or
in the range of 48 to 53, inclusive, it shall not contain any SEI messages with payloadType less than 36 or equal to 45, 47,
137, 142, 144, 147, 148, 149, 150, 151, 154, 155, 156, 200, 201, 202, or 205, and the first SEI message in the SEI NAL
unit shall have payloadType in the range of 36 to 44, inclusive, or equal to 46, or in the range of 48 to 53, inclusive.


When an MVC scalable nesting SEI message (payloadType equal to 37), a view scalability information SEI message
(payloadType equal to 38), an operation point not present SEI message (payloadType equal to 43), an MVCD scalable
nesting SEI message (payloadType equal to 48), or an MVCD view scalability information SEI messages (payloadType
equal to 49) is present in an SEI NAL unit, it shall be the only SEI message in the SEI NAL unit.


**I.13.2.1** **MVCD view scalability information SEI message semantics**


The syntax elements in the MVCD view scalability information SEI message that have the same names as those in the
view scalability information SEI message specified in Annex H, except num_directly_dependent_depth_views[ i ] and
directly_dependent_depth_view_id[ i ][ j ], have the same semantics as the corresponding syntax elements in the view
scalability information SEI message, but apply to operation points that may potentially contain depth view components as
well as texture view components.


**num_directly_dependent_views[** i **]** and **directly_dependent_view_id[** i **][** j **]** apply only to the texture view components
of an operation point if the operation point contains both texture and depth, and otherwise have the same semantics as the
corresponding syntax elements in the view scalability information SEI message.


**I.13.2.1.1** **MVCD operation point view information semantics**


**view_info_depth_view_present_flag** equal to 0 specifies that the depth view is not included in the operation point for the
view for which the mvcd_op_view_info( ) syntax structure is present. view_info_depth_view_present_flag equal to 1
specifies that the depth view is included in the operation point for the view for which the mvcd_op_view_info( ) syntax
structure is present.


**mvcd_depth_view_flag** equal to 0 indicates that some VCL NAL units for the depth view for the view for which the
mvcd_op_view_info( ) syntax structure is present may include NAL units with nal_unit_type equal to 21 and
avc_3d_extension_flag equal to 1. mvcd_depth_view_flag equal to 1 indicates that the VCL NAL units for the depth view
for the view for which the mvcd_op_view_info( ) syntax structure is present does not include NAL units with both
nal_unit_type equal to 21 and avc_3d_extension_flag equal to 1.





**view_info_texture_view_present_flag** equal to 0 specifies that the texture view is not included in the operation point for
the view for which the mvcd_op_view_info( ) syntax structure is present. view_info_depth_view_present_flag equal to 1
specifies that the texture view is included in the operation point for the view for which the mvcd_op_view_info( ) syntax
structure is present. When view_info_depth_view_present_flag is equal to 0, view_info_texture_view_present_flag shall
be equal to 1.


**mvcd_texture_view_flag** equal to 0 indicates that some VCL NAL units for the texture view for the view for which the
mvcd_op_view_info( ) syntax structure is present may include NAL units with nal_unit_type equal to 21 and
avc_3d_extension_flag equal to 1. mvcd_texture_view_flag equal to 0 indicates that the VCL NAL units for the texture
view for the view for which the mvcd_op_view_info( ) syntax structure is present does not include NAL units with both
nal_unit_type equal to 21 and avc_3d_extension_flag equal to 1.


**I.13.2.2** **MVCD scalable nesting SEI message semantics**


The syntax elements in the MVCD scalable nesting SEI message have the same semantics as the ones with the same names
and present in the MVC scalable nesting SEI message in Annex H.


**sei_view_applicability_flag[** i **]** equal to 1 indicates that the nested SEI message applies to both the texture view
component and the depth view component of the view with view_id equal to sei_view_id[ i ].
sei_view_applicability_flag[ i ] equal to 0 indicates that the nested SEI message applies only to the depth view component
of the view with view_id equal to sei_view_id[ i ].


**sei_op_texture_only_flag** equal to 0 specifies that the semantics of sei_op_view_id[ i ] and sei_op_temporal_id apply to
both texture and depth views, if present. sei_op_texture_only_flag equal to 1 specifies that the nested SEI message as well
as the semantics of sei_op_view_id[ i ] and sei_op_temporal_id apply to the sub-bitstream obtained by the sub-bitstream
extraction process of clause I.8.5.3 with depthPresentFlagTarget equal to 0, tIdTarget equal to sei_op_temporal_id, and
viewIdTargetList equal to sei_op_view_id[ i ] for all values of i in the range of 0 to num_view_components_op_minus1,
inclusive, as inputs.

NOTE 1 – MVC scalable nesting SEI message should be used for nesting SEI messages, when depth views may or may not be
present in the bitstream, the nested SEI messages apply only to indicated texture view components and the semantics of the nested
SEI messages apply when VCL and non-VCL NAL units are classified according to Annex H NAL unit type class of Table 7-1.
NOTE 2 – MVCD scalable nesting SEI message with sei_op_texture_only_flag equal to 1 should be used when the nested SEI
messages concern a sub-bitstream from which depth views have been excluded. For example, MVCD scalable nesting SEI message
with sei_op_texture_only_flag equal to 1 may include buffering period and picture timing SEI messages which apply only to a subbitstream containing texture views from which depth views have been removed using the sub-bitstream extraction process of
clause I.8.5.3 with depthPresentFlagTarget equal to 0.


**sei_op_depth_flag** [ i ] equal to 0 specifies that the depth view with view_id equal to sei_op_view_id[ i ] is not included
in the operation point to which the nested SEI message applies. sei_op_depth_flag[ i ] equal to 1 specifies that the depth
view with view_id equal to sei_op_view_id[ i ] is included in the operation point to which the nested SEI message applies.
If sei_op_depth_flag[ i ] is not present, it is inferred to be equal to 1.


**sei_op_texture_flag** [ i ] equal to 0 specifies that the texture view with view_id equal to sei_op_view_id[ i ] is not included
in the operation point to which the nested SEI message applies. sei_op_texture_flag[ i ] equal to 1 specifies that the texture
view with view_id equal to sei_op_view_id[ i ] is included in the operation point to which the nested SEI message applies.
If sei_op_texture_flag[ i ] is not present, it is inferred to be equal to 1. When sei_op_depth_flag[ i ] is equal to 0,
sei_op_texture_flag[ i ] shall be equal to 1.


**I.13.2.3** **Depth representation information SEI message semantics**


The syntax elements in the depth representation information SEI message specifies various parameters for depth views for
the purpose of processing decoded texture and depth view components prior to rendering on a 3D display, such as view
synthesis. Specifically, depth or disparity ranges for depth views are specified. When present, the depth representation
information SEI message may be associated with any access unit. It is recommended, when present, the SEI message is
associated with an IDR access unit for the purpose of random access. The information indicated in the SEI message applies
to all the access units from the access unit the SEI message is associated with to the next access unit, in decoding order,
containing an SEI message of the same type, exclusive, or to the end of the coded video sequence, whichever is earlier in
decoding order.

NOTE 1 – Camera parameters for depth views may be indicated by including a multiview acquisition information SEI message in
an MVCD scalable nesting SEI message with operation_point_flag equal to 0.


**all_views_equal_flag** equal to 0 specifies that depth acquisition information may not be identical to respective values for
each view in target views. all_views_equal_flag equal to 1 specifies that the depth acquisition information are identical to
respective values for all target views.


**num_views_minus1** plus 1 specifies the number of views to which subsequent syntax element apply. When present,
num_views_minus1 shall be less than or equal to NumDepthViews − 1. The value of num_views_minus1 shall be in the
range of 0 to 1023, inclusive.





**z_near_flag** equal to 0 specifies that the syntax elements specifying the nearest depth value are not present in the syntax
structure. z_near_flag equal to 1 specifies that the syntax elements specifying the nearest depth value are present in the
syntax structure.


**z_far_flag** equal to 0 specifies that the syntax elements specifying the farthest depth value are not present in the syntax
structure. z_far_flag equal to 1 specifies that the syntax elements specifying the farthest depth value are present in the
syntax structure.


**z_axis_equal_flag** equal to 0 specifies that the syntax element z_axis_reference_view[ i ] is present. z_axis_equal_flag
equal to 1 specifies that the ZNear and ZFar values, when present, and the decoded samples of depth views, when
depth_representation_type is equal to 0 or 2, have the same Z-axis, which is the Z-axis of the depth view indicated by the
syntax element common_z_axis_reference view.


**common_z_axis_reference_view** specifies the view_id value of the depth view of the Z-axis of the ZNear and ZFar values,
when present, and the decoded samples of depth views, when depth_representation_type is equal to 0 or 2. The value of
common_z_axis_reference_view shall be in the range of 0 to 1023, inclusive.


**d_min_flag** equal to 0 specifies that the syntax elements specifying the minimum disparity value are not present in the
syntax structure. d_min_flag equal to 1 specifies that the syntax elements specifying the minimum disparity value are
present in the syntax structure.


**d_max_flag** equal to 0 specifies that the syntax elements specifying the maximum disparity value are not present in the
syntax structure. d_max_flag equal to 1 specifies that the syntax elements specifying the maximum disparity value are
present in the syntax structure.


**depth_representation_type** specifies the representation definition of decoded luma samples of depth views as specified
in Table I-1. In Table I-1, disparity specifies the horizontal displacement between two texture views and Z value specifies
the distance from a camera. The value of depth_representation_type shall be in the range of 0 to 3, inclusive, in bitstreams
conforming to this version of this Specification. The values of 4 to 15, inclusive, for depth_representation_type are reserved
for future use by ITU-T | ISO/IEC. Although the value of depth_representation_type is required to be in the range of 0 to
3, inclusive, in this version of this Specification, decoders shall allow values of depth_representation_type in the range of
4 to 15, inclusive, to appear in the syntax. Decoders conforming to this version of this Specification shall ignore all data
that follow a value of depth_representation_type in the range of 4 to 15, inclusive, in the depth representation information
SEI messsage.


**Table I-1 – Definition of depth_representation_type**







|depth_representation_type|Interpretation|
|---|---|
|0|Each decoded luma sample value of depth views represents an inverse of Z<br>value that is uniformly quantized into the range of 0 to 255, inclusive.|
|1|Each decoded luma sample value of depth views represents disparity that is<br>uniformly quantized into the range of 0 to 255, inclusive.|
|2|Each decoded luma sample value of depth views represents a Z value<br>uniformly quantized into the range of 0 to 255, inclusive.|
|3|Each decoded luma sample value of depth views represents a non-linearly<br>mapped disparity, normalized in range from 0 to 255, as specified by<br>depth_nonlinear_representation_num_minus1 and<br>depth_nonlinear_representation_model[ i ].|
|4..15|Reserved|


**depth_info_view_id** [ i ] specifies the view_id value to which subsequent syntax elements apply. The value of
depth_info_view_id[ i ] shall be in the range of 0 to 1023, inclusive.


**z_axis_reference_view** [ i ] specifies the view_id value of the depth view of the Z-axis of the ZNear[ i ] and ZFar[ i ]
values, when present, and the decoded samples of the depth view with view_id equal to depth_info_view_id[ i ], when
depth_representation_type is equal to 0 or 2. The value of z_axis_reference_view[ i ] shall be in the range of 0 to 1023,
inclusive.





**disparity_reference_view** [ i ] specifies the view_id value of the depth view used to derive the DMin[ i ] and Dmax[ i ]
values, when present, and the decoded samples of the depth view with view_id equal to depth_info_view_id[ i ], when
depth_representation_type is equal to 1 or 3. The value of disparity_reference_view[ i ] shall be in the range of 0 to 1023,
inclusive.


The variables in the x column of Table I-2 are derived as follows from the respective variables in the s, e, n, and v columns
of Table I-2 as follows:


- If 0 < e < 127, x = ( −1 ) [s] - 2 [( e − 31 )] - ( 1 + n ÷ 2 [v] ).


- Otherwise (e is equal to 0), x = ( −1 ) [s] - 2 [−( 30 + v )] - n.

NOTE 2 – The above specification is similar to that found in IEC 60559.


**Table I-2 – Association between depth parameter variables and syntax elements**

|x|s|e|n|v|
|---|---|---|---|---|
|ZNear[ vId ]|ZNearSign[ vId ]|ZNearExp[ vId ]|ZNearMantissa[ vId ]|ZNearManLen[ vId ]|
|ZFar[ vId ]|ZFarSign[ vId ]|ZFarExp[ vId ]|ZFarMantissa[ vId ]|ZFarManLen[ vId ]|
|DMax[ vId ]|DMaxSign[ vId ]|DMaxExp[ vId ]|DMaxMantissa[ vId ]|DMaxManLen[ vId ]|
|DMin[ vId ]|DMinSign[ vId ]|DMinExp[ vId ]|DMinMantissa[ vId ]|DMinManLen[ vId ]|



If all_views_equal_flag is equal to 0, the variables x in Table I-2 are specified as follows:


- ZNear[ vId ]: The closest depth value for view_id equal to vId.


- ZFar[ vId ]: The farthest depth value for view_id equal to vId.


- DMax[ vId ]: The maximum disparity value for view_id equal to vId.


- DMin[ vId ]: The minimum disparity value for view_id equal to vId.


Otherwise, the variables x in Table I-2 are specified as follows:


- ZNear[ 0 ]: The closest depth value for all depth views.


- ZFar[ 0 ]: The farthest depth value for all depth views.


- DMax[ 0 ]: The maximum disparity value for all depth views.


- DMin[ 0 ]: The minimum disparity value for all depth views.


The DMin and DMax values, when present, are specified in units of a luma sample width of the texture views.


The ZNear and ZFar values, when present, are specified in units of a unit vector of the 3-dimensional coordinate system
used to specify the extrinsic camera parameters as specified by the Multiview Acquisition Information SEI message
associated with the respective depth views, if present. Otherwise, ZNear and ZFar values, when present, are specified in
units of a unit vector of the 3-dimensional coordinate system used to specify the extrinsic camera parameters specified by
the Multiview Acquisition Information SEI message associated with the respective texture views, if present. Otherwise,
the units for the ZNear and ZFar values, if present, are identical but unspecified.


**depth_nonlinear_representation_num_minus1** plus 2 specifies the number of piecewise linear segments for mapping
of depth values to a scale that is uniformly quantized in terms of disparity. The value of
depth_nonlinear_representation_num_minus1 shall be in the range of 0 to 62, inclusive.


**depth_nonlinear_representation_model** [ i ] specifies the piecewise linear segments for mapping of decoded luma
sample values of depth views to a scale that is uniformly quantized in terms of disparity. The value of
depth_nonlinear_representation_model[ i ] shall be in the range of 0 to 65 535, inclusive.

NOTE 3 – When depth_representation_type is equal to 3, depth view component contains non-linearly transformed depth samples.
Variable DepthLUT[ i ], as specified below, is used to transform coded depth sample values from non-linear representation to the
linear representation – disparity normalized in range from 0 to 255. The shape of this transform is defined by means of line-segmentapproximation in two-dimensional linear-disparity-to-non-linear-disparity space. The first (0, 0) and the last (255, 255) nodes of the
curve are predefined. Positions of additional nodes are transmitted in form of deviations (depth_nonlinear_representation_model[ i ])
from the straight-line curve. These deviations are uniformly distributed along the whole range of 0 to 255, inclusive, with spacing
depending on the value of nonlinear_depth_representation_num_minus1.


Variable DepthLUT[ i ] for i in the range of 0 to 255, inclusive, is specified as follows:





depth_nonlinear_representation_model[ 0 ] = 0
depth_nonlinear_representation_model[depth_nonlinear_representation_num_minus1 + 2 ] = 0
for( k=0; k<= depth_nonlinear_representation_num_minus1 + 1; ++k ) {
pos1 = ( 255 * k ) / (depth_nonlinear_representation_num_minus1 + 2 )
dev1 = depth_nonlinear_representation_model[ k ]
pos2 = ( 255 * ( k+1 ) ) / (depth_nonlinear_representation_num_minus1 + 2 ) )
dev2 = depth_nonlinear_representation_model[ k+1 ]

x1 = pos1 − dev1
y1 = pos1 + dev1
x2 = pos2 − dev2
y2 = pos2 + dev2

for ( x = max( x1, 0 ); x <= min( x2, 255 ); ++x )
DepthLUT[ x ] = Clip3( 0, 255, Round( ( ( x - x1 ) * ( y2 - y1 ) ) ÷ ( x2 - x1 ) + y1 ) )
}


When depth_representation_type is equal to 3, DepthLUT[ dS ] for all decoded luma sample values dS of depth views in
the range of 0 to 255, inclusive, represents disparity that is uniformly quantized into the range of 0 to 255, inclusive.


**I.13.2.3.1** **Depth representation SEI element semantics**


The syntax structure specifies the value of an element in depth representation information.


The depth_representation_sei_element( outSign, outExp, outMantissa, outManLen ) syntax structure sets the values of the
outSign, outExp, outMantissa, and outManLen variables that represent a floating-point value. When the syntax structure
is included in another syntax structure, the variable names outSign, outExp, outMantissa, and outManLen are to be
interpreted as being replaced by the variable names used when the syntax structure is included.


**da_sign_flag** equal to 0 indicates that the sign of the floating-point value is positive. da_sign_flag equal to 1 indicates that
the sign is negative. The variable outSign is set equal to da_sign_flag.


**da_exponent** specifies the exponent of the floating-point value. The value of da_exponent shall be in the range of 0
to 2 [7] - 2, inclusive. The value 2 [7] - 1 is reserved for future use by ITU-T | ISO/IEC. Decoders shall treat the value 2 [7] - 1
as indicating an unspecified value. The variable outExp is set equal to da_exponent.


**da_mantissa_len_minus1** plus 1 specifies the number of bits in the da_mantissa syntax element. The variable outManLen
is set equal to da_mantissa_len_minus1 + 1.


**da_mantissa** specifies the mantissa of the floating-point value. The variable outMantissa is set equal to da_mantissa.


**I.13.2.4** **3D reference displays information SEI message semantics**


When present, this SEI message shall be associated with an IDR access unit. A reference displays information message
contains information about the reference display width(s) and reference viewing distance(s) as well as information about
the corresponding baseline distance(s) and additional horizontal image shift(s), which form a stereo pair for the reference
display width and the reference viewing distance. This information enables a view renderer to produce a proper stereo pair
for the target screen width and the viewing distance. The reference display width and viewing distance values are signalled
in units of centimetres. The reference baseline values shall be signalled in the same units as the x component of the
translation vector in the multiview acquisition information SEI message that is valid for the same access unit. When a
reference displays information SEI message is present in an access unit, the multiview acquisition information SEI message
shall also be present in the same access unit. The baseline and shift information signalled for the reference display is valid
for all access units they associated with and until the next IDR access unit or the next access unit containing multiview
acquisition information SEI message or reference displays information SEI message.


NOTE – The reference displays information SEI message specifies display parameters for which the 3D sequence was optimized
and the corresponding reference parameters. Each reference display (i.e. a reference display width and possibly a corresponding
viewing distance) is associated with one reference baseline distance.


The following formulas can be used for calculating the baseline distance and horizontal shift for the receiver's display when the
ratio between the receiver's viewing distance and the reference viewing distance is the same as the ratio between the receiver screen
width and the reference screen width:


baseline = ref_baseline * ( ref_display_width ÷ display_width )


shift = ref_shift * ( ref_display_width ÷ display_width )


In the provided formulas, the width of the visible part of the display used for showing the video sequence should be understood
under "display width". The same formulas can also be used for choosing the baseline distance and horizontal shift in cases when
the viewing distance is not scaled proportionally to the screen width compared to the reference display parameters. In this case, the





effect of applying these formulas would be to keep the perceived depth in the same proportion to the viewing distance as in the
reference setup.


When camera parameters are updated by a multiview acquisition information SEI message in a following access unit and the
baseline between the views used in the view synthesis process in the following access unit changes relative to that in the in the
access unit which the reference displays information SEI belongs to, the baseline and the horizontal shift for the receiver's display
in the following access unit should be modified accordingly. Let the scaling factor _s_ be equal to the ratio of the baseline between
two views in the following access unit and the baseline between the same two views in the access unit, which the reference displays
information SEI message belongs to, where the two views are used in the view synthesis process. Then the baseline distance for
the receiver's display in the following access unit should be modified with the scaling factor _s_ relative to the baseline distance for
the receiver's display in the access unit which the reference displays information SEI belongs to. The horizontal shift for the
receiver's display should also be modified by scaling it with the same factor as that used to scale the baseline distance.


**prec_ref_baseline** specifies the exponent of the maximum allowable truncation error for ref_baseline[ i ] as given by
2 [−prec_ref_baseline] . The value of prec_ref_baseline shall be in the range of 0 to 31, inclusive.


**prec_ref_display_width** specifies the exponent of the maximum allowable truncation error for ref_display_width[ i ] as
given by 2 [−prec_ref_display_width] . The value of prec_ref_display_width shall be in the range of 0 to 31, inclusive.


**ref_viewing_distance_flag** equal to 1 indicates the presence of reference viewing distance. ref_viewing_distance_flag
equal to 0 indicates that the reference viewing distance is not present in the bitstream.


**prec_ref_viewing_dist** specifies the exponent of the maximum allowable truncation error for ref_viewing_dist[ i ] as
given by 2 [−prec_ref_viewing_dist] . The value of prec_ref_viewing_dist shall be in the range of 0 to 31, inclusive.


**num_ref_displays_minus1** plus 1 specifies the number of reference displays that are signalled in the bitstream. The value
of num_ref_displays_minus1 shall be in the range of 0 to 31, inclusive.


**exponent_ref_baseline[** i **]** specifies the exponent part of the reference baseline for the i-th reference display. The value
of exponent_ref_baseline[ i ] shall be in the range of 0 to 62, inclusive. The value 63 is reserved for future use by
ITU-T | ISO/IEC. Decoders shall treat the value 63 as indicating an unspecified reference baseline.


**mantissa_ref_baseline[** i **]** specifies the mantissa part of the reference baseline for the i-th reference display. The length
of the mantissa_ref_baseline[ i ] syntax element is variable and determined as follows:


- If exponent_ref_baseline[ i ] = = 0, the length is Max( 0, prec_ref_baseline - 30 ).


- Otherwise (0 < exponent_ref_baseline[ i ] < 63), the length is Max( 0, exponent_ref_baseline[ i ] + prec_ref_baseline

   - 31 ).


**exponent_ref_display_width[** i **]** specifies the exponent part of the reference display width of the i-th reference display.
The value of exponent_ref_display_width[ i ] shall be in the range of 0 to 62, inclusive. The value 63 is reserved for future
use by ITU-T | ISO/IEC. Decoders shall treat the value 63 as indicating an unspecified reference display width.


**mantissa_ref_display_width[** i **]** specifies the mantissa part of the reference display width of the i-th reference display.
The length of the mantissa_ref_display_width[ i ] syntax element is variable and determined as follows:


- If exponent_ref_display_width[ i ] = = 0, the length is Max( 0, prec_ref_display_width - 30 ).


- Otherwise (0 < exponent_ref_display_width[ i ] < 63), the length is Max( 0, exponent_ref_display_width[ i ] +
prec_ref_display_width   - 31 ).


**exponent_ref_viewing_distance[** i **]** specifies the exponent part of the reference viewing distance of the i-th reference
display. The value of exponent_ref_viewing_distance[ i ] shall be in the range of 0 to 62, inclusive. The value 63 is reserved
for future use by ITU-T | ISO/IEC. Decoders shall treat the value 63 as indicating an unspecified reference display width.


**mantissa_ref_viewing_distance[** i **]** specifies the mantissa part of the reference viewing distance of the i-th reference
display. The length of the mantissa_ref_viewing_distance[ i ] syntax element is variable and determined as follows:


- If exponent_ref_viewing_distance[ i ] = = 0, the length is Max( 0, prec_ref_viewing_distance − 30 ).


- Otherwise ( 0 < exponent_ref_viewing_distance[ i ] < 63 ), the length is Max( 0, exponent_ref_viewing_distance[ i ]
+ prec_ref_viewing_distance − 31 ).


The variables in the x column of Table I-3 are derived as follows from the respective variables or values in the s, e, and n
columns of Table I-3 as follows:


- If 0 < e < 63, x = ( −1 ) [s] - 2 [( e − 31 )] - ( 1 + n ÷ 2 [v] ).


- Otherwise (e is equal to 0), x = ( −1 ) [s] - 2 [−( 30 + v )] - n.


NOTE – The above specification is similar to that found in IEC 60559.





**Table I-3 – Association between camera parameter variables and syntax elements**

|x|s|e|n|
|---|---|---|---|
|refBaseline[ i ]|0|exponent_ref_baseline[ i ]|mantissa_ref_baseline[ i ]|
|refDisplayWidth[ i ]|0|exponent_ref_display_width[ i ]|mantissa_ref_display_width[ i ]|
|refViewingDistance[ i ]|0|exponent_ref_viewing_distance[ i ]|mantissa_ref_viewing_distance[ i ]|



**additional_shift_present_flag[** i **]** equal to 1 indicates that the information about additional horizontal shift of the left and
right views for the i-th reference display is present in the bitstream. additional_shift_present_flag[ i ] equal to 0 indicates
that the information about additional horizontal shift of the left and right views for the i-th reference display is not present
in the bitstream.


**num_sample_shift_plus512[** i **]** indicates the recommended additional horizontal shift for a stereo pair corresponding to
the i-th reference baseline and the i-th reference display. If ( num_sample_shift_plus512[ i ] − 512 ) is less than 0, it is
recommended that the left view of the stereo pair corresponding to the i-th reference baseline and the i-th reference display
is shifted in the left direction by ( 512 − num_sample_shift_plus512[ i ] ) samples with respect to the right view of the
stereo pair; if num_sample_shift_plus512[ i ] is equal to 512, it is recommended that shifting is not applied; if
( num_sample_shift_plus512[ i ] − 512 ) is greater than 0, it is recommended that the left view in the stereo pair
corresponding to the i-th reference baseline and the i-th reference display should be shifted in the right direction by
( num_sample_shift_plus512[ i ] − 512 ) samples with respect to the right view of the stereo pair. The value of
num_sample_shift_plus512[ i ] shall be in the range of 0 to 1023, inclusive.


**three_dimensional_reference_displays_extension_flag** equal to 0 indicates that no additional data follows within the
reference displays SEI message. The value of three_dimensional_reference_displays_extension_flag shall be equal to 0.
The value of 1 for three_dimensional_reference_displays_extension_flag is reserved for future use by ITU-T | ISO/IEC.
Decoders shall ignore all data that follows the value of 1 for three_dimensional_reference_displays_extension_flag in a
reference displays SEI message.

NOTE – Shifting the left view in the left (or right) direction by X samples with respect to the right view can be performed by the
following two-step processing:


1) shift the left view by X / 2 samples in the left (or right) direction, and shift the right view by X / 2 samples in the right (or
left) direction


2) fill the left and right image margins of X / 2 samples in width in both the left and right views in background colour.


The following pseudo code explains the recommended shifting processing in the case of shifting the left view in the left direction
by X samples with respect to the right view.


for ( i = X / 2; i < width − X / 2; i++ ) {
for ( j = 0; j < height; j++ ) {
left_view[ j ][ i ] = left_view[ j ][ i + X / 2 ]
right_view[ j ][ width − 1 − i ] = right_view[ j ][ width − 1 − i − X / 2 ]
}
}
for ( i = 0; i < X / 2; i++ ) {
for ( j = 0; j < height; j++ ) {
left_view[ j ][ width − 1 − i ] = left_view[ j ][ i ] = BackgroundColour
right_view[ j ][ width − 1 − i ] = right_view[ j ][ i ] = BackgroundColour
}
}


The following pseudo code explains the recommended shifting processing in the case of shifting the left view in the right direction
by X samples with respect to the right view.


for ( i = X / 2; i < width − X / 2; i++ ) {
for ( j = 0; j < height; j++ ) {
left_view[ j ][ width − 1 − i ] = left_view[ j ][ width − 1 − i − X / 2 ]
right_view[ j ][ i ] = right_view[ j ][ i + X / 2 ]
}
}
for ( i=0; i < X / 2; i++ ) {
for ( j = 0; j < height; j++ ) {
left_view[ j ][ width − 1− i ] = left_view[ j ][ i ] = BackgroundColour
right_view[ j ][ width − 1− i ] = right_view[ j ][ i ] = BackgroundColour





}
}


BackgroundColour may take different values in different systems, for example black or grey.


**I.13.2.5** **Depth timing SEI message semantics**


The depth timing SEI message indicates the acquisition time of the depth view components of one or more access units
relative to the DPB output time of the same access units. The depth timing SEI message may be present in any access unit
and it pertains until the end of the coded video sequence or until the next depth timing SEI message, whichever is earlier
in decoding order. The access units that the message pertains to are referred to as the target access unit set.


**per_view_depth_timing_flag** equal to 0 specifies that all the depth view components within the target access unit set have
the same acquisition time offset relative to the DPB output time of the respective access unit in the target access unit set.
The single occurrence of the depth_timing_offset structure specifies this acquisition time offset.


per_view_depth_timing_flag equal to 1 specifies that a depth_timing_offset syntax structure is present for each depth view
in ascending order of view order index values for the depth views and specifies the acquisition time offset for that view.


**I.13.2.5.1** **Depth timing offset semantics**


**offset_len_minus1** specifies the length of the depth_disp_delay_offset_fp syntax element.


**depth_disp_delay_offset_fp** and **depth_disp_delay_offset_dp** specify that the acquisition offset of the respective depth
view component or components relative to the DPB output time of the access unit containing the depth view component
or components is equal to depth_disp_delay_offset_fp ÷ 2 [depth_disp_delay_offset_dp] in units of clock ticks as specified in Annex C.


The length of depth_disp_delay_offset_fp syntax element is equal to offset_len_minus1 + 1.


If depth_disp_delay_offset_fp is not present, it is inferred to be equal to 0. If depth_disp_delay_offset_dp is not present, it
is inferred to be equal to 0.


**I.13.2.6** **Alternative depth information SEI message semantics**


The alternative depth information SEI message indicates that the view components of one output view contain samples of
multiple distinct spatially packed constituent pictures. The related output view is not suitable to be displayed directly. This
SEI message can be used by the application after the decoder output to rearrange the samples to produce additional views
that are appropriate for display or use by a server for other purposes (which are outside the scope of this Recommendation |
International Standard).


When the alternative depth information SEI message is present, the texture and depth views referred to by this SEI message
shall conform to the specifications in Annexes I and J, ChromaArrayType shall be equal to 1 (4:2:0),
bit_depth_luma_minus8 shall be equal to 0, bit_depth_chroma_minus8 shall be equal to 0, and the depth representation
information SEI message shall be present. The depth_representation_type is defined in the depth representation information
SEI message. The depth representation is only carried in the luma component.


The alternative depth information SEI message, when present, shall be associated with an IDR access unit. The information
signalled in this SEI message applies to all the access units in the coded video sequence. Also, num_anchor_refs_l0[ i ],
num_anchor_refs_l1[ i ], num_non_anchor_refs_l0[ i ] and num_non_anchor_refs_l1[ i ] shall be equal to 0 in the active
SPS RBSP syntax structure for the coded video sequence and avc_3d_extension_flag shall be equal to 0 in all VCL NAL
units of the coded video sequence.

NOTE 1 – These constraints disable inter-view and inter-component prediction.


In such a coded video sequence, there shall be two output views. The base view is a complete view and the non-base view
contains a packing arrangement of 1 to 4 additional views with half the width and half the height of the base view. Such
lower-resolution views are referred to as constituent pictures herein.

NOTE 2 – The view_id of the non-base view is not used in any particular way for this SEI message.


All constituent pictures have a width and a height equal to ( ( pic_width_in_mbs_minus1 + 1 ) * 8 ) and
( ( pic_height_in_map_unit_minus1 + 1 ) * 8 ) in luma samples, respectively. frame_mbs_only_flag shall be equal to 1.
The variable i, with a value from 0 to num_constituent_views_gvd_minus1 + 1, inclusive, indicates the location of the
constituent pictures in the non-base texture view as specified in Table I-4. i equal to 0 indicates the base texture view. i
greater than 0 indicates the constituent texture view.





**Table I-** 4 **– Locations of the top-left luma samples of constituent pictures packed in a non-base texture view**

**relative to the top-left luma sample of this picture**

|Constituent picture index i|Location of the top-left luma sample in a non-base texture view|
|---|---|
|1|( 0, 0 )|
|2|( 0, ( pic_height_in_map_unit_minus1 + 1 ) * 8 )|
|3|( ( pic_width_in_mbs_minus1 + 1 ) * 8, 0 )|
|4|( (pic_width_in_mbs_minus1 + 1 ) * 8, ( pic_height_in_map_unit_minus1 + 1 ) * 8 )|



The decoded depth views have the same structure as the decoded texture views, i.e., they consist of a base depth view (i
equal to 0) and a non-base depth view. The non-base depth view contains up to four constituent depth pictures (i in the
range of 1 to 4) for the constituent views. The constituent depth pictures are packed in the non-base depth view in the same
arrangement as the constituent texture pictures as specified in Table I-4.


This arrangement of texture and depth constituent views is referred to as global view and depth (GVD) information.


**depth_type** shall be equal to 0. Other values are reserved for future use by ITU-T | ISO/IEC and shall not be present in
bitstreams conforming to this Specification. Decoders shall ignore alternative depth information SEI messages in which
such other values are present.


**num_constituent_views_gvd_minus1** plus 1 identifies the number of constituent texture pictures packed into each texture
component of the non-base view. num_constituent_views_gvd_minus1 shall be in the range of 0 to 3, inclusive.


**depth_present_gvd_flag** equal to 1 indicates that constituent depth pictures are packed into the depth components of the
non-base view, with a packing arrangement as described above. depth_present_gvd_flag equal to 0 specifies that the depth
component of the non-base view is not present.


Each constituent picture in the depth component of the non-base view is associated with a constituent picture in the texture
component of the non-base view in the same relative location. The number of depth views in the coded video sequence is
equal to 1 + depth_present_gvd_flag. The depth component of the base view shall always be present, independent of the
value of depth_present_gvd_flag.

NOTE 3 – The following SEI message parameters can be used along with the decoded depth components to project samples from
the base view into the coordinates of constituent views such that reconstructed views can be generated by combining projected
samples and samples from the constituent views.


The function binToFp( s, e, n, v ) is specified as follows:


binToFp( s, e, n, v ) = ( −1 ) [s]   - ( e = = 0 ? ( 2 [−( 30 + v ) ]   - n ) : ( 2 [( e − 31 )]   - ( 1 + n ÷ 2 [v] ) ) ) (I-1)
NOTE 4 – The above specification is similar to what is found in IEC 60559:1989, Binary floating-point arithmetic for
microprocessor systems.


**z_gvd_flag** equal to 1 indicates the presence of the syntax elements sign_gvd_z_near_flag[ i ], exp_gvd_z_near[ i ],
man_len_gvd_z_near_minus1[ i ], man_gvd_z_near[ i ], sign_gvd_z_far_flag[ i ], exp_gvd_z_far[ i ],
man_len_gvd_z_far_minus1[ i ], and man_gvd_z_far[ i ], for i in the range of 0 to num_constituent_views_gvd_minus1 +
1, inclusive. z_gvd_flag equal to 0 indicates that these syntax elements are not present.


**intrinsic_param_gvd_flag** equal to 1 indicates the presence of intrinsic camera parameter syntax elements.
intrinsic_param_gvd_flag equal to 0 indicates that these syntax elements are not present.


**rotation_gvd_flag** equal to 1 indicates the presence of rotation camera parameter syntax elements. rotation_gvd_flag equal
to 0 indicates that these syntax elements are not present. When rotation_gvd_flag is 0, a default rotation camera parameter
of a unit matrix value is inferred.


**translation_gvd_flag** equal to 1 indicates the presence of horizontal translation camera parameter syntax elements.
translation_gvd_flag equal to 0 indicates that these syntax elements are not present.


**sign_gvd_z_near_flag** [ i ] equal to 0 indicates that the sign of the nearest depth value of the i-th camera is positive.
sign_gvd_z_near[ i ] equal to 1 indicates that the sign of the nearest depth value of the i-th camera is negative.


**exp_gvd_z_near** [ i ] specifies the exponent part of the nearest depth value of the i-th camera. The value of
exp_gvd_z_near[ i ] shall be in the range of 0 to 126, inclusive. The value 127 is reserved for future use by ITU-T |
ISO/IEC. When exp_gvd_z_near[ i ] is equal to 127, the value of zNear[ i ] is unspecified.


**man_len_gvd_z_near_minus1** [ i ] plus 1 specifies the length in bits of the mantissa of the nearest depth value of the i-th
camera. The value of man_len_gvd_z_near_minus1 [ i ] shall be in the range of 0 to 31, inclusive.


**man_gvd_z_near** [ i ] specifies the mantissa part of the nearest depth value of the i-th camera. The length of
man_gvd_z_near[ i ] syntax elements is man_len_gvd_z_near_minus1[ i ] + 1 bits.





When exp_gvd_z_near[ i ] is not equal to 127, zNear[ i ] is set equal to
binToFp( sign_gvd_z_near_flag[ i ], exp_gvd_z_near[ i ], man_gvd_z_near[ i ], man_len_gvd_z_near_minus1[ i ] + 1 ).


**sign_gvd_z_far_flag** [ i ] equal to 0 indicates that the sign of the farthest depth value of the i-th camera is positive.
sign_gvd_z_far_flag[ i ] equal to 1 indicates that the sign of the farthest depth value of the i-th camera is negative.


**exp_gvd_z_far** [ i ] specifies the exponent part of the farthest depth value of the i-th camera. The value of
exp_gvd_z_far[ i ] shall be in the range of 0 to 126, inclusive. The value 127 is reserved for future use by ITU-T | ISO/IEC.
When exp_gvd_z_far[ i ] is equal to 127, the value of zFar[ i ] is unspecified.


**man_len_gvd_z_far_minus1** [ i ] plus 1 specifies the length in bits of the mantissa of the farthest depth value of the i-th
camera. The value of man_len_gvd_z_far_minus1[ i ] shall be in the range of 0 to 31, inclusive.


**man_gvd_z_far** [ i ] specifies the mantissa part of the farthest depth value of the i-th camera. The length of
man_gvd_z_far[ i ] syntax elements is man_len_gvd_z_far_minus1[ i ] + 1 bits.


When exp_gvd_z_far[ i ] is not equal to 127, zFar[ i ] is set equal to
binToFp( sign_gvd_z_far_flag[ i ], exp_gvd_z_far[ i ], man_gvd_z_far[ i ], man_len_gvd_z_far_minus1[ i ] + 1 ).


**prec_gvd_focal_length** specifies the exponent of the maximum allowable truncation error for focalLengthX[ i ] and
focalLengthY[ i ] as given by 2 [−prec_gvd_focal_length] . The value of prec_gvd_focal_length shall be in the range of 0 to 31,
inclusive.


**prec_gvd_principal_point** specifies the exponent of the maximum allowable truncation error for principalPointX[ i ] and
principalPointY[ i ] as given by 2 [−prec_gvd_principal_point] . The value of prec_gvd_principal_point shall be in the range of 0 to 31,
inclusive.


**prec_gvd_rotation_param** specifies the exponent of the maximum allowable truncation error for r[ i ][ j ][ k ] as given
by 2 [−pre_gvdc_rotation_param] . The value of prec_gvd_rotation_param shall be in the range of 0 to 31, inclusive.


**prec_gvd_translation_param s** pecifies the exponent of the maximum allowable truncation error for tX[ i ] as given by
2 [−prec_gvd_translation_param] . The value of prec_gvd_translation_param shall be in the range of 0 to 31, inclusive.


**sign_gvd_focal_length_x** [ i ] equal to 0 indicates that the sign of the focal length of the i-th camera in the horizontal
direction is positive. sign_gvd_focal_length_x[ i ] equal to 1 indicates that the sign of the focal length of the i-th camera
in the horizontal direction is negative.


**exp_gvd_focal_length_x** [ i ] specifies the exponent part of the focal length of the i-th camera in the horizontal direction.
The value of exp_gvd_focal_length_x[ i ] shall be in the range of 0 to 62, inclusive. The value 63 is reserved for future use
by ITU-T | ISO/IEC. When exp_gvd_focal_length_x[ i ] is equal to 63, the value of focal length of the horizontal direction
for the i-th camera is unspecified.


**man_gvd_focal_length_x** [ i ] specifies the mantissa part of the focal length of the i-th camera in the horizontal direction.
The length v of the man_gvd_focal_length_x[ i ] syntax element is determined as follows:


- If exp_gvd_focal_length_x[ i ] is equal to 0, the length v is set equal to Max( 0, prec_gvd_focal_length − 30 ).


- Otherwise (exp_gvd_focal_length_x[ i ] is in the range of 1 to 62, inclusive), the length v is
Max( 0, exp_gvd_focal_length_x[ i ] + prec_gvd_focal_length − 31 ).


When exp_gvd_focal_length_x[ i ] is not equal to 63, the variable focalLengthX[ i ] is set equal to
binToFp( sign_gvd_focal_length_x[ i ], exp_gvd_focal_length_x[ i ], man_gvd_focal_length_x[ i ], v ).


**sign_gvd_focal_length_y** [ i ] equal to 0 indicates that the sign of the focal length of the i-th camera in the vertical direction
is positive. sign_gvd_focal_length_y[ i ] equal to 1 indicates that the sign of the focal length of the i-th camera in the
vertical direction is negative.


**exp_gvd_focal_length_y** [ i ] specifies the exponent part of the focal length of the i-th camera in the vertical direction. The
value of exp_gvd_focal_length_y[ i ] shall be in the range of 0 to 62, inclusive. The value 63 is reserved for future use by
ITU-T | ISO/IEC. When exp_gvd_focal_length_y[ i ] is equal to 63, the value of focal length of the vertical direction is
unspecified.


**man_gvd_focal_length_y** [ i ] specifies the mantissa part of the focal length of the i-th camera in the vertical direction.


The length v of the man_gvd_focal_length_y[ i ] syntax element is determined as follows:


- If exp_gvd_focal_length_y[ i ] is equal to 0, the length v is set equal to Max( 0, prec_gvd_focal_length − 30 ).


- Otherwise (exp_gvd_focal_length_y[ i ] is in the range of 1 to 62, inclusive), the length v is set equal to
Max( 0, exp_gvd_focal_length_y[ i ] + prec_gvd_focal_length − 31 ).


When exp_gvd_focal_length_y[ i ] is not equal to 63, the variable focalLengthY[ i ] is set equal to





binToFp( sign_gvd_focal_length_y[ i ], exp_gvd_focal_length_y[ i ], man_gvd_focal_length_y[ i ], v ).


**sign_gvd_principal_point_x** [ i ] equal to 0 indicates that the sign of the principal point of the i-th camera in the horizontal
direction is positive. sign_gvd_principal_point_x[ i ] equal to 1 indicates that the sign of the principal point of the i-th
camera in the horizontal direction is negative.


**exp_gvd_principal_point_x** [ i ] specifies the exponent part of the principal point of the i-th camera in the horizontal
direction. The value of exp_gvd_principal_point_x[ i ] shall be in the range of 0 to 62, inclusive. The value 63 is reserved
for future use by ITU-T | ISO/IEC. When exp_gvd_principal_point_x[ i ] is equal to 63, the value of principal point in the
horizontal direction for the i-th camera is unspecified.


**man_gvd_principal_point_x** [ i ] specifies the mantissa part of the principal point of the i-th camera in the horizontal
direction The length v of the man_gvd_principal_point_x[ i ] syntax element in units of bits is determined as follows:


- If exp_gvd_principal_point_x[ i ] is equal to 0, the length v is set equal to Max( 0, prec_gvd_principal_point − 30 ).


- Otherwise (exp_gvd_principal_point_x[ i ] is in the range of 1 to 62, inclusive), the length v is set equal to
Max( 0, exp_gvd_principal_point_x[ i ] + prec_gvd_principal_point − 31 ).


When exp_gvd_principal_point_x[ i ] is not equal to 63, the variable principalPointX[ i ] is set equal to
binToFp( sign_gvd_principal_point_x[ i ], exp_gvd_principal_point_x[ i ], man_gvd_principal_point_x[ i ], v ).


**sign_gvd_principal_point_y** [ i ] equal to 0 indicates that the sign of the principal point of the i-th camera in the vertical
direction is positive. sign_gvd_principal_point_y[ i ] equal to 1 indicates that the sign of the principal point of the i-th
camera in the vertical direction is negative.


**exp_gvd_principal_point_y** [ i ] specifies the exponent part of the principal point of the i-th camera in the vertical
direction. The value of exp_gvd_principal_point_y[ i ] shall be in the range of 0 to 62, inclusive. The value 63 is reserved
for future use by ITU-T | ISO/IEC. When exp_gvd_principal_point_y[ i ] is equal to 63, the value of principal point in the
vertical direction for the i-th camera is unspecified.


**man_gvd_principal_point_y** [ i ] specifies the mantissa part of the principal point of the i-th camera in the vertical
direction The length v of the man_gvd_principal_point_y[ i ] syntax element in units of bits is determined as follows:


- If exp_gvd_principal_point_y[ i ] is equal to 0, the length v is set equal to Max( 0, prec_gvd_principal_point − 30 ).


- Otherwise (exp_gvd_principal_point_y[ i ] is in the range of 1 to 62, inclusive), the length v is set equal to
Max( 0, exp_gvd_principal_point_y[ i ] + prec_gvd_principal_point − 31 ).


When exp_gvd_principal_point_y[ i ] is not equal to 63, the variable principalPointY[ i ] is set equal to
binToFp( sign_gvd_principal_point_y[ i ], exp_gvd_principal_point_y[ i ], man_gvd_principal_point_y[ i ], v ).


**sign_gvd_r** [ i ][ j ][ k ] equal to 0 indicates that the sign of the (j, k) component of the rotation matrix for the i-th camera
is positive. sign_gvd_r[ i ][ j ][ k ] equal to 1 indicates that the sign of the (j, k) component of the rotation matrix for the ith camera is negative.


**exp_gvd_r** [ i ][ j ][ k ] specifies the exponent part of the (j, k) component of the rotation matrix for the i-th camera. The
value of exp_gvd_r[ i ][ j ][ k ] shall be in the range of 0 to 62, inclusive. The value 63 is reserved for future use by ITU-T
| ISO/IEC. When exp_gvd_r[ i ][ j ][ k ] is equal to 63, the value of rotation matrix is unspecified.


**man_gvd_r** [ i ][ j ][ k ] specifies the mantissa part of the (j, k) component of the rotation matrix for the i-th camera.


The length v of the man_gvd_r[ i ][ j ][ k ] syntax element in units of bits is determined as follows:


- If exp_gvd_r[ i ][ j ][ k ] is equal to 0, the length v is set equal to Max( 0, prec_gvd_rotation_param − 30 ).


- Otherwise (exp_gvd_r[ i ][ j ][ k ] is in the range of 1 to 62, inclusive), the length v is set equal to
Max( 0, exp_gvd_r[ i ][ j ][ k ] + prec_gvd_rotation_param − 31 ).


When exp_gvd_r[ i ][ j ][ k ] is not equal to 63, the variable r[ i ][ j ][ k ] is set equal to
binToFp( sign_gvd_r[ i ][ j ][ k ], exp_gvd_r[ i ][ j ][ k ], man_gvd_r[ i ][ j ][ k ], v ).


The rotation matrix R[ i ] for i-th camera is represented as follows:



R[ i ] = [



r[ i ][ 0 ][ 0 ] r[ i ][ 0 ][ 1 ] r[ i ][ 0 ][ 2 ]
r[ i ][ 1 ][ 0 ] r[ i ][ 1 ][ 1 ] r[ i ][ 1 ][ 2 ] ] (I-2)
r[ i ][ 2 ][ 0 ] r[ i ][ 2 ][ 1 ] r[ i ][ 2 ][ 2 ]



**sign_gvd_t_x** [ i ] equal to 0 indicates that the sign of the horizontal component of the translation vector for the i-th camera
is positive. sign_gvd_t_x[ i ] equal to 1 indicates that the sign of the horizontal component of the translation vector for the
i-th camera is negative.





**exp_gvd_t_x** [ i ] specifies the exponent part of the horizontal component of the translation vector for the i-th camera. The
value of exp_gvd_t_x[ i ] shall be in the range of 0 to 62, inclusive. The value 63 is reserved for future use by ITU-T |
ISO/IEC. When exp_gvd_t_x[ i ] is equal to 63, the value of the translation vector is unspecified.


**man_gvd_t_x** [ i ] specifies the mantissa part of the horizontal component of the translation vector for the i-th camera.


The length v of the man_gvd_t_x[ i ] syntax element in units of bits is determined as follows:


- If exp_gvd_t_x[ i ] is equal to 0, the length v is set equal to Max( 0, prec_gvd_translation_param − 30 ).


- Otherwise (exp_gvd_t_x[ i ] is in the range of 1 to 62, inclusive), the length v is set equal to
Max( 0, exp_gvd_t_x[ i ] + prec_gvd_translation_param − 31 ).


When exp_gvd_t_x[ i ] is not equal to 63, the variable tX[ i ] is set equal to
binToFp( sign_gvd_t_x[ i ], exp_gvd_t_x[ i ], man_gvd_t_x[ i ], v ).


**I.13.2.7** **Depth sampling information SEI message semantics**


The depth sampling information SEI message specifies the depth sample size relative to luma texture sample size. In
addition, the depth sampling information SEI message specifies the depth sampling grid position of one or more depth
view components of the associated access unit relative to the sampling grid of the texture view components of the same
access unit with the same view_id value. When present, the depth sampling information SEI message shall be associated
with an IDR access unit. The semantics of the message are valid for the current coded video sequence.


NOTE – The depth sample size and the depth sampling grid position are indicated for frame or field view components that are
present in the associated IDR access unit. In subsequent access units in the coded video sequence the depth view components may
have different values of field_pic_flag and bottom_field_flag compared to those of the depth view components of the IDR access
unit. Likewise, in subsequent access units in the coded video sequence the texture view components may have different values of
field_pic_flag and bottom_field_flag compared to those of the texture view components of the IDR access unit. The depth sample
size and depth sampling grid position should be modified according to the values of field_pic_flag and bottom_field_flag of the
texture and depth view components of an access unit compared to those of the IDR access unit.


**dttsr_x_mul** and **dttsr_x_dp** indicate that the width of a depth sample relative to the width of a luma texture sample is
approximately dttsr_x_mul ÷ 2 [dttsr_x_dp] . When dttsr_x_mul is not present, it is inferred to be equal to 1. When dttsr_x_dp is
not present, it is inferred to be equal to 0. The value of 0 for dttsr_x_mul is reserved.


**dttsr_y_mul** and **dttsr_y_dp** indicate that the height of a depth sample relative to the height of a luma texture sample is
approximately dttsr_y_mul ÷ 2 [dttsr_y_dp] . When dttsr_y_mul is not present, it is inferred to be equal to 1. When dttsr_y_dp is
not present, it is inferred to be equal to 0. The value of 0 for dttsr_y_mul is reserved.


**per_view_depth_grid_pos_flag** equal to 0 specifies that the depth sampling grid position information is the same for all
depth views for which there is a texture view with the same view_id present. The single occurrence of the
depth_grid_position( ) syntax structure indicates the depth sampling grid position. per_view_depth_grid_pos_flag equal
to 1 specifies that a depth_grid_position( ) syntax structure is present for indicated depth views.


**num_video_plus_depth_views_minus1** (when present) plus 1 specifies the number of views for which the depth sampling
grid position information is present in this SEI message.


**depth_grid_view_id[** i **]** specifies the i-th view_id value for which the depth sampling grid position information is
specified with the depth_grid_position( ) structure following in the syntax structure.


**I.13.2.7.1** **Depth grid position semantics**


**depth_grid_pos_x_fp**, **depth_grid_pos_x_dp** and **depth_grid_pos_x_sign_flag** indicate that the location of the
horizontal position of the top-left sample in the sampling grid for a depth view component, relative to the location of the
top-left sample in the sampling grid for the luma component of the texture view component with the same value of view_id,
is equal to ( 1 − 2 * depth_grid_pos_x_sign_flag ) * ( depth_grid_pos_x_fp ÷ 2 [depth_grid_pos_x_dp] ).


When depth_grid_pos_x_fp, depth_grid_pos_x_dp, and depth_grid_pos_x_sign_flag are not present, they should be
inferred to be equal to 0.


**depth_grid_pos_y_fp**, **depth_grid_pos_y_dp** and **depth_grid_pos_y_sign_flag** indicate that the location of the vertical
position of the top-left sample in the sampling grid for a depth view component, relative to the location of the top-left
sample in the sampling grid for the luma component of the texture view component with the same value of view_id, is
equal to ( 1 − 2 * depth_grid_pos_y_sign_flag ) * ( depth_grid_pos_y_fp ÷ 2 [depth_grid_pos_y_dp] ).


When depth_grid_pos_y_fp, depth_grid_pos_y_dp, and depth_grid_pos_y_sign_flag are not present, they should be
inferred to be equal to 0.
