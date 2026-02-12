**D.1** **SEI payload syntax**


**D.1.1** **General SEI message syntax**

|sei_payload( payloadType, payloadSize ) {|C|Descriptor|
|---|---|---|
|if( payloadType = = 0 )|||
|buffering_period( payloadSize )|5||
|else if( payloadType = = 1 )|||
|pic_timing( payloadSize )|5||
|else if( payloadType = = 2 )|||
|pan_scan_rect( payloadSize )|5||
|else if( payloadType = = 3 )|||
|<br>filler_payload( payloadSize )|5||
|else if( payloadType = = 4 )|||
|<br>user_data_registered_itu_t_t35( payloadSize )|5||
|else if( payloadType = = 5 )|||
|<br>user_data_unregistered( payloadSize )|5||
|else if( payloadType = = 6 )|||
|<br>recovery_point( payloadSize )|5||
|else if( payloadType = = 7 )|||
|<br>dec_ref_pic_marking_repetition( payloadSize )|5||
|else if( payloadType = = 8 )|||
|<br>spare_pic( payloadSize )|5||
|else if( payloadType = = 9 )|||
|<br>scene_info( payloadSize )|5||
|else if( payloadType = = 10 )|||
|<br>sub_seq_info( payloadSize )|5||
|else if( payloadType = = 11 )|||
|<br>sub_seq_layer_characteristics( payloadSize )|5||
|else if( payloadType = = 12 )|||
|<br>sub_seq_characteristics( payloadSize )|5||






|else if( payloadType = = 13 )|Col2|Col3|
|---|---|---|
|<br>full_frame_freeze( payloadSize )|5||
|else if( payloadType = = 14 )|||
|<br>full_frame_freeze_release( payloadSize )|5||
|else if( payloadType = = 15 )|||
|<br>full_frame_snapshot( payloadSize )|5||
|else if( payloadType = = 16 )|||
|<br>progressive_refinement_segment_start( payloadSize )|5||
|else if( payloadType = = 17 )|||
|<br>progressive_refinement_segment_end( payloadSize )|5||
|else if( payloadType = = 18 )|||
|<br>motion_constrained_slice_group_set( payloadSize )|5||
|else if( payloadType = = 19 )|||
|<br>film_grain_characteristics( payloadSize )|5||
|else if( payloadType = = 20 )|||
|<br>deblocking_filter_display_preference( payloadSize )|5||
|else if( payloadType = = 21 )|||
|<br>stereo_video_info( payloadSize )|5||
|else if( payloadType = = 22 )|||
|<br>post_filter_hint( payloadSize )|5||
|else if( payloadType = = 23 )|||
|<br>tone_mapping_info( payloadSize )|5||
|else if( payloadType = = 24 )|||
|<br>scalability_info( payloadSize ) /* specified in Annex G */|5||
|else if( payloadType = = 25 )|||
|<br>sub_pic_scalable_layer( payloadSize ) /* specified in Annex G */|5||
|else if( payloadType = = 26 )|||
|<br>non_required_layer_rep( payloadSize ) /* specified in Annex G */|5||
|else if( payloadType = = 27 )|||
|<br>priority_layer_info( payloadSize ) /* specified in Annex G */|5||
|else if( payloadType = = 28 )|||
|<br>layers_not_present( payloadSize ) /* specified in Annex G */|5||
|else if( payloadType = = 29 )|||
|<br>layer_dependency_change( payloadSize ) /* specified in Annex G */|5||
|else if( payloadType = = 30 )|||
|<br>scalable_nesting( payloadSize ) /* specified in Annex G */|5||
|else if( payloadType = = 31 )|||
|<br>base_layer_temporal_hrd( payloadSize ) /* specified in Annex G */|5||
|else if( payloadType = = 32 )|||
|<br>quality_layer_integrity_check( payloadSize ) /* specified in Annex G */|5||
|else if( payloadType = = 33 )|||
|<br>redundant_pic_property( payloadSize ) /* specified in Annex G */|5||
|else if( payloadType = = 34 )|||
|<br>tl0_dep_rep_index( payloadSize ) /* specified in Annex G */|5||
|else if( payloadType = = 35 )|||
|<br>tl_switching_point( payloadSize ) /* specified in Annex G */|5||
|else if( payloadType = = 36 )|||
|<br>parallel_decoding_info( payloadSize ) /* specified in Annex H */|5||





|else if( payloadType = = 37 )|Col2|Col3|
|---|---|---|
|<br>mvc_scalable_nesting( payloadSize ) /* specified in Annex H */|5||
|else if( payloadType = = 38 )|||
|<br>view_scalability_info( payloadSize ) /* specified in Annex H */|5||
|else if( payloadType = = 39 )|||
|<br>multiview_scene_info( payloadSize ) /* specified in Annex H */|5||
|else if( payloadType = = 40 )|||
|<br>multiview_acquisition_info( payloadSize ) /* specified in Annex H */|5||
|else if( payloadType = = 41 )|||
|<br>non_required_view_component( payloadSize ) /* specified in Annex H */|5||
|else if( payloadType = = 42 )|||
|<br>view_dependency_change( payloadSize ) /* specified in Annex H */|5||
|else if( payloadType = = 43 )|||
|<br>operation_points_not_present( payloadSize ) /* specified in Annex H */|5||
|else if( payloadType = = 44 )|||
|<br>base_view_temporal_hrd( payloadSize ) /* specified in Annex H */|5||
|else if( payloadType = = 45 )|||
|<br>frame_packing_arrangement( payloadSize )|5||
|else if( payloadType = = 46 )|||
|<br>multiview_view_position( payloadSize ) /* specified in Annex H */|5||
|else if( payloadType = = 47 )|||
|<br>display_orientation( payloadSize )|5||
|else if( payloadType = = 48 )|||
|<br>mvcd_scalable_nesting( payloadSize ) /* specified in Annex I */|5||
|else if( payloadType = = 49 )|||
|<br>mvcd_view_scalability_info( payloadSize ) /* specified in Annex I */|5||
|else if( payloadType = = 50 )|||
|<br>depth_representation_info( payloadSize ) /* specified in Annex I */|5||
|else if( payloadType = = 51 )|||
|<br>three_dimensional_reference_displays_info( payloadSize )<br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br>/* specified in Annex I */|5||
|else if( payloadType = = 52 )|||
|<br>depth_timing( payloadSize ) /* specified in Annex I */|5||
|else if( payloadType = = 53 )|||
|<br>depth_sampling_info( payloadSize ) /* specified in Annex I */|5||
|else if( payloadType = = 54 )|||
|<br>constrained_depth_parameter_set_identifier( payloadSize )<br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br>/* specified in Annex J */|5||
|else if( payloadType = = 56 )|||
|<br>green_metadata( payloadSize ) /* specified in ISO/IEC 23001-11 */|5||
|else if( payloadType = = 137 )|||
|<br>mastering_display_colour_volume( payloadSize )|5||
|else if( payloadType = = 142 )|||
|<br>colour_remapping_info( payloadSize )|5||
|else if( payloadType = = 144 )|||
|<br>content_light_level_info( payloadSize )|5||
|else if( payloadType = = 147 )|||
|<br>alternative_transfer_characteristics( payloadSize )|5||





|else if( payloadType = = 148 )|Col2|Col3|
|---|---|---|
|<br>ambient_viewing_environment( payloadSize )|5||
|else if( payloadType = = 149 )|||
|<br>content_colour_volume( payloadSize )|5||
|else if( payloadType = = 150 )|||
|<br>equirectangular_projection( payloadSize )|5||
|else if( payloadType = = 151 )|||
|<br>cubemap_projection( payloadSize )|5||
|else if( payloadType = = 154 )|||
|<br>sphere_rotation( payloadSize )|5||
|else if( payloadType = = 155 )|||
|<br>regionwise_packing( payloadSize )|5||
|else if( payloadType = = 156 )|||
|<br>omni_viewport( payloadSize )|5||
|else if( payloadType = = 181 )|||
|<br>alternative_depth_info( payloadSize ) /* specified in Annex I */|5||
|else if( payloadType = = 200 )|||
|<br>sei_manifest( payloadSize )|5||
|else if( payloadType = = 201 )|||
|<br>sei_prefix_indication( payloadSize )|5||
|else if( payloadType = = 202 )|||
|<br>annotated_regions( payloadSize )|5||
|else if( payloadType = = 205 )|||
|<br>shutter_interval_info( payloadSize )|5||
|else|||
|<br>reserved_sei_message( payloadSize )|5||
|if( !byte_aligned( ) ) {|||
|**bit_equal_to_one** /* equal to 1 */|5|f(1)|
|while( !byte_aligned( ) )|||
|**bit_equal_to_zero** /* equal to 0 */|5|f(1)|
|}|||
|}|||





