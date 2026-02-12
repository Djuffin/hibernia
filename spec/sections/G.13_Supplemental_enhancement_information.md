**G.13** **Supplemental enhancement information**


The specifications in Annex D together with the extensions and modifications specified in this clause apply.


**G.13.1** **SEI payload syntax**





**G.13.1.1Scalability information SEI message syntax**

|scalability_info( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**temporal_id_nesting_flag**|5|u(1)|
|<br>**priority_layer_info_present_flag**|5|u(1)|
|<br>**priority_id_setting_flag**|5|u(1)|
|<br>**num_layers_minus1**|5|ue(v)|
|<br>for( i = 0; i <= num_layers_minus1; i++ ) {|||
|<br> <br>**layer_id**[ i ]|5|ue(v)|
|<br> <br>**priority_id**[ i ]|5|u(6)|
|<br> <br>**discardable_flag**[ i ]|5|u(1)|
|<br> <br>**dependency_id**[ i ]|5|u(3)|
|<br> <br>**quality_id**[ i ]|5|u(4)|
|<br> <br>**temporal_id**[ i ]|5|u(3)|
|<br> <br>**sub_pic_layer_flag**[ i ]|5|u(1)|
|<br> <br>**sub_region_layer_flag**[ i ]|5|u(1)|
|<br> <br>**iroi_division_info_present_flag**[ i ]|5|u(1)|
|<br> <br>**profile_level_info_present_flag**[ i ]|5|u(1)|
|<br> <br>**bitrate_info_present_flag**[ i ]|5|u(1)|
|<br> <br>**frm_rate_info_present_flag**[ i ]|5|u(1)|
|<br> <br>**frm_size_info_present_flag**[ i ]|5|u(1)|
|<br> <br>**layer_dependency_info_present_flag**[ i ]|5|u(1)|
|<br> <br>**parameter_sets_info_present_flag**[ i ]|5|u(1)|
|<br> <br>**bitstream_restriction_info_present_flag**[ i ]|5|u(1)|
|<br> <br>**exact_inter_layer_pred_flag**[ i ]|5|u(1)|
|<br> <br>if( sub_pic_layer_flag[ i ] | | iroi_division_info_present_flag[ i ] )|||
|<br> <br> <br>**exact_sample_value_match_flag**[ i ]|5|u(1)|
|<br> <br>**layer_conversion_flag**[ i ]|5|u(1)|
|<br> <br>**layer_output_flag**[ i ]|5|u(1)|
|<br> <br>if( profile_level_info_present_flag[ i ] )|||
|<br> <br> <br>**layer_profile_level_idc**[ i ]|5|u(24)|
|<br> <br>if( bitrate_info_present_flag[ i ] ) {|||
|<br> <br> <br>**avg_bitrate**[ i ]|5|u(16)|
|<br> <br> <br>**max_bitrate_layer**[ i ]|5|u(16)|
|<br> <br> <br>**max_bitrate_layer_representation**[ i ]|5|u(16)|
|<br> <br> <br>**max_bitrate_calc_window**[ i ]|5|u(16)|
|<br> <br>}|||
|<br> <br>if( frm_rate_info_present_flag[ i ] ) {|||
|<br> <br> <br>**constant_frm_rate_idc**[ i ]|5|u(2)|
|<br> <br> <br>**avg_frm_rate**[ i ]|5|u(16)|
|<br> <br>}|||
|<br> <br>if( frm_size_info_present_flag[ i ] | |<br> <br> <br> <br>iroi_division_info_present_flag[ i ] ) {|||
|<br> <br> <br>**frm_width_in_mbs_minus1**[ i ]|5|ue(v)|
|<br> <br> <br>**frm_height_in_mbs_minus1**[ i ]|5|ue(v)|
|<br> <br>}|||
|<br> <br>if( sub_region_layer_flag[ i ] ) {|||
|<br> <br> <br>**base_region_layer_id**[ i ]|5|ue(v)|






|dynamic_rect_flag[ i ]|5|u(1)|
|---|---|---|
|<br> <br> <br>if( !dynamic_rect_flag[ i ] ) {|||
|<br> <br> <br> <br>**horizontal_offset**[ i ]|5|u(16)|
|<br> <br> <br> <br>**vertical_offset**[ i ]|5|u(16)|
|<br> <br> <br> <br>**region_width**[ i ]|5|u(16)|
|<br> <br> <br> <br>**region_height**[ i ]|5|u(16)|
|<br> <br> <br>}|||
|<br> <br>}|||
|<br> <br>if( sub_pic_layer_flag[ i ] )|||
|<br> <br> <br>**roi_id**[ i ]|5|ue(v)|
|<br> <br>if( iroi_division_info_present_flag[ i ] ) {|||
|<br> <br> <br>**iroi_grid_flag**[ i ]|5|u(1)|
|<br> <br> <br>if( iroi_grid_flag[ i ] ) {|||
|<br> <br> <br> <br>**grid_width_in_mbs_minus1**[ i ]|5|ue(v)|
|<br> <br> <br> <br>**grid_height_in_mbs_minus1**[ i ]|5|ue(v)|
|<br> <br> <br>} else {|||
|<br> <br> <br> <br>**num_rois_minus1**[ i ]|5|ue(v)|
|<br> <br> <br> <br>for(j = 0; j <= num_rois_minus1[ i ]; j++ ) {|||
|<br> <br> <br> <br> <br>**first_mb_in_roi**[ i ][ j ]|5|ue(v)|
|<br> <br> <br> <br> <br>**roi_width_in_mbs_minus1**[ i ][ j ]|5|ue(v)|
|<br> <br> <br> <br> <br>**roi_height_in_mbs_minus1**[ i ][ j ]|5|ue(v)|
|<br> <br> <br> <br>}|||
|<br> <br> <br>}|||
|<br> <br>}|||
|<br> <br>if( layer_dependency_info_present_flag[ i ] ) {|||
|<br> <br> <br>**num_directly_dependent_layers**[ i ]|5|ue(v)|
|<br> <br> <br>for( j = 0; j < num_directly_dependent_layers[ i ]; j++ )|||
|<br> <br> <br> <br>**directly_dependent_layer_id_delta_minus1**[ i ][ j ]|5|ue(v)|
|<br> <br>} else|||
|<br> <br> <br>**layer_dependency_info_src_layer_id_delta**[ i ]|5|ue(v)|
|<br> <br>if( parameter_sets_info_present_flag[ i ] ) {|||
|<br> <br> <br>**num_seq_parameter_sets**[ i ]|5|ue(v)|
|<br> <br> <br>for( j = 0; j < num_seq_parameter_sets[ i ]; j++ )|||
|<br> <br> <br> <br>**seq_parameter_set_id_delta**[ i ][ j ]|5|ue(v)|
|<br> <br> <br>**num_subset_seq_parameter_sets**[ i ]|5|ue(v)|
|<br> <br> <br>for( j = 0; j < num_subset_seq_parameter_sets[ i ]; j++ )|||
|<br> <br> <br> <br>**subset_seq_parameter_set_id_delta**[ i ][ j ]|5|ue(v)|
|<br> <br> <br>**num_pic_parameter_sets_minus1**[ i ]|5|ue(v)|
|<br> <br> <br>for( j = 0; j <= num_pic_parameter_sets_minus1[ i ]; j++ )|||
|<br> <br> <br> <br>**pic_parameter_set_id_delta**[ i ][ j ]|5|ue(v)|
|<br> <br>} else|||
|<br> <br> <br>**parameter_sets_info_src_layer_id_delta**[ i ]|5|ue(v)|
|<br> <br>if( bitstream_restriction_info_present_flag[ i ] ) {|||
|<br> <br> <br>**motion_vectors_over_pic_boundaries_flag**[ i ]|5|u(1)|
|<br> <br> <br>**max_bytes_per_pic_denom**[ i ]|5|ue(v)|
|<br> <br> <br>**max_bits_per_mb_denom**[ i ]|5|ue(v)|
|<br> <br> <br>**log2_max_mv_length_horizontal**[ i ]|5|ue(v)|
|<br> <br> <br>**log2_max_mv_length_vertical**[ i ]|5|ue(v)|





|max_num_reorder_frames[ i ]|5|ue(v)|
|---|---|---|
|<br> <br> <br>**max_dec_frame_buffering**[ i ]|5|ue(v)|
|<br> <br>}|||
|<br> <br>if( layer_conversion_flag[ i ] ) {|||
|<br> <br> <br>**conversion_type_idc**[ i ]|5|ue(v)|
|<br> <br> <br>for( j=0; j < 2; j++ ) {|||
|<br> <br> <br> <br>**rewriting_info_flag**[ i ][ j ]|5|u(1)|
|<br> <br> <br> <br>if( rewriting_info_flag[ i ][ j ] ) {|||
|<br> <br> <br> <br> <br>**rewriting_profile_level_idc**[ i ][ j ]|5|u(24)|
|<br> <br> <br> <br> <br>**rewriting_avg_bitrate**[ i ][ j ]|5|u(16)|
|<br> <br> <br> <br> <br>**rewriting_max_bitrate**[ i ][ j ]|5|u(16)|
|<br> <br> <br> <br>}|||
|<br> <br> <br>}|||
|<br> <br>}|||
|<br>}|||
|<br>if( priority_layer_info_present_flag ) {|||
|<br> <br>**pr_num_dIds_minus1**|5|ue(v)|
|<br> <br>for( i = 0; i <= pr_num_dIds_minus1; i++ ) {|||
|<br> <br> <br>**pr_dependency_id**[ i ]|5|u(3)|
|<br> <br> <br>**pr_num_minus1**[ i ]|5|ue(v)|
|<br> <br> <br>for( j = 0; j <= pr_num_minus1[ i ]; j++ ) {|||
|<br> <br> <br> <br>**pr_id**[ i ][ j ]|5|ue(v)|
|<br> <br> <br> <br>**pr_profile_level_idc**[ i ][ j ]|5|u(24)|
|<br> <br> <br> <br>**pr_avg_bitrate**[ i ][ j ]|5|u(16)|
|<br> <br> <br> <br>**pr_max_bitrate**[ i ][ j ]|5|u(16)|
|<br> <br> <br>}|||
|<br> <br>}|||
|<br>}|||
|<br>if( priority_id_setting_flag ) {|||
|<br> <br>PriorityIdSettingUriIdx = 0|||
|<br> <br>do|||
|<br> <br> <br>**priority_id_setting_uri**[ PriorityIdSettingUriIdx ]|5|b(8)|
|<br> <br>while( priority_id_setting_uri[ PriorityIdSettingUriIdx++ ] != 0 )|||
|<br>}|||
|}|||


**G.13.1.2Sub-picture scalable layer SEI message syntax**

|sub_pic_scalable_layer( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**layer_id**|5|ue(v)|
|}|||






**G.13.1.3Non-required layer representation SEI message syntax**

|non_required_layer_rep( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**num_info_entries_minus1**|5|ue(v)|
|<br>for( i = 0; i <= num_info_entries_minus1; i++ ) {|||
|<br> <br>**entry_dependency_id[** i**] **|5|u(3)|
|<br> <br>**num_non_required_layer_reps_minus1[** i**] **|5|ue(v)|
|<br> <br>for( j = 0; j <= num_non_required_layer_reps_minus1**[** i**]**; j++ ) {|||
|<br> <br> <br>**non_required_layer_rep_dependency_id[** i**][** j**]**|5|u(3)|
|<br> <br> <br>**non_required_layer_rep_quality_id[** i**][** j**]**|5|u(4)|
|<br> <br>}|||
|<br>}|||
|}|||



**G.13.1.4Priority layer information SEI message syntax**

|priority_layer_info( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**pr_dependency_id**|5|u(3)|
|<br>**num_priority_ids**|5|u(4)|
|<br>for( i = 0; i < num_priority_ids; i++ ) {|||
|<br> <br>**alt_priority_id[** i**] **|5|u(6)|
|<br>}|||
|}|||



**G.13.1.5Layers not present SEI message syntax**

|layers_not_present( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**num_layers**|5|ue(v)|
|<br>for( i = 0; i < num_layers; i++ ) {|||
|<br> <br>**layer_id[** i**] **|5|ue(v)|
|<br>}|||
|}|||



**G.13.1.6Layer dependency change SEI message syntax**

|layer_dependency_change( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**num_layers_minus1**|5|ue(v)|
|<br>for( i = 0; i <= num_layers_minus1; i++ ) {|||
|<br> <br>**layer_id[** i**] **|5|ue(v)|
|<br> <br>**layer_dependency_info_present_flag[** i**]**|5|u(1)|
|<br> <br>if( layer_dependency_info_present_flag[ i ] ) {|||
|<br> <br> <br>**num_directly_dependent_layers[** i**] **|5|ue(v)|
|<br> <br> <br>for( j = 0; j < num_directly_dependent_layers[ i ]; j++ )|||
|<br> <br> <br> <br>**directly_dependent_layer_id_delta_minus1[** i**][** j**]**|5|ue(v)|
|<br> <br>} else {|||
|<br> <br> <br>**layer_dependency_info_src_layer_id_delta_minus1[** i**]**|5|ue(v)|
|<br> <br>}|||






|}|Col2|Col3|
|---|---|---|
|}|||


**G.13.1.7Scalable nesting SEI message syntax**

|scalable_nesting( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**all_layer_representations_in_au_flag**|5|u(1)|
|<br>if( all_layer_representations_in_au_flag = = 0) {|||
|<br> <br>**num_layer_representations_minus1**|5|ue(v)|
|<br> <br>for( i = 0; i <= num_layer_representations_minus1; i++ ) {|||
|<br> <br> <br>**sei_dependency_id[** i**] **|5|u(3)|
|<br> <br> <br>**sei_quality_id[** i**] **|5|u(4)|
|<br> <br>}|||
|<br> <br>**sei_temporal_id**|5|u(3)|
|<br>}|||
|<br>while( !byte_aligned( ) )|||
|<br> <br>**sei_nesting_zero_bit**/* equal to 0 */|5|f(1)|
|<br>do|||
|<br> <br>sei_message( )|5||
|<br>while( more_rbsp_data( ) )|||
|}|||



**G.13.1.8Base layer temporal HRD SEI message syntax**

|base_layer_temporal_hrd( payloadSize ) {|C|Descriptor|
|---|---|---|
|**num_of_temporal_layers_in_base_layer_minus1**|5|ue(v)|
|for( i = 0; i <= num_of_temporal_layers_in_base_layer_minus1; i++ ) {|||
|<br> <br>**sei_temporal_id[**i** ]**|5|u(3)|
|<br> <br>**sei_timing_info_present_flag[**i** ]**|5|u(1)|
|<br> <br>if( sei_timing_info_present_flag[ i ] ) {|||
|<br> <br> <br>**sei_num_units_in_tick[**i** ]**|5|u(32)|
|<br> <br> <br>**sei_time_scale[**i** ]**|5|u(32)|
|<br> <br> <br>**sei_fixed_frame_rate_flag[**i** ]**|5|u(1)|
|<br> <br>}|||
|<br> <br>**sei_nal_hrd_parameters_present_flag[**i** ]**|5|u(1)|
|<br> <br>if( sei_nal_hrd_parameters_present_flag[ i ] )|||
|<br> <br> <br>hrd_parameters( )|5||
|<br> <br>**sei_vcl_hrd_parameters_present_flag[**i** ]**|5|u(1)|
|<br> <br>if( sei_vcl_hrd_parameters_present_flag[ i ] )|||
|<br> <br> <br>hrd_parameters( )|5||
|<br> <br>if( sei_nal_hrd_parameters_present_flag[ i ] | |<br> <br> <br> <br>sei_vcl_hrd_parameters_present_flag[ i ] )|||
|<br> <br> <br>**sei_low_delay_hrd_flag[**i** ]**|5|u(1)|
|<br> <br>**sei_pic_struct_present_flag[**i** ]**|5|u(1)|
|<br>}|||
|}|||






**G.13.1.9Quality layer integrity check SEI message syntax**

|quality_layer_integrity_check( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**num_info_entries_minus1**|5|ue(v)|
|<br>for( i = 0; i <= num_info_entries_minus1; i++ ) {|||
|<br> <br>**entry_dependency_id[** i**] **|5|u(3)|
|<br> <br>**quality_layer_crc[** i**] **|5|u(16)|
|<br>}|||
|}|||



**G.13.1.10** **Redundant picture property SEI message syntax**

|redundant_pic_property( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**num_dIds_minus1**|5|ue(v)|
|<br>for( i = 0; i <= num_dIds_minus1; i++ ) {|||
|<br> <br>**dependency_id[** i**] **|5|u(3)|
|<br> <br>**num_qIds_minus1[** i**] **|5|ue(v)|
|<br> <br>for( j = 0; j <= num_qIds_minus1[ i ]; j++ ) {|||
|<br> <br> <br>**quality_id[** i**][** j**] **|5|u(4)|
|<br> <br> <br>**num_redundant_pics_minus1[** i**][** j**] **|5|ue(v)|
|<br> <br> <br>for( k = 0; k <= num_redundant_pics_minus1[ i ][ j ]; k++ ) {|||
|<br> <br> <br> <br>**redundant_pic_cnt_minus1[** i**][** j**][** k**] **|5|ue(v)|
|<br> <br> <br> <br>**pic_match_flag[** i**][** j**][** k**] **|5|u(1)|
|<br> <br> <br> <br>if( !pic_match_flag[ i ][ j ][ k ]) {|||
|<br> <br> <br> <br> <br>**mb_type_match_flag[** i**][** j**][** k**]**|5|u(1)|
|<br> <br> <br> <br> <br>**motion_match_flag[** i**][** j**][** k**]**|5|u(1)|
|<br> <br> <br> <br> <br>**residual_match_flag[** i**][** j**][** k**]**|5|u(1)|
|<br> <br> <br> <br> <br>**intra_samples_match_flag[** i**][** j**][** k**]**|5|u(1)|
|<br> <br> <br> <br>}|||
|<br> <br> <br>}|||
|<br> <br>}|||
|<br>}|||
|}|||



**G.13.1.11** **Temporal level zero dependency representation index SEI message syntax**

|tl0_dep_rep_index( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**tl0_dep_rep_idx**|5|u(8)|
|<br>**effective_idr_pic_id**|5|u(16)|
|}|||






