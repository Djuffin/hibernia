**G.7** **Syntax and semantics**


This clause specifies syntax and semantics for coded video sequences that conform to one or more of the profiles specified
in this annex.


**G.7.1** **Method of specifying syntax in tabular form**


The specifications in clause 7.1 apply.


**G.7.2** **Specification of syntax functions, categories, and descriptors**


The specifications in clause 7.2 apply.


**G.7.3** **Syntax in tabular form**


**G.7.3.1** **NAL unit syntax**


The syntax table is specified in clause 7.3.1.


**G.7.3.1.1** **NAL unit header SVC extension syntax**

|nal_unit_header_svc_extension( ) {|C|Descriptor|
|---|---|---|
|<br>**idr_flag**|All|u(1)|
|<br>**priority_id**|All|u(6)|
|<br>**no_inter_layer_pred_flag**|All|u(1)|
|<br>**dependency_id**|All|u(3)|
|<br>**quality_id**|All|u(4)|
|<br>**temporal_id**|All|u(3)|
|<br>**use_ref_base_pic_flag**|All|u(1)|
|<br>**discardable_flag**|All|u(1)|
|<br>**output_flag**|All|u(1)|
|<br>**reserved_three_2bits**|All|u(2)|
|}|||



**G.7.3.2** **Raw byte sequence payloads and RBSP trailing bits syntax**


**G.7.3.2.1** **Sequence parameter set RBSP syntax**


The syntax table is specified in clause 7.3.2.1.


**G.7.3.2.1.1** **Sequence parameter set data syntax**


The syntax table is specified in clause 7.3.2.1.1.


**G.7.3.2.1.1.1** **Scaling list syntax**


The syntax table is specified in clause 7.3.2.1.1.1.


**G.7.3.2.1.2** **Sequence parameter set extension RBSP syntax**


The syntax table is specified in clause 7.3.2.1.2.


**G.7.3.2.1.3** **Subset sequence parameter set RBSP syntax**


The syntax table is specified in clause 7.3.2.1.3.





**G.7.3.2.1.4** **Sequence parameter set SVC extension syntax**

|seq_parameter_set_svc_extension( ) {|C|Descriptor|
|---|---|---|
|<br>**inter_layer_deblocking_filter_control_present_flag**|0|u(1)|
|<br>**extended_spatial_scalability_idc**|0|u(2)|
|<br>if( ChromaArrayType = = 1 | | ChromaArrayType = = 2 )|||
|<br> <br>**chroma_phase_x_plus1_flag**|0|u(1)|
|<br>if( ChromaArrayType = = 1 )|||
|<br> <br>**chroma_phase_y_plus1**|0|u(2)|
|<br>if( extended_spatial_scalability_idc = = 1 ) {|||
|<br> <br>if( ChromaArrayType > 0 ) {|||
|<br> <br> <br>**seq_ref_layer_chroma_phase_x_plus1_flag**|0|u(1)|
|<br> <br> <br>**seq_ref_layer_chroma_phase_y_plus1**|0|u(2)|
|<br> <br>}|||
|<br> <br>**seq_scaled_ref_layer_left_offset**|0|se(v)|
|<br> <br>**seq_scaled_ref_layer_top_offset**|0|se(v)|
|<br> <br>**seq_scaled_ref_layer_right_offset**|0|se(v)|
|<br> <br>**seq_scaled_ref_layer_bottom_offset**|0|se(v)|
|<br>}|||
|<br>**seq_tcoeff_level_prediction_flag**|0|u(1)|
|<br>if( seq_tcoeff_level_prediction_flag ) {|||
|<br> <br>**adaptive_tcoeff_level_prediction_flag**|0|u(1)|
|<br>}|||
|<br>**slice_header_restriction_flag**|0|u(1)|
|}|||



**G.7.3.2.2** **Picture parameter set RBSP syntax**


The syntax table is specified in clause 7.3.2.2.


**G.7.3.2.3** **Supplemental enhancement information RBSP syntax**


The syntax table is specified in clause 7.3.2.3.


**G.7.3.2.3.1** **Supplemental enhancement information message syntax**


The syntax table is specified in clause 7.3.2.3.1.


**G.7.3.2.4** **Access unit delimiter RBSP syntax**


The syntax table is specified in clause 7.3.2.4.


**G.7.3.2.5** **End of sequence RBSP syntax**


The syntax table is specified in clause 7.3.2.5.


**G.7.3.2.6** **End of stream RBSP syntax**


The syntax table is specified in clause 7.3.2.6.


**G.7.3.2.7** **Filler data RBSP syntax**


The syntax table is specified in clause 7.3.2.7.


**G.7.3.2.8** **Slice layer without partitioning RBSP syntax**


The syntax table is specified in clause 7.3.2.8.


**G.7.3.2.9** **Slice data partition RBSP syntax**


Slice data partition syntax is not present in coded video sequences conforming to any of the profiles specified in this annex.





**G.7.3.2.10RBSP slice trailing bits syntax**


The syntax table is specified in clause 7.3.2.10.


**G.7.3.2.11RBSP trailing bits syntax**


The syntax table is specified in clause 7.3.2.11.


**G.7.3.2.12Prefix NAL unit RBSP syntax**


The syntax table is specified in clause 7.3.2.12.


**G.7.3.2.12.1** **Prefix NAL unit SVC syntax**

|prefix_nal_unit_svc( ) {|C|Descriptor|
|---|---|---|
|<br>if( nal_ref_idc != 0 ) {|||
|<br> <br>**store_ref_base_pic_flag**|2|u(1)|
|<br> <br>if( ( use_ref_base_pic_flag | | store_ref_base_pic_flag ) &&<br> <br> <br>   !idr_flag )|||
|<br> <br> <br>dec_ref_base_pic_marking( )|2||
|<br> <br>**additional_prefix_nal_unit_extension_flag**|2|u(1)|
|<br> <br>if( additional_prefix_nal_unit_extension_flag = = 1 )|||
|<br> <br> <br>while( more_rbsp_data( ) )|||
|<br> <br> <br> <br>**additional_prefix_nal_unit_extension_data_flag**|2|u(1)|
|<br> <br>rbsp_trailing_bits( )|2||
|<br>} else if( more_rbsp_data( ) ) {|||
|<br> <br>while( more_rbsp_data( ) )|||
|<br> <br> <br>**additional_prefix_nal_unit_extension_data_flag**|2|u(1)|
|<br> <br>rbsp_trailing_bits( )|2||
|<br>}|||
|}|||



**G.7.3.2.13Slice layer extension RBSP syntax**


The syntax table is specified in clause 7.3.2.13.


**G.7.3.3** **Slice header syntax**


The syntax table is specified in clause 7.3.3.


**G.7.3.3.1** **Reference picture list modification syntax**


The syntax table is specified in clause 7.3.3.1.


**G.7.3.3.2** **Prediction weight table syntax**


The syntax table is specified in clause 7.3.3.2.


**G.7.3.3.3** **Decoded reference picture marking syntax**


The syntax table is specified in clause 7.3.3.3.


**G.7.3.3.4** **Slice header in scalable extension syntax**

|slice_header_in_scalable_extension( ) {|C|Descriptor|
|---|---|---|
|<br>**first_mb_in_slice**|2|ue(v)|
|<br>**slice_type**|2|ue(v)|
|<br>**pic_parameter_set_id**|2|ue(v)|
|<br>if( separate_colour_plane_flag = = 1 )|||
|<br> <br>**colour_plane_id**|2|u(2)|
|<br>**frame_num**|2|u(v)|






|if( !frame_mbs_only_flag ) {|Col2|Col3|
|---|---|---|
|<br> <br>**field_pic_flag**|2|u(1)|
|<br> <br>if( field_pic_flag )|||
|<br> <br> <br>**bottom_field_flag**|2|u(1)|
|<br>}|||
|<br>if( idr_flag = = 1 )|||
|<br> <br>**idr_pic_id**|2|ue(v)|
|<br>if( pic_order_cnt_type = = 0 ) {|||
|<br> <br>**pic_order_cnt_lsb**|2|u(v)|
|<br> <br>if( bottom_field_pic_order_in_frame_present_flag && !field_pic_flag )|||
|<br> <br> <br>**delta_pic_order_cnt_bottom**|2|se(v)|
|<br>}|||
|<br>if( pic_order_cnt_type = = 1 && !delta_pic_order_always_zero_flag ) {|||
|<br> <br>**delta_pic_order_cnt[** 0**]**|2|se(v)|
|<br> <br>if( bottom_field_pic_order_in_frame_present_flag && !field_pic_flag )|||
|<br> <br> <br>**delta_pic_order_cnt[** 1**]**|2|se(v)|
|<br>}|||
|<br>if( redundant_pic_cnt_present_flag )|||
|<br> <br>**redundant_pic_cnt**|2|ue(v)|
|<br>if( quality_id = = 0 ) {|||
|<br> <br>if( slice_type = = EB )|||
|<br> <br> <br>**direct_spatial_mv_pred_flag**|2|u(1)|
|<br> <br>if( slice_type = = EP | | slice_type = = EB ) {|||
|<br> <br> <br>**num_ref_idx_active_override_flag**|2|u(1)|
|<br> <br> <br>if( num_ref_idx_active_override_flag ) {|||
|<br> <br> <br> <br>**num_ref_idx_l0_active_minus1**|2|ue(v)|
|<br> <br> <br> <br>if( slice_type = = EB )|||
|<br> <br> <br> <br> <br>**num_ref_idx_l1_active_minus1**|2|ue(v)|
|<br> <br> <br>}|||
|<br> <br>}|||
|<br> <br>ref_pic_list_modification( )|2||
|<br> <br>if( ( weighted_pred_flag && slice_type = = EP ) | |<br> <br> <br> <br>( weighted_bipred_idc = = 1 && slice_type = = EB ) ) {|||
|<br> <br> <br>if( !no_inter_layer_pred_flag )|||
|<br> <br> <br> <br>**base_pred_weight_table_flag**|2|u(1)|
|<br> <br> <br>if( no_inter_layer_pred_flag | | !base_pred_weight_table_flag )|||
|<br> <br> <br> <br>pred_weight_table( )|2||
|<br> <br>}|||
|<br> <br>if( nal_ref_idc != 0 ) {|||
|<br> <br> <br>dec_ref_pic_marking( )|2||
|<br> <br> <br>if( !slice_header_restriction_flag ) {|||
|<br> <br> <br> <br>**store_ref_base_pic_flag**|2|u(1)|
|<br> <br> <br> <br>if( ( use_ref_base_pic_flag | | store_ref_base_pic_flag ) &&<br> <br> <br> <br> <br>   !idr_flag )|||
|<br> <br> <br> <br> <br>dec_ref_base_pic_marking( )|2||
|<br> <br> <br>}|||
|<br> <br>}|||
|<br>}|||
|<br>if( entropy_coding_mode_flag && slice_type != EI )|||





|cabac_init_idc|2|ue(v)|
|---|---|---|
|<br>**slice_qp_delta**|2|se(v)|
|<br>if( deblocking_filter_control_present_flag ) {|||
|<br> <br>**disable_deblocking_filter_idc**|2|ue(v)|
|<br> <br>if( disable_deblocking_filter_idc != 1 ) {|||
|<br> <br> <br>**slice_alpha_c0_offset_div2**|2|se(v)|
|<br> <br> <br>**slice_beta_offset_div2**|2|se(v)|
|<br> <br>}|||
|<br>}|||
|<br>if( num_slice_groups_minus1 > 0 &&<br> <br> <br>slice_group_map_type >= 3 && slice_group_map_type <= 5 )|||
|<br> <br>**slice_group_change_cycle**|2|u(v)|
|<br>if( !no_inter_layer_pred_flag && quality_id = = 0 ) {|||
|<br> <br>**ref_layer_dq_id**|2|ue(v)|
|<br> <br>if( inter_layer_deblocking_filter_control_present_flag ) {|||
|<br> <br> <br>**disable_inter_layer_deblocking_filter_idc**|2|ue(v)|
|<br> <br> <br>if( disable_inter_layer_deblocking_filter_idc != 1 ) {|||
|<br> <br> <br> <br>**inter_layer_slice_alpha_c0_offset_div2**|2|se(v)|
|<br> <br> <br> <br>**inter_layer_slice_beta_offset_div2** <br>|2|se(v)|
|<br> <br> <br>}|||
|<br> <br>}|||
|<br> <br>**constrained_intra_resampling_flag**|2|u(1)|
|<br> <br>if( extended_spatial_scalability_idc = = 2 ) {|||
|<br> <br> <br>if( ChromaArrayType > 0 ) {|||
|<br> <br> <br> <br>**ref_layer_chroma_phase_x_plus1_flag**|2|u(1)|
|<br> <br> <br> <br>**ref_layer_chroma_phase_y_plus1**|2|u(2)|
|<br> <br> <br>}|||
|<br> <br> <br>**scaled_ref_layer_left_offset**|2|se(v)|
|<br> <br> <br>**scaled_ref_layer_top_offset**|2|se(v)|
|<br> <br> <br>**scaled_ref_layer_right_offset**|2|se(v)|
|<br> <br> <br>**scaled_ref_layer_bottom_offset**|2|se(v)|
|<br> <br>}|||
|<br>}|||
|<br>if( !no_inter_layer_pred_flag ) {|||
|<br> <br>**slice_skip_flag**|2|u(1)|
|<br> <br>if( slice_skip_flag )|||
|<br> <br> <br>**num_mbs_in_slice_minus1**|2|ue(v)|
|<br> <br>else {|||
|<br> <br> <br>**adaptive_base_mode_flag**|2|u(1)|
|<br> <br> <br>if( !adaptive_base_mode_flag )|||
|<br> <br> <br> <br>**default_base_mode_flag**|2|u(1)|
|<br> <br> <br>if( !default_base_mode_flag ) {|||
|<br> <br> <br> <br>**adaptive_motion_prediction_flag**|2|u(1)|
|<br> <br> <br> <br>if( !adaptive_motion_prediction_flag )|||
|<br> <br> <br> <br> <br>**default_motion_prediction_flag**|2|u(1)|
|<br> <br> <br>}|||
|<br> <br> <br>**adaptive_residual_prediction_flag**|2|u(1)|
|<br> <br> <br>if( !adaptive_residual_prediction_flag )|||
|<br> <br> <br> <br>**default_residual_prediction_flag**|2|u(1)|





|}|Col2|Col3|
|---|---|---|
|<br> <br>if( adaptive_tcoeff_level_prediction_flag )|||
|<br> <br> <br>**tcoeff_level_prediction_flag**|2|u(1)|
|<br>}|||
|<br>if( !slice_header_restriction_flag && !slice_skip_flag ) {|||
|<br> <br>**scan_idx_start**|2|u(4)|
|<br> <br>**scan_idx_end**|2|u(4)|
|<br>}|||
|}|||


**G.7.3.3.5** **Decoded reference base picture marking syntax**

|dec_ref_base_pic_marking( ) {|C|Descriptor|
|---|---|---|
|<br>**adaptive_ref_base_pic_marking_mode_flag**|2|u(1)|
|<br>if( adaptive_ref_base_pic_marking_mode_flag )|||
|<br> <br>do {|||
|<br> <br> <br>**memory_management_base_control_operation**|2|ue(v)|
|<br> <br> <br>if( memory_management_base_control_operation = = 1 )|||
|<br> <br> <br> <br>**difference_of_base_pic_nums_minus1**|2|ue(v)|
|<br> <br> <br>if( memory_management_base_control_operation = = 2 )|||
|<br> <br> <br> <br>**long_term_base_pic_num**|2|ue(v)|
|<br> <br>} while( memory_management_base_control_operation != 0 )|||
|}|||



**G.7.3.4** **Slice data syntax**


The syntax table is specified in clause 7.3.4.





**G.7.3.4.1** **Slice data in scalable extension syntax**

|slice_data_in_scalable_extension( ) {|C|Descriptor|
|---|---|---|
|<br>if( entropy_coding_mode_flag)|||
|<br> <br>while( !byte_aligned( ) )|||
|<br> <br> <br>**cabac_alignment_one_bit**|2|f(1)|
|<br>CurrMbAddr = first_mb_in_slice * ( 1 + MbaffFrameFlag )|||
|<br>moreDataFlag = 1|||
|<br>prevMbSkipped = 0|||
|<br>do {|||
|<br> <br>if( slice_type != EI )|||
|<br> <br> <br>if( !entropy_coding_mode_flag ) {|||
|<br> <br> <br> <br>**mb_skip_run**|2|ue(v)|
|<br> <br> <br> <br>prevMbSkipped = ( mb_skip_run > 0 )|||
|<br> <br> <br> <br>for( i = 0; i < mb_skip_run; i++ )|||
|<br> <br> <br> <br> <br>CurrMbAddr = NextMbAddress( CurrMbAddr )|||
|<br> <br> <br> <br>if( mb_skip_run > 0 )|||
|<br> <br> <br> <br> <br>moreDataFlag = more_rbsp_data( )|||
|<br> <br> <br>} else {|||
|<br> <br> <br> <br>**mb_skip_flag**|2|ae(v)|
|<br> <br> <br> <br>moreDataFlag = !mb_skip_flag|||
|<br> <br> <br>}|||
|<br> <br>if( moreDataFlag ) {|||
|<br> <br> <br>if( MbaffFrameFlag && ( ( CurrMbAddr % 2 ) = = 0 | |<br> <br> <br> <br> <br>( ( CurrMbAddr % 2 ) = = 1 && prevMbSkipped ) ) )|||
|<br> <br> <br> <br>**mb_field_decoding_flag**|2|u(1) | ae(v)|
|<br> <br> <br>macroblock_layer_in_scalable_extension( )|2 | 3 | 4||
|<br> <br>}|||
|<br> <br>if( !entropy_coding_mode_flag )|||
|<br> <br> <br>moreDataFlag = more_rbsp_data( )|||
|<br> <br>else {|||
|<br> <br> <br>if( slice_type != EI )|||
|<br> <br> <br> <br>prevMbSkipped = mb_skip_flag|||
|<br> <br> <br>if( MbaffFrameFlag && ( CurrMbAddr % 2 ) = = 0 )|||
|<br> <br> <br> <br>moreDataFlag = 1|||
|<br> <br> <br>else {|||
|<br> <br> <br> <br>**end_of_slice_flag**|2|ae(v)|
|<br> <br> <br> <br>moreDataFlag = !end_of_slice_flag|||
|<br> <br> <br>}|||
|<br> <br>}|||
|<br> <br>CurrMbAddr = NextMbAddress( CurrMbAddr )|||
|<br>} while( moreDataFlag )|||
|}|||



**G.7.3.5** **Macroblock layer syntax**


The syntax table is specified in clause 7.3.5.


**G.7.3.5.1** **Macroblock prediction syntax**


The syntax table is specified in clause 7.3.5.1.





**G.7.3.5.2** **Sub-macroblock prediction syntax**


The syntax table is specified in clause 7.3.5.2.


**G.7.3.5.3** **Residual data syntax**


The syntax table is specified in clause 7.3.5.3.


**G.7.3.5.3.1** **Residual luma syntax**


The syntax table is specified in clause 7.3.5.3.1.


**G.7.3.5.3.2** **Residual block CAVLC syntax**


The syntax table is specified in clause 7.3.5.3.2.


**G.7.3.5.3.3** **Residual block CABAC syntax**


The syntax table is specified in clause 7.3.5.3.3.


**G.7.3.6** **Macroblock layer in scalable extension syntax**

