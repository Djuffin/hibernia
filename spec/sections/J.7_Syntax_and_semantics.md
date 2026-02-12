**J.7** **Syntax and semantics**


This clause specifies syntax and semantics for coded video sequences that conform to one or more of the profiles specified
in this annex.


**J.7.1** **Method of specifying syntax in tabular form**


The specifications in clause I.7.1 apply.


**J.7.2** **Specification of syntax functions, categories, and descriptors**


The specifications in clause I.7.2 apply.


**J.7.3** **Syntax in tabular form**


**J.7.3.1** **NAL unit syntax**


The syntax table is specified in clause I.7.3.1.


**J.7.3.1.1** **NAL unit header 3D-AVC extension syntax**

|nal_unit_header_3davc_extension( ) {|C|Descriptor|
|---|---|---|
|<br>**view_idx**|All|u(8)|
|<br>**depth_flag**|All|u(1)|
|<br>**non_idr_flag**|All|u(1)|
|<br>**temporal_id**|All|u(3)|
|<br>**anchor_pic_flag**|All|u(1)|
|<br>**inter_view_flag**|All|u(1)|
|}|||






**J.7.3.2** **Raw byte sequence payloads and RBSP trailing bits syntax**


**J.7.3.2.1** **Sequence parameter set RBSP syntax**


The syntax table is specified in clause I.7.3.2.1.


**J.7.3.2.1.1** **Sequence parameter set data syntax**


The syntax table is specified in clause I.7.3.2.1.1.


**J.7.3.2.1.1.1** **Scaling list syntax**


The syntax table is specified in clause I.7.3.2.1.1.1.


**J.7.3.2.1.2** **Sequence parameter set extension RBSP syntax**


The syntax table is specified in clause I.7.3.2.1.2.


**J.7.3.2.1.3** **Subset sequence parameter set RBSP syntax**


The syntax table is specified in clause I.7.3.2.1.3.


**J.7.3.2.1.4** **Sequence parameter set MVC extension syntax**


The syntax table is specified in clause I.7.3.2.1.4.


**J.7.3.2.1.5** **Sequence parameter set 3D-AVC extension syntax**

|seq_parameter_set_3davc_extension( ) {|C|Descriptor|
|---|---|---|
|<br>if( NumDepthViews > 0 ) {|||
|<br> <br>**3dv_acquisition_idc**|0|ue(v)|
|<br> <br>for( i = 0; i < NumDepthViews; i++ )|||
|<br> <br> <br>**view_id_3dv**[ i ]|0|ue(v)|
|<br> <br>if( 3dv_acquisition_idc ) {|||
|<br> <br> <br>depth_ranges( NumDepthViews, 2, 0 )|||
|<br> <br> <br>vsp_param( NumDepthViews, 2, 0 )|||
|<br> <br>}|||
|<br> <br>**reduced_resolution_flag**|0|u(1)|
|<br> <br>if( reduced_resolution_flag ) {|||
|<br> <br> <br>**depth_pic_width_in_mbs_minus1**|0|ue(v)|
|<br> <br> <br>**depth_pic_height_in_map_units_minus1**|0|ue(v)|
|<br> <br> <br>**depth_hor_mult_minus1**|0|ue(v)|
|<br> <br> <br>**depth_ver_mult_minus1**|0|ue(v)|
|<br> <br> <br>**depth_hor_rsh**|0|ue(v)|
|<br> <br> <br>**depth_ver_rsh**|0|ue(v)|
|<br> <br>}|||
|<br> <br>**depth_frame_cropping_flag**|0|u(1)|
|<br> <br>if( depth_frame_cropping_flag ) {|||
|<br> <br> <br>**depth_frame_crop_left_offset**|0|ue(v)|
|<br> <br> <br>**depth_frame_crop_right_offset**|0|ue(v)|
|<br> <br> <br>**depth_frame_crop_top_offset**|0|ue(v)|
|<br> <br> <br>**depth_frame_crop_bottom_offset**|0|ue(v)|
|<br> <br>}|||
|<br> <br>**grid_pos_num_views**|0|ue(v)|
|<br> <br>for( i = 0; i < grid_pos_num_views; i++ ) {|||
|<br> <br> <br>**grid_pos_view_id**[ i ]|0|ue(v)|
|<br> <br> <br>**grid_pos_x**[ grid_pos_view_id[ i ] ]|0|se(v)|
|<br> <br> <br>**grid_pos_y**[ grid_pos_view_id[ i ] ]|0|se(v)|
|<br> <br>}|||
|<br> <br>**slice_header_prediction_flag**|0|u(1)|






|seq_view_synthesis_flag|0|u(1)|
|---|---|---|
|<br>}|||
|<br>**alc_sps_enable_flag**|0|u(1)|
|<br>**enable_rle_skip_flag**|0|u(1)|
|<br>if( !AllViewsPairedFlag ) {|||
|<br> <br>for( i = 1; i <= num_views_minus1; i++ )|||
|<br> <br> <br>if( texture_view_present_flag[ i ] ) {|||
|<br> <br> <br> <br>**num_anchor_refs_l0**[ i ]|0|ue(v)|
|<br> <br> <br> <br>for( j = 0; j < num_anchor_refs_l0[ i ]; j++ )|||
|<br> <br> <br> <br> <br>**anchor_ref_l0**[ i ][ j ]|0|ue(v)|
|<br> <br> <br> <br>**num_anchor_refs_l1**[ i ]|0|ue(v)|
|<br> <br> <br> <br>for( j = 0; j < num_anchor_refs_l1[ i ]; j++ )|||
|<br> <br> <br> <br> <br>**anchor_ref_l1**[ i ][ j ]|0|ue(v)|
|<br> <br> <br>}|||
|<br> <br>for( i = 1; i <= num_views_minus1; i++ )|||
|<br> <br> <br>if( texture_view_present_flag[ i ] ) {|||
|<br> <br> <br> <br>**num_non_anchor_refs_l0**[ i ]|0|ue(v)|
|<br> <br> <br> <br>for( j = 0; j < num_non_anchor_refs_l0[ i ]; j++ )|||
|<br> <br> <br> <br> <br>**non_anchor_ref_l0**[ i ][ j ]|0|ue(v)|
|<br> <br> <br> <br>**num_non_anchor_refs_l1**[ i ]|0|ue(v)|
|<br> <br> <br> <br>for( j = 0; j < num_non_anchor_refs_l1[ i ]; j++ )|||
|<br> <br> <br> <br> <br>**non_anchor_ref_l1**[ i ][ j ]|0|ue(v)|
|<br> <br> <br>}|||
|<br>}|||
|}|||


**J.7.3.2.2** **Picture parameter set RBSP syntax**


The syntax table is specified in clause I.7.3.2.2.


**J.7.3.2.3** **Supplemental enhancement information RBSP syntax**


The syntax table is specified in clause I.7.3.2.3.


**J.7.3.2.3.1** **Supplemental enhancement information message syntax**


The syntax table is specified in clause I.7.3.2.3.1.


**J.7.3.2.4** **Access unit delimiter RBSP syntax**


The syntax table is specified in clause I.7.3.2.4.


**J.7.3.2.5** **End of sequence RBSP syntax**


The syntax table is specified in clause I.7.3.2.5.


**J.7.3.2.6** **End of stream RBSP syntax**


The syntax table is specified in clause I.7.3.2.6.


**J.7.3.2.7** **Filler data RBSP syntax**


The syntax table is specified in clause I.7.3.2.7.


**J.7.3.2.8** **Slice layer without partitioning RBSP syntax**


The syntax table is specified in clause I.7.3.2.8.


**J.7.3.2.9** **Slice data partition RBSP syntax**


Slice data partition syntax is not present in coded video sequences conforming to one or more of the profiles specified in
this annex.





**J.7.3.2.10** **RBSP slice trailing bits syntax**


The syntax table is specified in clause I.7.3.2.10.


**J.7.3.2.11** **RBSP trailing bits syntax**


The syntax table is specified in clause I.7.3.2.11.


**J.7.3.2.12** **Prefix NAL unit RBSP syntax**


The syntax table is specified in clause I.7.3.2.12.


**J.7.3.2.13** **Depth parameter set RBSP syntax**

|depth_parameter_set_rbsp( ) {|C|Descriptor|
|---|---|---|
|<br>**depth_parameter_set_id**|11|ue(v)|
|<br>**pred_direction**|11|ue(v)|
|<br>if( pred_direction = = 0 | | pred_direction = = 1 ) {|||
|<br> <br>**ref_dps_id0**|11|ue(v)|
|<br> <br>predWeight0 = 64|||
|<br>}|||
|<br>if( pred_direction = = 0 ) {|||
|<br> <br>**ref_dps_id1**|11|ue(v)|
|<br> <br>**pred_weight0**|11|u(6)|
|<br> <br>predWeight0 = pred_weight0|||
|<br>}|||
|<br>**num_depth_views_minus1**|11|ue(v)|
|<br>depth_ranges( num_depth_views_minus1 + 1, pred_direction,<br> <br> <br>depth_parameter_set_id )|||
|<br>**vsp_param_flag**|11|u(1)|
|<br>if( vsp_param_flag )|||
|<br> <br>vsp_param( num_depth_views_minus1 + 1, pred_direction,<br> <br> <br>depth_parameter_set_id )|||
|<br>**depth_param_additional_extension_flag**|11|u(1)|
|<br>**nonlinear_depth_representation_num**|11|ue(v)|
|<br>for( i = 1; i <= nonlinear_depth_representation_num; i++ )|||
|<br> <br>**nonlinear_depth_representation_model**[ i ]|11|ue(v)|
|<br>if(depth_param_additional_extension_flag = = 1 )|||
|<br> <br>while( more_rbsp_data( ) )|||
|<br> <br> <br>**depth_param_additional_extension_data_flag**|11|u(1)|
|<br>rbsp_trailing_bits( )|||
|}|||






**J.7.3.2.13.1** **Depth ranges syntax**

|depth_ranges( numViews, predDirection, index ) {|C|Descriptor|
|---|---|---|
|<br>**z_near_flag**|11|u(1)|
|<br>**z_far_flag**|11|u(1)|
|<br>if( z_near_flag )|||
|<br> <br>3dv_acquisition_element( numViews, 0, predDirection, 7, index,<br> <br> <br> <br>ZNearSign, ZNearExp, ZNearMantissa, ZNearManLen )|||
|<br>if( z_far_flag )|||
|<br> <br>3dv_acquisition_element( numViews, 0, predDirection, 7, index,<br> <br> <br> <br>ZFarSign, ZFarExp, ZFarMantissa, ZFarManLen )|||
|}|||



**J.7.3.2.13.2** **3DV acquisition element syntax**