**G.13.1.12** **Temporal level switching point SEI message syntax**

|tl_switching_point( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**delta_frame_num**|5|se(v)|
|}|||



**G.13.2** **SEI payload semantics**


The semantics of the SEI messages with payloadType in the range of 0 to 23, inclusive, or equal to 45, 47, 137, 142, 144,
147, 148, 149, 150, 151, 154, 155, 156, 200, 201, 202, or 205, which are specified in clause D.2, are extended as follows:


- If payloadType is equal to 3, 8, 19, 20, or 22, the following applies:


   - If the SEI message is not included in a scalable nesting SEI message, it applies to the layer representations of the
current access unit that have dependency_id equal to 0 and quality_id equal to 0.


The semantics as specified in clause D.2 apply to the bitstream that would be obtained by invoking the bitstream
extraction process as specified in clause G.8.8.1 with dIdTarget equal to 0 and qIdTarget equal to 0. All syntax
elements and derived variables that are referred to in the semantics in clause D.2 are syntax elements and
variables for layer representations with dependency_id equal to 0 and quality_id equal to 0. All SEI messages
that are referred to in clause D.2 are SEI messages that apply to layer representations with dependency_id equal
to 0 and quality_id equal to 0.


   - Otherwise (the SEI message is included in a scalable nesting SEI message), the SEI message applies to all layer
representations of the current access unit for which DQId is equal to any value of
( ( sei_dependency_id[ i ] << 4 ) + sei_quality_id[ i ] ) with i in the range of 0 to
num_layer_representations_minus1, inclusive.


For each value of i in the range of 0 to num_layer_representations_minus1, inclusive, the semantics as specified
in clause D.2 apply to the bitstream that would be obtained by invoking the bitstream extraction process as
specified in clause G.8.8.1 with dIdTarget equal to sei_dependency_id[ i ] and qIdTarget equal to
sei_quality_id[ i ]. All syntax elements and derived variables that are referred to in the semantics in clause D.2
are syntax elements and variables for layer representations with dependency_id equal to sei_dependency_id[ i ]
and quality_id equal to sei_quality_id[ i ]. All SEI messages that are referred to in clause D.2 are SEI messages
that apply to layer representations with dependency_id equal to sei_dependency_id[ i ] and quality_id equal to
sei_quality_id[ i ].


- Otherwise, if payloadType is equal to 2, 6, 7, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 21, 23, 45, 47, 137, 142, 144, 147,
148, 149, 150, 151, 154, 155, 156, 200, 201, 202, or 205, the following applies:


   - If the SEI message is not included in a scalable nesting SEI message, it applies to the dependency representations
of the current access unit that have dependency_id equal to 0.


The semantics as specified in clause D.2 apply to the bitstream that would be obtained by invoking the bitstream
extraction process as specified in clause G.8.8.1 with dIdTarget equal to 0. All syntax elements and derived
variables that are referred to in the semantics in clause D.2 are syntax elements and variables for dependency
representations with dependency_id equal to 0. All SEI messages that are referred to in clause D.2 are SEI
messages that apply to dependency representations with dependency_id equal to 0.


   - Otherwise (the SEI message is included in a scalable nesting SEI message), the scalable nesting SEI message
containing the SEI message shall have all_layer_representations_in_au_flag equal to 1 or, when
all_layer_representations_in_au_flag is equal to 0, all values of sei_quality_id[ i ] present in the scalable nesting
SEI message shall be equal to 0. The SEI message that is included in the scalable nesting SEI message applies
to all dependency representations of the current access unit for which dependency_id is equal to any value of
sei_dependency_id[ i ] with i in the range of 0 to num_layer_representations_minus1, inclusive.


For each value of i in the range of 0 to num_layer_representations_minus1, inclusive, the semantics as specified
in clause D.2 apply to the bitstream that would be obtained by invoking the bitstream extraction process as
specified in clause G.8.8.1 with dIdTarget equal to sei_dependency_id[ i ]. All syntax elements and derived
variables that are referred to in the semantics in clause D.2 are syntax elements and variables for dependency
representations with dependency_id equal to sei_dependency_id[ i ]. All SEI messages that are referred to in
clause D.2 are SEI messages that apply to dependency representations with dependency_id equal to
sei_dependency_id[ i ].





When payloadType is equal to 10 for the SEI message that is included in a scalable nesting SEI message, the
semantics for sub_seq_layer_num of the sub-sequence information SEI message is modified as follows:


**sub_seq_layer_num** specifies the sub-sequence layer number of the current picture. When the current
picture resides in a sub-sequence for which the first picture in decoding order is an IDR picture, the value
of sub_seq_layer_num shall be equal to 0. For a non-paired reference field, the value of sub_seq_layer_num
shall be equal to 0. sub_seq_layer_num shall be in the range of 0 to 255, inclusive.


- Otherwise, if payloadType is equal to 0 or 1, the following applies:


   - If the SEI message is not included in a scalable nesting SEI message, the following applies. When the SEI
message and all other SEI messages with payloadType equal to 0 or 1 not included in a scalable nesting SEI
message are used as the buffering period and picture timing SEI messages for checking the bitstream
conformance according to Annex C and the decoding process specified in clauses 2 to 9 is used, the bitstream
shall be conforming to this Recommendation | International Standard.


The value of seq_parameter_set_id in a buffering period SEI message not included in a scalable nesting SEI
message shall be equal to the value of seq_parameter_set_id in the picture parameter set that is referenced by the
layer representation with DQId equal to 0 of the primary coded picture in the same access unit.


   - Otherwise (the SEI message is included in a scalable nesting SEI message), the following applies. When the SEI
message and all other SEI messages with payloadType equal to 0 or 1 included in a scalable nesting SEI message
with identical values of sei_temporal_id, sei_dependency_id[ i ], and sei_quality_id[ i ] are used as the buffering
period and picture timing SEI messages for checking the bitstream conformance according to Annex C, the
bitstream that would be obtained by invoking the bitstream extraction process as specified in clause G.8.8.1 with
tIdTarget equal to sei_temporal_id, dIdTarget equal to sei_dependency_id[ i ], and qIdTarget equal to
sei_quality_id[ i ] shall be conforming to this Recommendation | International Standard.


In the semantics of clauses D.2.1 and D.2.3, the syntax elements num_units_in_tick, time_scale,
fixed_frame_rate_flag, nal_hrd_parameters_present_flag, vcl_hrd_parameters_present_flag,
low_delay_hrd_flag, and pic_struct_present_flag and the derived variables NalHrdBpPresentFlag,
VclHrdBpPresentFlag, and CpbDpbDelaysPresentFlag are substituted with the syntax elements
vui_ext_num_units_in_tick[ i ], vui_ext_time_scale[ i ], vui_ext_fixed_frame_rate_flag[ i ],
vui_ext_nal_hrd_parameters_present_flag[ i ], vui_ext_vcl_hrd_parameters_present_flag[ i ],
vui_ext_low_delay_hrd_flag[ i ], and vui_ext_pic_struct_present_flag[ i ] and the derived variables
VuiExtNalHrdBpPresentFlag[ i ], VuiExtVclHrdBpPresentFlag[ i ], and VuiExtCpbDpbDelaysPresentFlag[ i ].


The value of seq_parameter_set_id in a buffering period SEI message included in a scalable nesting SEI message
with the values of sei_dependency_id[ i ] and sei_quality_id[ i ] shall be equal to the value of
seq_parameter_set_id in the picture parameter set that is referenced by the layer representation with DQId equal
to (( sei_dependency_id[ i ] << 4 ) + sei_quality_id[ i ]) of the primary coded picture in the same access unit.


- Otherwise (payloadType is equal to 4 or 5), the corresponding SEI message semantics are not extended.


When an SEI message having a particular value of payloadType equal to 137 or 144, contained in a scalable nesting SEI
message, and applying to a particular combination of dependency_id, quality_id, and temporal_id is present in an access
unit, the SEI message with the particular value of payloadType applying to the particular combination of dependency_id,
quality_id, and temporal_id shall be present a scalable nesting SEI message in the IDR access unit that is the first access
unit of the coded video sequence.


All SEI messages having a particular value of payloadType equal to 137 or 144, contained in scalable nesting SEI
messages, and applying to a particular combination of dependency_id, quality_id, and temporal_id present in a coded video
sequence shall have the same content.


For the semantics of SEI messages with payloadType in the range of 0 to 23, inclusive, or equal to 45, 47, 137, 142, 144,
147, 148, 149, 150, 151, 154, 155, 156, 200, 201, 202, or 205, which are specified in clause D.2, SVC sequence parameter
set is substituted for sequence parameter set; the parameters of the picture parameter set RBSP and SVC sequence
parameter set RBSP that are in effect are specified in clause G.7.4.1.2.1.


Coded video sequences conforming to one or more of the profiles specified in Annex G shall not include SEI NAL units
that contain SEI messages with payloadType in the range of 36 to 44, inclusive, or equal to 46, which are specified in
clause H.13, or with payloadType in the range of 48 to 53, inclusive, which are specified in clause I.13.


When an SEI NAL unit contains an SEI message with payloadType in the range of 24 to 35, inclusive, which are specified
in clause G.13, it shall not contain any SEI message that has payloadType less than 24 or equal to 45, 47, 137, 142, 144,
147, 148, 149, 150, 151, 154, 155, 156, 200, 201, 202, or 205 that is not included in a scalable nesting SEI message, and
the first SEI message in the SEI NAL unit shall have payloadType in the range of 24 to 35, inclusive.





When an SEI NAL unit contains an SEI message with payloadType equal to 24, 28, or 29, it shall not contain any SEI
message with payloadType not equal to 24, 28, or 29.


When a scalable nesting SEI message (payloadType is equal to 30) is present in an SEI NAL unit, it shall be the only SEI
message in the SEI NAL unit.


The semantics for SEI messages with payloadType in the range of 24 to 35, inclusive, are specified in the following.


**G.13.2.1Scalability information SEI message semantics**


The scalability information SEI message provides scalability information for subsets of the bitstream.


In the following specification of this clause, a VCL NAL unit of a primary coded picture is also referred to as primary
coded VCL NAL unit and a VCL NAL unit of a redundant coded picture is also referred to as redundant coded VCL NAL
unit.


A scalability information SEI message shall not be included in a scalable nesting SEI message.


A scalability information SEI message shall not be present in access units that contain primary coded VCL NAL units with
IdrPicFlag equal to 0. The set of access units consisting of the access unit associated with the scalability information SEI
message and all succeeding access units in decoding order until, but excluding, the next access unit that does not contain
any primary coded VCL NAL unit with IdrPicFlag equal to 0 (if present) or the end of the bitstream (otherwise) is referred
to as the target access unit set. The scalability information SEI message applies to the target access unit set.


The scalability information SEI message provides information for subsets of the target access unit set. These subsets are
referred to as scalable layers. A scalable layer represents a set of NAL units, inside the target access unit set, that consists
of VCL NAL units with the same values of dependency_id, quality_id, and temporal_id, as specified later in this clause,
and associated non-VCL NAL units. When present in the target access unit set, the following NAL units are associated
non-VCL NAL units for a scalable layer:


- sequence parameter set, subset sequence parameter set, and picture parameter set NAL units that are referenced in the
VCL NAL units of the scalable layer (via the syntax element pic_parameter_set_id),


- sequence parameter set extension NAL units that are associated with a sequence parameter set NAL unit referenced
in the VCL NAL units of the scalable layer,


- filler data NAL units that are associated with the same values of dependency_id, quality_id, and temporal_id as the
VCL NAL units of the scalable layer,


- SEI NAL units containing SEI messages, with payloadType not equal to 24, 28, or 29, that apply to subsets of the
bitstream that contain one or more VCL NAL units of the scalable layer,


- access unit delimiter, end of sequence, and end of stream NAL units that are present in access units that contain VCL
NAL units of the scalable layer,


- when dependency_id and quality_id are both equal to 0 in the VCL NAL units of a scalable layer, coded slice of an
auxiliary coded picture without partitioning NAL units that are present in access units that contain VCL NAL units
of the scalable layer.


A scalable layer A is directly dependent on a scalable layer B when any primary coded VCL NAL unit of the scalable layer
A references data of any VCL NAL unit of the scalable layer B through inter prediction or inter-layer prediction as specified
in the decoding process in clause G.8, with the following exception: A scalable layer A (identified by layer_id[ a ]) is not
directly dependent on a scalable layer B (identified by layer_id[ b ]) when dependency_id[ a ] is equal to
dependency_id[ b ], sub_pic_layer_flag[ a ] is equal to 1, and one of the following conditions is true:


- sub_pic_layer_flag[ b ] is equal to 0,


- sub_pic_layer_flag[ b ] is equal to 1 and (horizontal_offset[ a ] is not equal to horizontal_offset[ b ],
vertical_offset[ a ] is not equal to vertical_offset[ b ], region_width[ a ] is not equal to region_width[ b ], or
region_height[ a ] is not equal to region_height[ b ]).
NOTE 1 – Sub-picture scalable layers with a particular value of dependency_id and a particular sub-picture area are only considered
to depend on scalable layers with the same value of dependency_id when these scalable layers are associated with the same
sub-picture area.


A scalable layer A is indirectly dependent on a scalable layer B when the scalable layer A is not directly dependent on the
scalable layer B but there exists a set of n (with n being greater than 0) scalable layers {C0, .., Cn−1} with the following
properties: The scalable layer A is directly dependent on the scalable layer C0, each scalable layer Ci with i in the range of
0 to n − 2, inclusive, is directly dependent on the scalable layer Ci+1, and the scalable layer Cn−1 is directly dependent on
the scalable layer B.





The representation of a particular scalable layer is the set of NAL units that represents the set union of the particular
scalable layer and all scalable layers on which the particular scalable layer directly or indirectly depends. The representation
of a scalable layer is also referred to as scalable layer representation. In the following specification of this clause, the terms
representation of a scalable layer and scalable layer representation are also used for referring to the access unit set that can
be constructed from the NAL units of the scalable layer representation. A scalable layer representation can be decoded
independently of all NAL units that do not belong to the scalable layer representation. The decoding result of a scalable
layer representation is the set of decoded pictures that are obtained by decoding the access unit set of the scalable layer
representation.

NOTE 2 – The set of access units that is formed by the representation of a scalable layer with sub_pic_layer_flag[ i ] equal to 1 does
not conform to this Recommendation | International Standard, since the primary coded VCL NAL units with quality_id equal to 0
that belong to such a scalable layer representation do not cover all macroblocks of the layer pictures with dependency_id equal to
dependency_id[ i ] and quality_id equal to 0. For the following specification in this clause, the decoding result for the representation
of a scalable layer with sub_pic_layer_flag[ i ] equal to 1 is the decoding result that would be obtained for the sub-picture area (as
specified later in this clause) by following the decoding process in clause G.8 but ignoring the constraint that the layer representations
with quality_id equal to 0 of primary coded pictures must cover all macroblocks of the corresponding layer pictures.


Each scalable layer is associated with a unique layer identifier as specified later in this clause. The representation of a
particular scalable layer with a particular layer identifier layerId does not include any scalable layer with a layer identifier
greater than layerId, but it may include scalable layers with layer identifiers less than layerId. The scalable layers on which
a particular scalable layer depends may be indicated in the scalability information SEI message as specified later in this
clause.

NOTE 3 – When all scalable layers for which scalability information is provided in the scalability information SEI message have
sub_pic_layer_flag[ i ] equal to 0, the unique layer identifier values may be set equal to ( 128 * dependency_id + 8 * quality_id +
temporal_id ), with dependency_id, quality_id, and temporal_id being the corresponding syntax elements that are associated with
the VCL NAL units of a scalable layer.


**temporal_id_nesting_flag** indicates whether inter prediction is additionally restricted for the target access unit set.
Depending on the value of temporal_id_nesting_flag, the following applies:


- If temporal_id_nesting_flag is equal to 1, the scalability information SEI message indicates that the following
constraint is obeyed for all access units sets that can be derived from the target access unit set by invoking the
sub-bitstream extraction process as specified in clause G.8.8.1 with tIdTarget equal to any value in the range of 0 to
7, inclusive, dIdTarget equal to any value in the range of 0 to 7, inclusive, and qIdTarget equal to any value in the
range of 0 to 15, inclusive, as the inputs: The values of the samples in the decoded pictures for each access unit auA
with temporal_id equal to tIdA and all following access units in decoding order are independent of an access unit auB
with temporal_id equal to tIdB and tIdB less than or equal to tIdA, when there exists an access unit auC with
temporal_id equal to tIdC and tIdC less than tIdB, that follows the access unit auB and precedes the access unit auA
in decoding order.


- Otherwise (temporal_id_nesting_flag is equal to 0), the scalability information SEI message indicates that the
constraint specified for temporal_id_nesting_flag equal to 1 may or may not be obeyed.
NOTE 4 – The syntax element temporal_id_nesting_flag is used to indicate that temporal up-switching, i.e., switching from decoding
of up to a particular temporal_id value tIdN to decoding of up to a temporal_id value tIdM greater than tIdN, is always possible
inside the target access unit set.


**priority_layer_info_present_flag** equal to 1 specifies that characteristic information for priority layers, as specified later
in this clause, is present in the scalability information SEI message and that priority layer information SEI messages
associating an alternative value for priority_id with each layer representation of the primary coded pictures in the target
access unit set are present. priority_layer_info_present_flag equal to 0 specifies that characteristic information for priority
layers is not present in the scalability information SEI message.


**priority_id_setting_flag** equal to 1 specifies that syntax elements priority_id_setting_uri[ i ] are present in the scalability
information SEI message and that the description of the method used to calculate the priority_id values is provided by the
specified universal resource identifier (URI). priority_id_setting_flag equal to 0 specifies that syntax elements
priority_id_setting_uri[ i ] are not present in the scalability information SEI message.


**num_layers_minus1** plus 1 specifies the number of scalable layers for which information is provided in the scalability
information SEI message. The value of num_layers_minus1 shall be in the range of 0 to 2047, inclusive.


**layer_id[** i **]** specifies the layer identifier of the i-th scalable layer specified in the scalability information SEI message.
layer_id[ i ] shall be in the range of 0 to 2047, inclusive.