|macroblock_layer_in_scalable_extension( ) {|C|Descriptor|
|---|---|---|
|<br>if( InCropWindow( CurrMbAddr ) && adaptive_base_mode_flag )|||
|<br> <br>**base_mode_flag**|2|u(1) | ae(v)|
|<br>if( !base_mode_flag)|||
|<br> <br>**mb_type**|2|ue(v) | ae(v)|
|<br>if( mb_type = = I_PCM ) {|||
|<br> <br>while( !byte_aligned( ) )|||
|<br> <br> <br>**pcm_alignment_zero_bit**|3|f(1)|
|<br> <br>for( i = 0; i < 256; i++ )|||
|<br> <br> <br>**pcm_sample_luma[** i**]**|3|u(v)|
|<br> <br>for( i = 0; i < 2 * MbWidthC * MbHeightC; i++ )|||
|<br> <br> <br>**pcm_sample_chroma[** i**]**|3|u(v)|
|<br>} else {|||
|<br> <br>if( !base_mode_flag ) {|||
|<br> <br> <br>noSubMbPartSizeLessThan8x8Flag = 1|||
|<br> <br> <br>if( mb_type != I_NxN &&<br> <br> <br> <br> <br>MbPartPredMode( mb_type, 0 ) != Intra_16x16 &&<br> <br> <br> <br> <br>NumMbPart( mb_type ) = = 4 ) {|||
|<br> <br> <br> <br>sub_mb_pred_in_scalable_extension( mb_type )|2||
|<br> <br> <br> <br>for( mbPartIdx = 0; mbPartIdx < 4; mbPartIdx++ )|||
|<br> <br> <br> <br> <br>if( sub_mb_type[ mbPartIdx ] != B_Direct_8x8 ) {|||
|<br> <br> <br> <br> <br> <br>if( NumSubMbPart( sub_mb_type[ mbPartIdx ] ) > 1 )|||
|<br> <br> <br> <br> <br> <br> <br>noSubMbPartSizeLessThan8x8Flag = 0|||
|<br> <br> <br> <br> <br>} else if( !direct_8x8_inference_flag )|||
|<br> <br> <br> <br> <br> <br>noSubMbPartSizeLessThan8x8Flag = 0|||
|<br> <br> <br>} else {|||
|<br> <br> <br> <br>if( transform_8x8_mode_flag && mb_type = = I_NxN )|||
|<br> <br> <br> <br> <br>**transform_size_8x8_flag**|2|u(1) | ae(v)|
|<br> <br> <br> <br>mb_pred_in_scalable_extension( mb_type )|2||
|<br> <br> <br>}|||
|<br> <br>}|||






|if( adaptive_residual_prediction_flag && slice_type != EI &&<br>InCropWindow( CurrMbAddr ) &&<br>( base_mode_flag | |<br>( MbPartPredMode( mb_type, 0 ) != Intra_16x16 &&<br>MbPartPredMode( mb_type, 0 ) != Intra_8x8 &&<br>MbPartPredMode( mb_type, 0 ) != Intra_4x4 ) ) )|Col2|Col3|
|---|---|---|
|<br> <br> <br>**residual_prediction_flag**|2|u(1) | ae(v)|
|<br> <br>if( scan_idx_end >= scan_idx_start ) {|||
|<br> <br> <br>if( base_mode_flag | |<br> <br> <br> <br> <br>MbPartPredMode( mb_type, 0 ) != Intra_16x16 ) {|||
|<br> <br> <br> <br>**coded_block_pattern**|2|me(v) | ae(v)|
|<br> <br> <br> <br>if( CodedBlockPatternLuma > 0 &&<br> <br> <br> <br> <br> <br> transform_8x8_mode_flag &&<br> <br> <br> <br> <br> <br>( base_mode_flag | |<br> <br> <br> <br> <br> <br> <br>( mb_type != I_NxN &&<br> <br> <br> <br> <br> <br> <br> noSubMbPartSizeLessThan8x8Flag &&<br> <br> <br> <br> <br> <br> <br> ( mb_type != B_Direct_16x16 | |<br> <br> <br> <br> <br> <br> <br>  direct_8x8_inference_flag ) ) ) )|||
|<br> <br> <br> <br> <br>**transform_size_8x8_flag**|2|u(1) | ae(v)|
|<br> <br> <br>}|||
|<br> <br> <br>if( CodedBlockPatternLuma > 0 | |<br> <br> <br> <br> <br> CodedBlockPatternChroma > 0 | |<br> <br> <br> <br> <br> MbPartPredMode( mb_type, 0 ) = = Intra_16x16 ) {|||
|<br> <br> <br> <br>**mb_qp_delta**|2|se(v) | ae(v)|
|<br> <br> <br> <br>residual( scan_idx_start, scan_idx_end )|3 | 4||
|<br> <br> <br>}|||
|<br> <br>}|||
|<br>}|||
|}|||


**G.7.3.6.1** **Macroblock prediction in scalable extension syntax**

|mb_pred_in_scalable_extension( mb_type ) {|C|Descriptor|
|---|---|---|
|<br>if( MbPartPredMode( mb_type, 0 ) = = Intra_4x4 | |<br> <br> <br>MbPartPredMode( mb_type, 0 ) = = Intra_8x8 | |<br> <br> <br>MbPartPredMode( mb_type, 0 ) = = Intra_16x16 ) {|||
|<br> <br>if( MbPartPredMode( mb_type, 0 ) = = Intra_4x4 )|||
|<br> <br> <br>for( luma4x4BlkIdx = 0; luma4x4BlkIdx < 16; luma4x4BlkIdx++ ) {|||
|<br> <br> <br> <br>**prev_intra4x4_pred_mode_flag[** luma4x4BlkIdx**]**|2|u(1) | ae(v)|
|<br> <br> <br> <br>if( !prev_intra4x4_pred_mode_flag**[** luma4x4BlkIdx**]** )|||
|<br> <br> <br> <br> <br>**rem_intra4x4_pred_mode[** luma4x4BlkIdx**]**|2|u(3) | ae(v)|
|<br> <br> <br>}|||
|<br> <br>if( MbPartPredMode( mb_type, 0 ) = = Intra_8x8 )|||
|<br> <br> <br>for( luma8x8BlkIdx = 0; luma8x8BlkIdx < 4; luma8x8BlkIdx++ ) {|||
|<br> <br> <br> <br>**prev_intra8x8_pred_mode_flag[** luma8x8BlkIdx**]**|2|u(1) | ae(v)|
|<br> <br> <br> <br>if( !prev_intra8x8_pred_mode_flag[ luma8x8BlkIdx ] )|||
|<br> <br> <br> <br> <br>**rem_intra8x8_pred_mode[** luma8x8BlkIdx**]**|2|u(3) | ae(v)|
|<br> <br> <br>}|||
|<br> <br>if( ChromaArrayType != 0 )|||
|<br> <br> <br>**intra_chroma_pred_mode**|2|ue(v) | ae(v)|
|<br>} else if( MbPartPredMode( mb_type, 0 ) != Direct ) {|||
|<br> <br>if( InCropWindow( CurrMbAddr ) &&<br>       adaptive_motion_prediction_flag ) {|||






|for( mbPartIdx = 0; mbPartIdx < NumMbPart( mb_type ); mbPartIdx++ )|Col2|Col3|
|---|---|---|
|<br> <br> <br> <br>if( MbPartPredMode( mb_type, mbPartIdx ) != Pred_L1 )|||
|<br> <br> <br> <br> <br>**motion_prediction_flag_l0[** mbPartIdx**]**|2|u(1) | ae(v)|
|<br> <br> <br>for( mbPartIdx = 0; mbPartIdx < NumMbPart( mb_type ); mbPartIdx++ )|||
|<br> <br> <br> <br>if( MbPartPredMode( mb_type, mbPartIdx ) != Pred_L0 )|||
|<br> <br> <br> <br> <br>**motion_prediction_flag_l1[** mbPartIdx**]**|2|u(1) | ae(v)|
|<br> <br>}|||
|<br> <br>for( mbPartIdx = 0; mbPartIdx < NumMbPart( mb_type ); mbPartIdx++ )|||
|<br> <br> <br>if( ( num_ref_idx_l0_active_minus1 > 0 | |<br> <br> <br> <br> <br>  mb_field_decoding_flag != field_pic_flag ) &&<br> <br> <br> <br> <br>MbPartPredMode( mb_type, mbPartIdx ) != Pred_L1 &&<br> <br> <br> <br> <br>!motion_prediction_flag_l0[ mbPartIdx ] )|||
|<br> <br> <br> <br>**ref_idx_l0[** mbPartIdx**]**|2|te(v) | ae(v)|
|<br> <br>for( mbPartIdx = 0; mbPartIdx < NumMbPart( mb_type ); mbPartIdx++ )|||
|<br> <br> <br>if( ( num_ref_idx_l1_active_minus1 > 0 | |<br> <br> <br> <br> <br>  mb_field_decoding_flag != field_pic_flag ) &&<br> <br> <br> <br> <br>MbPartPredMode( mb_type, mbPartIdx ) != Pred_L0 &&<br> <br> <br> <br> <br>!motion_prediction_flag_l1[ mbPartIdx ] )|||
|<br> <br> <br> <br>**ref_idx_l1[** mbPartIdx**]**|2|te(v) | ae(v)|
|<br> <br>for( mbPartIdx = 0; mbPartIdx < NumMbPart( mb_type ); mbPartIdx++ )|||
|<br> <br> <br>if( MbPartPredMode ( mb_type, mbPartIdx ) != Pred_L1 )|||
|<br> <br> <br> <br>for( compIdx = 0; compIdx < 2; compIdx++ )|||
|<br> <br> <br> <br> <br>**mvd_l0[** mbPartIdx**][** 0**][** compIdx**]**|2|se(v) | ae(v)|
|<br> <br>for( mbPartIdx = 0; mbPartIdx < NumMbPart( mb_type ); mbPartIdx++ )|||
|<br> <br> <br>if( MbPartPredMode( mb_type, mbPartIdx ) != Pred_L0 )|||
|<br> <br> <br> <br>for( compIdx = 0; compIdx < 2; compIdx++ )|||
|<br> <br> <br> <br> <br>**mvd_l1[** mbPartIdx**][** 0**][** compIdx**]**|2|se(v) | ae(v)|
|<br>}|||
|}|||





**G.7.3.6.2** **Sub-macroblock prediction in scalable extension syntax**

|sub_mb_pred_in_scalable_extension( mb_type ) {|C|Descriptor|
|---|---|---|
|<br>for( mbPartIdx = 0; mbPartIdx < 4; mbPartIdx++ )|||
|<br> <br>**sub_mb_type[** mbPartIdx**]**|2|ue(v) | ae(v)|
|<br>if( InCropWindow( CurrMbAddr ) && adaptive_motion_prediction_flag ) {|||
|<br> <br>for( mbPartIdx = 0; mbPartIdx < 4; mbPartIdx++ )|||
|<br> <br> <br>if( SubMbPredMode( sub_mb_type[ mbPartIdx ] ) != Direct &&<br> <br> <br> <br> <br>SubMbPredMode( sub_mb_type[ mbPartIdx ] ) != Pred_L1 )|||
|<br> <br> <br> <br>**motion_prediction_flag_l0[ mbPartIdx ]**|2|u(1) | ae(v)|
|<br> <br>for( mbPartIdx = 0; mbPartIdx < 4; mbPartIdx++ )|||
|<br> <br> <br>if( SubMbPredMode( sub_mb_type[ mbPartIdx ] ) != Direct &&<br> <br> <br> <br> <br>SubMbPredMode( sub_mb_type[ mbPartIdx ] ) != Pred_L0 )|||
|<br> <br> <br> <br>**motion_prediction_flag_l1[ mbPartIdx ]**|2|u(1) | ae(v)|
|<br>}|||
|<br>for( mbPartIdx = 0; mbPartIdx < 4; mbPartIdx++ )|||
|<br> <br>if( ( num_ref_idx_l0_active_minus1 > 0 | |<br> <br> <br> <br>  mb_field_decoding_flag != field_pic_flag ) &&<br> <br> <br> <br>mb_type != P_8x8ref0 &&<br> <br> <br> <br>sub_mb_type[ mbPartIdx ] != B_Direct_8x8 &&<br> <br> <br> <br>SubMbPredMode( sub_mb_type[ mbPartIdx ] ) != Pred_L1 &&<br> <br> <br> <br>!motion_prediction_flag_l0[ mbPartIdx ] )|||
|<br> <br> <br>**ref_idx_l0[** mbPartIdx**]**|2|te(v) | ae(v)|
|<br>for( mbPartIdx = 0; mbPartIdx < 4; mbPartIdx++ )|||
|<br> <br>if( ( num_ref_idx_l1_active_minus1 > 0 | |<br> <br> <br> <br>  mb_field_decoding_flag != field_pic_flag ) &&<br> <br> <br> <br>sub_mb_type[ mbPartIdx ] != B_Direct_8x8 &&<br> <br> <br> <br>SubMbPredMode( sub_mb_type[ mbPartIdx ] ) != Pred_L0 &&<br> <br> <br> <br>!motion_prediction_flag_l1[ mbPartIdx ] )|||
|<br> <br> <br>**ref_idx_l1[** mbPartIdx**]**|2|te(v) | ae(v)|
|<br>for( mbPartIdx = 0; mbPartIdx < 4; mbPartIdx++ )|||
|<br> <br>if( sub_mb_type[ mbPartIdx ] != B_Direct_8x8 &&<br> <br> <br> <br>SubMbPredMode( sub_mb_type[ mbPartIdx ] ) != Pred_L1 )|||
|<br> <br> <br>for( subMbPartIdx = 0;<br> <br> <br> <br> <br>  subMbPartIdx < NumSubMbPart( sub_mb_type[ mbPartIdx ] );<br> <br> <br> <br> <br>  subMbPartIdx++ )|||
|<br> <br> <br> <br>for( compIdx = 0; compIdx < 2; compIdx++ )|||
|<br> <br> <br> <br> <br>**mvd_l0[** mbPartIdx**][** subMbPartIdx**][** compIdx**]**|2|se(v) | ae(v)|
|<br>for( mbPartIdx = 0; mbPartIdx < 4; mbPartIdx++ )|||
|<br> <br>if( sub_mb_type[ mbPartIdx ] != B_Direct_8x8 &&<br> <br> <br> <br>SubMbPredMode( sub_mb_type[ mbPartIdx ] ) != Pred_L0 )|||
|<br> <br> <br>for( subMbPartIdx = 0;<br> <br> <br> <br> <br>  subMbPartIdx < NumSubMbPart( sub_mb_type[ mbPartIdx ] );<br> <br> <br> <br> <br>  subMbPartIdx++ )|||
|<br> <br> <br> <br>for( compIdx = 0; compIdx < 2; compIdx++ )|||
|<br> <br> <br> <br> <br>**mvd_l1[** mbPartIdx**][** subMbPartIdx**][** compIdx**]**|2|se(v) | ae(v)|
|}|||



**G.7.4** **Semantics**


Semantics associated with the syntax structures and syntax elements within these structures (in clause G.7.3 and in
clause 7.3 by reference in clause G.7.3) are specified in this clause and by reference to clause 7.4. When the semantics of
a syntax element are specified using a table or a set of tables, any values that are not specified in the table(s) shall not be
present in the bitstream unless otherwise specified in this Recommendation | International Standard.


Sub-bitstreams that are derived according to the process specified in clause G.8.8.1 shall conform to one or more of the
profiles specified in Annex A or one or more of the profiles specified in this annex.





One or more sub-bitstreams shall conform to one or more of the profiles specified in Annex A. The decoding for these subbitstreams is specified in clauses 2 to 9 and Annexes B to E.


The decoding for bitstreams conforming to one or more of the profiles specified in this annex is completely specified in
this annex with reference made to clauses 2 to 9 and Annexes B to E.


A specification or a process in clauses 2 to 9 and Annexes B to E may be used as is or by specifying assignments or
alternative meanings of certain parts.


This clause describes the semantics of syntax elements. The syntax elements appear multiple times in the bitstream and in
each access unit. The meaning of each syntax element and derived variables depends on the position of the syntax structure
in the bitstream in which it is contained. A decoder conforming to this Recommendation | International Standard processes
the syntax structures in decoding order and determines the semantics according to the position derived from that.


**G.7.4.1** **NAL unit semantics**


The semantics for the syntax elements in clause G.7.3.1 are specified in clause 7.4.1. The following specifications
additionally apply.


For NAL units with nal_unit_type equal to 14, nal_ref_idc shall be identical to nal_ref_idc of the associated NAL unit,
which succeeds the NAL unit with nal_unit_type equal to 14 in decoding order.


The value of nal_ref_idc shall be the same for all VCL NAL units of a dependency representation.


The variable refNalRefIdc is derived as follows:


- If nal_unit_type is not equal to 20 or dependency_id is equal to the minimum value of dependency_id for all VCL
NAL units of the coded picture, refNalRefIdc is set equal to 0.


- Otherwise (nal_unit_type is equal to 20 and dependency_id is not equal to the minimum value of dependency_id for
all VCL NAL units of the coded picture), refNalRefIdc is set equal to the maximum value of nal_ref_idc for all VCL
NAL units of the coded picture with a value of dependency_id less than the current value of dependency_id.


When refNalRefIdc is greater than 0, the value of nal_ref_idc shall not be equal to 0.


nal_ref_idc equal to 0 for a NAL unit containing a slice and having a value of dependency_id that is equal to the maximum
value of dependency_id in the coded picture indicates that all coded slice NAL units of the coded picture are coded slice
NAL units of a non-reference picture.


nal_ref_idc greater than 0 for a NAL unit containing a slice and having a value of dependency_id that is equal to the
maximum value of dependency_id in the coded picture indicates that all coded slice NAL units of the coded picture are
coded slice NAL units of a reference picture.


**G.7.4.1.1** **NAL unit header SVC extension semantics**


The syntax elements idr_flag, priority_id, no_inter_layer_pred_flag, dependency_id, quality_id, temporal_id,
use_ref_base_pic_flag, discardable_flag, and output_flag, when present in a prefix NAL unit, are considered as if they
were present in the associated NAL unit.


**idr_flag** equal to 1 specifies that the current coded picture is an IDR picture when the value of dependency_id for the NAL
unit is equal to the maximum value of dependency_id in the coded picture. idr_flag equal to 0 specifies that the current
coded picture is not an IDR picture when the value of dependency_id for the NAL unit is equal to the maximum value of
dependency_id in the coded picture. The value of idr_flag shall be the same for all NAL units of a dependency
representation.

NOTE 1 – The classification of a coded picture as IDR picture and the partitioning of a sequence of access units in coded video
sequences depends on the maximum value of dependency_id that is present in the associated NAL units. When NAL units are
removed from a bitstream, e.g. in order to adjust the bitstream to the capabilities of a receiving device, the maximum value of
dependency_id in the coded pictures may change and hence the classification of coded pictures as IDR pictures may change and
with that the partitioning of the sequence of access units into coded video sequences may change.


When idr_flag is equal to 1 for a prefix NAL unit, the associated NAL unit shall have nal_unit_type equal to 5. When
idr_flag is equal to 0 for a prefix NAL unit, the associated NAL unit shall have nal_unit_type equal to 1.


When nal_ref_idc is equal to 0, the value of idr_flag shall be equal to 0.


For NAL units, in which idr_flag is present, the variable IdrPicFlag derived in clause 7.4.1 is modified by setting it equal
to idr_flag.


**priority_id** specifies a priority identifier for the NAL unit. The assignment of values to priority_id is constrained by the
sub-bitstream extraction process as specified in clause G.8.8.1.





NOTE 2  - The syntax element priority_id is not required by the decoding process specified in this
Recommendation | International Standard. The syntax element priority_id may be used as determined by the application within the
specified constraints.


**no_inter_layer_pred_flag** specifies whether inter-layer prediction may be used for decoding the coded slice. When
no_inter_layer_pred_flag is equal to 1, inter-layer prediction is not used for decoding the coded slice. When
no_inter_layer_pred_flag is equal to 0, inter-layer prediction may be used for decoding the coded slice as signalled in the
macroblock layer.


For prefix NAL units, no_inter_layer_pred_flag shall be equal to 1. When nal_unit_type is equal to 20 and quality_id is
greater than 0, no_inter_layer_pred_flag shall be equal to 0.


The variable MinNoInterLayerPredFlag is set equal to the minimum value of no_inter_layer_pred_flag for the slices of the
layer representation.


**dependency_id** specifies a dependency identifier for the NAL unit. dependency_id shall be equal to 0 in prefix NAL units.
The assignment of values to dependency_id is constrained by the sub-bitstream extraction process as specified in
clause G.8.8.1.


**quality_id** specifies a quality identifier for the NAL unit. quality_id shall be equal to 0 in prefix NAL units. The assignment
of values to quality_id is constrained by the sub-bitstream extraction process as specified in clause G.8.8.1.


The variable DQId is derived by


DQId = ( dependency_id << 4 ) + quality_id (G-63)


When nal_unit_type is equal to 20, the bitstream shall not contain data that result in DQId equal to 0.


**temporal_id** specifies a temporal identifier for the NAL unit. The assignment of values to temporal_id is constrained by
the sub-bitstream extraction process as specified in clause G.8.8.1.


The value of temporal_id shall be the same for all prefix NAL units and coded slice in scalable extension NAL units of an
access unit. When an access unit contains any NAL unit with nal_unit_type equal to 5 or idr_flag equal to 1, temporal_id
shall be equal to 0.


**use_ref_base_pic_flag** equal to 1 specifies that reference base pictures (when present) and decoded pictures (when
reference base pictures are not present) are used as reference pictures for inter prediction as specified in clause G.8.2.3.
use_ref_base_pic_flag equal to 0 specifies that reference base pictures are not used as reference pictures for inter prediction
(i.e., only decoded pictures are used for inter prediction).


The values of use_ref_base_pic_flag shall be the same for all NAL units of a dependency representation.