|3dv_acquisition_element( numViews, predDirection, expLen, index, outSign,<br>outExp, outMantissa, outManLen ) {|C|Descriptor|
|---|---|---|
|<br>if( numViews > 1 )|||
|<br> <br>**element_equal_flag**|11|u(1)|
|<br>if( element_equal_flag = = 0 )|||
|<br> <br>numValues = numViews|||
|<br>else|||
|<br> <br>numValues = 1|||
|<br>for( i = 0; i < numValues; i++ ) {|||
|<br> <br>if( predDirection = = 2 && i = = 0 ) {|||
|<br> <br> <br>**mantissa_len_minus1**|11|u(5)|
|<br> <br> <br>outManLen[ index, i ] = manLen = mantissa_len_minus1 + 1|||
|<br> <br>}|||
|<br> <br>if( predDirection = = 2 ) {|||
|<br> <br> <br>**sign0**|11|u(1)|
|<br> <br> <br>outSign[ index, i ] = sign0|||
|<br> <br> <br>**exponent0**|11|u(v)|
|<br> <br> <br>outExp[ index, i ] = exponent0|||
|<br> <br> <br>**mantissa0**|11|u(v)|
|<br> <br> <br>outMantissa[ index, i ] = mantissa0|||
|<br> <br>} else {|||
|<br> <br> <br>**skip_flag**|11|u(1)|
|<br> <br> <br>if( skip_flag = = 0 ) {|||
|<br> <br> <br> <br>**sign1**|11|u(1)|
|<br> <br> <br> <br>outSign[ index, i ] = sign1|||
|<br> <br> <br> <br>**exponent_skip_flag**|11|u(1)|
|<br> <br> <br> <br>if( exponent_skip_flag = = 0 ) {|||
|<br> <br> <br> <br> <br>**exponent1**|11|u(v)|
|<br> <br> <br> <br> <br>outExp[ index, i ] = exponent1|||
|<br> <br> <br> <br>} else|||
|<br> <br> <br> <br> <br>outExp[ index, i ] = outExp[ ref_dps_id0, i ]|||
|<br> <br> <br> <br>**mantissa_diff**|11|se(v)|
|<br> <br> <br> <br>if( predDirection = = 0 )|||
|<br> <br> <br> <br> <br>mantissaPred = (( OutMantissa[ ref_dps_id0, i ] * predWeight0 +<br> <br> <br> <br> <br> <br> <br>outMantissa[ ref_dps_id1, i ] * ( 64-predWeight0 ) + 32 ) >> 6 )|||





|else|Col2|Col3|
|---|---|---|
|<br> <br> <br> <br> <br>mantissaPred = outMantissa[ ref_dps_id0, i ]|||
|<br> <br> <br> <br>outMantissa[ index, i ] = mantissaPred + mantissa_diff|||
|<br> <br> <br> <br>outManLen[ index, i ] = outManLen[ ref_dps_id0, i ]|||
|<br> <br> <br>} else {|||
|<br> <br> <br> <br>outSign[ index, i ] = outSign[ ref_dps_id0, i ]|||
|<br> <br> <br> <br>outExp[ index, i ] = outExp[ ref_dps_id0, i ]|||
|<br> <br> <br> <br>outMantissa[ index, i ] = outMantissa[ ref_dps_id0, i ]|||
|<br> <br> <br> <br>outManLen[ index, i ] = outManLen[ ref_dps_id0, i ]|||
|<br> <br> <br>}|||
|<br> <br>}|||
|<br>}|||
|<br>if( element_equal_flag = = 1 ) {|||
|<br> <br>for( i = 1; i < num_views_minus1 + 1 − deltaFlag; i++ ) {|||
|<br> <br> <br>outSign[ index, i ] = outSign[ index, 0 ]|||
|<br> <br> <br>outExp[ index, i ] = outExp[ index, 0 ]|||
|<br> <br> <br>outMantissa[ index, i ] = outMantissa[ index, 0 ]|||
|<br> <br> <br>outManLen[ index, i ] = outManLen[ index, 0 ]|||
|<br> <br>}|||
|<br>}|||
|}|||


**J.7.3.2.13.3** **View synthesis prediction parameters syntax**

|vsp_param( numViews, predDirection, index ) {|C|Descriptor|
|---|---|---|
|<br>for( i = 0; i < numViews; i++ )|||
|<br> <br>for( j = 0; j < i; j++ ) {|||
|<br> <br> <br>**disparity_diff_wji**[ j ][ i ]|0|ue(v)|
|<br> <br> <br>**disparity_diff_oji**[ j ][ i ]|0|ue(v)|
|<br> <br> <br>**disparity_diff_wij**[ i ][ j ]|0|ue(v)|
|<br> <br> <br>**disparity_diff_oij**[ i ][ j ]|0|ue(v)|
|<br> <br>}|||
|}|||



**J.7.3.2.14** **Slice layer extension RBSP syntax**


The syntax table is specified in clause I.7.3.2.13.


**J.7.3.3** **Slice header syntax**


The syntax table is specified in clause I.7.3.3.


**J.7.3.3.1** **Reference picture list modification syntax**


The syntax table is specified in clause I.7.3.3.1.


**J.7.3.3.1.1** **Reference picture list MVC modification syntax**


The syntax table is specified in clause I.7.3.3.1.1.


**J.7.3.3.2** **Prediction weight table syntax**


The syntax table is specified in clause I.7.3.3.2.


**J.7.3.3.3** **Decoded reference picture marking syntax**


The syntax table is specified in clause I.7.3.3.3.





**J.7.3.3.4** **Slice header in 3D-AVC extension syntax**

