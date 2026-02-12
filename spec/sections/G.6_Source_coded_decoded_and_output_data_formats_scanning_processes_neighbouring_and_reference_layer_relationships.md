**G.5** **Conventions**


The specifications in clause 5 apply.


**G.6** **Source, coded, decoded and output data formats, scanning processes, neighbouring and**
**reference layer relationships**


The specifications in clause 6 apply with substituting SVC sequence parameter set for sequence parameter set. The
specification in clause 6.3 also applies to layer pictures. Additionally, the following processes are specified.


**G.6.1** **Derivation process for reference layer macroblocks**


This process is only invoked when no_inter_layer_pred_flag is equal to 0.


Inputs to this process are:


- a luma location ( xP, yP ) relative to the upper-left luma sample of the current macroblock,


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a one-dimensional array refLayerFieldMbFlag with RefLayerPicSizeInMbs elements specifying which macroblocks
of the reference layer representation are field macroblocks and which macroblocks are frame macroblocks,


- a one-dimensional array refLayerMbType with RefLayerPicSizeInMbs elements specifying macroblock types for the
macroblocks of the reference layer representation.


Outputs of this process are:


- the macroblock address mbAddrRefLayer specifying the reference layer macroblock,


- a luma location ( xB, yB ) relative to the upper-left luma sample of the reference layer macroblock.


Let currDQId be the current value of DQId and let levelIdc be the value of level_idc in the SVC sequence parameter set
that is referred to in coded slice NAL units with DQId equal to (( currDQId >> 4 ) << 4).


The variables shiftX, shiftY, scaleX, scaleY, offsetX, and offsetY are derived as specified in the following ordered steps:


1. The variables refW, refH, scaledW, scaledH, offsetX, and offsetY are derived by


refW   = RefLayerPicWidthInSamplesL (G-1)
refH    = RefLayerPicHeightInSamplesL (G-2)
scaledW = ScaledRefLayerPicWidthInSamplesL (G-3)
scaledH = ScaledRefLayerPicHeightInSamplesL (G-4)
offsetX  = ScaledRefLayerLeftOffset (G-5)
offsetY  = ScaledRefLayerTopOffset / ( 1 + field_pic_flag ) (G-6)


2. The variables shiftX and shiftY are derived by


shiftX = ( ( levelIdc <= 30 ) ? 16 : ( 31 − Ceil( Log2( refW ) ) ) ) (G-7)
shiftY = ( ( levelIdc <= 30 ) ? 16 : ( 31 − Ceil( Log2( refH ) ) ) ) (G-8)


3. The variables scaleX and scaleY are derived by





scaleX = ( ( refW << shiftX ) + ( scaledW >> 1 ) ) / scaledW (G-9)
scaleY = ( ( refH << shiftY ) + ( scaledH >> 1 ) ) / scaledH (G-10)


NOTE 1 – The variables shiftX, shiftY, scaleX, scaleY, offsetX, and offsetY do not depend on the luma location ( xP, yP ), the
variable fieldMbFlag, or the current macroblock.


The reference layer luma location ( xRef, yRef ) relative to the upper-left sample of the reference layer picture is derived
as specified by the following ordered steps:


1. The inverse macroblock scanning process as specified in clause 6.4.1 is invoked with CurrMbAddr as the input

and the output is assigned to ( xM, yM ). For this invocation of the process in clause 6.4.1, the current macroblock
is treated as field macroblock when fieldMbFlag is equal to 1, and it is treated as frame macroblock when
fieldMbFlag is equal to 0.


2. The luma location ( xC, yC ) is derived by


xC = xM + xP (G-11)
yC = yM + yP * ( 1 + fieldMbFlag − field_pic_flag ) (G-12)


3. The reference layer luma location is derived by


xRef = ( ( xC − offsetX ) * scaleX + ( 1 << ( shiftX − 1 ) ) ) >> shiftX (G-13)
yRef = ( ( yC − offsetY ) * scaleY + ( 1 << ( shiftY − 1 ) ) ) >> shiftY (G-14)


4. The reference layer luma location is modified by


xRef = Min( RefLayerPicWidthInSamplesL − 1, xRef ) (G-15)
yRef = Min( RefLayerPicHeightInSamplesL − 1, yRef ) (G-16)


The reference layer macroblock address mbAddrRefLayer and a luma location ( xB, yB ) relative to the upper-left sample
of the reference layer macroblock mbAddrRefLayer are derived as follows:


- If MbaffFrameFlag is equal to 0 and RefLayerMbaffFrameFlag is equal to 0, the following ordered steps are specified:


1. The reference layer macroblock address mbAddrRefLayer is derived by


mbAddrRefLayer = ( yRef / 16 ) * RefLayerPicWidthInMbs + ( xRef / 16 ) (G-17)


2. The luma location ( xB, yB ) is derived as follows:


      - If mbAddrRefLayer is not available, ( xB, yB ) is marked as not available.


      - Otherwise (mbAddrRefLayer is available), ( xB, yB ) is set equal to ( xRef % 16, yRef % 16 ).


- Otherwise (MbaffFrameFlag is equal to 1 or RefLayerMbaffFrameFlag is equal to 1), the following ordered steps are
specified:

NOTE 2 – When MbaffFrameFlag is equal to 1 or RefLayerMbaffFrameFlag is equal to 1, field_pic_flag and
RefLayerFieldPicFlag are both equal to 0 (see clause G.7.4.3.4).


1. A virtual reference layer macroblock address virtMbAddrRefLayer is derived as follows:


      - If RefLayerMbaffFrameFlag is equal to 1, virtMbAddrRefLayer is derived by


virtMbAddrRefLayer = 2 * ( ( yRef / 32 ) * RefLayerPicWidthInMbs + ( xRef / 16 ) ) +
( yRef % 32 ) / 16 (G-18)


      - Otherwise (RefLayerMbaffFrameFlag is equal to 0), virtMbAddrRefLayer is derived by


virtMbAddrRefLayer = ( yRef / 16 ) * RefLayerPicWidthInMbs + ( xRef / 16 ) (G-19)


2. The reference layer macroblock address mbAddrRefLayer and the luma location ( xB, yB ) are derived as

follows:


      - If fieldMbFlag is equal to 0 and refLayerFieldMbRef[ virtMbAddrRefLayer ] is equal to 1, the field-toframe reference layer macroblock conversion process as specified in clause G.6.1.1 is invoked with
virtMbAddrRefLayer, ( xRef, yRef ), and refLayerMbType as the inputs and the outputs are assigned to
mbAddrRefLayer and ( xB, yB ).





      - Otherwise, if fieldMbFlag is equal to 1 and refLayerFieldMbRef[ virtMbAddrRefLayer ] is equal to 0, the
frame-to-field reference layer macroblock conversion process as specified in clause G.6.1.2 is invoked with
virtMbAddrRefLayer and ( xRef, yRef ) as the inputs and the outputs are assigned to mbAddrRefLayer and
( xB, yB ).


      - Otherwise (fieldMbFlag is equal to refLayerFieldMbRef[ virtMbAddrRefLayer ]), mbAddrRefLayer and
( xB, yB ) are derived by


mbAddrRefLayer = ( ( virtMbAddrRefLayer >> fieldMbFlag ) << fieldMbFlag )
+ ( CurrMbAddr % 2 ) * fieldMbFlag (G-20)
xB = ( xRef % 16 ) (G-21)
yB = ( yRef % ( 16 << fieldMbFlag ) ) >> fieldMbFlag (G-22)


**G.6.1.1** **Field-to-frame reference layer macroblock conversion process**


Inputs to this process are:


- a virtual reference layer macroblock address virtMbAddrRefLayer,


- a reference layer luma location ( xRef, yRef ) relative to the upper-left luma sample of the reference layer picture,


- a one-dimensional array refLayerMbType with RefLayerPicSizeInMbs elements specifying macroblock types for the
macroblocks of the reference layer representation.


Outputs of this process are:


- the macroblock address mbAddrRefLayer of the reference layer macroblock,


- a luma location ( xB, yB ) relative to the upper-left luma sample of the reference layer macroblock.


The macroblock addresses mbAddrRefLayerTop and mbAddrRefLayerBot are derived by


mbAddrRefLayerTop = virtMbAddrRefLayer − ( virtMbAddrRefLayer % 2 ) (G-23)
mbAddrRefLayerBot = mbAddrRefLayerTop + 1 (G-24)


The reference layer macroblock address mbAddrRefLayer is derived as follows:


- If refLayerMbType[ mbAddrRefLayerTop ] is equal to I_PCM, I_16x16, I_8x8, I_4x4, or I_BL, mbAddrRefLayer is
set equal to mbAddrRefLayerBot.


