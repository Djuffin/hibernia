**J.9** **Parsing process**


The specifications in clause 9 apply. Additionally, the following modifications are specified and added.


**J.9.1** **Alternative CABAC parsing process for slice data and macroblock layer in depth extension**


Clause J.9.1.1 specifies the initialization process for the alternative CABAC parsing process for slice data and macroblock
layer when nal_unit_type is equal to 21 and avc_3d_extension_flag is equal to 1.


Clause J.9.1.2 specifies the binarization process for the alternative CABAC parsing process for slice data and macroblock
layer when nal_unit_type is equal to 21 and avc_3d_extension_flag is equal to 1


Clause J.9.1.3 specifies the decoding process flow for the alternative CABAC parsing process for slice data and
macroblock layer when nal_unit_type is equal to 21 and avc_3d_extension_flag is equal to 1.


**J.9.1.1** **Initialization process**


Outputs of this process are the initialized CABAC context variables indexed by ctxIdx.


Table J-6 contains the values of the variables n and m used in the initialization of context variables that are assigned to
syntax elements mb_vsskip_flag, and mb_direct_type_flag. Table J-7 contains the values of the variables n and m used in
the initialization of context variables that are assigned to syntax element mb_skip_run_type, mb_alc_skip_flag,
mb_alc_flag and mb_vsp_flag. The initialization proces for two variables pStateIdx and valMPS is the same as other syntax
elements, as defined in Equation 9-5. For all other syntax elements in clause 7.3.5 the initialization process of context
variables as specified in clause 9.3.1 applies.


**Table J-5 – Association of ctxIdx and syntax elements in the initialization process**

|Syntax element|Table|Slice type|Col4|Col5|
|---|---|---|---|---|
|**Syntax element**|**Table**|**I **|**P **|**B **|
|mb_vsskip_flag|Table J-6||1031..1033|1034..1036|
|mb_direct_type_flag|Table J-6|||1037..1039|






|mb_skip_run_type|Table J-7|Col3|Col4|1040|
|---|---|---|---|---|
|mb_alc_skip_flag|Table J-7||1041 .. 1043||
|mb_alc_flag|Table J-7||1044 .. 1046||
|mb_vsp_flag|Table J-7|<br>|1047..1049|1050..1052|


**Table J-6 – Values of variables m and n for ctxIdx from 1031 to 1039**








|Value of<br>cabac_init_idc|Initializatio<br>n variables|ctxIdx|Col4|Col5|Col6|Col7|Col8|Col9|Col10|Col11|
|---|---|---|---|---|---|---|---|---|---|---|
|**Value of**<br>**cabac_init_idc**|**Initializatio**<br>**n variables**|**1031**|**1032**|**1033**|**1034**|**1035**|**1036**|**1037**|**1038**|**1039**|
|**0 **|**m **|23|23|21|18|9|29|−46|−20|1|
|**0 **|**n **|33|2|0|64|43|0|127|104|67|
|**1 **|**m **|22|34|16|26|19|40|−45|−15|−4|
|**1 **|**n **|25|0|0|34|22|0|127|101|76|
|**2 **|**m **|29|25|14|20|20|29|−32|−22|−2|
|**2 **|**n **|16|0|0|40|10|0|127|−117|74|







|Col1|Table J-7 – Values of variables m and n for ctxIdx from 1040 to 1052|Col3|Col4|Col5|Col6|Col7|Col8|Col9|Col10|Col11|Col12|Col13|Col14|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|**Initialization**<br>**variables**|**ctxIdx**|**ctxIdx**|**ctxIdx**|**ctxIdx**|**ctxIdx**|**ctxIdx**|**ctxIdx**|**ctxIdx**|**ctxIdx**|**ctxIdx**|**ctxIdx**|**ctxIdx**|**ctxIdx**|
|**Initialization**<br>**variables**|**1040**|**1041**|**1042**|**1043**|**1044**|**1045**|**1046**|**1047**|**1048**|**1049**|**1050**|**1051**|**1052**|
|**m **|18|14|14|10|14|14|10|0|0|0|0|0|0|
|**n **|64|4|27|52|4|27|52|64|64|64|64|64|64|


**J.9.1.2** **Binarization process**


Input to this process is a request for a syntax element.


Output of this process is the binarization of the syntax element, maxBinIdxCtx, ctxIdxOffset, and bypassFlag.