|slice_header_in_3davc_extension( ) {|C|Descriptor|
|---|---|---|
|<br>**first_mb_in_slice**|2|ue(v)|
|<br>**slice_type** <br>|2|ue(v)|
|<br>**pic_parameter_set_id**|2|ue(v)|
|<br>if( avc_3d_extension_flag && slice_header_prediction_flag != 0 ) {|||
|<br> <br>**pre_slice_header_src**|2|u(2)|
|<br> <br>if( slice_type = = P | | slice_type = = SP<br> <br> <br> <br>| | slice_type = = B ) {|||
|<br> <br> <br>**pre_ref_lists_src**|2|u(2)|
|<br> <br> <br>if( !pre_ref_lists_src ) {|||
|<br> <br> <br> <br>**num_ref_idx_active_override_flag**|2|u(1)|
|<br> <br> <br> <br>if( num_ref_idx_active_override_flag ) {|||
|<br> <br> <br> <br>**num_ref_idx_l0_active_minus1**|2|ue(v)|
|<br> <br> <br> <br>if( slice_type = = B )|||
|<br> <br> <br> <br> <br>**num_ref_idx_l1_active_minus1**|2|ue(v)|
|<br> <br> <br> <br>}|||
|<br> <br> <br> <br>ref_pic_list_mvc_modification( ) /* specified in Annex H */|2||
|<br> <br> <br>}|||
|<br> <br>}|||
|<br> <br>if( ( weighted_pred_flag && ( slice_type = = P | |<br> <br> <br> <br>slice_type = = SP ) ) | |<br> <br> <br> <br>( weighted_bipred_idc = = 1 && slice_type = = B ) ) {|||
|<br> <br> <br>**pre_pred_weight_table_src**|2|u(2)|
|<br> <br> <br>if( !pre_pred_weight_table_src )|||
|<br> <br> <br> <br>pred_weight_table( )|2||
|<br> <br>if( nal_ref_idc != 0 ) {|||
|<br> <br> <br>**pre_dec_ref_pic_marking_src**|2|u(2)|
|<br> <br> <br>if( !pre_dec_ref_pic_marking_src )|||
|<br> <br> <br> <br>dec_ref_pic_marking( )|2||
|<br> <br>}|||
|<br> <br>**slice_qp_delta**|2|se(v)|
|<br>} else {|||
|<br> <br>if( separate_colour_plane_flag = = 1 )|||
|<br> <br> <br>**colour_plane_id**|2|u(2)|
|<br> <br>**frame_num**|2|u(v)|
|<br> <br>if( !frame_mbs_only_flag ) {<br>|||
|<br> <br> <br>**field_pic_flag**|2|u(1)|
|<br> <br> <br>if( field_pic_flag )|||
|<br> <br> <br> <br>**bottom_field_flag**|2|u(1)|
|<br> <br>}|||
|<br> <br>if( IdrPicFlag )|||
|<br> <br> <br>**idr_pic_id**|2|ue(v)|
|<br> <br>if( pic_order_cnt_type = = 0 ) {|||
|<br> <br> <br>**pic_order_cnt_lsb**|2|u(v)|
|<br> <br> <br>if( bottom_field_pic_order_in_frame_present_flag && !field_pic_flag )|||
|<br> <br> <br> <br>**delta_pic_order_cnt_bottom**|2|se(v)|
|<br> <br>}|||






|if( pic_order_cnt_type = = 1 && !delta_pic_order_always_zero_flag ) {|Col2|Col3|
|---|---|---|
|<br> <br> <br>**delta_pic_order_cnt[** 0**] **|2|se(v)|
|<br> <br> <br>if( bottom_field_pic_order_in_frame_present_flag &&<br> <br> <br> <br> <br>!field_pic_flag )|||
|<br> <br> <br> <br>**delta_pic_order_cnt[** 1**] **|2|se(v)|
|<br> <br>}|||
|<br> <br>if( redundant_pic_cnt_present_flag )|||
|<br> <br> <br>**redundant_pic_cnt**|2|ue(v)|
|<br> <br>if( slice_type = = B )|||
|<br> <br> <br>**direct_spatial_mv_pred_flag**|2|u(1)|
|<br> <br>if( slice_type = = P | | slice_type = = SP | | slice_type = = B ) {|||
|<br> <br> <br>**num_ref_idx_active_override_flag**|2|u(1)|
|<br> <br> <br>if( num_ref_idx_active_override_flag ) {|||
|<br> <br> <br> <br>**num_ref_idx_l0_active_minus1**|2|ue(v)|
|<br> <br> <br> <br>if( slice_type = = B )|||
|<br> <br> <br> <br> <br>**num_ref_idx_l1_active_minus1**|2|ue(v)|
|<br> <br> <br>}|||
|<br> <br>}|||
|<br> <br>if( nal_unit_type = = 20 | | nal_unit_type = = 21 )|||
|<br> <br> <br>ref_pic_list_mvc_modification( ) /* specified in Annex H */|2||
|<br> <br>else|||
|<br> <br> <br>ref_pic_list_modification( )|2||
|<br> <br>if( ( weighted_pred_flag && ( slice_type = = P | |<br> <br> <br> <br> <br>slice_type = = SP ) ) | |<br> <br> <br> <br>( weighted_bipred_idc = = 1 && slice_type = = B ) )|||
|<br> <br> <br>pred_weight_table( )|2||
|<br> <br>if( nal_ref_idc != 0 )|||
|<br> <br> <br>dec_ref_pic_marking( )|2||
|<br> <br>if( entropy_coding_mode_flag && slice_type != I &&<br> <br> <br> <br>slice_type != SI )|||
|<br> <br> <br>**cabac_init_idc**|2|ue(v)|
|<br> <br>**slice_qp_delta**|2|se(v)|
|<br> <br>if( slice_type = = SP | | slice_type = = SI ) {|||
|<br> <br> <br>if( slice_type = = SP )|||
|<br> <br> <br> <br>**sp_for_switch_flag**|2|u(1)|
|<br> <br> <br>**slice_qs_delta**|2|se(v)|
|<br> <br>}|||
|<br> <br>if( deblocking_filter_control_present_flag ) {|||
|<br> <br> <br>**disable_deblocking_filter_idc**|2|ue(v)|
|<br> <br> <br>if( disable_deblocking_filter_idc != 1 ) {|||
|<br> <br> <br> <br>**slice_alpha_c0_offset_div2**|2|se(v)|
|<br> <br> <br> <br>**slice_beta_offset_div2**|2|se(v)|
|<br> <br> <br>}|||
|<br> <br>}|||
|<br> <br>if( num_slice_groups_minus1 > 0 &&<br> <br> <br> <br>slice_group_map_type >= 3 && slice_group_map_type <= 5)|||
|<br> <br> <br>**slice_group_change_cycle**|2|u(v)|
|<br> <br>if( nal_unit_type = = 21 && ( slice_type != I &&<br> <br> <br> <br>slice_type != SI ) ) {|||
|<br> <br> <br>if( DepthFlag )|||
|<br> <br> <br>**depth_weighted_pred_flag**|2|u(1)|





|else if( avc_3d_extension_flag ) {|Col2|Col3|
|---|---|---|
|<br> <br> <br> <br>**dmvp_flag**|2|u(1)|
|<br> <br> <br> <br>if( seq_view_synthesis_flag )|||
|<br> <br> <br> <br> <br>**slice_vsp_flag**|2|u(1)|
|<br> <br> <br>}|||
|<br> <br> <br>if( 3dv_acquisition_idc != 1 &&<br> <br> <br> <br> <br>( depth_weighted_pred_flag | | dmvp_flag ) )|||
|<br> <br> <br> <br>**dps_id**|2|ue(v)|
|<br> <br>}|||
|<br>}|||
|}|||


**J.7.3.4** **Slice data syntax**


The syntax table is specified in clause I.7.3.4.


**J.7.3.4.1** **Slice data in 3D-AVC extension syntax**

|slice_data_in_3davc_extension( ) {|C|Descriptor|
|---|---|---|
|<br>if( entropy_coding_mode_flag )|||
|<br> <br>while( !byte_aligned( ) )|||
|<br> <br> <br>**cabac_alignment_one_bit**|2|f(1)|
|<br>CurrMbAddr = first_mb_in_slice * ( 1 + MbaffFrameFlag )|||
|<br>moreDataFlag = 1|||
|<br>prevMbSkipped = 0|||
|<br>RunLength = 0<br>|||
|<br>do {|||
|<br> <br>if( slice_type != I && slice_type != SI )|||
|<br> <br> <br>if( !entropy_coding_mode_flag ) {|||
|<br> <br> <br> <br>**mb_skip_run**|2|ue(v)|
|<br> <br> <br> <br>prevMbSkipped = ( mb_skip_run > 0 )|||
|<br> <br> <br> <br>for( i=0; i<mb_skip_run; i++ )|||
|<br> <br> <br> <br> <br>CurrMbAddr = NextMbAddress( CurrMbAddr )|||
|<br> <br> <br> <br>if( nal_unit_type = = 21 && !DepthFlag &&<br> <br> <br> <br> <br> <br>mb_skip_run > 0 && VspRefExist )|||
|<br> <br> <br> <br> <br>**mb_skip_type_flag**|2|u(1)|
|<br> <br> <br> <br>if( mb_skip_run > 0 )|||
|<br> <br> <br> <br> <br>moreDataFlag = more_rbsp_data( )|||
|<br> <br> <br>} else {|||
|<br> <br> <br> <br>if( nal_unit_type = = 21 && !DepthFlag &&<br> <br> <br> <br> <br> <br>VspRefExist && leftMbVSSkipped &&<br> <br> <br> <br> <br> <br>upMbVSSkipped ) {|||
|<br> <br> <br> <br> <br>**mb_vsskip_flag**|2|ae(v)|
|<br> <br> <br> <br> <br>moreDataFlag = !mb_vsskip_flag|||
|<br> <br> <br> <br> <br>if( !mb_vsskip_flag ) {|||
|<br> <br> <br> <br> <br> <br>**mb_skip_flag**|2|ae(v)|
|<br> <br> <br> <br> <br> <br>moreDataFlag = !mb_skip_flag|||
|<br> <br> <br> <br> <br>}|||
|<br> <br> <br> <br> <br>RunLength = 0|||






|} else {|Col2|Col3|
|---|---|---|
|<br> <br> <br> <br> <br>rleCtx = RLESkipContext( )|||
|<br> <br> <br> <br> <br>if( rleCtx && !RunLength ) {|||
|<br> <br> <br> <br> <br> <br>**mb_skip_run_type**|2|ae(v)|
|<br> <br> <br> <br> <br> <br>RunLength = 16|||
|<br> <br> <br> <br> <br>} else if( !rleCtx && RunLength )|||
|<br> <br> <br> <br> <br> <br>RunLength = 0|||
|<br> <br> <br> <br> <br>if( rleCtx && mb_skip_run_type )|||
|<br> <br> <br> <br> <br> <br>RunLength -= 1|||
|<br> <br> <br> <br> <br>else|||
|<br> <br> <br> <br> <br>**mb_skip_flag**|2|ae(v)|
|<br> <br> <br> <br> if( rleCtx && !mb_skip_flag )|||
|<br> <br> <br> <br> <br>RunLength = 0|||
|<br> <br> <br> <br> moreDataFlag = !mb_skip_flag|||
|<br> <br> <br> <br> <br>if( nal_unit_type = = 21 && !DepthFlag &&<br> <br> <br> <br> <br> <br> <br>VspRefExist && !mb_skip_flag ) {|||
|<br> <br> <br> <br> <br> <br>**mb_vsskip_flag**|2|ae(v)|
|<br> <br> <br> <br> <br> <br>moreDataFlag = !mb_vsskip_flag|||
|<br> <br> <br> <br> <br>}|||
|<br> <br> <br> <br>}|||
|<br> <br> <br> <br>if(alc_sps_enable_flag && nal_unit_type = = 21<br> <br> <br> <br> <br> <br>&& slice_type = = P && !DepthFlag &&<br> <br> <br> <br> <br> <br>!mb_vsskip_flag && mb_skip_flag = = 1 )|||
|<br> <br> <br> <br> <br>**mb_alc_skip_flag**|2|ae(v)|
|<br> <br> <br>}|||
|<br> <br>if( moreDataFlag ) {|||
|<br> <br> <br>if( MbaffFrameFlag && ( CurrMbAddr % 2 = = 0 | |<br> <br> <br> <br> <br>( CurrMbAddr % 2 = = 1 && prevMbSkipped ) ) )|||
|<br> <br> <br> <br>**mb_field_decoding_flag**|2|u(1) | ae(v)|
|<br> <br> <br>macroblock_layer_in_3davc_extension( )|2 | 3 | 4||
|<br> <br>}|||
|<br> <br>if( !entropy_coding_mode_flag )|||
|<br> <br> <br>moreDataFlag = more_rbsp_data( )|||
|<br> <br>else {|||
|<br> <br> <br>if( slice_type != I && slice_type != SI )|||
|<br> <br> <br> <br>prevMbSkipped = mb_skip_flag | | mb_vsskip_flag|||
|<br> <br> <br>if( MbaffFrameFlag && CurrMbAddr % 2 = = 0 )|||
|<br> <br> <br> <br>moreDataFlag = 1|||
|<br> <br> <br>else {|||
|<br> <br> <br> <br>**end_of_slice_flag**|2|ae(v)|
|<br> <br> <br> <br>moreDataFlag = !end_of_slice_flag|||
|<br> <br> <br>}|||
|<br> <br>}|||
|<br> <br>CurrMbAddr = NextMbAddress( CurrMbAddr )|||
|<br>} while( moreDataFlag )|||
|}|||


**J.7.3.5** **Macroblock layer syntax**


The syntax table is specified in clause I.7.3.5.





**J.7.3.5.1** **Macroblock prediction syntax**


The syntax table is specified in clause I.7.3.5.1.


**J.7.3.5.2** **Sub-macroblock prediction syntax**


The syntax table is specified in clause I.7.3.5.2.


**J.7.3.5.3** **Residual data syntax**


The syntax table is specified in clause I.7.3.5.3.


**J.7.3.5.3.1** **Residual luma syntax**


The syntax table is specified in clause I.7.3.5.3.1.


**J.7.3.5.3.2** **Residual block CAVLC syntax**


The syntax table is specified in clause I.7.3.5.3.2.


**J.7.3.5.3.3** **Residual block CABAC syntax**


The syntax table is specified in clause I.7.3.5.3.3.


**J.7.3.6** **Macroblock layer in 3D-AVC extension syntax**

|macroblock_layer_in_3davc_extension( ) {|C|Descriptor|
|---|---|---|
|<br>**mb_type**|2|ue(v) | ae(v)|
|<br>if( nal_unit_type = = 21 && !DepthFlag<br> <br> <br>&& slice_type = = B<br> <br> <br>&& direct_spatial_mv_pred_flag && VspRefExist<br> <br> <br>&& mb_type = = B_Direct_16x16 )|||
|<br> <br>**mb_direct_type_flag**|2|u(1) | ae(v)|
|<br>if( alc_sps_enable_flag && nal_unit_type = = 21 &&<br> <br> <br>slice_type = = P && !DepthFlag &&<br> <br> <br>( mb_type = = P_L0_16x16 | |<br> <br> <br> <br>mb_type = = P_L0_L0_16x8 | |<br> <br> <br> <br>mb_type = = P_L0_L0_8x16 ) )|||
|<br> <br>**mb_alc_flag**|2|u(1) | ae(v)|
|<br>if( mb_type = = I_PCM ) {|||
|<br> <br>while( !byte_aligned( ) )|||
|<br> <br> <br>**pcm_alignment_zero_bit**|3|f(1)|
|<br> <br>for( i = 0; i < 256; i++ )|||
|<br> <br> <br>**pcm_sample_luma[** i**]**|3|u(v)|
|<br> <br>for( i = 0; i < 2 * MbWidthC * MbHeightC; i++ )|||
|<br> <br> <br>**pcm_sample_chroma[** i**]**|3|u(v)|
|<br>} else {|||
|<br> <br>noSubMbPartSizeLessThan8x8Flag = 1|||
|<br> <br>if( mb_type != I_NxN &&<br> <br> <br> <br>MbPartPredMode( mb_type, 0 ) != Intra_16x16 &&<br> <br> <br> <br>NumMbPart( mb_type ) = = 4 ) {|||
|<br> <br> <br>sub_mb_pred_in_3davc_extension( mb_type )|2||
|<br> <br> <br>for( mbPartIdx = 0; mbPartIdx < 4; mbPartIdx++ )|||
|<br> <br> <br> <br>if( sub_mb_type[ mbPartIdx ] != B_Direct_8x8 ) {|||
|<br> <br> <br> <br> <br>if( NumSubMbPart( sub_mb_type[ mbPartIdx ] ) > 1 )|||
|<br> <br> <br> <br> <br> <br>noSubMbPartSizeLessThan8x8Flag = 0|||
|<br> <br> <br> <br>} else if( !direct_8x8_inference_flag )|||
|<br> <br> <br> <br> <br>noSubMbPartSizeLessThan8x8Flag = 0|||
|<br> <br>} else {|||
|<br> <br> <br>if( transform_8x8_mode_flag && mb_type = = I_NxN )|||
|<br> <br> <br> <br>**transform_size_8x8_flag**|2|u(1) | ae(v)|






|mb_pred_in_3davc_extension( mb_type )|2|Col3|
|---|---|---|
|<br> <br>}|||
|<br> <br>if( MbPartPredMode( mb_type, 0 ) != Intra_16x16 ) {|||
|<br> <br> <br>**coded_block_pattern**|2|me(v) | ae(v)|
|<br> <br> <br>if( ( CodedBlockPatternLuma > 0 | | mb_alc_flag = = 1 ) &&<br> <br> <br> <br> <br> transform_8x8_mode_flag && mb_type != I_NxN &&<br> <br> <br> <br> <br> noSubMbPartSizeLessThan8x8Flag &&<br> <br> <br> <br> <br> ( mb_type != B_Direct_16x16 | |<br> <br> <br> <br> <br>  direct_8x8_inference_flag ) )|||
|<br> <br> <br> <br>**transform_size_8x8_flag**|2|u(1) | ae(v)|
|<br> <br>}|||
|<br> <br>if( CodedBlockPatternLuma > 0 | |<br> <br> <br> <br>CodedBlockPatternChroma > 0 | |<br> <br> <br> <br>MbPartPredMode( mb_type, 0 ) = = Intra_16x16 ) {|||
|<br> <br> <br>**mb_qp_delta**|2|se(v) | ae(v)|
|<br> <br> <br>residual( 0, 15 )|3 | 4||
|<br> <br>}|||
|<br>}|||
|}|||


**J.7.3.6.1** **Macroblock prediction in 3D-AVC extension syntax**

|mb_pred_in_3davc_extension( mb_type ) {|C|Descriptor|
|---|---|---|
|<br>if( MbPartPredMode( mb_type, 0 ) = = Intra_4x4 | |<br> <br> <br>MbPartPredMode( mb_type, 0 ) = = Intra_8x8 | |<br> <br> <br>MbPartPredMode( mb_type, 0 ) = = Intra_16x16 ) {|||
|<br> <br>if( MbPartPredMode( mb_type, 0 ) = = Intra_4x4 )|||
|<br> <br> <br>for( luma4x4BlkIdx=0; luma4x4BlkIdx<16; luma4x4BlkIdx++ ) {|||
|<br> <br> <br> <br>**prev_intra4x4_pred_mode_flag[** luma4x4BlkIdx**]**|2|u(1) | ae(v)|
|<br> <br> <br> <br>if( !prev_intra4x4_pred_mode_flag**[** luma4x4BlkIdx**]** )|||
|<br> <br> <br> <br> <br>**rem_intra4x4_pred_mode[** luma4x4BlkIdx**]**|2|u(3) | ae(v)|
|<br> <br> <br>}|||
|<br> <br>if( MbPartPredMode( mb_type, 0 ) = = Intra_8x8 )|||
|<br> <br> <br>for( luma8x8BlkIdx=0; luma8x8BlkIdx<4; luma8x8BlkIdx++ ) {|||
|<br> <br> <br> <br>**prev_intra8x8_pred_mode_flag[** luma8x8BlkIdx**]**|2|u(1) | ae(v)|
|<br> <br> <br> <br>if( !prev_intra8x8_pred_mode_flag[ luma8x8BlkIdx ] )|||
|<br> <br> <br> <br> <br>**rem_intra8x8_pred_mode[** luma8x8BlkIdx**]**|2|u(3) | ae(v)|
|<br> <br> <br>}|||
|<br> <br>if( ChromaArrayType = = 1 | | ChromaArrayType = = 2 )|||
|<br> <br> <br>**intra_chroma_pred_mode**|2|ue(v) | ae(v)|
|<br>} else if( MbPartPredMode( mb_type, 0 ) != Direct ) {|||
|<br> <br>for( mbPartIdx = 0; mbPartIdx < NumMbPart( mb_type ); mbPartIdx++ )|||
|<br> <br> <br>if( ( num_ref_idx_l0_active_minus1 > 0 | |<br> <br> <br> <br> <br> <br>mb_field_decoding_flag != field_pic_flag ) &&<br> <br> <br> <br> <br>MbPartPredMode( mb_type, mbPartIdx ) != Pred_L1 &&<br> <br> <br> <br> <br>mb_alc_flag = = 0 ) {|||
|<br> <br> <br> <br>**ref_idx_l0[** mbPartIdx**]**|2|te(v) | ae(v)|
|<br> <br> <br> <br>if( VspRefL0Flag[ mbPartIdx ] && slice_vsp_flag )|||
|<br> <br> <br> <br> <br>**bvsp_flag_l0**[ mbPartIdx ]|2|u(1) | ae(v)|
|<br> <br> <br>}|||
|<br> <br>for( mbPartIdx = 0; mbPartIdx < NumMbPart( mb_type ); mbPartIdx++ )|||






|if( ( num_ref_idx_l1_active_minus1 > 0 | |<br>mb_field_decoding_flag != field_pic_flag ) &&<br>MbPartPredMode( mb_type, mbPartIdx ) != Pred_L0 ) {|Col2|Col3|
|---|---|---|
|<br> <br> <br> <br>**ref_idx_l1[** mbPartIdx**]**|2|te(v) | ae(v)|
|<br> <br> <br> <br>if( VspRefL1Flag[ mbPartIdx ] && slice_vsp_flag )|||
|<br> <br> <br> <br> <br>**bvsp_flag_l1**[ mbPartIdx ]|2|u(1) | ae(v)|
|<br> <br> <br>}|||
|<br> <br>for( mbPartIdx = 0; mbPartIdx < NumMbPart( mb_type ); mbPartIdx++ )|||
|<br> <br> <br>if( MbPartPredMode ( mb_type, mbPartIdx ) != Pred_L1 &&<br> <br> <br> <br> <br>( !VspRefL0Flag[ mbPartIdx ] | | !bvsp_flag_l0[ mbPartIdx ] ) )|||
|<br> <br> <br> <br>for( compIdx = 0; compIdx < 2; compIdx++ )|||
|<br> <br> <br> <br> <br>**mvd_l0[** mbPartIdx**][** 0**][** compIdx**]**|2|se(v) | ae(v)|
|<br> <br>for( mbPartIdx = 0; mbPartIdx < NumMbPart( mb_type ); mbPartIdx++ )|||
|<br> <br> <br>if( MbPartPredMode( mb_type, mbPartIdx ) != Pred_L0 &&<br> <br> <br> <br> <br>( !VspRefL1Flag[ mbPartIdx ] | | !bvsp_flag_l1[ mbPartIdx ] ) )|||
|<br> <br> <br> <br>for( compIdx = 0; compIdx < 2; compIdx++ )|||
|<br> <br> <br> <br> <br>**mvd_l1[** mbPartIdx**][** 0**][** compIdx**]**|2|se(v) | ae(v)|
|<br>}|||
|}|||


**J.7.3.6.2** **Sub-macroblock prediction syntax**

|sub_mb_pred_in_3davc_extension( mb_type ) {|C|Descriptor|
|---|---|---|
|<br>for( mbPartIdx = 0; mbPartIdx < 4; mbPartIdx++ )|||
|<br> <br>**sub_mb_type[** mbPartIdx**]**|2|ue(v) | ae(v)|
|<br>for( mbPartIdx = 0; mbPartIdx < 4; mbPartIdx++ )|||
|<br> <br>if( ( num_ref_idx_l0_active_minus1 > 0 | |<br> <br> <br> <br> <br>mb_field_decoding_flag != field_pic_flag ) &&<br> <br> <br> <br>mb_type != P_8x8ref0 &&<br> <br> <br> <br>sub_mb_type[ mbPartIdx ] != B_Direct_8x8 &&<br> <br> <br> <br>SubMbPredMode( sub_mb_type[ mbPartIdx ] ) != Pred_L1 &&<br> <br> <br> <br>mb_alc_flag = = 0 ) {|||
|<br> <br> <br>**ref_idx_l0[** mbPartIdx**]**|2|te(v) | ae(v)|
|<br> <br> <br>if( VspRefL0Flag[ mbPartIdx ] && slice_vsp_flag )|||
|<br> <br> <br> <br>**bvsp_flag_l0**[ mbPartIdx ]|2|u(1) | ae(v)|
|<br> <br>}|||
|<br>for( mbPartIdx = 0; mbPartIdx < 4; mbPartIdx++ )|||
|<br> <br>if( ( num_ref_idx_l1_active_minus1 > 0 | |<br> <br> <br> <br> <br>mb_field_decoding_flag != field_pic_flag ) &&<br> <br> <br> <br> sub_mb_type[ mbPartIdx ] != B_Direct_8x8 &&<br> <br> <br> <br> SubMbPredMode( sub_mb_type[ mbPartIdx ] ) != Pred_L0 ) {|||
|<br> <br> <br>**ref_idx_l1[** mbPartIdx**]**|2|te(v) | ae(v)|
|<br> <br> <br>if( VspRefL1Flag[ mbPartIdx ] && slice_vsp_flag )|||
|<br> <br> <br> <br>**bvsp_flag_l1**[ mbPartIdx ]|2|u(1) | ae(v)|
|<br> <br>}|||
|<br>for( mbPartIdx = 0; mbPartIdx < 4; mbPartIdx++ )|||
|<br> <br>if( sub_mb_type[ mbPartIdx ] != B_Direct_8x8 &&<br> <br> <br> <br>SubMbPredMode( sub_mb_type[ mbPartIdx ] ) != Pred_L1 &&<br> <br> <br> <br>( !VspRefL0Flag[ mbPartIdx ] | | !bvsp_flag_l0[ mbPartIdx ] ) )|||






|for( subMbPartIdx = 0;<br>subMbPartIdx < NumSubMbPart( sub_mb_type[ mbPartIdx ] );<br>subMbPartIdx++ )|Col2|Col3|
|---|---|---|
|<br> <br> <br> <br>for( compIdx = 0; compIdx < 2; compIdx++ )|||
|<br> <br> <br> <br> <br>**mvd_l0[** mbPartIdx**][** subMbPartIdx**][** compIdx**]**|2|se(v) | ae(v)|
|<br>for( mbPartIdx = 0; mbPartIdx < 4; mbPartIdx++ )|||
|<br> <br>if( sub_mb_type[ mbPartIdx ] != B_Direct_8x8 &&<br> <br> <br> <br>SubMbPredMode( sub_mb_type[ mbPartIdx ] ) != Pred_L0 &&<br> <br> <br> <br>( !VspRefL1Flag[ mbPartIdx ] | | !bvsp_flag_l1[ mbPartIdx ] ) )|||
|<br> <br> <br>for( subMbPartIdx = 0;<br> <br> <br> <br> <br>  subMbPartIdx < NumSubMbPart( sub_mb_type[ mbPartIdx ] );<br> <br> <br> <br> <br>  subMbPartIdx++ )|||
|<br> <br> <br> <br>for( compIdx = 0; compIdx < 2; compIdx++ )|||
|<br> <br> <br> <br> <br>**mvd_l1[** mbPartIdx**][** subMbPartIdx**][** compIdx**]**|2|se(v) | ae(v)|
|}|||


**J.7.4** **Semantics**


Semantics associated with the syntax structures and syntax elements within these structures (in clause J.7.3 and in
clause I.7.3 by reference in clause J.7.3) are specified in this clause and by reference to clause I.7.4. When the semantics
of a syntax element are specified using a table or a set of tables, any values that are not specified in the table(s) shall not
be present in the bitstream unless otherwise specified in this Recommendation | International Standard.


**J.7.4.1** **NAL unit semantics**


The semantics for the syntax elements clause J.7.3.1 are specified in clause I.7.3.1.


**J.7.4.1.1** **NAL unit header MVC extension semantics**


**view_idx** specifies the view oder index for the NAL unit.


view_id is inferred to be equal to view_id[ view_idx ], where view_id[ ] is present in the active sequence parameter set.


The variable VOIdx, representing the view order index of the view identified by view_id[ i ], is set equal to view_idx.


**depth_flag** equal to 1 indicates that the current NAL unit belongs to a depth view component, depth_flag equal to 0
indicates that the current NAL unit belongs to a texture view component.


**non_idr_flag**, **temporal_id**, **anchor_pic_flag**, and **inter_view_flag** have the same semantics as those syntax elements
with the same names in Annex H.


**J.7.4.1.2** **Order of NAL units and association to coded pictures, access units, and video sequences**


The specification of clause I.7.4.1.2 applies.


**J.7.4.1.2.1** **Order of 3D-AVC sequence parameter set RBSPs and picture parameter set RBSPs and their**
**activation**


The specification of clause I.7.4.1.2.1 applies.


In addition, the following applies for the activation of depth parameter set.


A depth parameter set includes parameters that can be referred to by the coded slice NAL units of one or more texture view
or depth view components of one or more coded pictures. A depth parameter set associated with depth_parameter_set_id
equal to 0 contains the depth ranges syntax structure and the view synthesis prediction parameter syntax structure included
in the active sequence parameter set. A depth parameter set with depth_parameter_set_id greater than 0 is a depth parameter
set RBSP with that depth_parameter_set_id value.


Each depth parameter set is initially considered not active at the start of the operation of the decoding process. At most one
depth parameter set is considered as the active depth parameter set at any given moment during the operation of the
decoding process, and when any particular depth parameter set becomes the active depth parameter set, the previouslyactive depth parameter set (if any) is deactivated.


When a depth parameter set (with a particular value of depth_parameter_set_id) is not the active depth parameter set and
it is referred to by a coded slice NAL unit (when dps_id is present in a slice header), it is activated. This depth parameter
set is called the active depth parameter set until it is deactivated when another depth parameter set becomes the active
depth parameter set. A depth parameter set, with that particular value of depth_parameter_set_id, shall be available to the





decoding process prior to its activation. When a depth parameter set is activated, the same depth parameter set shall remain
active for subsequent coded slice NAL units of the same access unit.


If any depth parameter set is present that is never activated in the bitstream (i.e., it never becomes the active depth parameter
set), its syntax elements shall have values that would conform to the specified constraints if it were activated by reference
in an otherwise-conforming bitstream.


**J.7.4.1.2.2** **Order of access units and association to coded video sequences**


The specification of clause I.7.4.1.2.2 apply.


**J.7.4.1.2.3** **Order of NAL units and coded pictures and association to access units**


The specification of clause H.7.4.1.2.3 applies with the following modifications.


The association of VCL NAL units to primary or redundant coded pictures is specified in clause I.7.4.1.2.5.


**J.7.4.1.2.4** **Detection of the first VCL NAL unit of a primary coded picture**


The specification of clause H.7.4.1.2.4 applies.


**J.7.4.1.2.5** **Order of VCL NAL units and association to coded pictures**


Each VCL NAL unit is part of a coded picture.


Let voIdx be the value of VOIdx of any particular VCL NAL unit. The order of the VCL NAL units within a coded picture
is constrained as follows:

- For all VCL NAL units following this particular VCL NAL unit, the value of VOIdx shall be greater than or equal
to voIdx.

- All VCL NAL units for a depth view component, if present, shall follow any VCL NAL unit of an MVC texture
view component with a same value of VOIdx.


For each set of VCL NAL units within a texture or depth view component, the following applies:


- If arbitrary slice order, as specified in Annex A, clause H.10, clause I.10 or clause J.10, is allowed, coded slice NAL
units of a view component may have any order relative to each other.


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


**J.7.4.2** **Raw byte sequence payloads and RBSP trailing bits semantics**


**J.7.4.2.1** **Sequence parameter set RBSP semantics**


The semantics specified in clause I.7.4.2.1 apply.





**J.7.4.2.1.1** **Sequence parameter set data semantics**


The semantics specified in clause I.7.4.2.1.1 apply.


**J.7.4.2.1.1.1** **Scaling list semantics**


The semantics specified in clause I.7.4.2.1.1.1 apply.


**J.7.4.2.1.2** **Sequence parameter set extension RBSP semantics**


The semantics specified in clause I.7.4.2.1.2 apply.


**J.7.4.2.1.3** **Subset sequence parameter set RBSP semantics**


The semantics specified in clause I.7.4.2.1.3 apply.


**J.7.4.2.1.4** **Sequence parameter set MVCD extension semantics**


The semantics specified in clause I.7.4.2.1.4 apply with the substitution of texture view component or depth view
component for view component.


**J.7.4.2.1.5** **Sequence parameter set 3D-AVC extension semantics**


The function ViewCompOrder( depthFlag, viewId ) is specified to return the value of viewCompOrder derived as follows:


i = 0
while ( i <= num_views_minus1 && ( view_id[ i ] != viewId | |
( depthFlag && !depth_view_present_flag[ i ] ) | |
( !depthFlag && !texture_view_present_flag[ i ] ) ) (J-7)
i++
if( i > num_views_minus1 )
viewCompOrder = MAX_INT
else
viewCompOrder = 2 * i + depthFlag


**3dv_acquisition_idc** equal to 0 indicates that no depth ranges or view synthesis prediction parameters syntax structures
are present in the sequence parameter set. 3dv_acquisition_idc equal to 1 indicates that depth ranges and view synthesis
prediction parameters syntax structures are present in the sequence parameter set and valid for the entire coded video
sequence. 3dv_acquisition_idc equal to 2 indicates that depth ranges and view synthesis prediction parameters syntax
structures are present in the sequence parameter set and depth parameter sets with depth_parameter_set_id greater than 0
may be activated. 3dv_acquisition_idc values greater than 2 are reserved.


The function ViewIdTo3DVAcquisitionParamIndex( viewId ) is specified to return the value of i for which
view_id_3dv[ i ] is equal to viewId in the active sequence parameter set.


**reduced_resolution_flag** equal to 1 specifies that the depth view components of a view component pair have a lower
spatial resolution than the luma component of the texture view component of the same view component pair, and the width
and height (as represented by pic_width_in_mbs_minus1+1 and pic_height_in_map_units_minus1+1 in the referred subset
sequence parameter set) of the depth view components are both half of the width and height of all the texture view
components. reduced_resolution_flag equal to 0 specifies that when both depth view components and texture view
components are present, they have the same spatial resolution.


**depth_pic_width_in_mbs_minus1** and **depth_pic_height_in_map_units_minus1**, when present, are used to infer the
width and height of depth view components. When reduced_resolution_flag is equal to 1, the values of
pic_width_in_mbs_minus1 and pic_height_in_map_units_minus1 are inferred to be equal to
depth_pic_width_in_mbs_minus1 and depth_pic_height_in_map_units_minus1 for depth views for which this 3D-AVC
sequence parameter set is an active view 3D-AVC sequence parameter set.


**depth_hor_mult_minus1**, **depth_ver_mult_minus1**, **depth_hor_rsh** and **depth_ver_rsh** are used for specifying the
depth-based disparity value derivation process (specified in clause J.8.2.1.1). When not present, depth_hor_mult_minus1
and depth_ver_mult_minus1 are inferred to be equal to 1, and depth_hor_rsh and depth_ver_rsh are inferred to be equal to
0. depth_hor_mult_minus1 and depth_ver_mult_minus1 shall be in the range of 0 to 1023, inclusive. depth_hor_rsh and
depth_ver_rsh shall be in the range of 0 to 31, inclusive.


**depth_frame_cropping_flag** equal to 0 specifies that the frame cropping offset parameters for depth view components
follow next in the sequence parameter set. depth_frame_cropping_flag equal to 0 specifies that the frame cropping offset
parameters for depth view components are not present.


**depth_frame_crop_left_offset**, **depth_frame_crop_right_offset**, **depth_frame_crop_top_offset** and
**depth_frame_crop_bottom_offset** specify the samples of the decoded depth view components in the coded video
sequence that are output from the decoding process, in terms of a rectangular region specified in frame coordinates for





output.


When depth_frame_cropping_flag is equal to 0, the values of depth_frame_crop_left_offset,
depth_frame_crop_right_offset, depth_frame_crop_top_offset, and depth_frame_crop_bottom_offset are inferred to be
equal to 0.


The values of frame_crop_left_offset, frame_crop_right_offset, frame_crop_top_offset, frame_crop_bottom_offset are
inferred to be equal to depth_frame_crop_left_offset, depth_frame_crop_right_offset, depth_frame_crop_top_offset, and
depth_frame_crop_bottom_offset for the decoding and output of depth views for which this 3D-AVC sequence parameter
set is an active view 3D-AVC sequence parameter set.


Let the variables DepthCropLeftCoord, DepthCropRightCoord, DepthCropTopCoord and DepthCropBottomCoord be
derived from the values of PicWidthInSamplesL, CropUnitX, CropUnitY, FrameHeightInMBs that apply to depth view
components as follows:


DepthCropLeftCoord = CropUnitX * depth_frame_crop_left_offset
DepthCropRightCoord = PicWidthInSamplesL − ( CropUnitX * depth_frame_crop_right_offset + 1 ) (J-8)
DepthCropTopCoord = CropUnitY * depth_frame_crop_top_offset
DepthCropBottomCoord = ( 16 * FrameHeightInMbs ) − ( CropUnitY * depth_frame_crop_bottom_offset + 1 )


**grid_pos_num_views** specifies the number of views for which grid_pos_view_id[ i ], grid_pos_x[ grid_pos_view_id[ i ] ]
and grid_pos_y[ grid_pos_view_id[ i ] ] are present. grid_pos_num_views shall be in the range of 0 to 1024, inclusive.


**grid_pos_view_id** [ i ] specifies a view_id value of a texture view.


**grid_pos_x** [ grid_pos_view_id[ i ] ] specifies a horizontal offset of a depth sampling grid relative to the luma texture
sampling grid in texture luma sample units.


**grid_pos_y** [ grid_pos_view_id[ i ] ] specifies a vertical offset of a depth sampling grid relative to the luma texture
sampling grid in texture luma sample units.


When no value of grid_pos_view_id[ i ] is equal to a view_id value of a texture view, grid_pos_x[ view_id ] and
grid_pos_y[ view_id ] are inferred to be equal to 0.


grid_pos_x[ grid_pos_view_id[ i ] ] and grid_pos_y[ grid_pos_view_id[ i ] ] are used for specifying the depth-based
disparity value derivation process (specified in clause J.8.2.1.1).


**slice_header_prediction_flag** equal to 0 indicates that slice header prediction from texture view component to depth view
component or vice versa is disallowed. slice_header_prediction_flag equal to 1 indicates that the prediction is used.


**seq_view_synthesis_flag** equal to 1 indicates view synthesis prediction is enabled. seq_view_synthesis_flag equal to 0
indicates that view synthesis prediction is disabled for all view components referring the current sequence parameter set.


**alc_sps_enable_flag** equal to 0 specifies that mb_alc_skip_flag and mb_alc_flag are not present. alc_sps_enable_flag
equal to 1 specifies that specifies that mb_alc_skip_flag and mb_alc_flag may be present.


**enable_rle_skip_flag** equal to 0 specifies that mb_skip_run_type are not present. enable_rle_skip_flag equal to 1 specifies
that mb_skip_run_type may be present. When enable_rle_skip_flag is not present, it is inferred to be equal to 0.


The variable AllViewsPairedFlag is derived as follows:


AllViewsPairedFlag = 1
for( i = 1; i <= num_views_minus1; i++ )
AllViewsPairedFlag = ( AllViewsPairedFlag && depth_view_present_flag[ i ] && (J-9)
texture_view_present_flag[ i ] )


For num_anchor_refs_l0[ i ], anchor_ref_l0[ i ][ j ], num_anchor_refs_l1[ i ], anchor_ref_l1[ i ][ j ],
num_non_anchor_refs_l0[ i ], non_anchor_ref_l0[ i ][ j ], num_non_anchor_refs_l1[ i ], and non_anchor_ref_l1[ i ][ j ],
the semantics specified in clause H.7.4.2.1.4 is applied with the substitution of texture view component for view
component. When num_anchor_refs_l0[ i ], anchor_ref_l0[ i ][ j ], num_anchor_refs_l1[ i ], anchor_ref_l1[ i ][ j ],
num_non_anchor_refs_l0[ i ], non_anchor_ref_l0[ i ][ j ], num_non_anchor_refs_l1[ i ], and non_anchor_ref_l1[ i ][ j ]
are not present, they are inferred to have the same values as the respective syntax elements in the
seq_parameter_set_mvcd_extension( ) syntax structure in the same subset_seq_parameter_set_rbsp( ) syntax structure that
also contains this seq_parameter_set_3davc_extension( ) syntax structure.


**J.7.4.2.2** **Picture parameter set RBSP semantics**


The semantics specified in in clause I.7.4.2.2 apply.





**J.7.4.2.3** **Supplemental enhancement information RBSP semantics**


The semantics specified in clause I.7.4.2.3 apply.


**J.7.4.2.3.1** **Supplemental enhancement information message semantics**


The semantics specified in clause I.7.4.2.3.1 apply.


**J.7.4.2.4** **Access unit delimiter RBSP semantics**


The semantics specified in clause I.7.4.2.4 apply.


**J.7.4.2.5** **End of sequence RBSP semantics**


The semantics specified in clause I.7.4.2.5 apply.


**J.7.4.2.6** **End of stream RBSP semantics**


The semantics specified in clause I.7.4.2.6 apply.


**J.7.4.2.7** **Filler data RBSP semantics**


The semantics specified in clause I.7.4.2.7 apply.


**J.7.4.2.8** **Slice layer without partitioning RBSP semantics**


The semantics specified in clause I.7.4.2.8 apply.


**J.7.4.2.9** **Slice data partition RBSP semantics**


Slice data partition syntax is not present in bitstreams conforming to one or more of the profiles specified in Annex J.


**J.7.4.2.10** **RBSP slice trailing bits semantics**


The semantics specified in clause I.7.4.2.10 apply.


**J.7.4.2.11** **RBSP trailing bits semantics**


The semantics specified in clause I.7.4.2.11 apply.


**J.7.4.2.12** **Prefix NAL unit RBSP semantics**


The semantics specified in clause I.7.4.2.12 apply.


**J.7.4.2.13** **Depth parameter set RBSP semantics**


**depth_parameter_set_id** identifies the depth parameter set that is referred to in the slice header. The value of
depth_parameter_set_id shall be in the range of 1 to 63, inclusive.


**pred_direction** equal to 0 specifies that the closest and farthest depth values for the base view may be predicted from the
respective variables of two pictures. pred_direction equal to 1 specifies that the closest and farthest depth values for the
base view may be predicted from the respective variables of one picture. pred_direction equal to 2 specifies that the closest
and farthest depth values for the base view are not predicted. The value of pred_direction shall be in the range of 0 to 2,
inclusive.


**ref_dps_id0** specifies a first reference depth parameter set to be used in prediction of the closest and farthest depth value.
The value of ref_dps_id0 shall be in the range of 0 to 63, inclusive.


**ref_dps_id1** specifies a second reference depth parameter set to be used in prediction of the closest and farthest depth
value. The value of ref_dps_id1 shall be in the range of 0 to 63, inclusive.


**pred_weight0** specifies a weight associated with the a first reference depth parameter set derived from ref_dps_id0. The
value of pred_weight0 shall be in the range of 0 to 63, inclusive.


**num_views_minus1** plus 1 specifies the number of views for which depth parameters are specified in the included
3dv_acquisition_element( ) structure. num_view_minus1 shall be in the range of 0 to 1023, inclusive.


**vsp_param_flag** equal to 1 indicates the presence of the vsp_param( ) syntax structure. vsp_param_flag equal to 0
indicates the absence of the vsp_param( ) syntax structure.


**nonlinear_depth_representation_num** plus 1 specifies the number of piecewise linear segments for mapping of depth
values to a scale that is uniformly quantized in terms of disparity.


**nonlinear_depth_representation_model** [ i ] specifies the piecewise linear segments for mapping of depth values to a
scale that is uniformly quantized in terms of disparity.





NOTE – When nonlinear_depth_representation_num is equal to 0, a depth sample represents a disparity normalized to the range of
0 to 255, inclusive, so that 0 corresponds to ZFar value and 255 corresponds to ZNear value. Depending on the value of
nonlinear_depth_representation_num, a depth view component is composed of either depth samples that can be converted to
disparity using a linear equation or non-linearly transformed depth samples. If nonlinear_depth_representation_num is equal to 0,
depth view component contains directly depth samples that are uniformly quantized in terms of disparity, i.e. depth samples that can
be transformed to disparity using a linear equation. If nonlinear_depth_representation_num is greater than 0, depth view component
contains non-linearly transformed depth samples.


When nonlinear_depth_representation_num is greater than 0, NdrInverse[ i ], as specified below, is used to transform depth sample
values from non-linear representation to the linear representation. The shape of this transform is defined by means of line-segmentapproximation in two-dimensional linear-disparity-to-non-linear-disparity space. The first (0, 0) and the last (255, 255) nodes of the
curve are predefined. Positions of additional nodes are transmitted in form of deviations (nonlinear_depth_representation_model[ i ])
from the straight-line curve. These deviations are uniformly distributed along the whole range of 0 to 255, inclusive, with spacing
depending on the value of nonlinear_depth_representation_num.


If nonlinear_depth_representation_num is equal to 0, variable NdrInverse[ i ] for i in the range of 0 to 255, inclusive, is
specified as follows:


nonlinear_depth_representation_model[ 0 ] = 0
nonlinear_depth_representation_model[ nonlinear_depth_representation_num + 1 ] = 0
for( k = 0; k <= nonlinear_depth_representation_num; ++k )
{
pos1 = ( 255 * k ) / ( nonlinear_depth_representation_num + 1 )
pos2 = ( 255 * ( k+1 ) ) / ( nonlinear_depth_representation_num + 1 ) )

x1 = pos1 (J-10)
y1 = pos1
x2 = pos2
y2 = pos2

for ( x = Max( x1, 0 ); x <= Min( x2, 255 ); ++x )
NdrInverse[ x ] = Clip3( 0, 255, Round( ( ( x - x1 ) * ( y2 - y1 ) ) ÷ ( x2 - x1 ) + y1 ) )
}


Otherwise (nonlinear_depth_representation_num is greater than 0), variable NdrInverse[ i ] for i in the range of 0 to 255,
inclusive, is specified as follows:


nonlinear_depth_representation_model[ 0 ] = 0
nonlinear_depth_representation_model[ nonlinear_depth_representation_num + 1 ] = 0
for( k = 0; k <= nonlinear_depth_representation_num; ++k ) {
pos1 = ( 255 * k ) / ( nonlinear_depth_representation_num + 1 )
dev1 = nonlinear_depth_representation_model[ k ]
pos2 = ( 255 * ( k+1 ) ) / ( nonlinear_depth_representation_num + 1 ) )
dev2 = nonlinear_depth_representation_model[ k+1 ]

x1 = pos1 - dev1 (J-11)
y1 = pos1 + dev1
x2 = pos2 - dev2
y2 = pos2 + dev2

for ( x = Max( x1, 0 ); x <= Min( x2, 255 ); ++x )
NdrInverse[ x ] = Clip3( 0, 255, Round( ( ( x - x1 ) * ( y2 - y1 ) ) ÷ ( x2 - x1 ) + y1 ) )
}