**D.1.2** **Buffering period SEI message syntax**

|buffering_period( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**seq_parameter_set_id**|5|ue(v)|
|<br>if( NalHrdBpPresentFlag )|||
|<br> <br>for( SchedSelIdx = 0; SchedSelIdx <= cpb_cnt_minus1; SchedSelIdx++ ) {|||
|<br> <br> <br>**initial_cpb_removal_delay[** SchedSelIdx**] **|5|u(v)|
|<br> <br> <br>**initial_cpb_removal_delay_offset[** SchedSelIdx**] **|5|u(v)|
|<br> <br>}|||
|<br>if( VclHrdBpPresentFlag )|||
|<br> <br>for( SchedSelIdx = 0; SchedSelIdx <= cpb_cnt_minus1; SchedSelIdx++ ) {|||
|<br> <br> <br>**initial_cpb_removal_delay[** SchedSelIdx**] **|5|u(v)|
|<br> <br> <br>**initial_cpb_removal_delay_offset[** SchedSelIdx**] **|5|u(v)|
|<br> <br>}|||
|}|||



**D.1.3** **Picture timing SEI message syntax**

|pic_timing( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>if( CpbDpbDelaysPresentFlag ) {|||
|<br> <br>**cpb_removal_delay**|5|u(v)|
|<br> <br>**dpb_output_delay**|5|u(v)|
|<br>}|||
|<br>if( pic_struct_present_flag ) {|||
|<br> <br>**pic_struct**|5|u(4)|
|<br> <br>for( i = 0; i < NumClockTS; i++ ) {|||
|<br> <br> <br>**clock_timestamp_flag[** i**] **|5|u(1)|
|<br> <br> <br>if( clock_timestamp_flag[ i ] ) {|||
|<br> <br> <br>**ct_type**|5|u(2)|
|<br> <br> <br>**nuit_field_based_flag**|5|u(1)|
|<br> <br> <br>**counting_type**|5|u(5)|
|<br> <br> <br>**full_timestamp_flag**|5|u(1)|
|<br> <br> <br>**discontinuity_flag**|5|u(1)|
|<br> <br> <br>**cnt_dropped_flag**|5|u(1)|
|<br> <br> <br>**n_frames**|5|u(8)|
|<br> <br> <br>if( full_timestamp_flag ) {|||
|<br> <br> <br>**seconds_value** /* 0..59 */|5|u(6)|
|<br> <br> <br>**minutes_value** /* 0..59 */|5|u(6)|
|<br> <br> <br>**hours_value** /* 0..23 */|5|u(5)|
|<br> <br> <br>} else {|||
|<br> <br> <br>**seconds_flag**|5|u(1)|
|<br> <br> <br>if( seconds_flag ) {|||






|seconds_value /* range 0..59 */|5|u(6)|
|---|---|---|
|<br> <br> <br> <br> <br> <br>**minutes_flag**|5|u(1)|
|<br> <br> <br> <br> <br> <br>if( minutes_flag ) {|||
|<br> <br> <br> <br> <br> <br> <br>**minutes_value** /* 0..59 */|5|u(6)|
|<br> <br> <br> <br> <br> <br> <br>**hours_flag**|5|u(1)|
|<br> <br> <br> <br> <br> <br> <br>if( hours_flag )|||
|<br> <br> <br> <br> <br> <br> <br> <br>**hours_value** /* 0..23 */|5|u(5)|
|<br> <br> <br> <br> <br> <br>}|||
|<br> <br> <br> <br> <br>}|||
|<br> <br> <br> <br>}|||
|<br> <br> <br> <br>if( time_offset_length > 0 )|||
|<br> <br> <br> <br> <br>**time_offset**|5|i(v)|
|<br> <br> <br>}|||
|<br> <br>}|||
|<br>}|||
|}|||


