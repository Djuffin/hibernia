**H.14** **Video usability information**


The specifications in Annex E apply with substituting MVC sequence parameter set for sequence parameter set.


With maxVOIdx being the maximum value of view order index for the views that reference the MVC sequence parameter
set containing the vui_parameters( ) syntax structure, the VUI parameters and the constraints specified in Annex E apply
to all views with a value of view order index that is less than or equal to maxVOIdx.


Additionally, the following applies.


**H.14.1** **MVC VUI parameters extension syntax**

|mvc_vui_parameters_extension( ) {|C|Descriptor|
|---|---|---|
|<br>**vui_mvc_num_ops_minus1**|0|ue(v)|
|<br>for( i = 0; i <= vui_mvc_num_ops_minus1; i++ ) {|||
|<br> <br>**vui_mvc_temporal_id[** i**] **|0|u(3)|
|<br> <br>**vui_mvc_num_target_output_views_minus1[** i**] **|5|ue(v)|
|<br> <br>for( j = 0; j <= vui_mvc_num_target_output_views_minus1[ i ]; j++ )|||
|<br> <br> <br>**vui_mvc_view_id[** i**][** j**] **|5|ue(v)|
|<br> <br>**vui_mvc_timing_info_present_flag[** i**]**|0|u(1)|
|<br> <br>if( vui_mvc_timing_info_present_flag[ i ] ) {|||
|<br> <br> <br>**vui_mvc_num_units_in_tick[** i**]**|0|u(32)|
|<br> <br> <br>**vui_mvc_time_scale[** i**]**|0|u(32)|
|<br> <br> <br>**vui_mvc_fixed_frame_rate_flag[** i**]**|0|u(1)|
|<br> <br>}|||
|<br> <br>**vui_mvc_nal_hrd_parameters_present_flag[** i**]**|0|u(1)|
|<br> <br>if( vui_mvc_nal_hrd_parameters_present_flag[ i ] )|||
|<br> <br> <br>hrd_parameters( )|0||
|<br> <br>**vui_mvc_vcl_hrd_parameters_present_flag[** i**]**|0|u(1)|
|<br> <br>if( vui_mvc_vcl_hrd_parameters_present_flag[ i ] )|||
|<br> <br> <br>hrd_parameters( )|0||
|<br> <br>if( vui_mvc_nal_hrd_parameters_present_flag[ i ] | |<br> <br> <br> <br>vui_mvc_vcl_hrd_parameters_present_flag[ i ] )|||
|<br> <br> <br>**vui_mvc_low_delay_hrd_flag[** i**]**|0|u(1)|






|vui_mvc_pic_struct_present_flag[ i ]|0|u(1)|
|---|---|---|
|<br>}|||
|}|||


**H.14.2** **MVC VUI parameters extension semantics**


The MVC VUI parameters extension specifies VUI parameters that apply to one or more operation points for the coded
video sequence. In Annex C it is specified which of the HRD parameter sets specified in the MVC VUI parameters extension are used
for conformance checking. All MVC VUI parameters extensions that are referred to by a coded video sequence shall be
identical.


Some views identified by vui_mvc_view_id[ i ][ j ] may not be present in the coded video sequence. Some temporal subsets
identified by vui_mvc_temporal_id[ i ] may not be present in the coded video sequence.


**vui_mvc_num_ops_minus1** plus 1 specifies the number of operation points for which timing information, NAL HRD
parameters, VCL HRD parameters, and the pic_struct_present_flag may be present. The value of
vui_mvc_num_ops_minus1 shall be in the range of 0 to 1023, inclusive.


**vui_mvc_temporal_id[** i **]** indicates the maximum value of temporal_id for all VCL NAL units in the representation of
the i-th operation point.


**vui_mvc_num_target_output_views_minus1[** i **]** plus one specifies the number of target output views for the i-th
operation point. The value of vui_mvc_num_target_output_views_minus1[ i ] shall be in the range of 0 to 1023, inclusive.


**vui_mvc_view_id[** i **][** j **]** indicates the j-th target output view in the i-th operation point. The value of
vui_mvc_view_id[ i ] shall be in the range of 0 to 1023, inclusive.


The following syntax elements apply to the coded video sequence that is obtained by the sub-bitstream extraction process
as specified in clause H.8.5.3 with tIdTarget equal to vui_mvc_temporal_id[ i ] and viewIdTargetList containing
vui_mvc_view_id[ i ][ j ] for all j in the range of 0 to vui_mvc_num_target_output_views_minus1[ i ], inclusive, as the
inputs and the i-th sub-bitstream as the output.


**vui_mvc_timing_info_present_flag[** i **]** equal to 1 specifies that vui_mvc_num_units_in_tick[ i ],
vui_mvc_time_scale[ i ], and vui_mvc_fixed_frame_rate_flag[ i ] for the i-th sub-bitstream are present in the MVC VUI
parameters extension. vui_mvc_timing_info_present_flag[ i ] equal to 0 specifies that vui_mvc_num_units_in_tick[ i ],
vui_mvc_time_scale[ i ], and vui_mvc_fixed_frame_rate_flag[ i ] for the i-th sub-bitstream are not present in the MVC
VUI parameters extension.