**depth_param_additional_extension_flag** equal to 0 indicates that no additional data follows within the depth parameter
set RBSP prior to the RBSP trailing bits. The value of depth_param_additional_extension_flag shall be equal to 0. The
value of 1 for depth_param_additional_extension_flag is reserved for future use by ITU-T | ISO/IEC. Decoders shall ignore
all data that follows the value of 1 for depth_param_additional_extension_flag in a depth parameter set RBSP.


**J.7.4.2.13.1** **Depth ranges semantics**


The contents of the syntax structure are controlled through input variables predDirection and index the semantics of which
are as follows:


- predDirection equal to 2 specifies that the first loop entry of the element is not predicted and coded in the sign,
exponent, and mantissa syntax elements. predDirection equal to 0 or 1 specifies that the first loop entry of the element
is predicted and a difference relative to a prediction value is coded in the difference syntax element.





- index may be equal to the depth_parameter_set_id of the depth parameter set wherein the parameters are present.


**z_near_flag** equal to 0 specifies that the syntax elements specifying the closest depth value are not present in the syntax
structure. z_near_flag equal to 1 specifies that the syntax elements specifying the closest depth value are present in the
syntax structure.


**z_far_flag** equal to 0 specifies that the syntax elements specifying the farthest depth value are not present in the syntax
structure. z_near_flag equal to 1 specifies that the syntax elements specifying the farthest depth value are present in the
syntax structure.