**D.1.4** **Pan-scan rectangle SEI message syntax**

|pan_scan_rect( payloadSize ) {|C|Descriptor|
|---|---|---|
|**pan_scan_rect_id**|5|ue(v)|
|**pan_scan_rect_cancel_flag**|5|u(1)|
|if( !pan_scan_rect_cancel_flag ) {|||
|<br>**pan_scan_cnt_minus1**|5|ue(v)|
|<br>for( i = 0; i <= pan_scan_cnt_minus1; i++ ) {|||
|<br> <br>**pan_scan_rect_left_offset[** i**] **|5|se(v)|
|<br> <br>**pan_scan_rect_right_offset[** i**]**|5|se(v)|
|<br> <br>**pan_scan_rect_top_offset[** i**] **|5|se(v)|
|<br> <br>**pan_scan_rect_bottom_offset[** i**]**|5|se(v)|
|<br>}|||
|<br>**pan_scan_rect_repetition_period**|5|ue(v)|
|}|||
|}|||



**D.1.5** **Filler payload SEI message syntax**

|filler_payload( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>for( k = 0; k < payloadSize; k++ )|||
|<br> <br>**ff_byte** /* equal to 0xFF */|5|f(8)|
|}|||






**D.1.6** **User data registered by Rec. ITU-T T.35 SEI message syntax**

|user_data_registered_itu_t_t35( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**itu_t_t35_country_code**|5|b(8)|
|<br>if( itu_t_t35_country_code != 0xFF )|||
|<br> <br>i = 1|||
|<br>else {|||
|<br> <br>**itu_t_t35_country_code_extension_byte**|5|b(8)|
|<br> <br>i = 2|||
|<br>}|||
|<br>do {|||
|<br> <br>**itu_t_t35_payload_byte**|5|b(8)|
|<br> <br>i++|||
|<br>} while( i < payloadSize )|||
|}|||



**D.1.7** **User data unregistered SEI message syntax**

|user_data_unregistered( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**uuid_iso_iec_11578**|5|u(128)|
|<br>for( i = 16; i < payloadSize; i++ )|||
|<br> <br>**user_data_payload_byte**|5|b(8)|
|}|||



**D.1.8** **Recovery point SEI message syntax**

|recovery_point( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**recovery_frame_cnt**|5|ue(v)|
|<br>**exact_match_flag**|5|u(1)|
|<br>**broken_link_flag**|5|u(1)|
|<br>**changing_slice_group_idc**|5|u(2)|
|}|||



**D.1.9** **Decoded reference picture marking repetition SEI message syntax**

|dec_ref_pic_marking_repetition( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**original_idr_flag**|5|u(1)|
|<br>**original_frame_num**|5|ue(v)|
|<br>if( !frame_mbs_only_flag ) {<br>|||
|<br> <br>**original_field_pic_flag**|5|u(1)|
|<br> <br>if( original_field_pic_flag )|||
|<br> <br> <br>**original_bottom_field_flag**|5|u(1)|
|<br>}|||
|<br>dec_ref_pic_marking( )|5||
|}|||






**D.1.10** **Spare picture SEI message syntax**

|spare_pic( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**target_frame_num**|5|ue(v)|
|<br>**spare_field_flag**|5|u(1)|
|<br>if( spare_field_flag )|||
|<br> <br>**target_bottom_field_flag**|5|u(1)|
|<br>**num_spare_pics_minus1**|5|ue(v)|
|<br>for( i = 0; i < num_spare_pics_minus1 + 1; i++ ) {|||
|<br> <br>**delta_spare_frame_num[**i** ]**|5|ue(v)|
|<br> <br>if( spare_field_flag )|||
|<br> <br> <br>**spare_bottom_field_flag[** i**] **|5|u(1)|
|<br> <br>**spare_area_idc[**i** ]**|5|ue(v)|
|<br> <br>if( spare_area_idc[ i ] = = 1 )|||
|<br> <br> <br>for( j = 0; j < PicSizeInMapUnits; j++ )|||
|<br> <br> <br> <br>**spare_unit_flag[**i** ][**j** ]**|5|u(1)|
|<br> <br>else if( spare_area_idc[ i ] = = 2 ) {|||
|<br> <br> <br>mapUnitCnt = 0|||
|<br> <br> <br>for( j=0; mapUnitCnt < PicSizeInMapUnits; j++ ) {|||
|<br> <br> <br>**zero_run_length[**i** ][**j** ]**|5|ue(v)|
|<br> <br> <br>mapUnitCnt += zero_run_length[ i ][ j ] + 1|||
|<br> <br> <br>}|||
|<br> <br>}|||
|<br>}|||
|}|||



**D.1.11** **Scene information SEI message syntax**

|scene_info( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**scene_info_present_flag**|5|u(1)|
|<br>if( scene_info_present_flag ) {|||
|<br> <br>**scene_id**|5|ue(v)|
|<br> <br>**scene_transition_type**|5|ue(v)|
|<br> <br>if( scene_transition_type > 3 )|||
|<br> <br> <br>**second_scene_id**|5|ue(v)|
|<br>}|||
|}|||






**D.1.12** **Sub-sequence information SEI message syntax**

|sub_seq_info( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**sub_seq_layer_num**|5|ue(v)|
|<br>**sub_seq_id**|5|ue(v)|
|<br>**first_ref_pic_flag**|5|u(1)|
|<br>**leading_non_ref_pic_flag**|5|u(1)|
|<br>**last_pic_flag**|5|u(1)|
|<br>**sub_seq_frame_num_flag**|5|u(1)|
|<br>if( sub_seq_frame_num_flag )|||
|<br> <br>**sub_seq_frame_num**|5|ue(v)|
|}|||



**D.1.13** **Sub-sequence layer characteristics SEI message syntax**

|sub_seq_layer_characteristics( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**num_sub_seq_layers_minus1**|5|ue(v)|
|<br>for( layer = 0; layer <= num_sub_seq_layers_minus1; layer++ ) {|||
|<br> <br>**accurate_statistics_flag**|5|u(1)|
|<br> <br>**average_bit_rate**|5|u(16)|
|<br> <br>**average_frame_rate**|5|u(16)|
|<br>}|||
|}|||