For the following specification inside this clause, the scalable layer with layer identifier equal to the current value of
layer_id[ i ] is referred to as the current scalable layer, and the representation of the current scalable layer is referred to as
the current scalable layer representation.





**priority_id** [ i ] indicates an upper bound for the priority_id values of the current scalable layer representation. All primary
coded VCL NAL units of the current scalable layer representation shall have a value of priority_id that is less than or equal
to priority_id[ i ].


**discardable_flag** [ i ] equal to 1 indicates that all primary coded VCL NAL units of the current scalable layer have
discardable_flag equal to 1. discardable_flag[ i ] equal to 0 indicates that the current scalable layer may contain one or
more primary coded VCL NAL units with discardable_flag equal to 0.


**dependency_id[** i **]**, **quality_id[** i **]**, and **temporal_id[** i **]** are equal to the values of dependency_id, quality_id, and
temporal_id, respectively, of the VCL NAL units of the current scalable layer. All VCL NAL units of a scalable layer have
the same values of dependency_id, quality_id, and temporal_id.


When the target access unit set does not contain any primary coded VCL NAL unit with particular values of dependency_id,
quality_id, and temporal_id, the scalability information SEI message shall not contain information for a scalable layer with
dependency_id[ i ], quality_id[ i ], and temporal_id[ i ] equal to the particular values of dependency_id, quality_id, and
temporal_id, respectively.

NOTE 5 – When an application removes NAL units from a scalable bitstream, e.g. in order to adapt the bitstream to a transmission
channel or the capabilities of a receiving device, and keeps the present scalability information SEI messages, it might need to modify
the content of the scalability information SEI messages in order to obtain a bitstream conforming to this Recommendation |
International Standard.


**sub_pic_layer_flag[** i **]** specifies whether the current scalable layer represents a sub-picture scalable layer as specified
subsequently. Depending on sub_pic_layer_flag[ i ], the following applies:


- If sub_pic_layer_flag[ i ] is equal to 0, the current scalable layer does not represent a sub-picture scalable layer. The
VCL NAL units of the current scalable layer are all VCL NAL units of the target access unit set that have
dependency_id, quality_id, and temporal_id equal to dependency_id[ i ], quality_id[ i ], and temporal_id[ i ],
respectively.


- Otherwise (sub_pic_layer_flag[ i ] is equal to 1), the current scalable layer represents a sub-picture scalable layer and
is associated with a sub-picture area as specified in the following:


(a) The sub-picture area is a rectangular area of slice group map units inside the layer frames with dependency_id

equal to dependency_id[ i ] and represents a proper subset of the area of the layer frames with dependency_id
equal to dependency_id[ i ]. The sub-picture area associated with a sub-picture scalable layer does not change
inside the target access unit set. The sub-picture area is specified by the syntax elements horizontal_offset[ i ],
vertical_offset[ i ], region_width[ i ], and region_height[ i ] as specified later in this clause.

NOTE 6 – The sub-picture area for a sub-picture scalable layer may additionally be indicated by the presence of subpicture scalable layer SEI messages with layer_id equal to value of layer_id[ i ] for the current scalable layer.


(b) When a VCL NAL unit of the target access unit set has dependency_id equal to dependency_id[ i ] and contains

any macroblock that resides inside the sub-picture area, it shall not contain any macroblock that resides outside
of the sub-picture area.


(c) The VCL NAL units of the current scalable layer are the coded slice NAL units of the target access unit set that

have dependency_id, quality_id, and temporal_id equal to dependency_id[ i ], quality_id[ i ], and
temporal_id[ i ], respectively, and for which the macroblock specified by first_mb_in_slice resides inside the
specified sub-picture area and the associated prefix NAL units (when present).


(d) For all access units sets that can be derived from the target access unit set by invoking the sub-bitstream extraction

process as specified in clause G.8.8.1 with dIdTarget equal to dependency_id[ i ] and qIdTarget equal to any value
in the range of 0 to 15, inclusive, as the inputs, the following constraint shall be obeyed: No sample value outside
the sub-picture area and no sample value at a fractional sample position that is derived using one or more sample
values outside the sub-picture area is used, in the decoding process as specified in clause G.8, for inter prediction
of any sample within the sub-picture area.


When the target access unit set contains any primary coded VCL NAL unit with particular values of dependency_id,
quality_id, and temporal_id, the scalability information SEI message shall contain information for a exactly one scalable
layer with dependency_id[ i ], quality_id[ i ], and temporal_id[ i ] equal to the particular values of dependency_id,
quality_id, and temporal_id, respectively, and sub_pic_layer_flag[ i ] equal to 0.

NOTE 7 – The scalability information SEI message may additionally contain information for one or more scalable layers with
dependency_id[ i ], quality_id[ i ], and temporal_id[ i ] equal to the particular values of dependency_id, quality_id, and temporal_id,
respectively, and sub_pic_layer_flag[ i ] equal to 1.


When sub_pic_layer_flag[ i ] is equal to 1 for the current scalable layer and the target access unit set contains any primary
coded VCL NAL unit that has dependency_id equal to dependency_id[ i ], resides inside the sub-picture area, and has
particular values of quality_id and temporal_id, with either quality_id not equal to quality_id[ i ] or temporal_id not equal
to temporal_id[ i ], the scalability information SEI message shall also contain information for a scalable layer j with
dependency_id[ j ] equal to dependency_id[ i ], quality_id[ j ] and temporal_id[ j ] equal to the particular values of





quality_id and temporal_id, respectively, sub_pic_layer_flag[ j ] equal to 1, and horizontal_offset[ j ], vertical_offset[ j ],
region_width[ j ], and region_height[ j ] equal to horizontal_offset[ i ], vertical_offset[ i ], region_width[ i ], and
region_height[ i ], respectively.


The scalability information SEI message shall not contain information for two or more scalable layers with
sub_pic_layer_flag[ i ] equal to 1 and the same values of dependency_id[ i ], quality_id[ i ], temporal_id[ i ],
sub_pic_layer_flag[ i ], horizontal_offset[ i ], vertical_offset[ i ], region_width[ i ], and region_height[ i ].


When the scalability information SEI message contains information for two scalable layers A and B (identified by
layer_id[ a ] and layer_id[ b ], respectively) with dependency_id[ a ] equal to dependency_id[ b ], quality_id[ a ] equal to
quality_id[ b ], temporal_id[ a ] equal to temporal_id[ b ], sub_pic_layer_flag[ a ] equal to 1, and sub_pic_layer_flag[ b ]
equal to 1, and the sub-picture areas associated with the scalable layers A and B overlap, the scalability information SEI
message shall also contain information for a scalable layer C (identified by layer_id[ c ]) with dependency_id[ c ] equal to
dependency_id[ b ], quality_id[ c ] equal to quality_id[ b ], temporal_id[ c ] equal to temporal_id[ b ], and
sub_pic_layer_flag[ c ] is equal to 1, and with the scalable layer C being associated with a sub-picture area that represents
the intersection of the sub-picture areas associated with the scalable layers A and B.


**sub_region_layer_flag[** i **]** equal to 1 specifies that the syntax elements base_region_layer_id[ i ] and
dynamic_rect_flag[ i ] for the current scalable layer are present in the scalability information SEI message.
sub_region_layer_flag[ i ] equal to 0 specifies that the syntax elements base_region_layer_id[ i ] and
dynamic_rect_flag[ i ] for the current scalable layer are not present in the scalability information SEI message.


When sub_pic_layer_flag[ i ] is equal to 1, sub_region_layer_flag[ i ] shall be equal to 1.


**iroi_division_info_present_flag[** i **]** equal to 1 specifies that the layer pictures with dependency_id equal to
dependency_id[ i ] are divided along slice group map unit boundaries into multiple rectangular regions of interest, referred
to as interactive regions of interest (IROIs), and that the IROI division information is explicitly signalled in the scalability
information SEI message as specified later in this clause, and that the syntax elements frame_width_in_mbs_minus1[ i ]
and frame_height_in_mbs_minus1[ i ] for the current scalable layer are present in the scalability information SEI message.
iroi_division_info_present_flag[ i ] equal to 0 specifies that the IROI division information for the current scalable layer is
not present in the scalability information SEI message.


When sub_pic_layer_flag[ i ] is equal to 1, iroi_division_info_present_flag[ i ] shall be equal to 0.


When iroi_division_info_present_flag[ i ] is equal to 1, the following is specified:


(a) When a primary coded VCL NAL unit of the target access unit set has dependency_id equal to dependency_id[ i ]

and contains any macroblock that resides inside a particular IROI, it shall not contain any macroblock that resides
outside of the particular IROI.


(b) For all access units sets that can be derived from the target access unit set by invoking the sub-bitstream extraction

process as specified in clause G.8.8.1 with dIdTarget equal to dependency_id[ i ] and qIdTarget equal to any value
in the range of 0 to 15, inclusive, as the inputs, the following constraint shall be obeyed: No sample value outside
a particular IROI and no sample value at a fractional sample position that is derived using one or more sample
values outside the particular IROI is used, in the decoding process as specified in clause G.8, for inter prediction
of any sample within the particular IROI.


All scalable layers with the same value of dependency_id[ i ] for which scalability information is present in the scalability
information SEI message shall have the same value of iroi_division_info_present_flag[ i ].


**profile_level_info_present_flag[** i **]** equal to 1 specifies that profile_idc, constraint_set0_flag, constraint_set1_flag,
constraint_set2_flag, constraint_set3_flag, constraint_set4_flag, constraint_set5_flag, reserved_zero_2bits, and level_idc
applicable for the current scalable layer representation are indicated by the value of layer_profile_level_idc[ i ] as specified
later in this clause.


When profile_level_info_present_flag[ i ] is equal to 0, profile_idc, constraint_set0_flag, constraint_set1_flag,
constraint_set2_flag, constraint_set3_flag, constraint_set4_flag, constraint_set5_flag, and level_idc applicable for the
current scalable layer representation are not indicated in the scalability information SEI message.


**bitrate_info_present_flag[** i **]** equal to 1 specifies that the bit rate information for the current scalable layer representation
is present in the scalability information SEI message. bitrate_info_present_flag[ i ] equal to 0 specifies that the bit rate
information for the current scalable layer representation is not present in the scalability information SEI message.


**frm_rate_info_present_flag[** i **]** equal to 1 specifies that the frame rate information for the current scalable layer
representation is present in the scalability information SEI message. frm_rate_info_present_flag[ i ] equal to 0 specifies
that the frame rate information for the current scalable layer representation is not present in the scalability information SEI
message.





**frm_size_info_present_flag[** i **]** equal to 1 specifies that the frame size information for the current scalable layer
representation is present in the scalability information SEI message. frm_size_info_present_flag[ i ] equal to 0 specifies
that the presence of the frame size information for the current scalable layer representation in the scalability information
SEI message is specified by iroi_division_info_present_flag[ i ].


**layer_dependency_info_present_flag[** i **]** equal to 1 specifies that one or more syntax elements
dependent_layer_id_delta_minus1[ i ][ j ] indicating the layer dependency information for the current scalable layer are
present in the scalability information SEI message. layer_dependency_info_present_flag **[** i **]** equal to 0 specifies that, for
the current scalable layer, the syntax element layer_dependency_info_src_layer_id_delta[ i ] is present in the scalability
information SEI message.


**parameter_sets_info_present_flag[** i **]** equal to 1 specifies that the values of seq_parameter_set_id of the sequence
parameter sets and subset sequence parameter sets and the values of pic_parameter_set_id of the picture parameter sets
that are referred to in the primary coded VCL NAL units of the current scalable layer representation are present in the
scalability information SEI message. parameter_sets_info_present_flag[ i ] equal to 0 specifies that, for the current scalable
layer, the syntax element parameter_sets_info_src_layer_id_delta[ i ] is present in the scalability information SEI message.


**bitstream_restriction_info_present_flag[** i **]** equal to 1 specifies that the bitstream restriction information for the current
scalable layer representation is present in the scalability information SEI message.
bitstream_restriction_info_present_flag[ i ] equal to 0 specifies that the bitstream restriction information for the current
scalable layer representation is not present in the scalability information SEI message.


**exact_inter_layer_pred_flag[** i **]** equal to 1 indicates that, for all primary coded VCL NAL units with
no_inter_layer_pred_flag equal to 0 of the current scalable layer representation, the reference layer representation
(specified by the syntax elements ref_layer_dq_id) that is used for inter-layer prediction in the decoding process, as
specified in clause G.8, is the same as the reference layer representation that was used during encoding.
exact_inter_layer_pred_flag[ i ] equal to 0 indicates that, for the primary coded VCL NAL units with
no_inter_layer_pred_flag equal to 0 of the current scalable layer representation, the reference layer representations that are
used for inter-layer prediction in the decoding process may or may not be the same as the reference layer representations
that were used during encoding.


NOTE 8 – A mismatch between the reference layer representation that is used for inter-layer prediction in the decoding process and
the reference layer representation that was used during encoding may be a result of a bitstream adaption, in which one or more layer
representations that are referred to in inter-layer prediction are removed from the bitstream, any of the primary coded VCL NAL
units that refer to any of the removed layer representations by inter-layer prediction is not removed from the bitstream, and the value
of the syntax elements ref_layer_dq_id in the primary coded VCL NAL units that refer to any of the removed layer representations
is modified in order to obtain a bitstream conforming to this Recommendation | International Standard.


exact_inter_layer_pred_flag[ i ] should be equal to 1. When the current scalable layer representation does not contain any
primary coded VCL NAL unit with no_inter_layer_pred_flag equal to 0, exact_inter_layer_pred_flag[ i ] shall be equal
to 1.


**exact_sample_value_match_flag[** i **]** indicates whether the values of decoded samples for decoding the representation of
the current sub-picture scalable layer (when sub_pic_layer_flag[ i ] is equal to 1) or any particular IROI within the current
scalable layer representation (when iroi_division_info_present_flag[ i ] is equal to 1) are identical to the values of the same
decoded samples that would be obtained by decoding all layer representations, of the primary coded pictures inside the
target access unit set, that have DQId less than or equal to 16 * dependency_id[ i ] + quality_id[ i ] and temporal_id less
than or equal to temporal_id[ i ].


With picSubset being the set of the primary coded pictures of the current scalable layer representation that contain any
VCL NAL unit with dependency_id equal to dependency_id[ i ], the following applies:


- If sub_pic_layer_flag[ i ] is equal to 1 (iroi_division_info_present_flag[ i ] is equal to 0), the following is specified:


1. Let picLRepSubset be the set of primary coded pictures that is formed by all the layer representations, of the

target access unit set, that contain any primary coded VCL NAL unit present in the set of pictures picSubset.

NOTE 9 – picSubset is a proper subset of picLRepSubset. picSubset only contains the primary coded slices of the
current (sub-picture) scalable layer representation, picLRepSubset contains all primary coded slices of the
corresponding layer representations (i.e. the complete layer representations that contain any slice of picSubset).


2. exact_sample_value_match_flag[ i ] equal to 1 indicates that the value of each decoded sample inside the

sub-picture area for decoding the picture set picSubset is identical to the value of the same decoded sample that
would be obtained by decoding the picture set picLRepSubset.


3. exact_sample_value_match_flag[ i ] equal to 0 indicates that the value of any decoded sample inside the

sub-picture area for decoding the picture set picSubset may or may not be identical to the value of the same
decoded sample that would be obtained by decoding the picture set picLRepSubset.


- Otherwise (sub_pic_layer_flag[ i ] is equal to 0 and iroi_division_info_present_flag[ i ] is equal to 1), for each
particular IROI, the following is specified:





1. Let picIROISubset be the set of primary coded VCL NAL units that is obtained by removing all the VCL NAL

units from the set of pictures picSubset that do not cover any macroblock inside the IROI.


2. exact_sample_value_match_flag[ i ] equal to 1 indicates that the value of each decoded sample inside the IROI

for decoding the picture set picSubset is identical to the value of the same decoded sample that would be obtained
by decoding the picture set picIROISubset.


3. exact_sample_value_match_flag[ i ] equal to 0 indicates that the value of any decoded sample inside the IROI

for decoding the picture set picSubset may or may not be identical to the value of the same decoded sample that
would be obtained by decoding the picture set picIROISubset.
NOTE 10 – In the above specification, the decoding result for picIROISubset is the decoding result that would be obtained for the
IROI by following the decoding process in clause G.8 but ignoring the constraint that the layer representations with quality_id equal
to 0 of primary coded pictures must cover all macroblocks of the corresponding layer pictures.
NOTE 11 – When disable_deblocking_filter_idc is equal to 1, 2, or 5 in all primary coded slices of the current scalable layer
representation that have dependency_id equal to dependency_id[ i ], exact_sample_value_match_flag should be equal to 1.


**layer_conversion_flag[** i **]** equal to 1 indicates that the representation of the current scalable layer can be converted into
an alternative set of access units that conforms to one or more of the profiles specified in Annex A and gives exactly the
same decoding result as the current scalable layer representation and that this conversion can be done without full
reconstruction and re-encoding. layer_conversion_flag[ i ] equal to 0 indicates that such a conversion of the current
scalable layer representation may or may not be possible.


**layer_output_flag[** i **]** equal to 1 indicates that the decoding result for the current scalable layer representation is intended
for output. layer_output_flag[ i ] equal to 0 indicates that the decoding result for the current scalable layer representation
is not intended for output.


NOTE 12 – The decoding result for a scalable layer representation with layer_output_flag[ i ] equal to 0 may be inappropriate for
output due to its low visual quality.


**layer_profile_level_idc** [ i ] indicates the conformance point of the representation of the current scalable layer.
layer_profile_level_idc[ i ] is the exact copy of the three bytes comprised of profile_idc, constraint_set0_flag,
constraint_set1_flag, constraint_set2_flag, constraint_set3_flag, constraint_set4_flag, constraint_set5_flag,
reserved_zero_2bits and level_idc, as if these syntax elements were used to specify the profile and level conformance of
the representation of the current scalable layer.

NOTE 13 – The representation of a sub-picture scalable layer (sub_pic_layer_flag[ i ] is equal to 1) does not conform to this
Recommendation | International Standard, since the primary coded VCL NAL units with quality_id equal to 0 that belong to a subpicture scalable layer representation do not cover all macroblocks of the layer pictures with dependency_id equal to
dependency_id[ i ] and quality_id equal to 0. For sub-picture scalable layers, the violation of the constraint that the layer
representations with quality_id equal to 0 of primary coded pictures must cover all macroblocks of the corresponding layer pictures
is ignored in the conformance point indication by layer_profile_level_idc[ i ].