- Otherwise (refLayerMbType[ mbAddrRefLayerTop ] is not equal to I_PCM, I_16x16, I_8x8, I_4x4, or I_BL),
mbAddrRefLayer is set equal to mbAddrRefLayerTop.


The luma location ( xB, yB ) is derived by


xB = xRef % 16 (G-25)
yB = 8 * ( ( yRef / 16 ) % 2 ) + 4 * ( ( yRef % 16 ) / 8 ) (G-26)


**G.6.1.2** **Frame-to-field reference layer macroblock conversion process**


Inputs to this process are:


- a virtual reference layer macroblock address virtMbAddrRefLayer,


- a virtual reference layer luma location ( xRef, yRef ) relative to the upper-left luma sample of the reference layer
picture.


Outputs of this process are:


- the macroblock address mbAddrRefLayer of the reference layer macroblock,


- a luma location ( xB, yB ) relative to the upper-left luma sample of the reference layer macroblock.





The reference layer macroblock address mbAddrRefLayer and the luma location ( xB, yB ) are derived by


mbAddrRefLayer = virtMbAddrRefLayer (G-27)
xB            = xRef % 16 (G-28)
yB            = yRef % 16 (G-29)


**G.6.2** **Derivation process for reference layer partitions**


This process is only invoked when no_inter_layer_pred_flag is equal to 0.


Inputs to this process are:


- a luma location ( xP, yP ) relative to the upper-left luma sample of the current macroblock,


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a one-dimensional array refLayerFieldMbFlag with RefLayerPicSizeInMbs elements specifying which macroblocks
of the reference layer representation are field macroblocks and which macroblocks are frame macroblocks,


- a one-dimensional array refLayerMbType with RefLayerPicSizeInMbs elements specifying macroblock types for the
macroblocks of the reference layer representation,


- a (RefLayerPicSizeInMbs)x4 array refLayerSubMbType specifying the sub-macroblock types for the macroblocks of
the reference layer representation.


Outputs of this process are:


- the macroblock address mbAddrRefLayer specifying the reference layer macroblock,


- the macroblock partition index mbPartIdxRefLayer specifying the reference layer macroblock partition inside the
reference layer macroblock mbAddrRefLayer,


- the sub-macroblock partition index subMbPartIdxRefLayer specifying the reference layer sub-macroblock partition
inside the macroblock partition mbPartIdxRefLayer of the reference layer macroblock mbAddrRefLayer.


The derivation process for reference layer macroblocks as specified in clause G.6.1 is invoked with the luma
location ( xP, yP ), fieldMbFlag, refLayerFieldMbFlag, and refLayerMbType as the input and the outputs are assigned to
mbAddrRefLayer and ( xB, yB ).


The SVC derivation process for macroblock and sub-macroblock partition indices as specified in clause G.6.4 is invoked
with currDQId set equal to ref_layer_dq_id, the luma location ( xB, yB ), the macroblock type
refLayerMbType[ mbAddrRefLayer ], and, when refLayerMbType[ mbAddrRefLayer ] is equal to P_8x8, P_8x8ref0, or
B_8x8, the list of sub-macroblock types refLayerSubMbType[ mbAddrRefLayer ] as the inputs and the outputs are the
reference layer macroblock partition index mbPartIdxRefLayer and the reference layer sub-macroblock partition index
subMbPartIdxRefLayer.


**G.6.3** **Derivation process for reference layer sample locations in resampling**


Inputs to this process are:


- a variable chromaFlag specifying whether the luma or a chroma component is subject to the resampling process,


- a sample location ( xP, yP ) relative to the upper-left sample of the current macroblock,


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a variable botFieldFlag specifying whether a top or a bottom field is subject to the resampling process (when
RefLayerFrameMbsOnlyFlag is equal to 0 or frame_mbs_only_flag is equal to 0).


Output of this process is a reference layer sample location ( xRef16, yRef16 ), which specifies the following:


- If RefLayerFrameMbsOnlyFlag is equal to 1 or RefLayerFieldPicFlag is equal to 1, ( xRef16, yRef16 ) specifies the
reference layer sample location in units of 1/16-th sample relative to the upper-left sample of the reference layer
picture.