Associated with each binarization or binarization part of a syntax element is a specific value of the context index offset
(ctxIdxOffset) variable and a specific value of the maxBinIdxCtx variable as given in Table J-8.


The use of the DecodeBypass process and the variable bypassFlag is derived as follows:


- If no value is assigned to ctxIdxOffset for the corresponding binarization or binarization part in Table J-8 labelled as
"na", all bins of the bit strings of the corresponding binarization or of the binarization prefix/suffix part are decoded
by invoking the DecodeBypass process as specified in clause 9.3.3.2.3. In such a case, bypassFlag is set equal to 1,
where bypassFlag is used to indicate that for parsing the value of the bin from the bitstream the DecodeBypass process
is applied.


- Otherwise, for each possible value of binIdx up to the specified value of maxBinIdxCtx given in Table J-8, a specific
value of the variable ctxIdx is further specified in clause 9.3.3. bypassFlag is set equal to 0.


The possible values of the context index ctxIdx are in the range 1031 to 1052, inclusive. The value assigned to ctxIdxOffset
specifies the lower value of the range of ctxIdx assigned to the corresponding binarization or binarization part of a syntax
element.





**Table J-8 – Syntax elements and associated types of binarization, maxBinIdxCtx, and ctxIdxOffset**

|Syntax element|Type of binarization|maxBinIdxCtx|ctxIdxOffset|
|---|---|---|---|
|mb_vsskip_flag (P slices only)|FL, cMax=1|0|1031|
|mb_vsskip_flag (B slices only)|FL, cMax=1|0|1034|
|mb_direct_type_flag (B slices only)|FL, cMax=1|0|1037|
|mb_skip_run_type (B slices only)|FL, cMax=1|0|1040|
|mb_alc_skip_flag|FL, cMax=1|0|1041|
|mb_alc_flag|FL, cMax=1|0|1044|
|mb_vsp_flag (P slices only)|FL, cMax=1|0|1047|
|mb_vsp_flag (B slices only)|FL, cMax=1|0|1050|



**J.9.1.3** **Decoding process flow**


Input to this process is a binarization of the requested syntax element, maxBinIdxCtx, bypassFlag and ctxIdxOffset as
specified in clause 9.1.2.


Output of this process is the value of the syntax element.


This process specifies how each bit of a bit string is parsed for each syntax element.


After parsing each bit, the resulting bit string is compared to all bin strings of the binarization of the syntax element and
the following applies.


- If the bit string is equal to one of the bin strings, the corresponding value of the syntax element is the output.


- Otherwise (the bit string is not equal to one of the bin strings), the next bit is parsed.


While parsing each bin, the variable binIdx is incremented by 1 starting with binIdx being set equal to 0 for the first bin.


The parsing of each bin is specified by the following two ordered steps:


1. Given binIdx, maxBinIdxCtx and ctxIdxOffset, ctxIdx is derived as specified in clause J.9.1.3.1.


2. Given ctxIdx, the value of the bin from the bitstream as specified in clause 9.3.3.2 is decoded.


**J.9.1.3.1** **Derivation process for ctxIdx**


Inputs to this process are binIdx, maxBinIdxCtx and ctxIdxOffset.


Output of this process is ctxIdx.


Table J-9 shows the assignment of ctxIdx increments (ctxIdxInc) to binIdx for all ctxIdxOffset values for the syntax
mb_vsskip_flag, mb_direct_type_flag, mb_alc_skip_flag, mb_alc_flag and mb_vsp_flag.


The ctxIdx to be used with a specific binIdx is the sum of ctxIdxOffset and ctxIdxInc, which is found in Table J-9. When
more than one value is listed in Table J-9 or 9-39 for a binIdx, the assignment process for ctxIdxInc for that binIdx is
further specified in the clauses given in parenthesis of the corresponding table entry.


All entries in Table J-9 labelled with "na" correspond to values of binIdx that do not occur for the corresponding
ctxIdxOffset.





**Table J-9 – Assignment of ctxIdxInc to binIdx for the ctxIdxOffset values related to the syntax elements**

**mb_vsskip_flag, mb_direct_type_flag, mb_alc_skip_flag, mb_alc_flag and mb_vsp_flag**