**D.1.14** **Sub-sequence characteristics SEI message syntax**

|sub_seq_characteristics( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**sub_seq_layer_num**|5|ue(v)|
|<br>**sub_seq_id**|5|ue(v)|
|<br>**duration_flag**|5|u(1)|
|<br>if( duration_flag)|||
|<br> <br>**sub_seq_duration**|5|u(32)|
|<br>**average_rate_flag**|5|u(1)|
|<br>if( average_rate_flag ) {|||
|<br> <br>**accurate_statistics_flag**|5|u(1)|
|<br> <br>**average_bit_rate**|5|u(16)|
|<br> <br>**average_frame_rate**|5|u(16)|
|<br>}|||
|<br>**num_referenced_subseqs**|5|ue(v)|
|<br>for( n = 0; n < num_referenced_subseqs; n++ ) {|||
|<br> <br>**ref_sub_seq_layer_num**|5|ue(v)|
|<br> <br>**ref_sub_seq_id**|5|ue(v)|
|<br> <br>**ref_sub_seq_direction**|5|u(1)|
|<br>}|||
|}|||






**D.1.15** **Full-frame freeze SEI message syntax**

|full_frame_freeze( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**full_frame_freeze_repetition_period**|5|ue(v)|
|}|||



**D.1.16** **Full-frame freeze release SEI message syntax**

|full_frame_freeze_release( payloadSize ) {|C|Descriptor|
|---|---|---|
|}|||



**D.1.17** **Full-frame snapshot SEI message syntax**

|full_frame_snapshot( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**snapshot_id**|5|ue(v)|
|}|||



**D.1.18** **Progressive refinement segment start SEI message syntax**

|progressive_refinement_segment_start( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**progressive_refinement_id**|5|ue(v)|
|<br>**num_refinement_steps_minus1**|5|ue(v)|
|}|||



**D.1.19** **Progressive refinement segment end SEI message syntax**

|progressive_refinement_segment_end( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**progressive_refinement_id**|5|ue(v)|
|}|||



**D.1.20** **Motion-constrained slice group set SEI message syntax**

|motion_constrained_slice_group_set( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**num_slice_groups_in_set_minus1**|5|ue(v)|
|<br>if( num_slice_groups_minus1 > 0 )|||
|<br> <br>for( i = 0; i <= num_slice_groups_in_set_minus1; i++ )|||
|<br> <br> <br>**slice_group_id[**i** ]**|5|u(v)|
|<br>**exact_sample_value_match_flag**|5|u(1)|
|<br>**pan_scan_rect_flag**|5|u(1)|
|<br>if( pan_scan_rect_flag )|||
|<br> <br>**pan_scan_rect_id**|5|ue(v)|
|}|||






**D.1.21** **Film grain characteristics SEI message syntax**

|film_grain_characteristics( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**film_grain_characteristics_cancel_flag**|5|u(1)|
|<br>if( !film_grain_characteristics_cancel_flag ) {|||
|<br> <br>**film_grain_model_id**|5|u(2)|
|<br> <br>**separate_colour_description_present_flag**|5|u(1)|
|<br> <br>if( separate_colour_description_present_flag ) {|||
|<br> <br> <br>**film_grain_bit_depth_luma_minus8**|5|u(3)|
|<br> <br> <br>**film_grain_bit_depth_chroma_minus8**|5|u(3)|
|<br> <br> <br>**film_grain_full_range_flag**|5|u(1)|
|<br> <br> <br>**film_grain_colour_primaries**|5|u(8)|
|<br> <br> <br>**film_grain_transfer_characteristics**|5|u(8)|
|<br> <br> <br>**film_grain_matrix_coefficients**|5|u(8)|
|<br> <br>}|||
|<br> <br>**blending_mode_id**|5|u(2)|
|<br> <br>**log2_scale_factor**|5|u(4)|
|<br> <br>for( c = 0; c < 3; c++ )|||
|<br> <br> <br>**comp_model_present_flag[** c**] **|5|u(1)|
|<br> <br>for( c = 0; c < 3; c++ )|||
|<br> <br> <br>if( comp_model_present_flag[ c ] ) {|||
|<br> <br> <br> <br>**num_intensity_intervals_minus1[** c**]**|5|u(8)|
|<br> <br> <br> <br>**num_model_values_minus1[** c**]**|5|u(3)|
|<br> <br> <br> <br>for( i = 0; i <= num_intensity_intervals_minus1[ c ]; i++ ) {|||
|<br> <br> <br> <br> <br>**intensity_interval_lower_bound[** c**][** i**] **|5|u(8)|
|<br> <br> <br> <br> <br>**intensity_interval_upper_bound[** c**][** i**]**|5|u(8)|
|<br> <br> <br> <br> <br>for( j = 0; j <= num_model_values_minus1[ c ]; j++ )|||
|<br> <br> <br> <br> <br> <br>**comp_model_value[** c**][** i**][** j**]**|5|se(v)|
|<br> <br> <br> <br>}|||
|<br> <br> <br>}|||
|<br>**film_grain_characteristics_repetition_period**|5|ue(v)|
|<br>}|||
|}|||



**D.1.22** **Deblocking filter display preference SEI message syntax**

|deblocking_filter_display_preference( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**deblocking_display_preference_cancel_flag**|5|u(1)|
|<br>if( !deblocking_display_preference_cancel_flag ) {|||
|<br> <br>**display_prior_to_deblocking_preferred_flag**|5|u(1)|
|<br> <br>**dec_frame_buffering_constraint_flag**|5|u(1)|
|<br> <br>**deblocking_display_preference_repetition_period**|5|ue(v)|
|<br>}|||
|}|||






**D.1.23** **Stereo video information SEI message syntax**

|stereo_video_info( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**field_views_flag**|5|u(1)|
|<br>if( field_views_flag )|||
|<br> <br>**top_field_is_left_view_flag**|5|u(1)|
|<br>else {|||
|<br> <br>**current_frame_is_left_view_flag**|5|u(1)|
|<br> <br>**next_frame_is_second_view_flag**|5|u(1)|
|<br>}|||
|<br>**left_view_self_contained_flag**|5|u(1)|
|<br>**right_view_self_contained_flag**|5|u(1)|
|}|||



**D.1.24** **Post-filter hint SEI message syntax**