**avg_bitrate[** i **]** indicates the average bit rate of the representation of the current scalable layer. The average bit rate for
the representation of the current scalable layer in bits per second is given by BitRateBPS( avg_bitrate[ i ] ) with the function
BitRateBPS( ) being specified by:


BitRateBPS( x ) = ( x & ( 2 [14]        - 1 ) ) * 10 [( 2 + ( x >> 14 ) )] (G-370)


The average bit rate is derived according to the access unit removal time specified in Annex C of this Recommendation |
International Standard. In the following, bTotal is the number of bits in all NAL units of the current scalable layer
representation, t1 is the removal time (in seconds) of the access unit associated with the scalability information SEI
message, and t2 is the removal time (in seconds) of the last access unit (in decoding order) of the target access unit set.


With x specifying the value of avg_bitrate[ i ], the following applies:


- If t1 is not equal to t2, the following condition shall be true:


( x & ( 2 [14]         - 1 ) ) = = Round( bTotal ÷ ( ( t2 − t1 ) * 10 [( 2 + ( x >> 14 ) )] ) ) (G-371)


- Otherwise (t1 is equal to t2), the following condition shall be true:


( x & ( 2 [14]         - 1 ) ) = = 0 (G-372)


**max_bitrate_layer[** i **]** indicates an upper bound for the bit rate of the current scalable layer in any fixed-size time window,
specified by max_bitrate_calc_window[ i ], of access unit removal time as specified in Annex C. The upper bound for the
bit rate of the current scalable layer in bits per second is given by BitRateBPS( max_bitrate_layer[ i ] ) with the function
BitRateBPS( ) being specified in Equation G-370. The bit rate values are derived according to the access unit removal time
specified in Annex C of this Recommendation | International Standard. In the following, t1 is any point in time (in seconds),
t2 is set equal to t1 + max_bitrate_calc_window[ i ] ÷ 100, and bTotal is the number of bits in all NAL units of the current
scalable layer that belong to access units with a removal time greater than or equal to t1 and less than t2. With x specifying
the value of max_bitrate_layer[ i ], the following condition shall be obeyed for all values of t1:





( x & ( 2 [14]         - 1 ) ) >= bTotal ÷ ( ( t2 − t1 ) * 10 [( 2 + ( x >> 14 ) )] ) (G-373)


**max_bitrate_layer_representation[** i **]** indicates an upper bound for the bit rate of the current scalable layer
representation in any fixed-size time window, specified by max_bitrate_calc_window[ i ], of access unit removal time as
specified in Annex C. The upper bound for the bit rate of the current scalable layer representation in bits per second is
given by BitRateBPS( max_bitrate_layer_representation[ i ] ) with the function BitRateBPS( ) being specified in
Equation G-370. The bit rate values are derived according to the access unit removal time specified in Annex C of this
Recommendation | International Standard. In the following, t1 is any point in time (in seconds), t2 is set equal to
t1 + max_bitrate_calc_window[ i ] ÷ 100, and bTotal is the number of bits in all NAL units of the current scalable layer
representation that belong to access units with a removal time greater than or equal to t1 and less than t2. With x specifying
the value of max_bitrate_layer_representation[ i ], the condition specified in Equation G-373 shall be obeyed.


**max_bitrate_calc_window[** i **]** specifies the size of the time window that is used for calculating the upper bounds for the
bit rate of the current scalable layer (indicated by max_bitrate_layer[ i ]) and the bit rate of the current scalable layer
representation (indicated by max_bitrate_layer_representation[ i ]) in units of 1/100 second.


**constant_frm_rate_idc[** i **]** indicates whether the frame rate of the current scalable layer representation is constant. In the
following, a temporal segment tSeg is any set of two or more consecutive access units, in decoding order, of the current
scalable layer representation, fTotal( tSeg ) is the number of frames, complementary field pairs, and non-paired fields in
the temporal segment tSeg, t1( tSeg ) is the removal time (in seconds) of the first access unit (in decoding order) of the
temporal segment tSeg, t2( tSeg ) is the removal time (in seconds) of the last access unit (in decoding order) of the temporal
segment tSeg, and avgFR( tSeg ) is the average frame rate in the temporal segment tSeg, which is given by:


avgFR( tSeg) = = Round( fTotal( tSeg ) * 256 ÷ ( t2( tSeg ) − t1( tSeg ) ) ) (G-374)


If the current scalable layer representation does only contain one access unit or the value of avgFR( tSeg ) is constant over
all temporal segments of the scalable layer representation, the frame rate is constant; otherwise, the frame rate is not
constant. constant_frm_rate_idc[ i ] equal to 0 indicates that the frame rate of the current scalable layer representation is
not constant. constant_frm_rate_idc[ i ] equal to 1 indicates that the frame rate of the current scalable layer representation
is constant. constant_frm_rate_idc[ i ] equal to 2 indicates that the frame rate of the current scalable layer representation
may or may not be constant. The value of constant_frm_rate_idc[ i ] shall be in the range of 0 to 2, inclusive.


**avg_frm_rate[** i **]** indicates the average frame rate, in units of frames per 256 seconds, of the representation of the current
scalable layer. With fTotal being the number of frames, complementary field pairs, and non-paired fields in the current
scalable layer representation, t1 being the removal time (in seconds) of the access unit associated with the scalability
information SEI message, and t2 being the removal time (in seconds) of the last access unit (in decoding order) of the target
access unit set, the following applies:


- If t1 is not equal to t2, the following condition shall be true:


avg_frm_rate[ i ] = = Round( fTotal * 256 ÷ ( t2 − t1 ) ) (G-375)


- Otherwise (t1 is equal to t2), the following condition shall be true:


avg_frm_rate[ i ] = = 0 (G-376)


**frm_width_in_mbs_minus1[** i **]** and **frm_height_in_mbs_minus1[** i **]** indicate the width and height, respectively, of the
decoded pictures for the current scalable layer representation (when sub_pic_layer_flag[ i ] is equal to 0) or the sub-picture
area inside the decoded pictures for the current sub-picture scalable layer (when sub_pic_layer_flag[ i ] is equal to 1).
When frame_mbs_only_flag is equal to 0 for any primary coded VCL NAL unit of the current scalable layer,
( frm_height_in_mbs_minus1[ i ] + 1 ) % 2 shall be equal to 0.


Let picSubset be the set of the primary coded pictures inside the current scalable layer representation that contain any VCL
NAL unit with dependency_id equal to dependency_id[ i ]. For decoding the picture set picSubset, the following applies:


- If sub_pic_layer_flag[ i ] is equal to 0, the width and height of a decoded picture are equal to
frm_width_in_mbs_minus1[ i ] + 1 and ( ( frm_height_in_mbs_minus1[ i ] + 1 ) / ( 1 + field_pic_flag ) )
macroblocks, respectively, with field_pic_flag being the slice header syntax element in the slices with dependency_id
equal to dependency_id[ i ] of the corresponding primary coded picture. The width and height of the decoded pictures
that are indicated by frm_width_in_mbs_minus1[ i ] and frm_height_in_mbs_minus1[ i ], respectively, shall be
identical to the width and height of the decoded pictures that are specified by the syntax elements
pic_width_in_mbs_minus1 and pic_height_in_map_units_minus1, respectively, of the SVC sequence parameter sets
referenced in the corresponding coded slice NAL units with dependency_id equal to dependency_id[ i ].


- Otherwise (sub_pic_layer_flag[ i ] is equal to 1), the width and height of the sub-picture area inside a decoded picture
are equal to frm_width_in_mbs_minus1[ i ] + 1 and ( ( frm_height_in_mbs_minus1[ i ] + 1 ) / ( 1 + field_pic_flag ) )
macroblocks, respectively, with field_pic_flag being the slice header syntax element in the slices with dependency_id
equal to dependency_id[ i ] of the corresponding primary coded picture. The sub-picture area that is indicated by
frm_width_in_mbs_minus1[ i ] and frm_height_in_mbs_minus1[ i ] shall be less than the area of the decoded





pictures, which is specified by the syntax elements pic_width_in_mbs_minus1 and pic_height_in_map_units_minus1
of the SVC sequence parameter sets referenced in the corresponding coded slice NAL units with dependency_id equal
to dependency_id[ i ].


The variable FrmWidthInMbs[ i ] is set equal to ( frm_width_in_mbs_minus1[ i ] + 1 ). The variable FrmHeightInMbs[ i ]
is set equal to ( frm_height_in_mbs_minus1[ i ] + 1 ). The variable FrmSizeInMbs[ i ] is set equal to
( FrmWidthInMbs[ i ] * FrmHeightInMbs[ i ] ).


**base_region_layer_id[** i **]** indicates the layer identifier layer_id[ b ] of the scalable layer b that represents the base region
for the current scalable layer as specified in the following. The value of base_region_layer_id[ i ] shall be in the range of 0
to 2047, inclusive.


Let picSubset be the set of the primary coded pictures, inside the current scalable layer representation, that contain any
VCL NAL unit with dependency_id equal to dependency_id[ i ]. Let basePicSubset be the set of the primary coded
pictures, inside the representation of the scalable layer b with layer_id[ b ] equal to base_region_layer_id[ i ], that contain
any VCL NAL unit with dependency_id equal to dependency_id[ b ]. Depending on sub_pic_layer_flag[ i ], the following
applies:


- If sub_pic_layer_flag[ i ] is equal to 0, it is indicated that the decoded pictures for the picture set picSubset represent
a subset of the areas that are represented by the decoded pictures for the picture set basePicSubset. The value of
dependency_id[ b ] for the scalable layer b shall be less than the value of dependency_id[ i ] for the current scalable
layer. The area that is represented by the decoded pictures for the picture set picSubset is also referred to as the region
represented by the current scalable layer and the area represented by the corresponding decoded pictures for the picture
set basePicSubset is also referred to as the base region for the current scalable layer.


- Otherwise (sub_pic_layer_flag[ i ] is equal to 1), it is indicated that the sub-picture area inside the decoded pictures
for the picture set picSubset represents a proper subset of the areas that are represented by the decoded pictures for
the picture set basePicSubset. The value of dependency_id[ b ] shall be equal to the value of dependency_id[ i ] for
the current sub-picture scalable layer. The area that is represented by the sub-picture area inside the decoded pictures
for the picture set picSubset is also referred to as the region represented by the current scalable layer and the area
represented by the corresponding decoded pictures for the picture set basePicSubset is also referred to as the base
region for the current scalable layer.


NOTE 14 – When sub_pic_layer_flag[ i ] is equal to 1, the base region represents the area of the layer pictures with
dependency_id equal to dependency_id[ i ].


The scalability information SEI message shall contain information for the scalable layer b with layer_id[ b ] equal to
base_region_layer_id[ i ], the value of sub_pic_layer_flag[ b ] for the scalable layer b shall be equal to 0, and the value of
temporal_id[ i ] for the scalable layer b shall be equal to the value of temporal_id[ i ] for the current scalable layer.


**dynamic_rect_flag[** i **]** equal to 1 indicates that the region represented by the current scalable layer representation is a
dynamically changing rectangular subset of the base region. dynamic_rect_flag[ i ] equal to 0 indicates that the region
represented by the current scalable layer representation is a fixed rectangular subset of the base region and is specified by
the syntax elements horizontal_offset[ i ], vertical_offset[ i ], region_width[ i ], and region_height[ i ]. When
sub_pic_layer_flag[ i ] is equal to 1, dynamic_rect_flag[ i ] shall be equal to 0.


**horizontal_offset[** i **]**, **vertical_offset[** i **]**, **region_width[** i **]**, and **region_height[** i **]** indicate the position and size of the
region represented by the current scalable layer in relation to its base region.


Let picSubset be the set of the primary coded pictures, inside the current scalable layer representation, that contain any
VCL NAL unit with dependency_id equal to dependency_id[ i ]. Let basePicSubset be the set of the primary coded
pictures, inside the representation of the scalable layer b with layer_id[ b ] equal to base_region_layer_id[ i ], that contain
any VCL NAL unit with dependency_id equal to dependency_id[ b ]. Depending on sub_pic_layer_flag[ i ], the following
applies:


- If sub_pic_layer_flag[ i ] is equal to 0, the top-left luma frame sample in the decoded pictures for picture set picSubset
corresponds to the luma frame sample at the luma frame sample location ( horizontal_offset[ i ], vertical_offset[ i ] )
in the decoded pictures for the picture set basePicSubset. The region represented by the decoded pictures for picture
set picSubset represents an area of (region_width[ i ])x(region_height[ i ]) luma frame samples in the decoded pictures
for the picture set basePicSubset. When frame_mbs_only_flag is equal to 0 for any primary coded VCL NAL unit of
the current scalable layer, (vertical_offset[ i ] % 2) and (region_height[ i ] % 2) shall both be equal to 0.


- Otherwise (sub_pic_layer_flag[ i ] is equal to 1), the top-left luma frame sample of the sub-picture area in the decoded
pictures for picture set picSubset corresponds to the luma frame sample at the luma frame sample location
( horizontal_offset[ i ], vertical_offset[ i ] ) in the decoded pictures for the picture set basePicSubset. The region
represented by the sub-picture area in the decoded pictures for picture set picSubset represents an area of
(region_width[ i ])x(region_height[ i ]) luma frame samples in the decoded pictures for the picture set basePicSubset.





(horizontal_offset[ i ] % 16) and (region_width[ i ] % 16) shall both be equal to 0, and depending on the values of
frame_mbs_only_flag for the primary coded VCL NAL units of the current scalable layer, the following applies:


   - If frame_mbs_only_flag is equal to 1 for all primary coded VCL NAL units of the current scalable layer,
(vertical_offset[ i ] % 16) and (region_height[ i ] % 16) shall both be equal to 0.


   - Otherwise (frame_mbs_only_flag is equal to 0 for any primary coded VCL NAL units of the current scalable
layer), (vertical_offset[ i ] % 32) and (region_height[ i ] % 32) shall both be equal to 0.


When sub_pic_layer_flag[ i ] is equal to 1 and frm_size_info_present_flag[ i ] is equal to 1, the values of region_width[ i ]
and region_height[ i ] shall be equal to ( FrmWidthInMbs[ i ] << 4) and (FrmHeightInMbs[ i ] << 4 ), respectively.


**roi_id** [ i ] specifies a region-of-interest identifier for the region represented by the current sub-picture scalable layer, which
may be used for identifying the purpose of the current sub-picture scalable layer by an application. The value of roi_id[ i ]
shall be in the range of 0 to 63, inclusive.


Let layerIdA and layerIdB be the layer identifiers of two scalable layers A and B, respectively, both of which having
sub_pic_layer_flag[ i ] equal to 1, and roiIdA and roidIdB be the region-of-interest identifiers of the scalable layers A and
B, respectively. When layerIdA is less than layerIdB, roiIdA shall not be greater than roidIdB.


**iroi_grid_flag** [ i ] specifies how the IROI division information is indicated for the current scalable layer. iroi_grid_flag[ i ]
equal to 1 indicates that all IROIs for the current scalable layer are aligned on a fixed-size grid as specified in the following
and that the syntax elements grid_width_in_mbs_minus1[ i ] and grid_width_in_mbs_minus1[ i ] for the current scalable
layer are present in the scalability information SEI message. iroi_grid_flag[ i ] equal to 0 indicates that the IROIs for the
current scalable layer may or may not be aligned on a fixed-size grid.


All scalable layers with the same value of dependency_id[ i ] for which scalability information is present in the scalability
information SEI message and for which iroi_division_info_present_flag[ i ] is equal to 1 shall have the same value of
iroi_grid_flag[ i ].


**grid_width_in_mbs_minus1** [ i ] and **grid_height_in_mbs_minus1** [ i ] indicate the size of the IROI grid for the current
scalable layer. When frame_mbs_only_flag is equal to 0 for any primary coded VCL NAL unit of the current scalable
layer, ( grid_height_in_mbs_minus1[ i ] + 1 ) % 2 shall be equal to 0.


The value of grid_width_in_mbs_minus1[ i ] shall be in the range of 0 to FrmWidthInMbs[ i ] − 1, inclusive. The value of
grid_height_in_mbs_minus1[ i ] shall be in the range of 0 to FrmHeightInMbs[ i ] − 1, inclusive.


Let numX and numY be equal to ( FrmWidthInMbs[ i ] + grid_width_in_mbs_minus1[ i ] ) /
( grid_width_in_mbs_minus1[ i ] + 1 ) and ( FrmHeightInMbs[ i ] + grid_height_in_mbs_minus1[ i ] ) /
( grid_height_in_mbs_minus1[ i ] + 1 ), respectively.


The layer pictures with dependency_id equal to dependency_id[ i ] are partitioned into (numX * numY) IROIs. Let
( xI[ k ], yI[ k ] ) be the location of the top-left luma sample of the k-th IROI relative to the top-left luma sample of the
layer picture and let w[ k ] and h[ k ] be the width and height, in luma samples, of the k-th IROI in the layer picture. With
field_pic_flag being the slice header syntax element for a particular layer picture with dependency_id equal to
dependency_id[ i ], the location of the top-left luma sample and the width and height of the k-th IROI, with
k = 0..(numX * numY − 1), are given by


xI[ k ] = 16 * ( k % numX ) * ( grid_width_in_mbs_minus1[ i ] + 1 ) (G-377)


yI[ k ] = 16 * ( k / numX ) * ( grid_height_in_mbs_minus1[ i ] + 1 ) / ( 1 + field_pic_flag ) (G-378)


w[ k ] = Min( 16 * ( grid_width_in_mbs_minus1[ i ] + 1 ), 16 * FrmWidthInMbs[ i ] − xI[ k ] ) (G-379)


h[ k ] = Min( 16 * ( grid_height_in_mbs_minus1[ i ] + 1 ) / ( 1 + field_pic_flag ),
16 * FrmHeightInMbs[ i ] / ( 1 + field_pic_flag ) − yI[ k ] ) (G-380)