**J.7.4.2.13.2** **3DV acquisition element semantics**


The syntax structure specifies the value of an element in the depth ranges syntax structure. The element may contain one
or more loop entries i of the order specified by view_id_3dv syntax elements.


The contents of the syntax structure are controlled through input variables predDirection, expLen and index, the semantics
of which are as follows:


- predDirection equal to 2 specifies that the first loop entry of the element is not predicted and coded in the sign,
exponent, and mantissa syntax elements. predDirection equal to 0 or 1 specifies that the first loop entry of the element
is predicted and a difference relative to a prediction value is coded in the difference syntax element.


- expLen specifies the number of bits in the exponent syntax element.


- index greater than 0 specifies the depth_parameter_set_id of the depth parameter set wherein the parameters are
present, and index equal to 0 specifies that the parameters are present in a sequence parameter set.


The syntax structure uses outSign, outExp, outMantissa and outManLen variables for both input and output, where each
variable is indexed by [ index, viewIdc ], index being an identifier (equal to either 0 when decoding depth ranges in
sequence parameter set or depth_parameter_set_id value when decoding depth range parameter set) to a depth parameter
set and viewIdc being a view indicator (in the order of views for 3DV acquisition parameters).


**element_equal_flag** equal to 0 specifies that the sign, exponent, and mantissa may or may not be identical to respective
values for any two loop entries i and j. element_equal_flag equal to 1 specifies that the sign, exponent, and mantissa are
identical to respective values for any two loop entries i and j. When not present, element_equal_flag is inferred to be equal
to 0.


