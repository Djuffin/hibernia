**J.10** **Profiles and levels**


The specifications in Annex I apply. Additional profiles and specific values of profile_idc are specified in the following.


The profiles that are specified in clause J.10.1 are also referred to as the profiles specified in Annex J.


**J.10.1** **Profiles**


All constraints for picture parameter sets that are specified in the following are constraints for picture parameter sets that
become the active picture parameter set or an active view picture parameter set inside the bitstream. All constraints for 3DAVC sequence parameter sets that are specified in the following are constraints for 3D-AVC sequence parameter sets that
become the active 3D-AVC sequence parameter set or an active view 3D-AVC sequence parameter set inside the bitstream.





**J.10.1.1** **Enhanced Multiview Depth High profile**


Bitstreams conforming to the Enhanced Multiview Depth High profile shall obey the following constraints:


- The base view bitstream as specified in clause I.8.5.3 shall obey all constraints of the High profile specified in
clause A.2.4 and all active sequence parameter sets shall fulfil one of the following conditions:

   - profile_idc is equal to 77 or constraint_set1_flag is equal to 1,

   - profile_idc is equal to 100.


- Only I, P, and B slice types may be present.


- NAL unit streams shall not contain nal_unit_type values in the range of 2 to 4, inclusive.


- Arbitrary slice order is not allowed.


- Picture parameter sets shall have num_slice_groups_minus1 equal to 0 only.


- Picture parameter sets shall have redundant_pic_cnt_present_flag equal to 0 only.


- When the profile_idc is equal to 139 in a sequence parameter set, the frame_mbs_only_flag in the sequence parameter
set shall be equal to 1.


- 3D-AVC sequence parameter sets shall have chroma_format_idc equal to 0 only.


- 3D-AVC sequence parameter sets shall have bit_depth_luma_minus8 equal to 0 only.


- 3D-AVC sequence parameter sets shall have bit_depth_chroma_minus8 equal to 0 only.


- 3D-AVC sequence parameter sets shall have qpprime_y_zero_transform_bypass_flag equal to 0 only.


- For each access unit, the value of level_idc for all active view 3D-AVC sequence parameter set RBSPs shall be the
same as the value of level_idc for the active 3D-AVC sequence parameter set RBSP.


- The level constraints specified for the Enhanced Multiview Depth High profile in clause I.10.2 shall be fulfilled.


Conformance of a bitstream to the Enhanced Multiview Depth High profile is indicated by profile_idc being equal to 139.


Decoders conforming to the Enhanced Multiview Depth High profile at a specific level shall be capable of decoding all
bitstreams in which both of the following conditions are true:

a) All active 3D-AVC sequence parameter sets have one or more of the following conditions fulfilled:


   - profile_idc is equal to 139,


   - profile_idc is equal to 138,


   - profile_idc is equal to 128,


   - profile_idc is equal to 118 and constraint_set5_flag is equal to 1,


   - profile_idc is equal to 100,


   - profile_idc is equal to 77 or constraint_set1_flag is equal to 1.

b) All active 3D-AVC sequence parameter sets have one or more of the following conditions fulfilled:


   - level_idc or (level_idc and constraint_set3_flag) represent a level less than or equal to the specific level,


   - level_idc[ i ] or (level_idc[ i ] and constraint_set3_flag) represent a level less than or equal to the specific level.


**J.10.2** **Levels**


The specification of clause I.10.2 applies.


**J.10.2.1** **Level limits for Enhanced Multiview Depth High profile**


The specification of clause I.10.2.1 applies.


**J.10.2.2** **Profile specific level limits**


a) In bitstreams conforming to the Enhanced Multiview Depth High profile, 3D-AVC sequence parameter sets shall

have frame_mbs_only_flag equal to 1 for the levels specified in Table A-4.