The following syntax elements for the i-th sub-bitstream are specified using references to Annex E. For these syntax
elements the same semantics and constraints as the ones specified in Annex E apply, as if these syntax elements
vui_mvc_num_units_in_tick[ i ], vui_mvc_time_scale[ i ], vui_mvc_fixed_frame_rate_flag[ i ],
vui_mvc_nal_hrd_parameters_present_flag[ i ], vui_mvc_vcl_hrd_parameters_present_flag[ i ],
vui_mvc_low_delay_hrd_flag[ i ], and vui_mvc_pic_struct_present_flag[ i ] were present as the syntax elements
num_units_in_tick, time_scale, fixed_frame_rate_flag, nal_hrd_parameters_present_flag,
vcl_hrd_parameters_present_flag, low_delay_hrd_flag, and pic_struct_present_flag, respectively, in the VUI parameters
of the active MVC sequence parameter sets for the i-th sub-bitstream.


**vui_mvc_num_units_in_tick[** i **]** specifies the value of num_units_in_tick, as specified in clause E.2.1, for the i-th subbitstream.


**vui_mvc_time_scale[** i **]** specifies the value of time_scale, as specified in clause E.2.1, for the i-th sub-bitstream.


**vui_mvc_fixed_frame_rate_flag[** i **]** specifies the value of fixed_frame_rate_flag, as specified in clause E.2.1, for the ith sub-bitstream.


**vui_mvc_nal_hrd_parameters_present_flag[** i **]** specifies the value of nal_hrd_parameters_present_flag, as specified in
clause E.2.1, for the i-th sub-bitstream.


When vui_mvc_nal_hrd_parameters_present_flag[ i ] is equal to 1, NAL HRD parameters (clauses E.1.2 and E.2.2) for
the i-th sub-bitstream immediately follow the flag.


The variable VuiMvcNalHrdBpPresentFlag[ i ] is derived as follows:


- If any of the following is true, the value of VuiMvcNalHrdBpPresentFlag[ i ] shall be set equal to 1:

   - vui_mvc_nal_hrd_parameters_present_flag[ i ] is present in the bitstream and is equal to 1,





   - for the i-th sub-bitstream, the need for presence of buffering periods for NAL HRD operation to be present in
the bitstream in buffering period SEI messages is determined by the application, by some means not specified in
this Recommendation | International Standard.


- Otherwise, the value of VuiMvcNalHrdBpPresentFlag[ i ] shall be set equal to 0.


**vui_mvc_vcl_hrd_parameters_present_flag[** i **]** specifies the value of vcl_hrd_parameters_present_flag, as specified in
clause E.2.1, for the i-th sub-bitstream.


When vui_mvc_vcl_hrd_parameters_present_flag[ i ] is equal to 1, VCL HRD parameters (clauses E.1.2 and E.2.2) for
the i-th sub-bitstream immediately follow the flag.


The variable VuiMvcVclHrdBpPresentFlag[ i ] is derived as follows:


- If any of the following is true, the value of VuiMvcVclHrdBpPresentFlag[ i ] shall be set equal to 1:

   - vui_mvc_vcl_hrd_parameters_present_flag[ i ] is present in the bitstream and is equal to 1,

   - for the i-th sub-bitstream, the need for presence of buffering period parameters for VCL HRD operation in the
bitstream in buffering period SEI messages is determined by the application, by some means not specified in this
Recommendation | International Standard.


- Otherwise, the value of VuiMvcVclHrdBpPresentFlag[ i ] shall be set equal to 0.


The variable VuiMvcCpbDpbDelaysPresentFlag[ i ] is derived as follows:


- If any of the following is true, the value of VuiMvcCpbDpbDelaysPresentFlag[ i ] shall be set equal to 1:

   - vui_mvc_nal_hrd_parameters_present_flag[ i ] is present in the bitstream and is equal to 1,

   - vui_mvc_vcl_hrd_parameters_present_flag[ i ] is present in the bitstream and is equal to 1,

   - for the i-th sub-bitstream, the need for presence of CPB and DPB output delays in the bitstream in picture timing
SEI messages is determined by the application, by some means not specified in this Recommendation |
International Standard.


- Otherwise, the value of VuiMvcCpbDpbDelaysPresentFlag[ i ] shall be set equal to 0.


**vui_mvc_low_delay_hrd_flag[** i **]** specifies the value of low_delay_hrd_flag, as specified in clause E.2.1, for the i-th subbitstream.


**vui_mvc_pic_struct_present_flag[** i **]** specifies the value of pic_struct_present_flag, as specified in clause E.2.1, for the
i-th sub-bitstream.