**mantissa_len_minus1** plus 1 specifies the number of bits in the mantissa syntax element. The value of
mantissa_len_minus1 shall be in the range of 0 to 31, inclusive.


**sign0** equal to 0 indicates that the sign of the value provided in the loop entry is positive. sign0 equal to 1 indicates that
the sign is negative.


**exponent0** specifies the exponent of the value provided by the loop entry. The syntax element exponent0 is represented
by expLen bits. The value of exponent0 shall be in the range of 0 to 2 [expLen] - 2, inclusive. The value 2 [expLen] - 1 is reserved
for future use by ITU-T | ISO/IEC. Decoders shall treat the value 2 [expLen] - 1 as indicating an unspecified value.


**mantissa0** specifies the mantissa of the value provided by the loop entry. The syntax element mantissa0 is represented by
manLen bits.


**skip_flag** equal to 0 specifies that syntax elements sign1, exponent_skip_flag and mantissa_diff are present for the loop
entry. skip_flag equal to 1 specifies that elements sign1, exponent_skip_flag and mantissa_diff are not present for the loop
entry.


**sign1** equal to 0 indicates that the sign of the value provided in the loop entry is positive. sign1 equal to 1 indicates that
the sign is negative.


**exponent1**, if present, specifies the exponent of the value provided by the loop entry. The syntax element exponent1 is
represented by expLen bits. The value of exponent1 shall be in the range of 0 to 2 [expLen] - 2, inclusive. The value 2 [expLen] 1 is reserved for future use by ITU-T | ISO/IEC. Decoders shall treat the value 2 [expLen] - 1 as indicating an unspecified
value.


