**G.9** **Parsing process**


Inputs to this process are bits from the RBSP, a request for a value of a syntax element, and values of prior parsed syntax
elements (if applicable).


Output of this process is the value of the syntax element.


This process is invoked for all syntax elements in the syntax tables in clause G.7.3 with descriptors equal to u(v), ue(v),
me(v), se(v), te(v), ce(v), and ae(v).


When the parsing process is invoked for the first request for a value of a syntax element in the slice data and
entropy_coding_mode_flag is equal to 1, the following ordered steps apply:


1. The initialization process as specified in clause 9.3.1 is invoked, where a slice_type equal to EI is interpreted as

I, a slice_type equal to EP is interpreted as P, and a slice_type equal to EB is interpreted as B.


2. The initialization process as specified in clause G.9.3.1 is invoked.


Depending on entropy_coding_mode_flag and the descriptor, the value of a syntax element is derived as follows:


- If entropy_coding_mode_flag is equal to 0, the following applies:


1. The parsing process for syntax elements coded as coded as ue(v), se(v), or te(v) is specified in clause 9.1.


2. The parsing process for the syntax element coded_block_pattern is specified in clause G.9.1.


3. The parsing process for syntax elements of the residual_block_cavlc( ) syntax structure is specified in

clause G.9.2.


- Otherwise (entropy_coding_mode_flag is equal to 1), the value of the syntax element is derived as follows:


   - If the syntax element is equal to base_mode_flag, motion_prediction_flag_l0, motion_prediction_flag_l1, or
residual_prediction_flag, the following applies:


1. The binarization process as specified in clause G.9.3.2 is invoked.


2. The decoding process flow as specified in clause G.9.3.3 is invoked.


   - Otherwise (the syntax element is not equal to base_mode_flag, motion_prediction_flag_l0,
motion_prediction_flag_l1, or residual_prediction_flag), the following applies:


1. The binarization process as specified in clause 9.3.2 is invoked, where a slice_type equal to EI is

interpreted as I, a slice_type equal to EP is interpreted as P, and a slice_type equal to EB is interpreted
as B.


2. The decoding process flow as specified in clause 9.3.3 is invoked.

NOTE – For macroblocks with base_mode_flag equal to 1, mb_type is inferred to be equal to Mb_Inferred and
the specifications in clause G.7.4.6 apply.


3. When the syntax element is equal to mb_type and the decoded value of mb_type is equal to I_PCM,

the arithmetic decoding engine is initialized after decoding of any pcm_alignment_zero_bit and all
pcm_sample_luma and pcm_sample_chroma data as specified in clause 9.3.1.2.


**G.9.1** **Alternative parsing process for coded block pattern**


This process is invoked for the parsing syntax elements with descriptor equal to me(v) when entropy_coding_mode_flag
is equal to 0.


Inputs to this process are bits from the RBSP.


Outputs of this process is a value of the syntax element coded_block_pattern.


The parsing process for the syntax elements begins with reading the bits starting at the current location in the bitstream up
to and including the first non-zero bit. By counting the number of leading bits that are equal to 0 and assigning this value
to the variable leadingZeroBits, the variable codeNum is then derived as

codeNum = 2 [leadingZeroBits]     - 1 + read_bits( leadingZeroBits )


where the value returned from read_bits( leadingZeroBits ) is interpreted as a binary representation of an unsigned integer
with most significant bit written first.


When ref_layer_dq_id is greater than or equal to 0 and ( scan_idx_end − scan_idx_start ) is less than 15, codeNum is set
equal to (codeNum − 1).


Depending on codeNum, the following applies:





- If codeNum is equal to −1, the following ordered steps are specified:


1. The derivation process for neighbouring macroblocks specified in clause 6.4.11.1 is invoked and the output is

assigned to mbAddrA and mbAddrB.


2. When mbAddrN is available, the variable codedBlockPatternN (with N being either A or B) is derived as follows:


     - If mb_type for the macroblock mbAddrN is equal to P_Skip, B_Skip, or I_PCM, codedBlockPatternN is set
equal to 0.


     - Otherwise (mb_type for the macroblock mbAddrN is not equal to P_Skip, B_Skip, or I_PCM),