**discardable_flag** equal to 1 specifies that the current NAL unit is not used for decoding dependency representations that
are part of the current coded picture or any subsequent coded picture in decoding order and have a greater value of
dependency_id than the current NAL unit. discardable_flag equal to 0 specifies that the current NAL unit may be used for
decoding dependency representations that are part of the current coded picture or any subsequent coded picture in decoding
order and have a greater value of dependency_id than the current NAL unit.


**output_flag** affects the decoded picture output and removal processes as specified in Annex C. The value of output_flag
shall be the same for all NAL units of a dependency representation. For any particular value of dependency_id, the value
of output_flag shall be the same for both fields of a complementary field pair.


**reserved_three_2bits** shall be equal to 3. Other values of reserved_three_2bits may be specified in the future by ITU-T |
ISO/IEC. Decoders shall ignore the value of reserved_three_2bits.


**G.7.4.1.2** **Order of NAL units and association to coded pictures, access units, and video sequences**


This clause specifies constraints on the order of NAL units in the bitstream. Any order of NAL units in the bitstream
obeying these constraints is referred to in the text as the decoding order of NAL units. Within a NAL unit, the syntax in
clauses 7.3, D.1, E.1, G.7.3, G.13.1, and G.14.1 specifies the decoding order of syntax elements. Decoders shall be capable
of receiving NAL units and their syntax elements in decoding order.


**G.7.4.1.2.1** **Order of SVC sequence parameter set RBSPs and picture parameter set RBSPs and their activation**

NOTE 1 – The sequence and picture parameter set mechanism decouples the transmission of infrequently changing information
from the transmission of coded macroblock data. Sequence and picture parameter sets may, in some applications, be conveyed "outof-band" using a reliable transport mechanism.


A picture parameter set RBSP includes parameters that can be referred to by the coded slice NAL units of one or more
layer representations of one or more coded pictures.


Each picture parameter set RBSP is initially considered not active at the start of the operation of the decoding process. At
most one picture parameter set RBSP is considered as the active picture parameter set RBSP at any given moment during





the operation of the decoding process, and when any particular picture parameter set RBSP becomes the active picture
parameter set RBSP, the previously-active picture parameter set RBSP (if any) is deactivated.


In addition to the active picture parameter set RBSP, zero or more picture parameter set RBSPs may be specifically active
for layer representations (with a particular value of DQId less than DQIdMax) that may be referred to through inter-layer
prediction in decoding the target layer representation. Such a picture parameter set RBSP is referred to as active layer
picture parameter set RBSP for the particular value of DQId (less than DQIdMax). The restrictions on active picture
parameter set RBSPs also apply to active layer picture parameter set RBSPs with a particular value of DQId.


When a picture parameter set RBSP (with a particular value of pic_parameter_set_id) is not the active picture parameter
set RBSP and it is referred to by a coded slice NAL unit with DQId equal to DQIdMax (using that value of
pic_parameter_set_id), it is activated. This picture parameter set RBSP is called the active picture parameter set RBSP
until it is deactivated when another picture parameter set RBSP becomes the active picture parameter set RBSP. A picture
parameter set RBSP, with that particular value of pic_parameter_set_id, shall be available to the decoding process prior to
its activation.


When a picture parameter set RBSP (with a particular value of pic_parameter_set_id) is not the active layer picture
parameter set for a particular value of DQId less than DQIdMax and it is referred to by a coded slice NAL unit with the
particular value of DQId (using that value of pic_parameter_set_id), it is activated for layer representations with the
particular value of DQId. This picture parameter set RBSP is called the active layer picture parameter set RBSP for the
particular value of DQId until it is deactivated when another picture parameter set RBSP becomes the active layer picture
parameter set RBSP for the particular value of DQId or when decoding an access unit with DQIdMax less than or equal to
the particular value of DQId. A picture parameter set RBSP, with that particular value of pic_parameter_set_id, shall be
available to the decoding process prior to its activation.


Any picture parameter set NAL unit containing the value of pic_parameter_set_id for the active picture parameter set
RBSP for a coded picture shall have the same content as that of the active picture parameter set RBSP for the coded picture
unless it follows the last VCL NAL unit of the coded picture and precedes the first VCL NAL unit of another coded picture.
Any picture parameter set NAL unit containing the value of pic_parameter_set_id for the active layer picture parameter
set RBSP for a particular value of DQId less than DQIdMax for a coded picture shall have the same content as that of the
active layer picture parameter set RBSP for the particular value of DQId for the coded picture unless it follows the last
VCL NAL unit of the coded picture and precedes the first VCL NAL unit of another coded picture.


When a picture parameter set NAL unit with a particular value of pic_parameter_set_id is received, its content replaces
the content of the previous picture parameter set NAL unit, in decoding order, with the same value of pic_parameter_set_id
(when a previous picture parameter set NAL unit with the same value of pic_parameter_set_id was present in the
bitstream).

NOTE 2 – A decoder must be capable of simultaneously storing the contents of the picture parameter sets for all values of
pic_parameter_set_id. The content of the picture parameter set with a particular value of pic_parameter_set_id is overwritten when
a new picture parameter set NAL unit with the same value of pic_parameter_set_id is received.


An SVC sequence parameter set RBSP includes parameters that can be referred to by one or more picture parameter set
RBSPs or one or more SEI NAL units containing a buffering period SEI message.


Each SVC sequence parameter set RBSP is initially considered not active at the start of the operation of the decoding
process. At most one SVC sequence parameter set RBSP is considered as the active SVC sequence parameter set RBSP at
any given moment during the operation of the decoding process, and when any particular SVC sequence parameter set
RBSP becomes the active SVC sequence parameter set RBSP, the previously-active SVC sequence parameter set RBSP
(if any) is deactivated.


In addition to the active SVC sequence parameter set RBSP, zero or more SVC sequence parameter set RBSPs may be
specifically active for layer representations (with a particular value of DQId less than DQIdMax) that may be referred to
through inter-layer prediction in decoding the target layer representation. Such an SVC sequence parameter set RBSP is
referred to as active layer SVC sequence parameter set RBSP for the particular value of DQId (less than DQIdMax). The
restrictions on active SVC sequence parameter set RBSPs also apply to active layer SVC sequence parameter set RBSPs
with a particular value of DQId.


For the following specification in this clause, the activating buffering period SEI message is specified as follows:


- If the access unit contains one or more buffering period SEI messages that are included in a scalable nesting SEI
message and are associated with values of DQId in the range of ( ( DQIdMax >> 4) << 4) to
( ( ( DQIdMax >> 4 ) << 4 ) + 15), inclusive, the last of these buffering period SEI messages in decoding order is the
activating buffering period SEI message.


- Otherwise, if DQIdMax is equal to 0 and the access unit contains a buffering period SEI message that is not included
in a scalable nesting SEI message, this buffering period SEI message is the activating buffering period SEI message.


- Otherwise, the access unit does not contain an activating buffering period SEI message.





When a sequence parameter set RBSP (nal_unit_type is equal to 7) with a particular value of seq_parameter_set_id is not
already the active SVC sequence parameter set RBSP and it is referred to by activation of a picture parameter set RBSP
(using that value of seq_parameter_set_id) and the picture parameter set RBSP is activated by a coded slice NAL unit with
nal_unit_type equal to 1 or 5 (the picture parameter set RBSP becomes the active picture parameter set RBSP and
DQIdMax is equal to 0) and the access unit does not contain an activating buffering period SEI message, the sequence
parameter set RBSP is activated. This sequence parameter set RBSP, is called the active SVC sequence parameter set
RBSP until it is deactivated when another SVC sequence parameter set RBSP becomes the active SVC sequence parameter
set RBSP. A sequence parameter set RBSP, with that particular value of seq_parameter_set_id, shall be available to the
decoding process prior to its activation.


When a sequence parameter set RBSP (nal_unit_type is equal to 7) with a particular value of seq_parameter_set_id is not
already the active SVC sequence parameter set RBSP and it is referred to by an activating buffering period SEI message
(using that value of seq_parameter_set_id) that is not included in a scalable nesting SEI message (DQIdMax is equal to 0),
the sequence parameter set RBSP is activated. This sequence parameter set RBSP is called the active SVC sequence
parameter set RBSP until it is deactivated when another SVC sequence parameter set RBSP becomes the active SVC
sequence parameter set RBSP. A sequence parameter set RBSP, with that particular value of seq_parameter_set_id, shall
be available to the decoding process prior to its activation.


When a subset sequence parameter set RBSP (nal_unit_type is equal to 15) with a particular value of seq_parameter_set_id
is not already the active SVC sequence parameter set RBSP and it is referred to by activation of a picture parameter set
RBSP (using that value of seq_parameter_set_id) and the picture parameter set RBSP is activated by a coded slice in
scalable extension NAL unit (nal_unit_type is equal to 20) with DQId equal to DQIdMax (the picture parameter set RBSP
becomes the active picture parameter set RBSP) and the access unit does not contain an activating buffering period SEI
message, the subset sequence parameter set RBSP is activated. This subset sequence parameter set RBSP is called the
active SVC sequence parameter set RBSP until it is deactivated when another SVC sequence parameter set RBSP becomes
the active SVC sequence parameter set RBSP. A subset sequence parameter set RBSP, with that particular value of
seq_parameter_set_id, shall be available to the decoding process prior to its activation.


When a subset sequence parameter set RBSP (nal_unit_type is equal to 15) with a particular value of seq_parameter_set_id
is not already the active SVC sequence parameter set RBSP and it is referred to by an activating buffering period SEI
message (using that value of seq_parameter_set_id) that is included in a scalable nesting SEI message, the subset sequence
parameter set RBSP is activated. This subset sequence parameter set RBSP, is called the active SVC sequence parameter
set RBSP until it is deactivated when another SVC sequence parameter set RBSP becomes the active SVC sequence
parameter set RBSP. A subset sequence parameter set RBSP, with that particular value of seq_parameter_set_id, shall be
available to the decoding process prior to its activation.

NOTE 3 – The active SVC sequence parameter set RBSP is either a sequence parameter set RBSP or a subset sequence parameter
set RBSP. Sequence parameter set RBSPs are activated by coded slice NAL units with nal_unit_type equal to 1 or 5 or buffering
period SEI messages that are not included in a scalable nesting SEI message. Subset sequence parameter set RBSPs are activated by
coded slice in scalable extension NAL units (nal_unit_type equal to 20) or buffering period SEI messages that are included in a
scalable nesting SEI message. A sequence parameter set RBSP and a subset sequence parameter set RBSP may have the same value
of seq_parameter_set_id.
NOTE 4 – Buffering period SEI messages have a higher priority for activating SVC sequence parameter sets than coded slice NAL
units. When an SVC sequence parameter set RBSP is referred to by activation of a picture parameter set RBSP inside a particular
access unit and this picture parameter set RBSP is activated by a coded slice NAL unit with DQId equal to DQIdMax (the picture
parameter set RBSP becomes the active picture parameter set RBSP) and this particular access unit also contains an activating
buffering period SEI message that refers to an SVC sequence parameter set RBSP that is different than the SVC sequence parameter
set RBSP referred to by the activation of the picture parameter set RBSP, the SVC sequence parameter set RBSP that is referred to
by the activating buffering period SEI message becomes the active SVC sequence parameter set.
NOTE 5 – Compared to the specifications for profiles specified in Annex A, where an activated sequence parameter set RBSP must
remain active for the entire coded video sequence, the specification for profiles specified in this annex differs. When an SVC
sequence parameter set RBSP is already active (as the active SVC sequence parameter set RBSP), another SVC sequence parameter
set RBSP becomes the active SVC sequence parameter set RBSP in a non-IDR access unit when it is referred to by an activating
buffering period SEI message or by the activation of a picture parameter set RBSP (as the active picture parameter set RBSP). In
this case, the contents of the de-activated and activated SVC sequence parameter set RBSP are mutually restricted as described
below. Hence, within a coded video sequence, multiple successively activated/de-activated SVC sequence parameter set RBSPs can
be present.


For the following specification in this clause, the activating layer buffering period SEI message for a particular value of
DQId is specified as follows:


- If the access unit contains a buffering period SEI messages that is included in a scalable nesting SEI message and is
associated with the particular value of DQId, this buffering period SEI message is the activating layer buffering period
SEI message for the particular value of DQId.


- Otherwise, if the particular value of DQId is equal to 0 and the access unit contains a buffering period SEI message
that is not included in a scalable nesting SEI message, this buffering period SEI message is the activating layer
buffering period SEI message for the particular value of DQId.





- Otherwise, the access unit does not contain an activating layer buffering period SEI message for the particular value
of DQId.


When a sequence parameter set RBSP (nal_unit_type is equal to 7) with a particular value of seq_parameter_set_id is not
already the active layer SVC sequence parameter set RBSP for DQId equal to 0 and it is referred to by activation of a
picture parameter set RBSP (using that value of seq_parameter_set_id) and the picture parameter set RBSP is activated by
a coded slice NAL unit with nal_unit_type equal to 1 or 5 and DQIdMax is greater than 0 (the picture parameter set RBSP
becomes the active layer picture parameter set RBSP for DQId equal to 0) and the access unit does not contain an activating
layer buffering period SEI message for DQId equal to 0, the sequence parameter set RBSP is activated for layer
representations with DQId equal to 0. This sequence parameter set RBSP is called the active layer SVC sequence parameter
set RBSP for DQId equal to 0 until it is deactivated when another SVC sequence parameter set RBSP becomes the active
layer SVC sequence parameter set RBSP for DQId equal to 0 or when decoding an access unit with DQIdMax equal to 0.
A sequence parameter set RBSP, with that particular value of seq_parameter_set_id, shall be available to the decoding
process prior to its activation.


When a sequence parameter set RBSP (nal_unit_type is equal to 7) with a particular value of seq_parameter_set_id is not
already the active layer SVC sequence parameter set RBSP for DQId equal to 0 and it is referred to by an activating layer
buffering period SEI message for DQId equal to 0 (using that value of seq_parameter_set_id) that is not included in a
scalable nesting SEI message and DQIdMax is greater than 0, the sequence parameter set RBSP is activated for layer
representations with DQId equal to 0. This sequence parameter set RBSP is called the active layer SVC sequence parameter
set RBSP for DQId equal to 0 until it is deactivated when another SVC sequence parameter set RBSP becomes the active
layer SVC sequence parameter set RBSP for DQId equal to 0 or when decoding an access unit with DQIdMax equal to 0.
A sequence parameter set RBSP, with that particular value of seq_parameter_set_id, shall be available to the decoding
process prior to its activation.


When a subset sequence parameter set RBSP (nal_unit_type is equal to 15) with a particular value of seq_parameter_set_id
is not already the active layer SVC sequence parameter set RBSP for a particular value of DQId less than DQIdMax and
it is referred to by activation of a picture parameter set RBSP (using that value of seq_parameter_set_id) and the picture
parameter set RBSP is activated by a coded slice in scalable extension NAL unit (nal_unit_type is equal to 20) with the
particular value of DQId (the picture parameter set RBSP becomes the active layer picture parameter set RBSP for the
particular value of DQId) and the access unit does not contain an activating layer buffering period SEI message for the
particular value of DQId, the subset sequence parameter set is activated for layer representations with the particular value
of DQId. This subset sequence parameter set RBSP is called the active layer SVC sequence parameter set RBSP for the
particular value of DQId until it is deactivated when another SVC sequence parameter set RBSP becomes the active layer
SVC sequence parameter set RBSP for the particular value of DQId or when decoding an access unit with DQIdMax less
than or equal to the particular value of DQId. A subset sequence parameter set RBSP, with that particular value of
seq_parameter_set_id, shall be available to the decoding process prior to its activation.


When a subset sequence parameter set RBSP (nal_unit_type is equal to 15) with a particular value of seq_parameter_set_id
is not already the active layer SVC sequence parameter set RBSP for a particular value of DQId less than DQIdMax and
it is referred to by an activating layer buffering period SEI message for the particular value of DQId (using that value of
seq_parameter_set_id) that is included in a scalable nesting SEI message, the subset sequence parameter set RBSP is
activated for layer representations with the particular value of DQId. This subset sequence parameter set RBSP is called
the active layer SVC sequence parameter set RBSP for the particular value of DQId until it is deactivated when another
SVC sequence parameter set RBSP becomes the active layer SVC sequence parameter set RBSP for the particular value
of DQId or when decoding an access unit with DQIdMax less than or equal to the particular value of DQId. A subset
sequence parameter set RBSP, with that particular value of seq_parameter_set_id, shall be available to the decoding
process prior to its activation.


A sequence parameter set RBSP or a subset sequence parameter set RBSP that includes a value of profile_idc not specified
in Annex A or G shall not be referred to by activation of a picture parameter set RBSP as the active picture parameter set
RBSP or as active layer picture parameter set RBSP (using that value of seq_parameter_set_id) or referred to by a buffering
period SEI message (using that value of seq_parameter_set_id). A sequence parameter set RBSP or a subset sequence
parameter set RBSP including a value of profile_idc not specified in Annex A or G is ignored in the decoding for profiles
specified in Annex A or G.


Let spsA and spsB be two SVC sequence parameter set RBSPs with one of the following properties:


- spsA is the SVC sequence parameter set RBSP that is referred to by the coded slice NAL units (via the picture
parameter set) of a layer representation with a particular value of dependency_id and quality_id equal to 0 and spsB
is the SVC sequence parameter set RBSP that is referred to by the coded slice NAL units (via the picture parameter
set) of another layer representation, in the same access unit, with the same value of dependency_id and quality_id
greater than 0,





- spsA is the active SVC sequence parameter set RBSP for an access unit and spsB is the SVC sequence parameter set
RBSP that is referred to by the coded slice NAL units (via the picture parameter set) of the layer representation with
DQId equal to DQIdMax,


- spsA is the active SVC sequence parameter set RBSP for an IDR access unit and spsB is the active SVC sequence
parameter set RBSP for any non-IDR access unit of the same coded video sequence.


The SVC sequence parameter set RBSPs spsA and spsB are restricted with regards to their contents as specified in the
following.


- The values of the syntax elements in the seq_parameter_set_data( ) syntax structure of spsA and spsB may only differ
for the following syntax elements and shall be the same otherwise: profile_idc, constraint_setX_flag (with X being
equal to 0 to 5, inclusive), reserved_zero_2bits, level_idc, seq_parameter_set_id, timing_info_present_flag,
num_units_in_tick, time_scale, fixed_frame_rate_flag, nal_hrd_parameters_present_flag,
vcl_hrd_parameters_present_flag, low_delay_hrd_flag, pic_struct_present_flag, and the hrd_parameters( ) syntax
structures.


- When spsA is the active SVC sequence parameter set RBSP and spsB is the SVC sequence parameter set RBSP that
is referred to by the coded slice NAL units of the layer representation with DQId equal to DQIdMax, the level
specified by level_idc (or level_idc and constraint_set3_flag) in spsA shall not be less than the level specified by
level_idc (or level_idc and constraint_set3_flag) in spsB.


- When the seq_parameter_set_svc_extension( ) syntax structure is present in both spsA and spsB, the values of all
syntax elements in the seq_parameter_set_svc_extension( ) syntax structure shall be the same.


It is a requirement of bitstream conformance that the following constraints are obeyed:


- For each particular value of DQId, all coded slice NAL units of a coded video sequence shall refer to the same value
of seq_parameter_set_id (via the picture parameter set RBSP that is referred to by the value of pic_parameter_set_id).


- The value of seq_parameter_set_id in a buffering period SEI message that is not included in a scalable nesting SEI
message shall be identical to the value of seq_parameter_set_id in the picture parameter set RBSP that is referred to
by coded slice NAL units with nal_unit_type equal to 1 or 5 (via the value of pic_parameter_set_id) in the same access
unit.


- The value of seq_parameter_set_id in a buffering period SEI message that is included in a scalable nesting SEI
message and is associated with a particular value of DQId shall be identical to the value of seq_parameter_set_id in
the picture parameter set RBSP that is referred to by coded slice NAL units with the particular value of DQId (via the
value of pic_parameter_set_id) in the same access unit.


The active layer SVC sequence parameter set RBSPs for different values of DQId may be the same SVC sequence
parameter set RBSP. The active SVC sequence parameter set RBSP and an active layer SVC sequence parameter set RBSP
for a particular value of DQId may be the same SVC sequence parameter set RBSP.


When the active SVC sequence parameter set RBSP for a coded picture is a sequence parameter set RBSP, any sequence
parameter set RBSP with the value of seq_parameter_set_id for the active SVC sequence parameter set RBSP for the coded
picture shall have the same content as that of the active SVC sequence parameter set RBSP for the coded picture unless it
follows the last access unit of the coded video sequence containing the coded picture and precedes the first VCL NAL unit
and the first SEI NAL unit containing a buffering period SEI message (when present) of another coded video sequence.