|post_filter_hint( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**filter_hint_size_y**|5|ue(v)|
|<br>**filter_hint_size_x**|5|ue(v)|
|<br>**filter_hint_type**|5|u(2)|
|<br>for( colour_component = 0; colour_component < 3; colour_component ++ )|||
|<br> <br>for( cy = 0; cy < filter_hint_size_y; cy ++ )|||
|<br> <br> <br>for( cx = 0; cx < filter_hint_size_x; cx ++ )|||
|<br> <br> <br> <br>**filter_hint[** colour_component**][** cy**][** cx**]**|5|se(v)|
|<br>**additional_extension_flag**|5|u(1)|
|}|||






**D.1.25** **Tone mapping information SEI message syntax**

|tone_mapping_info( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**tone_map_id**|5|ue(v)|
|<br>**tone_map_cancel_flag**|5|u(1)|
|<br>if( !tone_map_cancel_flag ) {|||
|<br> <br>**tone_map_repetition_period**|5|ue(v)|
|<br> <br>**coded_data_bit_depth**|5|u(8)|
|<br> <br>**target_bit_depth**|5|u(8)|
|<br> <br>**tone_map_model_id**|5|ue(v)|
|<br> <br>if( tone_map_model_id = = 0 ) {|||
|<br> <br> <br>**min_value**|5|u(32)|
|<br> <br> <br>**max_value**|5|u(32)|
|<br> <br>}|||
|<br> <br>if( tone_map_model_id = = 1 ) {|||
|<br> <br> <br>**sigmoid_midpoint**|5|u(32)|
|<br> <br> <br>**sigmoid_width**|5|u(32)|
|<br> <br>}|||
|<br> <br>if( tone_map_model_id = = 2 )|||
|<br> <br> <br>for( i = 0; i < ( 1 << target_bit_depth ); i++ )|||
|<br> <br> <br> <br>**start_of_coded_interval[** i**] **|5|u(v)|
|<br> <br>if( tone_map_model_id = = 3 ) {|||
|<br> <br> <br>**num_pivots**|5|u(16)|
|<br> <br> <br>for( i=0; i < num_pivots; i++ ) {|||
|<br> <br> <br> <br>**coded_pivot_value[** i**]**|5|u(v)|
|<br> <br> <br> <br>**target_pivot_value[** i**]**|5|u(v)|
|<br> <br> <br>}|||
|<br> <br>}|||
|<br> <br>if( tone_map_model_id = = 4 ) {|||
|<br> <br> <br>**camera_iso_speed_idc**|5|u(8)|
|<br> <br> <br>if( camera_iso_speed_idc = = Extended_ISO )|||
|<br> <br> <br> <br>**camera_iso_speed_value**|5|u(32)|
|<br> <br> <br>**exposure_index_idc**|5|u(8)|
|<br> <br> <br>if( exposure_index_idc = = Extended_ISO )|||
|<br> <br> <br> <br>**exposure_index_value**|5|u(32)|
|<br> <br> <br>**exposure_compensation_value_sign_flag**|5|u(1)|
|<br> <br> <br>**exposure_compensation_value_numerator**|5|u(16)|
|<br> <br> <br>**exposure_compensation_value_denom_idc**|5|u(16)|
|<br> <br> <br>**ref_screen_luminance_white**|5|u(32)|
|<br> <br> <br>**extended_range_white_level**|5|u(32)|
|<br> <br> <br>**nominal_black_level_luma_code_value**|5|u(16)|
|<br> <br> <br>**nominal_white_level_luma_code_value**|5|u(16)|
|<br> <br> <br>**extended_white_level_luma_code_value**|5|u(16)|
|<br> <br>}|||
|<br>}|||
|}|||






**D.1.26** **Frame packing arrangement SEI message syntax**

|frame_packing_arrangement( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**frame_packing_arrangement_id**|5|ue(v)|
|<br>**frame_packing_arrangement_cancel_flag**|5|u(1)|
|<br>if( !frame_packing_arrangement_cancel_flag ) {|||
|<br> <br>**frame_packing_arrangement_type**|5|u(7)|
|<br> <br>**quincunx_sampling_flag**|5|u(1)|
|<br> <br>**content_interpretation_type**|5|u(6)|
|<br> <br>**spatial_flipping_flag**|5|u(1)|
|<br> <br>**frame0_flipped_flag**|5|u(1)|
|<br> <br>**field_views_flag**|5|u(1)|
|<br> <br>**current_frame_is_frame0_flag**|5|u(1)|
|<br> <br>**frame0_self_contained_flag**|5|u(1)|
|<br> <br>**frame1_self_contained_flag**|5|u(1)|
|<br> <br>if( !quincunx_sampling_flag &&<br> <br> <br> <br>frame_packing_arrangement_type != 5 ) {|||
|<br> <br> <br>**frame0_grid_position_x**|5|u(4)|
|<br> <br> <br>**frame0_grid_position_y**|5|u(4)|
|<br> <br> <br>**frame1_grid_position_x**|5|u(4)|
|<br> <br> <br>**frame1_grid_position_y**|5|u(4)|
|<br> <br>}|||
|<br> <br>**frame_packing_arrangement_reserved_byte**|5|u(8)|
|<br> <br>**frame_packing_arrangement_repetition_period**|5|ue(v)|
|<br>}|||
|<br>**frame_packing_arrangement_extension_flag**|5|u(1)|
|}|||



**D.1.27** **Display orientation SEI message syntax**

|display_orientation( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**display_orientation_cancel_flag**|5|u(1)|
|<br>if( !display_orientation_cancel_flag ) {|||
|<br> <br>**hor_flip**|5|u(1)|
|<br> <br>**ver_flip**|5|u(1)|
|<br> <br>**anticlockwise_rotation**|5|u(16)|
|<br> <br>**display_orientation_repetition_period**|5|ue(v)|
|<br> <br>**display_orientation_extension_flag**|5|u(1)|
|<br>}|||
|}|||



**D.1.28** **Green metadata SEI message syntax**


The syntax for this SEI message is specified in ISO/IEC 23001-11 (Green metadata), which facilitates reduced power
consumption in decoders, encoders, displays, and in media selection.





**D.1.29** **Mastering display colour volume SEI message syntax**

