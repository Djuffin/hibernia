**G.14** **Video usability information**


The specifications in Annex E apply with substituting SVC sequence parameter set for sequence parameter set. The VUI
parameters and the constraints specified in Annex E apply to coded video sequences for which the SVC sequence parameter
set becomes the active SVC sequence parameter set.


Additionally, the following applies.


**G.14.1** **SVC VUI parameters extension syntax**

|svc_vui_parameters_extension( ) {|C|Descriptor|
|---|---|---|
|<br>**vui_ext_num_entries_minus1**|0|ue(v)|
|<br>for( i = 0; i <= vui_ext_num_entries_minus1; i++ ) {|||
|<br> <br>**vui_ext_dependency_id[** i**] **|0|u(3)|
|<br> <br>**vui_ext_quality_id[** i**] **|0|u(4)|
|<br> <br>**vui_ext_temporal_id[** i**] **|0|u(3)|
|<br> <br>**vui_ext_timing_info_present_flag[** i**]**|0|u(1)|
|<br> <br>if( vui_ext_timing_info_present_flag[ i ] ) {|||
|<br> <br> <br>**vui_ext_num_units_in_tick[** i**]**|0|u(32)|
|<br> <br> <br>**vui_ext_time_scale[** i**]**|0|u(32)|
|<br> <br> <br>**vui_ext_fixed_frame_rate_flag[** i**]**|0|u(1)|
|<br> <br>}|||
|<br> <br>**vui_ext_nal_hrd_parameters_present_flag[** i**]**|0|u(1)|
|<br> <br>if( vui_ext_nal_hrd_parameters_present_flag[ i ] )|||
|<br> <br> <br>hrd_parameters( )|0||
|<br> <br>**vui_ext_vcl_hrd_parameters_present_flag[** i**]**|0|u(1)|
|<br> <br>if( vui_ext_vcl_hrd_parameters_present_flag[ i ] )|||
|<br> <br> <br>hrd_parameters( )|0||
|<br> <br>if( vui_ext_nal_hrd_parameters_present_flag[ i ] | |<br> <br> <br> <br>vui_ext_vcl_hrd_parameters_present_flag[ i ] )|||
|<br> <br> <br>**vui_ext_low_delay_hrd_flag[** i**]**|0|u(1)|
|<br> <br>**vui_ext_pic_struct_present_flag[** i**]**|0|u(1)|
|<br>}|||
|}|||



**G.14.2** **SVC VUI parameters extension semantics**


The SVC VUI parameters extension specifies timing information, HRD parameter sets, and the presence of picture
structure information for subsets of coded video sequences (including the complete coded video sequences) conforming
one or more of the profiles specified in Annex G. In Annex C it is specified which of the HRD parameter sets specified in
the SVC VUI parameters extension are used for conformance checking.


**vui_ext_num_entries_minus1** plus 1 specifies the number of information entries that are present in the SVC VUI
parameters extension syntax structure. The value of vui_ext_num_entries_minus1 shall be in the range of 0 to 1023,
inclusive. Each information entry is associated with particular values of temporal_id, dependency_id, and quality_id and
may indicate timing information, NAL HRD parameters, VCL HRD parameters, and the presence of picture structure
information for a particular subset of coded video sequences as specified in the following.





**vui_ext_dependency_id** [ i ] and **vui_ext_quality_id** [ i ] indicate the maximum value of DQId for the i-th subset of coded
video sequences. The maximum value of DQId for the i-th subset of coded video sequences is derived by
vui_ext_dependency_id[ i ] + ( vui_ext_quality_id[ i ] << 4 ).


**vui_ext_temporal_id** [ i ] indicates the maximum value of temporal_id for the i-th subset of coded video sequences.


The SVC VUI parameters extension syntax structure shall not contain two or more information entries with identical values
of vui_ext_dependency_id[ i ], vui_ext_quality_id[ i ], and vui_ext_temporal_id[ i ].


The following syntax elements apply to the coded video sequences that are obtained by the invoking the sub-bitstream
extraction process as specified in clause G.8.8.1 with tIdTarget equal to vui_ext_temporal_id[ i ], dIdTarget equal to
vui_ext_dependency_id[ i ], and qIdTarget equal to vui_ext_quality_id[ i ] as the inputs and the i-th subset of coded video
sequences as the output.


**vui_ext_timing_info_present_flag** [ i ] equal to 1 specifies that vui_ext_num_units_in_tick[ i ], vui_ext_time_scale[ i ],
and vui_ext_fixed_frame_rate_flag[ i ] for the i-th subset of coded video sequences are present in the SVC VUI parameters
extension. vui_ext_timing_info_present_flag[ i ] equal to 0 specifies that vui_ext_num_units_in_tick[ i ],
vui_ext_time_scale[ i ], and vui_ext_fixed_frame_rate_flag[ i ] for the i-th subset of coded video sequences are not present
in the SVC VUI parameters extension.