When the active SVC sequence parameter set RBSP for a coded picture is a subset sequence parameter set RBSP, any
subset sequence parameter set RBSP with the value of seq_parameter_set_id for the active SVC sequence parameter set
RBSP for the coded picture shall have the same content as that of the active SVC sequence parameter set RBSP for the
coded picture unless it follows the last access unit of the coded video sequence containing the coded picture and precedes
the first VCL NAL unit and the first SEI NAL unit containing a buffering period SEI message (when present) of another
coded video sequence.


For each particular value of DQId, the following applies:


- When the active layer SVC sequence parameter set RBSP for a coded picture is a sequence parameter set RBSP, any
sequence parameter set RBSP with the value of seq_parameter_set_id for the active layer SVC sequence parameter
set RBSP for the coded picture shall have the same content as that of the active layer SVC sequence parameter set
RBSP for the coded picture unless it follows the last access unit of the coded video sequence containing the coded
picture and precedes the first VCL NAL unit and the first SEI NAL unit containing a buffering period SEI message
(when present) of another coded video sequence.


- When the active layer SVC sequence parameter set RBSP for a coded picture is a subset sequence parameter set
RBSP, any subset sequence parameter set RBSP with the value of seq_parameter_set_id for the active layer SVC
sequence parameter set RBSP for the coded picture shall have the same content as that of the active layer SVC





sequence parameter set RBSP for the coded picture unless it follows the last access unit of the coded video sequence
containing the coded picture and precedes the first VCL NAL unit and the first SEI NAL unit containing a buffering
period SEI message (when present) of another coded video sequence.
NOTE 6 – If picture parameter set RBSP or SVC sequence parameter set RBSP are conveyed within the bitstream, these constraints
impose an order constraint on the NAL units that contain the picture parameter set RBSP or SVC sequence parameter set RBSP,
respectively. Otherwise (picture parameter set RBSP or SVC sequence parameter set RBSP are conveyed by other means not
specified in this Recommendation | International Standard), they must be available to the decoding process in a timely fashion such
that these constraints are obeyed.


When a sequence parameter set NAL unit with a particular value of seq_parameter_set_id is received, its content replaces
the content of the previous sequence parameter set NAL unit, in decoding order, with the same value of
seq_parameter_set_id (when a previous sequence parameter set NAL unit with the same value of seq_parameter_set_id
was present in the bitstream). When a subset sequence parameter set NAL unit with a particular value of
seq_parameter_set_id is received, its content replaces the content of the previous subset sequence parameter set NAL unit,
in decoding order, with the same value of seq_parameter_set_id (when a previous subset sequence parameter set NAL unit
with the same value of seq_parameter_set_id was present in the bitstream).

NOTE 7 – A decoder must be capable of simultaneously storing the contents of the sequence parameter sets and subset sequence
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
SVC sequence parameter set RBSP. The contents of sequence parameter set extension RBSPs only apply when the base
layer, which conforms to one or more of the profiles specified in Annex A, of a coded video sequence conforming to one
or more of the profiles specified in Annex G is decoded. Subset sequence parameter set RBSPs shall not be followed by a
sequence parameter set extension RBSP.

NOTE 8 – Sequence parameter sets extension RBSPs are not considered to be part of a subset sequence parameter set RBSP and
subset sequence parameter set RBSPs must not be followed by a sequence parameter set extension RBSP.


For layer representations with DQId equal to DQIdMax, all constraints that are expressed on the relationship between the
values of the syntax elements (and the values of variables derived from those syntax elements) in SVC sequence parameter
sets and picture parameter sets and other syntax elements are expressions of constraints that apply only to the active SVC
sequence parameter set and the active picture parameter set. For layer representations with a particular value of DQId less
than DQIdMax, all constraints that are expressed on the relationship between the values of the syntax elements (and the
values of variables derived from those syntax elements) in SVC sequence parameter sets and picture parameter sets and
other syntax elements are expressions of constraints that apply only to the active layer SVC sequence parameter set and
the active layer picture parameter set for the particular value of DQId. If any SVC sequence parameter set RBSP having
profile_idc equal to one of the profile_idc values specified in Annex A or G is present that is never activated in the bitstream
(i.e., it never becomes the active SVC sequence parameter set or an active layer SVC sequence parameter set), its syntax
elements shall have values that would conform to the specified constraints if it were activated by reference in an otherwiseconforming bitstream. If any picture parameter set RBSP is present that is never activated in the bitstream (i.e., it never
becomes the active picture parameter set or an active layer picture parameter set), its syntax elements shall have values
that would conform to the specified constraints if it were activated by reference in an otherwise-conforming bitstream.


During operation of the decoding process (see clause G.8), for layer representations with DQId equal to DQIdMax, the
values of parameters of the active picture parameter set and the active SVC sequence parameter set shall be considered in
effect. For layer representations with a particular value of DQId less than DQIdMax, the values of the parameters of the
active layer picture parameter set and the active layer SVC sequence parameter set for the particular value of DQId shall
be considered in effect. For interpretation of SEI messages that apply to access units or dependency representations with
dependency_id equal to DependencyIdMax or layer representation with DQId equal to DQIdMax, the values of the
parameters of the active picture parameter set and the active SVC sequence parameter set for the access unit shall be
considered in effect unless otherwise specified in the SEI message semantics. For interpretation of SEI messages that apply
to dependency representations with a particular value of dependency_id less than DependencyIdMax, the values of the
parameters of the active layer picture parameter set and the active layer SVC sequence parameter set for the layer
representation with DQId equal to ( dependency_id << 4 ) of the access unit shall be considered in effect unless otherwise
specified in the SEI message semantics. For interpretation of SEI messages that apply to layer representations with a
particular value of DQId less than DQIdMax, the values of the parameters of the active layer picture parameter set and the
active layer SVC sequence parameter set for the layer representation with the particular value of DQId of the access unit
shall be considered in effect unless otherwise specified in the SEI message semantics.





**G.7.4.1.2.2** **Order of access units and association to coded video sequences**


The specification of clause 7.4.1.2.2 applies with the following modifications.


The first access unit of the bitstream shall only contain coded slice NAL units with nal_unit_type equal to 5 or idr_flag
equal to 1.


The order of NAL units and coded pictures and their association to access units is described in clause G.7.4.1.2.3.


**G.7.4.1.2.3** **Order of NAL units and coded pictures and association to access units**


The specification of clause 7.4.1.2.3 applies with the following modifications.

NOTE – Some bitstreams that conform to one or more profiles specified in this annex do not conform to any profile specified in
Annex A (prior to operation of the base layer extraction process specified in clause G.8.8.2). As specified in clauses 7.4.1
and 7.4.1.2.3, for the profiles specified in Annex A, NAL units with nal_unit_type equal to 20 are classified as non-VCL NAL units
that must be preceded within each access unit by at least one NAL unit with nal_unit_type in the range of 1 to 5, inclusive. For this
reason, any bitstream that conforms to one or more profiles specified in this annex does not conform to any profile specified in
Annex A when it contains any of the following:

  - any access unit that does not contain any NAL units with nal_unit_type equal to 1 or 5, but contains one or more NAL units
with nal_unit_type equal to 6, 7, 8, 9, or 15;

  - any access unit in which one or more NAL units with nal_unit_type equal to 7, 8, or 15 is present after the last NAL unit in the
access unit with nal_unit_type equal to 1 or 5.


The association of VCL NAL units to primary or redundant coded pictures is specified in clause G.7.4.1.2.5. When the
primary coded picture does not contain a layer representation with a particular value of DQId, all redundant coded pictures
(when present) in the same access unit shall not contain a layer representation with the particular value of DQId.


The constraints for the detection of the first VCL NAL unit of a primary coded picture are specified in clause G.7.4.1.2.4.


The constraint expressed in clause 7.4.1.2.3 on the order of a buffering period SEI message is replaced by the following
constraints.


- When an SEI NAL unit containing a buffering period SEI message is present, the following applies:


   - If the buffering period SEI message is the only buffering period SEI message in the access unit and it is not
included in a scalable nesting SEI message, the buffering period SEI message shall be the first SEI message
payload of the first SEI NAL unit in the access unit.


   - Otherwise (the buffering period SEI message is not the only buffering period SEI message in the access unit or
it is included in a scalable nesting SEI message), the following constraints are specified:


      - When a buffering period SEI message that is not included in a scalable nesting SEI message is present, this
buffering period SEI message shall be the only SEI message payload of the first SEI NAL unit in the access
unit.


      - A scalable nesting SEI message that includes a buffering period SEI message shall not include any other
SEI messages and the scalable nesting SEI message that includes a buffering period SEI message shall be
the only SEI message inside an SEI NAL unit.


      - All SEI NAL units that precede an SEI NAL unit that contains a scalable nesting SEI message with a
buffering period SEI message as payload in an access unit shall only contain buffering period SEI messages
or scalable nesting SEI messages with a buffering period SEI message as payload.


      - When present, a scalable nesting SEI message with all_layer_representations_in_au_flag equal to 1 and a
buffering period SEI message as payload shall be the first scalable nesting SEI message in an access unit.


      - Any scalable nesting SEI message with a buffering period SEI message as payload that immediately
precedes another scalable nesting SEI message with a buffering period SEI message as payload shall have
values of 128 * sei_dependency_id[ i ] + 8 * sei_quality_id[ i ] + sei_temporal_id, for all present i, that are
less than any of the values of 128 * sei_dependency_id[ i ] + 8 * sei_quality_id[ i ] + sei_temporal_id in the
immediately following scalable nesting SEI message with a buffering period SEI message as payload.


The following additional constraints shall be obeyed:


- Each NAL unit with nal_unit_type equal to 1 or 5 shall be immediately preceded by a prefix NAL unit.


- In bitstreams conforming to this Recommendation | International Standard, each prefix NAL unit shall be
immediately followed by a NAL unit with nal_unit_type equal to 1 or 5.


**G.7.4.1.2.4** **Detection of the first VCL NAL unit of a primary coded picture**


This clause specifies constraints on VCL NAL unit syntax that are sufficient to enable the detection of the first VCL NAL
unit of each primary coded picture.





The first VCL NAL unit of the primary coded picture of the current access unit, in decoding order, shall be different from
the last VCL NAL unit of the primary coded picture of the previous access unit, in decoding order, in one or more of the
following ways:


- dependency_id of the first VCL NAL unit of the primary coded picture of the current access unit is less than
dependency_id of the last VCL NAL unit of the primary coded picture of the previous access unit


- dependency_id of the first VCL NAL unit of the primary coded picture of the current access unit is equal to
dependency_id of the last VCL NAL unit of the primary coded picture of the previous access unit and any of the
following conditions are true


   - quality_id of the first VCL NAL unit of the primary coded picture of the current access unit is less than quality_id
of the last VCL NAL unit of the primary coded picture of the previous access unit


   - quality_id of the first VCL NAL unit of the primary coded picture of the current access unit and the last VCL
NAL unit of the primary coded picture of the previous access unit is equal to 0, and any of the conditions
specified in clause 7.4.1.2.4 is fulfilled


**G.7.4.1.2.5** **Order of VCL NAL units and association to coded pictures**


Each VCL NAL unit is part of a coded picture.


Let dId be the value of dependency_id and let qId be the value of quality_id of any particular VCL NAL unit. The order
of the VCL NAL units within a coded picture is constrained as follows:


- For all VCL NAL units following this particular VCL NAL unit, the value of dependency_id shall be greater than or
equal to dId.


- For all VCL NAL units with a value of dependency_id equal to dId following this particular VCL NAL unit, the value
of quality_id shall be greater than or equal to qId.


For each set of VCL NAL units within a layer representation, the following applies:


- If arbitrary slice order, as specified in Annex A or clause G.10, is allowed, coded slice NAL units of a layer
representation may have any order relative to each other.


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


**G.7.4.2** **Raw byte sequence payloads and RBSP trailing bits semantics**


**G.7.4.2.1** **Sequence parameter set RBSP semantics**


The semantics specified in clause 7.4.2.1 apply.


**G.7.4.2.1.1** **Sequence parameter set data semantics**


The semantics specified in clause 7.4.2.1.1 apply with substituting SVC sequence parameter set for sequence parameter
set. Additionally, the following applies.


**profile_idc** and **level_idc** indicate the profile and level to which the coded video sequence conforms when the SVC
sequence parameter set is the active SVC sequence parameter set.


**constraint_set0_flag** is specified as follows:

- If the sequence parameter set data syntax structure is included in a sequence parameter set RBSP, the semantics
specified in clause 7.4.2.1.1 apply.

- Otherwise (the sequence parameter set data syntax structure is included in a subset sequence parameter set RBSP),
constraint_set0_flag equal to 1 specifies that all of the following conditions are obeyed:

   - the coded video sequence obeys all constraints specified in clause G.10.1.1,





   - the output of the decoding process as specified in clause G.8 is identical to the output of the decoding process
that is obtained when profile_idc would be set equal to 83.

constraint_set0_flag equal to 0 specifies that the coded video sequence may or may not obey all constraints specified
in clause G.10.1.1 and that the output of the decoding process as specified in clause G.8 may or may not be identical
to the output of the decoding process that is obtained when profile_idc would be set equal to 83.

NOTE 1 – The output of the decoding process may be different, if the array sTCoeff contains non-zero scaled luma
transform coefficient values for a transform block of a macroblock that is coded in an Inter macroblock prediction mode,
but all reconstructed luma residual samples of the array rSL that are associated with the transform blocks are equal to 0.
In this case, the boundary filter strength that is derived as specified in clause G.8.7.4.3 can depend on the value of
profile_idc.


**constraint_set1_flag** is specified as follows:

- If the sequence parameter set data syntax structure is included in a sequence parameter set RBSP, the semantics
specified in clause 7.4.2.1.1 apply.

- Otherwise (the sequence parameter set data syntax structure is included in a subset sequence parameter set RBSP),
constraint_set1_flag equal to 1 specifies that all of the following conditions are obeyed:

   - the coded video sequence obeys all constraints specified in clause G.10.1.2,

   - the output of the decoding process as specified in clause G.8 is identical to the output of the decoding process
that is obtained when profile_idc would be set equal to 86.

constraint_set1_flag equal to 0 specifies that the coded video sequence may or may not obey all constraints specified
in clause G.10.1.2 and that the output of the decoding process as specified in clause G.8 may or may not be identical
to the output of the decoding process that is obtained when profile_idc would be set equal to 86.

NOTE 2 – The output of the decoding process may be different, if the array sTCoeff contains non-zero scaled luma
transform coefficient values for a transform block of a macroblock that is coded in an Inter macroblock prediction mode,
but all reconstructed luma residual samples of the array rSL that are associated with the transform blocks are equal to 0.
In this case, the boundary filter strength that is derived as specified in clause G.8.7.4.3 can depend on the value of
profile_idc.


**constraint_set2_flag** is specified as follows:

- If the sequence parameter set data syntax structure is included in a sequence parameter set RBSP, the semantics
specified in clause 7.4.2.1.1 apply.

- Otherwise (the sequence parameter set data syntax structure is included in a subset sequence parameter set RBSP),
the value of 1 for constraint_set2_flag is reserved for future use by ITU-T | ISO/IEC. constraint_set2_flag shall be
equal to 0 for coded video sequences with profile_idc equal to 83 and 86 in bitstreams conforming to this
Recommendation | International Standard. Decoders shall ignore the value of constraint_set2_flag when profile_idc
is equal to 83 or 86.


**constraint_set3_flag** is specified as follows:

- If the sequence parameter set data syntax structure is included in a sequence parameter set RBSP, the semantics
specified in clause 7.4.2.1.1 apply.

- Otherwise (the sequence parameter set data syntax structure is included in a subset sequence parameter set RBSP),
the following applies:

   - If profile_idc is equal to 86, constraint_set3_flag equal to 1 specifies that the coded video sequence obeys all
constraints specified in clause G.10.1.3, and constraint_set3_flag equal to 0 specifies that the coded video
sequence may or may not obey these corresponding constraints.

   - Otherwise (profile_idc is not equal to 86), the value of 1 for constraint_set3_flag is reserved for future use by
ITU-T | ISO/IEC. constraint_set3_flag shall be equal to 0 for coded video sequences with profile_idc not equal
to 86 in bitstreams conforming to this Recommendation | International Standard. Decoders shall ignore the value
of constraint_set3_flag when profile_idc is not equal to 86.


**constraint_set5_flag** is specified as follows:

- If the sequence parameter set data syntax structure is included in a sequence parameter set RBSP, the semantics
specified in clause 7.4.2.1.1 apply.

- Otherwise (the sequence parameter set data syntax structure is included in a subset sequence parameter set RBSP),
the following applies:

`–` If the profile_idc is equal to 83, constraint_set5_flag equal to 1 specifies that the coded video sequence obeys all
constraints specified in clause G.10.1.1.1.

`–` Otherwise, if the profile_idc is equal to 86, constraint_set5_flag equal to 1 specifies that the coded video
sequence obeys all constraints specified in clause G.10.1.2.1.





`–` Otherwise (profile_idc is not equal to 83 or 86), the value of 1 for constraint_set5_flag is reserved for future use
by ITU-T | ISO/IEC. constraint_set5_flag shall be equal to 0 for coded video sequences with profile_idc not
equal to 83 or 86 in bitstreams conforming to this Recommendation | International Standard. Decoders shall
ignore the value of constraint_set5_flag when profile_idc is not equal to 83 or 86.


The value of separate_colour_plane_flag shall be equal to 0 and the value of qpprime_y_zero_transform_bypass_flag shall
be equal to 0.


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


When the seq_parameter_set_data( ) syntax structure is present in a sequence parameter set RBSP and
vui_parameters_present_flag is equal to 1, the values of timing_info_present_flag, num_units_in_tick, time_scale,
fixed_frame_rate_flag, nal_hrd_parameters_present_flag, vcl_hrd_parameters_present_flag, low_delay_hrd_flag,
pic_struct_present_flag and the values of syntax elements included in the hrd_parameters( ) syntax structures, when
present, shall be such that the bitstream activating the sequence parameter set is conforming to one or more of the profiles
specified in Annex A.


**max_num_ref_frames** specifies the maximum number of short-term and long-term reference frames, complementary
reference field pairs, and non-paired reference fields that may be used by the decoding process for inter prediction of any
picture in the coded video sequence. max_num_ref_frames also determines the size of the sliding window operation as
specified in clause G.8.2.4.2. The value of max_num_ref_frames shall be in the range of 0 to MaxDpbFrames (as specified
in clause G.10), inclusive.


The allowed range of values for pic_width_in_mbs_minus1, pic_height_in_map_units_minus1, and frame_mbs_only_flag
is specified by constraints in clause G.10.


**G.7.4.2.1.1.1** **Scaling list semantics**


The semantics specified in clause 7.4.2.1.1.1 apply.


**G.7.4.2.1.2** **Sequence parameter set extension RBSP semantics**


The semantics specified in clause 7.4.2.1.2 apply. Additionally, the following applies.


Sequence parameter set extension RBSPs can only follow sequence parameter set RBSPs in decoding order. Subset
sequence parameter set RBSPs shall not be followed by a sequence parameter set extension RBSP. The contents of
sequence parameter set extension RBSPs only apply when the base layer, which conforms to one or more of the profiles
specified in Annex A, of a coded video sequence conforming to one or more of the profiles specified in Annex G is
decoded.


**G.7.4.2.1.3** **Subset sequence parameter set RBSP semantics**


The semantics specified in clause 7.4.2.1.3 apply.


**G.7.4.2.1.4** **Sequence parameter set SVC extension semantics**


**inter_layer_deblocking_filter_control_present_flag** equal to 1 specifies that a set of syntax elements controlling the
characteristics of the deblocking filter for inter-layer prediction is present in the slice header.
inter_layer_deblocking_filter_control_present_flag equal to 0 specifies that the set of syntax elements controlling the
characteristics of the deblocking filter for inter-layer prediction is not present in the slice headers and their inferred values
are in effect.


**extended_spatial_scalability_idc** specifies the presence of syntax elements related to geometrical parameters for the
resampling processes. The value of extended_spatial_scalability_idc shall be in the range of 0 to 2, inclusive, and the
following applies:


- If extended_spatial_scalability_idc is equal to 0, no geometrical parameters are present in the subset sequence
parameter set and the slice headers referring to this subset sequence parameter set.


- Otherwise, if extended_spatial_scalability_idc is equal to 1, geometrical parameters are present in the subset sequence
parameter set, but not in the slice headers referring to this subset sequence parameter set.