- Otherwise (RefLayerFrameMbsOnlyFlag is equal to 0 and RefLayerFieldPicFlag is equal to 0), ( xRef16, yRef16 )
specifies the reference layer sample location in units of 1/16-th field sample relative to the upper-left sample of the
field specified by botFieldFlag of the reference layer picture.


Let currDQId be the current value of DQId and let levelIdc be the value of level_idc in the SVC sequence parameter set
that is referred to in coded slice NAL units with DQId equal to (( currDQId >> 4 ) << 4).





The variables subW, subH, shiftX, shiftY, scaleX, scaleY, offsetX, offsetY, addX, addY, deltaX, and deltaY are derived
as specified in the following ordered steps:


1. With Z being replaced by L for chromaFlag equal to 0 and C for chromaFlag equal to 1, the variables refW, refH,

scaledW, and scaledH are derived by


refW   = RefLayerPicWidthInSamplesZ (G-30)
refH    = RefLayerPicHeightInSamplesZ * ( 1 + RefLayerFieldPicFlag ) (G-31)
scaledW = ScaledRefLayerPicWidthInSamplesZ (G-32)
scaledH = ScaledRefLayerPicHeightInSamplesZ * ( 1 + field_pic_flag ) (G-33)


2. When frame_mbs_only_flag is equal to 0 and RefLayerFrameMbsOnlyFlag is equal to 1, the variable scaledH is

modified by


scaledH = scaledH / 2 (G-34)


3. The variables refPhaseX, refPhaseY, phaseX, phaseY, subW, and subH are derived by


refPhaseX = ( ( chromaFlag = = 0 ) ? 0 : ( ref_layer_chroma_phase_x_plus1_flag − 1 ) ) (G-35)
refPhaseY = ( ( chromaFlag = = 0 ) ? 0 : ( ref_layer_chroma_phase_y_plus1 − 1 ) ) (G-36)
phaseX   = ( ( chromaFlag = = 0 ) ? 0 : ( chroma_phase_x_plus1_flag − 1 ) ) (G-37)
phaseY   = ( ( chromaFlag = = 0 ) ? 0 : ( chroma_phase_y_plus1 − 1 ) ) (G-38)
subW     = ( ( chromaFlag = = 0 ) ? 1 : SubWidthC ) (G-39)
subH     = ( ( chromaFlag = = 0 ) ? 1 : SubHeightC ) (G-40)


4. When RefLayerFrameMbsOnlyFlag is equal to 0 or frame_mbs_only_flag is equal to 0, the following applies:


     - If RefLayerFrameMbsOnlyFlag is equal to 1, the variables phaseY and refPhaseY are modified by


phaseY   = phaseY + 4 * botFieldFlag + 3 − subH (G-41)
refPhaseY = 2 * refPhaseY + 2 (G-42)


     - Otherwise (RefLayerFrameMbsOnlyFlag is equal to 0), the variables phaseY and refPhaseY are modified
by


phaseY   = phaseY + 4 * botFieldFlag (G-43)
refPhaseY = refPhaseY + 4 * botFieldFlag (G-44)


5. The variables shiftX and shiftY are derived by


shiftX = ( ( levelIdc <= 30 ) ? 16 : ( 31 − Ceil( Log2( refW ) ) ) ) (G-45)
shiftY = ( ( levelIdc <= 30 ) ? 16 : ( 31 − Ceil( Log2( refH ) ) ) ) (G-46)


6. The variables scaleX and scaleY are derived by


scaleX = ( ( refW << shiftX ) + ( scaledW >> 1 ) ) / scaledW (G-47)
scaleY = ( ( refH << shiftY ) + ( scaledH >> 1 ) ) / scaledH (G-48)


7. The variables offsetX, addX, and deltaX are derived by


offsetX = ScaledRefLayerLeftOffset / subW (G-49)
addX  = ( ( ( refW * ( 2 + phaseX ) ) << ( shiftX − 2 ) ) + ( scaledW >> 1 ) ) / scaledW
+ ( 1 << ( shiftX − 5 ) ) (G-50)
deltaX = 4 * ( 2 + refPhaseX ) (G-51)


8. The variables offsetY, addY, and deltaY are derived as follows:


     - If RefLayerFrameMbsOnlyFlag is equal to 1 and frame_mbs_only_flag is equal to 1, the variables offsetY,
addY, and deltaY are derived by