|ctxIdxOffset|binIdx|Col3|
|---|---|---|
|**ctxIdxOffset**|**0 **|**>= 1**|
|**1031**|0,1,2<br>(clause J.9.1.3.2)|na|
|**1034**|0,1,2<br>(clause J.9.1.3.2)|na|
|**1037**|0,1,2<br>(clause J.9.1.3.3)|na|
|**1041**|0,1,2<br>(clause J.9.1.3.4)|na|
|**1044**|0,1,2<br>(clause J.9.1.3.5)|na|
|**1047**|0,1,2<br>(clause J.9.1.3.6)|na|
|**1050**|0,1,2<br>(clause J.9.1.3.6)|na|


**J.9.1.3.2** **Derivation process of ctxIdxInc for the syntax element mb_vsskip_flag**


Output of this process is ctxIdxInc.


The derivation process for neighbouring macroblocks specified in clause 6.4.11.1 is invoked and the output is assigned to
mbAddrA and mbAddrB.


Let the variable condTermFlagN (with N being either A or B) be derived as follows:


- If mbAddrN is not available or mb_vsskip_flag for the macroblock mbAddrN is equal to 1, condTermFlagN is set
equal to 0.


- Otherwise (mbAddrN is available and mb_vsskip_flag for the macroblock mbAddrN is equal to 0), condTermFlagN
is set equal to 1.


The variable ctxIdxInc is derived by:


ctxIdxInc = condTermFlagA + condTermFlagB (J-81)


**J.9.1.3.3** **Derivation process of ctxIdxInc for the syntax element mb_direct_type_flag**


Output of this process is ctxIdxInc.


The derivation process for neighbouring macroblocks specified in clause 6.4.11.1 is invoked and the output is assigned to
mbAddrA and mbAddrB.


Let the variable condTermFlagN (with N being either A or B) be derived as follows:


- If any of the following conditions is true, condTermFlagN is set to 0:


   - If mbAddrN is not available,


   - Both mb_vsskip_flag and mb_direct_type_flag for the macroblock mbAddrN is equal to 0


- Otherwise, condTermFlagN is set equal to 1.


The variable ctxIdxInc is derived by:


ctxIdxInc = condTermFlagA + condTermFlagB (J-82)





**J.9.1.3.4** **Derivation process of ctxIdxInc for the syntax element mb_alc_skip_flag**


Output of this process is ctxIdxInc.


The derivation process for neighbouring macroblocks specified in clause 6.4.11.1 is invoked and the output is assigned to
mbAddrA and mbAddrB.


Let the variable condTermFlagN (with N being either A or B) be derived as follows:


- If mbAddrN is available and mb_alc_skip_flag or mb_alc_flag for the macroblock mbAddrN is equal to 1,
condTermFlagN is set equal to 1.


- Otherwise (mbAddrN is not available or mb_alc_skip_flag and mb_alc_flag for the macroblock mbAddrN is equal to
0), condTermFlagN is set equal to 0.


The variable ctxIdxInc is derived by


ctxIdxInc = condTermFlagA + condTermFlagB (J-83)


**J.9.1.3.5** **Derivation process of ctxIdxInc for the syntax element mb_alc_flag**


Output of this process is ctxIdxInc.


The derivation process for neighbouring macroblocks specified in clause 6.4.11.1 is invoked and the output is assigned to
mbAddrA and mbAddrB.


Let the variable condTermFlagN (with N being either A or B) be derived as follows:


- If mbAddrN is available and mb_alc_skip_flag or mb_alc_flag for the macroblock mbAddrN is equal to 1,
condTermFlagN is set equal to 1.


- Otherwise (mbAddrN is not available or mb_alc_skip_flag and mb_alc_flag for the macroblock mbAddrN is equal
to 0), condTermFlagN is set equal to 0.


The variable ctxIdxInc is derived by:


ctxIdxInc = condTermFlagA + condTermFlagB (J-84)


**J.9.1.3.6** **Derivation process of ctxIdxInc for the syntax element mb_vsp_flag**


Output of this process is ctxIdxInc.


The derivation process for neighbouring macroblocks specified in clause 6.4.11.1 is invoked and the output is assigned to
mbAddrA and mbAddrB.


Let the variable condTermFlagN (with N being either A or B) be derived as follows:


- If mbAddrN is not available or mb_vsp_flag for the macroblock mbAddrN is equal to 0, condTermFlagN is set equal
to 1.


- Otherwise (mbAddrN is available and mb_vsp_flag for the macroblock mbAddrN is equal to 1), condTermFlagN is
set equal to 0.


The variable ctxIdxInc is derived by:


ctxIdxInc = condTermFlagA + condTermFlagB (J-85)