The following syntax elements for the i-th subset of coded video sequences are specified using references to Annex E. For
these syntax elements the same semantics and constraints as the ones specified in Annex E apply, as if these syntax
elements vui_ext_num_units_in_tick[ i ], vui_ext_time_scale[ i ], vui_ext_fixed_frame_rate_flag[ i ],
vui_ext_nal_hrd_parameters_present_flag[ i ], vui_ext_vcl_hrd_parameters_present_flag[ i ],
vui_ext_low_delay_hrd_flag[ i ], and vui_ext_pic_struct_present_flag[ i ] were present as the syntax elements
num_units_in_tick, time_scale, fixed_frame_rate_flag, nal_hrd_parameters_present_flag,
vcl_hrd_parameters_present_flag, low_delay_hrd_flag, and pic_struct_present_flag, respectively, in the VUI parameters
of the active SVC sequence parameter sets for the i-th subset of coded video sequences.


**vui_ext_num_units_in_tick** [ i ] specifies the value of num_units_in_tick, as specified in clause E.2.1, for the i-th subset
of coded video sequences.


**vui_ext_time_scale** [ i ] specifies the value of time_scale, as specified in clause E.2.1, for the i-th subset of coded video
sequences.


**vui_ext_fixed_frame_rate_flag** [ i ] specifies the value of fixed_frame_rate_flag, as specified in clause E.2.1, for the i-th
subset of coded video sequences.


**vui_ext_nal_hrd_parameters_present_flag** [ i ] specifies the value of nal_hrd_parameters_present_flag, as specified in
clause E.2.1, for the i-th subset of coded video sequences.


When vui_ext_nal_hrd_parameters_present_flag[ i ] is equal to 1, NAL HRD parameters (clauses E.1.2 and E.2.2) for the
i-th subset of coded video sequences immediately follow the flag.


The variable VuiExtNalHrdBpPresentFlag[ i ] is derived as follows:


- If any of the following is true, the value of VuiExtNalHrdBpPresentFlag[ i ] shall be set equal to 1:

   - vui_ext_nal_hrd_parameters_present_flag[ i ] is present in the bitstream and is equal to 1,

   - for the i-th subset of coded video sequences, the need for presence of buffering periods for NAL HRD operation
to be present in the bitstream in buffering period SEI messages is determined by the application, by some means
not specified in this Recommendation | International Standard.


- Otherwise, the value of VuiExtNalHrdBpPresentFlag[ i ] shall be set equal to 0.


**vui_ext_vcl_hrd_parameters_present_flag** [ i ] specifies the value of vcl_hrd_parameters_present_flag, as specified in
clause E.2.1, for the i-th subset of coded video sequences.


When vui_ext_vcl_hrd_parameters_present_flag[ i ] is equal to 1, VCL HRD parameters (clauses E.1.2 and E.2.2) for **the**
**i-th subset of coded video sequences immediately follow the flag.**


The variable VuiExtVclHrdBpPresentFlag[ i ] is derived as follows:


- If any of the following is true, the value of VuiExtVclHrdBpPresentFlag[ i ] shall be set equal to 1:

   - vui_ext_vcl_hrd_parameters_present_flag[ i ] is present in the bitstream and is equal to 1,

   - for the i-th subset of coded video sequences, the need for presence of buffering period parameters for VCL HRD
operation in the bitstream in buffering period SEI messages is determined by the application, by some means not
specified in this Recommendation | International Standard.


- Otherwise, the value of VuiExtVclHrdBpPresentFlag[ i ] shall be set equal to 0.





The variable VuiExtCpbDpbDelaysPresentFlag[ i ] is derived as follows:


- If any of the following is true, the value of VuiExtCpbDpbDelaysPresentFlag[ i ] shall be set equal to 1:

   - vui_ext_nal_hrd_parameters_present_flag[ i ] is present in the bitstream and is equal to 1,

   - vui_ext_vcl_hrd_parameters_present_flag[ i ] is present in the bitstream and is equal to 1,

   - for the i-th subset of coded video sequences, the need for presence of CPB and DPB output delays in the bitstream
in picture timing SEI messages is determined by the application, by some means not specified in this
Recommendation | International Standard.


- Otherwise, the value of VuiExtCpbDpbDelaysPresentFlag[ i ] shall be set equal to 0.


**vui_ext_low_delay_hrd_flag** [ i ] specifies the value of low_delay_hrd_flag, as specified in clause E.2.1, for the i-th subset
of coded video sequences.


**vui_ext_pic_struct_present_flag** [ i ] specifies the value of pic_struct_present_flag, as specified in clause E.2.1, for the ith subset of coded video sequences.