**mantissa_diff** specifies the difference of the mantissa of the value provided by the loop entry relative to its prediction
value.


**J.7.4.2.13.3** **View synthesis prediction parameters semantics**


The contents of the syntax structure are controlled through input variables predDirection and index the semantics of which
are as follows:





- predDirection equal to 2 specifies that the first loop entry of the element is not predicted and coded in the sign,
exponent, and mantissa syntax elements. predDirection equal to 0 or 1 specifies that the first loop entry of the element
is predicted and a difference relative to a prediction value is coded in the difference syntax element.


- index identifies a depth parameter set.


**disparity_diff_wji** [ j ][ i ], **disparity_diff_oji** [ j ][ i ], **disparity_diff_wij** [ i ][ j ] and **disparity_diff_oij** [ i ][ j ] specify
the variables DisparityScale and DisparityOffset as follows:


if( predDirection = = 2 ) {
DisparityScale[ index ][ j ][ i ] = disparity_diff_wji[ j ][ i ]
DisparityOffset[ index ][ j ][ i ] = disparity_diff_oji[ j ][ i ]
DisparityScale[ index ][ i ][ j ] = disparity_diff_wij[ i ][ j ] − disparity_diff_wji[ j ][ i ]
DisparityOffset[ index ][ i ][ j ] = disparity_diff_oij[ i ][ j ] − disparity_diff_oji[ j ][ i ]
} else { (J-12)
DisparityScale[ index ][ j ][ i ] = disparity_diff_wji[ j ][ i ] + ( DisparityScale[ ref_dps_id0 ][ j ][ i ] *
predWeight0 + DisparityScale[ ref_dps_id1 ][ j ][ i ] * ( 64 − predWeight0 ) + 32 ) >> 6
DisparityOffset[ index ][ j ][ i ] = disparity_diff_oji[ j ][ i ] + ( DisparityOffset[ ref_dps_id0 ][ j ][ i ] *
predWeight0 + DisparityOffset[ ref_dps_id1][ j ][ i ] * ( 64 − predWeight0 ) + 32 ) >> 6
DisparityScale[ index ][ i ][ j ] = disparity_diff_wij[ i ][ j ] + ( DisparityScale[ ref_dps_id0 ] i ][ j ] *
predWeight0 + DisparityScale[ ref_dps_id1 ][ i ][ j ] * ( 64 − predWeight0 ) + 32 ) >> 6
DisparityOffset[ index ][ i ][ j ] = disparity_diff_oij[ i ][ j ] + ( DisparityOffset[ ref_dps_id0 ][ i ][ j ] *
predWeight0 + DisparityOffset[ ref_dps_id1 ][ i ][ j ] * ( 64 − predWeight0 ) + 32 ) >> 6
}


**J.7.4.2.14** **Slice layer extension RBSP semantics**


The semantics specified in clause I.7.4.2.13 apply.


**J.7.4.3** **Slice header semantics**


The semantics specified in clause I.7.4.3 apply.


**J.7.4.3.1** **Reference picture list modification semantics**


The semantics specified in clause I.7.4.3.1 apply.


**J.7.4.3.1.1** **Reference picture list MVC modification semantics**


The semantics specified in clause I.7.4.3.1.1 apply **.**


**J.7.4.3.2** **Prediction weight table semantics**


The semantics specified in clause I.7.4.3.2 apply.


**J.7.4.3.3** **Decoded reference picture marking semantics**


The semantics specified in clause I.7.4.3.3 apply to each view independently, with "sequence parameter set" being replaced
by "3D-AVC sequence parameter set", and "primary coded picture" being replaced by "texture view component" for
nal_unit_type equal to 1, 5, and 20 as well as nal_unit_type 21 when DepthFlag is equal to 1, and by "depth view
component" for nal_unit_type equal to 21 when DepthFlag is equal to 0.


**J.7.4.3.4** **Slice header in 3D-AVC semantics**


The semantics specified in clause H.7.4.3 apply with the substitution of texture view component or depth view component
for view component and with the following modifications.


When nal_unit_type is equal to 1, 5, 20, or 21 with DepthFlag equal to 0, all constraints specified in clause H.7.4.3 apply
only to the texture view components with the same value of VOIdx. When nal_unit_type is equal to 21 and DepthFlag is
equal to 1, all constraints specified in clause H.7.4.3 apply only to the depth view components with the same value of
VOIdx.


The value of the following 3D-AVC sequence parameter set syntax elements shall be the same across all coded slice NAL
units of nal_unit_type 1, 5, 20 and 21 with DepthFlag equal to 0 of an access unit: chroma_format_idc.


The value of the following slice header syntax elements shall be the same across all coded slice NAL units of nal_unit_type
1, 5, 20 and 21 with DepthFlag equal to 0 of an access unit: field_pic_flag and bottom_field_flag.


The value of the following slice header syntax elements shall be the same across all coded slice NAL units of nal_unit_type
equal to 21 and DepthFlag equal to 1 of an access unit: field_pic_flag and bottom_field_flag.





**pre_slice_header_src**, **pre_ref_lists_src**, **pre_pred_weight_table_src** and **pre_dec_ref_pic_marking_src** specify if the
respective syntax elements are present in the slice header, and, if not, the slice header from which the values of the
respective syntax elements are taken as specified in Table J-1 and Table J-2.