codedBlockPatternN is set equal to (16 * cbpChromaN + cbpLumaN) with cbpChromaN and cbpLumaN
representing the values of CodedBlockPatternLuma and CodedBlockPatternChroma for the macroblock
mbAddrN.


3. Depending on mbAddrA and mbAddrB, the following applies:


     - If mbAddrA is available, coded_block_pattern is set equal to codedBlockPatternA.


     - Otherwise, if mbAddrB is available, coded_block_pattern is set equal to codedBlockPatternB.


     - Otherwise (mbAddrA and mbAddrB are not available), coded_block_pattern is set equal to 0.


- Otherwise (codeNum is greater than or equal to 0), the mapping process for coded block pattern as specified in
clause 9.1.2 is invoked with codeNum as input and the output is assigned to the syntax element coded_block_pattern.


**G.9.2** **Alternative CAVLC parsing process for transform coefficient levels**


This process is invoked for the parsing syntax elements with descriptor equal to ce(v) when entropy_coding_mode_flag is
equal to 0.


Inputs to this process are a request for a value of a syntax element, bits from slice data, a maximum number of non-zero
transform coefficient levels maxNumCoeff, the luma block index luma4x4BlkIdx or the chroma block index
chroma4x4BlkIdx, cb4x4BlkIdx or cr4x4BlkIdx of the current block of transform coefficient levels.


Output of this process is the list coeffLevel containing transform coefficient levels of the luma block with block index
luma4x4BlkIdx or the chroma block with block index chroma4x4BlkIdx, cb4x4BlkIdx or cr4x4BlkIdx.


The process is specified in the following ordered steps:


1. All transform coefficient levels, with indices from 0 to maxNumCoeff − 1, in the list coeffLevel are set equal to 0.


2. The total number of non-zero transform coefficient levels TotalCoeff( coeff_token ) and the number of trailing

one transform coefficient levels TrailingOnes( coeff_token ) are derived by parsing coeff_token as specified by
the following ordered steps:


a. The parsing process of coeff_token as specified in clause 9.2.1 is invoked and the outputs are
TotalCoeff( coeff_token ), TrailingOnes( coeff_token ), and nC.

NOTE – For macroblocks with base_mode_flag equal to 1, mb_type is inferred to be equal to Mb_Inferred and
the specifications in clause G.7.4.6 apply.


b. When the CAVLC parsing process is invoked for LumaLevel4x4, LumaLevel8x8, Intra16x16ACLevel,

ChromaACLevel, CbIntra16x16ACLevel, or CrIntra16x16ACLevel and ( scan_idx_end − scan_idx_start ) is
less than 15, nC is modified by setting it equal to Min( 7, nC ), and the additional parsing process for total
number of non-zero transform coefficient levels and number of trailing ones as specified in clause G.9.2.1 is
invoked with nC, totalCoeffStart set equal to TotalCoeff( coeff_token ), and trailingOnesStart set equal to
TrailingOnes( coeff_token ) as the inputs and the outputs are assigned to TotalCoeff( coeff_token ) and
TrailingOnes( coeff_token ).


3. When TotalCoeff( coeff_token ) is greater than 0, the following ordered steps are specified:


a. The non-zero transform coefficient levels are derived by parsing trailing_ones_sign_flag, level_prefix, and
level_suffix as specified in clause 9.2.2.


b. The runs of zero transform coefficient levels before each non-zero transform coefficient level are derived by

parsing total_zeros and run_before as specified in clause G.9.2.2.


c. The level and run information are combined into the list coeffLevel as specified in clause 9.2.4.


**G.9.2.1** **Additional parsing process for total number of non-zero transform coefficient levels and number of trailing**

**ones**


Inputs to this process are variables nC, totalCoeffStart, and trailingOnesStart.





Outputs of this process are variables totalCoeff and trailingOnes.


Let invTotalCoeff( coeffTokenIdx ) and invTrailingOnes( coeffTokenIdx) be functions that map the variable
coeffTokenIdx to the variables nX and nY, respectively, as specified in Table G-10 for each value of the variable nC.