All scalable layers with the same value of dependency_id[ i ] for which scalability information is present in the scalability
information SEI message and for which iroi_division_info_present_flag[ i ] is equal to 1 and iroi_grid_flag[ i ] is equal to
1 shall have the same values of grid_width_in_mbs_minus1[ i ] and grid_height_in_mbs_minus1[ i ].


**num_rois_minus1** [ i ] plus 1 indicates the number of IROIs for the current scalable layer.


Depending on the primary coded VCL NAL units of the current scalable layer, the following applies:


- If frame_mbs_only_flag is equal to 1 for all primary coded VCL NAL units of the current scalable layer, the value of
num_rois_minus1[ i ] shall be in the range of 0 to FrmSizeInMbs[ i ] − 1, inclusive.


- Otherwise (frame_mbs_only_flag is equal to 0 for any primary coded VCL NAL unit of the current scalable layer),
the value of num_rois_minus1[ i ] shall be in the range of 0 to FrmSizeInMbs[ i ] / 2 − 1, inclusive.





All scalable layers with the same value of dependency_id[ i ] for which scalability information is present in the scalability
information SEI message and for which iroi_division_info_present_flag[ i ] is equal to 1 and iroi_grid_flag[ i ] is equal to
0 shall have the same value of num_rois_minus1[ i ].


**first_mb_in_roi** [ i ][ j ] indicates the macroblock address of the first macroblock in the j-th IROI for the current scalable
layer. The value of first_mb_in_roi[ i ][ j ] shall be in the range of 0 to FrmSizeInMbs[ i ] − 1, inclusive. When j is greater
than 0, the value of first_mb_in_roi[ i ][ j ] shall not be equal to any of the values of first_mb_in_roi[ i ][ k ] with
k = 0..(j − 1).


The variables firstMbY and firstMbInROIFld are derived as


firstMbY       = first_mb_in_roi[ i ][ j ] − ( first_mb_in_roi[ i ][ j ] % FrmWidthInMbs[ i ] ) (G-3 81)


firstMbInROIFld = ( firstMbY >> 1 ) + ( first_mb_in_roi[ i ][ j ] % FrmWidthInMbs[ i ] ) (G-382)


When frame_mbs_only_flag is equal to 0 for any primary coded VCL NAL unit of the current scalable layer,
(firstMbY % 2) shall be equal to 0.


For each dependency representation that contains any primary coded VCL NAL unit of the current scalable layer, the
following applies:


- If field_pic_flag is equal to 0 and MbaffFrameFlag is equal to 0 for the dependency representation, the value of
first_mb_in_roi[ i ][ j ] shall be equal to the syntax element first_mb_in_slice in the slice that belongs to the
dependency representation and covers the top-left macroblock of the j-th IROI.


- Otherwise (field_pic_flag is equal to 1 or MbaffFrameFlag is equal to 1 for the dependency representation), the value
of firstMbInROIFld shall be equal to the syntax element first_mb_in_slice in the slice that belongs to the dependency
representation and covers the top-left macroblock of the j-th IROI.


**roi_width_in_mbs_minus1** [ i ][ j ] and **roi_height_in_mbs_minus1** [ i ][ j ] specify the size of the j-th IROI for the
current scalable layer. When frame_mbs_only_flag is equal to 0 for any primary coded VCL NAL unit of the current
scalable layer, ( roi_height_in_mbs_minus1[ i ][ j ] + 1 ) % 2 shall be equal to 0.


The value of roi_width_in_mbs_minus1[ i ][ j ] shall be in the range of 0 to (FrmWidthInMbs[ i ] − 1 −
( first_mb_in_roi[ i ][ j ] % FrmWidthInMbs[ i ] )), inclusive. The value of roi_height_in_mbs_minus1[ i ][ j ] shall be in
the range of 0 to (FrmHeightInMbs[ i ] − 1 − ( firstMbY / FrmWidthInMbs[ i ] )), inclusive.


With field_pic_flag being the slice header syntax element for a particular layer picture with dependency_id equal to
dependency_id[ i ], the width and height of the j-th IROI in the layer pictures with dependency_id equal to
dependency_id[ i ] are equal to 16 * ( roi_width_in_mbs_minus1[ i ][ j ] + 1 ) and
16 * ( roi_height_in_mbs_minus1[ i ][ j ] + 1 ) / ( 1 + field_pic_flag ), respectively, in units of luma samples.


All scalable layers with the same value of dependency_id[ i ] for which scalability information is present in the scalability
information SEI message and for which iroi_division_info_present_flag[ i ] is equal to 1 and iroi_grid_flag[ i ] is equal to
0 shall have the same values of first_mb_in_roi[ i ][ j ], roi_width_in_mbs_minus1[ i ][ j ], and
roi_height_in_mbs_minus1[ i ][ j ] with j in the range of 0 to num_rois_minus1[ i ], inclusive.


**num_directly_dependent_layers[** i **]** specifies the number of the syntax elements
directly_dependent_layer_id_delta_minus1[ i ][ j ] that are present for the current scalable layer. The value of
num_directly_dependent_layers shall be in the range of 0 to 255, inclusive.


**directly_dependent_layer_id_delta_minus1[** i **][** j **]** plus 1 indicates the difference between the value of layer_id[ i ] for
the current scalable layer and the layer identifier of a particular scalable layer, on which the current scalable layer directly
depends. The value of directly_dependent_layer_id_delta_minus1[ i ][ j ] shall be in the range of 0 to layer_id[ i ] − 1,
inclusive. The layer identifier of the particular scalable layer, on which the current scalable layer directly depends, is equal
to layer_id[ i ] − directly_dependent_layer_id_delta_minus1[ i ][ j ] − 1. The scalability information SEI message shall
contain information for a scalable layer b with layer_id[ b ] equal to
layer_id[ i ] − directly_dependent_layer_id_delta_minus1[ i ][ j ] − 1 and this information shall not contain a value of
layer_dependency_info_src_layer_id_delta[ i ] equal to 0.


Let setOfDepLayers be the set union of the representations of the scalable layers b that have layer_id[ b ] equal to
layer_id[ i ] − directly_dependent_layer_id_delta_minus1[ i ][ j ] − 1, with j = 0..num_directly_dependent_layers[ i ] − 1.
When layer_dependency_info_present_flag[ i ] is equal to 1, the set setOfDepLayers shall not contain any scalable layer,
on which the current scalable layer does not directly or indirectly depends and the current scalable layer shall not depend
on any scalable layer that is not included in the set setOfDepLayers.


**layer_dependency_info_src_layer_id_delta** [ i ] greater than 0 indicates that the current scalable layer has the same layer
dependency information as the scalable layer with layer identifier equal to
layer_id[ i ] − layer_dependency_info_src_layer_id_delta[ i ]. layer_dependency_info_src_layer_id_delta[ i ] equal to 0
specifies that the layer dependency information of the current scalable layer is not present in the scalability information





SEI message. The value of layer_dependency_info_src_layer_id_delta[ i ] shall be in the range of 0 to layer_id[ i ],
inclusive. When layer_dependency_info_src_layer_id_delta[ i ] is greater than 0, the scalability information SEI message
shall contain information for a scalable layer b with layer_id[ b ] equal to
layer_id[ i ] − layer_dependency_info_src_layer_id_delta[ i ] and this information shall not contain a value of
layer_dependency_info_src_layer_id_delta[ b ] equal to 0.


When layer_dependency_info_present_flag[ i ] is equal to 0 and layer_dependency_info_src_layer_id_delta[ i ] is greater
than 0, the set of scalable layers on which the current scalable layer depends shall be identical to the set of layers on which
the scalable layer b with layer_id[ b ] equal to layer_id[ i ] − layer_dependency_info_src_layer_id_delta[ i ] depends.


NOTE 15 – When layer_dependency_info_src_layer_id_delta[ i ] equal to 0 is not present for the current scalable layer, the
representation of the current scalable layer is specified by the syntax element layer_dependency_info_src_layer_id_delta[ i ]
or by the syntax elements directly_dependent_layer_id_delta_minus1[ i ][ j ], with
j = 0..num_directly_dependent_layers[ i ] − 1.


NOTE 16 – A change for the layer dependency information may be signalled by the presence of one or more layer dependency
change SEI messages. When a scalability information SEI message specifies that a scalable layer A does not directly or
indirectly depend on a scalable layer B, this relationship applies to the complete target access unit set. When a scalability
information SEI message specifies that a scalable layer A does directly or indirectly depend on a scalable layer B, a following
layer dependency change SEI message may indicate that this dependency does not apply for a subset of the target access unit
set.


**num_seq_parameter_sets[** i **]** indicates the number of different sequence parameter sets that are referred to by the primary
coded VCL NAL units of the current scalable layer representation. The value of num_seq_parameter_sets[ i ] shall be in
the range of 0 to 32, inclusive.


**seq_parameter_set_id_delta[** i **][** j **]** indicates the smallest value of the seq_parameter_set_id of any sequence parameter
set required for decoding the representation of the current scalable layer, if j is equal to 0. Otherwise (j is greater than 0),
seq_parameter_set_id_delta[ i ][ j ] indicates the difference between the value of the seq_parameter_set_id of the j-th
required sequence parameter set and the value of the seq_parameter_set_id of the (j − 1)-th required sequence parameter
set for decoding the representation of the current scalable layer. The value of seq_parameter_set_id_delta[ i ][ j ] shall not
be greater than 31. When j is greater than 0, the value of seq_parameter_set_id_delta[ i ][ j ] shall not be equal to 0. When
parameter_sets_info_present_flag[ i ] is equal to 1, the primary coded VCL NAL units of the current scalable layer
representation shall not refer to any sequence parameter set for which the value of seq_parameter_set_id is not indicated
by the syntax elements seq_parameter_set_id_delta[ i ][ j ] for the current scalable layer and the syntax elements
seq_parameter_set_id_delta[ i ][ j ] for the current scalable layer shall not indicate any sequence parameter set that is not
referenced in any primary coded VCL NAL unit of the current scalable layer representation.


**num_subset_seq_parameter_sets[** i **]** indicates the number of different subset sequence parameter sets that are referred
to by the primary coded VCL NAL units of the current scalable layer representation. The value of
num_subset_seq_parameter_sets[ i ] shall be in the range of 0 to 32, inclusive.


**subset_seq_parameter_set_id_delta[** i **][** j **]** indicates the smallest value of the seq_parameter_set_id of any subset
sequence parameter set required for decoding the representation of the current scalable layer, if j is equal to 0. Otherwise
(j is greater than 0), subset_seq_parameter_set_id_delta[ i ][ j ] indicates the difference between the value of the
seq_parameter_set_id of the j-th required subset sequence parameter set and the value of the seq_parameter_set_id of the
(j − 1)-th required subset sequence parameter set for decoding the representation of the current scalable layer. The value
of subset_seq_parameter_set_id_delta[ i ][ j ] shall not be greater than 31. When j is greater than 0, the value of
subset_seq_parameter_set_id_delta[ i ][ j ] shall not be equal to 0. When parameter_sets_info_present_flag[ i ] is equal
to 1, the primary coded VCL NAL units of the current scalable layer representation shall not refer to any subset sequence
parameter set for which the value of seq_parameter_set_id is not indicated by the syntax elements
subset_seq_parameter_set_id_delta[ i ][ j ] for the current scalable layer and the syntax elements
subset_seq_parameter_set_id_delta[ i ][ j ] for the current scalable layer shall not indicate any subset sequence parameter
set that is not referenced in any primary coded VCL NAL unit of the current scalable layer representation.


**num_pic_parameter_sets_minus1[** i **]** plus 1 indicates the number of different picture parameter sets that are referred to
by the primary coded VCL NAL units of the current scalable layer representation. The value of
num_pic_parameter_sets_minus1[ i ] shall be in the range of 0 to 255, inclusive.


**pic_parameter_set_id_delta[** i **][** j **]** indicates the smallest value of the pic_parameter_set_id of any picture parameter set
required for decoding the representation of the current scalable layer, if j is equal to 0. Otherwise (j is greater than 0),
pic_parameter_set_id_delta[ i ][ j ] indicates the difference between the value of the pic_parameter_set_id of the j-th
required picture parameter set and the value of the pic_parameter_set_id of the (j − 1)-th required picture parameter set for
decoding the representation of the current scalable layer. The value of pic_parameter_set_id_delta[ i ][ j ] shall not be
greater than 255. When j is greater than 0, the value of pic_parameter_set_id_delta[ i ][ j ] shall not be equal to 0. When
parameter_sets_info_present_flag[ i ] is equal to 1, the primary coded VCL NAL units of the current scalable layer
representation shall not refer to any picture parameter set for which the value of pic_parameter_set_id is not indicated by
the syntax elements pic_parameter_set_id_delta[ i ][ j ] for the current scalable layer and the syntax elements





pic_parameter_set_id_delta[ i ][ j ] for the current scalable layer shall not indicate any picture parameter set that is not
referenced in any primary coded VCL NAL unit of the current scalable layer representation.


**parameter_sets_info_src_layer_id_delta** [ i ] greater than 0 indicates that the values of seq_parameter_set_id of the
sequence parameter sets and subset sequence parameter sets and the values of pic_parameter_set_id of the picture
parameter sets that are referred to by the primary coded VCL NAL units of the current scalable layer representation are the
same as those that are referred to by the primary coded VCL NAL units of the representation of the scalable layer b with
the layer identifier layer_id[ b ] equal to layer_id[ i ] − parameter_sets_info_src_layer_id_delta[ i ]. When
parameter_sets_info_src_layer_id_delta[ i ] is greater than 0, the scalability information SEI message shall contain
information for a scalable layer b with layer_id[ b ] equal to layer_id[ i ] − parameter_sets_info_src_layer_id_delta[ i ] and
this information shall not contain a value of parameter_sets_info_src_layer_id_delta[ b ] equal to 0.


parameter_sets_info_src_layer_id_delta[ i ] equal to 0 indicates that the values of seq_parameter_set_id of the sequence
parameter sets and subset sequence parameter sets and the values of pic_parameter_set_id of the picture parameter sets
that are referred to by the primary coded VCL NAL units of the current scalable layer representation are not indicated in
the scalability information SEI message.


The value of parameter_sets_info_src_layer_id_delta[ i ] shall be in the range of 0 to layer_id[ i ], inclusive.


**motion_vectors_over_pic_boundaries_flag[** i **]** indicates the value of motion_vectors_over_pic_boundaries_flag, as
specified in clause E.2.1, that applies to the current scalable layer representation.


**max_bytes_per_pic_denom[** i **]** indicates the value of max_bytes_per_pic_denom, as specified in clause E.2.1, that
applies to the current scalable layer representation.


**max_bits_per_mb_denom[** i **]** indicates the value of max_bits_per_mb_denom, as specified in clause E.2.1, that applies
to the current scalable layer representation.


**log2_max_mv_length_horizontal[** i **]** and **log2_max_mv_length_vertical[** i **]** indicate the values of
log2_max_mv_length_horizontal and log2_max_mv_length_vertical, as specified in clause E.2.1, that apply to the current
scalable layer representation.

NOTE 17 – The maximum absolute value of a decoded vertical or horizontal motion vector component is also constrained by profile
and level limits as specified in Annex A and clause G.10.


**max_num_reorder_frames[** i **]** indicates the value of max_num_reorder_frames, as specified in clause E.2.1, that applies
to the current scalable layer representation.


**max_dec_frame_buffering[** i **]** indicates the value of max_dec_frame_buffering, as specified in clause E.2.1, that applies
to the current scalable layer representation.


**conversion_type_idc** [ i ] equal to 0 indicates that tcoeff_level_prediction_flag is equal to 1 for all primary coded slices of
the current scalable layer representation excluding those having no_inter_layer_pred_flag equal to 1 and that the
information specified by the syntax elements rewriting_profile_level_idc[ i ][ j ], rewriting_avg_bitrate[ i ][ j ], and
rewriting_max_bitrate[ i ][ j ], when present, is correct, though the method for converting the current scalable layer
representation into an alternative set of access units that conforms to one or more of the profiles specified in Annex A and
gives exactly the same decoding result as the current scalable layer representation is unspecified.


conversion_type_idc[ i ] equal to 1 indicates that the slice_header_restriction_flag in the subset sequence parameter sets
referred to by the primary coded VCL NAL units of the current scalable layer is equal to 1, that slice_skip_flag is equal to
1 for all primary coded VCL NAL units with no_inter_layer_pred_flag equal to 0 in the current scalable layer
representation, and that the alternative set of access units obtained by applying the following operations in sequential order
to the current scalable layer representation conforms to one or more of the profiles specified in Annex A:


1. For all picture parameter set NAL units referred to by NAL units with nal_unit_type equal to 1 or 5, change the

value of seq_parameter_set_id to be equal to the value of seq_parameter_set_id in a subset sequence parameter
set NAL unit with profile_idc equal to 83 or 86 that is referred to by slices with nal_unit_type equal to 20 of the
current scalable layer.


2. Remove all NAL units with nal_unit_type equal to 20 and slice_skip_flag equal to 1.


3. Remove all NAL units with nal_unit_type equal to 14.


4. Remove all redundant coded VCL NAL units.


5. In each access unit, remove all VCL NAL units with DQId less than DQIdMax, with DQIdMax being the

maximum value of DQId in the primary coded slices of the access unit after removing the NAL units with
nal_unit_type equal to 20 and slice_skip_flag equal to 1.


6. Remove the NAL unit header SVC extension from NAL units with nal_unit_type equal to 20.


7. For NAL units with nal_unit_type equal to 20 and idr_flag equal to 1, set nal_unit_type equal to 5.





8. For NAL units with nal_unit_type equal to 20 and idr_flag equal to 0, set nal_unit_type equal to 1.


9. Remove all SEI NAL units.


10. Remove all NAL units with nal_unit_type equal to 7.


11. For all NAL units with nal_unit_type equal to 15, set nal_unit_type equal to 7, remove all the syntax elements

after the syntax structure seq_parameter_set_data( ) and before the rbsp_trailing_bits( ) syntax structure, replace
the three bytes starting from profile_idc as specified by
rewriting_profile_level_idc[ i ][ entropy_coding_mode_flag ], when present, and change RBSP trailing bits
appropriately.