When a syntax element has an inferred value in the slice header from which its value is taken according to Table J-1 and
Table J-2, the syntax element value in the current slice header is equal to this inferred value. When a syntax element is not
present and has no inferred value in the slice header from which its value is taken according to Table J-1 and Table J-2,
the syntax element is inferred to be absent in the current slice header.


pre_slice_header_src shall not be equal to 0.


When ViewCompOrder( DepthFlag, view_id ) is smaller than ViewCompOrder( !DepthFlag, view_id ),
pre_slice_header_src, pre_ref_lists_src, pre_pred_weight_table_src and pre_dec_ref_pic_marking_src shall not be equal
to 2.


**Table J-1 – Respective syntax elements for pre_slice_header_src, pre_ref_lists_src, pre_pred_weight_table_src**

**and pre_dec_ref_pic_marking_src**







|Prediction indication syntax<br>element|Respective syntax elements|
|---|---|
|pre_slice_header_src|colour_plane_id,<br>frame_num,<br>field_pic_flag,<br>bottom_field_flag,<br>idr_pic_id,<br>pic_order_cnt_lsb,<br>delta_pic_order_cnt_bottom,<br>delta_pic_order_cnt[ 0 ],<br>delta_pic_order_cnt[ 1 ], redundant_pic_cnt, direct_spatial_mv_pred_flag, cabac_init_idc,<br>sp_for_switch_flag,<br>slice_qs_delta,<br>disable_deblocking_filter_idc,<br>slice_alpha_c0_offset_div2,<br>slice_beta_offset_div2,<br>slice_group_change_cycle,<br>depth_weighted_pred_flag, dmvp_flag, slice_vsp_flag, dps_id|
|pre_ref_lists_src|num_ref_idx_active_override_flag,<br>num_ref_idx_l0_active_minus1,<br>num_ref_idx_l1_active_minus1 and reference picture list modification syntax table|
|pre_pred_weight_table_src|pred_weight_table( ) syntax structure|
|pre_dec_ref_pic_marking_src|dec_ref_pic_marking( ) syntax structure|


**Table J-2 – Semantics of the values of pre_slice_header_src, pre_ref_lists_src, pre_pred_weight_table_src and**

**pre_dec_ref_pic_marking_src**







|Value of pre_slice_header_src,<br>pre_ref_lists_src,<br>pre_pred_weight_table_src or<br>pre_dec_ref_pic_marking_src|Semantics|
|---|---|
|0|The respective syntax elements are not predicted but included in the slice header.|
|1|The values of the respective syntax elements are taken from the slice header of the first slice of<br>the previous view component in decoding order having the same value of DepthFlag as the<br>current slice, belonging to a dependent view of the current view, and residing in the same access<br>unit.|
|2|The values of the respective syntax elements are taken from the first slice header of the first<br>slice of the view component having the same view_id as the current slice and a different value<br>of DepthFlag.|
|3|The values of the respective syntax elements are taken from the first slice header of the first<br>slice of the view component in the same access unit having view order index equal to 0 and the<br>same value of DepthFlag as the current slice.|


**depth_weighted_pred_flag** equal to 0 specifies that no depth-range-based weighted prediction is used for corresponding
slice RBSP. depth_weighted_pred_flag equal to 1 specifies that depth-range-based weighted prediction is used for
corresponding slice RBSP. When not present, depth_weighted_pred_flag is inferred to be equal to 0. When
depth_weighted_pred_flag is equal to 1, the process of derivation of prediction weights specified in clause J.8.2.2 applies.


**dmvp_flag** is used in the decoding process for inter prediction, inter-view prediction, view synthesis prediction and
adaptive luminance compensation as specified in clause J.8.2.





**slice_vsp_flag** together with bvsp_flag[ mbPartIdx ] specify, when ref_idx_lX[ mbPartIdx ] (with X equal to 0 or 1) refers
to an inter-view reference picture, which motion vector derivation process specified in clause J.8.2.1 is in use.


**dps_id** specifies the depth parameter set in use. The value of dps_id shall be in the range of 0 to 63, inclusive. When dps_id
is equal to 0, depth parameters are set according to syntax elements in seq_parameter_set_3davc_extension( ) of the active
sequence parameter set. When present, the value of dps_id shall be the same in all slice headers within an access unit.


**J.7.4.4** **Slice data semantics**


The semantics specified in clause I.7.4.4 apply.


**J.7.4.4.1** **Slice data in 3D-AVC extension semantics**


The semantics specified in clause I.7.4.4 apply with the following additions.


When mb_skip_flag is not present, it is inferred to be equal to 0.


**mb_skip_type_flag** is used to derive the variable MbVSSkipFlag. When mb_skip_type_flag is present, the variable
MbVSSkipFlag is set equal to mb_skip_type_flag.


**mb_vsskip_flag** is used to derive the variable MbVSSkipFlag. When mb_vsskip_flag is present, the variable
MbVSSkipFlag is set equal to mb_vsskip_flag.


NOTE – MbVSSkipFlag controls whether clause J.8.2.1.2 or J.8.2.1.3 is used for deriving motion vectors for P_Skip macroblocks
and whether clause J.8.2.1.4 or J.8.2.1.6 is used for deriving motion vectors and reference indices for B_Skip macroblocks.


leftMbVSSkipped is derived to be 1 if the left macroblock adjacent to the current macroblock is available and the
MbVSSkipFlag of the left macroblock is equal to 1, leftMbVSSkipped is derived to be 0 otherwise.


upMbVSSkipped is derived to be 1 if the upper macroblock adjacent to the current macroblock is available and the
MbVSSkipFlag of the upper macrolbock is equal to 1, upMbVSSkipped is derived to be 0 otherwise.


The function RLESkipContext( ) is specified as follows:


- The derivation process for neighbouring macroblocks specified in clause 6.4.11.1 is invoked and the output is assigned
to mbAddrA and mbAddrB;


- If all of the following conditions are true, the return value of RLESkipContext( ) is equal to TRUE.


   - enable_rle_skip_flag is equal to 1


   - nal_unit_type is equal to 21


   - slice_type is equal to B


   - mbAddrA is unavailable or mb_skip_flag for the macroblock mbAddrA is equal to 1


   - mbAddrB is unavailable or mb_skip_flag for the macroblock mbAddrB is equal to 1


- Otherwise, the return value of RLESkipContext( ) is equal to FALSE.


**mb_skip_run_type** equal to 0 specifies that mb_skip_run_type is not present in the slice_data( ) until mb_skip_flag is
equal 0. When RunLength is greater than 0 and mb_skip_run_type equal to 1, mb_skip_flag is inferred to be equal to 1.
mb_skip_run_type shall be equal to 0 or 1.


**mb_alc_skip_flag** equal to 1 specifies that the adaptive luminance compensation is applied for the current macroblock.
mb_alc_skip_flag equal to 0 specifies that the adaptive luminance compensation is not applied for the current macroblock.
When not present, mb_alc_skip_flag is inferred to be equal to 0. When mb_alc_skip_flag is equal to 1, the current
macroblock shall be coded as P_Skip.


**J.7.4.5** **Macroblock layer semantics**


The semantics specified in clause I.7.4.5 apply.


**J.7.4.5.1** **Macroblock prediction semantics**


The semantics specified in clause I.7.4.5.1 apply.


**J.7.4.5.2** **Sub-macroblock prediction semantics**


The semantics specified in clause I.7.4.5.2 apply.


**J.7.4.5.3** **Residual data semantics**


The semantics specified in clause I.7.4.5.3 apply.





**J.7.4.5.3.1** **Residual luma semantics**


The semantics specified in clause I.7.4.5.3.1 apply.


**J.7.4.5.3.2** **Residual block CAVLC semantics**


The semantics specified in clause I.7.4.5.3.2 apply.


**J.7.4.5.3.3** **Residual block CABAC semantics**


The semantics specified in clause I.7.4.5.3.3 apply.


**J.7.4.6** **Macroblock layer in 3D-AVC extension semantics**


The semantics specified in clause I.7.4.5 apply by replacing Table 7-13 with Table J-3 and with the following additions.


**mb_direct_type_flag** is used to determine the derivation process for motion vectors and reference indices for
B_Direct_16x16 macroblocks and B_Direct_8x8 sub-macroblocks as specified in clause J.8.2.1.


**mb_alc_flag** equal to 1 specifies that the adaptive luminance compensation mode is in use for the current macroblock.
mb_alc_flag equal to 0 specifies that the adaptive luminance compensation mode is not in use for the current macroblock.
When mb_alc_flag is not present, it is inferred to be equal to 0. When mb_alc_flag is equal to 1, the current macroblock
shall be coded as P_L0_16x16, P_L0_L0_16x8, or P_L0_L0_8x16.


**Table J-3 – Macroblock type values 0 to 4 for P and SP slices**
















|mb_type|Name of mb_type|NumMbPart ( mb_type )|MbPartPredMode ( mb_type, 0 )|MbPartPredMode ( mb_type, 1 )|MbPartWidth ( mb_type )|MbPartHeight ( mb_type )|
|---|---|---|---|---|---|---|
|0|P_L0_16x16|1|Pred_L0|na|16|16|
|1|P_L0_L0_16x8|2|Pred_L0|Pred_L0|16|8|
|2|P_L0_L0_8x16|2|Pred_L0|Pred_L0|8|16|
|3|P_8x8|4|na|na|8|8|
|4|P_8x8ref0|4|na|na|8|8|
|inferred|P_Skip<br>(when<br>MbVSSkipFlag<br>is equal to 0)|1|Pred_L0|na|16|16|
|inferred|P_Skip<br>(when<br>MbVSSkipFlag<br>is equal to 1)|4|Pred_L0|na|8|8|



**J.7.4.6.1** **Macroblock prediction in 3D-AVC extension semantics**


The semantics specified in clause I.7.4.5.1 apply with the following additions.


**bvsp_flag_lX** [ mbPartIdx ] (X being equal to 0 or 1) equal to 0 specifies, when ref_idx_lX[ mbPartIdx ] (with X equal to
0 or 1) refers to an inter-view reference picture, which motion vector derivation process specified in clause J.8.2.1 is in
use. When bvsp_flag_lX[ mbPartIdx ] is not present and VspRefLXFlag[ mbPartIdx ] is equal to 0,
bvsp_flag_lX[ mbPartIdx ] is inferred to be equal to 0. When bvsp_flag_lX[ mbPartIdx ] is not present and
VspRefLXFlag[ mbPartIdx ] is equal to 1, bvsp_flag_lX[ mbPartIdx ] is inferred to be equal to slice_vsp_flag.


**J.7.4.6.2** **Sub-macroblock prediction in 3D-AVC semantics**


The semantics specified in clause I.7.4.5.2 apply with the following additions.