A variable dX is set equal to (scan_idx_end − scan_idx_start + 2). A variable dY is set equal to
Min( 4, scan_idx_end − scan_idx_start + 2 ). A variable targetCoeffTokenIdx is derived as specified by Table G-10 given
the variables nC, nX = totalCoeffStart, and nY = trailingOnesStart.


The bitstream shall not contain data that result in a value of targetCoeffTokenIdx that exceeds the range of values from 0
to ( dX * dY − Min( 7, ( 1 << ( dY − 1 ) ) ) ), inclusive.


A variable coeffTokenIdx is derived as specified by the following pseudo code:

for( coeffTokenIdx = 0, i = 0; i <= targetCoeffTokenIdx; coeffTokenIdx++ )
if( invTotalCoeff( coeffTokenIdx ) < dX && invTrailingOnes( coeffTokenIdx ) < dY ) (G-359)
i++


The variable totalCoeff is set equal to invTotalCoeff( coeffTokenIdx − 1 ) and the variable trailingOnes is set equal to
invTrailingOnes( coeffTokenIdx − 1 ).


When the CAVLC parsing process is invoked for Intra16x16ACLevel, CbIntra16x16ACLevel, CrIntra16x16ACLevel, or
ChromaACLevel, it is a requirement of bitstream conformance that the bitstream shall not contain data that result in
totalCoeff being greater than (scan_idx_end − Max( 1, scan_idx_start ) + 1).


**Table G-10 – Mapping of ( nX, nY ) to coeffTokenIdx and vice versa**

|nY|nX|0 <= nC < 2|2 <= nC < 4|4 <= nC < 8|
|---|---|---|---|---|
|0|0|0|0|0|
|0|1|4|7|16|
|1|1|1|1|1|
|0|2|9|11|20|
|1|2|5|5|8|
|2|2|2|2|2|
|0|3|13|15|23|
|1|3|10|8|11|
|2|3|7|9|9|
|3|3|3|3|3|
|0|4|17|19|24|
|1|4|14|12|13|
|2|4|11|13|12|
|3|4|6|4|4|
|0|5|21|22|28|
|1|5|18|16|15|
|2|5|15|17|14|
|3|5|8|6|5|
|0|6|25|23|30|
|1|6|22|20|17|
|2|6|19|21|18|
|3|6|12|10|6|
|0|7|29|27|31|
|1|7|26|24|21|






**Table G-10 – Mapping of ( nX, nY ) to coeffTokenIdx and vice versa**

|nY|nX|0 <= nC < 2|2 <= nC < 4|4 <= nC < 8|
|---|---|---|---|---|
|2|7|23|25|22|
|3|7|16|14|7|
|0|8|32|31|32|
|1|8|30|28|25|
|2|8|27|29|26|
|3|8|20|18|10|
|0|9|33|35|36|
|1|9|34|32|33|
|2|9|31|33|29|
|3|9|24|26|19|
|0|10|37|39|40|
|1|10|38|36|37|
|2|10|35|37|34|
|3|10|28|30|27|
|0|11|41|42|44|
|1|11|42|40|41|
|2|11|39|41|38|
|3|11|36|34|35|
|0|12|45|43|47|
|1|12|46|44|45|
|2|12|43|45|42|
|3|12|40|38|39|
|0|13|50|47|49|
|1|13|49|48|48|
|2|13|47|49|46|
|3|13|44|46|43|
|0|14|54|51|53|
|1|14|51|54|50|
|2|14|52|52|51|
|3|14|48|50|52|
|0|15|58|55|57|
|1|15|55|56|54|
|2|15|56|57|55|
|3|15|53|53|56|
|0|16|61|59|61|
|1|16|59|60|58|
|2|16|60|61|59|






**Table G-10 – Mapping of ( nX, nY ) to coeffTokenIdx and vice versa**

|nY|nX|0 <= nC < 2|2 <= nC < 4|4 <= nC < 8|
|---|---|---|---|---|
|3|16|57|58|60|



**G.9.2.2** **Alternative parsing process for run information**


Inputs to this process are bits from slice data and the number of non-zero transform coefficient levels
TotalCoeff( coeff_token ).


Output of this process is a list of runs of zero transform coefficient levels preceding non-zero transform coefficient levels
called runVal.