- Otherwise (extended_spatial_scalability_idc is equal to 2), geometrical parameters are not present in the subset
sequence parameter set, but they are present in the slice headers with no_inter_layer_pred_flag equal to 0 and
quality_id equal to 0 that refer to this subset sequence parameter set.


**chroma_phase_x_plus1_flag** specifies the horizontal phase shift of the chroma components in units of half luma samples
of a frame or layer frame. When chroma_phase_x_plus1_flag is not present, it shall be inferred to be equal to 1.


When ChromaArrayType is equal to 1 and chroma_sample_loc_type_top_field and
chroma_sample_loc_type_bottom_field are present, the following applies:


- If chroma_phase_x_plus1_flag is equal to 0, chroma_sample_loc_type_top_field and
chroma_sample_loc_type_bottom_field should be equal to 0, 2, or 4.


- Otherwise (chroma_phase_x_plus1_flag is equal to 1), chroma_sample_loc_type_top_field and
chroma_sample_loc_type_bottom_field should be equal to 1, 3, or 5.


When ChromaArrayType is equal to 2, chroma_phase_x_plus1_flag should be equal to 1.


**chroma_phase_y_plus1** specifies the vertical phase shift of the chroma components in units of half luma samples of a
frame or layer frame. When chroma_phase_y_plus1 is not present, it shall be inferred to be equal to 1. The value of
chroma_phase_y_plus1 shall be in the range of 0 to 2, inclusive.


When ChromaArrayType is equal to 1 and chroma_sample_loc_type_top_field and
chroma_sample_loc_type_bottom_field are present, the following applies:


- If chroma_phase_y_plus1 is equal to 0, chroma_sample_loc_type_top_field and
chroma_sample_loc_type_bottom_field should be equal to 2 or 3.


- Otherwise, if chroma_phase_y_plus1 is equal to 1, chroma_sample_loc_type_top_field and
chroma_sample_loc_type_bottom_field should be equal to 0 or 1.


- Otherwise (chroma_phase_y_plus1 is equal to 2), chroma_sample_loc_type_top_field and
chroma_sample_loc_type_bottom_field should be equal to 4 or 5.


**seq_ref_layer_chroma_phase_x_plus1_flag** specifies the horizontal phase shift of the chroma components in units of
half luma samples of a layer frame for the layer pictures that may be used for inter-layer prediction. When
seq_ref_layer_chroma_phase_x_plus1_flag is not present, it shall be inferred to be equal to chroma_phase_x_plus1_flag.


**seq_ref_layer_chroma_phase_y_plus1** specifies the vertical phase shift of the chroma components in units of half luma
samples of a layer frame for the layer pictures that may be used for inter-layer prediction. When
seq_ref_layer_chroma_phase_y_plus1 is not present, it shall be inferred to be equal to chroma_phase_y_plus1. The value
of seq_ref_layer_chroma_phase_y_plus1 shall be in the range of 0 to 2, inclusive.


**seq_scaled_ref_layer_left_offset** specifies the horizontal offset between the upper-left luma sample of a resampled layer
picture used for inter-layer prediction and the upper-left luma sample of the current picture or current layer picture in units
of two luma samples. When seq_scaled_ref_layer_left_offset is not present, it shall be inferred to be equal to 0. The value
of seq_scaled_ref_layer_left_offset shall be in the range of −2 [15] to 2 [15] - 1, inclusive.


**seq_scaled_ref_layer_top_offset** specifies the vertical offset between the upper-left luma sample of a resampled layer
picture used for inter-layer prediction and the upper-left luma sample of the current picture or current layer picture.
Depending on the value of frame_mbs_only_flag, the following applies:


- If frame_mbs_only_flag is equal to 1, the vertical offset is specified in units of two luma samples.


- Otherwise (frame_mbs_only_flag is equal to 0), the vertical offset is specified in units of four luma samples.


When seq_scaled_ref_layer_top_offset is not present, it shall be inferred to be equal to 0. The value of
seq_scaled_ref_layer_top_offset shall be in the range of −2 [15] to 2 [15] - 1, inclusive.


**seq_scaled_ref_layer_right_offset** specifies the horizontal offset between the bottom-right luma sample of a resampled
layer picture used for inter-layer prediction and the bottom-right luma sample of the current picture or current layer picture
in units of two luma samples. When seq_scaled_ref_layer_right_offset is not present, it shall be inferred to be equal to 0.
The value of seq_scaled_ref_layer_right_offset shall be in the range of −2 [15] to 2 [15] - 1, inclusive.


**seq_scaled_ref_layer_bottom_offset** specifies the vertical offset between the bottom-right luma sample of a resampled
layer picture used for inter-layer prediction and the bottom-right luma sample of the current picture or current layer picture.
Depending on the value of frame_mbs_only_flag, the following applies:


- If frame_mbs_only_flag is equal to 1, the vertical offset is specified in units of two luma samples.


- Otherwise (frame_mbs_only_flag is equal to 0), the vertical offset is specified in units of four luma samples.





When seq_scaled_ref_layer_bottom_offset is not present, it shall be inferred to be equal to 0. The value of
seq_scaled_ref_layer_bottom_offset shall be in the range of −2 [15] to 2 [15] - 1, inclusive.


**seq_tcoeff_level_prediction_flag** specifies the presence of the syntax element adaptive_tcoeff_level_prediction_flag in
the subset sequence parameter set.


**adaptive_tcoeff_level_prediction_flag** specifies the presence of tcoeff_level_prediction_flag in slice headers that refer to
the subset sequence parameter set. When adaptive_tcoeff_level_prediction_flag is not present, it shall be inferred to be
equal to 0.


**slice_header_restriction_flag** specifies the presence of syntax elements in slice headers that refer to the subset sequence
parameter set.


**G.7.4.2.2** **Picture parameter set RBSP semantics**


The semantics specified in clause 7.4.2.2 apply with substituting "SVC sequence parameter set" for "sequence parameter
set" and substituting "active SVC sequence parameter set or active layer SVC sequence parameter set" for "active sequence
parameter set". Additionally, the following applies.


**num_slice_groups_minus1** plus 1 specifies the number of slice groups for a picture. When num_slice_groups_minus1 is
equal to 0, all slices of the picture belong to the same slice group. The allowed range of num_slice_groups_minus1 is
specified in clause G.10.


**G.7.4.2.3** **Supplemental enhancement information RBSP semantics**


The semantics specified in clause 7.4.2.3 apply.


**G.7.4.2.3.1** **Supplemental enhancement information message semantics**


The semantics specified in clause 7.4.2.3.1 apply.


**G.7.4.2.4** **Access unit delimiter RBSP semantics**


The semantics specified in clause 7.4.2.4 apply.

NOTE – The value of primary_pic_type applies to the slice_type values in all slice headers of the primary coded picture, including
the slice_type syntax elements in all NAL units with nal_unit_type equal to 1, 5, or 20. NAL units with nal_unit_type equal to 2 are
not present in bitstreams conforming to any of the profiles specified in this annex.


**G.7.4.2.5** **End of sequence RBSP semantics**


The end of sequence RBSP specifies that the next subsequent access unit in the bitstream in decoding order (if any) shall
be an access unit for which all layer representation of the primary coded picture have IdrPicFlag equal to 1. The syntax
content of the SODB and RBSP for the end of sequence RBSP are empty. No normative decoding process is specified for
an end of sequence RBSP.


**G.7.4.2.6** **End of stream RBSP semantics**


The semantics specified in clause 7.4.2.6 apply.


**G.7.4.2.7** **Filler data RBSP semantics**


The semantics specified in clause 7.4.2.7 apply with the following addition.


Filler data NAL units shall be considered to contain the syntax elements dependency_id, quality_id, temporal_id, and
priority_id with values that are inferred as follows:


1. Let prevSvcNalUnit be the most recent NAL unit in decoding order that has nal_unit_type equal to 14 or 20.

NOTE – The most recent NAL unit in decoding order with nal_unit_type equal to 14 or 20 always belongs to the same access
unit as the filler data NAL unit.


2. The values of dependency_id, quality_id, temporal_id, and priority_id for the filler data NAL unit are inferred to

be equal to the values of dependency_id, quality_id, temporal_id, and priority_id, respectively, of the NAL unit
prevSvcNalUnit.


**G.7.4.2.8** **Slice layer without partitioning RBSP semantics**


The semantics specified in clause 7.4.2.8 apply.


**G.7.4.2.9** **Slice data partition RBSP semantics**


Slice data partition syntax is not present in bitstreams conforming to any of the profiles specified in Annex G.


**G.7.4.2.10RBSP slice trailing bits semantics**


The semantics specified in clause 7.4.2.10 apply with the following modifications.





Let NumBytesInVclNALunits be the sum of the values of NumBytesInNALunit for all VCL NAL units of a layer
representation and let BinCountsInNALunits be the number of times that the parsing process function DecodeBin( ),
specified in clause 9.3.3.2, is invoked to decode the contents of all VCL NAL units of the layer representation. When
entropy_coding_mode_flag is equal to 1, it is a requirement of bitstream conformance that BinCountsInNALunits shall
not exceed ( 32 ÷ 3 ) * NumBytesInVclNALunits + ( RawMbBits * PicSizeInMbs ) ÷ 32.

NOTE – The constraint on the maximum number of bins resulting from decoding the contents of the slice layer NAL units of a layer
representation can be met by inserting a number of cabac_zero_word syntax elements to increase the value of
NumBytesInVclNALunits. Each cabac_zero_word is represented in a NAL unit by the three-byte sequence 0x000003 (as a result of
the constraints on NAL unit contents that result in requiring inclusion of an emulation_prevention_three_byte for each
cabac_zero_word).


**G.7.4.2.11RBSP trailing bits semantics**


The semantics specified in clause 7.4.2.11 apply.


**G.7.4.2.12Prefix NAL unit RBSP semantics**


The semantics specified in clause 7.4.2.12 apply.


**G.7.4.2.12.1** **Prefix NAL unit SVC semantics**


The syntax element store_ref_base_pic_flag is considered as if it was present in the associated NAL unit.


**store_ref_base_pic_flag** equal to 1 specifies that, when the value of dependency_id as specified in the NAL unit header
is equal to the maximum value of dependency_id for the VCL NAL units of the current coded picture, an additional
representation of the coded picture that may or may not be identical to the decoded picture is marked as "used for
reference". This additional representation is also referred to as reference base picture and may be used for inter prediction
of following pictures in decoding order, but it is not output. When store_ref_base_pic_flag is not present, it shall be inferred
to be equal to 0.


The syntax element store_ref_base_pic_flag shall have the same value for all VCL NAL units of a dependency
representation. When nal_ref_idc is equal to 0, store_ref_base_pic_flag shall be equal to 0.


When max_num_ref_frames is less than 2 in the SVC sequence parameter set that is referred to by the associated NAL
unit, store_ref_base_pic_flag shall be equal to 0.


**additional_prefix_nal_unit_extension_flag** equal to 0 specifies that the prefix_nal_unit_svc( ) syntax structure does not
contain any additional_prefix_nal_unit_extension_data_flag syntax elements. additional_prefix_nal_unit_extension_flag
shall be equal to 0 in bitstreams conforming to this Recommendation | International Standard. The value of 1 for
additional_prefix_nal_unit_extension_flag is reserved for future use by ITU-T | ISO/IEC. Decoders shall ignore all data
that follow the value 1 for additional_prefix_nal_unit_extension_flag in a prefix_nal_unit_svc( ) syntax structure.


**additional_prefix_nal_unit_extension_data_flag** may have any value.

NOTE – The syntax elements additional_prefix_nal_unit_extension_flag and additional_prefix_nal_unit_extension_data_flag are
not used by the decoding process specified in this Recommendation | International Standard.


**G.7.4.2.13Slice layer extension RBSP semantics**


The semantics specified in clause 7.4.2.13 apply.


**G.7.4.3** **Slice header semantics**


The semantics specified in clause 7.4.3 apply with the following modifications.


a) All referenced syntax elements and variables are syntax elements and variables for the dependency representation

with dependency_id equal to 0.


b) A frame, field, top field, bottom field, picture, and decoded picture is interpreted as layer frame, layer field, layer

top field, layer bottom field, layer picture, and decoded layer picture, respectively, that represent an intermediate
decoding result for the dependency representation with dependency_id equal to 0.


c) An IDR picture is interpreted as layer picture with IdrPicFlag equal to 1 for the dependency representation with

dependency_id equal to 0.


d) An IDR access unit is interpreted as an access unit containing a primary coded picture with IdrPicFlag equal to 1

for the dependency representation with dependency_id equal to 0.


e) A reference frame, reference field, and reference picture is interpreted as layer frame, layer field, and layer picture

with nal_ref_idc greater than 0 for the dependency representation with dependency_id equal to 0.


f) A non-reference frame, non-reference field, and non-reference picture is interpreted as layer frame, layer field,
and layer picture with nal_ref_idc equal to 0 for the dependency representation with dependency_id equal to 0.





g) All constraints specified in clause 7.4.3 apply only to layer representations with DQId equal to 0.


h) The slice_header( ) syntax structure shall be considered to contain the following syntax elements with the

following inferred values:


     - ref_layer_dq_id is inferred to be equal to −1.


     - scan_idx_start is inferred to be equal to 0.


     - scan_idx_end is inferred to be equal to 15.


i) References to the decoded reference picture marking process as specified in clause 8.2.5 are replaced with
reference to the SVC decoded reference picture marking process as specified in clause G.8.2.4.


j) The value of direct_spatial_mv_pred_flag shall be equal to 1.


k) The variable MaxRefLayerDQId is set equal to −1.


l) The variable CroppingChangeFlag is set equal to 0.


m) The variable SpatialResolutionChangeFlag is set equal to 0.


n) In the semantics of first_mb_in_slice, the reference to Annex A is substituted with a reference to clause G.10.


**G.7.4.3.1** **Reference picture list modification semantics**


The semantics specified in clause 7.4.3.1 apply. For this specification, the modifications a) to f) specified in clause G.7.4.3
apply. When quality_id is greater than 0, all syntax elements of the ref_pic_list_modification( ) syntax structure are
inferred as specified in the beginning of clause G.7.4.3.4.


**G.7.4.3.2** **Prediction weight table semantics**


The semantics specified in clause 7.4.3.2 apply. When quality_id is greater than 0, all syntax elements of the
pred_weight_table( ) syntax structure are inferred as specified in the beginning of clause G.7.4.3.4.


**G.7.4.3.3** **Decoded reference picture marking semantics**


The semantics specified in clause 7.4.3.3 apply with substituting "SVC sequence parameter set" for "sequence parameter
set" and with considering the reference pictures marked as "reference base pictures" as not present. The constraints
specified in clause 7.4.3.3 apply only to the dependency representation with dependency_id equal to the current value of
dependency_id and the modifications a) and b) specified in clause G.8.2 apply with currDependencyId being equal to the
current value of dependency_id.


When quality_id is greater than 0, all syntax elements of the dec_ref_pic_marking( ) syntax structure are inferred as
specified in the beginning of clause G.7.4.3.4.


In addition to the constraints specified in clause 7.4.3.3, the following constraints are specified:


a) When decoding a frame, the dec_ref_pic_marking( ) syntax structure shall not contain a
memory_management_control_operation command equal to 3 that assigns a long-term frame index to a
complementary reference field pair (not marked as "reference base picture") when any of the following conditions
are true (when processing the memory_management_control_operation command equal to 3):


     - there exists a non-paired reference base field (marked as "reference base picture") that is associated with one
of the fields of the complementary reference field pair and that is marked as "used for reference",


     - there exists a complementary reference base field pair (marked as "reference base picture") that is associated
with the complementary reference field pair and in which one field is marked as "used for reference" and
the other field is marked as "unused for reference".


b) When decoding a field, the dec_ref_pic_marking( ) syntax structure shall not contain a
memory_management_control_operation command equal to 3 that assigns a long-term frame index to a field (not
marked as "reference base picture") of a reference frame or a complementary reference field pair when both of
the following conditions are true (when processing the memory_management_control_operation command equal
to 3):


     - the other field of the reference frame or complementary reference field pair is marked as "unused for
reference",


     - there exists a reference base frame or a complementary reference base field pair (marked as "reference base
picture") that is associated with the reference frame or complementary reference field pair, respectively, and
in which both fields are marked as "used for reference".





c) When decoding the second field (in decoding order) of a complementary reference field pair, the

dec_ref_pic_marking( ) syntax structure shall not contain a memory_management_control_operation command
equal to 6 that assigns a long-term frame index to this field when both of the following conditions are true:


     - there exists a reference base field (marked as "reference base picture") that is associated with the first field
of the complementary reference field pair and that is marked as "used for short-term reference" when the
memory_management_control_operation command equal to 6 is processed,


     - the dec_ref_pic_marking( ) syntax structure does not contain a memory_management_control_operation
command equal to 3 that assigns the same long-term frame index to the first field of the complementary
reference field pair.
NOTE – The additional constraints specified above (in connection with the constraints specified in clause 7.4.3.3) ensure that after
processing all memory_management_control_operation commands of the decoded reference picture marking syntax structure the
following applies, with reference entry being a collective term for a non-paired reference field, a reference frame, or a complementary
reference field pair (not marked as "reference base picture") and reference base entry being a collective term for a non-paired
reference base field, a reference base frame, or a complementary reference base field pair (marked as "reference base picture"):
When one or more fields of a reference entry are marked as "used for reference" and there exists a reference base entry that is
associated with the reference entry or one field of the reference entry and one or more fields of the reference base entry are marked
as "used for reference", either all fields of the reference entry and the reference base entry that are marked as "used for reference"
must be marked as "used for short-term reference" or all fields of the reference entry and the reference base entry that are marked as
"used for reference" must be marked as "used for long-term reference". When these fields are marked as "used for long-term
reference", the same value of long-term frame index must be assigned to all fields of the reference entry and the reference base entry
that are marked as "used for reference".


**G.7.4.3.4** **Slice header in scalable extension semantics**


Unless stated otherwise, for all references to clause 7.4.3 inside this clause, the following modifications apply.


a) All referenced syntax elements and variables are syntax elements and variables for the dependency representation

with dependency_id equal to the current value of dependency_id.


b) A frame, field, top field, bottom field, picture, and decoded picture is interpreted as layer frame, layer field, layer

top field, layer bottom field, layer picture, and decoded layer picture, respectively, that represent an intermediate
decoding result for the dependency representation with dependency_id equal to the current value of
dependency_id.


c) An IDR picture is interpreted as layer picture with IdrPicFlag equal to 1 for the dependency representation with

dependency_id equal to the current value of dependency_id.


d) An IDR access unit is interpreted as an access unit containing a primary coded picture with IdrPicFlag equal to 1

for the dependency representation with dependency_id equal to the current value of dependency_id.


e) A reference frame, reference field, and reference picture is interpreted as layer frame, layer field, and layer picture

with nal_ref_idc greater than 0 for the dependency representation with dependency_id equal to the current value
of dependency_id.


f) A non-reference frame, non-reference field, and non-reference picture is interpreted as layer frame, layer field,
and layer picture with nal_ref_idc equal to 0 for the dependency representation with dependency_id equal to the
current value of dependency_id.


g) References to the decoded reference picture marking process as specified in clause 8.2.5 are replaced with

reference to the SVC decoded reference picture marking process as specified in clause G.8.2.4.


When quality_id is greater than 0, the following syntax elements (which are not present) shall be inferred to be equal to
the corresponding syntax elements of the slice header of the slice with dependency_id equal to the current value of
dependency_id and quality_id equal to 0, in the same coded picture, that covers the macroblock with the macroblock
address (first_mb_in_slice * ( 1 + MbaffFrameFlag )), when present in this slice: direct_spatial_mv_pred_flag,
num_ref_idx_active_override_flag, num_ref_idx_l0_active_minus1, num_ref_idx_l1_active_minus1, all syntax elements
of the syntax structure ref_pic_list_modification( ), base_pred_weight_table_flag, all syntax elements of the syntax
structure pred_weight_table( ), all syntax elements of the syntax structure dec_ref_pic_marking( ), all syntax elements of
the syntax structure dec_ref_base_pic_marking( ), and store_ref_base_pic_flag.


The value of the following SVC sequence parameter set syntax elements shall be the same across all coded slice NAL units
of an access unit: bit_depth_luma_minus8, bit_depth_chroma_minus8, and chroma_format_idc.


The value of the following picture parameter set syntax elements shall be the same across all coded slice NAL units of a
dependency representation: bottom_field_pic_order_in_frame_present_flag, num_ref_idx_l0_default_active_minus1,
num_ref_idx_l1_default_active_minus1, weighted_pred_flag, and weighted_bipred_idc.


For all coded slice NAL units of a coded picture in which the syntax element field_pic_flag is present, field_pic_flag shall
have the same value.