|mastering_display_colour_volume( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>for( c = 0; c < 3; c++ ) {|||
|<br> <br>**display_primaries_x[** c**]**|5|u(16)|
|<br> <br>**display_primaries_y[** c**] **|5|u(16)|
|<br>}|||
|<br>**white_point_x**|5|u(16)|
|<br>**white_point_y**|5|u(16)|
|<br>**max_display_mastering_luminance**|5|u(32)<br>|
|<br>**min_display_mastering_luminance**|5|u(32)|
|}|||






**D.1.30** **Colour remapping information SEI message syntax**

|colour_remapping_info( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**colour_remap_id**|5|ue(v)|
|<br>**colour_remap_cancel_flag**|5|u(1)|
|<br>if( !colour_remap_cancel_flag ) {|||
|<br> <br>**colour_remap_repetition_period**|5|ue(v)|
|<br> <br>**colour_remap_video_signal_info_present_flag**|5|u(1)|
|<br> <br>if( colour_remap_video_signal_info_present_flag ) {|||
|<br> <br> <br>**colour_remap_full_range_flag**|5|u(1)|
|<br> <br> <br>**colour_remap_primaries**|5|u(8)|
|<br> <br> <br>**colour_remap_transfer_function**|5|u(8)|
|<br> <br> <br>**colour_remap_matrix_coefficients**|5|u(8)|
|<br> <br>}|||
|<br> <br>**colour_remap_input_bit_depth**|5|u(8)|
|<br> <br>**colour_remap_output_bit_depth**|5|u(8)|
|<br> <br>for( c = 0; c < 3; c++ ) {|||
|<br> <br> <br>**pre_lut_num_val_minus1[** c**]**|5|u(8)|
|<br> <br> <br>if( pre_lut_num_val_minus1[ c ] > 0 )|||
|<br> <br> <br> <br>for( i = 0; i <= pre_lut_num_val_minus1[ c ]; i++ ) {|||
|<br> <br> <br> <br> <br>**pre_lut_coded_value[** c**][** i**]**|5|u(v)|
|<br> <br> <br> <br> <br>**pre_lut_target_value[** c**][** i**]**|5|u(v)|
|<br> <br> <br> <br>}|||
|<br> <br>}|||
|<br> <br>**colour_remap_matrix_present_flag**|5|u(1)|
|<br> <br>if( colour_remap_matrix_present_flag ) {|||
|<br> <br> <br>**log2_matrix_denom**|5|u(4)|
|<br> <br> <br>for( c = 0; c < 3; c++ )|||
|<br> <br> <br> <br>for( i = 0; i < 3; i++ )|||
|<br> <br> <br> <br> <br>**colour_remap_coeffs[** c**][** i**]**|5|se(v)|
|<br> <br>}|||
|<br> <br>for( c = 0; c < 3; c++ ) {|||
|<br> <br> <br>**post_lut_num_val_minus1[** c**]**|5|u(8)|
|<br> <br> <br>if( post_lut_num_val_minus1[ c ] > 0 )|||
|<br> <br> <br> <br>for( i = 0; i <= post_lut_num_val_minus1[ c ]; i++ ) {|||
|<br> <br> <br> <br> <br>**post_lut_coded_value[** c**][** i**]**|5|u(v)|
|<br> <br> <br> <br> <br>**post_lut_target_value[** c**][** i**]**|5|u(v)|
|<br> <br> <br> <br>}|||
|<br> <br>}|||
|<br>}|||
|}|||






**D.1.31** **Content light level information SEI message syntax**

|content_light_level_info( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**max_content_light_level**|5|u(16)|
|<br>**max_pic_average_light_level**|5|u(16)|
|}|||



**D.1.32** **Alternative transfer characteristics SEI message syntax**

|alternative_transfer_characteristics( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**preferred_transfer_characteristics**|5|u(8)|
|}|||



**D.1.33** **Content colour volume SEI message syntax**

|content_colour_volume( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**ccv_cancel_flag**|5|u(1)|
|<br>if( !ccv_cancel_flag ) {|||
|<br> <br>**ccv_persistence_flag**|5|u(1)|
|<br> <br>**ccv_primaries_present_flag**|5|u(1)|
|<br> <br>**ccv_min_luminance_value_present_flag**|5|u(1)|
|<br> <br>**ccv_max_luminance_value_present_flag**|5|u(1)|
|<br> <br>**ccv_avg_luminance_value_present_flag**|5|u(1)|
|<br> <br>**ccv_reserved_zero_2bits**|5|u(2)|
|<br> <br>if( ccv_primaries_present_flag )|||
|<br> <br> <br>for( c = 0; c < 3; c++ ) {|||
|<br> <br> <br> <br>**ccv_primaries_x[** c**]**|5|i(32)|
|<br> <br> <br> <br>**ccv_primaries_y[** c**]**|5|i(32)|
|<br> <br> <br>}|||
|<br> <br>if( ccv_min_luminance_value_present_flag )|||
|<br> <br> <br>**ccv_min_luminance_value**|5|u(32)|
|<br> <br>if( ccv_max_luminance_value_present_flag )|||
|<br> <br> <br>**ccv_max_luminance_value**|5|u(32)|
|<br> <br>if( ccv_avg_luminance_value_present_flag )|||
|<br> <br> <br>**ccv_avg_luminance_value**|5|u(32)|
|<br>}|||
|}|||



**D.1.34** **Ambient viewing environment SEI message syntax**

|ambient_viewing_environment( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**ambient_illuminance**|5|u(32)|
|<br>**ambient_light_x**|5|u(16)|
|<br>**ambient_light_y**|5|u(16)|
|}|||






**D.1.35** **Syntax of omnidirectional video specific SEI messages**


**D.1.35.1** **Equirectangular projection SEI message syntax**

|equirectangular_projection( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**erp_cancel_flag**|5|u(1)|
|<br>if( !erp_cancel_flag )|||
|<br> <br>**erp_persistence_flag**|5|u(1)|
|<br> <br>**erp_padding_flag**|5|u(1)|
|<br> <br>**erp**_**reserved_zero_2bits**|5|u(2)|
|<br> <br>if( erp_padding_flag = = 1 ) {|||
|<br> <br> <br>**gp_erp_type**|5|u(3)|
|<br> <br> <br>**left_gb_erp_width**|5|u(8)|
|<br> <br> <br>**right_gb_erp_width**|5|u(8)|
|<br> <br>}|||
|<br>}|||
|}|||