offsetY = ScaledRefLayerTopOffset / subH (G-52)
addY  = ( ( ( refH * ( 2 + phaseY ) ) << ( shiftY − 2 ) ) + ( scaledH >> 1 ) ) / scaledH
+ ( 1 << ( shiftY − 5 ) ) (G-53)
deltaY = 4 * ( 2 + refPhaseY ) (G-54)





     - Otherwise (RefLayerFrameMbsOnlyFlag is equal to 0 or frame_mbs_only_flag is equal to 0), the variables
offsetY, addY, and deltaY are derived by


offsetY = ScaledRefLayerTopOffset / ( 2 * subH ) (G-55)
addY  = ( ( ( refH * ( 2 + phaseY ) ) << ( shiftY − 3 ) ) + ( scaledH >> 1 ) ) / scaledH
+ ( 1 << ( shiftY − 5 ) ) (G-56)
deltaY = 2 * ( 2 + refPhaseY ) (G-57)


NOTE – The variables subW, subH, shiftX, shiftY, scaleX, scaleY, offsetX, offsetY, addX, addY, deltaX, and deltaY do not depend
on the input sample location ( xP, yP ), the input variable fieldMbFlag, or the current macroblock address CurrMbAddr.


The sample location ( xC, yC ) is derived as specified in the following ordered steps:


1. The inverse macroblock scanning process as specified in clause 6.4.1 is invoked with CurrMbAddr as input and

the output is assigned to ( xM, yM ). For this invocation of the process in clause 6.4.1, the current macroblock is
treated as field macroblock when fieldMbFlag is equal to 1 and it is treated as frame macroblock when
fieldMbFlag is equal to 0.


2. The sample location ( xC, yC ) is derived by


xC = xP + ( xM >> ( subW − 1 ) ) (G-58)
yC = yP + ( yM >> ( subH − 1 + fieldMbFlag − field_pic_flag ) ) (G-59)


3. When RefLayerFrameMbsOnlyFlag is equal to 0 or frame_mbs_only_flag is equal to 0, the vertical component

of the sample location ( xC, yC ) is modified by


yC = yC >> ( 1 − fieldMbFlag ) (G-60)


The reference layer sample location ( xRef16 yRef16 ) is derived by


xRef16 = ( ( ( xC − offsetX ) * scaleX + addX ) >> ( shiftX − 4 ) ) − deltaX (G-61)
yRef16 = ( ( ( yC − offsetY ) * scaleY + addY ) >> ( shiftY − 4 ) ) − deltaY (G-62)


**G.6.4** **SVC derivation process for macroblock and sub-macroblock partition indices**


Inputs to this process are:


- a variable currDQId specifying an identifier for a layer representation,


- a luma location ( xP, yP ) relative to the upper-left luma sample of a macroblock,


- a macroblock type mbType,


- when mbType is equal to P_8x8, P_8x8ref0, or B_8x8, a list of sub-macroblock types subMbType with 4 elements.


Outputs of this process are:


- a macroblock partition index mbPartIdx,


- a sub-macroblock partition index subMbPartIdx.


The variable svcDirectModeFlag is derived as follows:


- If currDQId is greater than 0 and any of the following conditions are true, svcDirectModeFlag is set equal to 1.


   - mbType is equal to B_Skip or B_Direct_16x16


   - mbType is equal to B_8x8 and subMbType[ 2 * ( yP / 8 ) + ( xP / 8 ) ] is equal to B_Direct_8x8


- Otherwise, svcDirectModeFlag is set equal to 0.


Depending on svcDirectModeFlag, the following applies:


- If svcDirectModeFlag is equal to 0, the derivation process for macroblock and sub-macroblock partition indices as
specified in clause 6.4.13.4 is invoked with the luma location ( xP, yP ), the macroblock type mbType, and, when
mbType is equal to P_8x8, P_8x8ref0, or B_8x8, the list of sub-macroblock types subMbType as the inputs and the
outputs are the macroblock partition index mbPartIdx and the sub-macroblock partition index subMbPartIdx.


- Otherwise, if mbType is equal to B_Skip or B_Direct_16x16, mbPartIdx is set equal to 0 and subMbPartIdx is set
equal to 0.





- Otherwise (currDQId is greater than 0, mbType is equal to B_8x8, and subMbType[ 2 * ( yP / 8 ) + ( xP / 8 ) ] is
equal to B_Direct_8x8), mbPartIdx is set equal to ( 2 * ( yP / 8 ) + ( xP / 8 ) ) and subMbPartIdx is set equal to 0.