For all coded slice NAL units of a coded picture in which the syntax element bottom_field_flag is present,
bottom_field_flag shall have the same value.


When present in any coded slice NAL unit of a dependency representation, the value of the following slice header syntax
elements shall be the same across all slices of the dependency representation including slice headers of NAL units with
nal_unit_type equal to 1 or 5: frame_num, idr_pic_id, pic_order_cnt_lsb, delta_pic_order_cnt_bottom,
delta_pic_order_cnt[ 0 ], and delta_pic_order_cnt[ 1 ].


When present, the value of the following slice header syntax elements shall be the same across all slices of a layer
representation: pic_parameter_set_id, ref_layer_dq_id, disable_inter_layer_deblocking_filter_idc,
inter_layer_slice_alpha_c0_offset_div2, inter_layer_slice_beta_offset_div2, constrained_intra_resampling_flag,
ref_layer_chroma_phase_x_plus1_flag, ref_layer_chroma_phase_y_plus1, scaled_ref_layer_left_offset,
scaled_ref_layer_top_offset, scaled_ref_layer_right_offset, scaled_ref_layer_bottom_offset, slice_group_change_cycle,
store_ref_base_pic_flag, tcoeff_level_prediction_flag, and all syntax elements of the syntax structures
dec_ref_pic_marking( ) and dec_ref_base_pic_marking( ).


Let setOfRefLayerSlices be the set of slices with dependency_id equal to the current value of dependency_id and quality_id
equal to 0, inside the current coded picture, that are covered or partly covered by the macroblocks of the current slice.


When quality_id is greater than 0, the value of (slice_type % 5) for all slices in the set setOfRefLayerSlices shall be the
same as the value of (slice_type % 5) for the current slice.


When quality_id is greater than 0 and setOfRefLayerSlices contains more than one slice, the following constraints shall be
obeyed:


a) When slice_type specifies an EP or EB slice, the value of num_ref_idx_l0_active_minus1 (either the value

transmitted in the slice header when num_ref_idx_active_override_flag is equal to 1 or the inferred value when
num_ref_idx_active_override_flag is equal to 0) shall be the same across all slices of the set setOfRefLayerSlices.


b) When slice_type specifies an EB slice, the value of num_ref_idx_l1_active_minus1 (either the value transmitted

in the slice header when num_ref_idx_active_override_flag is equal to 1 or the inferred value when
num_ref_idx_active_override_flag is equal to 0) shall be the same across all slices of the set setOfRefLayerSlices.


c) All elements of the syntax structure ref_pic_list_modification( ) shall be the same across all slices of the set

setOfRefLayerSlices.


d) When slice_type specifies an EP slice, the following applies:


i) When weighted_pred_flag is equal to 1, the value of base_pred_weight_table_flag shall be the same across

all slices of the set setOfRefLayerSlices.


ii) When weighted_pred_flag is equal to 1 and pred_weight_table( ) is present in the slices of the set

setOfRefLayerSlices, the values of all syntax elements inside the syntax structure pred_weight_table( ) shall
be the same across all slices of the set setOfRefLayerSlices.


e) When slice_type specifies an EB slice, the following applies:


i) When weighted_bipred_idc is equal to 1, the value of base_pred_weight_table_flag shall be the same across

all slices of the set setOfRefLayerSlices.


ii) When weighted_bipred_idc is equal to 1 and pred_weight_table( ) is present in the slices of the set

setOfRefLayerSlices, the values of all syntax elements inside the syntax structure pred_weight_table( ) shall
be the same across all slices of the set setOfRefLayerSlices.


**first_mb_in_slice** has the same semantics as specified in clause 7.4.3 with the term current picture being substituted by
the term current layer representation and with the reference to Annex A being substituted by a reference to clause G.10.


**slice_type** specifies the coding type of the slice according to Table G-1.


**Table G-1 – Name association to slice_type for NAL units with nal_unit_type equal to 20**

|slice_type|Name of slice_type|
|---|---|
|0, 5|EP (P slice in scalable extension)|
|1, 6|EB (B slice in scalable extension)|
|2, 7|EI (I slice in scalable extension)|



When slice_type has a value in the range 5..7, it is a requirement of bitstream conformance that all other slices of the
current layer representation shall have a value of slice_type equal to the current value of slice_type or equal to the current
value of slice_type minus 5.





NOTE 1 – Values of slice_type in the range 5..7 can be used by an encoder to indicate that all slices of a layer representation have
the same value of (slice_type % 5). Values of slice_type in the range 5..7 are otherwise equivalent to corresponding values in the
range 0..2.


When idr_flag is equal to 1 or max_num_ref_frames is equal to 0, slice_type shall be equal to 2 or 7.


In the text (in particular when the clauses 7 to 9 are referenced in this annex), slices with ( slice_type % 5 ) equal to 0, 1,
and 2 may be collectively referred to as P, B, and I slices, respectively, regardless of whether the slices are coded using
NAL units with nal_unit_type equal to 20 (slice_type is present in the slice_header_in_scalable_extension( ) syntax
structure) or NAL units with nal_unit_type in the range of 1 to 5, inclusive (slice_type is present in the slice_header( )
syntax structure).


**pic_parameter_set_id** has the same semantics as specified in clause 7.4.3.


**colour_plane_id** has the same semantics as specified in clause 7.4.3.


**frame_num** is used as an identifier for dependency representations and shall be represented by
log2_max_frame_num_minus4 + 4 bits in the bitstream.


frame_num is constrained as specified in clause 7.4.3. For this specification, the modifications a) to f) specified in the first
paragraph of this clause apply.


**field_pic_flag** and **bottom_field_flag** have the same semantics as specified in clause 7.4.3. For this specification, the
modifications a) to d) specified in the first paragraph of this clause apply.


**idr_pic_id** identifies an IDR picture when dependency_id is equal to the maximum present value of dependency_id in the
VCL NAL units of the current coded picture. The value of idr_pic_id shall be in the range of 0 to 65535, inclusive.


When two consecutive access units in decoding order are both IDR access units, the value of idr_pic_id in the slices of the
target dependency representation in the primary coded pictures of the first such IDR access unit shall differ from the
idr_pic_id in the slices of the target dependency representation in the primary coded pictures of the second such IDR access
unit.

NOTE 2 – The classification of an access unit as IDR access unit depends on the maximum present value of dependency_id. When
NAL units are removed from a bitstream, e.g. in order to adjust the bitstream to the capabilities of a receiving device, the
classification of access units as IDR access units may change. Since all bitstreams for different conformance points supported in a
scalable bitstream (in particular for different maximum values of dependency_id) must conform to this
Recommendation | International Standard (as specified in clause G.8.8.1), the constraints on idr_pic_id must be obeyed for all
conformance points contained in a scalable bitstream.


**pic_order_cnt_lsb**, **delta_pic_order_cnt_bottom**, **delta_pic_order_cnt** [ 0 ], and **delta_pic_order_cnt** [ 1 ] have the
same semantics as specified in clause 7.4.3. For this specification, the modifications a) to f) specified in the first paragraph
of this clause apply.


**redundant_pic_cnt** has the same semantics as specified in clause 7.4.3. For this specification, the modifications a) to g )
specified in the first paragraph of this clause apply.


**direct_spatial_mv_pred_flag** specifies the method used in the decoding process to derive motion vectors and reference
indices for inter prediction. When quality_id is greater than 0, direct_spatial_mv_pred_flag is inferred as specified in the
beginning of this clause. The value of direct_spatial_mv_pred_flag shall be equal to 1.


**num_ref_idx_active_override_flag**, **num_ref_idx_l0_active_minus1**, and **num_ref_idx_l1_active_minus1** have the
same semantics as specified in clause 7.4.3. When quality_id is greater than 0, num_ref_idx_active_override_flag,
num_ref_idx_l0_active_minus1, and num_ref_idx_l1_active_minus1 are inferred as specified in the beginning of this
clause.


**base_pred_weight_table_flag** equal to 1 specifies that the variables for weighted prediction are inferred. When
base_pred_weight_table_flag is not present, it shall be inferred as follows:


- If quality_id is greater than 0, base_pred_weight_table_flag is inferred as specified in the beginning of this clause.


- Otherwise (quality_id is equal to 0), base_pred_weight_table_flag is inferred to be equal to 0.


When base_pred_weight_table_flag is equal to 1 and quality_id is equal to 0, let refSetOfSlices be the set of slices that is
represented by the VCL NAL units with dependency_id equal to (ref_layer_dq_id >> 4) and quality_id equal to 0 inside
the current coded picture.


When base_pred_weight_table_flag is equal to 1 and quality_id is equal to 0, the following constraints shall be obeyed:


a) For all slices in refSetOfSlices, the value of (slice_type % 5) shall be equal to (slice_type % 5) of the current

slice.


b) base_pred_weight_table_flag shall have the same value in all slices in refSetOfSlices.





c) When the syntax structure pred_weight_table( ) is present in the slices of the set refSetOfSlices, the values of all

syntax elements inside the syntax structure pred_weight_table( ) shall be the same for all slices in refSetOfSlices.


d) When the current slice is an EP slice, the following applies:


i) The value of num_ref_idx_l0_active_minus1 of all slices in refSetOfSlices shall be identical to the value of

num_ref_idx_l0_active_minus1 of the current slice.


ii) For each slice in refSetOfSlices, the syntax elements inside the syntax structure ref_pic_list_modification( )

shall be the same, and the syntax structure ref_pic_list_modification( ) for the slices in refSetOfSlices shall
contain syntax elements so that for useRefBasePicFlag equal to 0 and 1, an invocation of clause G.8.2.3 with
currDependencyId set equal to (ref_layer_dq_id >> 4), useRefBasePicFlag, and any slice of refSetOfSlices
as the inputs derives a reference picture list refPicList0RefLayer that is identical to the reference picture list
refPicList0, which is derived by invoking clause G.8.2.3 with currDependencyId set equal to
dependency_id, useRefBasePicFlag, and the current slice as the inputs. The entries of two reference picture
lists are considered the same when they represent entries that correspond to same coded frame, the same
complementary reference field pair, the same coded field, or the same field of a coded frame.


iii) weighted_pred_flag shall be equal to 1 for the slices in refSetOfSlices.


e) When the current slice is an EB slice, the following applies:


i) The values of num_ref_idx_l0_active_minus1 and num_ref_idx_l1_active_minus1 of all slices in

refSetOfSlices shall be identical to the values of num_ref_idx_l0_active_minus1 and
num_ref_idx_l1_active_minus1, respectively, of the current slice.


ii) For each slice in refSetOfSlices, the syntax elements inside the syntax structure ref_pic_list_modification( )

shall be the same, and the syntax structure ref_pic_list_modification( ) for the slices in refSetOfSlices shall
contain syntax elements so that for useRefBasePicFlag equal to 0 and 1, an invocation of clause G.8.2.3 with
currDependencyId set equal to (ref_layer_dq_id >> 4), useRefBasePicFlag, and any slice of refSetOfSlices
as the inputs derives reference picture lists refPicList0RefLayer and refPicList1RefLayer that are identical
to the reference picture lists refPicList0 and refPicList1, respectively, which are derived by invoking
clause G.8.2.3 with currDependencyId set equal to dependency_id, useRefBasePicFlag, and the current slice
as the inputs. The entries of two reference picture lists are considered the same when they represent entries
that correspond to same coded frame, the same complementary reference field pair, the same coded field, or
the same field of a coded frame.


iii) weighted_bipred_idc shall be equal to 1 for the slices in refSetOfSlices.


**store_ref_base_pic_flag** equal to 1 specifies that, when the value of dependency_id is equal to the maximum value of
dependency_id for the VCL NAL units of the current coded picture, an additional representation of the coded picture that
may or may not be identical to the decoded picture is marked as "used for reference". This additional representation is also
referred to as reference base picture and may be used for inter prediction of following pictures in decoding order, but it is
not output. When store_ref_base_pic_flag is not present, it shall be inferred as follows:


- If quality_id is equal to 0, store_ref_base_pic_flag is inferred to be equal to 0.


- Otherwise (quality_id is greater than 0), store_ref_base_pic_flag is inferred as specified in the beginning of this
clause.


The syntax element store_ref_base_pic_flag shall have the same value for all VCL NAL units of a dependency
representation. When nal_ref_idc is equal to 0, store_ref_base_pic_flag shall be equal to 0.


When max_num_ref_frames is less than 2, store_ref_base_pic_flag shall be equal to 0.


**cabac_init_idc** and **slice_qp_delta** have the same semantics as specified in clause 7.4.3.


**disable_deblocking_filter_idc** specifies whether the operation of the deblocking filter shall be disabled across some block
edges of the slice, specifies for which edges the filtering is disabled, and specifies the order of deblocking filter operations.
When disable_deblocking_filter_idc is not present in the slice header, the value of disable_deblocking_filter_idc shall be
inferred to be equal to 0.


The value of disable_deblocking_filter_idc shall be in the range of 0 to 6, inclusive. disable_deblocking_filter_idc equal
to 0 specifies that all luma and chroma block edges of the slice are filtered. disable_deblocking_filter_idc equal to 1
specifies that deblocking is disabled for all block edges of the slice. disable_deblocking_filter_idc equal to 2 specifies that
all luma and chroma block edges of the slice are filtered with exception of the block edges that coincide with slice
boundaries. disable_deblocking_filter_idc equal to 3 specifies a two stage deblocking filter process for the slice: After
filtering all block luma and chroma block edges that do not coincide with slice boundaries (as if
disable_deblocking_filter_idc were equal to 2), the luma and chroma block edges that coincide with slice boundaries are
filtered. disable_deblocking_filter_idc equal to 4 specifies that all luma block edges of the slice are filtered, but the





deblocking of the chroma block edges is disabled. disable_deblocking_filter_idc equal to 5 specifies that all luma block
edges of the slice are filtered with exception of the block edges that coincide with slice boundaries (as if
disable_deblocking_filter_idc were equal to 2), and that deblocking for chroma block edges of the slice is disabled.
disable_deblocking_filter_idc equal to 6 specifies that the deblocking for chroma block edges is disabled and that the two
stage deblocking filter process is used for luma block edges of the slice: After filtering all block luma block edges that do
not coincide with slice boundaries (as if disable_deblocking_filter_idc were equal to 2), the luma block edges that coincide
with slice boundaries are filtered.


When no_inter_layer_pred_flag is equal to 1 or tcoeff_level_prediction_flag is equal to 1, the value of
disable_deblocking_filter_idc shall be in the range of 0 to 2, inclusive.


**slice_alpha_c0_offset_div2**, and **slice_beta_offset_div2** have the same semantics as specified in clause 7.4.3.


**slice_group_change_cycle** has the same semantics as specified in clause 7.4.3.


**ref_layer_dq_id** specifies the layer representation inside the current coded picture that is used for inter-layer prediction
of the current layer representation. When present, the value of ref_layer_dq_id shall be in the range of 0 to DQId − 1,
inclusive. When ref_layer_dq_id is not present, it shall be inferred as follows:


- If quality_id is greater than 0, ref_layer_dq_id is inferred to be equal to (DQId − 1).


- Otherwise (quality_id is equal to 0), ref_layer_dq_id is inferred to be equal to −1.


When quality_id is equal to 0, the NAL units with DQId equal to ref_layer_dq_id shall have discardable_flag equal to 0.


When ref_layer_dq_id is greater than or equal to 0, it is a requirement of bitstream conformance that the layer
representation with DQId equal to ref_layer_dq_id is present in the bitstream.


The variable MaxRefLayerDQId is set equal to the maximum value of ref_layer_dq_id for the slices of the current layer
representation.


When MinNoInterLayerPredFlag is equal to 0, the layer representation inside the current coded picture that has a value of
DQId equal MaxRefLayerDQId is also referred to as reference layer representation.


When MaxRefLayerDQId is not equal to −1, the following variables are derived as follows:


- RefLayerPicSizeInMbs is set equal to the value of the variable PicSizeInMbs for the reference layer representation.


- RefLayerPicWidthInMbs is set equal to the value of the variable PicWidthInMbs for the reference layer
representation.


- RefLayerPicHeightInMbs is set equal to the value of the variable PicHeightInMbs for the reference layer
representation.


- RefLayerChromaFormatIdc is set equal to the value of the syntax element chroma_format_idc for the reference layer
representation.


- RefLayerChromaArrayType is set equal to the value of ChromaArrayType for the reference layer representation.


- RefLayerPicWidthInSamplesL is set equal to the value of the variable PicWidthInSamplesL for the reference layer
representation.


- RefLayerPicHeightInSamplesL is set equal to the value of the variable PicHeightInSamplesL for the reference layer
representation.


- RefLayerPicWidthInSamplesC is set equal to the value of the variable PicWidthInSamplesC for the reference layer
representation.


- RefLayerPicHeightInSamplesC is set equal to the value of the variable PicHeightInSamplesC for the reference layer
representation.


- RefLayerMbWidthC is set equal to the value of the variable MbWidthC for the reference layer representation.


- RefLayerMbHeightC is set equal to the value of the variable MbHeightC for the reference layer representation.


- RefLayerFrameMbsOnlyFlag is set equal to the value of the syntax element frame_mbs_only_flag for the reference
layer representation.


- RefLayerFieldPicFlag is set equal to the value of the syntax element field_pic_flag for the reference layer
representation.


- RefLayerBottomFieldFlag is set equal to the value of the syntax element bottom_field_flag for the reference layer
representation.





- RefLayerMbaffFrameFlag is set equal to the value of the variable MbaffFrameFlag for the reference layer
representation.


**disable_inter_layer_deblocking_filter_idc** specifies whether the operation of the deblocking filter for inter-layer intra
prediction is disabled across some block edges of the reference layer representation, specifies for which edges the filtering
is disabled, and specifies the order of deblocking filter operations for inter-layer intra prediction. When
disable_inter_layer_deblocking_filter_idc is not present in the slice header, the value of
disable_inter_layer_deblocking_filter_idc shall be inferred to be equal to 0. The value of
disable_inter_layer_deblocking_filter_idc shall be in the range of 0 to 6, inclusive. The values 0 to 6 of
disable_inter_layer_deblocking_filter_idc specify the same deblocking filter operations as the corresponding values of
disable_deblocking_filter_idc, but for the deblocking of the intra macroblocks of the reference layer representation
specified by ref_layer_dq_id before resampling.


When disable_inter_layer_deblocking_filter_idc is present, quality_id is equal to 0, and SpatialResolutionChangeFlag as
specified in the following paragraphs is equal to 0, disable_inter_layer_deblocking_filter_idc shall be equal to 1.


**inter_layer_slice_alpha_c0_offset_div2** specifies the offset used in accessing the α and tC0 deblocking filter tables for
filtering operations of the intra macroblocks of the reference layer representation before resampling. From this value, the
offset that is applied when addressing these tables shall be computed as:


InterlayerFilterOffsetA = inter_layer_slice_alpha_c0_offset_div2 << 1 (G-64)


The value of inter_layer_slice_alpha_c0_offset_div2 shall be in the range of −6 to +6, inclusive. When
inter_layer_slice_alpha_c0_offset_div2 is not present in the slice header, the value of
inter_layer_slice_alpha_c0_offset_div2 shall be inferred to be equal to 0.


**inter_layer_slice_beta_offset_div2** specifies the offset used in accessing the β deblocking filter table for filtering
operations of the intra macroblocks of the reference layer representation before resampling. From this value, the offset that
is applied when addressing the β table of the deblocking filter is computed as:


InterlayerFilterOffsetB = inter_layer_slice_beta_offset_div2 << 1 (G-65)


The value of inter_layer_slice_beta_offset_div2 shall be in the range of −6 to +6, inclusive. When
inter_layer_slice_beta_offset_div2 is not present in the slice header the value of inter_layer_slice_beta_offset_div2 shall
be inferred to be equal to 0.


**constrained_intra_resampling_flag** specifies whether slice boundaries in the layer picture that is used for inter-layer
prediction (as specified by ref_layer_dq_id) are treated similar to layer picture boundaries for the intra resampling process.
When constrained_intra_resampling_flag is equal to 1, disable_inter_layer_deblocking_filter_idc shall be equal to 1, 2,
or 5.


When constrained_intra_resampling_flag is equal to 1, a macroblock cannot be coded using the Intra_Base macroblock
prediction mode when it covers more than one slice in the layer picture that is used for inter-layer prediction, as specified
in clause G.8.6.2.


When constrained_intra_resampling_flag is not present, it shall be inferred to be equal to 0.


**ref_layer_chroma_phase_x_plus1_flag** specifies the horizontal phase shift of the chroma components in units of half
luma samples of a layer frame for the layer pictures that may be used for inter-layer prediction.


When ref_layer_chroma_phase_x_plus1_flag is not present, it shall be inferred as follows:


- If quality_id is greater than 0, ref_layer_chroma_phase_x_plus1_flag is inferred to be equal to
chroma_phase_x_plus1_flag.


- Otherwise (quality_id is equal to 0), ref_layer_chroma_phase_x_plus1_flag is inferred to be equal to
seq_ref_layer_chroma_phase_x_plus1_flag.


When no_inter_layer_pred_flag is equal to 0, the following is specified:


a) When ref_layer_dq_id is greater than 0, ref_layer_chroma_phase_x_plus1_flag should be equal to

chroma_phase_x_plus1_flag of the subset sequence parameter set RBSP that is referred to by the reference layer
representation (with DQId equal to ref_layer_dq_id).


b) When RefLayerChromaArrayType is equal to 1 and chroma_sample_loc_type_top_field and
chroma_sample_loc_type_bottom_field are present in the SVC sequence parameter set that is referred to by the
reference layer representation (with DQId equal to ref_layer_dq_id), the following applies:





    - If ref_layer_chroma_phase_x_plus1_flag is equal to 0, chroma_sample_loc_type_top_field and
chroma_sample_loc_type_bottom_field of the SVC sequence parameter set that is referred to by the reference
layer representation should be equal to 0, 2, or 4.


    - Otherwise (ref_layer_chroma_phase_x_plus1_flag is equal to 1), chroma_sample_loc_type_top_field and
chroma_sample_loc_type_bottom_field of the SVC sequence parameter set that is referred to by the reference
layer representation should be equal to 1, 3, or 5.


c) When RefLayerChromaArrayType is not equal to 1, ref_layer_chroma_phase_x_plus1_flag should be equal to 1.


**ref_layer_chroma_phase_y_plus1** specifies the vertical phase shift of the chroma components in units of half luma
samples of a layer frame for the layer pictures that may be used for inter-layer prediction.


When ref_layer_chroma_phase_y_plus1 is not present, it shall be inferred as follows:


- If quality_id is greater than 0, ref_layer_chroma_phase_y_plus1 is inferred to be equal to chroma_phase_y_plus1.


- Otherwise (quality_id is equal to 0), ref_layer_chroma_phase_y_plus1 is inferred to be equal to
seq_ref_layer_chroma_phase_y_plus1.


The value of ref_layer_chroma_phase_y_plus1 shall be in the range of 0 to 2, inclusive.


When no_inter_layer_pred_flag is equal to 0, the following applies:


a) When ref_layer_dq_id is greater than 0, ref_layer_chroma_phase_y_plus1 should be equal to
chroma_phase_y_plus1 of the subset sequence parameter set RBSP that is referred to by the reference layer
representation (with DQId equal to ref_layer_dq_id).


b) When RefLayerChromaArrayType is equal to 1 and chroma_sample_loc_type_top_field and
chroma_sample_loc_type_bottom_field are present in the SVC sequence parameter set that is referred to by the
reference layer representation (with DQId equal to ref_layer_dq_id), the following applies:


    - If ref_layer_chroma_phase_y_plus1 is equal to 0, chroma_sample_loc_type_top_field and
chroma_sample_loc_type_bottom_field of the SVC sequence parameter set that is referred to by the reference
layer representation should be equal to 2 or 3.


    - Otherwise, if ref_layer_chroma_phase_y_plus1 is equal to 1, chroma_sample_loc_type_top_field and
chroma_sample_loc_type_bottom_field of the SVC sequence parameter set that is referred to by the reference
layer representation should be equal to 0 or 1.


    - Otherwise (chroma_phase_y_plus1 is equal to 2), chroma_sample_loc_type_top_field and
chroma_sample_loc_type_bottom_field of the SVC sequence parameter set that is referred to by the reference
layer representation should be equal to 4 or 5.


c) When RefLayerChromaArrayType is not equal to 1, ref_layer_chroma_phase_y_plus1 should be equal to 1.


**scaled_ref_layer_left_offset** specifies the horizontal offset between the upper-left luma sample of a resampled layer
picture used for inter-layer prediction and the upper-left luma sample of the current picture or current layer picture in units
of two luma samples.


When scaled_ref_layer_left_offset is not present, it shall be inferred as follows:


- If quality_id is greater than 0, scaled_ref_layer_left_offset is inferred to be equal to 0.


- Otherwise (quality_id is equal to 0), scaled_ref_layer_left_offset is inferred to be equal to
seq_scaled_ref_layer_left_offset.


The value of scaled_ref_layer_left_offset shall be in the range of −2 [15] to 2 [15] - 1, inclusive.


**scaled_ref_layer_top_offset** specifies the vertical offset between the upper-left luma sample of a resampled layer picture
used for inter-layer prediction and the upper-left luma sample of the current picture or current layer picture. The vertical
offset is specified in units of two luma samples when frame_mbs_only_flag is equal to 1, and it is specified in units of four
luma samples when frame_mbs_only_flag is equal to 0.


When scaled_ref_layer_top_offset is not present, it shall be inferred as follows:


- If quality_id is greater than 0, scaled_ref_layer_top_offset is inferred to be equal to 0.


- Otherwise (quality_id is equal to 0), scaled_ref_layer_top_offset is inferred to be equal to
seq_scaled_ref_layer_top_offset.


The value of scaled_ref_layer_top_offset shall be in the range of −2 [15] to 2 [15] - 1, inclusive.





**scaled_ref_layer_right_offset** specifies the horizontal offset between the bottom-right luma sample of a resampled layer
picture used for inter-layer prediction and the bottom-right luma sample of the current picture or current layer picture in
units of two luma samples.


When scaled_ref_layer_right_offset is not present, it shall be inferred as follows:


- If quality_id is greater than 0, scaled_ref_layer_right_offset is inferred to be equal to 0.


- Otherwise (quality_id is equal to 0), scaled_ref_layer_right_offset is inferred to be equal to
seq_scaled_ref_layer_right_offset.


The value of scaled_ref_layer_right_offset shall be in the range of −2 [15] to 2 [15] - 1, inclusive.


**scaled_ref_layer_bottom_offset** specifies the vertical offset between the bottom-right luma sample of a resampled layer
picture used for inter-layer prediction and the bottom-right luma sample of the current picture or current layer picture. The
vertical offset is specified in units of two luma samples when frame_mbs_only_flag is equal to 1, and it is specified in
units of four luma samples when frame_mbs_only_flag is equal to 0.


When scaled_ref_layer_bottom_offset is not present, it shall be inferred as follows:


- If quality_id is greater than 0, scaled_ref_layer_bottom_offset is inferred to be equal to 0.


- Otherwise (quality_id is equal to 0), scaled_ref_layer_bottom_offset is inferred to be equal to
seq_scaled_ref_layer_bottom_offset.


The value of scaled_ref_layer_bottom_offset shall be in the range of −2 [15] to 2 [15] - 1, inclusive.


The variables scaledLeftOffset, scaledRightOffset, scaledTopOffset, and scaledBottomOffset are derived as follows:


- If MinNoInterLayerPredFlag is equal to 0, scaledLeftOffset, scaledRightOffset, scaledTopOffset, and
scaledBottomOffset are set equal to the values of scaled_ref_layer_left_offset, scaled_ref_layer_right_offset,
scaled_ref_layer_top_offset, and scaled_ref_layer_bottom_offset, respectively, for the slices of the current layer
representation that have no_inter_layer_pred_flag equal to 0.


- Otherwise (MinNoInterLayerPredFlag is equal to 1), scaledLeftOffset, scaledRightOffset, scaledTopOffset, and
scaledBottomOffset are set equal to the values of scaled_ref_layer_left_offset, scaled_ref_layer_right_offset,
scaled_ref_layer_top_offset, and scaled_ref_layer_bottom_offset, respectively.


The variables ScaledRefLayerLeftOffset, ScaledRefLayerRightOffset, ScaledRefLayerTopOffset,
ScaledRefLayerBottomOffset, ScaledRefLayerPicWidthInSamplesL, and ScaledRefLayerPicHeightInSamplesL are
derived by


ScaledRefLayerLeftOffset          = 2 * scaledLeftOffset (G-66)
ScaledRefLayerRightOffset         = 2 * scaledRightOffset (G-67)
ScaledRefLayerTopOffset          = 2 * scaledTopOffset * ( 2 − frame_mbs_only_flag ) (G-68)
ScaledRefLayerBottomOffset        = 2 * scaledBottomOffset * ( 2 − frame_mbs_only_flag ) (G-69)
ScaledRefLayerPicWidthInSamplesL  = PicWidthInMbs * 16 − ScaledRefLayerLeftOffset −
ScaledRefLayerRightOffset (G-70)
ScaledRefLayerPicHeightInSamplesL = PicHeightInMbs * 16 −
( ScaledRefLayerTopOffset + ScaledRefLayerBottomOffset ) /
( 1 + field_pic_flag ) (G-71)


When no_inter_layer_pred_flag is equal to 0, the following constraints shall be obeyed:


a) The bitstream shall not contain data that result in ScaledRefLayerPicWidthInSamplesL less than

RefLayerPicWidthInSamplesL.


b) When RefLayerFrameMbsOnlyFlag is equal to 0 or frame_mbs_only_flag is equal to 1, the bitstream shall not

contain data that result in (ScaledRefLayerPicHeightInSamplesL * ( 1 + field_pic_flag )) less than
(RefLayerPicHeightInSamplesL * ( 1 + RefLayerFieldPicFlag )).


c) When RefLayerFrameMbsOnlyFlag is equal to 1 and frame_mbs_only_flag is equal to 0, the bitstream shall not

contain data that result in (ScaledRefLayerPicHeightInSamplesL * ( 1 + field_pic_flag )) less than
(2 * RefLayerPicHeightInSamplesL).


When ChromaArrayType is not equal to 0, the variables ScaledRefLayerPicWidthInSamplesC, and
ScaledRefLayerPicHeightInSamplesC are derived by


ScaledRefLayerPicWidthInSamplesC = ScaledRefLayerPicWidthInSamplesL / SubWidthC (G-72)
ScaledRefLayerPicHeightInSamplesC = ScaledRefLayerPicHeightInSamplesL / SubHeightC (G-73)





The variable CroppingChangeFlag is derived as follows:


- If MinNoInterLayerPredFlag is equal to 0, quality_id is equal to 0, and extended_spatial_scalability_idc is equal to 2,
CroppingChangeFlag is set equal to 1.


- Otherwise (MinNoInterLayerPredFlag is equal to 1, quality_id is greater than 0, or extended_spatial_scalability_idc
is less than 2), CroppingChangeFlag is set equal to 0.
NOTE 3 – Encoder designers are encouraged to set the value of no_inter_layer_pred_flag equal to 0 for at least one slice of each
layer representation with extended_spatial_scalability_idc equal to 2 and quality_id equal to 0.


The variable SpatialResolutionChangeFlag is derived as follows:


- If MinNoInterLayerPredFlag is equal to 1, quality_id is greater than 0, or all of the following conditions are true,
SpatialResolutionChangeFlag is set equal to 0:


   - CroppingChangeFlag is equal to 0,


   - ScaledRefLayerPicWidthInSamplesL is equal to RefLayerPicWidthInSamplesL,


   - ScaledRefLayerPicHeightInSamplesL is equal to RefLayerPicHeightInSamplesL,


   - ( ScaledRefLayerLeftOffset % 16 ) is equal to 0,


   - ( ScaledRefLayerTopOffset % ( 16 * ( 1 + field_pic_flag + MbaffFrameFlag ) ) ) is equal to 0,


   - field_pic_flag is equal to RefLayerFieldPicFlag,


   - MbaffFrameFlag is equal to RefLayerMbaffFrameFlag,


   - chroma_format_idc is equal to RefLayerChromaFormatIdc,


   - chroma_phase_x_plus1_flag is equal to ref_layer_chroma_phase_x_plus1_flag for the slices with
no_inter_layer_pred_flag equal to 0,


   - chroma_phase_y_plus1 is equal to ref_layer_chroma_phase_y_plus1 for the slices with
no_inter_layer_pred_flag equal to 0.


- Otherwise, SpatialResolutionChangeFlag is set equal to 1.


The variable RestrictedSpatialResolutionChangeFlag is derived as follows:


- If SpatialResolutionChangeFlag is equal to 0 or all of the following conditions are true,
RestrictedSpatialResolutionChangeFlag is set equal to 1:


   - ScaledRefLayerPicWidthInSamplesL is equal to RefLayerPicWidthInSamplesL or
(2 * RefLayerPicWidthInSamplesL),


   - ScaledRefLayerPicHeightInSamplesL is equal to RefLayerPicHeightInSamplesL or
(2 * RefLayerPicHeightInSamplesL),


   - ( ScaledRefLayerLeftOffset % 16 ) is equal to 0,


   - ( ScaledRefLayerTopOffset % ( 16 * ( 1 + field_pic_flag ) ) ) is equal to 0,


   - MbaffFrameFlag is equal to 0,


   - RefLayerMbaffFrameFlag is equal to 0,


   - field_pic_flag is equal to RefLayerFieldPicFlag.


- Otherwise, RestrictedSpatialResolutionChangeFlag is set equal to 0.


**slice_skip_flag** specifies the presence of the slice data in scalable extension syntax structure. When slice_skip_flag is not
present, it shall be inferred to be equal to 0. slice_skip_flag equal to 0 specifies that the slice data in scalable extension
syntax structure is present in the NAL unit. slice_skip_flag equal to 1 specifies that the slice data in scalable extension
syntax structure is not present in the NAL unit and that the syntax elements for the macroblock layer of the slice are derived
by the following process:


1. CurrMbAddr is derived by


CurrMbAddr = first_mb_in_slice * ( 1 + MbaffFrameFlag ) (G-74)


2. The variable mbIdx proceeds over the values 0..num_mbs_in_slice_minus1, and for each value of mbIdx, the

following ordered steps are specified:





a. The bitstream shall not contain data that result in InCropWindow( CurrMbAddr ) equal to 0.


b. For the macroblock with address CurrMbAddr, the syntax elements mb_skip_flag (when applicable),

mb_skip_run (when applicable), mb_field_decoding_flag, base_mode_flag, residual_prediction_flag and
coded_block_pattern shall be inferred as follows:


        - mb_skip_flag (when applicable) and mb_skip_run (when applicable) are inferred to be equal to 0.


        - mb_field_decoding_flag is inferred to be equal to 0.
NOTE 4 – The frame/field mode used for decoding is inferred in clause G.8.1.5.1.


        - base_mode_flag is inferred to be equal to 1.


        - residual_prediction_flag is inferred to be equal to 1.


        - coded_block_pattern is inferred to be equal to 0.


        - QPY is inferred to be equal to SliceQPY.


        - QP′Y is inferred to be equal to (QPY + QpBdOffsetY).


c. When the variable mbIdx is less than num_mbs_in_slice_minus1, CurrMbAddr is set to
NextMbAddress( CurrMbAddr ). The bitstream shall not contain data that result in CurrMbAddr being set
equal to a value that is not less than PicSizeInMbs.


**num_mbs_in_slice_minus1** plus 1specifies the number of macroblocks for a slice with slice_skip_flag equal to 1.


**adaptive_base_mode_flag** specifies the presence of syntax elements in the slice header and in the macroblock layer in
scalable extension. When adaptive_base_mode_flag is not present, it shall be inferred to be equal to 0.


**default_base_mode_flag** specifies how base_mode_flag is inferred when it is not present in macroblock layer in scalable
extension. When default_base_mode_flag is not present, it shall be inferred to be equal to 0.


**adaptive_motion_prediction_flag** specifies the presence of syntax elements in the macroblock layer in scalable extension.
When adaptive_motion_prediction_flag is not present, it shall be inferred to be equal to 0.


**default_motion_prediction_flag** specifies how motion_prediction_flag_l0[ ] and motion_prediction_flag_l1[ ] are
inferred when they are not present in macroblock layer in scalable extension. When default_motion_prediction_flag is not
present, it shall be inferred to be equal to 0.


**adaptive_residual_prediction_flag** specifies the presence of syntax elements in the macroblock layer in scalable
extension. When adaptive_residual_prediction_flag is not present, it shall be inferred to be equal to 0.


**default_residual_prediction_flag** specifies how residual_prediction_flag is inferred when it is not present in the
macroblock layer in scalable extension. When default_residual_prediction_flag is not present, it shall be inferred to be
equal to 0.


**tcoeff_level_prediction_flag** equal to 1 specifies that an alternative inter-layer prediction process is applied as specified
in clause G.8. When tcoeff_level_prediction_flag is not present, it shall be inferred as follows:


- If no_inter_layer_pred_flag is equal to 1, tcoeff_level_prediction_flag is inferred to be equal to 0.


- Otherwise (no_inter_layer_pred_flag is equal to 0), tcoeff_level_prediction_flag is inferred to be equal to the value
of seq_tcoeff_level_prediction_flag.


When SpatialResolutionChangeFlag is equal to 1, tcoeff_level_prediction_flag shall be equal to 0.


When tcoeff_level_prediction_flag is equal to 1, the following constraints shall be obeyed:


a) The slices of the reference layer representation (with DQId equal to ref_layer_dq_id) shall have

no_inter_layer_pred_flag equal to 1 or tcoeff_level_prediction_flag equal to 1.


b) All elements of ScalingList4x4 shall be the same for the slices of the current layer representation and all slices

of the reference layer representation (with DQId equal to the value of ref_layer_dq_id).


c) All elements of ScalingList8x8 shall be the same for the slices of the current layer representation and all slices

of the reference layer representation (with DQId equal to the value of ref_layer_dq_id).


d) The value of the syntax element use_ref_base_pic_flag shall be equal to 0 for the slices of the current layer

representation and all slices of the reference layer representation (with DQId equal to the value of
ref_layer_dq_id).





e) When slice_skip_flag is equal to 1, the value of constrained_intra_pred_flag for the current layer representation

shall be identical to the value of constrained_intra_pred_flag for the reference layer representation (with DQId
equal to ref_layer_dq_id).


The variable MaxTCoeffLevelPredFlag is set equal to the maximum value of tcoeff_level_prediction_flag for the slices of
the current layer representation.


**scan_idx_start** specifies the first scanning position for the transform coefficient levels in the current slice. When
scan_idx_start is not present, it shall be inferred to be equal to 0.


**scan_idx_end** specifies the last scanning position for the transform coefficient levels in the current slice. When
scan_idx_end is not present, it shall be inferred to be equal to 15.


When default_base_mode_flag is equal to 1, (slice_type % 5) is equal to 2, and entropy_coding_mode_flag is equal to 0,
it is a requirement of bitstream conformance that the value of scan_idx_end is greater than or equal to scan_idx_start.


**G.7.4.3.5** **Decoded reference base picture marking semantics**


The specification of this clause applies to the current dependency representation. The modifications a) and b) specified in
clause G.8.2 apply with currDependencyId being equal to the current value of dependency_id.


The syntax elements adaptive_ref_base_pic_marking_mode_flag, memory_management_base_control_operation,
difference_of_base_pic_nums_minus1, and long_term_base_pic_num specify marking of reference base pictures as
"unused for reference".


When present in a prefix NAL unit, all syntax elements of the dec_ref_base_pic_marking( ) syntax structure are considered
as if they were present in the associated NAL unit.


When quality_id is greater than 0, all syntax elements of the dec_ref_base_pic_marking( ) syntax structure are inferred as
specified in the beginning of clause G.7.4.3.4.


The content of the decoded reference picture base marking syntax structure shall be the same in all slice headers of the
primary coded picture. When one or more redundant coded pictures are present, the content of the decoded reference base
picture marking syntax structure shall be the same in all slice headers of a redundant coded picture with a particular value
of redundant_pic_cnt.

NOTE 1 – It is not required that the content of the decoded reference base picture marking syntax structure in a redundant coded
picture with a particular value of redundant_pic_cnt is identical to the content of the decoded reference base picture marking syntax
structure in the corresponding primary coded picture or a redundant coded picture with a different value of redundant_pic_cnt.
However, as specified in clause G.7.4.3.4 (by referencing clause 7.4.3), the content of the decoded reference base picture marking
syntax structure in a redundant coded picture is constrained in the way that the marking status of reference pictures and the value of
frame_num after the SVC decoded reference picture marking process in clause G.8.2.4 must be identical regardless whether the
primary coded picture or any redundant coded picture of the access unit would be decoded.


The memory_management_base_control_operation commands of the dec_ref_base_pic_marking( ) syntax structure are
processed by the decoding process before the memory_management_control_operation commands of the
dec_ref_pic_marking( ) syntax structure are processed.


**adaptive_ref_base_pic_marking_mode_flag** selects the reference base picture marking mode for the current picture or
layer picture as specified in Table G-2. When adaptive_ref_base_pic_marking_mode_flag is not present and quality_id is
equal to 0, it shall be inferred to be equal to 0.