The variable maxCoeff is derived as follows:


- If the CAVLC parsing process is invoked for Intra16x16DCLevel, CbIntra16x16DCLevel, or CrIntra16x16DCLevel,
maxCoeff is set equal to 16.


- Otherwise, if the CAVLC parsing process is invoked for ChromaDCLevel, maxCoeff is set equal to
4 * chroma_format_idc.


- Otherwise, if the CAVLC parsing process is invoked for LumaLevel4x4 or LumaLevel8x8, maxCoeff is set equal to
(scan_idx_end − scan_idx_start + 1).


- Otherwise (the CAVLC parsing process is invoked for Intra16x16ACLevel, CbIntra16x16ACLevel,
CrIntra16x16ACLevel, or ChromaACLevel), maxCoeff is set equal to (scan_idx_end − Max( 1, scan_idx_start ) + 1).


Initially, an index i is set equal to 0.


The variable zerosLeft is derived as follows:


- If the number of non-zero transform coefficient levels TotalCoeff( coeff_token ) is equal to the maximum number of
non-zero transform coefficient levels maxCoeff, a variable zerosLeft is set equal to 0.


- Otherwise (the number of non-zero transform coefficient levels TotalCoeff( coeff_token ) is less than the maximum
number of non-zero transform coefficient levels maxCoeff), total_zeros is decoded and zerosLeft is set equal to its
value.


The VLC used to decode total_zeros is derived as follows:


- If maxCoeff is less than or equal to 4, one of the VLCs specified in Table 9-9(a) is used with tzVlcIndex being derived
by


tzVlcIndex = TotalCoeff( coeff_token ) + 4 − maxCoeff (G-360)


- Otherwise, if maxCoeff is greater than 4 and less than or equal to 8, one of the VLCs specified in Table 9-9(b) is used
with tzVlcIndex being derived by


tzVlcIndex = TotalCoeff( coeff_token ) + 8 − maxCoeff (G-361)


- Otherwise, if maxCoeff is greater than 8 and less than 15, VLCs from Tables 9-7 and 9-8 are used with tzVlcIndex
being derived by


tzVlcIndex = TotalCoeff( coeff_token ) + 16 − maxCoeff (G-362)


- Otherwise (maxCoeff is greater than or equal to 15), VLCs from Tables 9-7 and 9-8 are used with tzVlcIndex equal
to TotalCoeff( coeff_token ).


The following procedure is then applied iteratively (TotalCoeff( coeff_token ) − 1) times:


1. The variable runVal[ i ] is derived as follows:


     - If zerosLeft is greater than zero, a value run_before is decoded based on Table 9-10 and zerosLeft. runVal[ i ]
is set equal to run_before.


     - Otherwise (zerosLeft is equal to 0), runVal[ i ] is set equal to 0.


2. The value of runVal[ i ] is subtracted from zerosLeft and the result assigned to zerosLeft. The result of the

subtraction shall be greater than or equal to 0.





3. The index i is incremented by 1.


Finally the value of zerosLeft is assigned to runVal[ i ].


**G.9.3** **Alternative CABAC parsing process for slice data in scalable extension**


Clause G.9.3.1 specifies the initialization process for the alternative CABAC parsing process for slice data in scalable
extension.


Clause G.9.3.2 specifies the binarization process for the alternative CABAC parsing process for slice data in scalable
extension.


Clause G.9.3.3 specifies the decoding process flow for the alternative CABAC parsing process for slice data in scalable
extension.


**G.9.3.1** **Initialization process**


Outputs of this process are the initialized CABAC context variables indexed by ctxIdx.


Tables G-12 and G-13 contain the values of the variables n and m used in the initialization of context variables that are
assigned to syntax element base_mode_flag, motion_prediction_flag_l0, motion_prediction_flag_l1, and
residual_prediction_flag in clause G.7.3.4.1 and G.7.3.6. For all other syntax elements in clauses G.7.3.4.1 and G.7.3.6 the
initialization process of context variables as specified in clause 9.3.1 applies.


For each context variable, the two variables pStateIdx and valMPS are initialized. The two values assigned to pStateIdx
and valMPS for the initialization are derived from SliceQPY, which is derived in Equation 7-30. Given the two table entries
( m, n ), the initialization is specified by the following pseudo-code process:


preCtxState = Clip3( 1, 126, ( ( m  Clip3( 0, 51, SliceQPY ) ) >> 4 ) + n )
if( preCtxState <= 63 ) {
pStateIdx = 63 − preCtxState
valMPS = 0
} else { (G-363)
pStateIdx = preCtxState − 64
valMPS = 1
}


In Table G-11, the ctxIdx for which initialization is needed for each of the slice types EI, EP, and EB are listed. Also listed
is the table number that includes the values of m and n needed for the initialization. For EP and EB slices, the initialization
depends also on the value of the cabac_init_idc syntax element. Note that the syntax element names do not affect the
initialization process.


**Table G-11 – Association of ctxIdx and syntax elements for each slice type in the initialization process**







|Col1|Syntax element|Table|Slice type|Col5|Col6|
|---|---|---|---|---|---|
||**Syntax element**|**Table**|**EI**|**EP**|**EB**|
|macroblock_layer_in_scalable_extension( )|base_mode_flag|Table G-12|1024..1026|1024..1026|1024..1026|
|mb_pred_in_scalable_extension( ) and<br>sub_mb_pred_in_scalable_extension( )|motion_prediction_flag_l0|Table G-13||1027|1027|
|mb_pred_in_scalable_extension( ) and<br>sub_mb_pred_in_scalable_extension( )|motion_prediction_flag_l1|Table G-13||1028|1028|
|macroblock_layer_in_scalable_extension( )|residual_prediction_flag|Table G-13||1029..1030|1029..1030|





**Table G-12 – Values of variables m and n for ctxIdx from 1024 to 1026**







|ctxIdx|EI slices|Col3|Value of cabac_init_idc (EP, EB slices)|Col5|Col6|Col7|Col8|Col9|
|---|---|---|---|---|---|---|---|---|
|**ctxIdx**|<br>**EI slices**|<br>**EI slices**|**0 **|**0 **|**1 **|**1 **|**2 **|**2 **|
|**ctxIdx**|**m **|**n **|**m **|**n **|**m **|**n **|**m **|**n **|
|**1024**|−14|138|0|75|0|75|0|75|
|**1025**|−22|140|2|65|2|65|2|65|
|**1026**|−11|99|2|59|2|59|2|59|


**Table G-13 – Values of variables m and n for ctxIdx from 1027 to 1030**







**G.9.3.2** **Binarization process**

|ctxIdx|Value of cabac_init_idc|Col3|Col4|Col5|Col6|Col7|
|---|---|---|---|---|---|---|
|**ctxIdx**|<br>**0 **|<br>**0 **|**1 **|**1 **|**2 **|**2 **|
|**ctxIdx**|**m **|**n **|**m **|**n **|**m **|**n **|
|**1027**|−6|67|−6|67|−6|67|
|**1028**|−6|67|−6|67|−6|67|
|**1029**|−23|104|−23|104|−23|104|
|**1030**|−35|106|−35|106|−35|106|



Input to this process is a request for a syntax element.


Output of this process is the binarization of the syntax element, maxBinIdxCtx, ctxIdxOffset, and bypassFlag.


Associated with each binarization or binarization part of a syntax element is a specific value of the context index offset
(ctxIdxOffset) variable and a specific value of the maxBinIdxCtx variable as given in Table G-14.


The variable bypassFlag is set equal to 0.


The possible values of the context index ctxIdx are in the range 1024 to 1030, inclusive. The value assigned to ctxIdxOffset
specifies the lower value of the range of ctxIdx assigned to the corresponding binarization or binarization part of a syntax
element.


**Table G-14 – Syntax elements and associated types of binarization, maxBinIdxCtx, and ctxIdxOffset**

|Syntax element|Type of binarization|maxBinIdxCtx|ctxIdxOffset|
|---|---|---|---|
|base_mode_flag|FL, cMax=1|0|1024|
|motion_prediction_flag_l0|FL, cMax=1|0|1027|
|motion_prediction_flag_l1|FL, cMax=1|0|1028|
|residual_prediction_flag|FL, cMax=1|0|1029|