**D.1.35.2** **Cubemap projection SEI message syntax**

|cubemap_projection( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**cmp_cancel_flag**|5|u(1)|
|<br>if( !cmp_cancel_flag )|||
|<br> <br>**cmp_persistence_flag**|5|u(1)|
|}|||



**D.1.35.3** **Sphere rotation SEI message syntax**

|sphere_rotation( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**sphere_rotation_cancel_flag**|5|u(1)|
|<br>if( !sphere_rotation_cancel_flag ) {|||
|<br> <br>**sphere_rotation_persistence_flag**|5|u(1)|
|<br> <br>**sphere_rotation_reserved_zero_6bits**|5|u(6)|
|<br> <br>**yaw_rotation**|5|i(32)|
|<br> <br>**pitch_rotation**|5|i(32)|
|<br> <br>**roll_rotation**|5|i(32)|
|<br>}|||
|}|||






**D.1.35.4** **Region-wise packing SEI message syntax**

|regionwise_packing( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**rwp_cancel_flag**|5|u(1)|
|<br>if( !rwp_cancel_flag ) {|||
|<br> <br>**rwp_persistence_flag**|5|u(1)|
|<br> <br>**constituent_picture_matching_flag**|5|u(1)|
|<br> <br>**rwp_reserved_zero_5bits**|5|u(5)|
|<br> <br>**num_packed_regions**|5|u(8)|
|<br> <br>**proj_picture_width**|5|u(32)|
|<br> <br>**proj_picture_height**|5|u(32)|
|<br> <br>**packed_picture_width**|5|u(16)|
|<br> <br>**packed_picture_height**|5|u(16)|
|<br> <br>for( i = 0; i < num_packed_regions; i++ ) {|||
|<br> <br> <br>**rwp_reserved_zero_4bits[** i**] **|5|u(4)|
|<br> <br> <br>**transform_type[** i**] **|5|u(3)|
|<br> <br> <br>**guard_band_flag[** i**] **|5|u(1)|
|<br> <br> <br>**proj_region_width[** i**] **|5|u(32)|
|<br> <br> <br>**proj_region_height[** i**]**|5|u(32)|
|<br> <br> <br>**proj_region_top[** i**] **|5|u(32)|
|<br> <br> <br>**proj_region_left[** i**]**|5|u(32)|
|<br> <br> <br>**packed_region_width[** i**] **|5|u(16)|
|<br> <br> <br>**packed_region_height[** i**]**|5|u(16)|
|<br> <br> <br>**packed_region_top[** i**] **|5|u(16)|
|<br> <br> <br>**packed_region_left[** i**]**|5|u(16)|
|<br> <br> <br>if( guard_band_flag[ i ] ) {|||
|<br> <br> <br> <br>**left_gb_width[** i**]**|5|u(8)|
|<br> <br> <br> <br>**right_gb_width[** i**]**|5|u(8)|
|<br> <br> <br> <br>**top_gb_height[** i**]**|5|u(8)|
|<br> <br> <br> <br>**bottom_gb_height[** i**]**|5|u(8)|
|<br> <br> <br> <br>**gb_not_used_for_pred_flag[** i**]**|5|u(1)|
|<br> <br> <br> <br>for( j = 0; j < 4; j++ )|||
|<br> <br> <br> <br> <br>**gb_type[** i**][** j**]**|5|u(3)|
|<br> <br> <br> <br>**rwp_gb_reserved_zero_3bits[** i**]**|5|u(3)|
|<br> <br> <br>}|||
|<br> <br>}|||
|<br>}|||
|}|||






**D.1.35.5** **Omnidirectional viewport SEI message syntax**

|omni_viewport( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**omni_viewport_id**|5|u(10)|
|<br>**omni_viewport_cancel_flag**|5|u(1)|
|<br>if( !omni_viewport_cancel_flag ) {|||
|<br> <br>**omni_viewport_persistence_flag**|5|u(1)|
|<br> <br>**omni_viewport_cnt_minus1**|5|u(4)|
|<br> <br>for( i = 0; i <= omni_viewport_cnt_minus1; i++ ) {|||
|<br> <br> <br>**omni_viewport_azimuth_centre[** i**] **|5|i(32)|
|<br> <br> <br>**omni_viewport_elevation_centre[** i**] **|5|i(32)|
|<br> <br> <br>**omni_viewport_tilt_centre[** i**] **|5|i(32)|
|<br> <br> <br>**omni_viewport_hor_range[** i**] **|5|u(32)|
|<br> <br> <br>**omni_viewport_ver_range[** i**] **|5|u(32)|
|<br> <br>}|||
|<br>}|||
|}|||



**D.1.36** **SEI manifest SEI message syntax**

|sei_manifest( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**manifest_num_sei_msg_types**|5|u(16)|
|<br>for( i = 0; i < manifest_num_sei_msg_types; i++ ) {|||
|<br> <br>**manifest_sei_payload_type[** i**] **|5|u(16)|
|<br> <br>**manifest_sei_description[** i**] **|5|u(8)|
|<br>}|||
|}|||



**D.1.37** **SEI prefix indication SEI message syntax**

|sei_prefix_indication( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**prefix_sei_payload_type**|5|u(16)|
|<br>**num_sei_prefix_indications_minus1**|5|u(8)|
|<br>for( i = 0; i <= num_sei_prefix_indications_minus1; i++ ) {|||
|<br> <br>**num_bits_in_prefix_indication_minus1[** i**] **|5|u(16)|
|<br> <br>for( j = 0; j <= num_bits_in_prefix_indication_minus1[ i ]; j++ )|||
|<br> <br> <br>**sei_prefix_data_bit[** i**][** j**] **|5|u(1)|
|<br> <br>while( !byte_aligned( ) )|||
|<br> <br> <br>**byte_alignment_bit_equal_to_one** /* equal to 1 */|5|f(1)|
|<br>}|||
|}|||






**D.1.38** **Annotated regions SEI message syntax**

