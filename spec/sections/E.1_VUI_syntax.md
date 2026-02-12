**E.1** **VUI syntax**


**E.1.1** **VUI parameters syntax**

|vui_parameters( ) {|C|Descriptor|
|---|---|---|
|<br>**aspect_ratio_info_present_flag**|0|u(1)|
|<br>if( aspect_ratio_info_present_flag ) {|||
|<br> <br>**aspect_ratio_idc**|0|u(8)|
|<br> <br>if( aspect_ratio_idc = = Extended_SAR ) {|||
|<br> <br> <br>**sar_width**|0|u(16)|
|<br> <br> <br>**sar_height**|0|u(16)|
|<br> <br>}|||
|<br>}|||
|<br>**overscan_info_present_flag**|0|u(1)|
|<br>if( overscan_info_present_flag )|||
|<br> <br>**overscan_appropriate_flag**|0|u(1)|
|<br>**video_signal_type_present_flag**|0|u(1)|
|<br>if( video_signal_type_present_flag ) {|||
|<br> <br>**video_format**|0|u(3)|
|<br> <br>**video_full_range_flag**|0|u(1)|
|<br> <br>**colour_description_present_flag**|0|u(1)|
|<br> <br>if( colour_description_present_flag ) {|||
|<br> <br> <br>**colour_primaries**|0|u(8)|
|<br> <br> <br>**transfer_characteristics**|0|u(8)|
|<br> <br> <br>**matrix_coefficients**|0|u(8)|
|<br> <br>}|||
|<br>}|||
|<br>**chroma_loc_info_present_flag**|0|u(1)|
|<br>if( chroma_loc_info_present_flag ) {|||
|<br> <br>**chroma_sample_loc_type_top_field**|0|ue(v)|
|<br> <br>**chroma_sample_loc_type_bottom_field**|0|ue(v)|
|<br>}|||






|timing_info_present_flag|0|u(1)|
|---|---|---|
|<br>if( timing_info_present_flag ) {|||
|<br> <br>**num_units_in_tick**|0|u(32)|
|<br> <br>**time_scale**|0|u(32)|
|<br> <br>**fixed_frame_rate_flag**|0|u(1)|
|<br>}|||
|<br>**nal_hrd_parameters_present_flag**|0|u(1)|
|<br>if( nal_hrd_parameters_present_flag )|||
|<br> <br>hrd_parameters( )|0||
|<br>**vcl_hrd_parameters_present_flag**|0|u(1)|
|<br>if( vcl_hrd_parameters_present_flag )|||
|<br> <br>hrd_parameters( )|0||
|<br>if( nal_hrd_parameters_present_flag | | vcl_hrd_parameters_present_flag )|||
|<br> <br>**low_delay_hrd_flag**|0|u(1)|
|<br>**pic_struct_present_flag**|0|u(1)|
|<br>**bitstream_restriction_flag**|0|u(1)|
|<br>if( bitstream_restriction_flag ) {|||
|<br> <br>**motion_vectors_over_pic_boundaries_flag**|0|u(1)|
|<br> <br>**max_bytes_per_pic_denom**|0|ue(v)|
|<br> <br>**max_bits_per_mb_denom**|0|ue(v)|
|<br> <br>**log2_max_mv_length_horizontal**|0|ue(v)|
|<br> <br>**log2_max_mv_length_vertical**|0|ue(v)|
|<br> <br>**max_num_reorder_frames**|0|ue(v)|
|<br> <br>**max_dec_frame_buffering**|0|ue(v)|
|<br>}|||
|}|||


**E.1.2** **HRD parameters syntax**

|hrd_parameters( ) {|C|Descriptor|
|---|---|---|
|<br>**cpb_cnt_minus1**|0 | 5|ue(v)|
|<br>**bit_rate_scale**|0 | 5|u(4)|
|<br>**cpb_size_scale**|0 | 5|u(4)|
|<br>for( SchedSelIdx = 0; SchedSelIdx <= cpb_cnt_minus1; SchedSelIdx++ ) {|||
|<br> <br>**bit_rate_value_minus1[** SchedSelIdx**]**|0 | 5|ue(v)|
|<br> <br>**cpb_size_value_minus1[** SchedSelIdx**]**|0 | 5|ue(v)|
|<br> <br>**cbr_flag[** SchedSelIdx**]**|0 | 5|u(1)|
|<br>}|||
|<br>**initial_cpb_removal_delay_length_minus1**|0 | 5|u(5)|
|<br>**cpb_removal_delay_length_minus1**|0 | 5|u(5)|
|<br>**dpb_output_delay_length_minus1**|0 | 5|u(5)|
|<br>**time_offset_length**|0 | 5|u(5)|
|}|||