conversion_type_idc[ i ] equal to 2 indicates that slice_header_restriction_flag in the subset sequence parameter sets
referred to by the primary coded VCL NAL units of the current scalable layer is equal to 1, that no_inter_layer_pred_flag
is equal to 1 in all primary coded VCL NAL units of the current scalable layer, and that the alternative set of access units
obtained by applying the following operations in sequential order to the current scalable layer representation conforms to
one or more of the profiles specified in Annex A:


1. Remove all NAL units with nal_unit_type equal to 14.


2. Remove all redundant coded VCL NAL units.


3. In each access unit, remove all VCL NAL units with DQId less than DQIdMax.


4. Remove the NAL unit header SVC extension from NAL units with nal_unit_type equal to 20.


5. For NAL units with nal_unit_type equal to 20 and idr_flag equal to 1, set nal_unit_type equal to 5.


6. For NAL units with nal_unit_type equal to 20 and idr_flag equal to 0, set nal_unit_type equal to 1.


7. Remove all SEI NAL units.


8. Remove all NAL units with nal_unit_type equal to 7.


9. For all NAL units with nal_unit_type equal to 15, set nal_unit_type equal to 7, remove all the syntax elements

after the syntax structure seq_parameter_set_data( ) and before the rbsp_trailing_bits( ) syntax structure, replace
the three bytes starting from profile_idc as specified by
rewriting_profile_level_idc[ i ][ entropy_coding_mode_flag ], when present, and change RBSP trailing bits
appropriately.


The value of conversion_type_idc[ i ] shall be in the range of 0 to 2, inclusive.


For the following syntax elements rewriting_info_flag[ i ][ j ], rewriting_profile_level_idc[ i ][ j ],
rewriting_avg_bitrate[ i ][ j ], and rewriting_max_bitrate[ i ][ j ], the variable j specifies the value of
entropy_coding_mode_flag for all picture parameter set NAL units that are referenced in the VCL NAL units of the
alternative set of access units obtained by converting the current scalable layer representation, with values for j equal to 0
or 1 indicating use of the CAVLC or CABAC entropy coding methods, respectively.

NOTE 18 – It might be possible to convert the current scalable layer representation into two alternative sets of access units that
conform to one or more of the profiles specified in Annex A, with one of these sets having entropy_coding_mode_flag equal to 0
and the other set having entropy_coding_mode_flag equal to 1 in all picture parameter set NAL units that are referenced in the VCL
NAL units of the alternative set of access units.


**rewriting_info_flag** [ i ][ j ] equal to 1 specifies that information about the alternative set of access units obtained by
converting the current scalable layer representation is present in the scalability information SEI message.
rewriting_info_flag[ i ][ j ] equal to 0 specifies that information about the alternative set of access units is not present in
the scalability information SEI message. When rewriting_info_flag[ i ][ j ] is equal to 1, it is asserted that the information
signalled by the syntax elements rewriting_profile_level_idc[ i ][ j ], rewriting_avg_bitrate[ i ][ j ], and
rewriting_max_bitrate[ i ][ j ] is correct, though, when conversion_type_idc[ i ] is equal to 0 or the value of
entropy_coding_mode_flag is modified, the method for constructing the alternative set of access units is unspecified.


**rewriting_profile_level_idc** [ i ][ j ] indicates the conformance point of the alternative set of access units for the current
scalable layer representation after conversion. rewriting_profile_level_idc[ i ] is the exact copy of the three bytes consist
of profile_idc, constraint_set0_flag, constraint_set1_flag, constraint_set2_flag, constraint_set3_flag, constraint_set4_flag,
constraint_set5_flag, reserved_zero_2bits, and level_idc, as if these syntax elements were used to specify the profile and
level conformance of the alternative set of access units obtained by converting the scalable layer representation.


**rewriting_avg_bitrate[** i **]** [ j ] indicates the average bit rate of the alternative set of access units obtained by converting
the representation of the current scalable layer. The average bit rate of the alternative set of access units in bits per second
is given by BitRateBPS( rewriting_avg_bitrate[ i ][ j ] ) with the function BitRateBPS( ) being specified in Equation G370. The average bit rate is derived according to the access unit removal time specified in Annex C of the Recommendation
| International Standard.





**rewriting_max_bitrate[** i **]** [ j ] indicates an upper bound for the bit rate of the alternative set of access units obtained by
converting the representation of the current scalable layer, in any one-second time window of access unit removal time as
specified in Annex C. The upper bound for the bit rate of the alternative set of access units in bits per second is given by
BitRateBPS( rewriting_max_bitrate[ i ][ j ] ) with the function BitRateBPS( ) being specified in Equation G-370.


For the following specification, the terms priority layer, dependency layer, and priority layer representation are defined as
follows. A priority layer consists of the set of primary coded VCL NAL units, inside the target access unit set, that are
associated with a particular value of dependency_id and a value of alt_priority_id[ i ], as specified in clause G.13.2.4, that
is less than or equal to a particular priority identifier pId and the set of associated non-VCL NAL units. A priority layer is
associated with a particular value of dependency_id and a particular priority layer identifier pId. When present in the target
access unit, the following NAL units are associated non-VCL NAL units for a priority layer:


- sequence parameter set, subset sequence parameter set, and picture parameter set NAL units that are referenced in the
VCL NAL units of the priority layer (via the syntax element pic_parameter_set_id),


- sequence parameter set extension NAL units that are associated with a sequence parameter set NAL unit referenced
in the VCL NAL units of the priority layer,


- filler data NAL units that belong to an access unit containing VCL NAL units of the priority layer and are associated
with the same values of dependency_id and quality_id as the VCL NAL units of the priority layer in the same access
unit,


- SEI NAL units containing SEI messages, with payloadType not equal to 24, 28, or 29, that apply to subsets of the
bitstream that contain one or more VCL NAL units of the priority layer,


- access unit delimiter, end of sequence, and end of stream NAL units that are present in access units that contain VCL
NAL units of the priority layer.


The set of NAL units that represents the set union of all priority layers that are associated with the same value of
dependency_id is referred to as dependency layer. A dependency layer is associated with a particular value of
dependency_id.


A priority layer A is directly dependent on a priority layer B when any VCL NAL unit of the priority layer A references
data of any VCL NAL unit of the priority layer B through inter prediction or inter-layer prediction as specified in the
decoding process in clause G.8. A priority layer A is indirectly dependent on a priority layer B when the priority layer A
is not directly dependent on the priority layer B but there exists a set of n (with n being greater than 0) priority layers
{C0, .., Cn−1} with the following properties: The priority layer A is directly dependent on the priority layer C0, each priority
layer Ci with i in the range of 0 to n − 2, inclusive, is directly dependent on the priority layer Ci+1, and the priority layer
Cn−1 is directly dependent on the priority layer B.


The representation of a particular priority layer is the set of NAL units that represents the set union of the particular priority
layer and all priority layers on which the particular priority layer directly or indirectly depends. The representation of a
priority layer is also referred to as priority layer representation. In the following specification of this clause, the terms
representation of a priority layer and priority layer representation are also used for referring to the access unit set that can
be constructed from the NAL units of the priority layer representation. A priority layer representation can be decoded
independently of all NAL units that do not belong to the priority layer representation.


**pr_num_dIds_minus1** plus 1 specifies the number of dependency layers for which the priority layer characteristic
information as specified by the following syntax elements is present in the scalability information SEI message. The value
of pr_num_dIds_minus1 shall be in the range of 0 to 7, inclusive.


**pr_dependency_id[** i **]** specifies the value of dependency_id of the dependency layer for which the priority layer
characteristic information is signalled by the following syntax elements. When i is greater than 0, the value of
pr_dependency_id[ i ] shall not be equal to any of the values of pr_dependency_id[ j ] with j = 0..(i − 1).


**pr_num_minus1[** i **]** plus 1 specifies the number of priority layers with dependency_id equal to pr_dependency_id[ i ] for
which priority layer characteristic information as specified by the following syntax elements is present in the scalability
information SEI message. The value of pr_num_minus1[ i ] shall be in the range of 0 to 63, inclusive.


**pr_id[** i **][** j **]** specifies the priority identifier pId for a priority layer with dependency_id equal to pr_dependency_id[ i ].
The value of pr_id[ i ][ j ] shall be in the range of 0 to 63, inclusive. The target access unit set shall contain one or more
primary coded VCL NAL units that are associated with dependency_id equal to pr_dependency_id[ i ] and
alt_priority_id[ i ] equal to pr_id[ i ][ j ], where the value of alt_priority_id[ i ] that is associated with a primary coded VCL
NAL unit is specified in clause G.13.2.4. When j is greater than 0, the value of pr_id[ i ][ j ] shall not be equal to any of
the values of pr_id[ i ][ k ] with k = 0..(j − 1).


For the following specification inside the clause, the priority layer with dependency_id equal to the current value of
pr_dependency_id[ i ] and the priority layer identifier pId equal to the current value of pr_id[ i ][ j ] is referred to as the





current priority layer and the representation of the current priority layer is referred to as the current priority layer
representation.


**pr_profile_level_idc[** i **][** j **]** indicates the conformance point of the current priority layer representation.
pr_profile_level_idc[ i ] is the exact copy of the three bytes consisting of profile_idc, constraint_set0_flag,
constraint_set1_flag, constraint_set2_flag, constraint_set3_flag, constraint_set4_flag, constraint_set5_flag,
reserved_zero_2bits, and level_idc, as if these syntax elements were used to specify the profile and level conformance of
the current priority layer representation.


**pr_avg_bitrate[** i **][** j **]** indicates the average bit rate of the current priority layer representation. The average bit rate of the
current priority layer representation in bits per second is given by BitRateBPS( pr_avg_bitrate[ i ][ j ] ) with the function
BitRateBPS( ) being specified in Equation G-370. The average bit rate is derived according to the access unit removal time
specified in Annex C of this Recommendation | International Standard. In the following, bTotal is the number of bits in all
NAL units of the current priority layer representation, t1 is the removal time (in seconds) of the access unit associated with
the scalability information SEI message, and t2 is the removal time (in seconds) of the last access unit (in decoding order)
of the target access unit set.


With x specifying the value of pr_avg_bitrate[ i ], the following applies:


- If t1 is not equal to t2, the condition specified in Equation G-371 shall be fulfilled.


- Otherwise (t1 is equal to t2), the condition specified in Equation G-372 shall be fulfilled.


**pr_max_bitrate[** i **][** j **]** indicates an upper bound for the bit rate of the current priority layer representation in any
one-second time window of access unit removal time as specified in Annex C. The upper bound for the bit rate of the
current priority layer representation in bits per second is given by BitRateBPS( pr_max_bitrate[ i ][ j ] ) with the function
BitRateBPS( ) being specified in Equation G-370. The bit rate values are derived according to the access unit removal time
specified in Annex C of this Recommendation | International Standard. In the following, t1 is any point in time (in seconds),
t2 is set equal to t1 + 1, and bTotal is the number of bits in all NAL units of the current priority layer representation that
belong to access units with a removal time greater than or equal to t1 and less than t2. With x specifying the value of
pr_max_bitrate[ i ][ j ], the condition specified in Equation G-373 shall be obeyed.


**priority_id_setting_uri[** PriorityIdSettingUriIdx **]** is the PriorityIdSettingUriIdx-th byte of a null-terminated string
encoded in UTF-8 characters, specifying the universal resource identifier (URI) of the description of the method used to
calculate the priority_id values in the NAL unit headers for the target access unit set.


**G.13.2.2Sub-picture scalable layer SEI message semantics**


The sub-picture scalable SEI message provides a mechanism for associating a slice group set indicated in a motionconstrained slice group set SEI message with a sub-picture scalable layer.


In the following specification of this clause, the terms scalable layer, sub-picture scalable layer, and primary coded VCL
NAL unit are used as specified in clause G.13.2.1.


A sub-picture scalable layer SEI message shall not be succeeded, in decoding order, by a scalability information SEI
message inside the same access unit.


When a sub-picture scalable SEI message is present, the following applies:


- If the sub-picture scalable layer SEI message is included in a scalable nesting SEI message, a motion-constrained slice
group set SEI message, which is also referred to as the associated motion-constrained slice group set SEI message,
shall be present in the same scalable nesting SEI message and it shall immediately precede the sub-picture scalable
layer SEI message in decoding order. The scalable nesting SEI message that contains the sub-picture scalable layer
SEI message shall contain num_layer_representations_minus1 equal to 0 and sei_quality_id[ 0 ] equal to 0. The
variable depId is set equal to the value of sei_dependency_id[ 0 ] that is present in the scalable nesting SEI message
containing the sub-picture scalable layer SEI message.


- Otherwise (the sub-picture scalable layer SEI message is not included in a scalable nesting SEI message), the
sub-picture scalable layer SEI message shall be the first SEI payload in an SEI NAL unit and the NAL unit
immediately preceding the SEI NAL unit containing the sub-picture scalable layer SEI message shall be an SEI NAL
unit that contains a motion-constrained slice group set SEI message, which is also referred to as associated motionconstrained slice group set SEI message, as last SEI payload. The variable depId is set equal to 0.


The slice group set identified by the associated motion-constrained slice group set SEI message is referred to as the
associated slice group set of the sub-picture scalable layer SEI message.


The access unit associated with the sub-picture scalable layer SEI message shall not contain any primary coded VCL NAL
unit that has dependency_id equal to depId and IdrPicFlag equal to 0. The set of access units consisting of the access unit
associated with the sub-picture scalable layer SEI message and all succeeding access units in decoding order until, but
excluding, the next access unit that contains any primary coded VCL NAL unit with dependency_id equal to depId and





IdrPicFlag equal to 1 or that does not contain any primary coded VCL NAL units with IdrPicFlag equal to 0 (if present) or
the end of the bitstream (otherwise) is referred to as the target access unit set. The sub-picture scalable layer SEI message
applies to the target access unit set.

NOTE – The set of primary coded pictures in the target access unit set for a sub-picture scalable layer SEI message is identical to
the target picture set for the associated motion-constrained slice group set SEI message.


**layer_id** indicates, when the access unit containing the sub-picture scalable layer SEI message belongs to the target access
unit set of a scalability information SEI message, the layer identifier of the sub-picture scalable layer to which the coded
slice NAL units in the associated slice group set belong. The value of layer_id shall be in the range of 0 to 2047, inclusive.


The access unit containing the sub-picture scalable layer SEI message may or may not belong to the target access unit set
of a scalability information SEI message. When the access unit containing the sub-picture scalable layer SEI message
belongs to the target access unit set of a scalability information SEI message, the corresponding scalability information
SEI message may or may not contain information for a scalable layer i with layer identifier layer_id[ i ] equal to layer_id.
When the access unit containing the sub-picture scalable layer SEI message belongs to the target access unit set of a
scalability information SEI message and the corresponding scalability information SEI message contains information for
a scalable layer i with layer identifier layer_id[ i ] equal to layer_id, which is referred to as the current scalable layer in the
following, the following applies. The information for the current scalable layer in the scalability SEI shall contain
sub_pic_layer_flag[ i ] equal to 1. The sub-picture area for the current scalable layer i, which is specified by the syntax
elements horizontal_offset[ i ], vertical_offset[ i ], region_width[ i ], and region_height[ i ] in the scalability information
SEI message, shall be identical to the area specified by the associated slice group set.


**G.13.2.3Non-required layer representation SEI message semantics**


The non-required layer representation SEI message provides a mechanism for indicating which layer representations of
the current primary coded picture are not required for decoding dependency representations with a particular value of
dependency_id of the current primary coded picture and succeeding primary coded pictures, in decoding order.


The non-required layer representation SEI message shall not be included in a scalable nesting SEI message.


**num_info_entries_minus1** plus 1 specifies the number of dependency_id values for which non-required layer
representations are indicated in the SEI message. The value of num_info_entries_minus1 shall be in the range of 0 to 7,
inclusive.


**entry_dependency_id[** i **]** specifies the dependency_id value for which non-required layer representations are indicated
by the following syntax elements. The instances of entry_dependency_id[ i ] shall appear in increasing order of their
values.


The dependency representation of the primary coded picture with dependency_id equal to entry_dependency_id[ i ] is
referred to as the target dependency representation.


The target dependency representation may or may not be present in the access unit.


**num_non_required_layer_reps_minus1[** i **]** plus 1 specifies the number of non-required layer representations for the
target dependency representation that are indicated in the SEI message. The value of
num_non_required_layer_reps_minus1[ i ] shall be in the range of 0 to 127, inclusive.


**non_required_layer_rep_dependency_id[** i **][** j **]** indicates the value of dependency_id of the j-th non-required layer
representation for the target dependency representation.


**non_required_layer_rep_quality_id[** i **][** j **]** indicates the value of quality_id of the j-th non-required layer representation
for the target dependency representation.


The i-th non-required layer representation for the target dependency representation is the layer representation of the
primary coded picture that has dependency_id equal to non_required_layer_rep_dependency_id[ i ][ j ] and quality_id
equal to non_required_layer_rep_quality_id[ i ][ j ]. A non-required layer representation for the target dependency
representation is not required for decoding the target dependency representation and any dependency representation with
dependency_id equal to entry_dependency_id[ i ] of primary coded pictures that follow the current primary picture in
decoding order.


When DependencyIdMax is equal to entry_dependency_id[ i ], the VCL NAL units of the non-required layer
representations shall not be referenced through inter or inter-layer prediction in the decoding process as specified in
clause G.8.

NOTE – In addition to the i-th non-required layer representation for the target dependency representation, those layer representations
that have dependency_id equal to non_required_layer_rep_dependency_id[ i ][ j ] and quality_id greater than
non_required_layer_rep_quality_id[ i ][ j ] are also non-required layer representations for the target dependency representation.


The i-th non-required layer representation may or may not be present in the access unit.





**G.13.2.4Priority layer information SEI message semantics**


The priority layer information SEI message provides a mechanism for signalling alternative priority_id values for VCL
NAL units of the primary coded picture. The alternative values for priority_id indicate priority layers.


The priority layer information SEI message shall not be included in a scalable nesting SEI message.


**pr_dependency_id** specifies the value of dependency_id for the VCL NAL units for which alternative values for
priority_id are indicated.


**num_priority_ids** specifies the number of layer representations with dependency_id equal to pr_dependency_id for which
alternative values of priority_id are indicated.