**G.9.3.3** **Decoding process flow**


Input to this process is a binarization of the requested syntax element, maxBinIdxCtx, bypassFlag and ctxIdxOffset as
specified in clause G.9.3.2.


Output of this process is the value of the syntax element.


This process specifies how each bit of a bit string is parsed for each syntax element.





After parsing each bit, the resulting bit string is compared to all bin strings of the binarization of the syntax element and
the following applies:


- If the bit string is equal to one of the bin strings, the corresponding value of the syntax element is the output.


- Otherwise (the bit string is not equal to one of the bin strings), the next bit is parsed.


While parsing each bin, the variable binIdx is incremented by 1 starting with binIdx being set equal to 0 for the first bin.


The parsing of each bin is specified by the following two ordered steps:


1. Given binIdx, maxBinIdxCtx and ctxIdxOffset, ctxIdx is derived as specified in clause G.9.3.3.1.


2. Given ctxIdx, the value of the bin from the bitstream as specified in clause 9.3.3.2 is decoded.


**G.9.3.3.1** **Derivation process for ctxIdx**


Inputs to this process are binIdx, maxBinIdxCtx and ctxIdxOffset.


Output of this process is ctxIdx.


Table G-15 shows the assignment of ctxIdx increments (ctxIdxInc) to binIdx for all ctxIdxOffset values for the syntax
elements base_mode_flag, motion_prediction_flag_l0, motion_prediction_flag_l1, and residual_prediction_flag.


The ctxIdx to be used with a specific binIdx is the sum of ctxIdxOffset and ctxIdxInc, which is found in Table G-15. When
more than one value is listed in Table G-15 or 9-39 for a binIdx, the assignment process for ctxIdxInc for that binIdx is
further specified in the clauses given in parenthesis of the corresponding table entry.


All entries in Table G-15 labelled with "na" correspond to values of binIdx that do not occur for the corresponding
ctxIdxOffset.


**Table G-15 – Assignment of ctxIdxInc to binIdx for the ctxIdxOffset values related to the syntax elements**

**base_mode_flag and residual_prediction_flag**







|ctxIdxOffset|binIdx|Col3|Col4|Col5|Col6|Col7|Col8|
|---|---|---|---|---|---|---|---|
|**ctxIdxOffset**|**0 **|**1 **|**2 **|**3 **|**4 **|** 5**|**>= 6**|
|**1024**|0,1,2<br>(clause G.9.3.3.2.1)|na|na|na|na|na|na|
|**1027**|0|na|na|na|na|na|na|
|**1028**|0|na|na|na|na|na|na|
|**1029**|0,1<br>(clause G.9.3.3.2.2)|na|na|na|na|na|na|


**G.9.3.3.2** **Assignment process of ctxIdxInc using neighbouring syntax elements**


Clause G.9.3.3.2.1 specifies the derivation process of ctxIdxInc for the syntax element base_mode_flag.


Clause G.9.3.3.2.2 specifies the derivation process of ctxIdxInc for the syntax element residual_prediction_flag.


**G.9.3.3.2.1** **Derivation process of ctxIdxInc for the syntax element base_mode_flag**


Output of this process is ctxIdxInc.


The derivation process for neighbouring macroblocks specified in clause 6.4.11.1 is invoked and the output is assigned to
mbAddrA and mbAddrB.


Let the variable condTermFlagN (with N being either A or B) be derived as follows:


- If mbAddrN is available and base_mode_flag for the macroblock mbAddrN is equal to 1, condTermFlagN is set equal
to 0.


- Otherwise (mbAddrN is not available or base_mode_flag for the macroblock mbAddrN is equal to 0),
condTermFlagN is set equal to 1.


The variable ctxIdxInc is derived by


ctxIdxInc = condTermFlagA + condTermFlagB (G-364)





**G.9.3.3.2.2** **Derivation process of ctxIdxInc for the syntax element residual_prediction_flag**


Output of this process is ctxIdxInc.


Depending on base_mode_flag, the following applies:


- If base_mode_flag is equal to 1, ctxIdxInc is set equal to 0.


- Otherwise (base_mode_flag is equal to 0), ctxIdxInc is set equal to 1.