**Table G-2 – Interpretation of adaptive_ref_base_pic_marking_mode_flag**

|adaptive_ref_base_pic_marking_mode_flag|Reference base picture marking mode specified|
|---|---|
|0|Sliding window reference picture marking mode: A<br>marking<br>mode<br>providing<br>a <br>first-in,<br>first-out<br>mechanism for short-term reference pictures|
|1|Adaptive reference base picture marking mode: A<br>reference picture marking mode providing syntax<br>elements to specify marking of reference base<br>pictures as "unused for reference"|



**memory_management_base_control_operation** specifies a control operation to be applied to affect the marking of
reference base pictures. The memory_management_base_control_operation syntax element is followed by data necessary
for the operation specified by the value of memory_management_base_control_operation. The values and control
operations associated with memory_management_base_control_operation are specified in Table G-3. The
memory_management_base_control_operation syntax elements are processed by the decoding process in the order in





which they appear, and the semantics constraints expressed for each memory_management_base_control_operation apply
at the specific position in that order at which that individual memory_management_base_control_operation is processed.


For interpretation of memory_management_base_control_operation, the terms reference picture and reference base picture
are interpreted as follows:


- If the current picture is a frame, the term reference picture refers either to a reference frame or a complementary
reference field pair and the term reference base picture refers either to a reference base frame or a complementary
reference base field pair.


- Otherwise (the current picture is a field), the term reference picture refers either to a reference field or a field of a
reference frame and the term reference base picture refers either to a reference base field or a field of a reference base
frame.


memory_management_base_control_operation shall not be equal to 1 unless the specified reference base picture is marked
as "used for short-term reference" (and as "reference base picture") when the
memory_management_base_control_operation is processed by the decoding process.


memory_management_base_control_operation shall not be equal to 2 unless the specified long-term picture number refers
to a reference base picture that is marked as "used for long-term reference" (and as "reference base picture") when the
memory_management_base_control_operation is processed by the decoding process.


When the dec_ref_pic_marking( ) syntax structure contains a memory_management_control_operation equal to 5,
memory_management_base_control_operation shall not be equal to 1 or 2.


**Table G-3 – Memory management base control operation (memory_management_base_control_operation) values**

|memory_management_base_control_operation|Memory Management Base Control Operation|
|---|---|
|0|End memory_management_base_control_operation<br>syntax element loop|
|1|Mark a short-term reference base picture as<br>"unused for reference"|
|2|Mark a long-term reference base picture as<br>"unused for reference"|



**difference_of_base_pic_nums_minus1** is used (with memory_management_base_control_operation equal to 1) to mark
a short-term reference base picture as "unused for reference". When the associated
memory_management_base_control_operation is processed by the decoding process, the resulting picture number derived
from difference_of_base_pic_nums_minus1 shall be a picture number assigned to one of the reference pictures marked as
"used for short-term reference" and as "reference base picture".


The resulting picture number is constrained as follows:


- If field_pic_flag is equal to 0, the resulting picture number shall be one of the set of picture numbers assigned to
reference frames or complementary reference field pairs marked as "reference base picture".

NOTE 2 – When field_pic_flag is equal to 0, the resulting picture number must be a picture number assigned to a
complementary reference field pair in which both fields are marked as "used for short-term reference" and "reference
base picture" or a reference frame in which both fields are marked as "used for short-term reference" and "reference base
picture".


- Otherwise (field_pic_flag is equal to 1), the resulting picture number shall be one of the set of picture numbers
assigned to reference fields marked as "reference base picture".


**long_term_base_pic_num** is used (with memory_management_base_control_operation equal to 2) to mark a long-term
reference base picture as "unused for reference". When the associated memory_management_base_control_operation is
processed by the decoding process, long_term_base_pic_num shall be equal to a long-term picture number assigned to one
of the reference pictures marked as "used for long-term reference" and as "reference base picture".


The resulting long-term picture number is constrained as follows:


- If field_pic_flag is equal to 0, the resulting long-term picture number shall be one of the set of long-term picture
numbers assigned to reference frames or complementary reference field pairs marked as "reference base picture".

NOTE 3 – When field_pic_flag is equal to 0, the resulting long-term picture number must be a long-term picture number
assigned to a complementary reference field pair in which both fields are marked as "used for long-term reference" and
"reference base picture" or a reference frame in which both fields are marked as "used for long-term reference" and
"reference base picture".


- Otherwise (field_pic_flag is equal to 1), the resulting long-term picture number shall be one of the set of long-term
picture numbers assigned to reference fields marked as "reference base picture".





**G.7.4.4** **Slice data semantics**


The semantics specified in clause 7.4.4 apply.


**G.7.4.4.1** **Slice data in scalable extension semantics**


The semantics specified in clause 7.4.4 apply with the following modifications.


**mb_skip_run** specifies the number of consecutive skipped macroblocks for which, when decoding an EP slice, mb_type
shall be inferred to be P_Skip and the macroblock type is collectively referred to as a P macroblock type, or for which,
when decoding an EB slice, mb_type shall be inferred to be B_Skip and the macroblock type is collectively referred to as
a B macroblock type. The value of mb_skip_run shall be in the range of 0 to PicSizeInMbs − CurrMbAddr, inclusive.


**mb_skip_flag** equal to 1 specifies that for the current macroblock, when decoding an EP slice, mb_type shall be inferred
to be P_Skip and the macroblock type is collectively referred to as P macroblock type, or for which, when decoding an EB
slice, mb_type shall be inferred to be B_Skip and the macroblock type is collectively referred to as B macroblock type.
mb_skip_flag equal to 0 specifies that the current macroblock is not skipped.


**G.7.4.5** **Macroblock layer semantics**


The semantics specified in clause 7.4.5 apply. Additionally, the following applies.


The macroblock_layer( ) syntax structure shall be considered to contain the following syntax elements with the following
inferred values:


- base_mode_flag is inferred to be equal to 0.


- residual_prediction_flag is inferred to be equal to 0.


**G.7.4.5.1** **Macroblock prediction semantics**


The semantics specified in clause 7.4.5.1 apply. Additionally, the following applies.


The value of mvd_l0[ mbPartIdx ][ 0 ][ compIdx ] and mvd_l1[ mbPartIdx ][ 0 ][ compIdx ] shall be in the range of −8192
to 8191.75, inclusive. The range of mvd_l0[ mbPartIdx ][ 0 ][ compIdx ] and mvd_l1[ mbPartIdx ][ 0 ][ compIdx ] is also
constrained indirectly by constraints on the motion vector variable values derived from it as specified in clause G.10.


The mb_pred( ) syntax structure shall be considered to contain the following syntax elements with the following inferred
values:


- motion_prediction_flag_l0[ mbPartIdx ] is inferred to be equal to 0 for each value of mbPartIdx in the range of 0
to NumMbPart( mb_type ) − 1, inclusive.


- motion_prediction_flag_l1[ mbPartIdx ] is inferred to be equal to 0 for each value of mbPartIdx in the range of 0
to NumMbPart( mb_type ) − 1, inclusive.


**G.7.4.5.2** **Sub-macroblock prediction semantics**


The semantics specified in clause 7.4.5.2 apply. Additionally, the following applies.


The value of mvd_l0[ mbPartIdx ][ subMbPartIdx ][ compIdx ] and mvd_l1[ mbPartIdx ][ subMbPartIdx ][ compIdx ]
shall be in the range of −8192 to 8191.75, inclusive. The range of mvd_l0[ mbPartIdx ][ subMbPartIdx ][ compIdx ] and
mvd_l1[ mbPartIdx ][ subMbPartIdx ][ compIdx ] is also constrained indirectly by constraints on the motion vector
variable values derived from it as specified in clause G.10.


The sub_mb_pred( ) syntax structure shall be considered to contain the following syntax elements with the following
inferred values:


- motion_prediction_flag_l0[ mbPartIdx ] is inferred to be equal to 0 for each value of mbPartIdx in the range of 0 to 3,
inclusive.


- motion_prediction_flag_l1[ mbPartIdx ] is inferred to be equal to 0 for each value of mbPartIdx in the range of 0 to 3,
inclusive.


**G.7.4.5.3** **Residual data semantics**


The semantics specified in clause 7.4.5.3 apply.


**G.7.4.5.3.1** **Residual luma semantics**


The semantics specified in clause 7.4.5.3.1 apply.


**G.7.4.5.3.2** **Residual block CAVLC semantics**


The semantics specified in clause 7.4.5.3.2 apply.





**G.7.4.5.3.3** **Residual block CABAC semantics**


The semantics specified in clause 7.4.5.3.3 apply.


**G.7.4.6** **Macroblock layer in scalable extension semantics**


The semantics specified in clause 7.4.5 apply. Additionally, the following modifications and extensions are specified.


The function InCropWindow( mbAddr ) is specified by the following ordered steps:


1. The variable mbX is set equal to (( mbAddr / ( 1 + MbaffFrameFlag ) ) % PicWidthInMbs).


2. The variables mbY0 and mbY1 are derived as follows:


      - If MbaffFrameFlag is equal to 0, mbY0 and mbY1 are set equal to (mbAddr / PicWidthInMbs).


      - Otherwise (MbaffFrameFlag is equal to 1), mbY0 is set equal to (2 * ( ( mbAddr / PicWidthInMbs ) / 2 ))
and mbY1 is set equal to (mbY0 + 1).


3. The variable scalMbH is set equal to (16 * ( 1 + field_pic_flag )).


4. The return value of InCropWindow( mbAddr ) is derived as follows:


     - If all of the following conditions are true, the return value of InCropWindow( mbAddr ) is equal to TRUE.


        - no_inter_layer_pred_flag is equal to 0


        - mbX is greater than or equal to (( ScaledRefLayerLeftOffset + 15 ) / 16)


        - mbX is less than (( ScaledRefLayerLeftOffset + ScaledRefLayerPicWidthInSamplesL ) / 16)


        - mbY0 is greater than or equal to (( ScaledRefLayerTopOffset + scalMbH − 1 ) / scalMbH)


        - mbY1 is less than (( ScaledRefLayerTopOffset + ScaledRefLayerPicHeightInSamplesL ) / scalMbH)


     - Otherwise, the return value of InCropWindow( mbAddr ) is equal to FALSE.


**base_mode_flag** equal to 1 specifies that the macroblock partitioning, the macroblock (partition) prediction mode(s), and
the corresponding motion data (when applicable) are inferred as specified in clause G.8. base_mode_flag equal to 0
specifies that the syntax element mb_type is present in the macroblock layer in scalable extension syntax structure or that
mb_type shall be inferred as specified in clause G.7.4.4.1.


When base_mode_flag is not present, base_mode_flag shall be inferred as follows:


- If InCropWindow( CurrMbAddr ) is equal to 0, the value of base_mode_flag is inferred to be equal to 0.


- Otherwise, if the syntax element mb_skip_run (when entropy_coding_mode_flag is equal to 0) or mb_skip_flag
(when entropy_coding_mode_flag is equal to 1) specifies that mb_type is inferred to be equal to P_Skip or B_Skip
as specified in clause G.7.4.4.1, the value of base_mode_flag is inferred to be equal to 0.


- Otherwise (InCropWindow( CurrMbAddr ) is equal to 1 and the syntax element mb_skip_run (when
entropy_coding_mode_flag is equal to 0) or mb_skip_flag (when entropy_coding_mode_flag is equal to 1) does not
specify that mb_type is inferred to be equal to P_Skip or B_Skip), the value of base_mode_flag is inferred to be equal
to default_base_mode_flag.


When store_ref_base_pic_flag is equal to 1 and quality_id is greater than 0, base_mode_flag shall be equal to 1.


**mb_type** specifies the macroblock type. The semantics of mb_type depend on the slice type.


When mb_type is not present, it shall be inferred as follows:


- If base_mode_flag is equal to 1, mb_type is inferred to be equal to Mb_Inferred.


- Otherwise, (base_mode_flag is equal to 0), mb_type is inferred as specified in clause G.7.4.4.1.


The macroblock type Mb_Inferred specifies that the macroblock partitioning and the macroblock (partition) prediction
mode(s) are not known during the parsing process. In the decoding process specified in clause G.8, the macroblock type
used for decoding is inferred to be equal to any of the macroblock types specified in Tables 7-11, 7-13, 7-14, or G-5. For
the purpose of parsing the slice_data_in_scalable_extension( ) syntax structure including the processes specified in clause 9
and clause G.9, Mb_Inferred shall be considered an additional macroblock type that is different from all macroblock types
specified in Tables 7-11, 7-13, 7-14, and G-5 and the following applies:


- macroblocks with mb_type equal to Mb_Inferred are considered as coded in an Inter macroblock prediction mode
and not coded in an Intra macroblock prediction mode,





- NumMbPart( Mb_Inferred ) is considered to be equal to 1,


- MbPartWidth( Mb_Inferred ) and MbPartHeight( Mb_Inferred ) are considered to be equal to 16,


- MbPartPredMode( Mb_Inferred, 0 ) is considered to be not equal to Intra_4x4, Intra_8x8, Intra_16x16, Pred_L0,
Pred_L1, BiPred, and Direct.


Tables and semantics are specified for the various macroblock types for EI, EP, and EB slices. Each table presents the
value of mb_type, the name of mb_type, the number of macroblock partitions used (given by NumMbPart( mb_type )
function), the prediction mode of the macroblock (when it is not partitioned) or the first partition (given by the
MbPartPredMode( mb_type, 0 ) function) and the prediction mode of the second partition (given by the
MbPartPredMode( mb_type, 1 ) function). When a value is not applicable it is designated by "na". In the text, the value of
mb_type may be referred to as the macroblock type and a value X of MbPartPredMode( ) may be referred to in the text by
"X macroblock (partition) prediction mode" or as "X prediction macroblocks". The tables do not include the macroblock
type Mb_Inferred.


Table G-4 shows the allowed collective macroblock types for each slice_type.


**Table G-4 – Allowed collective macroblock types for slice_type**

|slice_type|allowed collective macroblock types|
|---|---|
|EI (slice)|I (see Table 7-11 and Table G-5) (macroblock types)|
|EP (slice)|P (see Table 7-13) and I (see Table 7-11 and Table G-5) (macroblock types)|
|EB (slice)|B (see Table 7-14) and I (see Table 7-11 and Table G-5) (macroblock types)|



Macroblock types that may be collectively referred to as I macroblock types are specified in Tables G-5 and 7-11. mb_type
values 0 to 25 are specified in Table 7-11. Table G-5 specifies the additional macroblock type I_BL that can be inferred in
the decoding process specified in clause G.8 for macroblocks with base_mode_flag equal to 1 (mb_type inferred to be
equal to Mb_Inferred).


The macroblock types for EI slices are all I macroblock types.


**Table G-5 – Inferred macroblock type I_BL for EI slices**

|mb_type|Name of mb_type|transform_size_8x8_flag|MbPartPredMode ( mb_type, 0 )|Intra16x16PredMode|CodedBlockPatternChroma|CodedBlockPatternLuma|
|---|---|---|---|---|---|---|
|inferred|I_BL|na|Intra_Base|na|Equation7-36|Equation7-36|



Intra_Base specifies the macroblock prediction mode and specifies that the intra prediction samples are derived using
constructed intra samples of the reference layer representation as specified in clause G.8. Intra_Base is an Intra macroblock
prediction mode.


Macroblock types that may be collectively referred to as P macroblock types are specified in Table 7-13.


The macroblock types for EP slices are specified in Tables 7-13, 7-11, and G-5. mb_type values 0 to 4 are specified in
Table 7-13 and mb_type values 5 to 30 are specified in Table 7-11, indexed by subtracting 5 from the value of mb_type.
Table G-5 specifies the additional macroblock type I_BL that can be inferred in the decoding process specified in
clause G.8 for macroblocks with base_mode_flag equal to 1 (mb_type inferred to be equal to Mb_Inferred).


Macroblock types that may be collectively referred to as B macroblock types are specified in Table 7-14.


The macroblock types for EB slices are specified in Tables 7-14, 7-11, and G-5. mb_type values 0 to 22 are specified in
Table 7-14 and mb_type values 23 to 48 are specified in Table 7-11, indexed by subtracting 23 from the value of mb_type.





Table G-5 specifies the additional macroblock type I_BL that can be inferred in the decoding process specified in
clause G.8 for macroblocks with base_mode_flag equal to 1 (mb_type inferred to be equal to Mb_Inferred).


**coded_block_pattern** specifies which of the four 8x8 luma blocks and associated chroma blocks of a macroblock may
contain non-zero transform coefficient values. When coded_block_pattern is present in the bitstream, the variables
CodedBlockPatternLuma and CodedBlockPatternChroma are derived as specified by Equation 7-36.


When scan_idx_end is less than scan_idx_start and one of the following conditions is true, the variables
CodedBlockPatternLuma and CodedBlockPatternChroma are set equal to 0:


- base_mode_flag is equal to 1,


- base_mode_flag is equal to 0, the macroblock type is not equal to P_Skip, B_Skip, or I_PCM, and the macroblock
prediction mode is not equal to Intra_16x16.


When the macroblock type is not equal to P_Skip, B_Skip, or I_PCM, the following constraints shall be obeyed:


a) When scan_idx_end is less than scan_idx_start, and the macroblock prediction mode is equal to Intra_16x16, the

bitstream shall not contain data that result in derived values of CodedBlockPatternLuma and
CodedBlockPatternChroma that are not equal to 0.


b) When scan_idx_start is equal to 0, scan_idx_end is equal to 0, and the macroblock prediction mode is equal to

Intra_16x16, the bitstream shall not contain data that result in a derived value of CodedBlockPatternLuma that is
not equal to 0.


c) When scan_idx_start is equal to 0 and scan_idx_end is equal to 0, the bitstream shall not contain data that result

in a derived value of CodedBlockPatternChroma that is equal to 2.


The meaning of CodedBlockPatternLuma and CodedBlockPatternChroma is specified in clause 7.4.5.


**residual_prediction_flag** equal to 1 specifies that the residual signal of the current macroblock is predicted as specified
in clause G.8 using the reference layer representation specified by ref_layer_dq_id. residual_prediction_flag equal to 0
specifies that the residual signal of the current macroblock is not predicted.


When the syntax element residual_prediction_flag is not present, residual_prediction_flag shall be inferred as follows:


- If all of the following conditions are true, residual_prediction_flag is inferred to be equal to
default_residual_prediction_flag:


   - slice_type is not equal to EI,


   - InCropWindow( CurrMbAddr ) is equal to 1,


   - base_mode_flag is equal to 1 or mb_type does not specify an I macroblock type.


- Otherwise, residual_prediction_flag is inferred to be equal to 0.


All elements of the arrays LumaLevel4x4, LumaLevel8x8, Intra16x16DCLevel, Intra16x16ACLevel, CbLevel4x4,
CbLevel8x8, CbIntra16x16DCLevel, CbIntra16x16ACLevel, CrLevel4x4, CrLevel8x8, CrIntra16x16DCLevel,
CrIntra16x16ACLevel, ChromaDCLevel, and ChromaACLevel are set equal to 0 before parsing the residual( ) syntax
structure. All elements of these arrays are also set equal to 0 when the residual( ) syntax structure is not present.


**G.7.4.6.1** **Macroblock prediction in scalable extension semantics**


The semantics specified in clause 7.4.5.1 apply. Additionally, the following semantics are specified.


**motion_prediction_flag_l0[** mbPartIdx **]** equal to 1 specifies that an alternative motion vector prediction process as
specified in clause G.8 is used for deriving the list 0 motion vector of the macroblock partition mbPartIdx and that the list 0
reference index of the macroblock partition mbPartIdx is inferred as specified in clause G.8.


When motion_prediction_flag_l0[ mbPartIdx ] is not present, motion_prediction_flag_l0[ mbPartIdx ] shall be inferred as
follows:


- If InCropWindow( CurrMbAddr ) is equal to 0, motion_prediction_flag_l0[ mbPartIdx ] is inferred to be equal to 0.


- Otherwise (InCropWindow( CurrMbAddr ) is equal to 1), motion_prediction_flag_l0[ mbPartIdx ] is inferred to be
equal to default_motion_prediction_flag.


**motion_prediction_flag_l1[** mbPartIdx **]** has the same semantics as motion_prediction_flag_l0[ mbPartIdx ], with l0 and
list 0 replaced by l1 and list 1, respectively.





**G.7.4.6.2** **Sub-macroblock prediction in scalable extension semantics**


The semantics specified in clause 7.4.5.2 apply. Additionally, the following semantics are specified.


**motion_prediction_flag_l0[** mbPartIdx **]** and **motion_prediction_flag_l1[** mbPartIdx **]** have the same semantics as
specified in clause G.7.4.6.1.