**alt_priority_id[** i **]** specifies the alternative value for priority_id for the VCL NAL units of the primary coded picture that
have dependency_id equal to pr_dependency_id and quality_id equal to i.


The layer representation of the primary coded picture with dependency_id equal to pr_dependency_id and quality_id equal
to i may or may not be present in the access unit.


**G.13.2.5Layers not present SEI message semantics**


The layers not present SEI message provides a mechanism for signalling that NAL units of particular scalable layers
indicated by the preceding scalability information SEI message are not present in a particular set of access units.


In the following specification of this clause, the terms scalable layer and primary coded VCL NAL unit are used as specified
in clause G.13.2.1.


A layers not present SEI message shall not be included in a scalable nesting SEI message.


A layers not present SEI message shall not be present in an access unit that does not belong to the target access unit set of
any scalability information SEI message. A layers not present SEI message shall not be succeeded, in decoding order, by
a scalability information SEI message inside the same access unit. The set of access units consisting of the access unit
associated with the layers not present SEI message and all succeeding access units in decoding order until, but excluding,
the next access unit that contains a layers not present SEI message or that does not contain any primary coded VCL NAL
units with IdrPicFlag equal to 0 (if present), or the end of the bitstream (otherwise) is referred to as the target access unit
set. The layers not present SEI message applies to the target access unit set.


A layers not present SEI message refers to the most recent scalability information SEI message in decoding order. Each
scalable layer that is referred to in this clause is a scalable layer indicated in the most recent scalability information SEI
message in decoding order. Each layer identifier for a scalable layer that is referred to in this clause is a layer identifier for
a scalable layer indicated in the most recent scalability information SEI message in decoding order.

NOTE 1 – Layers not present SEI messages do not have a cumulative effect.


**num_layers** specifies the number of syntax elements layer_id[ i ] that are present in the layers not present SEI message.
The value of num_layers shall be in the range of 0 to 2047, inclusive.


**layer_id[** i **]** indicates the layer identifier of a scalable layer for which no VCL NAL units are present in the target access
unit set. The value of layer_id[ i ] shall be in the range of 0 to 2047, inclusive. The value of layer_id[ i ] shall be equal to
one of the values of layer_id[ i ] in the most recent scalability information SEI message. The target access unit set shall
not contain any VCL NAL unit of the scalable layer having a layer identifier equal to layer_id[ i ]. When i is greater than 0,
the value of layer_id[ i ] shall not be equal to any of the values of layer_id[ j ] with j = 0..(i − 1).

NOTE 2 – When an application removes NAL units from a scalable bitstream, e.g. in order to adapt the bitstream to a transmission
channel or the capabilities of a receiving device, and keeps the present layers not present SEI messages, it might need to modify the
content of some of the layers not present SEI messages and remove some other layers not present SEI messages in order to obtain a
bitstream conforming to this Recommendation | International Standard.


**G.13.2.6Layer dependency change SEI message semantics**


The layer dependency change SEI message provides a mechanism for signalling that the interdependencies between
particular scalable layers indicated by the preceding scalability information SEI message are changed for a particular set
of access units.


In the following specification of this clause, the terms scalable layer, representation of a scalable layer, scalable layer
representation, and primary coded VCL NAL unit are used as specified in clause G.13.2.1.


A layer dependency change SEI message shall not be included in a scalable nesting SEI message.


A layer dependency change SEI message shall not be present in an access unit that does not belong to the target access unit
set of any scalability information SEI message. A layer dependency change SEI message shall not be succeeded, in
decoding order, by a scalability information SEI message or a layers not present SEI message inside the same access unit.
The set of access units consisting of the access unit associated with the layer dependency change SEI message and all





succeeding access units in decoding order until, but excluding, the next access unit that contains a layer dependency change
SEI message or a layers not present SEI message or that does not contain any primary coded VCL NAL units with
IdrPicFlag equal to 0 (if present), or the end of the bitstream (otherwise) is referred to as the target access unit set. The
layer dependency change SEI message applies to the target access unit set.


A layer dependency change SEI message refers to the most recent scalability information SEI message in decoding order.
Each scalable layer that is referred to in this clause is a scalable layer indicated in the most recent scalability information
SEI message in decoding order. Each layer identifier for a scalable layer that is referred to in this clause is a layer identifier
for a scalable layer indicated in the most recent scalability information SEI message in decoding order.

NOTE 1 – Layer dependency change SEI messages do not have a cumulative effect.


The presence of the layer dependency change SEI message specifies the following. For a scalable layer with a layer
identifier equal to any value of layer_id[ i ] present in the layer dependency change SEI message, the layer dependency
relationship is changed for the target access unit set relative to the layer dependency relationship specified by the most
recent scalability information SEI message in decoding order. For a scalable layer with a layer identifier not equal to any
value of layer_id[ i ] present in the layer dependency change SEI message, the layer dependency relationship remains the
same as the one specified in the most recent scalability information SEI message in decoding order.


When, according to the layer dependency information indicated in the most recent scalability information SEI message in
decoding order, a scalable layer A does not directly or indirectly depend on another scalable layer B, the layer dependency
change SEI message shall not specify that the scalable layer A directly or indirectly depends on the scalable layer B.


When a scalable layer is considered to directly or indirectly depend on another scalable layer is specified in clause G.13.2.1,
with the target access unit set being the target access unit set for the layer dependency change SEI message.


**num_layers_minus1** plus 1 specifies the number of scalable layers for which a layer dependency information change
relative to the most recent scalability information SEI message, in decoding order, is indicated in the layer dependency
change SEI message. The value of num_layers_minus1 is in the range of 0 to 2047, inclusive.


**layer_id** [ i ] indicates the layer identifier of the scalable layer for which a layer dependency information change is indicated
by the following syntax elements. The value of layer_id[ i ] shall be in the range of 0 to 2047, inclusive. The value of
layer_id[ i ] shall be equal to one of the values of layer_id[ i ] in the most recent scalability information SEI message.
When i is greater than 0, the value of layer_id[ i ] shall not be equal to any of the values of layer_id[ j ] with j = 0..(i − 1).

NOTE 2 – When an application removes NAL units from a scalable bitstream, e.g. in order to adapt the bitstream to a transmission
channel or the capabilities of a receiving device, and keeps the present layer dependency change SEI messages, it might need to
modify the content of some of the layer dependency change SEI messages and remove some other layer dependency change SEI
messages in order to obtain a bitstream conforming to this Recommendation | International Standard.


For the following specification of this clause, the scalable layer with layer identifier equal to the current value of
layer_id[ i ] is referred to as the current scalable layer and the representation of the current scalable layer is referred to as
current scalable layer representation.


**layer_dependency_info_present_flag** [ i ] equal to 1 specifies that one or more syntax elements
dependent_layer_id_delta_minus1[ i ][ j ] indicating the layer dependency information for the current scalable layer are
present in the layer dependency change SEI message. layer_dependency_info_present_flag[ i ] equal to 0 specifies that the
syntax element layer_dependency_info_src_layer_id_delta_minus1[ i ] for the current scalable layer is present in the layer
dependency change SEI message.


**num_directly_dependent_layers** [ i ] specifies the number of the syntax elements
directly_dependent_layer_id_delta_minus1[ i ][ j ] that are present for the current scalable layer. The value of
num_directly_dependent_layers shall be in the range of 0 to 255, inclusive.


**directly_dependent_layer_id_delta_minus1** [ i ][ j ] plus 1 indicates the difference between the value of layer_id[ i ] for
the current scalable layer and the layer identifier of a particular scalable layer, on which the current scalable layer directly
depends. The value of directly_dependent_layer_id_delta_minus1[ i ][ j ] shall be in the range of 0 to layer_id[ i ] − 1,
inclusive. The layer identifier of the particular scalable layer, on which the current scalable layer directly depends, is equal
to layer_id[ i ] − directly_dependent_layer_id_delta_minus1 − 1. The most recent scalability information SEI message in
decoding order shall contain information for a scalable layer b with layer_id[ b ] equal to
layer_id[ i ] − directly_dependent_layer_id_delta_minus1[ i ][ j ] − 1 and this information shall not contain a value of
layer_dependency_info_src_layer_id_delta[ i ] equal to 0.


Let setOfDepLayers be the set union of the representations of the scalable layers b that have layer_id[ b ] equal to
layer_id[ i ] − directly_dependent_layer_id_delta_minus1[ i ][ j ] − 1, with j = 0..num_directly_dependent_layers[ i ] − 1.
When layer_dependency_info_present_flag[ i ] is equal to 1, the set setOfDepLayers shall not contain any scalable layer,
on which the current scalable layer does not directly or indirectly depends inside the target access unit set and the current
scalable layer shall not depend on any scalable layer, inside the target access unit set, that is not included in the set
setOfDepLayers.





**layer_dependency_info_src_layer_id_delta_minus1** [ i ] indicates that the current scalable layer has the same layer
dependency information as the scalable layer with layer identifier equal to layer_id[ i ] −
layer_dependency_info_src_layer_id_delta_minus1[ i ] − 1. The value of
layer_dependency_info_src_layer_id_delta_minus1[ i ] shall be in the range of 0 to layer_id[ i ] − 1, inclusive. The most
recent scalability information SEI message in decoding order shall contain information for a scalable layer b with
layer_id[ b ] equal to layer_id[ i ] − layer_dependency_info_src_layer_id_delta_minus1[ i ] − 1 and this information shall
not contain a value of layer_dependency_info_src_layer_id_delta[ b ] equal to 0.


When layer_dependency_info_present_flag[ i ] is equal to 0, the set of scalable layers on which the current scalable layer
depends inside the target access unit set shall be identical to the set of layers on which the scalable layer b with layer_id[ b ]
equal to layer_id[ i ] − layer_dependency_info_src_layer_id_delta_minus1[ i ] − 1 depends inside the target access unit
set.


**G.13.2.7Scalable nesting SEI message semantics**


The scalable nesting SEI message provides a mechanism for associating SEI messages with subsets of a bitstream.


A scalable nesting SEI message shall contain one or more SEI messages with payloadType not equal to 30 and it shall not
contain any SEI message with payloadType equal to 30. An SEI message contained in a scalable nesting SEI message is
referred to as a nested SEI message. An SEI message not contained in a scalable nesting SEI message is referred to as a
non-nested SEI message. The scope to which the nested SEI message applies is indicated by the syntax elements
all_layer_representations_in_au_flag, num_layer_representations_minus1, sei_dependency_id[ i ], sei_quality_id[ i ], and
sei_temporal_id, when present.


A buffering period SEI message and an SEI message of any other type shall not be nested in the same scalable nesting SEI
message. A picture timing SEI message and an SEI message of any other type shall not be nested in the same scalable
nesting SEI message.


**all_layer_representations_in_au_flag** equal to 1 specifies that the nested SEI message applies to all layer representations
of the access unit. all_layer_representations_in_au_flag equal to 0 specifies that the scope of the nested SEI message is
specified by the syntax elements num_layer_representations_minus1, sei_dependency_id[ i ], sei_quality_id[ i ], and
sei_temporal_id.


**num_layer_representations_minus1** plus 1 specifies, when num_layer_representations_minus1 is present, the number
of syntax element pairs sei_dependency_id[ i ] and sei_quality_id[ i ] that are present in the scalable nesting SEI message.
When num_layer_representations_minus1 is not present, it shall be inferred to be equal to ( numSVCLayers − 1 ) with
numSVCLayers being the number of layer representations that are present in the primary coded picture of the access unit.
The value of num_layer_representations_minus1 shall be in the range of 0 to 127, inclusive.


**sei_dependency_id** [ i ] and **sei_quality_id** [ i ] indicate the dependency_id and the quality_id values, respectively, of the
layer representations to which the nested SEI message applies. The access unit may or may not contain layer representations
with dependency_id equal to sei_dependency_id[ i ] and quality_id equal to sei_quality_id[ i ]. When i is greater than 0,
the value of (16 * sei_dependency_id[ i ] + sei_quality_id[ i ]) shall not be equal to any of the values of
(16 * sei_dependency_id[ j ] + sei_quality_id[ j ]) with j = 0..(i − 1).


When num_layer_representations_minus1 is not present, the values of sei_dependency_id[ i ] and sei_quality_id[ i ] for i
in the range of 0 to num_layer_representations_minus1 (with num_layer_representations_minus1 being the inferred value),
inclusive, shall be inferred as specified in the following:


1. Let setDQId be the set of the values DQId for all layer representations that are present in the primary coded

picture of the access unit.


2. For i proceeding from 0 to num_layer_representations_minus1, inclusive, the following applies:


a. sei_dependency_id[ i ] and sei_quality_id[ i ] are inferred to be equal to ( minDQId >> 4 ) and
( minDQId & 15 ), respectively, with minDQId being the smallest value (smallest value of DQId) in the
set setDQId.


b. The smallest value (smallest value of DQId) of the set setDQId is removed from setDQId and thus the

number of elements in the set setDQId is decreased by 1.


**sei_temporal_id** indicates the temporal_id value of the bitstream subset to which the nested SEI message applies. When
sei_temporal_id is not present, it shall be inferred to be equal to temporal_id of the access unit.


When the nested SEI message is a buffering period SEI message or a picture timing SEI message (i.e., payloadType is
equal to 0 or 1 for the nested SEI message), sei_temporal_id indicates the bitstream subset for which the nested buffering
period SEI message or picture timing SEI message applies. For a buffering period SEI message or picture timing SEI
message that is nested in a scalable nesting SEI message, sei_dependency_id[ i ], sei_quality_id[ i ], and sei_temporal_id
specify the greatest values of dependency_id, quality_id, and temporal_id, respectively, of the bitstream subsets to which





the nested buffering period SEI message or picture timing SEI message applies. The bitstream may or may not contain
access units with temporal_id equal to sei_temporal_id.


When the scalable nesting SEI message contains one or more SEI messages with payloadType not equal to 0 or 1,
sei_temporal_id shall be equal to the value of temporal_id for the access unit associated with the scalable nesting SEI
message. For an nested SEI message with payloadType not equal to 0 or 1, the values of sei_dependency_id[ i ],
sei_quality_id[ i ], and sei_temporal_id, present in or inferred for the associated scalable nesting SEI message, indicate the
values of dependency_id, quality_id, and temporal_id, respectively, of the VCL NAL units to which the nested SEI
message applies.


**sei_nesting_zero_bit** shall be equal to 0.


**G.13.2.8Base layer temporal HRD SEI message semantics**


The base layer temporal HRD SEI message provides HRD parameters for subsets of the base layer bitstream.


The base layer temporal HRD SEI message shall not be included in a scalable nesting SEI message. The base layer temporal
HRD SEI message shall not be present in access units that do not contain VCL NAL units of the primary coded picture
with nal_unit_type equal to 5.


When present, this SEI message applies to the target access unit set that consists of the current access unit and all
subsequent access units in decoding order until, but excluding, the next access unit containing a NAL unit of the primary
coded picture with nal_unit_type equal to 5 (if present) or the end of the bitstream (otherwise).


**num_of_temporal_layers_in_base_layer_minus1** plus 1 specifies the number of bitstream subsets inside the target
access unit set for which the following syntax elements are specified in the base layer temporal HRD SEI message. The
value of num_of_temporal_layers_in_base_layer_minus1 shall be in the range of 0 to 7, inclusive.


**sei_temporal_id[** i **]** specifies the temporal_id value of the i-th bitstream subset. When i is greater than 0, the value of
sei_temporal_id[ i ] shall not be equal to any of the values of sei_temporal_id[ j ] with j = 0..(i − 1).


Access units with temporal_id equal to sei_temporal_id[ i ] may or may not be present in the target access unit set. When
access units with temporal_id equal to sei_temporal_id[ i ] are not present in the target access unit set, the i-th bitstream
subset is considered as not existing.


When access units with temporal_id equal to sei_temporal_id[ i ] are present in the target access unit set, the i-th bitstream
subset is the bitstream subset that is obtained by invoking the bitstream extraction process as specified in clause G.8.8.1
for the target access unit set with tIdTarget equal to sei_temporal_id[ i ], dIdTarget equal to 0, and qIdTarget equal to 0 as
the inputs.


**sei_timing_info_present_flag[** i **]** equal to 1 specifies that sei_num_units_in_tick[ i ], sei_time_scale[ i ], and
sei_fixed_frame_rate_flag[ i ] are present in the base layer temporal HRD SEI message. sei_timing_info_present_flag[ i ]
equal to 0 specifies that sei_num_units_in_tick[ i ], sei_time_scale[ i ], and sei_fixed_frame_rate_flag[ i ] are not present
in the base layer temporal HRD SEI message.


The following syntax elements for the i-th bitstream subset are specified using references to Annex E. For these syntax
elements the same semantics and constraints as the ones specified in Annex E apply, as if these syntax elements
sei_num_units_in_tick[ i ], sei_time_scale[ i ], sei_fixed_frame_rate_flag[ i ], sei_nal_hrd_parameters_present_flag[ i ],
sei_vcl_hrd_parameters_present_flag[ i ], sei_low_delay_hrd_flag[ i ], and sei_pic_struct_present_flag[ i ] were present
as num_units_in_tick, time_scale, fixed_frame_rate_flag, nal_hrd_parameters_present_flag,
vcl_hrd_parameters_present_flag, low_delay_hrd_flag, and pic_struct_present_flag, respectively, in the VUI parameters
of the active SVC sequence parameter sets for the i−th bitstream subset.


The parameters for the i-th bitstream subset that are coded in the base layer temporal HRD SEI message shall be correct,
as if these parameters are used for conformance checking (as specified in Annex C) of the i-th bitstream subset.


**sei_num_units_in_tick[** i **]** indicates the value of num_units_in_tick, as specified in clause E.2.1, that applies to the i-th
bitstream subset.


**sei_time_scale[** i **]** indicates the value of time_scale, as specified in clause E.2.1, that applies to the i-th bitstream subset.


**sei_fixed_frame_rate_flag[** i **]** indicates the value of fixed_frame_rate_flag, as specified in clause E.2.1, that applies to
the i-th bitstream subset.


**sei_nal_hrd_parameters_present_flag[** i **]** indicates the value of nal_hrd_parameters_present_flag, as specified in
clause E.2.1, that applies to the i-th bitstream subset. When sei_nal_hrd_parameters_present_flag[ i ] is equal to 1, the
NAL HRD parameters that apply to the i-th bitstream subset immediately follow the
sei_nal_hrd_parameters_present_flag[ i ].