|annotated_regions( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**ar_cancel_flag**|5|u(1)|
|<br>if( !ar_cancel_flag ) {|||
|<br> <br>**ar_not_optimized_for_viewing_flag**|5|u(1)|
|<br> <br>**ar_true_motion_flag**|5|u(1)|
|<br> <br>**ar_occluded_object_flag**|5|u(1)|
|<br> <br>**ar_partial_object_flag_present_flag**|5|u(1)|
|<br> <br>**ar_object_label_present_flag**|5|u(1)|
|<br> <br>**ar_object_confidence_info_present_flag**|5|u(1)|
|<br> <br>if( ar_object_confidence_info_present_flag )|||
|<br> <br> <br>**ar_object_confidence_length_minus1**|5|u(4)|
|<br> <br>if( ar_object_label_present_flag ) {|||
|<br> <br> <br>**ar_object_label_language_present_flag**|5|u(1)|
|<br> <br> <br>if( ar_object_label_language_present_flag ) {|||
|<br> <br> <br> <br>while( !byte_aligned( ) )|||
|<br> <br> <br> <br> <br>**ar_bit_equal_to_zero** /* equal to 0 */|5|f(1)|
|<br> <br> <br> <br>**ar_object_label_language**|5|st(v)|
|<br> <br> <br>}|||
|<br> <br> <br>**ar_num_label_updates**|5|ue(v)|
|<br> <br> <br>for( i = 0; i < ar_num_label_updates; i++ ) {|||
|<br> <br> <br> <br>**ar_label_idx[** i**] **|5|ue(v)|
|<br> <br> <br> <br>**ar_label_cancel_flag**|5|u(1)|
|<br> <br> <br> <br>LabelAssigned[ ar_label_idx[ i ] ] = !ar_label_cancel_flag|||
|<br> <br> <br> <br>if( !ar_label_cancel_flag ) {|||
|<br> <br> <br> <br> <br>while( !byte_aligned( ) )|||
|<br> <br> <br> <br> <br> <br>**ar_bit_equal_to_zero** /* equal to 0 */|5|f(1)|
|<br> <br> <br> <br> <br>**ar_label[** ar_label_idx[ i ]**] **|5|st(v)|
|<br> <br> <br> <br>}|||
|<br> <br> <br>}|||
|<br> <br>}|||
|<br> <br>**ar_num_object_updates**|5|ue(v)|
|<br> <br>for( i = 0; i < ar_num_object_updates; i++ ) {|||
|<br> <br> <br>**ar_object_idx[** i**]**|5|ue(v)|
|<br> <br> <br>**ar_object_cancel_flag**|5|u(1)|
|<br> <br> <br>ObjectTracked[ ar_object_idx[ i ] ] = !ar_object_cancel_flag|||
|<br> <br> <br>if( !ar_object_cancel_flag ) {|||
|<br> <br> <br> <br>if( ar_object_label_present_flag ) {|||
|<br> <br> <br> <br> <br>**ar_object_label_update_flag**|5|u(1)|
|<br> <br> <br> <br> <br>if( ar_object_label_update_flag )|||
|<br> <br> <br> <br> <br> <br>**ar_object_label_idx[** ar_object_idx[ i ]**]**|5|ue(v)|
|<br> <br> <br> <br>}|||
|<br> <br> <br> <br>**ar_bounding_box_update_flag**|5|u(1)|
|<br> <br> <br> <br>if( ar_bounding_box_update_flag ) {|||
|<br> <br> <br> <br> <br>**ar_bounding_box_cancel_flag**|5|u(1)|
|<br> <br> <br> <br> <br>ObjectBoundingBoxAvail[ ar_object_idx[ i ] ] =<br> <br> <br> <br> <br> <br> <br> <br>!ar_bounding_box_cancel_flag|||






|if( !ar_bounding_box_cancel_flag ) {|Col2|Col3|
|---|---|---|
|<br> <br> <br> <br> <br> <br>**ar_bounding_box_top[** ar_object_idx[ i ]**]**|5|u(16)|
|<br> <br> <br> <br> <br> <br>**ar_bounding_box_left[** ar_object_idx[ i ]**]**|5|u(16)|
|<br> <br> <br> <br> <br> <br>**ar_bounding_box_width[** ar_object_idx[ i ]**]**|5|u(16)|
|<br> <br> <br> <br> <br> <br>**ar_bounding_box_height[** ar_object_idx[ i ]**]**|5|u(16)|
|<br> <br> <br> <br> <br> <br>if( ar_partial_object_flag_present_flag )|||
|<br> <br> <br> <br> <br> <br> <br>**ar_partial_object_flag[** ar_object_idx[ i ]**] **|5|u(1)|
|<br> <br> <br> <br> <br> <br>if( ar_object_confidence_info_present_flag )|||
|<br> <br> <br> <br> <br> <br> <br>**ar_object_confidence[** ar_object_idx[ i ]**]**|5|u(v)|
|<br> <br> <br> <br> <br>}|||
|<br> <br> <br> <br>}|||
|<br> <br> <br>}|||
|<br> <br>}|||
|<br>}|||
|}|||


**D.1.39** **Shutter interval information SEI message syntax**

|shutter_interval_info( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**sii_sub_layer_idx**|5|ue(v)|
|<br>if( sii_sub_layer_idx = = 0 )|||
|<br> <br>**shutter_interval_info_present_flag**|5|u(1)|
|<br> <br>if( shutter_interval_info_present_flag )|||
|<br> <br> <br>**sii_time_scale**|5|u(32)|
|<br> <br> <br>**fixed_shutter_interval_within_cvs_flag**|5|u(1)|
|<br> <br> <br>if( fixed_shutter_interval_within_cvs_flag )|||
|<br> <br> <br> <br>**sii_num_units_in_shutter_interval**|5|u(32)|
|<br> <br> <br>else {|||
|<br> <br> <br> <br>**sii_max_sub_layers_minus1**|5|u(3)|
|<br> <br> <br> <br>for( i = 0; i <= sii_max_sub_layers_minus1; i++ )|||
|<br> <br> <br> <br> <br>**sub_layer_num_units_in_shutter_interval[** i**] **|5|u(32)|
|<br> <br> <br>}|||
|<br> <br>}|||
|<br>}|||
|}|||



**D.1.40** **Reserved SEI message syntax**

|reserved_sei_message( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>for( i = 0; i < payloadSize; i++ )|||
|<br> <br>**reserved_sei_message_payload_byte**|5|b(8)|
|}|||