**sei_vcl_hrd_parameters_present_flag[** i **]** indicates the value of vcl_hrd_parameters_present_flag, as specified in
clause E.2.1, that applies to the i-th bitstream subset. When sei_vcl_hrd_parameters_present_flag[ i ] is equal to 1, the
VCL HRD parameters that apply to the i-th bitstream subset immediately follow the
sei_vcl_hrd_parameters_present_flag[ i ].


**sei_low_delay_hrd_flag[** i **]** indicates the value of low_delay_hrd_flag, as specified in clause E.2.1, that applies to the ith bitstream subset.


**sei_pic_struct_present_flag[** i **]** indicates the value of pic_struct_present_flag, as specified in clause E.2.1, that applies to
the i-th bitstream subset.


**G.13.2.9Quality layer integrity check SEI message semantics**


The quality layer integrity check SEI message provides a mechanism for detecting whether VCL NAL units with quality_id
greater than 0 of the primary coded picture have been removed from the bitstream.


The quality layer integrity check SEI message shall not be included in a scalable nesting SEI message.


**num_info_entries_minus1** plus 1 specifies the number of syntax element pairs entry_dependency_id[ i ] and
quality_layer_crc[ i ] that are present in the quality layer integrity check SEI message. The value of
num_info_entries_minus1 shall be in the range of 0 to 7, inclusive.


**entry_dependency_id[** i **]** specifies the dependency_id value of the dependency representation for which
quality_layer_crc[ i ] is indicated. The instances of entry_dependency_id[ i ] shall appear in increasing order of their
values. The dependency representation of the primary coded picture that has dependency_id equal to
entry_dependency_id[ i ] is referred to as target dependency representation.


The target dependency representation may or may not be present in the access unit.


**quality_layer_crc[** i **]** specifies the cyclic redundancy check for all the VCL NAL units with quality_id greater than 0 in
the target dependency representation.


Let crcVal be a variable that is derived as specified by the following ordered steps:


1. Let the variable qNalUnits[ ] be the one-dimensional array of bytes that contains a concatenation, in decoding

order, of the bytes of the nal_unit( ) syntax structures of all VCL NAL units with quality_id greater than 0 in the
target dependency representation, in decoding order.


2. Let the variable pLen be the sum of the NumBytesInNALunit variables of all VCL NAL units with quality_id

greater than 0 in the target dependency representation.


3. The value of crcVal is derived as specified by the following pseudo-code process:


qNalUnits[ pLen    ] = 0
qNalUnits[ pLen + 1 ] = 0
crcVal = 65535
for( bitIdx = 0; bitIdx < ( pLen + 2 ) * 8; bitIdx++ ) { (G-383)
crcMsb = ( crcVal >> 15 ) & 1
bitVal = ( qNalUnits[ bitIdx >> 3 ] >> ( 7 − ( bitIdx & 7 ) ) ) & 1
crcVal = ( ( ( crcVal << 1 ) + bitVal ) & 65535 ) ^ ( crcMsb * 4129 )
}


When the target dependency representation is present in the access unit, a value of quality_layer_crc[ i ] not equal to crcVal
indicates that one or more VCL NAL units with quality_id greater than 0 of the target dependency representation have
been removed from the bitstream and that the output pictures may show undesirable visual artefacts.


**G.13.2.10** **Redundant picture property SEI message semantics**


The redundant picture property SEI message indicates properties for layer representations of redundant coded pictures. In
the following, a layer representation of a redundant coded picture is also referred to as redundant coded layer representation
and a layer representation of the primary coded picture is also referred to as primary coded layer representation.


The redundant picture property SEI message shall not be included in a scalable nesting SEI message.


**num_dIds_minus1** plus 1 specifies the number of dependency_id values for which properties of redundant coded layer
representations are indicated in the redundant picture property SEI message. The value of num_dIds_minus1 shall be in
the range of 0 to 7, inclusive.


**dependency_id[** i **]** specifies the dependency_id value of the redundant coded layer representations for which properties
are indicated by the following syntax elements. When i is greater than 0, the value of dependency_id[ i ] shall not be equal
to any of the values of dependency_id[ j ] with j = 0..(i − 1).





**num_qIds_minus1[** i **]** plus 1 specifies the number of quality_id values for which properties of redundant coded layer
representations with dependency_id equal to dependency_id[ i ] are indicated by the following syntax elements. The value
of num_qIds_minus1[ i ] shall be in the range of 0 to 15, inclusive.


**quality_id[** i **][** j **]** specifies the quality_id value of the redundant coded layer representations with dependency_id equal to
dependency_id[ i ] for which properties are indicated by the following syntax elements. When j is greater than 0, the value
of quality_id[ i ][ j ] shall not be equal to any of the values of quality_id[ i ][ k ] with k = 0..(j − 1).


**num_redundant_pics_minus1[** i **][** j **]** plus 1 specifies the number of redundant coded layer representations with
dependency_id equal to dependency_id[ i ] and quality_id equal to quality_id[ i ][ j ] for which properties are indicated by
the following syntax elements. The value of num_redundant_pics_minus1[ i ][ j ] shall be in the range of 0 to 127,
inclusive.


**redundant_pic_cnt_minus1[** i **][** j **][** k **]** plus 1 specifies the redundant_pic_cnt value of the redundant coded layer
representation with dependency_id equal to dependency_id[ i ] and quality_id equal to quality_id[ i ][ j ] for which
properties are indicated by the following syntax elements. The value of redundant_pic_cnt_minus1[ i ][ j ][ k ] shall be in
the range of 0 to 126, inclusive. When k is greater than 0, the value of redundant_pic_cnt_minus1[ i ][ j ][ k ] shall not be
equal to any of the values of redundant_pic_cnt_minus1[ i ][ j ][ m ] with m = 0..(k − 1).


The redundant coded layer representation having dependency_id equal to dependency_id[ i ], quality_id equal to
quality_id[ i ][ j ], and redundant_pic_cnt equal to ( redundant_pic_cnt_minus1[ i ][ j ][ k ] + 1 ) is referred to as the target
redundant coded layer representation. The primary coded layer representation (redundant_pic_cnt is equal to 0) having
dependency_id equal to dependency_id[ i ] and quality_id equal to quality_id[ i ][ j ] is referred to as the target primary
coded layer representation.


The target redundant coded layer representation may or may not be present in the access unit. The target primary coded
layer representation may or may not be present in the access unit.


For the following specification, the picture that only consists of the target redundant coded layer representation and the
primary coded layer representations with DQId less than (dependency_id[ i ] << 4) + quality_id[ i ] is referred to as target
redundant coded picture and the picture that only consists of the target primary coded layer representation and the primary
coded layer representations with DQId less than (dependency_id[ i ] << 4) + quality_id[ i ] is referred to as target primary
coded picture.


For the following specification, the arrays mbType, subMbType, predFlagL0, predFlagL1, refIdxL0, refIdxL1, mvL0,
mvL1, rSL, rSCb, rSCr, cSL, cSCb, and cSCr represent the corresponding arrays of the collective term currentVars after
completion of the target macroblock decoding process as specified in clause G.8.1.5.6.


**pic_match_flag[** i **][** j **][** k **]** equal to 1 indicates that the target redundant coded layer representation is an exact copy of the
target primary coded layer representation, with the only difference in the value of redundant_pic_cnt.


**mb_type_match_flag[** i **][** j **][** k **]** equal to 1 indicates that the array mbType for the target redundant coded picture is
identical to the array mbType for the target primary coded picture.


**motion_match_flag[** i **][** j **][** k **]** equal to 1 indicates that, for each macroblock mbAddr in the target layer representation
of the target primary coded picture for which the derived macroblock type mbType[ mbAddr ] represents a P or B
macroblock type, the variables and arrays mbType[ mbAddr ], subMbType[ mbAddr ], predFlagL0[ mbAddr ],
predFlagL1[ mbAddr ], refIdxL0[ mbAddr ], refIdxL1[ mbAddr ], mvL0[ mbAddr ], and mvL1[ mbAddr ] for the target
redundant coded picture are identical to the corresponding variables and arrays for the target primary coded picture.


**residual_match_flag[** i **][** j **][** k **]** equal to 1 indicates that, for each macroblock mbAddr in the target layer representation
of the target primary coded picture for which the derived macroblock type mbType[ mbAddr ] represents a P or B
macroblock type, the associated reconstructed residual sample values in the arrays rSL, rSCb, and rSCr for the target
redundant coded picture are identical or close to the corresponding reconstructed residual sample values for the target
primary coded picture.


**intra_samples_match_flag[** i **][** j **][** k **]** equal to 1 indicates that, for each macroblock mbAddr in the target layer
representation of the target primary coded picture for which the derived macroblock type mbType[ mbAddr ] represents
an I macroblock type, the associated reconstructed sample values in the arrays cSL, cSCb, and cSCr for the target redundant
coded picture are identical or close to the corresponding reconstructed sample values for the target primary coded picture.


**G.13.2.11** **Temporal level zero dependency representation index SEI message semantics**


The temporal level zero dependency representation index SEI message provides a mechanism for detecting whether a
dependency representation with temporal_id equal to 0 required for decoding the current access unit is available when
NAL unit losses are expected during transport.


Let setOfDId be a set of dependency_id values that is derived as follows:





- If the temporal level zero dependency representation index SEI message is not included in a scalable nesting SEI
message, setOfDId consists of exactly one value, which is equal to 0.


- Otherwise (the temporal level zero dependency representation index SEI message is included in a scalable nesting
SEI message), setOfDId consists of the values sei_dependency_id[ i ] for all i in the range of 0 to
num_layer_representations_minus1, inclusive, that are present in the scalable nesting SEI message associated with
the temporal level zero dependency representation index SEI message. For the scalable nesting SEI message that
contains the temporal level zero dependency representation index SEI message, all_layer_representations_in_au_flag
shall be equal to 1 or the value of sei_quality_id[ i ] shall be equal to 0 for all values of i in the range of 0 to
num_layer_representations_minus1, inclusive.


All dependency representations that are referred to in the following specification inside this clause are dependency
representations of a primary coded picture. Unless specified otherwise, all dependency representation that are referred to
in the following are dependency representations of the primary coded picture of the access unit that is associated with the
temporal level zero dependency representation index SEI message.


The dependency representations of the access unit that have dependency_id equal to any value of the set setOfDId are also
referred to as associated dependency representations.


For each value of dId in the set setOfDId, the access unit may or may not contain a dependency representation with
dependency_id equal to dId.


**tl0_dep_rep_idx** indicates the temporal level zero index for the associated dependency representations, if temporal_id is
equal to 0. Otherwise (temporal_id is greater than 0), tl0_dep_rep_idx indicates the temporal level zero index of the
dependency representations of the most recent access unit with temporal_id equal to 0 in decoding order that have the same
value of dependency_id as any of the associated dependency representations.


For each value of dId in the set setOfDId, the following applies:


- If the dependency representation with dependency_id equal to dId contains a NAL unit with nal_unit_type equal to 5
or a NAL unit with nal_unit_type equal to 20 and idr_flag equal to 1, tl0_dep_rep_idx shall be equal to 0.


- Otherwise (the dependency representation with dependency_id equal to dId does not contain a NAL unit with
nal_unit_type equal to 5 or a NAL unit with nal_unit_type equal to 20 and idr_flag equal to 1), the following is
specified:


1. Let prevTL0AU be the most recent access unit in decoding order that has temporal_id equal to 0 and for which

the primary coded picture contains a dependency representation with dependency_id equal to dId.


2. Let prevTL0DepRep be the dependency representation with dependency_id equal to dId of the primary coded

picture in access unit prevTL0AU.


3. Let prevTL0DepRepIdx be equal to the value of tl0_dep_rep_idx that is associated with the dependency

representation prevTL0DepRep, as indicated by a corresponding temporal level zero dependency
representation index SEI message.


4. Depending on temporal_id of the current access unit, the following applies:


       - If temporal_id of the current access unit is equal to 0, tl0_dep_rep_idx shall be equal to
( prevTL0DepRepIdx + 1 ) % 256.


       - Otherwise (temporal_id of the current access unit is greater than 0), tl0_dep_rep_idx shall be equal to
prevTL0DepRepIdx.


When the temporal level zero dependency representation index SEI message is associated with a particular dependency
representation depRepA that has dependency_id equal dIdA and IdrPicFlag equal to 0, a temporal level zero dependency
representation index SEI message shall also be associated with the previous dependency representation dIdB in decoding
order that has dependency_id equal to dIdA and IdrPicFlag equal to 1 and all dependency representations with
dependency_id equal to dIdA and temporal_id equal to 0 that follow the dependency representation dIdB and precede the
dependency representation dIdA in decoding order.


NOTE – For the tl0_dep_rep_idx mechanism to be effectively used, transport mechanisms should ensure that the information is
present in every packet that carries data for the particular values of dependency_id.


**effective_idr_pic_id** indicates the latest value of idr_pic_id in decoding order present in this access unit or any preceding
access unit for dependency representations indicated by sei_dependency_id[ i ].


For each value of dId in the set setOfDId, the following applies:





- If the dependency representation with dependency_id equal to dId contains a NAL unit with nal_unit_type equal to 5
or a NAL unit with nal_unit_type equal to 20 and idr_flag equal to 1, effective_idr_pic_id shall be equal to idr_pic_id
of the dependency representation with dependency_id equal to dId.


- Otherwise (the dependency representation with dependency_id equal to dId does not contain a NAL unit with
nal_unit_type equal to 5 or a NAL unit with nal_unit_type equal to 20 and idr_flag equal to 1), effective_idr_pic_id
shall be equal to idr_pic_id of the previous dependency representation in decoding order with dependency_id equal
to dId that contains a NAL unit with nal_unit_type equal to 5 or a NAL unit with nal_unit_type equal to 20 and
idr_flag equal to 1.


**G.13.2.12** **Temporal level switching point SEI message semantics**


The temporal level switching point SEI message provides a mechanism for identifying temporal level switching points. If
a dependency representation is associated with a temporal level switching point SEI message, then it is a temporal level
switching point as specified subsequently and constrained by delta_frame_num. Otherwise, the dependency representation
may or may not be a temporal level switching point.


All dependency representations that are referred to in the following specification of this clause are dependency
representations of primary coded pictures.


In the following, let tId be the value of temporal_id of the access unit that is associated with the temporal level switching
point SEI message.


NOTE 1 – Let dId be the value of dependency_id that a bitstream adaptation process has used to generate a bitstream subset
subBitstreamA that contains dependency representations with dependency_id less than or equal to dId and temporal_id less than tId
of an input bitstream (that is conforming to this Recommendation | International Standard) until the current access unit, exclusive.
The bitstream adaptation process can infer from a temporal level switching point SEI message whether or not the bitstream subset
containing subBitstreamA and the dependency representations with dependency_id less than or equal to dId and temporal_id less
than or equal to tId of the input bitstream starting from the current access unit, inclusive, is conforming to this
Recommendation | International Standard.


The temporal level switching point SEI message shall not be present in access units with temporal_id equal to 0.


The temporal level switching point SEI message shall be included in a scalable nesting SEI message. For the scalable
nesting SEI message that contains the temporal level switching point SEI message, all_layer_representations_in_au_flag
shall be equal to 1 or the value of sei_quality_id[ i ] shall be equal to 0 for all values of i in the range of 0 to
num_layer_representations_minus1, inclusive.


The following semantics apply independently to each value of sei_dependency_id[ i ] indicated by the scalable nesting SEI
message containing the temporal level switching point SEI message. The current access unit, i.e., the access unit associated
with the temporal level switching point SEI message, may or may not contain a dependency representation with
dependency_id equal to sei_dependency_id[ i ]. When the current access unit contains a dependency representation with
dependency_id equal to sei_dependency_id[ i ], the following semantics apply.


The following semantics are specified in a way that they apply to a bitstream conforming to this Recommendation |
International Standard for which DependencyIdMax for the current access unit is equal to sei_dependency_id[ i ].


Let the switch-to dependency representation be the dependency representation in the current access unit that has
dependency_id equal to sei_dependency_id[ i ] and let maxFrameNum be the value of MaxFrameNum for the SVC
sequence parameter set that is the active SVC sequence parameter set for the current access unit (with DependencyIdMax
equal to sei_dependency_id[ i ]).


**delta_frame_num** indicates the difference between the frame_num values of the switch-to dependency representation and
the dependency representation with dependency_id equal to sei_dependency_id[ i ] in the required access unit, as specified
subsequently. The value of delta_frame_num shall be in the range of 1 − maxFrameNum to maxFrameNum − 1, inclusive.


Let currFrameNum be the frame_num value of the switch-to dependency representation. The variable requiredFrameNum
is set equal to currFrameNum − delta_frame_num. Let lastIdrAU be the most recent access unit in decoding order that
contains a dependency representation with dependency_id equal to sei_dependency_id[ i ] and IdrPicFlag equal to 1. The
bitstream shall contain an access unit that succeeds the access unit lastIdrAU and precedes the current access unit in
decoding order and contains a dependency representation with frame_num equal to requiredFrameNum and dependency_id
equal to sei_dependency_id[ i ]. The most recent access unit in decoding order that contains a dependency representation
with frame_num equal to requiredFrameNum and dependency_id equal to sei_dependency_id[ i ] is referred to as the
required access unit. The required access unit shall have a value of temporal_id that is equal to tId − 1.


The current access unit and all subsequent access units in decoding order for which temporal_id is less than or equal to tId
shall not reference any of the following access units through inter prediction in the decoding process specified in
clause G.8:


- access units that precede the required access unit in decoding order and have temporal_id equal to tId − 1,





- access units that precede the current access unit in decoding order and have temporal_id equal to tId.


NOTE 2 – The set of access units consisting of the current access unit and all access units with temporal_id less than or equal to tId
that follow the current access unit in decoding order can be decoded when all of the following access units, which precede the current
access unit in decoding order, have been decoded: all access units required for decoding the required access unit (i.e., all access units
that are directly or indirectly referenced through inter prediction in the decoding process for the required access unit), the required
access unit, and all access units with temporal_id less than tId that succeed the required access unit and precede the current access
unit in decoding order.
