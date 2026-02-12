**G.8** **SVC decoding process**


This clause describes the decoding process for an access unit, given syntax elements and upper-case variables from
clause G.7 (with reference made to clause 7 in clause G.7) that are derived from the bitstream.

NOTE 1 – All syntax elements and upper-case variables from clause G.7 are available for the entire current access unit. When syntax
elements or upper-case variables appear with identical names in clause G.7 they are referred herein through unique identifiers.


Outputs of this process are decoded samples of the current primary coded picture.


The decoding process is specified such that all decoders shall produce numerically identical results. Any decoding process
that produces identical results to the process described here conforms to the decoding process requirements of this
Recommendation | International Standard.


All sub-bitstreams that can be derived using the sub-bitstream extraction process as specified in clause G.8.8.1 with any
combination of values for priority_id, temporal_id, dependency_id, or quality_id as the input shall result in a set of coded
video sequences, with each coded video sequence conforming to one or more of the profiles specified in Annexes A and G.


This clause specifies the decoding process for an access unit of a coded video sequence conforming to one or more of the
profiles specified in clause G.10.


Each picture referred to in this clause is a complete primary coded picture or part of a primary coded picture. Each
dependency representation referred to in this clause is a dependency representation of a primary coded picture. Each layer
representation referred to in this clause is a layer representation of a primary coded picture. Each slice referred to in this
clause is a slice of a primary coded picture. All syntax elements and derived variables referred to in this clause are syntax
elements and derived variables for primary coded pictures.


Unless stated otherwise, the syntax elements and derived upper-case variables that are referred to by the decoding process
specified in this clause and all child processes invoked from the process specified in this clause are the syntax elements
and derived upper-case variables for the current access unit.


The derivation process for the set of layer representations required for decoding as specified in clause G.8.1.1 is invoked
and the output is a list dqIdList of integer values specifying layer representation identifiers. The variables DQIdMin and
DQIdMax are set equal to the minimum and maximum values, respectively, of the entries of the list dqIdList, and the
variable DependencyIdMax is set equal to (DQIdMax >> 4). DependencyIdMax shall be the same for all access units of
the coded video sequence.


At the start of the decoding process for an access unit, the following applies:


1. Variables and functions relating to picture order count are derived by invoking the SVC decoding process for

picture order count as specified in clause G.8.2.1 with dqIdList as the input.


2. The SVC decoding process for gaps in frame_num as specified in clause G.8.2.5 is invoked with dqIdList as the

input.


3. For each value of currDQId that is contained in the list dqIdList, the following applies:


     - The decoding process for macroblock to slice group map as specified in clause 8.2.2 is invoked with the
syntax elements of the NAL units with DQId equal to currDQId as the input. For this invocation of the
process specified in clause 8.2.2, when currDQId is less than DQIdMax, "active picture parameter set" is
substituted with "active layer picture parameter set".


     - The function NextMbAddress( ) as specified in clause 8.2.2 is used for parsing the slice data syntax structures
of all NAL units with DQId equal to currDQId and for inferring slice data and macroblock layer syntax
elements for slices with slice_skip_flag equal to 1 and DQId equal to currDQId (see clause G.7.4.3.4).


The collective terms currentVars and refLayerVars are initially marked as not available.


The variable currDQId proceeds over the values DQIdMin..DQIdMax, and when a value of currDQId is present in the list
dqIdList, the following ordered steps apply:


1. The variable spatResChangeFlag is set equal to the variable SpatialResolutionChangeFlag of the layer

representation with DQId equal to currDQId.


2. Depending on spatResChangeFlag, the following applies:





      - If spatResChangeFlag is equal to 0, the base decoding process for layer representations without resolution
change as specified in clause G.8.1.3.1 is invoked with currDQId and currentVars as the inputs and the
output is a modified version of currentVars.


      - Otherwise (spatResChangeFlag is equal to 1), the base decoding process for layer representations with
resolution change as specified in clause G.8.1.3.2 is invoked with currDQId and currentVars as the inputs
and the outputs are variables assigned to the collective term refLayerVars and a modified version of
currentVars.


3. When currDQId is equal to (DependencyIdMax << 4) and store_ref_base_pic_flag for the dependency

representation with dependency_id equal to DependencyIdMax is equal to 1, the target layer representation
decoding process as specified in clause G.8.1.3.3 is invoked with currDQId, refLayerVars (when
spatResChangeFlag is equal to 1), and currentVars as the inputs and the outputs are assigned to the sample array
BL and, when ChromaArrayType is not equal to 0, the sample arrays BCb and BCr.

NOTE 2 – The sample arrays BL, BCb, and BCr represent the reference base picture for an access unit with
store_ref_base_pic_flag equal to 1 for the dependency representation with dependency_id equal to DependencyIdMax.


The target layer representation decoding process as specified in clause G.8.1.3.3 is invoked with currDQId set equal to
DQIdMax, refLayerVars (when the variable SpatialResolutionChangeFlag of the layer representation with DQId equal to
DQIdMax is equal to 1), and currentVars as the inputs and the outputs are assigned to the sample array SL and, when
ChromaArrayType is not equal to 0, the sample arrays SCb and SCr.

NOTE 3 – The sample arrays SL, SCb, and SCr represent the decoded picture for the access unit.


The SVC decoded reference picture marking process as specified in clause G.8.2.4 is invoked with dqIdList as the input.


**G.8.1** **SVC initialization and decoding processes**


Clause G.8.1.1 specifies the derivation process for the set of layer representations required for decoding.


Clause G.8.1.2 specifies the array assignment, initialization, and restructuring processes.


Clause G.8.1.3 specifies the layer representation decoding processes.


Clause G.8.1.4 specifies the slice decoding processes.


Clause G.8.1.5 specifies the macroblock initialization and decoding processes.


**G.8.1.1** **Derivation process for the set of layer representations required for decoding**


Inputs to this process are the coded slice NAL units of an access unit.


Output of this process is a list dqIdList of integer values specifying layer representation identifiers.


With currDQId being set equal to the maximum value of DQId for all coded slice NAL units of the current access unit and
with refLayerDQId( dqId ) being a function that returns the value of MaxRefLayerDQId for the layer representation, of
the current access unit, with DQId equal to dqId, the list dqIdList is derived as specified by the following pseudo-code.


numEntries = 0
dqIdList[ numEntries++ ] = currDQId
while( refLayerDQId( currDQId ) >= 0 ) {
dqIdList[ numEntries++ ] = refLayerDQId( currDQId ) (G-75)
currDQId = dqIdList[ numEntries − 1 ]
}


**G.8.1.2** **Array assignment, initialization, and restructuring processes**


Clause G.8.1.2.1 specifies the array assignment and initialization process.


Clause G.8.1.2.2 specifies the array restructuring process.


**G.8.1.2.1** **Array assignment and initialization process**


Output of this process is a set of arrays that are assigned to the collective term currentVars.


The following arrays are collectively referred to as currentVars:


- A one-dimensional array sliceIdc with PicSizeInMbs elements specifying slice identifications for the macroblocks of
a layer representation. An element of this array for a macroblock with address mbAddr is referred to as
sliceIdc[ mbAddr ]. All elements of the array sliceIdc are initially marked as unspecified.


- A one-dimensional array fieldMbFlag with PicSizeInMbs elements specifying which macroblocks of a layer
representation are field macroblocks and which macroblocks are frame macroblocks. An element of this array for a





macroblock with address mbAddr is referred to as fieldMbFlag[ mbAddr ]. All elements of the array fieldMbFlag are
initially marked as unspecified.


- A one-dimensional array cTrafo with PicSizeInMbs elements specifying the luma and, when ChromaArrayType is
equal to 3, chroma transform types for the macroblocks of a layer representation. An element of this array for a
macroblock with address mbAddr is referred to as cTrafo[ mbAddr ]. Unless marked as unspecified, each element of
cTrafo is equal to T_4x4, T_8x8, T_16x16, or T_PCM. All elements of the array cTrafo are initially marked as
unspecified.


- A one-dimensional array baseModeFlag with PicSizeInMbs elements specifying the syntax element base_mode_flag
for the macroblocks of a layer representation. An element of this array for a macroblock with address mbAddr is
referred to as baseModeFlag[ mbAddr ]. All elements of the array baseModeFlag are initially marked as unspecified.


- A one-dimensional array mbType with PicSizeInMbs elements specifying macroblock types for the macroblocks of
a layer representation. An element of this array for a macroblock with address mbAddr is referred to as
mbType[ mbAddr ]. Unless marked as unspecified, each element of mbType is equal to I_4x4, I_8x8, I_16x16,
I_PCM, I_BL, or one of the Inter macroblock types specified in Tables 7-13 and 7-14. All elements of the array
mbType are initially marked as unspecified.


- A (PicSizeInMbs)x4 array subMbType specifying sub-macroblock types for the macroblocks of a layer
representation. An element of this array for a macroblock with address mbAddr and a macroblock partition index
mbPartIdx is referred to as subMbType[ mbAddr ][ mbPartIdx ]. A one-dimensional array specifying sub-macroblock
types for the macroblock partitions of a macroblock with address mbAddr is referred to as subMbType[ mbAddr ].
Unless marked as unspecified, each element of subMbType is equal to one of the sub-macroblock types specified in
Tables 7-17 and 7-18. All elements of the array subMbType are initially marked as unspecified.


- A one-dimensional array mvCnt with PicSizeInMbs elements specifying the number of motion vectors for the
macroblocks of a layer representation. An element of this array for a macroblock with address mbAddr is referred to
as mvCnt[ mbAddr ]. All elements of the array mvCnt are initially set equal to 0.


- A one-dimensional array tQPY with PicSizeInMbs elements specifying luma quantization parameters for the
macroblocks of a layer representation. An element of this array for a macroblock with address mbAddr is referred to
as tQPY[ mbAddr ]. All elements of the array tQPY are initially set equal to 0.


- When ChromaArrayType is not equal to 0, two one-dimensional arrays tQPCb and tQPCr with PicSizeInMbs elements
specifying Cb and Cr quantization parameters, respectively, for the macroblocks of a layer representation. An element
of these arrays for a macroblock with address mbAddr is referred to as tQPCX[ mbAddr ] with CX being replaced by
Cb or Cr. All elements of the arrays tQPCb and tQPCr are initially set equal to 0.


- A (PicSizeInMbs)x16 array ipred4x4 specifying Intra_4x4 prediction modes for the macroblocks of a layer
representation. An element of this array for a macroblock with address mbAddr and a 4x4 block with index
c4x4BlkIdx is referred to as ipred4x4[ mbAddr ][ c4x4BlkIdx ]. A one-dimensional array specifying Intra_4x4
prediction modes for the 4x4 blocks of a macroblock with address mbAddr is referred to as ipred4x4[ mbAddr ]. All
elements of the array ipred4x4 are initially marked as unspecified.


- A (PicSizeInMbs)x4 array ipred8x8 specifying Intra_8x8 prediction modes for the macroblocks of a layer
representation. An element of this array for a macroblock with address mbAddr and a 8x8 block with index
c8x8BlkIdx is referred to as ipred8x8[ mbAddr ][ c8x8BlkIdx ]. A one-dimensional array specifying Intra_8x8
prediction modes for the 8x8 blocks of a macroblock with address mbAddr is referred to as ipred8x8[ mbAddr ]. All
elements of the array ipred8x8 are initially marked as unspecified.


- A one-dimensional array ipred16x16 with PicSizeInMbs elements specifying Intra_16x16 prediction modes for the
macroblocks of a layer representation. An element of this array for a macroblock with address mbAddr is referred to
as ipred16x16[ mbAddr ]. All elements of the array ipred16x16 are initially marked as unspecified.


- When ChromaArrayType is equal to 1 or 2, a one-dimensional array ipredChroma with PicSizeInMbs elements
specifying intra chroma prediction modes for the macroblocks of a layer representation. An element of this array for
a macroblock with address mbAddr is referred to as ipredChroma[ mbAddr ]. All elements of the array ipredChroma
are initially marked as unspecified.


- Two (PicSizeInMbs)x4 arrays predFlagL0 and predFlagL1 specifying prediction utilization flags for the macroblocks
of a layer representation. An element of these arrays for a macroblock with address mbAddr and a macroblock
partition index mbPartIdx is referred to as predFlagLX[ mbAddr ][ mbPartIdx ] with X being replaced by 0 or 1. A
one-dimensional array specifying prediction utilization flags for the macroblock partitions of a macroblock with
address mbAddr is referred to as predFlagLX[ mbAddr ] with X being replaced by 0 or 1. All elements of the arrays
predFlagL0 and predFlagL1 are initially set equal to 0.


- Two (PicSizeInMbs)x4 arrays refIdxL0 and refIdxL1 specifying reference indices for the macroblocks of a layer
representation. An element of these arrays for a macroblock with address mbAddr and a macroblock partition index





mbPartIdx is referred to as refIdxLX[ mbAddr ][ mbPartIdx ] with X being replaced by 0 or 1. A one-dimensional
array specifying reference indices for the macroblock partitions of a macroblock with address mbAddr is referred to
as refIdxLX[ mbAddr ] with X being replaced by 0 or 1. All elements of the arrays refIdxL0 and refIdxL1 are initially
set equal to −1.


- Two (PicSizeInMbs)x4x4x2 arrays mvL0 and mvL1 specifying motion vector components for the macroblocks of a
layer representation. An element of these arrays for a macroblock with address mbAddr, a macroblock partition index
mbPartIdx, a sub-macroblock partition index subMbPartIdx, and a motion vector component index c is referred to as
mvLX[ mbAddr ][ mbPartIdx ][ subMbPartIdx ][ c ] with X being replaced by 0 or 1. A one-dimensional array with
2 elements representing the motion vector for a sub-macroblock partition subMbPartIdx of a macroblock partition
mbPartIdx inside a macroblock mbAddr is referred to as mvLX[ mbAddr ][ mbPartIdx ][ subMbPartIdx ] with X
being replaced by 0 or 1. A 4x2 array representing the motion vectors for a macroblock partition mbPartIdx inside a
macroblock mbAddr is referred to as mvLX[ mbAddr ][ mbPartIdx ] with X being replaced by 0 or 1. A 4x4x2 array
representing the motion vectors for a macroblock mbAddr is referred to as mvLX[ mbAddr ] with X being replaced
by 0 or 1. A motion vector component with component index c for a macroblock partition mbPartIdx of a macroblock
mbAddr that is not split into sub-macroblock partitions can also be referred to as mvLX[ mbAddr ][ mbPartIdx ][ c ]
with X being replaced by 0 or 1, which is identical to mvLX[ mbAddr ][ mbPartIdx ][ 0 ][ c ]. A motion vector for a
macroblock partition mbPartIdx of a macroblock mbAddr that is not split into sub-macroblock partitions can also be
referred to as mvLX[ mbAddr ][ mbPartIdx ] with X being replaced by 0 or 1, which is identical to
mvLX[ mbAddr ][ mbPartIdx ][ 0 ]. All elements of the arrays mvL0 and mvL1 are initially set equal to 0.


- A (PicSizeInMbs)x(256 + 2 * MbWidthC * MbHeightC) array tCoeffLevel specifying transform coefficient level
values for the macroblocks of a layer representation. An element of this array for a macroblock with address mbAddr
and a transform coefficient index tCoeffIdx is referred to as tCoeffLevel[ mbAddr ][ tCoeffIdx ]. A one-dimensional
array specifying the transform coefficient level values for a macroblock with address mbAddr is referred to as
tCoeffLevel[ mbAddr ]. All elements of the array tCoeffLevel are initially set equal to 0.


- A (PicSizeInMbs)x(256 + 2 * MbWidthC * MbHeightC) array sTCoeff specifying scaled transform coefficient
values for the macroblocks of a layer representation. An element of this array for a macroblock with address mbAddr
and a transform coefficient index tCoeffIdx is referred to as sTCoeff[ mbAddr ][ tCoeffIdx ]. A one-dimensional array
specifying the scaled transform coefficient values for a macroblock with address mbAddr is referred to as
sTCoeff[ mbAddr ]. All elements of the array sTCoeff are initially set equal to 0.


- A (PicWidthInSamplesL)x(PicHeightInSamplesL) array rSL specifying residual luma sample values for a layer picture.
An element of this array for a luma location ( x, y ) relative to the upper-left luma sample of the macroblock with
address 0 is referred to as rSL[ x, y ]. All elements of the array rSL are initially set equal to 0.


- When ChromaArrayType is not equal to 0, two (PicWidthInSamplesC)x(PicHeightInSamplesC) arrays rSCb and rSCr
specifying residual chroma sample values for a layer picture. An element of these arrays for a chroma location ( x, y )
relative to the upper-left chroma sample of the macroblock with address 0 is referred to as rSCX[ x, y ] with CX being
replaced by Cb or Cr. All elements of the arrays rSCb and rSCr are initially set equal to 0.


- A (PicWidthInSamplesL)x(PicHeightInSamplesL) array cSL specifying constructed luma sample values for a layer
picture. An element of this array for a luma location ( x, y ) relative to the upper-left luma sample of the macroblock
with address 0 is referred to as cSL[ x, y ]. All elements of the array cSL are initially set equal to 0.


- When ChromaArrayType is not equal to 0, two (PicWidthInSamplesC)x(PicHeightInSamplesC) arrays cSCb and cSCr
specifying constructed chroma sample values for a layer picture. An element of these arrays for a chroma location
( x, y ) relative to the upper-left chroma sample of the macroblock with address 0 is referred to as cSCX[ x, y ] with
CX being replaced by Cb or Cr. All elements of the arrays cSCb and cSCr are initially set equal to 0.


**G.8.1.2.2** **Array restructuring process**


This process is only invoked when MinNoInterLayerPredFlag is equal to 0, SpatialResolutionChangeFlag is equal to 0,
and any of the variables ScaledRefLayerLeftOffset, ScaledRefLayerRightOffset, ScaledRefLayerTopOffset, or
ScaledRefLayerBottomOffset is not equal to 0.


Input to this process is a set of arrays collectively referred to as currentVars.


Output of this process is the set of arrays collectively referred to as currentVars with modifications related to the array
sizes as well as the ordering of array elements.


The variables that are assigned to the collective term currentVars are assigned to the collective term refLayerVars.


The array assignment and initialization process as specified in clause G.8.1.2.1 is invoked and the output is the set of arrays
collectively referred to as currentVars.


The variables xOffset, yOffset, xOffsetC, and yOffsetC are derived by





xOffset  = ScaledRefLayerLeftOffset (G-76)
yOffset  = ScaledRefLayerTopOffset / ( 1 + field_pic_flag ) (G-77)
xOffsetC = ( xOffset >> 4 ) * MbWidthC (G-78)
yOffsetC = ( yOffset >> 4 ) * MbHeightC (G-79)


For the macroblock address mbAddr proceeding over the values 0..(PicSizeInMbs − 1), the following ordered steps are
specified:


1. With eS set equal to (1 + MbaffFrameFlag), the variables refMbX and refMbY are derived by


refMbX = ( ( mbAddr / eS ) % PicWidthInMbs ) − ( xOffset / 16 ) (G-80)
refMbY = ( ( mbAddr / eS ) / PicWidthInMbs ) * eS + ( mbAddr % eS ) − ( yOffset / 16 ) (G-81)


2. The reference layer macroblock address refMbAddr is derived as follows:


     - If any of the following conditions are true, refMbAddr is marked as not available:


        - refMbX is less than 0 or refMbX is greater than or equal to RefLayerPicWidthInMbs,


        - refMbY is less than 0 or refMbY is greater than or equal to RefLayerPicHeightInMbs.


     - Otherwise, with bS set equal to (1 + RefLayerMbaffFrameFlag), refMbAddr is derived by


refMbAddr = ( refMbY / bS ) * bS * RefLayerPicWidthInMbs + ( refMbY % bS ) + refMbX (G-82)


3. When refMbAddr is available, for X being replaced by sliceIdc, fieldMbFlag, cTrafo, baseModeFlag, mbType,

subMbType, mvCnt, tQPY, tQPCb (when ChromaArrayType is not equal to 0), tQPCr (when ChromaArrayType
is not equal to 0), ipred4x4, ipred8x8, ipred16x16, ipredChroma (when ChromaArrayType is equal to 1 or 2),
predFlagL0, predFlagL1, refIdxL0, refIdxL1, mvL0, mvL1, tCoeffLevel, and sTCoeff and with currArray
representing the array X of the collective term currentVars and refLayerArray representing the array X of the
collective term refLayerVars, the array element currArray[ mbAddr ], which can be a scalar or an array, is set
equal to the array element refLayerArray[ refMbAddr ].


For X being replaced by rSL and cSL and with currArray representing the array X of the collective term currentVars and
refLayerArray representing the array X of the collective term refLayerVars, the array currArray is modified by


currArray[ x, y ] = refLayerArray[ x − xOffset, y − yOffset ]
(G-83)
with x = Max( 0, xOffset )..Min( PicWidthInSamplesL, RefLayerPicWidthInSamplesL + xOffset ) − 1
and y = Max( 0, yOffset )..Min( PicHeightInSamplesL, RefLayerPicHeightInSamplesL + yOffset ) − 1


When ChromaArrayType is not equal to 0, for X being replaced by rSCb, rSCr, cSCb, and cSCr and with currArray
representing the array X of the collective term currentVars and refLayerArray representing the array X of the collective
term refLayerVars, the array currArray is modified by


currArray[ x, y ] = refLayerArray[ x − xOffsetC, y − yOffsetC ]
(G-84)
with x = Max( 0, xOffsetC )..Min( PicWidthInSamplesC, RefLayerPicWidthInSamplesC + xOffsetC ) − 1
and y = Max( 0, yOffsetC )..Min( PicHeightInSamplesC, RefLayerPicHeightInSamplesC + yOffsetC ) − 1


**G.8.1.3** **Layer representation decoding processes**


Clause G.8.1.3.1 specifies the base decoding process for layer representations without resolution change.


Clause G.8.1.3.2 specifies the base decoding process for layer representations with resolution change.


Clause G.8.1.3.3 specifies the target layer representation decoding process


**G.8.1.3.1** **Base decoding process for layer representations without resolution change**


Inputs to this process are:


- a variable currDQId specifying the current layer representation,


- a set of arrays collectively referred to as currentVars.


Output of this process is the modified set of arrays collectively referred to as currentVars.


This process modifies the variables assigned to currentVars using syntax elements and derived upper-case variables for the
current layer representation with DQId equal to currDQId.


Unless stated otherwise, the syntax elements and derived upper-case variables that are referred to by the process specified
in this clause and all child processes invoked from this process are the syntax elements and derived upper-case variables
for the current layer representation with DQId equal to currDQId.





The base decoding process for layer representations without resolution change proceeds in the following ordered steps:


1. Depending on MinNoInterLayerPredFlag, the following applies:


     - If MinNoInterLayerPredFlag is equal to 1, the array assignment and initialization process as specified in
clause G.8.1.2.1 is invoked and the output is a modified set of arrays collectively referred to as currentVars.


     - Otherwise (MinNoInterLayerPredFlag is equal to 0), the following ordered steps are specified:


a. When MaxTCoeffLevelPredFlag is equal to 0, the macroblock address mbAddr proceeds over the
values 0..(RefLayerPicSizeInMbs − 1), and for each macroblock address mbAddr, the macroblock
decoding process prior to decoding a layer representation without resolution change and
MaxTCoeffLevelPredFlag equal to 0 as specified in clause G.8.1.5.4 is invoked with currDQId set equal
to MaxRefLayerDQId, mbAddr, and currentVars as the inputs and the output is a modified version of
currentVars.


b. When any of the variables ScaledRefLayerLeftOffset, ScaledRefLayerRightOffset,
ScaledRefLayerTopOffset, or ScaledRefLayerBottomOffset is not equal to 0, the array restructuring
process as specified in clause G.8.1.2.2 is invoked with currentVars as the input and the output is a
modified version of currentVars.


2. Let setOfSlices be the set of all slices of the current layer representation with DQId equal to currDQId. For each

slice of the set setOfSlices, the base decoding process for slices without resolution change as specified in
clause G.8.1.4.1 is invoked with currSlice representing the currently processed slice, currDQId, and currentVars
as the inputs and the output is a modified version of currentVars.


3. When currDQId is less than or equal to (DependencyIdMax << 4), with sliceIdc being the array sliceIdc of the

collective term currentVars, the bitstream shall not contain data that result in any value of
(sliceIdc[ mbAddr ] & 127) with mbAddr = 0..(PicSizeInMbs − 1) not equal to currDQId.

NOTE – This constraint and a similar constraint in clause G.8.1.3.2 specify that all layer representations with quality_id equal
to 0 and all layer representations that are used for inter-layer prediction must be completely covered by the slices of the access
unit. An additional constraint for layer representations with dependency_id equal to DependencyIdMax and quality_id greater
than 0 is specified in clause G.8.1.5.1.


**G.8.1.3.2** **Base decoding process for layer representations with resolution change**


Inputs to this process are:


- a variable currDQId specifying the current layer representation,


- a set of arrays collectively referred to as currentVars.


Outputs of this process are:


- a set of arrays collectively referred to as refLayerVars,


- the modified set of arrays collectively referred to as currentVars.


This process modifies the variables assigned to currentVars using syntax elements and derived upper-case variables for the
current layer representation with DQId equal to currDQId.


Unless stated otherwise, the syntax elements and derived upper-case variables that are referred to by the process specified
in this clause and all child processes invoked from this process are the syntax elements and derived upper-case variables
for the current layer representation with DQId equal to currDQId.


The base decoding process for layer representations with resolution change proceeds in the following ordered steps:


1. The macroblock address mbAddr proceeds over the values 0..(RefLayerPicSizeInMbs − 1), and for each

macroblock address mbAddr, the macroblock decoding process prior to resolution change as specified in
clause G.8.1.5.5 is invoked with currDQId set equal to MaxRefLayerDQId, mbAddr, and currentVars as the
inputs and the output is a modified version of currentVars.


2. The deblocking filter process for Intra_Base prediction as specified in clause G.8.7.1 is invoked with currDQId

and currentVars as the inputs and the output is a modified version of currentVars.


3. The variables that are assigned to the collective term currentVars are assigned to the collective term refLayerVars.


4. The array assignment and initialization process as specified in clause G.8.1.2.1 is invoked and the output is

assigned to the collective term currentVars.


5. Let setOfSlices be the set of all slices of the current layer representation with DQId equal to currDQId. For each

slice of the set setOfSlices, the base decoding process for slices with resolution change as specified in





clause G.8.1.4.2 is invoked with currSlice representing the currently processed slice, currDQId, refLayerVars,
and currentVars as the inputs and the output is a modified version of currentVars.


6. With sliceIdc being the array sliceIdc of the collective term currentVars, the bitstream shall not contain data that

result in any value of (sliceIdc[ mbAddr ] & 127) with mbAddr = 0..(PicSizeInMbs − 1) not equal to currDQId.

NOTE – This constraint and a similar constraint in clause G.8.1.3.1 specify that all layer representations with quality_id equal
to 0 and all layer representation that are used for inter-layer prediction must be completely covered by the slices of the access
unit. An additional constraint for layer representations with dependency_id equal to DependencyIdMax and quality_id greater
than 0 is specified in clause G.8.1.5.1.


**G.8.1.3.3** **Target layer representation decoding process**


Inputs to this process are:


- a variable currDQId specifying the current layer representation,


- when present, a set of arrays collectively referred to as refLayerVars,


- a set of arrays collectively referred to as currentVars.


Outputs of this process are:


- a (PicWidthInSamplesL)x(PicHeightInSamplesL) array sL containing constructed luma sample values,


- when ChromaArrayType is not equal to 0, two (PicWidthInSamplesC)x(PicHeightInSamplesC) arrays sCb and sCr
containing constructed chroma sample values.


In this process the constructed samples of the array sL and, when ChromaArrayType is not equal to 0, the arrays sCb and
sCr are derived using the variables that are assigned to currentVars.


Unless stated otherwise, the syntax elements and derived upper-case variables that are referred to by the process specified
in this clause and all child processes invoked from this process are the syntax elements and derived upper-case variables
for the current layer representation with DQId equal to currDQId.


The target layer representation decoding process proceeds in the following ordered steps:


1. The variables that are assigned to the collective term currentVars are assigned to the collective term tempVars,

and in the following of this clause, the arrays that are collectively referred to as tempVars are referred to by their
names as specified in clause G.8.1.2.1.

NOTE 1 – Any following modification of the variables assigned to the collective term tempVars does not influence the
variables assigned to the collective term currentVars.


2. The macroblock address mbAddr proceeds over the values 0..(PicSizeInMbs − 1), and for each macroblock

address mbAddr, the following ordered steps are specified:


a. Let currSlice specify the slice of the layer representation with DQId equal to
( ( ( sliceIdc[ mbAddr ] & 127 ) >> 4 ) << 4 ) that covers the macroblock with macroblock address
( ( sliceIdc[ mbAddr ] >> 7 ) * ( 1 + MbaffFrameFlag ) ).


b. Let firstMbInSlice and sliceType be the syntax elements first_mb_in_slice and slice_type of the slice

currSlice.


c. The variable firstMbAddrInSlice is set equal to (firstMbInSlice * ( 1 + MbaffFrameFlag )).


d. The reference picture lists refPicList0 and refPicList1 are marked as not available.


e. When (sliceType % 5) is less than 2, the following applies:


        - If mbAddr is greater than firstMbAddrInSlice, the reference picture list refPicList0 is set equal to the
reference picture list refPicList0 that was derived for the macroblock address mbAddr equal to
firstMbAddrInSlice inside this clause and, when (sliceType % 5) is equal to 1, the reference picture list
refPicList1 is set equal to the reference picture list refPicList1 that was derived for the macroblock
address mbAddr equal to firstMbAddrInSlice inside this clause.


        - Otherwise (mbAddr is equal to firstMbAddrInSlice), the SVC decoding process for reference picture
lists construction as specified in clause G.8.2.3 is invoked with currDependencyId set equal to
dependency_id, useRefBasePicFlag set equal to use_ref_base_pic_flag, and the slice currSlice as the
inputs and the outputs are the modified reference picture list refPicList0 and, when (sliceType % 5) is
equal to 1, the modified reference picture list refPicList1.

NOTE 2 – The reference picture lists refPicList0 and refPicList1 are only constructed for the slices of the layer
representation with dependency_id equal to DependencyIdMax and quality_id equal to 0. For slices with
dependency_id equal to DependencyIdMax and quality_id greater than 0, the reference picture lists are inferred.





f. The target macroblock decoding process as specified in clause G.8.1.5.6 is invoked with currDQId, mbAddr,
refLayerVars (when present as input to this clause), tempVars, refPicList0 (when available), and refPicList1
(when available) as the inputs and the output is a modified version of tempVars.

NOTE 3 – Although the target layer representation decoding process is invoked twice for pictures with
store_ref_base_pic_flag equal to 1, only a single motion compensation operation is needed for each macroblock.


3. The deblocking filter process for target representations as specified in clause G.8.7.2 is invoked with currDQId

and tempVars as the inputs and the output is a modified version of tempVars.


4. All sample values of the array cSL are copied to the array sL, which is output of this clause.


5. When ChromaArrayType is not equal to 0, all sample values of the arrays cSCb and cSCr are copied to the arrays

sCb and sCr, respectively, which are output of this clause.


**G.8.1.4** **Slice decoding processes**


Clause G.8.1.4.1 specifies the base decoding process for slices without resolution change.


Clause G.8.1.4.2 specifies the base decoding process for slices with resolution change.


**G.8.1.4.1** **Base decoding process for slices without resolution change**


Inputs to this process are:


- the current slice currSlice,


- a variable currDQId specifying the current layer representation,


- a set of arrays collectively referred to as currentVars.


Output of this process is the modified set of arrays collectively referred to as currentVars.


This process modifies the variables assigned to currentVars using syntax elements and derived upper-case variables for the
current slice currSlice.


Unless stated otherwise, the syntax elements and derived upper-case variables that are referred to by the process specified
in this clause and all child processes invoked from this process are the syntax elements and derived upper-case variables
for the slice header of the current slice currSlice, the current picture parameter, which is identified by the syntax element
pic_parameter_set_id inside the slice header of the current slice currSlice, and the current sequence parameter, which is
identified by the syntax element seq_parameter_set_id inside the current picture parameter set.


When currDQId is equal to 0 and (slice_type % 5) is equal to 1, the SVC decoding process for reference picture lists
construction as specified in clause G.8.2.3 is invoked with currDependencyId equal to 0, useRefBasePicFlag equal to
use_ref_base_pic_flag, and the current slice as input and the output is the reference picture list refPicList1.


The macroblocks of the current slice currSlice are processed in increasing order of their macroblock addresses. For each
macroblock with macroblock address mbAddr, the base decoding process for macroblocks in slices without resolution
change as specified in clause G.8.1.5.2 is invoked with currDQId, mbAddr, currentVars, and, when currDQId is equal to
0 and (slice_type % 5) is equal to 1, the reference picture list refPicList1 as the inputs and the output is a modified version
of currentVars.


**G.8.1.4.2** **Base decoding process for slices with resolution change**


Inputs to this process are:


- the current slice currSlice,


- a variable currDQId specifying the current layer representation,


- a set of arrays collectively referred to as refLayerVars,


- a set of arrays collectively referred to as currentVars.


Output of this process is the modified set of arrays collectively referred to as currentVars.


This process modifies the variables assigned to currentVars using syntax elements and derived upper-case variables for the
current slice currSlice.


Unless stated otherwise, the syntax elements and derived upper-case variables that are referred to by the process specified
in this clause and all child processes invoked from this process are the syntax elements and derived upper-case variables
for the slice header of the current slice currSlice, the current picture parameter, which is identified by the syntax element
pic_parameter_set_id inside the slice header of the current slice currSlice, and the current sequence parameter, which is
identified by the syntax element seq_parameter_set_id inside the current picture parameter set.





When CroppingChangeFlag is equal to 1 and (slice_type % 5) is less than 2, the SVC decoding process for reference
picture lists construction as specified in clause G.8.2.3 is invoked with currDependencyId equal to dependency_id,
useRefBasePicFlag equal to use_ref_base_pic_flag, and the current slice as the inputs and the outputs are the reference
picture list refPicList0 and, when (sliceType % 5) is equal to 1, the reference picture list refPicList1.


The macroblocks of the current slice currSlice are processed in increasing order of their macroblock addresses. For each
macroblock with macroblock address mbAddr, the base decoding process for macroblocks in slices with resolution change
as specified in clause G.8.1.5.3 is invoked with currDQId, mbAddr, refLayerVars, currentVars, refPicList0 (when
CroppingChangeFlag is equal to 1 and (slice_type % 5) is less than 2), and refPicList1 (when CroppingChangeFlag is
equal to 1 and (slice_type % 5) is equal to 1) as the inputs and the output is a modified version of currentVars.


**G.8.1.5** **Macroblock initialization and decoding processes**


Clause G.8.1.5.1 specifies the macroblock initialization process.


Clause G.8.1.5.2 specifies the base decoding process for macroblocks in slices without resolution change.


Clause G.8.1.5.3 specifies the base decoding process for macroblocks in slices with resolution change.


Clause G.8.1.5.4 specifies the macroblock decoding process prior to decoding a layer representation without resolution
change and MaxTCoeffLevelPredFlag equal to 0.


Clause G.8.1.5.5 specifies the macroblock decoding process prior to resolution change.


Clause G.8.1.5.6 specifies the target macroblock decoding process.


**G.8.1.5.1** **Macroblock initialization process**


Inputs to this process are:


- a set of arrays collectively referred to as refLayerVars,


- when CroppingChangeFlag is equal to 1 and (slice_type % 5) is less than 2, the reference picture list refPicList0,


- when CroppingChangeFlag is equal to 1 and (slice_type % 5) is equal to 1, the reference picture list refPicList1.


Outputs of this process are:


- a variable sliceIdc specifying the slice identification for the current macroblock,


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a variable cTrafo specifying the transform type for the current macroblock,


- a variable baseModeFlag specifying the syntax element base_mode_flag of the current macroblock,


- a variable mbType specifying the macroblock type of the current macroblock,


- a list subMbType with 4 elements specifying the sub-macroblock types of the current macroblock,


- a variable mvCnt specifying an initialization value for the motion vector count of the current macroblock,


- a variable tQPY specifying the luma quantization parameter for the current macroblock,


- when ChromaArrayType is not equal to 0, two variables tQPCb and tQPCr specifying the chroma quantization
parameters for the current macroblock,


- two 2x2 arrays refIdxILPredL0 and refIdxILPredL1 specifying inter-layer predictors for the reference indices of the
current macroblock,


- two 4x4x2 arrays mvILPredL0 and mvILPredL1 specifying inter-layer predictors for the motion vector components
of the current macroblock.


Inside this clause, the arrays sliceIdc, fieldMbFlag, cTrafo, mbType, subMbType, tQPY, predFlagL0, predFlagL1,
refIdxL0, refIdxL1, mvL0, mvL1, tCoeffLevel, and sTCoeff that are collectively referred to as refLayerVars are referred
to as refLayerSliceIdc, refLayerFieldMbFlag, refLayerCTrafo, refLayerMbType, refLayerSubMbType, refLayerQPY,
refLayerPredFlagL0, refLayerPredFlagL1, refLayerRefIdxL0, refLayerRefIdxL1, refLayerMvL0, refLayerMvL1,
refLayerTCoeffLevel, and refLayerSTCoeff, respectively.


The variable sliceIdc is set equal to ( (first_mb_in_slice << 7 ) + DQId ).


The variable baseModeFlag is set equal to base_mode_flag.


The variable fieldMbFlag is derived as follows:





- If field_pic_flag is equal to 1, fieldMbFlag is set equal to 1.


- Otherwise, if SpatialResolutionChangeFlag is equal to 0 and slice_skip_flag is equal to 1, fieldMbFlag is set equal to
refLayerFieldMbFlag[ CurrMbAddr ].


- Otherwise, fieldMbFlag is set equal to mb_field_decoding_flag.


The derivation process for macroblock type, sub-macroblock type, and inter-layer predictors for reference indices and
motion vectors as specified in clause G.8.1.5.1.1 is invoked with fieldMbFlag, refLayerFieldMbFlag, refLayerMbType,
refLayerSubMbType, refLayerPredFlagL0, refLayerPredFlagL1, refLayerRefIdxL0, refLayerRefIdxL1, refLayerMvL0,
refLayerMvL1, refPicList0 (when available), and refPicList1 (when available) as the inputs and the outputs are the variable
mbType, the list subMbType, the 2x2 arrays refIdxILPredL0 and refIdxILPredL1, and the 4x4x2 arrays mvILPredL0 and
mvILPredL1.


The derivation process for quantization parameters and transform type as specified in clause G.8.1.5.1.2 is invoked with
mbType, subMbType, refLayerMbType, refLayerCTrafo, refLayerQPY, refLayerTCoeffLevel, and refLayerSTCoeff as
the inputs and the outputs are cTrafo, tQPY, and, when ChromaArrayType is not equal to 0, tQPCb and tQPCr.


The variable mvCnt is set equal to 0.


When quality_id is greater than 0, the bitstream shall not contain data that result in
(refLayerSliceIdc[ CurrMbAddr ] & 127) not equal to (DQId − 1).


When no_inter_layer_pred_flag is equal to 0, SpatialResolutionChangeFlag is equal to 0, and fieldMbFlag is not equal to
refLayerFieldMbRef[ CurrMbAddr ], the following constraints shall be obeyed:


a) The bitstream shall not contain data that result in base_mode_flag equal to 1, or any
motion_prediction_flag_lX[ mbPartIdx ] with X being replaced by 0 and 1 and mbPartIdx = 0..3 equal to 1.


b) When residual_prediction_flag is equal to 1, refLayerMbType[ CurrMbAddr ] is not equal to I_PCM, I_16x16,

I_8x8, I_4x4, or I_BL, and mbType is not equal to I_PCM, I_16x16, I_8x8, I_4x4, or I_BL, the following applies:


     - If tcoeff_level_prediction_flag is equal to 0, the bitstream shall not contain data that result in any element
refLayerSTCoeff[ CurrMbAddr ][ i ] not equal to 0 for i = 0..(255 + 2 * MbWidthC * MbHeightC).


     - Otherwise (tcoeff_level_prediction_flag is equal to 1), the bitstream shall not contain data that result in any
element refLayerTCoeffLevel[ CurrMbAddr ][ i ] not equal to 0 for
i = 0..(255 + 2 * MbWidthC * MbHeightC).


**G.8.1.5.1.1** **Derivation process for macroblock type, sub-macroblock type, and inter-layer predictors for reference**
**indices and motion vectors**


Inputs to this process are:


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a one-dimensional array refLayerFieldMbFlag with RefLayerPicSizeInMbs elements specifying which macroblocks
of the reference layer representation are field macroblocks and which macroblocks are frame macroblocks,


- a one-dimensional array refLayerMbType with RefLayerPicSizeInMbs elements specifying the macroblock types for
the macroblocks of the reference layer representation,


- a (RefLayerPicSizeInMbs)x4 array refLayerSubMbType specifying the sub-macroblock types for the macroblocks of
the reference layer representation,


- two (RefLayerPicSizeInMbs)x4 arrays refLayerPredFlagL0 and refLayerPredFlagL1 specifying prediction utilization
flags for the macroblocks of the reference layer representation,


- two (RefLayerPicSizeInMbs)x4 arrays refLayerRefIdxL0 and refLayerRefIdxL1 specifying reference indices for the
macroblocks of the reference layer representation,


- two (RefLayerPicSizeInMbs)x4x4x2 arrays refLayerMvL0 and refLayerMvL1 specifying motion vector components
for the macroblocks of the reference layer representation,


- when CroppingChangeFlag is equal to 1 and (slice_type % 5) is less than 2, the reference picture list refPicList0,


- when CroppingChangeFlag is equal to 1 and (slice_type % 5) is equal to 1, the reference picture list refPicList1.


Outputs of this process are:


- a variable mbType specifying the macroblock type of the current macroblock,


- a list subMbType with 4 elements specifying the sub-macroblock types of the current macroblock,





- two 2x2 arrays refIdxILPredL0 and refIdxILPredL1 specifying inter-layer predictors for the reference indices of the
current macroblock,


- two 4x4x2 arrays mvILPredL0 and mvILPredL1 specifying inter-layer predictors for the motion vector components
of the current macroblock.


The variable mbTypeILPred, the list subMbTypeILPred, the 2x2 arrays refIdxILPredL0 and refIdxILPredL1, and the
4x4x2 arrays mvILPredL0 and mvILPredL1 are derived as follows:


- If base_mode_flag is equal to 1 or any syntax element motion_prediction_flag_lX[ mbPartIdx ] with X being replaced
by 0 and 1 and mbPartIdx = 0..3 is equal to 1, the derivation process for inter-layer predictors for macroblock type,
sub-macroblock type, reference indices, and motion vectors as specified in clause G.8.6.1 is invoked with
fieldMbFlag, refLayerFieldMbFlag, refLayerMbType, refLayerSubMbType, refLayerPredFlagL0,
refLayerPredFlagL1, refLayerRefIdxL0, refLayerRefIdxL1, refLayerMvL0, refLayerMvL1, refPicList0 (when
available), and refPicList1 (when available) as the inputs and the outputs are the variable mbTypeILPred, the list
subMbTypeILPred, the 2x2 arrays refIdxILPredL0 and refIdxILPredL1, and the 4x4x2 arrays mvILPredL0 and
mvILPredL1.


- Otherwise (base_mode_flag is equal to 0 and all syntax elements motion_prediction_flag_lX[ mbPartIdx ] with X
being replaced by 0 and 1 and mbPartIdx = 0..3 are equal to 0), mbTypeILPred is marked as not available, all elements
subMbTypeILPred[ mbPartIdx ] with mbPartIdx = 0..3 of the list subMbTypeILPred are marked as not available, all
elements of the 2x2 arrays refIdxILPredL0 and refIdxILPredL1 are set equal to −1, and all elements of the 4x4x2
arrays mvILPredL0 and mvILPredL1 are set equal to 0.


Depending on base_mode_flag, mb_type, SpatialResolutionChangeFlag, refLayerMbType[ CurrMbAddr ],
CodedBlockPatternLuma, and CodedBlockPatternChroma, the variable mbType is derived as follows:


- If base_mode_flag is equal to 1, the following applies:


   - If SpatialResolutionChangeFlag is equal to 0, refLayerMbType[ CurrMbAddr ] is equal to I_PCM,
CodedBlockPatternLuma is equal to 0, and CodedBlockPatternChroma is equal to 0, mbType is set equal to
I_PCM.


   - Otherwise (SpatialResolutionChangeFlag is equal to 1, refLayerMbType[ CurrMbAddr ] is not equal to I_PCM,
CodedBlockPatternLuma is not equal to 0, or CodedBlockPatternChroma is not equal to 0), mbType is set equal
to mbTypeILPred.


- Otherwise, if MbPartPredMode( mb_type, 0 ) is equal to Intra_4x4, mbType is set equal to I_4x4.


- Otherwise, if MbPartPredMode( mb_type, 0 ) is equal to Intra_8x8, mbType is set equal to I_8x8.


- Otherwise, if MbPartPredMode( mb_type, 0 ) is equal to Intra_16x16, mbType is set equal to I_16x16.


- Otherwise, if mb_type is equal to I_PCM, mbType is set equal to I_PCM.


- Otherwise (base_mode_flag is equal to 0 and mb_type specifies a P or B macroblock type), mbType is set equal to
mb_type.


Depending on mbType and base_mode_flag, the list subMbType is derived as follows:


- If mbType is not equal to P_8x8, P_8x8ref0, or B_8x8, all elements subMbType[ mbPartIdx ] with mbPartIdx = 0..3
are marked as unspecified.


- Otherwise, if base_mode_flag is equal to 1, each element subMbType[ mbPartIdx ] with mbPartIdx = 0..3 is set equal
to subMbTypeILPred[ mbPartIdx ].


- Otherwise (mbType is equal to P_8x8, P_8x8ref0, or B_8x8 and base_mode_flag is equal to 0), each element
subMbType[ mbPartIdx ] with mbPartIdx = 0..3 is set equal to sub_mb_type[ mbPartIdx ].


When slice_type is equal to EP, base_mode_flag is equal to 1, and mbType is not equal to I_PCM, I_16x16, I_8x8, I_4x4,
or I_BL, the bitstream shall not contain data that result in any element refIdxILPredL0[ mbPartIdx ] with mbPartIdx = 0..3
that is less than 0.


When residual_prediction_flag equal to 1 is present in the bitstream, the bitstream shall not contain data that result in
mbType being equal to I_PCM, I_16x16, I_8x8, I_4x4, or I_BL.


**G.8.1.5.1.2** **Derivation process for quantization parameters and transform type**


Inputs to this process are:


- a variable mbType specifying the macroblock type for the current macroblock,


- a list subMbType with 4 elements specifying the sub-macroblock types for the current macroblock,





- a one-dimensional array refLayerMbType specifying macroblock types for the macroblocks of the reference layer
representation,


- a one-dimensional array refLayerCTrafo specifying transform types for the macroblocks of the reference layer
representation,


- a one-dimensional array refLayerQPY specifying luma quantization parameters for the macroblocks of the reference
layer representation,


- an (RefLayerPicSizeInMbs)x(256 + 2 * MbWidthC * MbHeightC) array refLayerTCoeffLevel specifying transform
coefficient level values for the macroblocks of the reference layer representation,


- an (RefLayerPicSizeInMbs)x(256 + 2 * MbWidthC * MbHeightC) array refLayerSTCoeff specifying scaled
transform coefficient values for the macroblocks of the reference layer representation.


Outputs of this process are:


- a variable cTrafo specifying the transform type for the current macroblock,


- a variable tQPY specifying the luma quantization parameter for the current macroblock,


- when ChromaArrayType is not equal to 0, two variables tQPCb and tQPCr specifying the chroma quantization
parameters for the current macroblock.


The variable tQPY is derived as follows:


- If SpatialResolutionChangeFlag is equal to 0, and any of the following conditions are true, tQPY is set equal to
refLayerQPY[ CurrMbAddr ]:


   - mbType is equal to I_PCM, I_16x16, I_8x8, I_4x4, or I_BL, base_mode_flag is equal to 1,
CodedBlockPatternLuma is equal to 0, and CodedBlockPatternChroma is equal to 0,


   - mbType is equal to P_Skip or B_Skip and residual_prediction_flag is equal to 1,


   - mbType is not equal to I_PCM, I_16x16, I_8x8, I_4x4, I_BL, P_Skip, or B_Skip, residual_prediction_flag is
equal to 1, CodedBlockPatternLuma is equal to 0, and CodedBlockPatternChroma is equal to 0.


- Otherwise, tQPY is set equal to QPY.


When ChromaArrayType is not equal to 0, for CX being replaced by Cb and Cr, the variable tQPCX is set equal to the value
of QPCX that corresponds to a value of tQPY for QPY as specified in clause 8.5.8.


The variable predTrafoFlag is derived as follows:


- If SpatialResolutionChangeFlag is equal to 0 and any of the following conditions are true, predTrafoFlag is set equal
to 1:


   - base_mode_flag is equal to 1, tcoeff_level_prediction_flag is equal to 0, refLayerMbType[ CurrMbAddr ] is
equal to I_BL, and CodedBlockPatternLuma is equal to 0,


   - base_mode_flag is equal to 1, tcoeff_level_prediction_flag is equal to 0, refLayerMbType[ CurrMbAddr ] is
equal to I_PCM, CodedBlockPatternLuma is equal to 0, and CodedBlockPatternChroma is equal to 0,


   - base_mode_flag is equal to 1, tcoeff_level_prediction_flag is equal to 0, refLayerMbType[ CurrMbAddr ] is
equal to I_8x8 or I_4x4, and CodedBlockPatternLuma is equal to 0,


   - base_mode_flag is equal to 1, tcoeff_level_prediction_flag is equal to 1, and mbType is equal to I_PCM,
I_16x16, I_8x8, or I_4x4,


   - residual_prediction_flag is equal to 1, refLayerMbType[ CurrMbAddr ] is not equal to I_PCM, I_16x16, I_8x8,
I_4x4, or I_BL, mbType is not equal to I_PCM, I_16x16, I_8x8, I_4x4, or I_BL, and CodedBlockPatternLuma
is equal to 0.


- Otherwise, predTrafoFlag is set equal to 0.


The variable cTrafo is derived as follows:


- If mbType is equal to I_PCM, cTrafo is set equal to T_PCM.


- Otherwise, if mbType is equal to I_16x16, cTrafo is set equal to T_16x16.


- Otherwise, if mbType is equal to I_8x8 or transform_size_8x8_flag is equal to 1, cTrafo is set equal to T_8x8.


- Otherwise, if predTrafoFlag is equal to 1, cTrafo is set equal to refLayerCTrafo[ CurrMbAddr ].





- Otherwise (predTrafoFlag is equal to 0, transform_size_8x8_flag is equal to 0, and mbType is not equal to I_PCM,
I_16x16, or I_8x8), cTrafo is set equal to T_4x4.


When cTrafo is equal to T_8x8, the SVC sequence parameter set that is referred to by the coded slice NAL unit (via
pic_parameter_set_id in the slice header and seq_parameter_set_id in referenced the picture parameter set) shall have
transform_8x8_mode_flag equal to 1.


When base_mode_flag is equal to 1, the following constraints shall be obeyed:


a) When mbType is equal to P_8x8, P_8x8ref0, or B8x8 and NumSubMbPart( subMbType[ mbPartIdx ] ) is not

equal to 1 for any mbPartIdx = 0..3, the bitstream shall not contain transform_size_8x8_flag equal to 1.


b) When mbType is equal to I_PCM, the bitstream shall not contain data that result in CodedBlockPatternLuma not

equal to 0 or CodedBlockPatternChroma not equal to 0.


c) When mbType is equal to I_16x16 or I_4x4, the bitstream shall not contain transform_size_8x8_flag equal to 1.


d) When mbType is equal to I_8x8 and transform_size_8x8_flag is equal to 0, the bitstream shall not contain data

that result in CodedBlockPatternLuma not equal to 0.


The variable constrainedCoeffFlag is derived as follows:


- If SpatialResolutionChangeFlag is equal to 0 and any of the following conditions are true, constrainedCoeffFlag is
set equal to 1:


   - base_mode_flag is equal to 1, tcoeff_level_prediction_flag is equal to 0, and refLayerMbType[ CurrMbAddr ]
is equal to I_BL,


   - residual_prediction_flag is equal to 1, refLayerMbType[ CurrMbAddr ] is not equal to I_PCM, I_16x16, I_8x8,
I_4x4, or I_BL, and mbType is not equal to I_PCM, I_16x16, I_8x8, I_4x4, or I_BL.


- Otherwise, constrainedCoeffFlag is set equal to 0.


When constrainedCoeffFlag is equal to 1, the following constraints shall be obeyed:


a) When refLayerCTrafo[ CurrMbAddr ] is equal to T_8x8 and transform_size_8x8_flag is equal 0, any of the

following constraints shall be obeyed:


i) The bitstream shall not contain data that result in CodedBlockPatternLuma not equal to 0.


ii) Depending on tcoeff_level_prediction_flag, the following applies:


         - If tcoeff_level_prediction_flag is equal to 0, the bitstream shall not contain data that result in any
element refLayerSTCoeff[ CurrMbAddr ][ i ] not equal to 0 for
i = 0..( ( ChromaArrayType != 3 ) ? 255 : 767).


         - Otherwise (tcoeff_level_prediction_flag is equal to 1), the bitstream shall not contain data that
result in any element refLayerTCoeffLevel[ CurrMbAddr ][ i ] not equal to 0 for
i = 0..( ( ChromaArrayType != 3 ) ? 255 : 767).


b) When refLayerCTrafo[ CurrMbAddr ] is equal to T_4x4 and transform_size_8x8_flag equal to 1, the following

applies:


     - If tcoeff_level_prediction_flag is equal to 0, the bitstream shall not contain data that result in any element
refLayerSTCoeff[ CurrMbAddr ][ i ] not equal to 0 for i = 0..( ( ChromaArrayType != 3 ) ? 255 : 767).


     - Otherwise (tcoeff_level_prediction_flag is equal to 1), the bitstream shall not contain data that result in
any element refLayerTCoeffLevel[ CurrMbAddr ][ i ] not equal to 0 for
i = 0..( ( ChromaArrayType != 3 ) ? 255 : 767).


**G.8.1.5.2** **Base decoding process for macroblocks in slices without resolution change**


Inputs to this process are:


- a variable currDQId specifying the current layer representation,


- a variable mbAddr specifying the current macroblock inside the current layer representation,


- a set of arrays collectively referred to as currentVars,


- when currDQId is equal to 0 and (slice_type % 5) is equal to 1, the reference picture list refPicList1.


Output of this process is the modified set of arrays collectively referred to as currentVars.





This process modifies the variables assigned to currentVars using syntax elements and derived upper-case variables for the
current macroblock, which is specified by its address mbAddr and the layer representation identifier currDQId.


Unless stated otherwise, the syntax elements and derived upper-case variables that are referred to by the process specified
in this clause and all child processes invoked from this process are the syntax elements and derived upper-case variables
for the current macroblock, which is the macroblock with address mbAddr inside the layer representation with DQId equal
to currDQId, the slice header of the current slice, which is the slice that contains the current macroblock, the current picture
parameter, which is identified by the syntax element pic_parameter_set_id inside the slice header of the current slice, and
the current sequence parameter, which is identified by the syntax element seq_parameter_set_id inside the current picture
parameter set.


Inside this clause, the arrays that are collectively referred to as currentVars are referred to by their names as specified in
clause G.8.1.2.1.


The base decoding process for macroblocks in slices without resolution change is specified by the following ordered steps:


1. The variable CurrMbAddr is set equal to mbAddr.


2. When tcoeff_level_prediction_flag is equal to 1, the variable refQPY is set equal to tQPY[ mbAddr ] and, when

ChromaArrayType is not equal to 0, the variables refQPCb and refQPCr are set equal to tQPCb[ mbAddr ] and
tQPCr[ mbAddr ], respectively.


3. When no_inter_layer_pred_flag is equal to 0, the variable refLayerIntraBLFlag is derived as follows:


      - If mbType[ mbAddr ] is equal to I_BL, refLayerIntraBLFlag is set equal to 1.


      - Otherwise (mbType[ mbAddr ] is not equal to I_BL), refLayerIntraBLFlag is set equal to 0.


4. The variable resPredFlag is derived as follows:


      - If residual_prediction_flag is equal to 1 and mbType[ mbAddr ] is not equal to I_PCM, I_16x16, I_8x8,
I_4x4, or I_BL, resPredFlag is set equal to 1.


      - Otherwise (residual_prediction_flag is equal to 0 or mbType[ mbAddr ] is equal to I_PCM, I_16x16, I_8x8,
I_4x4, or I_BL), resPredFlag is set equal to 0.


5. The macroblock initialization process as specified in clause G.8.1.5.1 is invoked with refLayerVars set equal to

currentVars as the input and the outputs are assigned to sliceIdc[ mbAddr ], fieldMbFlag[ mbAddr ],
cTrafo[ mbAddr ], baseModeFlag[ mbAddr ], mbType[ mbAddr ], subMbType[ mbAddr ], mvCnt[ mbAddr ],
tQPY[ mbAddr ], tQPCb[ mbAddr ] (when ChromaArrayType is not equal to 0), tQPCr[ mbAddr ] (when
ChromaArrayType is not equal to 0), the 2x2 arrays refIdxILPredL0 and refIdxILPredL1, and the 4x4x2 arrays
mvILPredL0 and mvILPredL1.


6. The SVC derivation process for motion vector components and reference indices as specified in clause G.8.4.1

is invoked with sliceIdc, fieldMbFlag, mbType, subMbType, predFlagL0, predFlagL1, refIdxL0, refIdxL1,
mvL0, mvL1, mvCnt, refIdxILPredL0, refIdxILPredL1, mvILPredL0, mvILPredL1, and refPicList1 (when
available) as the inputs and the outputs are modified versions of the arrays predFlagL0, predFlagL1, refIdxL0,
refIdxL1, mvL0, mvL1, and mvCnt.


7. Depending on mbType[ mbAddr ], the following applies:


     - If mbType[ mbAddr ] is equal to I_PCM, I_16x16, I_8x8, or I_4x4, the following ordered steps are
specified:


a. When base_mode_flag is equal to 0, the SVC derivation process for intra prediction modes as specified
in clause G.8.3.1 is invoked with sliceIdc, fieldMbFlag, baseModeFlag, mbType, ipred4x4, ipred8x8,
ipred16x16, and, when ChromaArrayType is equal to 1 or 2, ipredChroma as the inputs and the outputs
are modified versions of ipred4x4, ipred8x8, ipred16x16, and, when ChromaArrayType is equal to 1
or 2, ipredChroma.


b. When tcoeff_level_prediction_flag is equal to 1 and base_mode_flag is equal to 1, the transform

coefficient level scaling process prior to transform coefficient refinement as specified in clause G.8.5.2
is invoked with cTrafo[ mbAddr ], tCoeffLevel[ mbAddr ], tQPY[ mbAddr ], refQPY, and, when
ChromaArrayType is not equal to 0, tQPCb[ mbAddr ], tQPCr[ mbAddr ], refQPCb, and refQPCr as the
inputs and the output is a modified version of tCoeffLevel[ mbAddr ].


c. The transform coefficient scaling and refinement process as specified in clause G.8.5.1 is invoked with
refinementFlag set equal to base_mode_flag, fieldMbFlag[ mbAddr ], mbType[ mbAddr ],
cTrafo[ mbAddr ], sTCoeff[ mbAddr ], tCoeffLevel[ mbAddr ], tQPY[ mbAddr ], and, when
ChromaArrayType is not equal to 0, tQPCb[ mbAddr ] and tQPCr[ mbAddr ] as the inputs and the outputs
are modified versions of sTCoeff[ mbAddr ] and tCoeffLevel[ mbAddr ].





d. The sample array re-initialization process as specified in clause G.8.5.5 is invoked with

fieldMbFlag[ mbAddr ], rSL, and, when ChromaArrayType is not equal to 0, rSCb and rSCr as the inputs
and the outputs are a modified version of rSL and, when ChromaArrayType is not equal to 0, modified
versions of rSCb and rSCr.


e. The sample array re-initialization process as specified in clause G.8.5.5 is invoked with
fieldMbFlag[ mbAddr ], cSL, and, when ChromaArrayType is not equal to 0, cSCb and cSCr as the inputs
and the outputs are a modified version of cSL and, when ChromaArrayType is not equal to 0, modified
versions of cSCb and cSCr.


     - Otherwise, if mbType[ mbAddr ] is equal to I_BL, the transform coefficient scaling and refinement process
as specified in clause G.8.5.1 is invoked with refinementFlag set equal to refLayerIntraBLFlag,
fieldMbFlag[ mbAddr ], mbType[ mbAddr ], cTrafo[ mbAddr ], sTCoeff[ mbAddr ],
tCoeffLevel[ mbAddr ], tQPY[ mbAddr ], and, when ChromaArrayType is not equal to 0, tQPCb[ mbAddr ]
and tQPCr[ mbAddr ] as the inputs and the outputs are modified versions of sTCoeff[ mbAddr ] and
tCoeffLevel[ mbAddr ].


     - Otherwise (mbType[ mbAddr ] is not equal to I_PCM, I_16x16, I_8x8, I_4x4, or I_BL), the following
ordered steps are specified:


a. When tcoeff_level_prediction_flag is equal to 1 and resPredFlag is equal to 1, the transform coefficient
level scaling process prior to transform coefficient refinement as specified in clause G.8.5.2 is invoked
with cTrafo[ mbAddr ], tCoeffLevel[ mbAddr ], tQPY[ mbAddr ], refQPY, and, when
ChromaArrayType is not equal to 0, tQPCb[ mbAddr ], tQPCr[ mbAddr ], refQPCb, and refQPCr as the
inputs and the output is a modified version of tCoeffLevel[ mbAddr ].


b. The transform coefficient scaling and refinement process as specified in clause G.8.5.1 is invoked with

refinementFlag set equal to resPredFlag, fieldMbFlag[ mbAddr ], mbType[ mbAddr ],
cTrafo[ mbAddr ], sTCoeff[ mbAddr ], tCoeffLevel[ mbAddr ], tQPY[ mbAddr ], and, when
ChromaArrayType is not equal to 0, tQPCb[ mbAddr ] and tQPCr[ mbAddr ] as the inputs and the outputs
are modified versions of sTCoeff[ mbAddr ] and tCoeffLevel[ mbAddr ].


c. When resPredFlag is equal to 0, the sample array re-initialization process as specified in clause G.8.5.5
is invoked with fieldMbFlag[ mbAddr ], rSL, and, when ChromaArrayType is not equal to 0, rSCb and
rSCr as the inputs and the outputs are a modified versions of rSL and, when ChromaArrayType is not
equal to 0, modified versions of rSCb and rSCr.


d. When resPredFlag is equal to 0, the sample array re-initialization process as specified in clause G.8.5.5

is invoked with fieldMbFlag[ mbAddr ], cSL, and, when ChromaArrayType is not equal to 0, cSCb and
cSCr as the inputs and the outputs are a modified versions of cSL and, when ChromaArrayType is not
equal to 0, modified versions of cSCb and cSCr.


8. The variable MvCnt for the macroblock mbAddr is set equal to mvCnt[ mbAddr ].


**G.8.1.5.3** **Base decoding process for macroblocks in slices with resolution change**


Inputs to this process are:


- a variable currDQId specifying the current layer representation,


- a variable mbAddr specifying the current macroblock inside the current layer representation,


- a set of arrays collectively referred to as refLayerVars,


- a set of arrays collectively referred to as currentVars,


- when CroppingChangeFlag is equal to 1 and (slice_type % 5) is less than 2, the reference picture list refPicList0,


- when CroppingChangeFlag is equal to 1 and (slice_type % 5) is equal to 1, the reference picture list refPicList1.


Output of this process is the modified set of arrays collectively referred to as currentVars.


This process modifies the variables assigned to currentVars using syntax elements and derived upper-case variables for the
current macroblock, which is specified by its address mbAddr and the layer representation identifier currDQId, as well as
variables assigned to refLayerVars.


Unless stated otherwise, the syntax elements and derived upper-case variables that are referred to by the process specified
in this clause and all child processes invoked from this process are the syntax elements and derived upper-case variables
for the current macroblock, which is the macroblock with address mbAddr inside the layer representation with DQId equal
to currDQId, the slice header of the current slice, which is the slice that contains the current macroblock, the current picture
parameter, which is identified by the syntax element pic_parameter_set_id inside the slice header of the current slice, and





the current sequence parameter, which is identified by the syntax element seq_parameter_set_id inside the current picture
parameter set.


Inside this clause, the arrays that are collectively referred to as currentVars are referred to by their names as specified in
clause G.8.1.2.1.


Inside this clause, the arrays sliceIdc, fieldMbFlag, cTrafo, mbType, cSL, cSCb, cSCr, rSL, rSCb, and rSCr of the collective
term refLayerVars are referred to as refLayerSliceIdc, refLayerFieldMbFlag, refLayerCTrafo, refLayerMbType, refSL,
refSCb, refSCr, refRL, refRCb, and refRCr, respectively.


The base decoding process for macroblocks in slices with resolution change is specified by the following ordered steps:


1. The variable CurrMbAddr is set equal to mbAddr.


2. The macroblock initialization process as specified in clause G.8.1.5.1 is invoked with refLayerVars, refPicList0

(when available), and refPicList1 (when available) as the inputs and the outputs are assigned to
sliceIdc[ mbAddr ], fieldMbFlag[ mbAddr ], cTrafo[ mbAddr ], baseModeFlag[ mbAddr ], mbType[ mbAddr ],
subMbType[ mbAddr ], mvCnt[ mbAddr ], tQPY[ mbAddr ], tQPCb[ mbAddr ] (when ChromaArrayType is not
equal to 0), tQPCr[ mbAddr ] (when ChromaArrayType is not equal to 0), the 2x2 arrays refIdxILPredL0 and
refIdxILPredL1, and the 4x4x2 arrays mvILPredL0 and mvILPredL1.


3. The SVC derivation process for motion vector components and reference indices as specified in clause G.8.4.1

is invoked with sliceIdc, fieldMbFlag, mbType, subMbType, predFlagL0, predFlagL1, refIdxL0, refIdxL1,
mvL0, mvL1, mvCnt, refIdxILPredL0, refIdxILPredL1, mvILPredL0, and mvILPredL1 as the inputs and the
outputs are modified versions of the arrays predFlagL0, predFlagL1, refIdxL0, refIdxL1, mvL0, mvL1, and
mvCnt.


4. The variable intraResamplingFlag is derived as follows:


     - If any of the following conditions are true, intraResamplingFlag is set equal to 1:


        - mbType[ mbAddr ] is equal to I_BL,


        - RestrictedSpatialResolutionChangeFlag is equal to 0, MbaffFrameFlag is equal to 0,
RefLayerMbaffFrameFlag is equal to 0, and base_mode_flag is equal to 1.


     - Otherwise, intraResamplingFlag is set equal to 0.


5. When intraResamplingFlag is equal to 1, the resampling process for intra samples as specified in clause G.8.6.2

is invoked with fieldMbFlag[ mbAddr ], refLayerSliceIdc, refLayerFieldMbFlag, refLayerMbType, refSL, cSL,
and, when ChromaArrayType is not equal to 0, refSCb, refSCr, cSCb, and cSCr as the inputs and the outputs are a
modified version of the array cSL and, when ChromaArrayType is not equal to 0, modified versions of the array
cSCb, and cSCr.


6. Depending on mbType[ mbAddr ], the following applies:


     - If mbType[ mbAddr ] is equal to I_PCM, I_16x16, I_8x8, or I_4x4, the SVC derivation process for intra
prediction modes as specified in clause G.8.3.1 is invoked with sliceIdc, fieldMbFlag, baseModeFlag,
mbType, ipred4x4, ipred8x8, ipred16x16, and, when ChromaArrayType is equal to 1 or 2, ipredChroma as
the inputs and the outputs are modified versions of ipred4x4, ipred8x8, ipred16x16, and, when
ChromaArrayType is equal to 1 or 2, ipredChroma.


     - Otherwise, if mbType[ mbAddr ] is not equal to I_BL and residual_prediction_flag is equal to 1, the
resampling process for residual samples as specified in clause G.8.6.3 is invoked with
fieldMbFlag[ mbAddr ], refLayerFieldMbFlag, refLayerCTrafo, refRL, rSL, and, when ChromaArrayType
is not equal to 0, refRCb, refRCr, rSCb, and rSCr as the inputs and the outputs are a modified version of the
array rSL and, when ChromaArrayType is not equal to 0, modified versions of the array rSCb, and rSCr.


     - Otherwise, the arrays of the collective term currentVars are not modified.


7. The transform coefficient scaling and refinement process as specified in clause G.8.5.1 is invoked with

refinementFlag set equal to 0, fieldMbFlag[ mbAddr ], mbType[ mbAddr ], cTrafo[ mbAddr ],
sTCoeff[ mbAddr ], tCoeffLevel[ mbAddr ], tQPY[ mbAddr ], and, when ChromaArrayType is not equal to 0,
tQPCb[ mbAddr ] and tQPCr[ mbAddr ] as the inputs and the outputs are modified versions of sTCoeff[ mbAddr ]
and tCoeffLevel[ mbAddr ].


8. The variable MvCnt for the macroblock mbAddr is set equal to mvCnt[ mbAddr ].





**G.8.1.5.4** **Macroblock decoding process prior to decoding a layer representation without resolution change and**

**MaxTCoeffLevelPredFlag equal to 0**


Inputs to this process are:


- a variable currDQId specifying the current layer representation,


- a variable mbAddr specifying the current macroblock inside the current layer representation,


- a set of arrays collectively referred to as currentVars.


Output of this process is the modified set of arrays collectively referred to as currentVars.


This process modifies the variables assigned to currentVars using syntax elements and derived upper-case variables for the
current macroblock, which is specified by its address mbAddr and the layer representation identifier currDQId.


Unless stated otherwise, the syntax elements and derived upper-case variables that are referred to by the process specified
in this clause and all child processes invoked from this process are the syntax elements and derived upper-case variables
for the current layer representation with DQId equal to currDQId.


Inside this clause, the arrays that are collectively referred to as currentVars are referred to by their names as specified in
clause G.8.1.2.1.


The macroblock decoding process prior to decoding a layer representation without resolution change and
MaxTCoeffLevelPredFlag equal to 0 is specified by the following ordered steps:


1. The variable CurrMbAddr is set equal to mbAddr.


2. The variable intraPredFlag is derived as follows:


     - If (sliceIdc[ mbAddr ] & 127) is equal to currDQId or MaxTCoeffLevelPredFlag is equal to 1, intraPredFlag
is set equal to 1.


     - Otherwise ((sliceIdc[ mbAddr ] & 127) is not equal to currDQId and MaxTCoeffLevelPredFlag is equal
to 0), intraPredFlag is set equal to 0.


3. When intraPredFlag is equal to 1 and mbType[ mbAddr ] is equal to I_PCM, I_16x16, I_8x8, or I_4x4, the SVC

intra sample prediction and construction process as specified in clause G.8.3.2 is invoked with sliceIdc,
fieldMbFlag, baseModeFlag, mbType, ipred4x4[ mbAddr ], ipred8x8[ mbAddr ], ipred16x16[ mbAddr ],
ipredChroma[ mbAddr ], cTrafo[ mbAddr ], sTCoeff[ mbAddr ], cSL, and, when ChromaArrayType is not equal
to 0, cSCb and cSCr as the inputs and the outputs are a modified version of the array cSL and, when
ChromaArrayType is not equal to 0, modified versions of the arrays cSCb and cSCr.


**G.8.1.5.5** **Macroblock decoding process prior to resolution change**


Inputs to this process are:


- a variable currDQId specifying the current layer representation,


- a variable mbAddr specifying the current macroblock inside the current layer representation,


- a set of arrays collectively referred to as currentVars.


Output of this process is the modified set of arrays collectively referred to as currentVars.


This process modifies the variables assigned to currentVars using syntax elements and derived upper-case variables for the
current macroblock, which is specified by its address mbAddr and the layer representation identifier currDQId.


Unless stated otherwise, the syntax elements and derived upper-case variables that are referred to by the process specified
in this clause and all child processes invoked from this process are the syntax elements and derived upper-case variables
for the current macroblock, which is the macroblock with address mbAddr inside the layer representation with DQId equal
to currDQId, the slice header of the current slice, which is the slice that contains the current macroblock, the current picture
parameter, which is identified by the syntax element pic_parameter_set_id inside the slice header of the current slice, and
the current sequence parameter, which is identified by the syntax element seq_parameter_set_id inside the current picture
parameter set.


Inside this clause, the arrays that are collectively referred to as currentVars are referred to by their names as specified in
clause G.8.1.2.1.


The macroblock decoding process prior to resolution change is specified by the following ordered steps:


1. The variable CurrMbAddr is set equal to mbAddr.


2. Depending on mbType[ mbAddr ], the following applies:





     - If mbType[ mbAddr ] is equal to I_PCM, I_16x16, I_8x8, or I_4x4, the SVC intra sample prediction and
construction process as specified in clause G.8.3.2 in invoked with sliceIdc, fieldMbFlag, baseModeFlag,
mbType, ipred4x4[ mbAddr ], ipred8x8[ mbAddr ], ipred16x16[ mbAddr ], ipredChroma[ mbAddr ],
cTrafo[ mbAddr ], sTCoeff[ mbAddr ], cSL, and, when ChromaArrayType is not equal to 0, cSCb and cSCr
as the inputs and the outputs are a modified version of the array cSL and, when ChromaArrayType is not
equal to 0, modified versions of the arrays cSCb and cSCr.


     - Otherwise, if mbType[ mbAddr ] is equal to I_BL, the following ordered steps are specified:


a. The residual construction and accumulation process as specified in clause G.8.5.3 is invoked with
accumulationFlag set equal to 0, fieldMbFlag[ mbAddr ], cTrafo[ mbAddr ], sTCoeff[ mbAddr ], rSL,
and, when ChromaArrayType is not equal to 0, rSCb and rSCr as the inputs and the outputs are a modified
version of rSL and, when ChromaArrayType is not equal to 0, modified versions of rSCb and rSCr.


b. The sample array accumulation process as specified in clause G.8.5.4 is invoked with

fieldMbFlag[ mbAddr ], rSL, cSL, and, when ChromaArrayType is not equal to 0, rSCb, rSCr cSCb, and
cSCr as the inputs and the outputs are a modified version of cSL and, when ChromaArrayType is not
equal to 0, modified versions of cSCb and cSCr.


c. The sample array re-initialization process as specified in clause G.8.5.5 is invoked with
fieldMbFlag[ mbAddr ], rSL, and, when ChromaArrayType is not equal to 0, rSCb and rSCr as the inputs
and the outputs are a modified version of rSL and, when ChromaArrayType is not equal to 0, modified
versions of rSCb and rSCr.


     - Otherwise (mbType[ mbAddr ] is not equal to I_PCM, I_16x16, I_8x8, I_4x4, or I_BL), the following
ordered steps are specified:


a. The residual construction and accumulation process as specified in clause G.8.5.3 is invoked with
accumulationFlag set equal to 1, fieldMbFlag[ mbAddr ], cTrafo[ mbAddr ], sTCoeff[ mbAddr ], rSL,
and, when ChromaArrayType is not equal to 0, rSCb and rSCr as the inputs and the outputs are a modified
version of rSL and, when ChromaArrayType is not equal to 0, modified versions of rSCb and rSCr.


b. The sample array re-initialization process as specified in clause G.8.5.5 is invoked with

fieldMbFlag[ mbAddr ], cSL, and, when ChromaArrayType is not equal to 0, cSCb and cSCr as the inputs
and the outputs are a modified version of cSL and, when ChromaArrayType is not equal to 0, modified
versions of cSCb and cSCr.


**G.8.1.5.6** **Target macroblock decoding process**


Inputs to this process are:


- a variable currDQId specifying the current layer representation,


- a variable mbAddr specifying the current macroblock inside the current layer representation,


- when present, a set of arrays collectively referred to as refLayerVars,


- a set of arrays collectively referred to as currentVars,


- when (slice_type % 5) is less than 2, the reference picture list refPicList0,


- when (slice_type % 5) is equal to 1, the reference picture list refPicList1.


Output of this process is the modified set of arrays collectively referred to as currentVars.


This process modifies the variables assigned to currentVars using syntax elements and derived upper-case variables for the
current macroblock, which is specified by its address mbAddr and the layer representation identifier currDQId.


Unless stated otherwise, the syntax elements and derived upper-case variables that are referred to by the process specified
in this clause and all child processes invoked from this process are the syntax elements and derived upper-case variables
for the current layer representation with DQId equal to currDQId.


Inside this clause, the arrays that are collectively referred to as currentVars are referred to by their names as specified in
clause G.8.1.2.1.


Inside this clause, the following applies:


- If refLayerVars is present as input to this clause, the arrays fieldMbFlag and mbType of the collective term
refLayerVars are referred to as refLayerFieldMbFlag and refLayerMbType, respectively.


- Otherwise (refLayerVars are not present as input to this clause), the variables refLayerFieldMbFlag and
refLayerMbType are marked as not available.





The target macroblock decoding process is specified by the following ordered steps:


1. The variable CurrMbAddr is set equal to mbAddr.


2. When MaxTCoeffLevelPredFlag is equal to 1, (sliceIdc[ mbAddr ] &127 ) is not equal to currDQId, and

ChromaArrayType is not equal to 0, the following ordered steps are specified:


a. The variable cQPY is set equal to tQPY[ mbAddr ], and for CX being replaced by Cb and Cr, the variable
cQPCX is set equal to the value of QPCX that corresponds to a value of cQPY for QPY as specified in
clause 8.5.8.


b. The transform coefficient level scaling process prior to transform coefficient refinement as specified in

clause G.8.5.2 is invoked with cTrafo[ mbAddr ], tCoeffLevel[ mbAddr ], tQPY set equal to cQPY, refQPY
set equal to tQPY[ mbAddr ], and, when ChromaArrayType is not equal to 0, tQPCb set equal to cQPCb, tQPCr
set equal to cQPCr, refQPCb set equal to tQPCb[ mbAddr ], refQPCr set equal to tQPCr[ mbAddr ] as the inputs
and the output is a modified version of tCoeffLevel[ mbAddr ].


c. The variables tQPCb[ mbAddr ] and tQPCr[ mbAddr ] are set equal to cQPCb and cQPCr, respectively.


d. The transform coefficient scaling and refinement process as specified in clause G.8.5.1 is invoked with

refinementFlag equal to 1, fieldMbFlag[ mbAddr ], mbType[ mbAddr ], cTrafo[ mbAddr ],
sTCoeff[ mbAddr ], tCoeffLevel[ mbAddr ], tQPY[ mbAddr ], tQPCb[ mbAddr ], and tQPCr[ mbAddr ] as
the inputs and the outputs are modified versions of sTCoeff[ mbAddr ] and tCoeffLevel[ mbAddr ]. For this
invocation of the process in clause G.8.5.1, all elements of the arrays LumaLevel4x4, LumaLevel8x8,
Intra16x16DCLevel, Intra16x16ACLevel, CbLevel4x4, CbLevel8x8, CbIntra16x16DCLevel,
CbIntra16x16ACLevel, CrLevel4x4, CrLevel8x8, CrIntra16x16DCLevel, CrIntra16x16ACLevel,
ChromaDCLevel, and ChromaACLevel are inferred to be equal to 0, tcoeff_level_prediction_flag is inferred
to be equal to 1, and base_mode_flag is inferred to be equal to 1.
NOTE – By the ordered steps specified above, the elements of the arrays tCoeffLevel[ mbAddr ] and sTCoeff[ mbAddr ] that
are related to luma transform coefficients are not modified. The array elements that are related to chroma transform
coefficients are only modified when the chroma quantization parameter offsets of the current layer representation with DQId
equal to currDQId and the layer representation with DQId equal to (sliceIdc[ mbAddr ] &127 ) are different.


3. Depending on mbType[ mbAddr ], the following applies:


     - If mbType[ mbAddr ] is equal to I_PCM, I_16x16, I_8x8, or I_4x4, the following ordered steps are
specified:


a. The variable intraPredFlag is derived as follows:


          - If (sliceIdc[ mbAddr ] & 127) is equal to currDQId or MaxTCoeffLevelPredFlag is equal to 1,
intraPredFlag is set equal to 1.


          - Otherwise ((sliceIdc[ mbAddr ] & 127) is not equal to currDQId and MaxTCoeffLevelPredFlag is
equal to 0), intraPredFlag is set equal to 0.


b. When intraPredFlag is equal to 1, the SVC intra sample prediction and construction process as specified

in clause G.8.3.2 is invoked with sliceIdc, fieldMbFlag, baseModeFlag, mbType, ipred4x4[ mbAddr ],
ipred8x8[ mbAddr ], ipred16x16[ mbAddr ], ipredChroma[ mbAddr ], cTrafo[ mbAddr ],
sTCoeff[ mbAddr ], cSL, and, when ChromaArrayType is not equal to 0, cSCb and cSCr as the inputs and
the outputs are a modified version of the array cSL and, when ChromaArrayType is not equal to 0,
modified versions of the arrays cSCb and cSCr.


     - Otherwise, if mbType[ mbAddr ] is equal to I_BL, the following ordered steps are specified:


a. The residual construction and accumulation process as specified in clause G.8.5.3 is invoked with
accumulationFlag set equal to 0, fieldMbFlag[ mbAddr ], cTrafo[ mbAddr ], sTCoeff[ mbAddr ], rSL,
and, when ChromaArrayType is not equal to 0, rSCb and rSCr as the inputs and the outputs are a modified
version of rSL and, when ChromaArrayType is not equal to 0, modified versions of rSCb and rSCr.


b. The sample array accumulation process as specified in clause G.8.5.4 is invoked with

fieldMbFlag[ mbAddr ], rSL, cSL, and, when ChromaArrayType is not equal to 0, rSCb, rSCr cSCb, and
cSCr as the inputs and the outputs are a modified version of cSL and, when ChromaArrayType is not
equal to 0, modified versions of cSCb and cSCr.


     - Otherwise (mbType[ mbAddr ] is not equal to I_PCM, I_16x16, I_8x8, I_4x4, or I_BL), the following
ordered steps are specified:


a. The SVC decoding process for Inter prediction samples as specified in clause G.8.4.2 is invoked with
targetQId set equal to (currDQId & 15), fieldMbFlag[ mbAddr ], sliceIdc[ mbAddr ],





mbType[ mbAddr ], subMbType[ mbAddr ], predFlagL0[ mbAddr ], predFlagL1[ mbAddr ],
refIdxL0[ mbAddr ], refIdxL1[ mbAddr ], mvL0[ mbAddr ], mvL1[ mbAddr ], refLayerFieldMbFlag
(when available), refLayerMbType (when available), refPicList0, refPicList1 (when available), cSL,
rSL, and, when ChromaArrayType is not equal to 0, cSCb, cSCr, rSCb, and rSCr as the inputs and the
outputs are modified version of cSL and rSL, and, when ChromaArrayType is not equal to 0, modified
versions of cSCb, cSCr, rSCb, and rSCr.


b. The residual construction and accumulation process as specified in clause G.8.5.3 is invoked with

accumulationFlag set equal to 1, fieldMbFlag[ mbAddr ], cTrafo[ mbAddr ], sTCoeff[ mbAddr ], rSL,
and, when ChromaArrayType is not equal to 0, rSCb and rSCr as the inputs and the outputs are a modified
version of rSL and, when ChromaArrayType is not equal to 0, modified versions of rSCb and rSCr.


c. The sample array accumulation process as specified in clause G.8.5.4 is invoked with
fieldMbFlag[ mbAddr ], rSL, cSL, and, when ChromaArrayType is not equal to 0, rSCb, rSCr cSCb, and
cSCr as the inputs and the outputs are a modified version of cSL and, when ChromaArrayType is not
equal to 0, modified versions of cSCb and cSCr.


**G.8.2** **SVC reference picture lists construction and decoded reference picture marking process**


The SVC decoding process for picture order count is specified in clause G.8.2.1.


The SVC decoding process for picture numbers is specified in clause G.8.2.2.


The SVC decoding process for reference picture lists construction is specified in clause G.8.2.3.


The SVC decoded reference picture marking process is specified in clause G.8.2.4.


The SVC decoding process for gaps in frame_num is specified in clause G.8.2.5.


The decoding process for picture order counts is independently applied for different values of dependency_id. Syntax
elements that are related to picture order count for a particular value of dependency_id do not influence the derivation of
picture order counts for other values of dependency_id.


The reference picture marking is independently applied for different values of dependency_id. Syntax elements that are
related to reference picture marking for a particular value of dependency_id do not influence the reference picture marking
for other values of dependency_id.


The decoding process for gaps is frame_num is independently applied for different values of dependency_id.


Reference picture lists for different dependency representations are constructed independently. Syntax elements that are
related to reference picture lists construction for a particular value of dependency_id do not influence the reference picture
lists construction for other values of dependency_id. Reference picture lists for a particular value of dependency_id are
constructed based on the reference picture marking for this particular value of dependency_id. The reference picture
marking for a particular value of dependency_id does not influence the reference picture lists construction for a different
value of dependency_id.


Only the elements of the reference picture lists for dependency_id equal to DependencyIdMax represent decoded pictures
that are associated with decoded samples. Only the reference picture lists for dependency_id equal to DependencyIdMax
are used for the derivation of inter prediction samples as specified in clause G.8.4.2. The elements of the reference picture
lists for dependency representation with dependency_id less than DependencyIdMax represent layer pictures, which are
not associated with decoded samples. The elements of the reference picture lists for dependency_id equal to 0 are associated
with the arrays fieldMbFlag, mbType, subMbType, predFlagL0, predFlagL1, refIdxL0, refIdxL1, mvL0, and mvL1 as
specified in clause G.8.1.2.1 that were derived when decoding the layer representation with dependency_id equal to 0 and
quality_id equal to 0 for the corresponding access unit. These arrays are used for the derivation of motion vectors and
reference indices for layer representation with dependency_id equal to 0 and quality_id equal to 0 as specified in
clause G.8.4.1.2. The elements of the reference picture lists for all dependency representations with dependency_id greater
than 0 are associated with the variables ScaledRefLayerLeftOffset, ScaledRefLayerRightOffset,
ScaledRefLayerTopOffset, and ScaledRefLayerBottomOffset. These variables are used for deriving inter-layer motion
vector predictions as specified in clause G.8.6.1.2.

NOTE – For each access unit, decoded samples only need to be stored for the dependency representation with dependency_id equal
to DependencyIdMax and motion data arrays only need to be stored for the dependency representation with dependency_id equal to
0.


The SVC decoding processes for picture order count, reference picture lists construction, reference picture marking, and
gaps in frame_num are specified using processes specified in clause 8. The following modifications to the processes
specified in this clause and the processes of clause 8 that are invoked from these processes apply with currDependencyId
representing the value of dependency_id for the dependency representation for which the processes are invoked:





a) All syntax elements and derived upper-case variables that are referred to in this process or in a child process invoked

from this process are syntax elements and upper-case variables for the dependency representation with
dependency_id equal to currDependencyId.


b) When dependency_id is less than DependencyIdMax, the following applies:


    - A frame, field, top field, bottom field, picture, and decoded picture is interpreted as layer frame, layer field,
layer top field, layer bottom field, layer picture, and decoded layer picture, respectively, for the dependency
representation with dependency_id equal to currDependencyId. A decoded layer picture is not associated
with the sample arrays SL, SCb, or SCr.


    - An IDR picture is interpreted as layer picture with IdrPicFlag equal to 1 for the dependency representation
with dependency_id equal to currDependencyId.


    - A reference frame, reference field, and reference picture is interpreted as layer frame, layer field, and layer
picture with nal_ref_idc greater than 0 for the dependency representation with dependency_id equal to
currDependencyId.


    - A non-reference frame, non-reference field, and non-reference picture is interpreted as layer frame, layer
field, and layer picture with nal_ref_idc equal to 0 for the dependency representation with dependency_id
equal to currDependencyId.


    - A complementary non-reference field pair is interpreted as complementary non-reference layer field pair for
the dependency representation with dependency_id equal to currDependencyId. A complementary
non-reference layer field pair for a particular value of dependency_id is a pair of two layer fields for the
particular value of dependency_id with the following properties: (i) the layer fields are in consecutive access
units containing a dependency representation with the particular value of dependency_id, (ii) the
dependency representations with the particular value of dependency_id in these access units have
nal_ref_idc equal to 0, field_pic_flag equal to 1, different values of bottom_field_flag, and they share the
same value of the frame_num syntax element, (iii) the first layer field is not already a paired layer field.


    - A complementary reference field pair is interpreted as complementary reference layer field pair for the
dependency representation with dependency_id equal to currDependencyId. A complementary reference
layer field pair for a particular value of dependency_id is a pair of two layer fields for the particular value
of dependency_id with the following properties: (i) the layer fields are in consecutive access units containing
a dependency representation with the particular value of dependency_id, (ii) the dependency representations
with the particular value of dependency_id in these access units have nal_ref_idc greater than 0,
field_pic_flag equal to 1, and the same value of frame_num, (iii) the dependency representation with the
particular value of dependency_id of the second access unit in decoding order has IdrPicFlag equal to 0 and
does not contain a memory_management_control_operation syntax element equal to 5.


    - A complementary field pair is interpreted as complementary layer field pair for the dependency
representation with dependency_id equal to currDependencyId. A complementary layer field pair is a
collective term for a complementary reference layer field pair and a complementary non-reference layer
field pair.


    - A non-paired non-reference field is interpreted as layer field with nal_ref_idc equal to 0 for the dependency
representation with dependency_id equal to currDependencyId that is not part of a complementary nonreference layer field pair.


    - A non-paired reference field is interpreted as layer field with nal_ref_idc greater than 0 for the dependency
representation with dependency_id equal to currDependencyId that is not part of a complementary reference
layer field pair.


    - A non-paired field is interpreted as layer field for the dependency representation with dependency_id equal
to currDependencyId that is not part of a complementary layer field pair.


    - A reference base frame is interpreted as reference layer base frame for the dependency representation with
dependency_id equal to currDependencyId. A reference layer base frame for a particular value of
dependency_id represents a second representation of a layer frame for dependency representations with
nal_ref_idc greater than 0, store_ref_base_pic_flag equal to 1, and field_pic_flag equal to 0.


    - A reference base field is interpreted as reference layer base field for the dependency representation with
dependency_id equal to currDependencyId. A reference layer base field for a particular value of
dependency_id represents a second representation of a layer field for dependency representations with
nal_ref_idc greater than 0, store_ref_base_pic_flag equal to 1, and field_pic_flag equal to 1.


    - A reference base picture is interpreted as reference layer base picture for the dependency representation with
dependency_id equal to currDependencyId. A reference layer base picture is a collective term for a reference





layer base field or a reference layer base frame. A reference layer base picture is not associated with the
sample arrays BL, BCb, or BCr.


     - A complementary reference base field pair is interpreted as complementary reference layer base field pair
for the dependency representation with dependency_id equal to currDependencyId. A complementary
reference layer base field pair for a particular value of dependency_id is a pair of two reference layer base
fields for the particular value of dependency_id with the following properties: (i) the reference layer base
fields are in consecutive access units containing a dependency representation with the particular value of
dependency_id, (ii) the dependency representations with the particular value of dependency_id in these
access units have nal_ref_idc greater than 0, store_ref_base_pic_flag equal to 1, field_pic_flag equal to 1
and the same value of frame_num, (iii) the dependency representation with the particular value of
dependency_id of the second of these access units in decoding order has IdrPicFlag equal to 0 and does not
contain a memory_management_control_operation syntax element equal to 5.


     - A non-paired reference base field is interpreted as reference layer base field for the dependency
representation with dependency_id equal to currDependencyId that is not part of a complementary reference
layer base field pair.


**G.8.2.1** **SVC decoding process for picture order count**


Input to this process is a list dqIdList of integer values specifying layer representation identifiers.


Outputs of this process are the variables TopFieldOrderCnt (if applicable) and BottomFieldOrderCnt (if applicable) for all
dependency representations of the set depRepSet specified in the following.


Let depRepSet be the set of dependency representations for which (dependency_id << 4) is contained in the list dqIdList.


For all dependency representations of the set depRepSet, the variables TopFieldOrderCnt (if applicable) and
BottomFieldOrderCnt (if applicable) are derived by invoking the decoding process for picture order count as specified in
clause 8.2.1. For these invocations of the process specified in clause 8.2.1, the modifications a) and b) specified in
clause G.8.2 apply with currDependencyId being equal to dependency_id of the corresponding dependency representation.


For all dependency representations of the set depRepSet for an access unit, either TopFieldOrderCnt or
BottomFieldOrderCnt or both are derived. When both are derived in two or more dependency representations of an access
unit, their difference shall be the same in these dependency representations of the access unit.


The values of TopFieldOrderCnt and BottomFieldOrderCnt are restricted as specified in the following ordered steps:


1. The set depRepSet for an access unit is the set depRepSet that has been derived in the process specified in this

clause for the corresponding access unit.


2. For each access unit, the one-dimensional array picOrderCnt is derived as follows:


     - If TopFieldOrderCnt is derived for all dependency representations of the set depRepSet for an access unit,
for each dependency representation of the set depRepSet for this access unit, the variable picOrderCnt[ dId ]
is set equal to TopFieldOrderCnt with dId being the value of dependency_id for the dependency
representation.


     - Otherwise (TopFieldOrderCnt is not derived for all dependency representations of the set depRepSet for an
access unit), for each dependency representation of the set depRepSet for this access unit, the variable
picOrderCnt[ dId ] is set equal to BottomFieldOrderCnt with dId being the value of dependency_id for the
dependency representation.


3. Let au0 and au1 be any pair of access units in the bitstream with au1 being later in decoding order than au0.


4. Let the flag idrConditionFlag be derived for each dependency representation of the set depRepSet for an access

unit as follows:


     - If the dependency representation in the access unit has IdrPicFlag equal to 1 or a
memory_management_control_operation syntax element equal to 5, idrConditionFlag is set equal to 1.


     - Otherwise (the dependency representation in the access unit has IdrPicFlag equal to 0 and does not have a
memory_management_control_operation syntax element equal to 5), idrConditionFlag is set equal to 0.


5. Let the set dIdSet0 be the set of all dependency_id values of the set depRepSet for au0.


6. Let the set dIdSet1 be the set of all dependency_id values of the set depRepSet for au1 for which idrConditionFlag

is not equal to 1 in any access unit in decoding order between the access unit that follows au0 and the access unit
au1, inclusive.





7. For all values of dId that are present in both sets dIdSet0 and dIdSet1, the differences between the value

picOrderCnt[ dId ] in au0 and the value picOrderCnt[ dId ] in au1 shall be the same.


**G.8.2.2** **SVC decoding process for picture numbers**


This process is invoked when the SVC decoding process for reference picture lists construction specified in clause G.8.2.3,
the SVC reference picture marking process for a dependency representation specified in clause G.8.2.4.1, or the SVC
decoding process for gaps in frame_num specified in clause G.8.2.5 is invoked.


Inputs to this process are:


- a variable currDependencyId specifying a dependency representation,


- a variable refPicListConstructionFlag specifying whether this process is invoked for reference picture lists
construction,


- when refPicListConstructionFlag is equal to 1, a variable useRefBasePicFlag specifying whether reference base
pictures are considered for reference picture lists construction.


From here to the end of this clause, the modifications a) and b) specified in clause G.8.2 apply.


The variables FrameNum, FrameNumWrap, and PicNum are assigned to all short-term reference pictures and the variables
LongTermFrameIdx and LongTermPicNum are assigned to all long-term reference pictures by invoking the decoding
process for picture numbers as specified in clause 8.2.4.1.

NOTE 1 – For this invocation of the process specified in clause 8.2.4.1, the pictures marked as "reference base pictures" and the
pictures not marked as "reference base pictures" are taken into account.


For the following specification of this clause, reference frames, complementary reference field pairs, and non-paired
reference fields with at least one field marked as "used for reference" are referred to as reference entries. When only one
field of a reference entry is marked as "used for reference", the reference entry is considered to have the same marking(s)
and the same assigned variables as its field marked as "used for reference". When a reference entry is marked as "not
available for reference list construction" in the following process, both of its fields are also marked as "not available for
reference list construction".


When refPicListConstructionFlag is equal to 1, the following applies:


- If useRefBasePicFlag is equal to 0, all reference entries that are marked as "reference base picture" are marked as "not
available for reference list construction".
NOTE 2 – When useRefBasePicFlag is equal to 0, only reference entries that are not marked as "reference base picture" are
considered as present for the purpose of reference picture lists construction.


- Otherwise (useRefBasePicFlag is equal to 1), all reference entries for which one of the following conditions is true
are marked as "not available for reference list construction":


    - the reference entry is not marked as "reference base picture", the reference entry is marked as "used for
short-term reference", and there exists a reference entry with the same value of FrameNum that is marked as
"reference base picture" and "used for short-term reference",


    - the reference entry is not marked as "reference base picture", the reference entry is marked as "used for
long-term reference", and there exists a reference entry with the same value of LongTermFrameIdx that is
marked as "reference base picture" and "used for long-term reference".
NOTE 3 – When useRefBasePicFlag is equal to 1 and either two short-term reference entries have the same value of FrameNum
or two long-term reference entries have the same value of LongTermFrameIdx (one of these reference entries is marked as
"reference base picture" and the other reference entry is not marked as "reference base picture"), only the reference entry marked
as "reference base picture" is considered as present for the purpose of reference picture lists construction.


**G.8.2.3** **SVC decoding process for reference picture lists construction**


This process is invoked at the beginning of the decoding process for each P, EP, B, or EB slice.


Inputs to this process are:


- a variable currDependencyId,


- a variable useRefBasePicFlag,


- the current slice currSlice.


Outputs of this process are:


- a reference picture list refPicList0,


- for B and EB slices, a reference picture list refPicList1.





After applying the process described in this clause, the output reference picture lists refPicList0 and refPicList1 (when
applicable) shall not contain any pictures for which the syntax element temporal_id is greater than the syntax element
temporal_id of the current picture.


From here to the end of this clause, the modifications a) and b) specified in clause G.8.2 apply.


Unless stated otherwise, the syntax elements and derived upper-case variables that are referred to by the process specified
in this clause and all child processes invoked from this process are the syntax elements and derived upper-case variables
for the slice header of the current slice currSlice, the current picture parameter, which is identified by the syntax element
pic_parameter_set_id inside the slice header of the current slice currSlice, and the current sequence parameter, which is
identified by the syntax element seq_parameter_set_id inside the current picture parameter set.


A variable biPred is derived as follows:


- If the current slice currSlice is a B or EB slice, biPred is set equal to 1.


- Otherwise, biPred is set equal to 0.


Decoded reference pictures are marked as "used for short-term reference" or "used for long-term reference" as specified
by the bitstream and specified in clause G.8.2.4. Short-term reference pictures are identified by the value of frame_num
that is decoded in the slice header(s) with dependency_id equal to currDependencyId. Long-term reference pictures are
identified by a long-term frame index as specified by the bitstream and specified in clause G.8.2.4.


Clause G.8.2.2 is invoked with currDependencyId, refPicListConstructionFlag equal to 1, and useRefBasePicFlag as inputs
to specify the following:


- the assignment of variables FrameNum, FrameNumWrap, and PicNum to each of the short-term reference pictures,


- the assignment of variables LongTermPicNum to each of the long-term reference pictures,


- the marking of reference pictures that are not used for reference picture lists construction as "not available for
reference list construction" (depending on the value of useRefBasePicFlag).
NOTE 1 – The marking of reference pictures as "not available for reference list construction" is removed after construction of the
reference picture lists.


Reference pictures are addressed through reference indices as specified in clause 8.4.2.1 with the modification e) specified
in clause G.8.4.2. A reference index is an index into a reference picture list. When biPred is equal to 0, a single reference
picture list refPicList0 is constructed. When decoding a B or EB slice (biPred is equal to 1), a second independent reference
picture list refPicList1 is constructed in addition to refPicList0.


At the beginning of the decoding process for each slice, reference picture list refPicList0, and for biPred equal to 1
refPicList1, are derived as specified in the following ordered steps:


1. Initial reference picture lists RefPicList0 and, for biPred equal to 1, RefPicList1 are derived by invoking the

initialization process for reference picture lists as specified in clause 8.2.4.2. During the initialization process in
clause 8.2.4.2 all reference frames, complementary reference field pairs, and non-paired reference fields that have
been marked as "not available for reference list construction" by the invocation of clause G.8.2.2 are considered as
not present.


2. When ref_pic_list_modification_flag_l0 is equal to 1 or, when decoding a B or EB slice (biPred is equal to 1),

ref_pic_list_modification_flag_l1 is equal to 1, the initial reference picture list RefPicList0 and for biPred equal
to 1 RefPicList1 are modified by invoking the modification process for reference picture lists as specified in
clause 8.2.4.3. During the modification process in clause 8.2.4.3 all reference frames, complementary reference
field pairs, and non-paired reference fields that have been marked as "not available for reference list construction"
by the invocation of clause G.8.2.2 are considered as not present.


3. RefPicList0 is assigned to refPicList0.


4. When biPred is equal to 1, RefPicList1 is assigned to refPicList1.
NOTE 2 – By the invocation of the process in clause G.8.2.2 some reference frames, complementary reference field pairs, and nonpaired reference fields might have been marked as "not available for reference list construction". Since, these pictures are not
considered in the construction process for reference picture lists, the reference picture lists refPicList0 and, for biPred equal to 1,
refPicList1 are dependent on the value of the input parameter useRefBasePicFlag.


The number of entries in the modified reference picture list refPicList0 is num_ref_idx_l0_active_minus1 + 1, and for
biPred equal to 1 the number of entries in the modified reference picture list refPicList1 is
num_ref_idx_l1_active_minus1 + 1. A reference picture may appear at more than one index in the modified reference
picture lists refPicList0 or refPicList1.


For all reference frames, complementary reference field pairs, and non-paired reference fields that have been marked as
"not available for reference list construction" during the invocation of clause G.8.2.2, this marking is removed.





**G.8.2.4** **SVC decoded reference picture marking process**


Input to this process is a list dqIdList of integer values specifying layer representation identifiers.


Let depRepSet be the set of dependency representations for which all of the following conditions are true:


- the list dqIdList contains the value (dependency_id << 4),


- nal_ref_idc is greater than 0.


For each dependency representation of the set depRepSet, the SVC reference picture marking process for a dependency
representation as specified in clause G.8.2.4.1 is invoked. For these invocations of the process specified in clause G.8.2.4.1,
the modifications a ) and b ) specified in clause G.8.2 apply with currDependencyId being equal to dependency_id for the
corresponding dependency representation.


**G.8.2.4.1** **SVC reference picture marking process for a dependency representation**


Input to this process is a variable currDependencyId.


Output of this process is a modified reference picture marking for dependency representations with dependency_id equal
to currDependencyId.


This process is invoked for a decoded picture when nal_ref_idc is not equal to 0 for the dependency representation with
dependency_id being equal to currDependencyId.


All syntax elements and derived upper-case variables that are referred to by the process specified in this clause and all
child processes invoked from this process are syntax elements and derived upper-case variables for the dependency
representation with dependency_id equal to currDependencyId.


A decoded picture with nal_ref_idc not equal to 0, referred to as a reference picture, is marked as "used for short-term
reference" or "used for long-term reference". When store_ref_base_pic_flag is equal to 1, a second representation of the
decoded picture also referred to as reference base picture is marked as "used for short-term reference" or "used for
long-term reference" and additionally marked as "reference base picture". Pictures that are marked as "reference base
picture" are only used as references for inter prediction of following pictures with use_ref_base_pic_flag equal to 1. These
pictures are not used for inter prediction of pictures with use_ref_base_pic_flag equal to 0, and these pictures do not
represent an output of the decoding process.


For a decoded reference frame, both of its fields are marked the same as the frame. For a complementary reference field
pair, the pair is marked the same as both of its fields. A picture that is marked as "used for short-term reference" is identified
by its FrameNum and, when it is a field, by its parity, and, when it is a reference base picture, by the marking "reference
base picture". A picture that is marked as "used for long-term reference" is identified by its LongTermFrameIdx and, when
it is a field, by its parity, and, when it is a reference base picture, by the marking "reference base picture".


While decoded pictures are represented by the sample arrays SL and, when ChromaArrayType is not equal to 0, SCb and
SCr, reference base pictures are represented by the sample arrays BL and, when ChromaArrayType is not equal to 0, BCb
and BCr. When reference base pictures are referenced in the inter prediction process via clause 8.4.2.1, the samples arrays
BL, BCb, and BCr are referred to as SL, SCb, and SCr, respectively. The sample arrays SL, SCb, SCr, BL, BCb, and BCr that
referenced in the inter prediction process via clause 8.4.2.1 are constructed as specified in clause G.8. Reference base
pictures are associated with the same descriptive information such as the variables FrameNum, FrameNumWrap, PicNum,
LongTermFrameIdx, and LongTermPicNum as decoded pictures.


Frames or complementary field pairs marked as "used for short-term reference" or as "used for long-term reference" can
be used as a reference for inter prediction when decoding a frame until the frame, the complementary field pair, or one of
its constituent fields is marked as "unused for reference". A field marked as "used for short-term reference" or as "used for
long-term reference" can be used as a reference for inter prediction when decoding a field until marked as "unused for
reference".


A picture can be marked as "unused for reference" by the sliding window reference picture marking process, a first-in,
first-out mechanism specified in clause G.8.2.4.2, or by the adaptive memory control reference picture marking process, a
customised adaptive marking operation specified in clauses G.8.2.4.3 and G.8.2.4.4.


A short-term reference picture is identified for use in the decoding process by its variables FrameNum and FrameNumWrap
and its picture number PicNum, and, when it is a reference base picture, by the marking as "reference base picture". A
long-term reference picture is identified for use in the decoding process by its variable LongTermFrameIdx, its long-term
picture number LongTermPicNum, and, when it is a reference base picture, by the marking as "reference base picture".


When the current picture is not an IDR picture, the variables FrameNum, FrameNumWrap, PicNum, LongTermFrameIdx,
and LongTermPicNum are assigned to the reference pictures by invoking the SVC decoding process for picture numbers
as specified in clause G.8.2.2 with currDependencyId and refPicListConstructionFlag set equal to 0 as the inputs.


Decoded reference picture marking proceeds in the following ordered steps:





1. All slices of the current access unit are decoded.


2. Depending on the current picture, the following applies:


     - If the current picture is an IDR picture, the following ordered steps are specified:


a. All reference pictures are marked as "unused for reference".


b. Depending on long_term_reference_flag, the following applies:


          - If long_term_reference_flag is equal to 0, the following ordered steps are specified:


i. The IDR picture is marked as "used for short-term reference" and MaxLongTermFrameIdx
is set equal to "no long-term frame indices".


ii. When store_ref_base_pic_flag is equal to 1, the reference base picture of the IDR picture is

marked as "used for short-term reference" and as "reference base picture".


          - Otherwise (long_term_reference_flag is equal to 1), the following ordered steps are specified:


i. The IDR picture is marked as "used for long-term reference", the LongTermFrameIdx for the
IDR picture is set equal to 0, and MaxLongTermFrameIdx is set equal to 0.


ii. When store_ref_base_pic_flag is equal to 1, the reference base picture of the IDR picture is

marked as "used for long-term reference" and as "reference base picture", and the
LongTermFrameIdx for the reference base picture of the IDR picture is set equal to 0.


     - Otherwise (the current picture is not an IDR picture), the following ordered steps are specified:


a. When adaptive_ref_base_pic_marking_mode_flag is equal to 1, the SVC adaptive memory control
reference base picture marking process as specified in clause G.8.2.4.3 is invoked.

NOTE 1 – By this invocation of the process specified in clause G.8.2.4.3, pictures that are marked as "used
for reference" and "reference base picture" can be marked as "unused for reference".


With currTId being the value of temporal_id for the current access unit, the bitstream shall not contain
data that result in the marking of pictures with temporal_id less currTId as "unused for reference" during
this invocation of the process in clause G.8.2.4.3.


b. Depending on adaptive_ref_pic_marking_mode_flag, the following applies:


          - If adaptive_ref_pic_marking_mode_flag is equal to 1, the SVC adaptive memory control decoded
reference picture marking process as specified in clause G.8.2.4.4 is invoked.


With currTId being the value of temporal_id for the current access unit, the bitstream shall not
contain data that result in the marking of pictures with temporal_id less currTId as "unused for
reference" during this invocation of the process in clause G.8.2.4.4.


          - Otherwise (adaptive_ref_pic_marking_mode_flag is equal to 0), the SVC sliding window decoded
reference picture marking process as specified in clause G.8.2.4.2 is invoked with refBasePicFlag
equal to 0 as the input.


c. When the current picture was not marked as "used for long-term reference" by a
memory_management_control_operation command equal to 6, the current picture is marked as "used
for short-term reference" and, when the current picture is the second field (in decoding order) of a
complementary reference field pair and the first field is marked as "used for short-term reference", the
complementary field pair is also marked as "used for short-term reference".


d. When store_ref_base_pic_flag is equal to 1 and the reference base picture for the current picture was

not marked as "used for long-term reference" by a memory_management_control_operation command
equal to 6, the following ordered steps are specified:


i. When adaptive_ref_base_pic_marking_mode_flag is equal to 0, the following ordered steps are
specified:


(1) The SVC decoding process for picture numbers as specified in clause G.8.2.2 is invoked with

currDependencyId and refPicListConstructionFlag set equal to 0 as the inputs.


(2) The SVC sliding window decoded reference picture marking process as specified in

clause G.8.2.4.2 is invoked with refBasePicFlag equal to 1 as the input.


ii. The reference base picture of the current picture is marked as "used for short-term reference" and

as "reference base picture" and, when the reference base picture of the current picture is the second
reference base field (in decoding order) of a complementary reference base field pair and the first





reference base field is marked as "used for short-term reference" (and "reference base picture"),
the complementary reference base field pair is also marked as "used for short-term reference" and
"reference base picture".
NOTE 2 – When both the decoded picture and the reference base picture for an access unit (including the current access unit)
are marked as "used for reference", either both pictures are marked as "used for short-term reference" or both pictures are
marked as "used for long-term reference" after the completion of the process specified in this clause. And in the latter case,
the same value of LongTermFrameIdx is assigned to both pictures.


It is a requirement of bitstream conformance that, after marking the current decoded reference picture and, when
store_ref_base_pic_flag is equal to 1, the current reference base picture, the total number of frames with at least one field
marked as "used for reference", plus the number of complementary field pairs with at least one field marked as "used for
reference", plus the number of non-paired fields marked as "used for reference" shall not be greater than
Max( max_num_ref_frames, 1 ).

NOTE 3 – For this constraint, the pictures marked as "reference base pictures" and the pictures not marked as "reference base
picture" are taken into account.


**G.8.2.4.2** **SVC sliding window decoded reference picture marking process**


Input to this process is a variable refBasePicFlag.


The variable newFrameBufferFlag is derived as follows:


- If one of the following conditions is true, newFrameBufferFlag is set equal to 0:


   - refBasePicFlag is equal to 0, the current picture is a coded field that is the second field in decoding order of a
complementary reference field pair, and the first field of the complementary reference field pair has been marked
as "used for short-term reference",


   - refBasePicFlag is equal to 1, the current reference base picture is a reference base field that is the second field in
decoding order of a complementary reference base field pair, and the first field has been marked as "used for
short-term reference" (and "reference base picture").


- Otherwise, newFrameBufferFlag is set equal to 1.


When newFrameBufferFlag is equal to 1, the following ordered steps are specified:


1. Let numShortTerm be the total number of reference frames, complementary reference field pairs, and non-paired

reference fields for which at least one field is marked as "used for short-term reference". Let numLongTerm be
the total number of reference frames, complementary reference field pairs, and non-paired reference fields for
which at least one field is marked as "used for long-term reference".

NOTE 1 – For this derivation of numShortTerm and numLongTerm, the pictures marked as "reference base pictures"
and the pictures not marked as "reference base picture" are taken into account.


2. When numShortTerm + numLongTerm is equal to Max( max_num_ref_frames, 1 ), the following ordered steps

are specified:


a. The condition that numShortTerm is greater than 0 shall be fulfilled.


b. Let frameNumWrapDecPic be the smallest value of FrameNumWrap that is assigned to reference frames,
complementary reference field pairs, and non-paired reference fields that are marked as "used for short-term
reference" and not marked as "reference base pictures". When there doesn't exist any reference frame,
complementary reference field pair, or non-paired reference field that is marked as "used for short-term
reference" and not marked as "reference base picture", frameNumWrapDecPic is set equal to
MaxFrameNum.


c. Let frameNumWrapBasePic be the smallest value of FrameNumWrap that is assigned to reference frames,
complementary reference field pairs, and non-paired reference fields that are marked as "used for shortterm reference" and marked as "reference base pictures". When there doesn't exist any reference frame,
complementary reference field pair, or non-paired reference field that is marked as "used for short-term
reference" and marked as "reference base picture", frameNumWrapBasePic is set equal to MaxFrameNum.

NOTE 2 – The value of MaxFrameNum is greater than all values of FrameNumWrap that are assigned to reference
frames, complementary reference field pairs, and non-paired reference fields marked as "used for short-term
reference.


d. The short-term reference frame, complementary reference field pair, or non-paired reference field picX is
derived as follows:


        - If frameNumWrapDecPic is less than frameNumWrapBasePic, picX is the short-term reference frame,
complementary reference field pair, or non-paired reference field that has the value of FrameNumWrap
equal to frameNumWrapDecPic (and is not marked as "reference base picture").





        - Otherwise (frameNumWrapDecPic is greater than or equal to frameNumWrapBasePic), picX is the
short-term reference frame, complementary reference field pair, or non-paired reference field that has
the value of FrameNumWrap equal to frameNumWrapBasePic and is marked as "reference base
picture".


e. It is a requirement of bitstream conformance that the short-term reference frame, complementary reference
field pair, or non-paired reference field picX shall not be the current picture or the complementary field pair
that contains the current picture.

NOTE 3 – When refBasePicFlag is equal to 1, the current picture has been marked as "used for short-term
reference" in the same invocation of the process specified in clause G.8.2.4.1.


f. The short-term reference frame, complementary reference field pair, or non-paired reference field picX is
marked as "unused for reference". When it is a frame or a complementary field pair, both of its fields are
also marked as "unused for reference".


**G.8.2.4.3** **SVC adaptive memory control reference base picture marking process**


This process is invoked when adaptive_ref_base_pic_marking_mode_flag is equal to 1.


The memory_management_base_control_operation commands with values of 1 and 2 are processed in the order they occur
in the dec_ref_base_pic_marking( ) syntax structure after the current picture has been decoded. The
memory_management_base_control_operation command with value of 0 specifies the end of the
memory_management_base_control_operation commands.


Memory management control base operations are applied to pictures as follows:


- If field_pic_flag is equal to 0, memory_management_base_control_operation commands are applied to the reference
base frames or complementary reference base field pairs specified.


- Otherwise (field_pic_flag is equal to 1), memory_management_base_control_operation commands are applied to the
individual reference base fields specified.


For each memory_management_base_control_operation command with a value not equal to 0, the following applies:


- If memory_management_base_control_operation is equal to 1, the marking process of a short-term reference picture
as "unused for reference" as specified in clause 8.2.5.4.1, is invoked with substituting
difference_of_pic_nums_minus1 with difference_of_base_pic_nums_minus1. For this invocation of the process
specified in clause 8.2.5.4.1, all pictures that are not marked as "reference base picture" are considered as not present.

NOTE 1 – Short-term reference pictures that are not marked as "reference base pictures" cannot be marked as "unused
for reference" by a memory_management_base_control_operation equal to 1.


- Otherwise, if memory_management_base_control_operation is equal to 2, the marking process of a long-term
reference picture as "unused for reference" as specified in clause 8.2.5.4.2 is invoked with substituting
long_term_pic_num with long_term_base_pic_num. For this invocation of the process specified in clause 8.2.5.4.2,
all pictures that are not marked as "reference base picture" are considered as not present.

NOTE 2 – Long-term reference pictures that are not marked as "reference base pictures" cannot be marked as "unused
for reference" by a memory_management_base_control_operation equal to 2.


**G.8.2.4.4** **SVC adaptive memory control decoded reference picture marking process**


This process is invoked when adaptive_ref_pic_marking_mode_flag is equal to 1.


The memory_management_control_operation commands with values of 1 to 6 are processed in the order they occur in the
dec_ref_pic_marking( ) syntax structure after the current picture has been decoded. The
memory_management_control_operation command with value of 0 specifies the end of the
memory_management_control_operation commands.


Memory management control operations are applied to pictures as follows:


- If field_pic_flag is equal to 0, memory_management_control_operation commands are applied to the frames or
complementary reference field pairs specified.


- Otherwise (field_pic_flag is equal to 1), memory_management_control_operation commands are applied to the
individual reference fields specified.


For each memory_management_control_operation command with a value not equal to 0, the following applies:


- If memory_management_control_operation is equal to 1, the marking process of a short-term reference picture as
"unused for reference" as specified in clause 8.2.5.4.1 is invoked. For this invocation of the process specified in
clause 8.2.5.4.1, all pictures that are marked as "reference base picture" are considered as not present.





NOTE 1 – Short-term reference pictures that are marked as "reference base pictures" cannot be marked as "unused for
reference" by a memory_management_control_operation equal to 1.


- Otherwise, if memory_management_control_operation is equal to 2, the marking process of a long-term reference
picture as "unused for reference" as specified in clause 8.2.5.4.2 is invoked. For this invocation of the process
specified in clause 8.2.5.4.2, all pictures that are marked as "reference base picture" are considered as not present.

NOTE 2 – Long-term reference pictures that are marked as "reference base pictures" cannot be marked as "unused for
reference" by a memory_management_control_operation equal to 2.


- Otherwise, if memory_management_control_operation is equal to 3, the following ordered steps are specified:


1. The assignment process of a LongTermFrameIdx to a short-term reference picture as specified in

clause 8.2.5.4.3 is invoked. For this invocation of the process specified in clause 8.2.5.4.3, all pictures that are
marked as "reference base picture" are considered as not present. The variable picNumX is set equal to the
value picNumX that is derived during the invocation of the process specified in clause 8.2.5.4.3.


2. Depending on whether there exists a picture that is marked as "reference base picture" and "used for short-term

reference" and has a value of PicNum equal to picNumX, the following applies:


      - If there exists a picture that is marked as "reference base picture" and "used for short-term reference" and
has a value of PicNum equal to picNumX, the assignment process of a LongTermFrameIdx to a short-term
reference picture as specified in clause 8.2.5.4.3 is invoked again. For this second invocation of the process
specified in clause 8.2.5.4.3, all pictures that are not marked as "reference base picture" are considered as
not present.

NOTE 3 – When the marking of a decoded picture (not marked as "reference base picture") is changed from "used
for short-term reference" to "used for long-term reference" and there exists a reference base picture (marked as
"reference base picture") that has the same value of PicNum as the decoded picture (before the marking is
modified), the marking of this reference base picture is also changed from "used for short-term reference" to "used
for long-term reference" and the same value of LongTermFrameIdx is assigned to both the decoded picture and
the reference base picture.


      - Otherwise, if LongTermFrameIdx equal to long_term_frame_idx is assigned to a long-term reference
frame marked as "reference base picture" or a long-term complementary reference field pair marked as
"reference base picture", that frame or complementary field pair and both of its fields are marked as
"unused for reference".


      - Otherwise, if LongTermFrameIdx equal to long_term_frame_idx is assigned to a long-term reference field
marked as "reference base picture", and the associated decoded picture (not marked as "reference base
picture") is not part of a complementary field pair that includes the picture specified by picNumX (before
invoking the process specified in clause 8.2.5.4.3) and not marked as "reference base picture", that field is
marked as "unused for reference".

NOTE 4 – When a particular value of LongTermFrameIdx is assigned to a reference base picture (marked as
"reference base picture") and a decoded picture (not marked as "reference base picture"), the reference base picture
is either associated with the same access unit as the decoded picture or with an access unit that represents a field
that is part of a complementary field pair that includes the decoded picture.


      - Otherwise, the reference picture marking is not modified.


- Otherwise, if memory_management_control_operation is equal to 4, the decoding process for
MaxLongTermFrameIdx as specified in clause 8.2.5.4.4 is invoked.

NOTE 5 – For this invocation of the process specified in clause 8.2.5.4.4, the pictures marked as "reference base
pictures" and the pictures not marked as "reference base picture" are taken into account.


- Otherwise, if memory_management_control_operation is equal to 5, the marking process of all reference pictures as
"unused for reference" and setting MaxLongTermFrameIdx to "no long-term frame indices" as specified in
clause 8.2.5.4.5 is invoked.

NOTE 6 – For this invocation of the process specified in clause 8.2.5.4.5, the pictures marked as "reference base
pictures" and the pictures not marked as "reference base picture" are taken into account.


- Otherwise (memory_management_control_operation is equal to 6), the following ordered steps are specified:


1. The process for assigning a long-term frame index to the current picture as specified in clause 8.2.5.4.6 in

invoked. For this invocation of the process specified in clause 8.2.5.4.6, all pictures that are marked as
"reference base picture" are considered as not present.


2. Depending on store_ref_base_pic_flag, the following applies:


      - If store_ref_base_pic_flag is equal to 1, the reference base picture of the current picture is marked as
"reference base picture" and the process for assigning a long-term frame index to the current picture as
specified in clause 8.2.5.4.6 is invoked again. For this second invocation of the process specified in





clause 8.2.5.4.6, the reference base picture is considered as the current picture and all pictures that are not
marked as "reference base picture" are considered as not present. When the reference base picture of the
current picture is the second reference base field (in decoding order) of a complementary reference base
field pair, the complementary reference base field pair is also marked as "reference base picture".

NOTE 7 – When the current decoded picture is marked as "used for long-term reference" and
store_ref_base_pic_flag is equal to 1, the current reference base picture is also marked as "used for long-term
reference" and the same value of LongTermFrameIdx is assigned to both the current decoded picture and the
current reference base picture. The current reference base picture is additionally marked as "reference base
picture".


      - Otherwise, if LongTermFrameIdx equal to long_term_frame_idx is assigned to a long-term reference
frame marked as "reference base picture" or a long-term complementary reference field pair marked as
"reference base picture", that frame or complementary field pair and both of its fields are marked as
"unused for reference".


      - Otherwise, if LongTermFrameIdx equal to long_term_frame_idx is assigned to a long-term reference field
marked as "reference base picture", and the associated decoded picture (not marked as "reference base
picture") is not part of a complementary field pair that includes the current picture, that field is marked as
"unused for reference".

NOTE 8 – When a particular value of LongTermFrameIdx is assigned to a reference base picture (marked as
"reference base picture") and a decoded picture (not marked as "reference base picture"), the reference base picture
is either associated with the same access unit as the decoded picture or with an access unit that represents a field
that is part of a complementary field pair that includes the decoded picture.


      - Otherwise, the reference picture marking is not modified.


3. It is a requirement of bitstream conformance that, after marking the current decoded reference picture and, when

store_ref_base_pic_flag is equal to 1, the current reference base picture, the total number of frames with at least
one field marked as "used for reference", plus the number of complementary field pairs with at least one field
marked as "used for reference", plus the number of non-paired fields marked as "used for reference" shall not
be greater than Max( max_num_ref_frames, 1 ).

NOTE 9 – For this constraint, the pictures marked as "reference base pictures" and the pictures not marked as "reference
base picture" are taken into account.
NOTE 10 – Under some circumstances, the above statement may impose a constraint on the order in which a
memory_management_control_operation syntax element equal to 6 can appear in the decoded reference picture marking
syntax relative to a memory_management_control_operation syntax element equal to 1, 2, 3, or 4, or it may impose a
constraint on the value of adaptive_ref_base_pic_marking_mode_flag.


**G.8.2.5** **SVC decoding process for gaps in frame_num**


Input to this process is a list dqIdList of integer values specifying layer representation identifiers.


Let depRepSet be the set of dependency representations for which (dependency_id << 4) is contained in the list dqIdList.


For all dependency representations of the set depRepSet, the following applies:


- The variable currDependencyId is set equal to the value of dependency_id for the currently considered dependency
representation of the set depRepSet.


- The syntax elements gaps_in_frame_num_value_allowed_flag and frame_num and the derived upper-case variables
PrevRefFrameNum and MaxFrameNum are the syntax elements and derived upper-case variables for the considered
dependency representation.


- When gaps_in_frame_num_value_allowed_flag is equal to 0, the bitstream shall not contain data that result in
frame_num not being equal to PrevRefFrameNum or ( PrevRefFrameNum + 1 ) % MaxFrameNum.

NOTE – When gaps_in_frame_num_value_allowed_flag is equal to 0 and frame_num is not equal to PrevRefFrameNum and
is not equal to ( PrevRefFrameNum + 1 ) % MaxFrameNum, the decoding process should infer an unintentional loss of
pictures.


- When frame_num is not equal to PrevRefFrameNum and is not equal to ( PrevRefFrameNum + 1 ) %
MaxFrameNum, the decoding process for gaps in frame_num as specified in clause 8.2.5.2 is invoked. For this
invocation of the process specified in clause 8.2.5.2, the modifications a) and b) specified in clause G.8.2 apply, the
invocation of the decoding process for picture numbers specified in clause 8.2.4.1 is substituted with the invocation
of the SVC decoding process for picture numbers specified in clause G.8.2.2 with currDependencyId and
refPicListConstructionFlag equal to 0 as the inputs, and the invocation of sliding window picture marking process
specified in clause 8.2.5.3 is substituted with the invocation of the SVC sliding window decoded reference picture
marking process specified in clause G.8.2.4.2 with refBasePicFlag equal to 0 as the input.





**G.8.3** **SVC intra decoding processes**


Clause G.8.3.1 specifies the SVC derivation process for intra prediction modes.


Clause G.8.3.2 specifies the SVC intra sample prediction and construction process.


**G.8.3.1** **SVC derivation process for intra prediction modes**


This process is only invoked when base_mode_flag is equal to 0 and mbType[ CurrMbAddr ] specified as input to this
process is equal to I_PCM, I_16x16, I_8x8, or I_4x4.


Inputs to this process are:


- a one-dimensional array sliceIdc with PicSizeInMbs elements specifying slice identifications for the macroblocks of
the current layer representation,


- a list fieldMbFlag with PicSizeInMbs elements specifying which macroblocks of the current layer representation are
coded as field macroblocks and which macroblocks are coded as frame macroblocks,


- a list baseModeFlag with PicSizeInMbs elements specifying the syntax element base_mode_flag for the macroblocks
of the current layer representation,


- a list mbType with PicSizeInMbs elements specifying macroblock types for the macroblocks of the current layer
representation,


- a (PicSizeInMbs)x16 array ipred4x4 specifying Intra_4x4 prediction modes for macroblocks of the current layer
representation,


- a (PicSizeInMbs)x4 array ipred8x8 specifying Intra_8x8 prediction modes for macroblocks of the current layer
representation,


- a list ipred16x16 with PicSizeInMbs elements specifying Intra_16x16 prediction modes for macroblocks of the
current layer representation,


- when ChromaArrayType is equal to 1 or 2, a list ipredChroma with PicSizeInMbs elements specifying intra chroma
prediction modes for macroblocks of the current layer representation.


Outputs of this process are:


- a modified version of the array ipred4x4,


- a modified version of the array ipred8x8,


- a modified version of the list ipred16x16,


- when ChromaArrayType is equal to 1 or 2, a modified version of the array ipredChroma.


For all processes specified in clause 6 that are invoked from the process specified in this clause or a child process of the
process specified in this clause, the following modifications apply:


a) In clause 6.4.12.2, a macroblock with address mbAddr is treated as field macroblock when

fieldMbFlag[ mbAddr ] is equal to 1, and it is treated as frame macroblock when fieldMbFlag[ mbAddr ] is equal
to 0. In particular, the current macroblock is treated as field macroblock when fieldMbFlag[ CurrMbAddr ] is
equal to 1, and it is treated as frame macroblock when fieldMbFlag[ CurrMbAddr ] is equal to 0.


b) In clause 6.4.8, a macroblock with address mbAddr is treated to belong to a different slice than the current

macroblock CurrMbAddr, when sliceIdc[ mbAddr ] is not equal to sliceIdc[ CurrMbAddr ].


c) In clause 6.4.12.2, a macroblock mbAddr is treated as top macroblock when (mbAddr % 2) is equal to 0, and it

is treated as bottom macroblock when (mbAddr % 2) is equal to 1.


When mbType[ CurrMbAddr ] is not equal to I_PCM, the following applies:


- If mbType[ CurrMbAddr ] is equal to I_4x4, the SVC derivation process for Intra_4x4 prediction modes as specified
in clause G.8.3.1.1 is invoked with sliceIdc, fieldMbFlag, baseModeFlag, mbType, ipred4x4, and ipred8x8 as the
inputs and the output is a modified version of the array ipred4x4.


- Otherwise, if mbType[ CurrMbAddr ] is equal to I_8x8, the SVC derivation process for Intra_8x8 prediction modes
as specified in clause G.8.3.1.2 is invoked with sliceIdc, fieldMbFlag, baseModeFlag, mbType, ipred4x4, and
ipred8x8 as the inputs and the output is a modified version of the array ipred8x8.


- Otherwise, if mbType[ CurrMbAddr ] is equal to I_16x16, ipred16x16[ CurrMbAddr ] is set equal to
Intra16x16PredMode.





When ChromaArrayType is equal to 1 or 2 and mbType[ CurrMbAddr ] is not equal to I_PCM,
ipredChroma[ CurrMbAddr ] is set equal to intra_chroma_pred_mode.


**G.8.3.1.1** **SVC derivation process for Intra_4x4 prediction modes**


Inputs to this process are:


- a one-dimensional array sliceIdc with PicSizeInMbs elements specifying slice identifications for the macroblocks of
the current layer representation,


- a list fieldMbFlag with PicSizeInMbs elements specifying which macroblocks of the current layer representation are
coded as field macroblocks and which macroblocks are coded as frame macroblocks,


- a list baseModeFlag with PicSizeInMbs elements specifying the syntax element base_mode_flag for the macroblocks
of the current layer representation,


- a list mbType with PicSizeInMbs elements specifying macroblock types for the macroblocks of the current layer
representation,


- a (PicSizeInMbs)x16 array ipred4x4 specifying Intra_4x4 prediction modes for macroblocks of the current layer
representation,


- a (PicSizeInMbs)x4 array ipred8x8 specifying Intra_8x8 prediction modes for macroblocks of the current layer
representation.


Output of this process is a modified version of the array ipred4x4.


The 4x4 blocks indexed by c4x4BlkIdx = 0..15 are processed in increasing order of c4x4BlkIdx, and for each 4x4 block,
the following ordered steps are specified:


1. The derivation process for neighbouring 4x4 luma blocks as specified in clause 6.4.11.4 is invoked with

c4x4BlkIdx as the input and the outputs are assigned to mbAddrA, c4x4BlkIdxA, mbAddrB, and c4x4BlkIdxB.
For this invocation of the process in clause 6.4.11.4, the modifications specified in items a) through c) in
clause G.8.3.1 apply.


2. For N being replaced by A and B, the variables availableFlagN are derived as follows:


     - If the macroblock mbAddrN is available and any of the following conditions are true, availableFlagN is set
equal to 1:


        - constrained_intra_pred_flag is equal to 0,


        - mbType[ mbAddrN ] is equal to I_PCM and tcoeff_level_prediction_flag is equal to 1,


        - mbType[ mbAddrN ] is equal to I_PCM and baseModeFlag[ mbAddrN ] is equal to 0,


        - mbType[ mbAddrN ] is equal to I_16x16, I_8x8, or I_4x4.


     - Otherwise, availableFlagN is set equal to 0.


3. The variable dcPredModePredictedFlag is derived as follows:


      - If availableFlagA or availableFlagB is equal to 0, dcPredModePredictedFlag is set equal to 1.


      - Otherwise (availableFlagA is equal to 1 and availableFlagB is equal to 1), dcPredModePredictedFlag is set
equal to 0.


4. For N being replaced by A and B, the variables intraMxMPredModeN are derived as follows:


      - If dcPredModePredictedFlag is equal to 0 and mbType[ mbAddrN ] is equal to I_4x4,
intraMxMPredModeN is set equal to ipred4x4[ mbAddrN ][ c4x4BlkIdxN ].


      - Otherwise, if dcPredModePredictedFlag is equal to 0 and mbType[ mbAddrN ] is equal to I_8x8,
intraMxMPredModeN is set equal to ipred8x8[ mbAddrN ][ c4x4BlkIdxN >> 2 ].


      - Otherwise (dcPredModePredictedFlag is equal to 1 or (mbType[ mbAddrN ] is not equal to I_4x4 and
mbType[ mbAddrN ] is not equal to I_8x8)), intraMxMPredModeN is set equal to 2.


5. The element ipred4x4[ CurrMbAddr ][ c4x4BlkIdx ] of the array ipred4x4 is derived by applying the procedure

specified in the following pseudo-code:


predIntra4x4PredMode = Min( intraMxMPredModeA, intraMxMPredModeB )
if( prev_intra4x4_pred_mode_flag[ c4x4BlkIdx ] )
ipred4x4[ CurrMbAddr ][ c4x4BlkIdx ] = predIntra4x4PredMode





else if( rem_intra4x4_pred_mode[ c4x4BlkIdx ] < predIntra4x4PredMode ) (G-85)
ipred4x4[ CurrMbAddr ][ c4x4BlkIdx ] = rem_intra4x4_pred_mode[ c4x4BlkIdx ]
else
ipred4x4[ CurrMbAddr ][ c4x4BlkIdx ] = rem_intra4x4_pred_mode[ c4x4BlkIdx ] + 1


**G.8.3.1.2** **SVC derivation process for Intra_8x8 prediction modes**


Inputs to this process are:


- a one-dimensional array sliceIdc with PicSizeInMbs elements specifying slice identifications for the macroblocks of
the current layer representation,


- a list fieldMbFlag with PicSizeInMbs elements specifying which macroblocks of the current layer representation are
coded as field macroblocks and which macroblocks are coded as frame macroblocks,


- a list baseModeFlag with PicSizeInMbs elements specifying the syntax element base_mode_flag for the macroblocks
of the current layer representation,


- a list mbType with PicSizeInMbs elements specifying macroblock types for the macroblocks of the current layer
representation,


- a (PicSizeInMbs)x16 array ipred4x4 specifying Intra_4x4 prediction modes for macroblocks of the current layer
representation,


- a (PicSizeInMbs)x4 array ipred8x8 specifying Intra_8x8 prediction modes for macroblocks of the current layer
representation.


Output of this process is a modified version of the array ipred8x8.


The 8x8 blocks indexed by c8x8BlkIdx = 0..3 are processed in increasing order of c8x8BlkIdx, and for each 8x8 block,
the following ordered steps are specified:


1. The derivation process for neighbouring 8x8 luma blocks as specified in clause 6.4.11.2 is invoked with

c8x8BlkIdx as the input and the outputs are assigned to mbAddrA, c8x8BlkIdxA, mbAddrB, and c8x8BlkIdxB.
For this invocation of the process in clause 6.4.11.2, the modifications specified in items a) through c) in
clause G.8.3.1 apply.


2. For N being replaced by A and B, the variables availableFlagN are derived as follows:


     - If the macroblock mbAddrN is available and any of the following conditions are true, availableFlagN is set
equal to 1:


        - constrained_intra_pred_flag is equal to 0,


        - mbType[ mbAddrN ] is equal to I_PCM and tcoeff_level_prediction_flag is equal to 1,


        - mbType[ mbAddrN ] is equal to I_PCM and baseModeFlag[ mbAddrN ] is equal to 0,


        - mbType[ mbAddrN ] is equal to I_16x16, I_8x8, or I_4x4.


     - Otherwise, availableFlagN is set equal to 0.


3. The variable dcPredModePredictedFlag is derived as follows:


      - If availableFlagA or availableFlagB is equal to 0, dcPredModePredictedFlag is set equal to 1.


      - Otherwise (availableFlagA is equal to 1 and availableFlagB are equal to 1), dcPredModePredictedFlag is
set equal to 0.


4. For N being replaced by A and B, the variables intraMxMPredModeN are derived as follows:


      - If dcPredModePredictedFlag is equal to 0 and mbType[ mbAddrN ] is equal to I_4x4,
intraMxMPredModeN is set equal to ipred4x4[ mbAddrN ][ c8x8BlkIdxN * 4 + c4x4Idx ] with the variable
c4x4Idx being derived as follows:


        - If N is equal to B, c4x4Idx is set equal to 2.


        - Otherwise, if fieldMbFlag[ CurrMbAddr ] is equal to 0, fieldMbFlag[ mbAddrN ] is equal to 1, and
c8x8BlkIdx is equal to 2, c4x4Idx is set equal to 3.


        - Otherwise (N is equal to A and (fieldMbFlag[ CurrMbAddr ] is equal to 1 or fieldMbFlag[ mbAddrN ]
is equal to 0 or c8x8BlkIdx is not equal to 2)), c4x4Idx is set equal to 1.





      - Otherwise, if dcPredModePredictedFlag is equal to 0 and mbType[ mbAddrN ] is equal to I_8x8,
intraMxMPredModeN is set equal to ipred8x8[ mbAddrN ][ c8x8BlkIdxN ].


      - Otherwise (dcPredModePredictedFlag is equal to 1 or (mbType[ mbAddrN ] is not equal to I_4x4 and
mbType[ mbAddrN ] is not equal to I_8x8)), intraMxMPredModeN is set equal to 2.


5. The element ipred8x8[ CurrMbAddr ][ c8x8BlkIdx ] of the array ipred8x8 is derived by applying the procedure

specified in the following pseudo-code:


predIntra8x8PredMode = Min( intraMxMPredModeA, intraMxMPredModeB )
if( prev_intra8x8_pred_mode_flag[ c8x8BlkIdx ] )
ipred8x8[ CurrMbAddr ][ c8x8BlkIdx ] = predIntra8x8PredMode
else if( rem_intra8x8_pred_mode[ c8x8BlkIdx ] < predIntra8x8PredMode ) (G-86)
ipred8x8[ CurrMbAddr ][ c8x8BlkIdx ] = rem_intra8x8_pred_mode[ c8x8BlkIdx ]
else
ipred8x8[ CurrMbAddr ][ c8x8BlkIdx ] = rem_intra8x8_pred_mode[ c8x8BlkIdx ] + 1


**G.8.3.2** **SVC intra sample prediction and construction process**


This process is only invoked when mbType specified as input to this process is equal to I_PCM, I_16x16, I_8x8, or I_4x4.


Inputs to this process are:


- a one-dimensional array sliceIdc with PicSizeInMbs elements specifying slice identifications for the macroblocks of
the current layer representation,


- a one-dimensional array fieldMbFlag with PicSizeInMbs elements specifying which macroblocks of the current layer
representation are coded as field macroblocks and which macroblocks are coded as frame macroblocks,


- a list baseModeFlag with PicSizeInMbs elements specifying the syntax element base_mode_flag for the macroblocks
of the current layer representation,


- a one-dimensional array mbType with PicSizeInMbs elements specifying macroblock types for the macroblocks of
the current layer representation,


- a list ipred4x4 with 16 elements specifying Intra_4x4 prediction modes for the current macroblock,


- a list ipred8x8 with 4 elements specifying Intra_8x8 prediction modes for the current macroblock,


- a variable ipred16x16 specifying the Intra_16x16 prediction mode for the current macroblock,


- a variable ipredChroma specifying the intra chroma prediction mode for the current macroblock,


- a variable cTrafo specifying the transform type for the current macroblock,


- a list of scaled transform coefficient values sTCoeff with (256 + 2 * MbWidthC * MbHeightC) elements,


- a (PicWidthInSamplesL)x(PicHeightInSamplesL) array picSamplesL containing constructed luma sample values for
the current layer representation.


- when ChromaArrayType is not equal to 0, two (PicWidthInSamplesC)x(PicHeightInSamplesC) arrays picSamplesCb
and picSamplesCr containing constructed chroma sample values for the current layer representation.


Outputs of this process are:


- a modified version of the array picSamplesL,


- when ChromaArrayType is not equal to 0, modified versions of the arrays picSamplesCb and picSamplesCr.


For all processes specified in clauses 6 or 8 that are invoked from the process specified in this clause or a child process of
the process specified in this clause, the following modifications apply.


a) In clause 6.4.12.2, a macroblock with address mbAddr is treated as field macroblock when

fieldMbFlag[ mbAddr ] is equal to 1, and it is treated as frame macroblock when fieldMbFlag[ mbAddr ] is equal
to 0. In particular, the current macroblock is treated as field macroblock when fieldMbFlag[ CurrMbAddr ] is
equal to 1, and it is treated as frame macroblock when fieldMbFlag[ CurrMbAddr ] is equal to 0.


b) In clause 6.4.8, a macroblock with address mbAddr is treated to belong to a different slice than the current

macroblock CurrMbAddr, when MbToSliceGroupMap[ mbAddr ] is not equal to
MbToSliceGroupMap[ CurrMbAddr ] or mbAddr is less than ( ( sliceIdc[ CurrMbAddr ] >> 7 ) *
( 1 + MbaffFrameFlag ) ), where MbToSliceGroupMap represents the variable that is derived as specified in
clause 8.2.2 for the layer representation with DQId equal to (sliceIdc[ CurrMbAddr ] & 127).





NOTE 1 – When MaxTCoeffLevelPredFlag is equal to 0 or when all macroblocks of the current layer picture are covered
by slices with the same value of DQId, the above condition can be simplified. In this case, two macroblocks mbAddrA
and mbAddrB can be treated to belong to different slices, when sliceIdc[ mbAddrA ] is not equal to sliceIdc[ mbAddrB ].


c) In clause 6.4.12.2, a macroblock mbAddr is treated as top macroblock when (mbAddr % 2) is equal to 0, and it

is treated as bottom macroblock when (mbAddr % 2) is equal to 1.


d) In clauses 8.3.1.2, 8.3.2.2, 8.3.3, and 8.3.4, the variables Intra4x4PredMode, Intra8x8PredMode,

Intra16x16PredMode, and intra_chroma_pred_mode are replaced by ipred4x4, ipred8x8, ipred16x16, and
ipredChroma, respectively.


e) In clauses 8.3.1.2, 8.3.2.2, 8.3.3, and 8.3.4, the syntax element mb_type of a macroblock with macroblock address

mbAddr is replaced by mbType[ mbAddr ].


f) The value of constrained_intra_pred_flag that is referred to in clauses 8.3.1.2, 8.3.2.2, 8.3.3, and 8.3.4 is specified
as follows:


     - If (sliceIdc[ CurrMbAddr ] & 127) is less than DQIdMax, the value of constrained_intra_pred_flag is the
value of constrained_intra_pred_flag of the active layer picture parameter set for the layer representation
with DQId equal to (sliceIdc[ CurrMbAddr ] & 127).


     - Otherwise ((sliceIdc[ CurrMbAddr ] & 127) is equal to DQIdMax), the value of
constrained_intra_pred_flag is the value of constrained_intra_pred_flag of the active picture parameter set.


g) In clauses 8.3.1.2, 8.3.2.2, 8.3.3, and 8.3.4, a macroblock with mbAddrN is treated as coded in an Inter

macroblock prediction mode when all of the following conditions are false:


     - mbType[ mbAddrN ] is equal to I_PCM and tcoeff_level_prediction_flag for the slice with DQId equal to
(sliceIdc[ mbAddrN ] & 127) and first_mb_in_slice equal to (sliceIdc[ mbAddrN ] >> 7) is equal to 1,


     - mbType[ mbAddrN ] is equal to I_PCM and baseModeFlag[ mbAddrN ] is equal to 0,


     - mbType[ mbAddrN ] is equal to I_16x16, I_8x8, or I_4x4,


     - sliceIdc[ mbAddrN ] is not equal to sliceIdc[ CurrMbAddr ].
NOTE 2 – The latter condition does only have an impact on the decoding process when MaxTCoeffLevelPredFlag is
equal to 1 and not all macroblocks of the current layer picture are covered by slices with the same value of DQId.
NOTE 3 – Encoder designers are encouraged to generate bitstreams for which the removal of zero or more slice data
NAL units with quality_id greater than 0 cannot result in a conforming bitstream for which a macroblock with address
mbAddr is intra-predicted from a macroblock with address mbAddrN and sliceIdc[ mbAddrN ] not equal to
sliceIdc[ mbAddr ].


The SVC intra sample prediction and construction process proceeds in the following ordered steps:


1. The construction process for luma residuals or chroma residuals with ChromaArrayType equal to 3 as specified

in clause G.8.5.3.1 is invoked with cTrafo and sTCoeff as the inputs and the outputs are residual luma sample
values as a 16x16 array mbResL with elements mbResL[ x, y ].


2. When ChromaArrayType is not equal to 0, the construction process for chroma residuals as specified in

clause G.8.5.3.2 is invoked with cTrafo and sTCoeff as the inputs and the outputs are residual chroma sample
values as two (MbWidthC)x(MbHeightC) arrays mbResCb and mbResCr with elements mbResCb[ x, y ] and
mbResCr[ x, y ], respectively.


3. The SVC intra prediction and construction process for luma samples or chroma samples with ChromaArrayType

equal to 3 as specified in clause G.8.3.2.1 is invoked with BitDepthY, sliceIdc, fieldMbFlag, mbType, ipred4x4,
ipred8x8, ipred16x16, mbResL, and picSamplesL as the inputs and the output is a modified version of the array
picSamplesL.


4. When ChromaArrayType is not equal to 0, the SVC intra prediction and construction process for chroma samples

as specified in clause G.8.3.2.2 is invoked with sliceIdc, fieldMbFlag, mbType, ipred4x4, ipred8x8, ipred16x16,
ipredChroma, mbResCb, mbResCr, picSamplesCb, and picSamplesCr as the inputs and the outputs are modified
versions of the arrays picSamplesCb and picSamplesCr.


**G.8.3.2.1** **SVC intra prediction and construction process for luma samples or chroma samples with**

**ChromaArrayType equal to 3**


Inputs to this process are:


- a variable bitDepth specifying the bit depth,


- a one-dimensional array sliceIdc with PicSizeInMbs elements specifying slice identifications for the macroblocks of
the current layer representation,





- a one-dimensional array fieldMbFlag with PicSizeInMbs elements specifying which macroblocks of the current layer
representation are coded as field macroblocks and which macroblocks are coded as frame macroblocks,


- a one-dimensional array mbType with PicSizeInMbs elements specifying macroblock types for the macroblocks of
the current layer representation,


- a list ipred4x4 with 16 elements specifying Intra_4x4 prediction modes for the current macroblock,


- a list ipred8x8 with 4 elements specifying Intra_8x8 prediction modes for the current macroblock,


- a variable ipred16x16 specifying the Intra_16x16 prediction mode for the current macroblock,


- a 16x16 array mbRes containing residual sample values for the current macroblock,


- a (PicWidthInSamplesL)x(PicHeightInSamplesL) array picSamples containing constructed sample values for the
current layer representation.


Outputs of this process is a modified version of the array picSamples.


Depending on mbType[ CurrMbAddr ], the following applies:


- If mbType[ CurrMbAddr ] is equal to I_PCM, the SVC construction process for luma samples and chroma samples
with ChromaArrayType equal to 3 of I_PCM macroblocks as specified in clause G.8.3.2.1.1 is invoked with
fieldMbFlag, mbRes, and picSamples as the inputs and the output is a modified version of the array picSamples.


- Otherwise, if mbType[ CurrMbAddr ] is equal to I_4x4, the SVC Intra_4x4 sample prediction and construction
process as specified in clause G.8.3.2.1.2 is invoked with bitDepth, sliceIdc, fieldMbFlag, mbType, ipred4x4, mbRes,
and picSamples as the inputs and the output is a modified version of the array picSamples.


- Otherwise, if mbType[ CurrMbAddr ] is equal to I_8x8, the SVC Intra_8x8 sample prediction and construction
process as specified in clause G.8.3.2.1.3 is invoked with bitDepth, sliceIdc, fieldMbFlag, mbType, ipred8x8, mbRes,
and picSamples as the inputs and the output is a modified version of the array picSamples.


- Otherwise (mbType[ CurrMbAddr ] is equal to I_16x16), the SVC Intra_16x16 sample prediction and construction
process as specified in clause G.8.3.2.1.4 is invoked with bitDepth, sliceIdc, fieldMbFlag, mbType, ipred16x16,
mbRes, and picSamples as the inputs and the output is a modified version of the array picSamples.


**G.8.3.2.1.1** **SVC construction process for luma samples and chroma samples with ChromaArrayType equal to 3**
**of I_PCM macroblocks**


Inputs to this process are:


- a one-dimensional array fieldMbFlag with PicSizeInMbs elements specifying which macroblocks of the current layer
representation are coded as field macroblocks and which macroblocks are coded as frame macroblocks,


- a 16x16 array mbRes containing residual sample values for the current macroblock,


- a (PicWidthInSamplesL)x(PicHeightInSamplesL) array picSamples containing constructed sample values for the
current layer representation.


Output of this process is a modified version of the array picSamples.


The picture sample array construction process for a signal component as specified in clause G.8.5.4.3 is invoked with
fieldMbFlag[ CurrMbAddr ], mbW set equal to 16, mbH set equal to 16, mbRes, and picSamples as the inputs and the
output is a modified version of the array picSamples.


**G.8.3.2.1.2** **SVC Intra_4x4 sample prediction and construction process**


Inputs to this process are:


- a variable bitDepth specifying the bit depth,


- a one-dimensional array sliceIdc with PicSizeInMbs elements specifying slice identifications for the macroblocks of
the current layer representation,


- a one-dimensional array fieldMbFlag with PicSizeInMbs elements specifying which macroblocks of the current layer
representation are coded as field macroblocks and which macroblocks are coded as frame macroblocks,


- a one-dimensional array mbType with PicSizeInMbs elements specifying macroblock types for the macroblocks of
the current layer representation,


- a list ipred4x4 with 16 elements specifying Intra_4x4 prediction modes for the current macroblock,


- a 16x16 array mbRes containing residual sample values for the current macroblock,





- a (PicWidthInSamplesL)x(PicHeightInSamplesL) array picSamples containing constructed sample values for the
current layer representation.


Output of this process is a modified version of the array picSamples.


Let mbSamples be a 16x16 array containing constructed intra sample values for the current macroblock. All elements of
mbSamples are initially set equal to 0.


The 4x4 blocks indexed by c4x4BlkIdx = 0..15 are processed in increasing order of c4x4BlkIdx, and for each 4x4 block,
the following ordered steps are specified:


1. The Intra_4x4 sample prediction process as specified in clause 8.3.1.2 is invoked with c4x4BlkIdx and

picSamples as the inputs and the outputs are intra prediction sample values as a 4x4 array pred4x4 with elements
pred4x4[ x, y ]. For this invocation of the process in clause 8.3.1.2, the modifications specified in items a)
through g) of clause G.8.3.2 apply. Additionally in clause 8.3.1.2.3, which may be invoked as part of the process
specified in clause 8.3.1.2, the variable BitDepthY is replaced by bitDepth.


2. The inverse 4x4 luma block scanning process as specified in clause 6.4.3 is invoked with c4x4BlkIdx as the input

and the output is assigned to ( xP, yP ).


3. For x = xP..(xP + 3) and y = yP..(yP + 3) and with Clip( a ) specifying Clip3( 0, ( 1 << bitDepth ) − 1, a ), the

elements mbSamples[ x, y ] of the 16x16 array mbSamples are derived by


mbSamples[ x, y ] = Clip( pred4x4[ x − xP, y − yP ] + mbRes[ x, y ] ) (G-87)


4. The picture sample array construction process for a signal component as specified in clause G.8.5.4.3 is invoked

with fieldMbFlag[ CurrMbAddr ], mbW set equal to 16, mbH set equal to 16, mbSamples, and picSamples as the
inputs and the output is a modified version of the array picSamples.

NOTE – When c4x4BlkIdx is less than 15, the array mbSamples does only contain constructed intra samples for 4x4
blocks with c4x4BlkIdx less than or equal to the current value of c4x4BlkIdx.


**G.8.3.2.1.3** **SVC Intra_8x8 sample prediction and construction process**


Inputs to this process are:


- a variable bitDepth specifying the bit depth,


- a one-dimensional array sliceIdc with PicSizeInMbs elements specifying slice identifications for the macroblocks of
the current layer representation,


- a one-dimensional array fieldMbFlag with PicSizeInMbs elements specifying which macroblocks of the current layer
representation are coded as field macroblocks and which macroblocks are coded as frame macroblocks,


- a one-dimensional array mbType with PicSizeInMbs elements specifying macroblock types for the macroblocks of
the current layer representation,


- a list ipred8x8 with 4 elements specifying Intra_8x8 prediction modes for the current macroblock,


- a 16x16 array mbRes containing residual sample values for the current macroblock,


- a (PicWidthInSamplesL)x(PicHeightInSamplesL) array picSamples containing constructed sample values for the
current layer representation.


Output of this process is a modified version of the array picSamples.


Let mbSamples be a 16x16 array containing constructed intra sample values for the current macroblock. All elements of
mbSamples are initially set equal to 0.


The 8x8 blocks indexed by c8x8BlkIdx = 0..3 are processed in increasing order of c8x8BlkIdx, and for each 8x8 block,
the following ordered steps are specified:


1. The Intra_8x8 sample prediction process as specified in clause 8.3.2.2 is invoked with c8x8BlkIdx and

picSamples as the inputs and the outputs are intra prediction sample values as an 8x8 array pred8x8 with elements
pred8x8[ x, y ]. For this invocation of the process in clause 8.3.2.2, the modifications specified in items a)
through g) of clause G.8.3.2 apply. Additionally in clause 8.3.2.2.4, which may be invoked as part of the process
specified in clause 8.3.2.2, the variable BitDepthY is replaced by bitDepth.


2. The inverse 8x8 luma block scanning process as specified in clause 6.4.5 is invoked with c8x8BlkIdx as the input

and the output is assigned to ( xP, yP ).


3. For x = xP..(xP + 7) and y = yP..(yP + 7) and with Clip( a ) specifying Clip3( 0, ( 1 << bitDepth ) − 1, a ), the

elements mbSamples[ x, y ] of the 16x16 array mbSamples are derived by





mbSamples[ x, y ] = Clip( pred8x8[ x − xP, y − yP ] + mbRes[ x, y ] ) (G-88)


4. The picture sample array construction process for a signal component as specified in clause G.8.5.4.3 is invoked

with fieldMbFlag[ CurrMbAddr ], mbW set equal to 16, mbH set equal to 16, mbSamples, and picSamples as the
inputs and the output is a modified version of the array picSamples.

NOTE – When c8x8BlkIdx is less than 3, the array mbSamples does only contain constructed intra samples for 8x8
blocks with c8x8BlkIdx less than or equal to the current value of c8x8BlkIdx.


**G.8.3.2.1.4** **SVC Intra_16x16 sample prediction and construction process**


Inputs to this process are:


- a variable bitDepth specifying the bit depth,


- a one-dimensional array sliceIdc with PicSizeInMbs elements specifying slice identifications for the macroblocks of
the current layer representation,


- a one-dimensional array fieldMbFlag with PicSizeInMbs elements specifying which macroblocks of the current layer
representation are coded as field macroblocks and which macroblocks are coded as frame macroblocks,


- a one-dimensional array mbType with PicSizeInMbs elements specifying macroblock types for the macroblocks of
the current layer representation,


- a variable ipred16x16 specifying the Intra_16x16 prediction mode for the current macroblock,


- a 16x16 array mbRes containing residual sample values for the current macroblock,


- a (PicWidthInSamplesL)x(PicHeightInSamplesL) array picSamples containing constructed sample values for the
current layer representation.


Output of this process is a modified version of the array picSamples.


The SVC Intra_16x16 sample prediction and construction process proceeds in the following ordered steps:


1. The Intra_16x16 prediction process for luma samples as specified in clause 8.3.3 is invoked with picSamples as

the input and the outputs are intra prediction sample values as a 16x16 array pred16x16 with elements
pred16x16[ x, y ]. For this invocation of the process in clause 8.3.3, the modifications specified in items a)
through g) of clause G.8.3.2 apply. Additionally in clause 8.3.3.3, which may be invoked as part of the process
specified in clause 8.3.3, the variable BitDepthY is replaced by bitDepth.


2. With Clip( a ) specifying Clip3( 0, ( 1 << bitDepth ) − 1, a ), the 16x16 array mbSamples is derived by


mbSamples[ x, y ] = Clip( pred16x16[ x, y ] + mbRes[ x, y ] )       with x, y = 0..15 (G-89)


3. The picture sample array construction process for a signal component as specified in clause G.8.5.4.3 is invoked

with fieldMbFlag[ CurrMbAddr ], mbW set equal to 16, mbH set equal to 16, mbSamples, and picSamples as the
inputs and the output is a modified version of the array picSamples.


**G.8.3.2.2** **SVC intra prediction and construction process for chroma samples**


Inputs to this process are:


- a one-dimensional array sliceIdc with PicSizeInMbs elements specifying slice identifications for the macroblocks of
the current layer representation,


- a one-dimensional array fieldMbFlag with PicSizeInMbs elements specifying which macroblocks of the current layer
representation are coded as field macroblocks and which macroblocks are coded as frame macroblocks,


- a one-dimensional array mbType with PicSizeInMbs elements specifying macroblock types for the macroblocks of
the current layer representation,


- a list ipred4x4 with 16 elements specifying Intra_4x4 prediction modes for the current macroblock,


- a list ipred8x8 with 4 elements specifying Intra_8x8 prediction modes for the current macroblock,


- a variable ipred16x16 specifying the Intra_16x16 prediction mode for the current macroblock,


- a variable ipredChroma specifying the intra chroma prediction mode for the current macroblock,


- two (MbWidthC)x(MbHeightC) arrays mbResCb and mbResCr containing residual chroma sample values for the
current macroblock,


- two (PicWidthInSamplesC)x(PicHeightInSamplesC) arrays picSamplesCb and picSamplesCr containing constructed
sample values for the current layer representation.





Outputs of this process are modified versions of the arrays picSamplesCb and picSamplesCr.


Depending on ChromaArrayType, the following applies:


- If ChromaArrayType is equal to 1 or 2, the following applies:


   - If mbType[ CurrMbAddr ] is equal to I_PCM, the SVC construction process for chroma samples of I_PCM
macroblock as specified in clause G.8.3.2.2.1 is invoked with fieldMbFlag, mbResCb, mbResCr, picSamplesCb,
and picSamplesCr as the inputs and the outputs are modified versions of picSamplesCb and picSamplesCr.


   - Otherwise (mbType[ CurrMbAddr ] is not equal to I_PCM), the SVC intra prediction and construction process
for chroma samples with ChromaArrayType equal to 1 or 2 as specified in clause G.8.3.2.2.2 is invoked with
sliceIdc, fieldMbFlag, mbType, ipredChroma, mbResCb, mbResCr, picSamplesCb, and picSamplesCr as the inputs
and the outputs are modified versions of the arrays picSamplesCb and picSamplesCr.


- Otherwise (ChromaArrayType is equal to 3), for CX being replaced by Cb and Cr, the SVC intra prediction and
construction process for luma samples or chroma samples with ChromaArrayType equal to 3 as specified in
clause G.8.3.2.1 is invoked with BitDepthC, sliceIdc, fieldMbFlag, mbType, ipred4x4, ipred8x8, ipred16x16,
mbResCX, and picSamplesCX as the inputs and the output is a modified version of the array picSamplesCX.


**G.8.3.2.2.1** **SVC construction process for chroma samples of I_PCM macroblocks**


Inputs to this process are:


- a one-dimensional array fieldMbFlag with PicSizeInMbs elements specifying which macroblocks of the current layer
representation are coded as field macroblocks and which macroblocks are coded as frame macroblocks,


- two (MbWidthC)x(MbHeightC) arrays mbResCb and mbResCr containing residual chroma sample values for the
current macroblock,


- two (PicWidthInSamplesC)x(PicHeightInSamplesC) arrays picSamplesCb and picSamplesCr containing constructed
chroma sample values for the current layer representation.


Outputs of this process are modified versions of the arrays picSamplesCb and picSamplesCr.


For CX being replaced by Cb and Cr, the picture sample array construction process for a signal component as specified in
clause G.8.5.4.3 is invoked with fieldMbFlag[ CurrMbAddr ], mbW set equal to MbWidthC, mbH set equal to MbHeightC,
mbResCX, and picSamplesCX as the inputs and the output is a modified version of the array picSamplesCX.


**G.8.3.2.2.2** **SVC intra prediction and construction process for chroma samples with ChromaArrayType equal to 1**
**or 2**


This process is only invoked when ChromaArrayType is equal to 1 or 2.


Inputs to this process are:


- a one-dimensional array sliceIdc with PicSizeInMbs elements specifying slice identifications for the macroblocks of
the current layer representation,


- a one-dimensional array fieldMbFlag with PicSizeInMbs elements specifying which macroblocks of the current layer
representation are coded as field macroblocks and which macroblocks are coded as frame macroblocks,


- a one-dimensional array mbType with PicSizeInMbs elements specifying macroblock types for the macroblocks of
the current layer representation,


- a variable ipredChroma specifying the intra chroma prediction mode for the current macroblock,


- two (MbWidthC)x(MbHeightC) arrays mbResCb and mbResCr containing residual chroma sample values for the
current macroblock,


- two (PicWidthInSamplesC)x(PicHeightInSamplesC) arrays picSamplesCb and picSamplesCr containing constructed
chroma sample values for the current layer representation.


Outputs of this process are modified versions of the arrays picSamplesCb and picSamplesCr.


The SVC intra prediction and construction process for chroma samples with ChromaArrayType equal to 1 or 2 proceeds
in the following ordered steps:


1. The intra prediction process for chroma samples as specified in clause 8.3.4 is invoked with picSamplesCb and

picSamplesCr as the inputs and the outputs are intra prediction chroma sample values as two
(MbWidthC)x(MbHeightC) arrays predCb and predCr with elements predCb[ x, y ] and predCr[ x, y ], respectively.
For this invocation of the process in clause 8.3.4, the modifications specified in items a) through g ) of
clause G.8.3.2 apply.





2. For CX being replaced by Cb and Cr, the (MbWidthC)x(MbHeightC) array mbSamplesCX is derived by


mbSamplesCX[ x, y ] = Clip1C( predCX[ x, y ] + mbResCX[ x, y ] )  with x = 0..(MbWidthC − 1)
and y = 0..(MbHeightC − 1) (G-90)


3. For CX being replaced by Cb and Cr, the picture sample array construction process for a signal component as

specified in clause G.8.5.4.3 is invoked with fieldMbFlag[ CurrMbAddr ], mbW set equal to MbWidthC, mbH
set equal to MbHeightC, mbSamplesCX, and picSamplesCX as the inputs and the output is a modified version of
the array picSamplesCX.


**G.8.4** **SVC Inter prediction process**


Clause G.8.4.1 specifies the SVC derivation process for motion vector components and reference indices.


Clause G.8.4.2 specifies the SVC decoding process for Inter prediction samples


**G.8.4.1** **SVC derivation process for motion vector components and reference indices**


Inputs to this process are:


- a one-dimensional array sliceIdc with PicSizeInMbs elements specifying slice identifications for the macroblocks of
the current layer representation,


- a one-dimensional array fieldMbFlag with PicSizeInMbs elements specifying which macroblocks of the current layer
representation are coded as field macroblocks and which macroblocks are coded as frame macroblocks,


- a one-dimensional array mbType with PicSizeInMbs elements specifying macroblock types for the macroblocks of
the current layer representation,


- a (PicSizeInMbs)x4 array subMbType specifying sub-macroblock types for the macroblocks of the current layer
representation,


- two (PicSizeInMbs)x4 arrays predFlagL0 and predFlagL1 specifying prediction utilization flags for the macroblocks
of the current layer representation,


- two (PicSizeInMbs)x4 arrays refIdxL0 and refIdxL1 specifying reference indices for the macroblocks of the current
layer representation,


- two (PicSizeInMbs)x4x4x2 arrays mvL0 and mvL1 specifying motion vector components for the macroblocks of the
current layer representation,


- a one-dimensional array mvCnt with PicSizeInMbs elements specifying the number of motion vectors for the
macroblocks of the current layer representation,


- two 2x2 arrays refIdxILPredL0 and refIdxILPredL1 specifying inter-layer reference index predictors for the current
macroblock,


- two 4x4x2 arrays mvILPredL0 and mvILPredL1 specifying inter-layer motion vector predictors for the current
macroblock,


- when DQId is equal to 0 and (slice_type % 5) is equal to 1, the reference list refPicList1.


Outputs of this process are:


- modified versions of the arrays predFlagL0 and predFlagL1,


- modified versions of the arrays refIdxL0 and refIdxL1,


- modified versions of the arrays mvL0 and mvL1,


- a modified version of the array mvCnt.


Depending on mbType[ CurrMbAddr ], the following applies:


- If mbType[ CurrMbAddr ] is equal to I_PCM, I_16x16, I_8x8, I_4x4, or I_BL, the arrays predFlagL0, predFlagL1,
refIdxL0, refIdxL1, mvL0, mvL1, and mvCnt are modified by:


predFlagLX[ CurrMbAddr ][ m ]   = 0   with X = 0..1, m = 0..3 (G-91)
refIdxLX[ CurrMbAddr ][ m ]     = −1  with X = 0..1, m = 0..3 (G-92)
mvLX[ CurrMbAddr ][ m ][ s ][ c ] = 0   with X = 0..1, m = 0..3, s = 0..3, c = 0..1 (G-93)
mvCnt[ CurrMbAddr ]           = 0 (G-94)





- Otherwise (mbType[ CurrMbAddr ] is not equal to I_PCM, I_16x16, I_8x8, I_4x4, or I_BL), the arrays predFlagL0,
predFlagL1, refIdxL0, refIdxL1, mvL0, mvL1, and mvCnt are modified as specified by the following text.


The variable numMbPart is derived as follows:


- If mbType[ CurrMbAddr ] is equal to B_Skip or B_Direct_16x16 and DQId is equal to 0 (nal_unit_type is not equal
to 20), numMbPart is set equal to 4.


- Otherwise, if mbType[ CurrMbAddr ] is equal to B_Skip or B_Direct_16x16 (and DQId is greater than 0 and
nal_unit_type is equal to 20), numMbPart is set equal to 1.


- Otherwise (mbType[ CurrMbAddr ] is not equal to B_Skip or B_Direct_16x16), numMbPart is set equal to
NumMbPart( mbType[ CurrMbAddr ] ).


The macroblock partition index mbPartIdx proceeds over the values 0..(numMbPart − 1), and for each value of mbPartIdx
the following ordered steps are specified:


1. The variable isDirectFlag is derived as follows:


      - If any of the following conditions are true, isDirectFlag is set equal to 1:


         - mbType[ CurrMbAddr ] is equal to B_Skip or B_Direct_16x16,


         - mbType[ CurrMbAddr ] is equal to B_8x8 and subMbType[ CurrMbAddr ][ mbPartIdx ] is equal to
B_Direct_8x8.


      - Otherwise, isDirectFlag is set equal to 0.


2. The variable numSubMbPart is derived as follows:


     - If isDirectFlag is equal to 1 and DQId is equal to 0 (nal_unit_type is not equal to 20), numSubMbPart is set
equal to 4.


     - Otherwise, if isDirectFlag is equal to 1 (and DQId is greater than 0 and nal_unit_type is equal to 20),
numSubMbPart is set equal to 1.


     - Otherwise (isDirectFlag is equal to 0), numSubMbPart is set equal to
NumSubMbPart( subMbType[ CurrMbAddr ][ mbPartIdx ] ).


3. The sub-macroblock partition index subMbPartIdx proceeds over values 0..(numSubMbPart − 1), and for each

value of subMbPartIdx the SVC derivation process for luma motion vector components and reference indices of
a macroblock or sub-macroblock partition as specified in clause G.8.4.1.1 is invoked with mbPartIdx,
subMbPartIdx, isDirectFlag, sliceIdc, fieldMbFlag, mbType, subMbType, predFlagL0, predFlagL1, refIdxL0,
refIdxL1, mvL0, mvL1, mvCnt, refIdxILPredL0, refIdxILPredL1, mvILPredL0, mvILPredL1, and, when DQId
is equal to 0 and (slice_type % 5) is equal to 1, the reference picture list refPicList1 as the inputs and the outputs
are modified versions of the arrays predFlagL0, predFlagL1, refIdxL0, refIdxL1, mvL0, mvL1, and mvCnt.


**G.8.4.1.1** **SVC derivation process for luma motion vector components and reference indices of a macroblock or**

**sub-macroblock partition**


This clause is only invoked when mbType[ CurrMbAddr ], which is specified as input to this clause, is not equal to I_PCM,
I_16x16, I_8x8, I_4x4, or I_BL.


Inputs to this process are:


- a variable mbPartIdx specifying the current macroblock partition,


- a variable subMbPartIdx specifying the current sub-macroblock partition,


- a variable isDirectFlag specifying whether the current macroblock partition is coded in direct mode,


- a one-dimensional array sliceIdc with PicSizeInMbs elements specifying slice identifications for the macroblocks of
the current layer representation,


- a one-dimensional array fieldMbFlag with PicSizeInMbs elements specifying which macroblocks of the current layer
representation are coded as field macroblocks and which macroblocks are coded as frame macroblocks,


- a one-dimensional array mbType with PicSizeInMbs elements specifying macroblock types for the macroblocks of
the current layer representation,


- a (PicSizeInMbs)x4 array subMbType specifying sub-macroblock types for the macroblocks of the current layer
representation,





- two (PicSizeInMbs)x4 arrays predFlagL0 and predFlagL1 specifying prediction utilization flags for the macroblocks
of the current layer representation,


- two (PicSizeInMbs)x4 arrays refIdxL0 and refIdxL1 specifying reference indices for the macroblocks of the current
layer representation,


- two (PicSizeInMbs)x4x4x2 arrays mvL0 and mvL1 specifying motion vector components for the macroblocks of the
current layer representation,


- a one-dimensional array mvCnt with PicSizeInMbs elements specifying the number of motion vectors for the
macroblocks of the current layer representation,


- two 2x2 arrays refIdxILPredL0 and refIdxILPredL1 specifying inter-layer reference index predictors for the current
macroblock,


- two 4x4x2 arrays mvILPredL0 and mvILPredL1 specifying inter-layer motion vector predictors for the current
macroblock,


- when DQId is equal to 0 and (slice_type % 5) is equal to 1, the reference picture list refPicList1.


Outputs of this process are:


- modified versions of the arrays predFlagL0 and predFlagL1,


- modified versions of the arrays refIdxL0 and refIdxL1,


- modified versions of the arrays mvL0 and mvL1,


- a modified version of the array mvCnt.


For all processes specified in clauses 6 or 8 that are invoked from the process specified in this clause or a child process of
the process specified in this clause, the following modifications apply:


a) In clauses 6.4.12.2 and 8.4.1.3.2, a macroblock with address mbAddr is treated as field macroblock when

fieldMbFlag[ mbAddr ] is equal to 1, and it is treated as frame macroblock when fieldMbFlag[ mbAddr ] is equal
to 0. In particular, the current macroblock is treated as field macroblock when fieldMbFlag[ CurrMbAddr ] is
equal to 1, and it is treated as frame macroblock when fieldMbFlag[ CurrMbAddr ] is equal to 0.


b) In clause 6.4.8, a macroblock with address mbAddr is treated to belong to a different slice than the current

macroblock CurrMbAddr, when sliceIdc[ mbAddr ] is not equal to sliceIdc[ CurrMbAddr ].


c) In clause 6.4.12.2, a macroblock mbAddr is treated as top macroblock when (mbAddr % 2) is equal to 0, and it

is treated as bottom macroblock when (mbAddr % 2) is equal to 1.


d) In clauses 6.4.2.1, 6.4.2.2, 6.4.11.7, 8.4.1.1, 8.4.1.3, any occurrence of mb_type is replaced by

mbType[ CurrMbAddr ] with mbType being the array mbType that is input to this clause.


e) In clauses 6.4.2.2 and 6.4.11.7, any occurrence of sub_mb_type is replaced by subMbType[ CurrMbAddr ] with

subMbType being the array subMbType that is input to this clause.


f) In clause 6.4.11.7, mb_type for a macroblock with macroblock address mbAddrN is replaced by
mbType[ mbAddrN ] with mbType being the array mbType that is input to this clause and sub_mb_type for a
macroblock with macroblock address mbAddrN is replaced by subMbType[ mbAddrN ] with subMbType being
the array subMbType that is input to this clause.


g) In clause 6.4.11.7, a macroblock partition or sub-macroblock partition given by mbAddrN, mbPartIdxN, and

subMbPartIdxN is treated as not yet decoded when mbAddrN is equal to CurrMbAddr and
(4 * mbPartIdxN + subMbPartIdxN) is greater than (4 * mbPartIdx + subMbPartIdx).


h) In clause 8.4.1.3.2, a macroblock with mbAddrN is treated as coded in an Intra macroblock prediction mode

when mbType[ mbAddrN ] is equal to I_PCM, I_16x16, I_8x8, I_4x4, or I_BL.


i) In clause 8.4.1.3.2, the variable predFlagLX of a macroblock or sub-macroblock partition given by
mbAddrN\mbPartIdxN\subMbPartIdxN is replaced by predFlagLX[ mbAddrN ][ mbPartIdxN ] with
predFlagLX being the array predFlagLX that is input to this clause.


j) In clause 8.4.1.3.2, the motion vector MvLX[ mbPartIdxN ][ subMbPartIdxN ] and the reference index
RefIdxLX[ mbPartIdxN ] of a macroblock or sub-macroblock partition given by
mbAddrN\mbPartIdxN\subMbPartIdxN are replaced by mvLX[ mbAddrN ][ mbPartIdxN ][ subMbPartIdxN ]
and refIdxLX[ mbAddrN ][ mbPartIdxN ], respectively, with mvLX and refIdxLX being the arrays mvLX and
refIdxLX, respectively, that are input to this clause.





k) In clause 8.4.1.2.1, any occurrence of RefPicList1[ 0 ] is replaced by refPicList1[ 0 ] with refPicList1[ 0 ] being

the first layer field (when field_pic_flag is equal to 1) or the first layer frame or layer complementary field pair
(when field_pic_flag is equal to 0) in the reference picture list refPicList1 that is specified as input to this clause.
The reference picture list refPicList1 is a reference list of layer pictures that correspond to layer representations
with DQId equal to 0 of previously decoded access units.


l) In clause 8.4.1.2.1, the current picture CurrPic represents the current layer picture with DQId equal to 0 and the
variable colPic specifies the layer picture, for the layer representation with DQId equal to 0, that contains the colocated macroblock as specified in Table 8-6.


m) In clause 8.4.1.2.1, all picture order count values are picture order count value for the dependency representation

with dependency_id equal to 0.


n) In clause 8.4.1.2.1, the modification b) specified in clause G.8.2 applies with currDependencyId being equal to

0.


o) In clause 8.4.1.2.1, for deriving the variable fieldDecodingFlagX, the macroblock mbAddrX is treated as field

macroblock when fieldMbColPicFlag[ mbAddrX ] is equal to 1, it is treated as frame macroblock when
fieldMbColPicFlag[ mbAddrX ] is equal to 0. The array fieldMbColPicFlag specifies the array fieldMbFlag that
was derived by the process in clause G.8.1.5.1 for the layer representation with DQId equal to 0.


p) In clause 8.4.1.2.1, the variables PredFlagL0, PredFlagL1, RefIdxL0, RefIdxL1, MvL0, and MvL1 for the

macroblock mbAddrCol inside the picture colPic are replaced with the predFlagL0[ mbAddrCol ],
predFlagL1[ mbAddrCol ], refIdxL0[ mbAddrCol ], refIdxL1[ mbAddrCol ], mvL0[ mbAddrCol ], and
mvL1[ mbAddrCol ], respectively, that have been derived for the layer picture colPic that is associated with DQId
equal to 0.


q) In clause 8.4.1.2.1, the macroblock mbAddrCol is interpreted as coded in an Intra macroblock prediction mode

when mbType[ mbAddrCol ] that has been derived for the layer picture colPic that is associated with DQId equal
to 0 is equal to I_16x16, I_8x8, I_4x4, or I_PCM.


r) In clause 8.4.1.2.1, the syntax element mb_type of the macroblock with address mbAddrCol inside the picture
colPic is replaced with mbType[ mbAddrCol ] that has been derived for the layer picture colPic that is associated
with DQId equal to 0 and the syntax element list sub_mb_type of the macroblock with address mbAddrCol inside
the picture colPic is replaced with the list subMbType[ mbAddrCol ] that has been derived for the layer picture
colPic that is associated with DQId equal to 0.


s) In clause 8.4.1.2.2, the co-located macroblock is treated as field macroblock when
fieldMbColPicFlag[ mbAddrCol ] is equal to 1, it is treated as frame macroblock when
fieldMbColPicFlag[ mbAddrCol ] is equal to 0. The array fieldMbColPicFlag specifies the array fieldMbFlag
that was derived by the process in clause G.8.1.5.1 for the layer representation with DQId equal to 0. The
macroblock address mbAddrCol is the macroblock address of the co-located macroblock as derived in
clause 8.4.1.2.1.


The reference index predictors refIdxPredL0 and refIdxPredL1, the motion vector predictors mvPredL0 and mvPredL1,
and the variable mvCntInc are derived as follows:


- If mbType[ CurrMbAddr ] is equal to P_Skip, the reference index predictor refIdxPredL1 is set equal to −1, both
components of the motion vector predictor mvPredL1 are set equal to 0, the variable mvCntInc is set equal to 1, and
the derivation process for luma motion vectors for skipped macroblocks in P slices as specified in clause 8.4.1.1 is
invoked with the outputs being assigned to the motion vector predictor mvPredL0 and the reference index predictor
refIdxPredL0. For this invocation of the process in clause 8.4.1.1, the modifications specified above in items a)
through j) of this clause apply.


- Otherwise, if isDirectFlag is equal to 1 and DQId is equal to 0 (nal_unit_type is not equal to 20), the derivation process
for spatial direct luma motion vector and reference index prediction mode as specified in clause 8.4.1.2.2 is invoked
with mbPartIdx and subMbPartIdx as the inputs and the output variables refIdxL0, refIdxL1, mvL0, mvL1, and
subMvCnt are assigned to the reference index predictors refIdxPredL0 and refIdxPredL1, the motion vectors
predictors mvPredL0 and mvPredL1, and the variable mvCntInc, respectively. For this invocation of the process in
clause 8.4.1.2.2, the modifications specified above in items a) through s) of this clause apply.
NOTE – When the current clause is invoked, direct_spatial_mv_pred_flag is always equal to 1.


- Otherwise, if isDirectFlag is equal to 1 (and DQId is greater than 0 and nal_unit_type is equal to 20), the SVC
derivation process for luma motion vectors and reference indices for B_Skip, B_Direct_16x16, and B_Direct_8x8 in
NAL units with nal_unit_type equal to 20 as specified in clause G.8.4.1.2 is invoked with mbPartIdx, fieldMbFlag,
mbType, subMbType, predFlagL0, predFlagL1, refIdxL0, refIdxL1, mvL0, and mvL1 as the inputs and the outputs
are refIdxPredL0, refIdxPredL1, mvPredL0, mvPredL1, and mvCntInc.





- Otherwise, the variable mvCntInc is initially set equal to 0, and for X being replaced by 0 and 1, the following applies:


   - If any of the following conditions are true, refIdxPredLX is set equal to −1 and both components of mvPredLX
are set equal to 0:


      - mbType[ CurrMbAddr ] is not equal to P_8x8, P_8x8ref0, or B_8x8 and
MbPartPredMode( mbType[ CurrMbAddr ], mbPartIdx ) is not equal to Pred_LX or BiPred,


      - mbType[ CurrMbAddr ] is equal to P_8x8, P_8x8ref0, or B_8x8 and
SubMbPartPredMode( subMbType[ CurrMbAddr ][ mbPartIdx ] ) is not equal to Pred_LX or BiPred.


   - Otherwise, if base_mode_flag is equal to 1 or motion_prediction_flag_lX[ mbPartIdx ] is equal to 1, the
following ordered steps are specified:


1. The inverse macroblock partition scanning process as specified in clause 6.4.2.1 is invoked with

mbPartIdx as the input and the output is assigned to ( xP, yP ). For this invocation of the process in
clause 6.4.2.1, the modification specified above in item d) of this clause applies.


2. Inverse sub-macroblock partition scanning process as specified in clause 6.4.2.2 is invoked with

mbPartIdx and subMbPartIdx as the inputs and the output is assigned to ( xS, yS ). For this invocation
of the process in clause 6.4.2.2, the modifications specified above in items d) and e) of this clause apply.


3. The reference index predictor refIdxPredLX and the motion vector predictor mvPredLX are derived by


refIdxPredLX  = refIdxILPredLX[ ( xP + xS ) / 8, ( yP + yS ) / 8 ]
mvPredLX[ c ] = mvILPredLX[ ( xP + xS ) / 4, ( yP + yS ) / 4 ][ c ]   with c = 0..1 (G-95)


The bitstream shall not contain data that result in refIdxPredLX less than 0 or refIdxPredLX greater
than num_ref_idx_active_lX_minus1.


The bitstream shall not contain data that result in horizontal motion vector components mvPredLX[ 0 ]
or vertical motion vector components mvPredLX[ 1 ] that exceed the range for motion vector
components specified in clause G.10.2.


4. mvCntInc is set equal to (mvCntInc + 1).


   - Otherwise, the following ordered steps are specified:


1. Depending on mbType[ CurrMbAddr ], the reference index predictor refIdxPredLX is derived as

follows:


          - If mbType[ CurrMbAddr ] is equal to P_8x8ref0, refIdxPredLX is set equal to 0.


          - Otherwise (mbType[ CurrMbAddr ] is not equal to P_8x8ref0), refIdxPredLX is set equal to
ref_idx_lX[ mbPartIdx ].


2. The derivation process for luma motion vector prediction as specified in clause 8.4.1.3 is invoked with

mbPartIdx, subMbPartIdx, refIdxPredLX, and currSubMbType set equal to
subMbType[ CurrMbAddr ][ mbPartIdx ] as the inputs and the output is assigned to mvPredLX. For
this invocation of the process in clause 8.4.1.3, the modifications specified in items a) through j) of this
clause apply


3. mvCntInc is set equal to (mvCntInc + 1).


For X being replaced by 0 and 1, the arrays refIdxLX, predFlagLX, and mvLX are modified by applying the following
ordered steps:


1. When subMbPartIdx is equal to 0, the arrays refIdxLX and predFlagLX are modified by


refIdxLX[ CurrMbAddr ][ mbPartIdx]   = refIdxPredLX (G-96)
predFlagLX[ CurrMbAddr ][ mbPartIdx ] = ( ( refIdxPredLX < 0 ) ? 0 : 1 ) (G-97)


2. The array mvLX is modified by


mvLX[ CurrMbAddr ][ mbPartIdx ][ subMbPartIdx ][ c ] = mvPredLX[ c ]   with c = 0..1 (G-98)


3. When predFlagLX[ CurrMbAddr ][ mbPartIdx ] is equal to 1, base_mode_flag is equal to 0, isDirectFlag is equal

to 0, and mbType[ CurrMbAddr ] is not equal to P_Skip, the array mvLX is modified by





mvLX[ CurrMbAddr ][ mbPartIdx ][ subMbPartIdx ][ c ] +=
mvd_lX[ mbPartIdx ][ subMbPartIdx ][ c ]   with c = 0..1 (G-99)


The array mvCnt is modified as follows:


- If mbPartIdx is equal to 0 and subMbPartIdx is equal to 0, mvCnt[ CurrMbAddr ] is set equal to mvCntInc.


- Otherwise (mbPartIdx is greater than 0 or subMbPartIdx is greater than 0), the array mvCnt is modified by


mvCnt[ CurrMbAddr ] += mvCntInc (G-100)


**G.8.4.1.2** **SVC derivation process for luma motion vectors and reference indices for B_Skip, B_Direct_16x16, and**

**B_Direct_8x8 in NAL units with nal_unit_type equal to 20**


Inputs to this process are:


- a variable mbPartIdx specifying the current macroblock partition,


- a one-dimensional array fieldMbFlag with PicSizeInMbs elements specifying which macroblocks of the current layer
representation are coded as field macroblocks and which macroblocks are coded as frame macroblocks,


- a one-dimensional array mbType with PicSizeInMbs elements specifying macroblock types for the macroblocks of
the current layer representation,


- a (PicSizeInMbs)x4 array subMbType specifying sub-macroblock types for the macroblocks of the current layer
representation,


- two (PicSizeInMbs)x4 arrays predFlagL0 and predFlagL1 specifying prediction utilization flags for the macroblocks
of the current layer representation,


- two (PicSizeInMbs)x4 arrays refIdxL0 and refIdxL1 specifying reference indices for the macroblocks of the current
layer representation,


- two (PicSizeInMbs)x4x4x2 arrays mvL0 and mvL1 specifying motion vector components for the macroblocks of the
current layer representation.


Outputs of this process are:


- the reference index predictors refIdxPredL0 and refIdxPredL1,


- the motion vector predictors mvPredL0 and mvPredL1,


- the variable mvCntInc.


The variable currSubMbType is derived as follows:


- If mbType[ CurrMbAddr ] is equal to B_Skip or B_Direct_16x16, currSubMbType is marked as "unspecified".


- Otherwise (mbType[ CurrMbAddr ] is equal to B_8x8 and subMbType[ CurrMbAddr ][ mbPartIdx ] is equal to
B_Direct_8x8), currSubMbType is set equal to B_Bi_8x8.
NOTE – The variable currSubMbType is only used for deriving the variable predPartWidth in clause 6.4.11.7, which specifies the
partition width of the current macroblock or sub-macroblock partition for determining neighbouring partitions that are used for
motion vector prediction. Inside clause 6.4.11.7, the variable predPartWidth is set equal to 16 when the current macroblock is
coded with macroblock type equal to B_Skip or B_Direct_16x16 or the current sub-macroblock is coded with sub macroblock
type equal B_Direct_8x8. When the current clause is invoked for a sub-macroblock coded with sub-macroblock type equal to
B_Direct_8x8 (the current clause is only invoked for NAL units with nal_unit_type equal to 20), currSubMbType is set equal to
B_Bi_8x8 in order to set the variable predPartWidth equal to 8 in clause 6.4.11.7.


For X being replaced by 0 and 1, the reference index predictor refIdxPredLX is derived by applying the following ordered
steps:


1. The derivation process for motion data of neighbouring partitions as specified in clause 8.4.1.3.2 is invoked with

mbPartIdx, subMbPartIdx set equal to 0, currSubMbType, and listSuffixFlag set equal to X as the inputs and the
outputs are the reference indices refIdxLXN with N being replaced by A, B, and C. For this invocation of the
process in clause 8.4.1.3.2, the modifications specified in items a) through j) of clause G.8.4.1.1 apply.


2. The reference index predictor refIdxPredLX is derived by


refIdxPredLX = MinPositive( refIdxLXA, MinPositive( refIdxLXB, refIdxLXC ) ) (G-101)


with





 Min( x, y ) if x = 0 and y = 0

MinPositive( x, y ) = 

 Max( x, y ) otherwise



(G-102)



When both reference index predictors refIdxPredL0 and refIdxPredL1 are less than 0, refIdxPredL0 and refIdxPredL1 are
set equal to 0.


For X being replaced by 0 and 1, the motion vector predictor mvPredLX is derived as follows:


- If refIdxPredLX is greater than or equal to 0, the derivation process for luma motion vector prediction as specified in
clause 8.4.1.3 is invoked with mbPartIdx, subMbPartIdx set equal to 0, refIdxPredLX, and currSubMbType as the
inputs and the output is assigned to mvPredLX. For this invocation of the process in clause 8.4.1.3, the modifications
specified in items a) through j) of clause G.8.4.1.1 apply.


- Otherwise, both components of the motion vector mvPredLX are set equal to 0.


The variable mvCntInc is derived as specified by the following ordered steps:


1. mvCntInc is set equal to 0


2. When refIdxPredL0 is greater than or equal to 0, mvCntInc is set equal to (mvCntInc + 1).


3. When refIdxPredL1 is greater than or equal to 0, mvCntInc is set equal to (mvCntInc + 1).


**G.8.4.2** **SVC decoding process for Inter prediction samples**


Inputs to this process are:


- a variable targetQId specifying the quality_id value for the target layer representation,


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a variable sliceIdc specifying the slice identification for the current macroblock,


- a variable mbType specifying the macroblock type for the current macroblock,


- a list subMbType with 4 elements specifying the sub-macroblock types for the current macroblock,


- two lists predFlagL0 and predFlagL1 with 4 elements specifying prediction utilization flags for the current
macroblock,


- two lists refIdxL0 and refIdxL1 with 4 elements specifying reference indices for the current macroblock,


- two 4x4x2 arrays mvL0 and mvL1 specifying motion vectors components for the current macroblock,


- when present, a one-dimensional array refLayerFieldMbFlag with RefLayerPicSizeInMbs elements specifying which
macroblocks of the reference layer representation are field macroblocks and which macroblocks are frame
macroblocks,


- when present, a one-dimensional array refLayerMbType with RefLayerPicSizeInMbs elements specifying
macroblock types for the macroblocks of the reference layer representation,


- the reference picture lists refPicList0 and refPicList1 (when available),


- a (PicWidthInSamplesL)x(PicHeightInSamplesL) array picSamplesL of luma sample values,


- a (PicWidthInSamplesL)x(PicHeightInSamplesL) array picResL of residual luma sample values,


- when ChromaArrayType is not equal to 0, two (PicWidthInSamplesC)x(PicHeightInSamplesC) arrays picSamplesCb
and picSamplesCr of chroma sample values,


- when ChromaArrayType is not equal to 0, two (PicWidthInSamplesC)x(PicHeightInSamplesC) arrays picResCb and
picResCr of residual chroma sample values.


Outputs of this process are:


- a modified version of the array of luma sample values picSamplesL,


- a modified version of the array of residual luma sample values picResL,


- when ChromaArrayType is not equal to 0, modified versions of the two arrays of chroma sample values picSamplesCb
and picSamplesCr,


- when ChromaArrayType is not equal to 0, modified versions of the two arrays of residual chroma sample values
picResCb and picResCr.





Unless stated otherwise, the syntax elements and derived upper-case variables that are referred to by the process specified
in this clause and all child processes invoked from this process are the syntax elements and derived upper-case variables
for the current macroblock, which is the macroblock with address CurrMbAddr inside the layer representation with DQId
equal to (sliceIdc & 127), the slice header of the current slice, which is the slice that contains the current macroblock, the
current picture parameter, which is identified by the syntax element pic_parameter_set_id inside the slice header of the
current slice, and the current sequence parameter, which is identified by the syntax element seq_parameter_set_id inside
the current picture parameter set.


For all processes specified in clauses 6 or 8 that are invoked from the process specified in this clause or a child process of
the process specified in this clause, the following modifications apply:


a) In clauses 8.4.3, 8.4.1.4, and 8.4.2.1, the current macroblock is treated as field macroblock when fieldMbFlag is

equal to 1, and it is treated as frame macroblock when fieldMbFlag is equal to 0. When field_pic_flag is equal to
0 and the current macroblock CurrMbAddr is a field macroblock, its parity is equal to top when
(CurrMbAddr % 2) is equal to 0 and its parity is equal to bottom when (CurrMbAddr % 2) is equal to 1.


b) In clauses 8.4.3 and 8.4.2.1, any occurrence of RefPicList0 or RefPicList1 is replaced with refPicList0 or

refPicList1, respectively, with refPicList0 and refPicList1 being the reference picture lists specified as inputs to
this clause.


c) In clause 8.4.1.4, the reference picture referred by refIdxLX is specified by refPicListX[ refIdxLX ] with

refPicList0 and refPicList1 specified as inputs to this clause.


d) In clauses 8.4.2.2.1 and 8.4.2.2.2, any occurrence of mb_field_decoding_flag is replaced by fieldMbFlag.


e) Decoded pictures are represented by the sample arrays SL and, when ChromaArrayType is not equal to 0, SCb and

SCr, reference base pictures are represented by the sample arrays BL and, when ChromaArrayType is not equal
to 0, BCb and BCr. When reference base pictures are referenced in the inter prediction process via clause 8.4.2.1,
the samples arrays BL, BCb, and BCr are referred to as SL, SCb, and SCr, respectively. The sample arrays SL, SCb,
SCr, BL, BCb, and BCr that referenced in the inter prediction process via clause 8.4.2.1 are constructed as specified
in clause G.8.


Let predMbL be a 16x16 array of luma prediction samples for the macroblock mbAddr.


When ChromaArrayType is not equal to 0, let predMbCb and predMbCr be two (MbWidthC)x(MbHeightC) arrays of
chroma prediction samples for the macroblock mbAddr.


The variable numMbPart is derived as follows:


- If mbType is equal to B_Skip or B_Direct_16x16 and DQId is equal to 0 (nal_unit_type is not equal to 20),
numMbPart is set equal to 4.


- Otherwise, if mbType is equal to B_Skip or B_Direct_16x16 (and DQId is greater than 0 and nal_unit_type is equal
to 20), numMbPart is set equal to 1.


- Otherwise (mbType is not equal to B_Skip or B_Direct_16x16), numMbPart is set equal to NumMbPart( mbType ).


The macroblock partition index mbPartIdx proceeds over the values 0..(numMbPart − 1), and for each value of mbPartIdx
the following ordered steps are specified:


1. The variable isDirectFlag is derived as follows:


      - If any of the following conditions are true, isDirectFlag is set equal to 1:


        - mbType is equal to B_Skip or B_Direct_16x16,


        - mbType is equal to B_8x8 and subMbType[ mbPartIdx ] is equal to B_Direct_8x8.


      - Otherwise, isDirectFlag is set equal to 0.


2. The variables implicitModeFlag and explicitModeFlag are derived as follows:


     - If weighted_bipred_idc is equal to 2, (slice_type % 5) is equal to 1, predFlagL0[ mbPartIdx ] is equal to 1,
and predFlagL1[ mbPartIdx ] is equal to 1, implicitModeFlag is set equal to 1 and explicitModeFlag is set
equal to 0.


     - Otherwise, if weighted_bipred_idc is equal to 1, (slice_type % 5) is equal to 1, and
predFlagL0[ mbPartIdx ] + predFlagL1[ mbPartIdx ] is equal to 1 or 2, implicitModeFlag is set equal to 0
and explicitModeFlag is set equal to 1.


     - Otherwise, if weighted_pred_flag is equal to 1, (slice_type % 5) is equal to 0, and predFlagL0[ mbPartIdx ]
is equal to 1, implicitModeFlag is set equal to 0 and explicitModeFlag is set equal to 1.





     - Otherwise, implicitModeFlag is set equal to 0 and explicitModeFlag is set equal to 0.


3. When implicitModeFlag is equal to 1 or explicitModeFlag is equal to 1, the SVC derivation process for prediction

weights as specified in clause G.8.4.2.1 is invoked with fieldMbFlag, refIdxL0[ mbPartIdx ],
refIdxL1[ mbPartIdx ], predFlagL0[ mbPartIdx ], predFlagL1[ mbPartIdx ], refPicList0, and refPicList1 (when
available) as inputs and the outputs are assigned to logWDL, w0L, w1L, o0L, o1L, and when ChromaArrayType is
not equal to 0, logWDC, w0C, w1C, o0C, o1C with C being replaced by Cb and Cr.


4. The luma location ( xP, yP ) is derived as follows:


     - If mbType is equal to B_Skip or B_Direct_16x16, xP is set equal to (8 * ( mbPartIdx % 2 )) and yP is set
equal to (8 * ( mbPartIdx / 2 )).


     - Otherwise (mbType is not equal to B_Skip or B_Direct_16x16), the inverse macroblock partition scanning
process as specified in clause 6.4.2.1 is invoked with mbPartIdx as the input and the output is assigned to
( xP, yP ). For this invocation of the process in clause 6.4.2.1, any occurrence of mb_type is replaced by
mbType.


5. The variable numSubMbPart is derived as follows:


     - If isDirectFlag is equal to 1 and DQId is equal to 0 (nal_unit_type is not equal to 20), numSubMbPart is set
equal to 4.


     - Otherwise, if isDirectFlag is equal to 1 (and DQId is greater than 0 and nal_unit_type is equal to 20),
numSubMbPart is set equal to 1.


     - Otherwise (isDirectFlag is equal to 0), numSubMbPart is set equal to
NumSubMbPart( subMbType[ mbPartIdx ] )


6. The sub-macroblock partition index proceeds over values 0..(numSubMbPart − 1), and for each value of

subMbPartIdx the following ordered steps are specified:


a. The variables partWidth and partHeight are derived as follows:


        - If isDirectFlag is equal to 1 and DQId is equal to 0 (nal_unit_type is not equal to 20), partWidth and
partHeight are set equal to 4.


        - Otherwise, if isDirectFlag is equal to 1 (and DQId is greater than 0 and nal_unit_type is equal to 20),
the following applies:


          - If mbType is equal to B_Skip or B_Direct_16x16, partWidth and partHeight are set equal to 16.


          - Otherwise (mbType is equal to B_8x8 and subMbType[ mbPartIdx ] is equal to B_Direct_8x8),
partWidth and partHeight are set equal to 8.


        - Otherwise (isDirectFlag is equal to 0), the following applies:


          - If mbType is not equal to P_8x8, P_8x8ref0, or B_8x8, partWidth and partHeight are derived by


partWidth = MbPartWidth( mbType ) (G-103)
partHeight = MbPartHeight( mbType ) (G-104)


          - Otherwise (mbType is equal to P_8x8, P_8x8ref0, or B_8x8), partWidth and partHeight are
derived by


partWidth = SubMbPartWidth( subMbType[ mbPartIdx ] ) (G-105)
partHeight = SubMbPartHeight( subMbType[ mbPartIdx ] ) (G-106)


b. When ChromaArrayType is not equal to 0, the variables partWidthC and partHeightC are derived by


partWidthC = partWidth / SubWidthC (G-107)
partHeightC = partHeight / SubWidthC (G-108)


c. For X being replaced by 0 and 1, when ChromaArrayType is not equal to 0 and predFlagLX[ mbPartIdx ] is
equal to 1, the derivation process for chroma motion vectors as specified in clause 8.4.1.4 is invoked with
mvLX[ mbPartIdx ][ subMbPartIdx ] and refIdxLX[ mbPartIdx ] as the inputs and the output is the chroma
motion vector mvCLX. For this invocation of the process in clause 8.4.1.4, the modifications specified above
in items a) and c) of this clause apply.


d. The decoding process for Inter prediction samples as specified in clause 8.4.2 is invoked with mbPartIdx,

subMbPartIdx, partWidth and partHeight, partWidthC and partHeightC (if available), luma motion vectors
mvL0[ mbPartIdx ][ subMbPartIdx ] and mvL1[ mbPartIdx ][ subMbPartIdx ], chroma motion vectors
mvCL0 and mvCL1 (if available), reference indices refIdxL0[ mbPartIdx ] and refIdxL1[ mbPartIdx ],





prediction utilization flags predFlagL0[ mbPartIdx ] and predFlagL1[ mbPartIdx ] as well as variables for
weighted prediction logWDL, w0L, w1L, o1L, o0L, and when ChromaArrayType is not equal to 0, logWDC,
w0C, w1C, o1C, and o0C (with C being replaced by Cb and Cr) as the inputs and the outputs are a
(partWidth)x(partHeight) array predPartL of luma prediction samples and, when ChromaArrayType is not
equal to 0, two (partWidthC)x(partHeightC) arrays predPartCb and predPartCr of chroma prediction samples.
For this invocation of the process in clause 8.4.2, the modifications specified above in items a), b), d), and
e) of this clause apply.


e. The luma location ( xS, yS ) is derived as follows:


        - If mbType is equal to B_8x8 and subMbType[ mbPartIdx ] is equal to B_Direct_8x8, xS is set equal to
(4 * ( subMbPartIdx % 2 )) and yS is set equal to (4 * ( subMbPartIdx / 2 )).


        - Otherwise (mbType is not equal to B_8x8 or subMbType[ mbPartIdx ] is not equal to B_Direct_8x8),
the inverse sub-macroblock partition scanning process as specified in clause 6.4.2.2 is invoked with
mbPartIdx and subMbPartIdx as the inputs and the output is assigned to ( xS, yS ). For this invocation
of the process in clause 6.4.2.2, any occurrence of mb_type is replaced by mbType and any occurrence
of sub_mb_type is replaced by subMbType.


f. For x = 0..(partWidth − 1) and y = 0..(partHeight − 1), the 16x16 array predMbL is modified by


predMbL[ xP + xS + x, yP + yS + y ] = predPartL[ x, y ] (G-109)


g. When ChromaArrayType is not equal to 0, for x = 0..(partWidthC − 1) and y = 0..(partHeightC − 1), the

(MbWidthC)x(MbHeightC) arrays predMbCb and predMbCr are modified by


predMbCb[ ( xP + xS ) / SubWidthC + x, ( yP + yS ) / SubHeightC + y ] = predPartCb[ x, y ] (G-110)
predMbCr[ ( xP + xS ) / SubWidthC + x, ( yP + yS ) / SubHeightC + y ] = predPartCr[ x, y ] (G-111)


When targetQId is equal to 0, base_mode_flag is equal to 1, MbaffFrameFlag is equal to 0, RefLayerMbaffFrameFlag is
equal to 0, and RestrictedSpatialResolutionChangeFlag is equal to 0, the intra-inter prediction combination process
specified in clause G.8.4.2.2 is invoked with fieldMbFlag, refLayerFieldMbFlag, refLayerMbType, predMbL,
picSamplesL, picResL, and, when ChromaArrayType is not equal to 0, predMbCb, predMbCr, picSamplesCb, picSamplesCr,
picResCb, and picResCr as the inputs, and the outputs are modified versions of predMbL and picResL, and, when
ChromaArrayType is not equal to 0, modified versions of predMbCb, predMbCr, picResCb, and picResCr.


The picture sample array construction process as specified in clause G.8.5.4.1 is invoked with fieldMbFlag, predMbL,
picSamplesL, and, when ChromaArrayType is not equal to 0, predMbCb, predMbCr, picSamplesCb, and picSamplesCr as the
inputs and the outputs are a modified version of picSamplesL and, when ChromaArrayType is not equal to 0, modified
versions of picSamplesCb, and picSamplesCr.


**G.8.4.2.1** **SVC derivation process for prediction weights**


Inputs to this process are:


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- the reference indices refIdxL0 and refIdxL1 for the current macroblock partition,


- the prediction list utilization flags predFlagL0 and predFlagL1 for the current macroblock partition,


- the reference picture lists refPicList0 and refPicList1 (when available).


Outputs of this process are:


- variables for weighted prediction of luma samples logWDL, w0L, w1L, o0L, o1L,


- when ChromaArrayType is not equal to 0 (monochrome), variables for weighted prediction of chroma samples
logWDC, w0C, w1C, o0C, o1C with C being replaced by Cb and Cr.

Depending on base_pred_weight_table_flag, the following applies:


- If base_pred_weight_table_flag is equal to 0, the derivation process for prediction weights as specified in clause 8.4.3
is invoked with refIdxL0, refIdxL1, predFlagL0, and predFlagL1 as inputs and the outputs are assigned to logWDL,
w0L, w1L, o0L, o1L, and when ChromaArrayType is not equal to 0, logWDC, w0C, w1C, o0C, o1C with C being replaced
by Cb and Cr. For this invocation of the process in clause 8.4.3, the modifications specified in items a) and b) of
clause G.8.4.2 apply.


- Otherwise (base_pred_weight_table_flag is equal to 1), for X being replaced by 0 and 1, the following ordered steps
are specified:


1. Let dqIdList be the list of DQId values that is derived by invoking the derivation process for the set of layer

representations required for decoding as specified in clause G.8.1.1. Let baseDQId be the largest value dqId,





inside the list dqIdList, that has the following properties: a) dqId is less than the current value of DQId; b)
the slices of the layer representation with DQId equal to dqId have base_pred_weight_table_flag equal to 0.


2. Let baseSlice be any slice of the layer representation with DQId equal to baseDQId.


3. Let refLayerLumaLogWD, aRefLayerLumaWeightLX[], and aRefLayerLumaOffsetLX[] be variables that

are set equal to the values of the syntax elements luma_log2_weight_denom, luma_weight_lX[], and
luma_offset_lX[], respectively, of baseSlice.


4. When ChromaArrayType is not equal to 0, let refLayerChromaLogWD, aRefLayerChromaWeightLX[][],

and aRefLayerChromaOffsetLX[][] be variables that are set equal to the values of the syntax elements
chroma_log2_weight_denom, chroma_weight_lX[], and chroma_offset_lX[], respectively, of baseSlice.


5. The variable refIdxLXWP is derived as follows:


        - If MbaffFrame is equal to 1 and fieldMbFlag is equal to 1,


refIdxLXWP = refIdxLX >> 1 (G-112)


        - Otherwise (MbaffFrameFlag is equal to 0 or fieldMbFlag is equal to 0),


refIdxLXWP = refIdxLX (G-113)


6. The variables logWDL, wXL, oXL are derived by:


logWDL = refLayerLumaLogWD (G-114)
wXL    = aRefLayerLumaWeightLX[ refIdxLXWP ] (G-115)
oXL     = aRefLayerLumaOffsetLX[ refIdxLXWP ] * ( 1 << ( BitDepthY − 8 ) ) (G-116)


7. When ChromaArrayType is not equal to 0, the variables logWDC, wXC, oXC (with C being replaced by Cb

and Cr and iCbCr = 0 for Cb and iCbCr = 1 for Cr) are derived by:


logWDC = refLayerChromaLogWD (G-117)
wXC    = aRefLayerChromaWeightLX[ refIdxLXWP ][ iCbCr ] (G-118)
oXC     = aRefLayerChromaOffsetLX[ refIdxLXWP ][ iCbCr ] * ( 1 << ( BitDepthC − 8 ) ) (G-119)


8. When predFlagL0 and predFlagL1 are equal to 1, the following constraint shall be obeyed for C equal to L

and, when ChromaArrayType is not equal to 0, Cb and Cr


−128 <= w0C + w1C <= ( ( logWDC = = 7 ) ? 127 : 128 ) (G-120)


**G.8.4.2.2** **Intra-inter prediction combination process**


Inputs to this process are:


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a one-dimensional array refLayerFieldMbFlag with RefLayerPicSizeInMbs elements specifying which macroblocks
of the reference layer representation are field macroblocks and which macroblocks are frame macroblocks,


- a one-dimensional array refLayerMbType with RefLayerPicSizeInMbs elements specifying macroblock types for the
macroblocks of the reference layer representation,


- a 16x16 array predMbL of luma inter prediction samples for the current macroblock,


- a (PicWidthInSamplesL)x(PicHeightInSamplesL) array picSamplesL of luma sample values,


- a (PicWidthInSamplesL)x(PicHeightInSamplesL) array picResL of residual luma sample values,


- when ChromaArrayType is not equal to 0, two (MbWidthC)x(MbHeightC) arrays predMbCb and predMbCr of chroma
prediction samples for the macroblock mbAddr,


- when ChromaArrayType is not equal to 0, two (PicWidthInSamplesC)x(PicHeightInSamplesC) arrays picSamplesCb
and picSamplesCr of chroma sample values,


- when ChromaArrayType is not equal to 0, two (PicWidthInSamplesC)x(PicHeightInSamplesC) arrays picResCb and
picResCr of residual chroma sample values.





Outputs of this process are:


- a modified version of the array predMbL of luma prediction samples for the macroblock mbAddr,


- a modified version of the array picResL of residual luma sample values,


- when ChromaArrayType is not equal to 0, modified versions of the two arrays predMbCb and predMbCr of chroma
prediction samples for the macroblock mbAddr,


- when ChromaArrayType is not equal to 0, modified versions of the two arrays picResCb and picResCr of residual
chroma sample values.


Let predMbTempL be a 16x16 array and, when ChromaArrayType is not equal to 0, let predMbTempCb and predMbTempCr
be two (MbWidthC)x(MbHeightC) arrays. The macroblock sample array extraction process as specified in clause G.8.5.4.2
is invoked with fieldMbFlag, picSamplesL, and when ChromaArrayType is not equal to 0, picSamplesCb and picSamplesCr
as the inputs and the outputs are assigned to predMbTempL, and when ChromaArrayType is not equal to 0, predMbTempCb
and predMbTempCr.


Let resMbL be a 16x16 array and, when ChromaArrayType is not equal to 0, let resMbCb and resMbCr be two
(MbWidthC)x(MbHeightC) arrays. The macroblock sample array extraction process as specified in clause G.8.5.4.2 is
invoked with fieldMbFlag, picResL, and, when ChromaArrayType is not equal to 0, picResCb and picResCr as the inputs
and the outputs are assigned to resMbL and, when ChromaArrayType is not equal to 0, resMbCb and resMbCr.


For x proceeding over the values 0..15 and y proceeding over the values 0..15, the following ordered steps are specified:


1. The derivation process for reference layer macroblocks as specified in clause G.6.1 is invoked with the luma

location ( x, y ), fieldMbFlag, refLayerFieldMbFlag, and refLayerMbType as the inputs and the outputs are
assigned to mbAddrRefLayer and ( xRef, yRef ).


2. When refLayerMbType[ mbAddrRefLayer ] is equal to I_PCM, I_16x16, I_8x8, I_4x4, or I_BL, the following

applies:


a. The prediction luma sample predMbL[ x, y ] is modified by


predMbL[ x, y ] = predMbTempL[ x, y ] (G-121)


b. When ChromaArrayType is not equal to 0, (x % SubWidthC) is equal to 0, and (y % SubHeightC) is equal

to 0, the prediction chroma samples predMbCb[ x / SubWidthC, y / SubHeightC ] and
predMbCr[ x / SubWidthC, y / SubHeightC ] are modified by


predMbCb[ x / SubWidthC, y / SubHeightC ] = predMbTempCb[ x / SubWidthC, y / SubHeightC ]
(G-122)


predMbCr[ x / SubWidthC, y / SubHeightC ] = predMbTempCr[ x / SubWidthC, y / SubHeightC ]
(G-123)


c. The residual luma sample resMbL[ x, y ] is set equal to 0.


d. When ChromaArrayType is not equal to 0, (x % SubWidthC) is equal to 0, and (y % SubHeightC) is equal

to 0, the residual chroma samples resMbCb[ x / SubWidthC, y / SubHeightC ] and
resMbCr[ x / SubWidthC, y / SubHeightC ] are set equal to 0.


The picture sample array construction process as specified in clause G.8.5.4.1 is invoked with fieldMbFlag, resMbL,
picResL, and, when ChromaArrayType is not equal to 0, resMbCb, resMbCr, picResCb, and picResCr as the inputs and the
outputs are a modified version of the array picResL and, when ChromaArrayType is not equal to 0, modified versions of
the arrays picResCb and picResCr.


**G.8.5** **SVC transform coefficient decoding and sample array construction processes**


Clause G.8.5.1 specifies the transform coefficient scaling and refinement process.


Clause G.8.5.2 specifies the transform coefficient level scaling process prior to transform coefficient refinement.


Clause G.8.5.3 specifies the residual construction and accumulation process.


Clause G.8.5.4 specifies the sample array accumulation process.


Clause G.8.5.5 specifies the sample array re-initialization process.





**G.8.5.1** **Transform coefficient scaling and refinement process**


Inputs to this process are:


- a variable refinementFlag specifying whether the transform coefficients for the current macroblock are combined with
the existent transform coefficients for the current macroblock, which were obtained from the reference layer
representation,


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a variable mbType specifying the macroblock type for the current macroblock,


- a variable cTrafo specifying the transform type for the current macroblock,


- a list of scaled transform coefficient values sTCoeff with (256 + 2 * MbWidthC * MbHeightC) elements,


- a list of transform coefficient level values tCoeffLevel with (256 + 2 * MbWidthC * MbHeightC) elements,


- the luma quantization parameter tQPY,


- when ChromaArrayType is not equal to 0, the chroma quantization parameters tQPCb and tQPCr.


Outputs of this process are:


- a modified version of the list sTCoeff,


- a modified version of the list tCoeffLevel.


The scaling functions are derived as specified in clause 8.5.9. For this invocation of clause 8.5.9, the current macroblock
is considered as coded using an Intra macroblock prediction mode when mbType is equal to I_PCM, I_16x16, I_8x8,
I_4x4, or I_BL; otherwise it is considered as coded using an Inter macroblock prediction mode.


The variable qP′Y is set equal to (tQPY + QpBdOffsetY). When ChromaArrayType is not equal to 0, the variables qP′Cb and
qP′Cr are set equal to (tQPCb + QpBdOffsetC) and (tQPCr + QpBdOffsetC), respectively.


When refinementFlag is equal to 0, all (256 + 2 * MbWidthC * MbHeightC) elements of the lists sTCoeff and tCoeffLevel
are set equal to 0.


The refinement process for luma transform coefficients as specified in clause G.8.5.1.1 is invoked with iYCbCr set equal
to 0, fieldMbFlag, cTrafo, sTCoeff, tCoeffLevel, and qP′Y as the inputs and the outputs are modified versions of the lists
sTCoeff and tCoeffLevel.


When ChromaArrayType is not equal to 0, the refinement process for chroma transform coefficients as specified in
clause G.8.5.1.2 is invoked with fieldMbFlag, cTrafo, sTCoeff, tCoeffLevel, qP′Cb, and qP′Cr as the inputs and the outputs
are modified versions of the lists sTCoeff and tCoeffLevel.


**G.8.5.1.1** **Refinement process for luma transform coefficients or chroma transform coefficients with**

**ChromaArrayType equal to 3**


Inputs to this process are:


- a variable iYCbCr specifying the colour component,


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a variable cTrafo specifying the transform type,


- a list of scaled transform coefficient values sTCoeff with (256 + 2 * MbWidthC * MbHeightC) elements,


- a list of transform coefficient level values tCoeffLevel with (256 + 2 * MbWidthC * MbHeightC) elements,


- when iYCbCr is equal to 0, the luma quantization parameter qP′Y,


- when iYCbCr is greater than 0, the chroma quantization parameters qP′Cb and qP′Cr.


Outputs of this process are:


- a modified version of the list sTCoeff,


- a modified version of the list tCoeffLevel.


When iYCbCr is not present as input to this clause, it is inferred to be equal to 0.


Depending on iYCbCr, the variables bitDepth, qP, cO, coeffLevel4x4, coeffLevel8x8, coeffDCLevel, and coeffACLevel
are derived as follows:





- If iYCbCr is equal to 0, bitDepth is set equal to BitDepthY, qP is set equal to qP′Y, cO is set equal to 0, coeffLevel4x4
is set equal to LumaLevel4x4, coeffLevel8x8 is set equal to LumaLevel8x8, coeffDCLevel is set equal to
Intra16x16DCLevel, and coeffACLevel is set equal to Intra16x16ACLevel.


- Otherwise, if iYCbCr is equal to 1, bitDepth is set equal to BitDepthC, qP is set equal to qP′Cb, cO is set equal to 256,
coeffLevel4x4 is set equal to CbLevel4x4, coeffLevel8x8 is set equal to CbLevel8x8, coeffDCLevel is set equal to
CbIntra16x16DCLevel, and coeffACLevel is set equal to CbIntra16x16ACLevel.


- Otherwise (iYCbCr is equal to 2), bitDepth is set equal to BitDepthC, qP is set equal to qP′Cr, cO is set equal to
(256 + MbWidthC * MbHeightC), coeffLevel4x4 is set equal to CrLevel4x4, coeffLevel8x8 is set equal to
CrLevel8x8, coeffDCLevel is set equal to CrIntra16x16DCLevel, and coeffACLevel is set equal to
CrIntra16x16ACLevel.


Depending on cTrafo, the following applies:


- If cTrafo is equal to T_PCM, the assignment process for luma transform coefficient values or chroma transform
coefficient values with ChromaArrayType equal to 3 for I_PCM macroblocks as specified in clause G.8.5.1.1.1 is
invoked with iYCbCr, sTCoeff, and tCoeffLevel as the inputs and the outputs are modified versions of sTCoeff and
tCoeffLevel.


- Otherwise, if cTrafo is equal to T_4x4, the refinement process for transform coefficients of residual 4x4 blocks as
specified in clause G.8.5.1.1.2 is invoked with fieldMbFlag, bitDepth, qP, cO, coeffLevel4x4, sTCoeff, and
tCoeffLevel as the inputs and the outputs are modified versions of the lists sTCoeff and tCoeffLevel.


- Otherwise, if cTrafo is equal to T_8x8, the refinement process for transform coefficients of residual 8x8 blocks as
specified in clause G.8.5.1.1.3 is invoked with fieldMbFlag, bitDepth, qP, cO, coeffLevel8x8, sTCoeff, and
tCoeffLevel as the inputs and the outputs are modified versions of the lists sTCoeff and tCoeffLevel.


- Otherwise (cTrafo is equal to T_16x16), the refinement process for transform coefficients of Intra_16x16 macroblocks
as specified in clause G.8.5.1.1.4 is invoked with fieldMbFlag, bitDepth, qP, cO, coeffDCLevel, coeffACLevel,
coeffLevel4x4, sTCoeff, and tCoeffLevel as the inputs and the outputs are modified versions of the lists sTCoeff and
tCoeffLevel.


**G.8.5.1.1.1** **Assignment process for luma transform coefficient values or chroma transform coefficient values with**
**ChromaArrayType equal to 3 for I_PCM macroblocks**


Inputs to this process are:


- a variable iYCbCr specifying the colour component,


- a list of scaled transform coefficient values sTCoeff with (256 + 2 * MbWidthC * MbHeightC) elements,


- a list of transform coefficient level values tCoeffLevel with (256 + 2 * MbWidthC * MbHeightC) elements.


Outputs of this process are:


- a modified version of the list sTCoeff,


- a modified version of the list tCoeffLevel.


When base_mode_flag is equal to 0, the following ordered steps are specified:


1. Depending on iYCbCr, the variables cO, cListOffset and pcmSample are derived by


cO        =  iYCbCr * 256
cListOffset = ( ( iYCbCr = = 0 ) ? 0 : ( iYCbCr − 1 ) * 256 ) (G-124)
pcmSample = ( ( iYCbCr = = 0 ) ? pcm_sample_luma : pcm_sample_chroma ) (G-125)


2. The lists tCoeffLevel and sTCoeff are modified by


tCoeffLevel[ cO + k ] = 0                       with k = 0..255 (G-126)


sTCoeff[ cO + k ] = pcmSample[ cListOffset + k ]   with k = 0..255 (G-127)


**G.8.5.1.1.2** **Refinement process for transform coefficients of residual 4x4 blocks**


Inputs to this process are:


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a variable bitDepth specifying the bit depth,


- a variable qP specifying the quantization parameter value,





- a variable cO specifying the first coefficient index in the list of scaled transform coefficient values sTCoeff and in the
list of transform coefficient values tCoeffLevel,


- a variable coeffLevel4x4 representing LumaLevel4x4, CbLevel4x4, or CrLevel4x4,


- a list of scaled transform coefficient values sTCoeff with (256 + 2 * MbWidthC * MbHeightC) elements,


- a list of transform coefficient level values tCoeffLevel with (256 + 2 * MbWidthC * MbHeightC) elements.


Outputs of this process are:


- a modified version of the list sTCoeff,


- a modified version of the list tCoeffLevel.


Depending on tcoeff_level_prediction_flag, the following applies:


- If tcoeff_level_prediction_flag is equal to 1, the list sTCoeff is modified by


sTCoeff[ cO + k ] = 0      with k = 0..255 (G-128)


- Otherwise (tcoeff_level_prediction_flag is equal to 0), the list tCoeffLevel is modified by


tCoeffLevel[ cO + k ] = 0   with k = 0..255 (G-129)


For each residual 4x4 block indexed by c4x4BlkIdx = 0..15, the following ordered steps are specified:


1. The inverse scanning process for 4x4 transform coefficients and scaling lists as specified in clause 8.5.6 is

invoked with coeffLevel4x4[ c4x4BlkIdx ] as the input and the outputs are transform coefficient level values as
a 4x4 array c with elements cij. For this invocation of the process in clause 8.5.6, the current macroblock is treated
as field macroblock when fieldMbFlag is equal to 1, and it is treated as frame macroblock when fieldMbFlag is
equal to 0.


2. The list tCoeffLevel and the 4x4 array c are modified by


tCoeffLevel[ cO + 16 * c4x4BlkIdx + 4 * i + j ] += cij   with i, j = 0..3 (G-130)


cij = tCoeffLevel[ cO + 16 * c4x4BlkIdx + 4 * i + j ]    with i, j = 0..3 (G-131)


3. The scaling process for residual 4x4 blocks as specified in clause 8.5.12.1 is invoked with bitDepth, qP, and the

4x4 array c as the inputs and the outputs are scaled transform coefficient values as a 4x4 array d with elements
dij. For this invocation of the process in clause 8.5.12.1, the array c is treated as not relating to a luma residual
block coded using the Intra_16x16 macroblock prediction mode and as not relating to a chroma residual block.


4. The list sTCoeff is modified by


sTCoeff[ cO + 16 * c4x4BlkIdx + 4 * i + j ] += dij      with i, j = 0..3 (G-132)


The bitstream shall not contain data that result in any element sTCoeff[ cO + k ] with k = 0..255 that exceeds the range of
integer values from −2 [(7 + bitDepth)] to 2 [(7 + bitDepth)] - 1, inclusive.


The bitstream shall not contain data that result in any element tCoeffLevel[ cO + k ] with k = 0..255 that exceeds the range
of integer values from −2 [(7 + bitDepth)] to 2 [(7 + bitDepth)] - 1, inclusive.


**G.8.5.1.1.3** **Refinement process for transform coefficients of residual 8x8 blocks**


Inputs to this process are:


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a variable bitDepth specifying the bit depth,


- a variable qP specifying the quantization parameter value,


- a variable cO specifying the first coefficient index in the list of scaled transform coefficient values sTCoeff and in the
list of transform coefficient values tCoeffLevel,


- a variable coeffLevel8x8 representing LumaLevel8x8, CbLevel8x8, or CrLevel8x8,


- a list of scaled transform coefficient values sTCoeff with (256 + 2 * MbWidthC * MbHeightC) elements,


- a list of transform coefficient level values tCoeffLevel with (256 + 2 * MbWidthC * MbHeightC) elements.





Outputs of this process are:


- a modified version of the list sTCoeff,


- a modified version of the list tCoeffLevel.


Depending on tcoeff_level_prediction_flag, the following applies:


- If tcoeff_level_prediction_flag is equal to 1, the list sTCoeff is modified by


sTCoeff[ cO + k ] = 0      with k = 0..255 (G-133)


- Otherwise (tcoeff_level_prediction_flag is equal to 0), the list tCoeffLevel is modified by


tCoeffLevel[ cO + k ] = 0   with k = 0..255 (G-134)


For each residual 8x8 block indexed by c8x8BlkIdx = 0..3, the following ordered steps are specified:


1. The inverse scanning process for 8x8 transform coefficients and scaling lists as specified in clause 8.5.7 is

invoked with coeffLevel8x8[ c8x8BlkIdx ] as the input and the outputs are transform coefficient level values as
an 8x8 array c with elements cij. For this invocation of the process in clause 8.5.7, the current macroblock is
treated as field macroblock when fieldMbFlag is equal to 1, and it is treated as frame macroblock when
fieldMbFlag is equal to 0.


2. The list tCoeffLevel and the 8x8 array c are modified by


tCoeffLevel[ cO + 64 * c8x8BlkIdx + 8 * i + j ] += cij   with i, j = 0..7 (G-135)


cij = tCoeffLevel[ cO + 64 * c8x8BlkIdx + 8 * i + j ]    with i, j = 0..7 (G-136)


3. The scaling process for residual 8x8 blocks as specified in clause 8.5.13.1 is invoked with bitDepth, qP, and the

8x8 array c as the inputs and the outputs are scaled transform coefficient values as an 8x8 array d with elements
dij.


4. The list sTCoeff is modified by


sTCoeff[ cO + 64 * c8x8BlkIdx + 8 * i + j ] += dij      with i, j = 0..7 (G-137)


The bitstream shall not contain data that result in any element sTCoeff[ cO + k ] with k = 0..255 that exceeds the range of
integer values from −2 [(7 + bitDepth)] to 2 [(7 + bitDepth)] - 1, inclusive.


The bitstream shall not contain data that result in any element tCoeffLevel[ cO + k ] with k = 0..255 that exceeds the range
of integer values from −2 [(7 + bitDepth)] to 2 [(7 + bitDepth)] - 1, inclusive.


**G.8.5.1.1.4** **Refinement process for transform coefficients of Intra_16x16 macroblocks**


This process is only invoked when base_mode_flag is equal to 0 or tcoeff_level_prediction_flag is equal to 1.


Inputs to this process are:


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a variable bitDepth specifying the bit depth,


- a variable qP specifying the quantization parameter value,


- a variable cO specifying the first coefficient index in the list of scaled transform coefficient values sTCoeff and in the
list of transform coefficient values tCoeffLevel,


- a variable coeffDCLevel representing Intra16x16DCLevel, CbIntra16x16DCLevel, or CrIntra16x16DCLevel,


- a variable coeffACLevel representing Intra16x16ACLevel, CbIntra16x16ACLevel, or CrIntra16x16ACLevel,


- a variable coeffLevel4x4 representing LumaLevel4x4, CbLevel4x4, or CrLevel4x4,


- a list of scaled transform coefficient values sTCoeff with (256 + 2 * MbWidthC * MbHeightC) elements,


- a list of transform coefficient level values tCoeffLevel with (256 + 2 * MbWidthC * MbHeightC) elements.


Outputs of this process are:


- a modified version of the list sTCoeff,


- a modified version of the list tCoeffLevel.
NOTE 1 – When tcoeff_level_prediction_flag is equal to 0, this clause is always invoked as part of an invocation of clause G.8.5.1
with refinementFlag equal to 0, in which case all elements of the list tCoeffLevel are set equal to 0 before invoking this clause.





For the DC transform coefficients of all residual 4x4 blocks, the following ordered steps are specified:


1. Depending on base_mode_flag, the 4x4 array c with elements cij is derived as follows:


     - If base_mode_flag is equal to 0, the inverse scanning process for 4x4 transform coefficients and scaling lists
as specified in clause 8.5.6 is invoked with coeffDCLevel as the input and the outputs are DC transform
coefficient level values for all residual 4x4 blocks as a 4x4 array c with elements cij. For this invocation of
the process in clause 8.5.6, the current macroblock is treated as field macroblock when fieldMbFlag is equal
to 1, and it is treated as frame macroblock when fieldMbFlag is equal to 0.


     - Otherwise (base_mode_flag is equal to 1), the 4x4 array c with elements cij containing DC transform
coefficient level values is derived by


cij = coeffLevel4x4[ 8 * ( i / 2 ) + 4 * ( j / 2 ) + 2 * ( i % 2 ) + ( j % 2 ) ][ 0 ]    with i, j = 0..3 (G-138)


2. The list tCoeffLevel and the 4x4 array c are modified by


tCoeffLevel[ cO + 128 * ( i / 2 ) + 64 * ( j / 2 ) + 32 * ( i % 2 ) + 16 * ( j % 2 ) ] += cij
with i, j = 0..3 (G-139)


cij = tCoeffLevel[ cO + 128 * ( i / 2 ) + 64 * ( j / 2 ) + 32 * ( i % 2 ) + 16 * ( j % 2 ) ]
with i, j = 0..3 (G-140)


3. The scaling and transformation process for DC transform coefficients for Intra_16x16 macroblock type as

specified in clause 8.5.10 is invoked with bitDepth, qP, and c as the inputs and the output is the 4x4 array d with
elements dij representing scaled DC transform coefficient values for all residual 4x4 blocks.


4. The list sTCoeff is modified by


sTCoeff[ cO + 128 * ( i / 2 ) + 64 * ( j / 2 ) + 32 * ( i % 2 ) + 16 * ( j % 2 ) ] = dij
with i, j = 0..3 (G-141)


For each residual 4x4 block indexed by c4x4BlkIdx = 0..15, the following ordered steps are specified:


1. Depending on base_mode_flag, the variable c4x4List, which is a list of 16 entries, is derived as follows:


     - If base_mode_flag is equal to 0, the following applies:


c4x4List[ k ] = ( ( k = = 0 ) ? 0 : coeffACLevel[ c4x4BlkIdx ][ k − 1 ] )     with k = 0..15 (G-142)


      - Otherwise (base_mode_flag is equal to 1), the following applies:


c4x4List[ k ] = ( ( k = = 0 ) ? 0 : coeffLevel4x4[ c4x4BlkIdx ][ k ] )        with k = 0..15 (G-143)


2. The inverse scanning process for 4x4 transform coefficients and scaling lists as specified in clause 8.5.6 is

invoked with c4x4List as the input and the outputs are transform coefficient level values as a 4x4 array e with
elements eij. For this invocation of the process in clause 8.5.6, the current macroblock is treated as field
macroblock when fieldMbFlag is equal to 1, and it is treated as frame macroblock when fieldMbFlag is equal
to 0.


3. The list tCoeffLevel and the 4x4 array e are modified by


tCoeffLevel[ cO + 16 * c4x4BlkIdx + 4 * i + j ] += eij       with i, j = 0..3 and i + j > 0 (G-144)


eij = tCoeffLevel[ cO + 16 * c4x4BlkIdx + 4 * i + j ]        with i, j = 0..3 and i + j > 0 (G-145)


4. The scaling process for residual 4x4 blocks as specified in clause 8.5.12.1 is invoked with bitDepth, qP, and the

4x4 array e as the inputs and the outputs are scaled transform coefficient values as a 4x4 array d with elements
dij. During the process in clause 8.5.12.1, the array e is treated as relating to a luma residual block coded using
the Intra_16x16 macroblock prediction mode.


5. The list sTCoeff is modified by


sTCoeff[ cO + 16 * c4x4BlkIdx + 4 * i + j ] = dij          with i, j = 0..3 and i + j > 0 (G-146)


NOTE 2 – The elements tCoeffLevel[ cO + 16* c4x4BlkIdx ] and sTCoeff[ cO + 16* c4x4BlkIdx ] are not modified
during the process for a residual 4x4 block with index c4x4BlkIdx.


The bitstream shall not contain data that result in any element tCoeffLevel[ cO + k ] with k = 0..255 that exceeds the range
of integer values from −2 [(7 + bitDepth)] to 2 [(7 + bitDepth)] - 1, inclusive.


**G.8.5.1.2** **Refinement process for chroma transform coefficients**


Inputs to this process are:





- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a variable cTrafo specifying the transform type,


- a list of scaled transform coefficient values sTCoeff with (256 + 2 * MbWidthC * MbHeightC) elements,


- a list of transform coefficient level values tCoeffLevel with (256 + 2 * MbWidthC * MbHeightC) elements,


- the chroma quantization parameters qP′Cb and qP′Cr.


Outputs of this process are:


- a modified version of the list sTCoeff,


- a modified version of the list tCoeffLevel.


For both chroma components indexed by iCbCr = 0..1, the following applies:


- If ChromaArrayType is equal to 1 or 2, the following applies:


   - If cTrafo is equal to T_PCM, the assignment process for chroma transform coefficient values for I_PCM
macroblocks as specified in clause G.8.5.1.2.1 is invoked with iCbCr, sTCoeff, and tCoeffLevel as the inputs
and the outputs are modified versions of sTCoeff and tCoeffLevel.


   - Otherwise (cTrafo is not equal to T_PCM), the refinement process for chroma transform coefficients with
ChromaArrayType equal to 1 or 2 as specified in clause G.8.5.1.2.2 is invoked with iCbCr, fieldMbFlag,
sTCoeff, tCoeffLevel, qP′Cb, and qP′Cr as the inputs and the outputs are modified versions of the lists sTCoeff
and tCoeffLevel.


- Otherwise (ChromaArrayType is equal to 3), the refinement process for luma transform coefficients or chroma
transform coefficients with ChromaArrayType equal to 3 as specified in clause G.8.5.1.1 is invoked with iYCbCr set
equal to (1 + iCbCr), fieldMbFlag, cTrafo, sTCoeff, tCoeffLevel, qP′Cb, and qP′Cr as the inputs and the outputs are
modified versions of the lists sTCoeff and tCoeffLevel.


**G.8.5.1.2.1** **Assignment process for chroma transform coefficient values for I_PCM macroblocks**


Inputs to this process are:


- a variable iCbCr specifying the chroma component,


- a list of scaled transform coefficient values sTCoeff with (256 + 2 * MbWidthC * MbHeightC) elements,


- a list of transform coefficient level values tCoeffLevel with (256 + 2 * MbWidthC * MbHeightC) elements.


Outputs of this process are:


- a modified version of the list sTCoeff,


- a modified version of the list tCoeffLevel.


The variable numC is set equal to (MbWidthC * MbHeightC) and the variable cCO is set equal to (iCbCr * numC).


When base_mode_flag is equal to 0, the lists tCoeffLevel and sTCoeff are modified by


tCoeffLevel[ 256 + cCO + k ] = 0                         with k = 0..(numC − 1) (G-147)


sTCoeff[ 256 + cCO + k ] = pcm_sample_chroma[ cCO + k ]   with k = 0..(numC − 1) (G-148)


**G.8.5.1.2.2** **Refinement process for chroma transform coefficients with ChromaArrayType equal to 1 or 2**


This process is only invoked when ChromaArrayType is equal to 1 or 2.


Inputs to this process are:


- a variable iCbCr specifying the chroma component,


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a list of scaled transform coefficient values sTCoeff with (256 + 2 * MbWidthC * MbHeightC) elements,


- a list of transform coefficient level values tCoeffLevel with (256 + 2 * MbWidthC * MbHeightC) elements,


- the chroma quantization parameters qP′Cb and qP′Cr.


Outputs of this process are:


- a modified version of the list sTCoeff,





- a modified version of the list tCoeffLevel.


The variables nW, nH, numB, cO, and qP are derived by


nW  = MbWidthC / 4 (G-149)
nH   = MbHeightC / 4 (G-150)
numB = nW * nH (G-151)
cO   = 256 + ( iCbCr * MbWidthC * MbHeightC ) (G-152)
qP   = ( ( iCbCr = = 0 ) ? qP′Cb : qP′Cr ) (G-153)


Depending on tcoeff_level_prediction_flag, the following applies:


- If tcoeff_level_prediction_flag is equal to 1, the list sTCoeff is modified by


sTCoeff[ cO + k ] = 0      with k = 0..(MbWidthC * MbHeightC − 1) (G-154)


- Otherwise (tcoeff_level_prediction_flag is equal to 0), the list tCoeffLevel is modified by


tCoeffLevel[ cO + k ] = 0   with k = 0..(MbWidthC * MbHeightC − 1) (G-155)


For the chroma DC transform coefficients of all residual 4x4 chroma blocks, the following ordered steps are specified:


1. Depending on ChromaArrayType, the (nW)x(nH) array c with elements cij is derived as follows:


     - If ChromaArrayType is equal to 1,


cij = ChromaDCLevel[ iCbCr ][ 2 * i + j ]              with i = 0..(nH − 1), j = 0..(nW − 1) (G-156)


     - Otherwise (ChromaArrayType is equal to 2),


cij = ChromaDCLevel[ iCbCr ][ scan422ChromaDC[ 2 * i + j ] ]
with i = 0..(nH − 1), j = 0..(nW − 1),
and scan422ChromaDC = { 0, 2, 1, 5, 3, 6, 4, 7 } (G-157)


2. The list tCoeffLevel and the (nW)x(nH) array c are modified by


tCoeffLevel[ cO + 32 * i + 16 * j ] += cij               with i = 0..(nH − 1), j = 0..(nW − 1) (G-158)


cij = tCoeffLevel[ cO + 32 * i + 16 * j ]                with i = 0..(nH − 1), j = 0..(nW − 1) (G-159)


3. The variable qPDC is derived by


qPDC = ( ( ChromaArrayType = = 1 ) ? qP : ( qP + 3 ) ) (G-160)


4. The (nW)x(nH) array d with elements dij representing scaled chroma DC transform coefficient values for all

residual 4x4 chroma blocks is derived by


dij = cij * ( LevelScale4x4( qPDC % 6, 0, 0 ) << ( qPDC / 6 ) )  with i = 0..(nH − 1), j = 0..(nW − 1)
(G-161)


5. The list sTCoeff is modified by


sTCoeff[ cO + 32 * i + 16 * j ] += dij                  with i = 0..(nH − 1), j = 0..(nW − 1) (G-162)


For each residual 4x4 chroma block indexed by c4x4BlkIdx = 0..(numB − 1), the following ordered steps are specified:


1. The variable c4x4List, which is a list of 16 entries, is derived by


c4x4List[ k ] = ( ( k = = 0 ) ? 0 : ChromaACLevel[ iCbCr ][ c4x4BlkIdx ][ k − 1 ] )
with k = 0..15 (G-163)


2. The inverse scanning process for 4x4 transform coefficients and scaling lists as specified in clause 8.5.6 is

invoked with c4x4List as the input and the outputs are chroma transform coefficient level values as a 4x4 array
e with elements eij. During the process in clause 8.5.6, the current macroblock is treated as field macroblock when
fieldMbFlag is equal to 1, and it is treated as frame macroblock when fieldMbFlag is equal to 0.


3. The list tCoeffLevel and the 4x4 array e are modified by


tCoeffLevel[ cO + 16 * c4x4BlkIdx + 4 * i + j ] += eij    with i, j = 0..3 and i + j > 0 (G-164)


eij = tCoeffLevel[ cO + 16 * c4x4BlkIdx + 4 * i + j ]     with i, j = 0..3 and i + j > 0 (G-165)


4. The scaling process for residual 4x4 blocks as specified in clause 8.5.12.1 is invoked with BitDepthC, qP, and the

4x4 array e as the inputs and the outputs are scaled chroma transform coefficient values as a 4x4 array d of with
elements dij. During the process in clause 8.5.12.1, the array e is treated as relating to a chroma residual block.


5. The list sTCoeff is modified by





sTCoeff[ cO + 16 * c4x4BlkIdx + 4 * i + j ] += dij       with i, j = 0..3 and i + j > 0 (G-166)


NOTE 1 – The elements tCoeffLevel[ cO + 16* c4x4BlkIdx ] and sTCoeff[ cO + 16* c4x4BlkIdx ] are not modified
during the process for a residual 4x4 chroma block with index c4x4BlkIdx.


The bitstream shall not contain data that result in any element sTCoeff[ cO + 16* b + k ] with b = 0..(numB − 1) and
k = 1..15 that exceeds the range of integer values from −2 [(7 + BitDepth][C][)] to 2 [(7 + BitDepth][C][)] - 1, inclusive.


The bitstream shall not contain data that result in any element tCoeffLevel[ cO + 16* b + k ] with b = 0..(numB − 1) and
k = 1..15 that exceeds the range of integer values from −2 [(7 + BitDepth][C][)] to 2 [(7 + BitDepth][C][)] - 1, inclusive.


NOTE 2 – The elements tCoeffLevel[ cO + 16* b ] and sTCoeff[ cO + 16* b ] with b = 0..(numB − 1) can exceed the range of
integer values from −2 [(7 + BitDepth][C][)] to 2 [(7 + BitDepth][C][)]   - 1, inclusive.


**G.8.5.2** **Transform coefficient level scaling process prior to transform coefficient refinement**


Inputs to this process are:


- a variable cTrafo specifying the luma transform type for the current macroblock,


- a list tCoeffLevel with (256 + 2 * MbWidthC * MbHeightC) elements specifying transform coefficient level values
for the current macroblock,


- a variable tQPY specifying the luma quantization parameter for the current macroblock,


- a variable refQPY specifying the quantization parameter for the macroblock of the reference layer representation,


- when ChromaArrayType is not equal to 0, two variables tQPCb and tQPCr specifying chroma quantization parameters
for the current macroblock,


- when ChromaArrayType is not equal to 0, two variables refQPCb and refQPCr specifying chroma quantization
parameters for the macroblock of the reference layer representation,


Output of this process is a modified version of the list tCoeffLevel.


Table G-6 specifies the scale values cS for transform coefficient level scaling.


**Table G-6 – Scale values cS for transform coefficient level scaling**

|( refQP − cQP + 54 ) % 6|scale value cS|
|---|---|
|0|8|
|1|9|
|2|10|
|3|11|
|4|13|
|5|14|



The variable iYCbCr proceeds over the values from 0 to ( ( ChromaArrayType = = 0 ) ? 0 : 2), inclusive, and for each
value of iYCbCr, the following ordered steps are specified:


1. The variables cO, iMax, cQP, and refQP are derived by


cO    = ( ( iYCbCr = = 0 ) ?      0 : ( 256 + ( iYCbCr − 1 ) * MbWidthC * MbHeightC ) ) (G-167)
iMax  = ( ( iYCbCr = = 0 ) ?    255 : ( MbWidthC * MbHeightC − 1 ) ) (G-168)
cQP   = ( ( iYCbCr = = 0 ) ?   tQPY : ( iYCbCr = = 1 ?   tQPCb :   tQPCr ) ) (G-169)
refQP = ( ( iYCbCr = = 0 ) ? refQPY : ( iYCbCr = = 1 ? refQPCb : refQPCr ) ) (G-170)


2. The variable cS is set as specified in Table G-6 using the values of refQP and cQP.


3. The variable rShift is calculated by


rShift = ( refQP − cQP + 54 ) / 6 (G-171)


4. The list tCoeffLevel of transform coefficient level values is modified by


tCoeffLevel[ cO + i ] = ( ( cS * tCoeffLevel[ cO + i ] ) << rShift ) >> 12   with i = 0..iMax (G-172)


The following constraints shall be obeyed:





a) The bitstream shall not contain data that result in any element tCoeffLevel[ k ] with k = 0..255 that exceeds the

range of integer values from −2 [(7 + BitDepth][Y][)] to 2 [(7 + BitDepth][Y][)]     - 1, inclusive.


b) When ChromaArrayType is equal to 1 or 2, the bitstream shall not contain data that result in any element

tCoeffLevel[ 256 + 16* b + k ] with b = 0..(MbWidthC * MbHeightC / 8 − 1), and k = 1..15 that exceeds the
range of integer values from −2 [(7 + BitDepth][C][)] to 2 [(7 + BitDepth][C][)]     - 1, inclusive.

NOTE 1 – When ChromaArrayType is equal to 1 or 2 and cTrafo is not equal to T_PCM, the elements
tCoeffLevel[ 256 + 16* b ] with b = 0..(MbWidthC * MbHeightC / 8 − 1) can exceed the range of integer values from
−2 [(7 + BitDepth][C][)] to 2 [(7 + BitDepth][C][)]     - 1, inclusive.


c) When ChromaArrayType is equal to 3, the bitstream shall not contain data that result in any element

tCoeffLevel[ 256 + k ] with k = 0..511 that exceeds the range of integer values from −2 [(7 + BitDepth][C][)]
to 2 [(7 + BitDepth][C][)]      - 1, inclusive.

NOTE 2 – When tQPY is less than 10 and cTrafo is equal to T_16x16, the range of values that can be represented by an
alternative representation of the bitstream with entropy_coding_mode_flag equal to 0 and profile_idc equal to 66, 77,
or 88, may not be sufficient to represent the full range of values of the elements tCoeffLevel[ 16 * b ] with b = 0..15
that could be necessary to form a close approximation of the content of any possible source picture.
NOTE 3 – When ChromaArrayType is equal to 1 or 2 and tQPCX with CX being replaced by Cb and Cr is less than 4,
the range of values that can be represented by an alternative representation of the bitstream with
entropy_coding_mode_flag equal to 0 and profile_idc equal to 66, 77, or 88, may not be sufficient to represent the full
range of values of the elements tCoeffLevel[ 256 + 16 * b ] with b = 0..(MbWidthC * MbHeightC / 8 − 1) that could
be necessary to form a close approximation of the content of any possible source picture.


**G.8.5.3** **Residual construction and accumulation process**


Inputs to this process are:


- a variable accumulationFlag specifying whether the constructed residual sample values for the current macroblock
are combined with the existent residual sample value for the macroblock,


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a variable cTrafo specifying the transform type,


- a list of scaled transform coefficient values sTCoeff with (256 + 2 * MbWidthC * MbHeightC) elements,


- a (PicWidthInSamplesL)x(PicHeightInSamplesL) array picResL containing residual luma sample values for the current
layer representation,


- when ChromaArrayType is not equal to 0, two (PicWidthInSamplesC)x(PicHeightInSamplesC) arrays picResCb and
picResCr containing residual chroma sample values for the current layer representation.


Outputs of this process are:


- a modified version of the array picResL,


- when ChromaArrayType is not equal to 0, modified versions of the arrays picResCb and picResCr.


The construction process for luma residuals as specified in clause G.8.5.3.1 is invoked with cTrafo and sTCoeff as the
inputs and the outputs are residual luma sample values as a 16x16 array mbResL.


When ChromaArrayType is not equal to 0, the construction process for chroma residuals as specified in clause G.8.5.3.2
is invoked with cTrafo and sTCoeff as the inputs and the outputs are residual chroma sample values as two
(MbWidthC)x(MbHeightC) arrays mbResCb and mbResCr.


When accumulationFlag is equal to 1, the following ordered steps are specified:


1. The macroblock sample array extraction process as specified in clause G.8.5.4.2 is invoked with fieldMbFlag,

picResL, and, when ChromaArrayType is equal to 0, picResCb and picResCr as the inputs and the outputs are a
16x16 array refLayerMbResL and, when ChromaArrayType is not equal to 0, two (MbWidthC)x(MbHeightC)
arrays refLayerMbResCb and refLayerMbResCr.


2. All elements mbResL[ x, y ] of the 16x16 array mbResL with x, y = 0..15 are modified by


mbResL[ x, y ] = Clip3( yMin, yMax, mbResL[ x, y ] + refLayerMbResL[ x, y ] ) (G-173)


with


yMin = − ( 1 << BitDepthY ) + 1 (G-174)
yMax =  ( 1 << BitDepthY ) − 1 (G-175)





3. When ChromaArrayType is not equal to 0, for CX being replaced by Cb and Cr, all elements mbResCX[ x, y ] of

the (MbWidthC)x(MbHeightC) array mbResCX with x = 0..(MbWidthC − 1) and y = 0..(MbHeightC − 1) are
modified by


mbResCX[ x, y ] = Clip3( cMin, cMax, mbResCX[ x, y ] + refLayerMbResCX[ x, y ] ) (G-176)


with


cMin = − ( 1 << BitDepthC ) + 1 (G-177)
cMax =  ( 1 << BitDepthC ) − 1 (G-178)


The picture sample array construction process as specified in clause G.8.5.4.1 is invoked with fieldMbFlag, mbResL,
picResL, and, when ChromaArrayType is not equal to 0, mbResCb, mbResCr, picResCb, and picResCr as the inputs and the
outputs are a modified version of the array picResL and, when ChromaArrayType is not equal to 0, modified versions of
the arrays picResCb and picResCr.


**G.8.5.3.1** **Construction process for luma residuals or chroma residuals with ChromaArrayType equal to 3**


Inputs to this process are:


- a variable iYCbCr specifying the colour component (when present),


- a variable cTrafo specifying the transform type,


- a list of scaled transform coefficient values sTCoeff with (256 + 2 * MbWidthC * MbHeightC) elements.


Outputs of this process are residual sample values as a 16x16 array mbRes with elements mbRes[ x, y ].


When iYCbCr is not present as input to this clause, it is inferred to be equal to 0.


Depending on iYCbCr, the variables bitDepth and cO are derived as follows:


- If iYCbCr is equal to 0, bitDepth is set equal to BitDepthY and cO is set equal to 0.


- Otherwise, if iYCbCr is equal to 1, bitDepth is set equal to BitDepthC and cO is set equal to 256.


- Otherwise (iYCbCr is equal to 2), bitDepth is set equal to BitDepthC and cO is set equal to
(256 + MbWidthC * MbHeightC).


Depending on cTrafo, the 16x16 array mbRes is derived as follows:


- If cTrafo is equal to T_PCM, the construction process for luma residuals or chroma residuals with ChromaArrayType
equal to 3 of I_PCM macroblocks as specified in clause G.8.5.3.1.1 is invoked with cO and sTCoeff as the inputs
and the output is the 16x16 array mbRes of residual sample values.


- Otherwise, if cTrafo is equal to T_4x4, the construction process for residual 4x4 blocks as specified in
clause G.8.5.3.1.2 is invoked with bitDepth, cO, and sTCoeff as the inputs and the output is the 16x16 array mbRes
of residual sample values.


- Otherwise, if cTrafo is equal to T_8x8, the construction process for residual 8x8 blocks as specified in
clause G.8.5.3.1.3 is invoked with bitDepth, cO, and sTCoeff as the inputs and the output is the 16x16 array mbRes
of residual sample values.


- Otherwise (cTrafo is equal to T_16x16), the construction process for residuals of Intra_16x16 macroblocks as
specified in clause G.8.5.3.1.4 is invoked with bitDepth, cO, and sTCoeff as the inputs and the output is the 16x16
array mbRes of residual sample values.


**G.8.5.3.1.1** **Construction process for luma residuals or chroma residuals with ChromaArrayType equal to 3 of**
**I_PCM macroblocks**


Inputs to this process are:


- a variable cO specifying the first coefficient index in the list of scaled transform coefficient values sTCoeff,


- a list of scaled transform coefficient values sTCoeff with (256 + 2 * MbWidthC * MbHeightC) elements.


Outputs of this process are residual sample values as a 16x16 array mbRes with elements mbRes[ x, y ].


The 16x16 array mbRes is derived by:


mbRes[ x, y ] = sTCoeff[ cO + y * 16 + x ]   with x, y = 0..15 (G-179)


**G.8.5.3.1.2** **Construction process for residual 4x4 blocks**


Inputs to this process are:





- a variable bitDepth specifying the bit depth,


- a variable cO specifying the first coefficient index in the list of scaled transform coefficient values sTCoeff,


- a list of scaled transform coefficient values sTCoeff with (256 + 2 * MbWidthC * MbHeightC) elements.


Outputs of this process are residual sample values as a 16x16 array mbRes with elements mbRes[ x, y ].


For each residual 4x4 block indexed by c4x4BlkIdx = 0..15, the following ordered steps are specified:


1. The 4x4 array d with elements dij is derived by:


dij = sTCoeff[ cO + 16 * c4x4BlkIdx + 4 * i + j ]     with i, j = 0..3 (G-180)


2. The transformation process for residual 4x4 blocks as specified in clause 8.5.12.2 is invoked with bitDepth and

the 4x4 array d as the inputs and the outputs are residual sample value as a 4x4 array r with elements rij.


3. The inverse 4x4 luma block scanning process as specified in clause 6.4.3 is invoked with c4x4BlkIdx as the input

and the output is assigned to ( xP, yP ).


4. The elements mbRes[ x, y ] of the 16x16 array mbRes with x = xP..(xP + 3) and y = yP..(yP + 3) are derived by


mbRes[ xP + j, yP + i ] = rij     with i, j = 0..3 (G-181)


**G.8.5.3.1.3** **Construction process for residual 8x8 blocks**


Inputs to this process are:


- a variable bitDepth specifying the bit depth,


- a variable cO specifying the first coefficient index in the list of scaled transform coefficient values sTCoeff,


- a list of scaled transform coefficient values sTCoeff with (256 + 2 * MbWidthC * MbHeightC) elements.


Outputs of this process are residual sample values as a 16x16 array mbRes with elements mbRes[ x, y ].


For each residual 8x8 block indexed by c8x8BlkIdx = 0..3, the following ordered steps are specified:


1. The 8x8 array d with elements dij is derived by:


dij = sTCoeff[ cO + 64 * c8x8BlkIdx + 8 * i + j ]     with i, j = 0..7 (G-182)


2. The transformation process for residual 8x8 blocks as specified in clause 8.5.13.2 is invoked with bitDepth and

the 8x8 array d as the inputs and the outputs are residual sample values as an 8x8 array r with elements rij.


3. The inverse 8x8 luma block scanning process as specified in clause 6.4.5 is invoked with c8x8BlkIdx as the input

and the output is assigned to ( xP, yP ).


4. The elements mbRes[ x, y ] of the 16x16 array mbRes with x = xP..(xP + 7) and y = yP..(yP + 7) are derived by


mbRes[ xP + j, yP + i ] = rij     with i, j = 0..7 (G-183)


**G.8.5.3.1.4** **Construction process for residuals of Intra_16x16 macroblocks**


Inputs to this process are:


- a variable bitDepth specifying the bit depth,


- a variable cO specifying the first coefficient index in the list of scaled transform coefficient values sTCoeff,


- a list of scaled transform coefficient values sTCoeff with (256 + 2 * MbWidthC * MbHeightC) elements.


Outputs of this process are residual sample values as a 16x16 array mbRes with elements mbRes[ x, y ].


For each residual 4x4 block indexed by c4x4BlkIdx = 0..15, the following ordered steps are specified:


1. The 4x4 array d with elements dij is derived by:


dij = sTCoeff[ cO + 16 * c4x4BlkIdx + 4 * i + j ]    with i, j = 0..3 (G-184)


2. The transformation process for residual 4x4 blocks as specified in clause 8.5.12.2 is invoked with bitDepth and

the 4x4 array d as the inputs and the outputs are residual sample values as a 4x4 array r with elements rij.


3. The inverse 4x4 luma block scanning process as specified in clause 6.4.3 is invoked with c4x4BlkIdx as the input

and the output is assigned to ( xP, yP ).


4. The elements mbRes[ x, y ] of the 16x16 array mbRes with x = xP..(xP + 3) and y = yP..(yP + 3) are derived by





mbRes[ xP + j, yP + i ] = rij     with i, j = 0..3 (G-185)


**G.8.5.3.2** **Construction process for chroma residuals**


Inputs to this process are:


- a variable cTrafo specifying the transform type,


- a list of scaled transform coefficient values sTCoeff with (256 + 2 * MbWidthC * MbHeightC) elements.


Outputs of this process are residual chroma sample values as two (MbWidthC)x(MbHeightC) arrays mbResCb and mbResCr
with elements mbResCb[ x, y ] and mbResCr[ x, y ], respectively.


For both chroma components indexed by iCbCr = 0..1 and for CX being replaced by Cb for iCbCr equal to 0 and Cr for
iCbCr equal to 1, the following applies:


- If ChromaArrayType is equal to 1 or 2, the following applies:


   - If cTrafo is equal to T_PCM, the construction process for chroma residuals of I_PCM macroblocks as specified
in clause G.8.5.3.2.1 is invoked with iCbCr and sTCoeff as the inputs and the output is the
(MbWidthC)x(MbHeightC) array mbResCX of residual chroma sample values.


   - Otherwise (cTrafo is not equal to T_PCM), the construction process for chroma residuals with
ChromaArrayType equal to 1 or 2 as specified in clause G.8.5.3.2.2 is invoked with iCbCr and sTCoeff as the
inputs and the output is the (MbWidthC)x(MbHeightC) array mbResCX of residual chroma sample values.


- Otherwise (ChromaArrayType is equal to 3), the construction process for luma residuals or chroma residuals with
ChromaArrayType equal to 3 as specified in clause G.8.5.3.1 is invoked with iYCbCr set equal to (1 + iCbCr), cTrafo,
and sTCoeff as the inputs and the output is the (MbWidthC)x(MbHeightC) array mbResCX of residual chroma sample
values.


**G.8.5.3.2.1** **Construction process for chroma residuals of I_PCM macroblocks**


Inputs to this process are:


- a variable iCbCr specifying the chroma component,


- a list of scaled transform coefficient values sTCoeff with (256 + 2 * MbWidthC * MbHeightC) elements.


Outputs of this process are residual chroma sample values as a (MbWidthC)x(MbHeightC) array mbRes with elements
mbRes[ x, y ].


The (MbWidthC)x(MbHeightC) array mbRes is derived by


mbRes[ x, y ] = sTCoeff[ 256 + iCbCr * MbWidthC * MbHeightC + y * MbWidthC + x ]
(G-186)
with x = 0..(MbWidthC − 1) and y = 0..(MbHeightC − 1)


**G.8.5.3.2.2** **Construction process for chroma residuals with ChromaArrayType equal to 1 or 2**


This process is only invoked when ChromaArrayType is equal to 1 or 2.


Inputs to this process are:


- a variable iCbCr specifying the chroma component,


- a list of scaled transform coefficient values sTCoeff with (256 + 2 * MbWidthC * MbHeightC) elements.


Outputs of this process are residual chroma sample values as a (MbWidthC)x(MbHeightC) array mbRes with elements
mbRes[ x, y ].


The variables nW, nH, numB, and cO are derived by


nW  = MbWidthC / 4 (G-187)
nH   = MbHeightC / 4 (G-188)
numB = nW * nH (G-189)
cO   = 256 + ( iCbCr * MbWidthC * MbHeightC ) (G-190)


For the chroma DC transform coefficients of all residual 4x4 chroma blocks, the following ordered steps are specified:


1. The (nW)x(nH) array c with the elements cij is derived by


cij = sTCoeff[ cO + 32 * i + 16 * j ]     with i = 0..(nH − 1), j = 0..(nW − 1) (G-191)





2. The transformation process for chroma DC transform coefficients as specified in clause 8.5.11.1 is invoked with

BitDepthC and the (nW)x(nH) array c as the inputs and the outputs are DC values for all residual 4x4 chroma
blocks as a (nW)x(nH) array f with elements fij.


3. Depending on ChromaArrayType, the (nW)x(nH) array dcC with elements dcCij is derived as follows:


     - If ChromaArrayType is equal to 1,


dcCij = fij >> 5                      with i = 0..(nH − 1), j = 0..(nW − 1) (G-192)


     - Otherwise (ChromaArrayType is equal to 2),


dcCij = ( fij + ( 1 << 5 ) ) >> 6          with i = 0..(nH − 1), j = 0..(nW − 1) (G-193)


The bitstream shall not contain data that result in any element dcCij of dcC with i = 0..(nH − 1) and
j = 0..(nW − 1) that exceeds the range of integer values from −2 [(7 + BitDepth][C][)] to 2 [(7 + BitDepth][C][)]      - 1, inclusive.


NOTE – For the layer representation with dependency_id equal to 0 and quality_id equal to 0, successive invocations
of clause G.8.5.1.2 (as part of an invocation of clause G.8.5.1) and this clause yield an array dcC that is identical to the
array dcC that would be obtained by an invocation of clause 8.5.11. However, the intermediate values cij and fij with
i = 0..(nH − 1) and j = 0..(nW − 1) that are derived in this clause can exceed the range of integer values from
−2 [(7 + BitDepth][C][)] to 2 [(7 + BitDepth][C][)]       - 1, inclusive.


For each residual 4x4 chroma block indexed by c4x4BlkIdx = 0..(numB − 1), the following ordered steps are specified.


1. The 4x4 array d with elements dij is derived as follows:


     - The element d00 is derived by:


d00 = dcCkl   with k = c4x4BlkIdx / 2 and l = c4x4BlkIdx % 2 (G-194)


     - The elements dij with i, j = 0..3 and i + j > 0 are derived by:


dij = sTCoeff[ cO + 16 * c4x4BlkIdx + 4 * i + j ] (G-195)


2. The transformation process for residual 4x4 blocks as specified in clause 8.5.12.2 is invoked with BitDepthC and

the 4x4 array d as the inputs and the outputs are residual chroma sample values as a 4x4 array r with elements rij.


3. The chroma location ( xP, yP ) is derived by:


xP = 4 * ( c4x4BlkIdx % 2 ) (G-196)
yP = 4 * ( c4x4BlkIdx / 2 ) (G-197)


4. The elements mbRes[ x, y ] of the (MbWidthC)x(MbHeightC) array mbRes with x = xP..(xP + 3) and

y = yP..(yP + 3) are derived by:


mbRes[ xP + j, yP + i ] = rij     with i, j = 0..3 (G-198)


**G.8.5.4** **Sample array accumulation process**


Inputs to this process are:


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a (PicWidthInSamplesL)x(PicHeightInSamplesL) array picResL containing residual luma sample values for the current
layer representation,


- a (PicWidthInSamplesL)x(PicHeightInSamplesL) array picSamplesL containing constructed luma sample values for
the current layer representation,


- when ChromaArrayType is not equal to 0, two (PicWidthInSamplesC)x(PicHeightInSamplesC) arrays picResCb and
picResCr containing residual chroma sample values for the current layer representation,


- when ChromaArrayType is not equal to 0, two (PicWidthInSamplesC)x(PicHeightInSamplesC) arrays picSamplesCb
and picSamplesCr containing constructed chroma sample values for the current layer representation.


Outputs of this process are:


- a modified version of the array picSamplesL,


- when ChromaArrayType is not equal to 0, modified versions of the arrays picSamplesCb and picSamplesCr.


The macroblock sample array extraction process as specified in clause G.8.5.4.2 is invoked with fieldMbFlag, picResL,
and, when ChromaArrayType is not equal to 0, picResCb and picResCr as the inputs and the outputs are assigned to mbResL
and, when ChromaArrayType is not equal to 0, mbResCb and mbResCr.





The macroblock sample array extraction process as specified in clause G.8.5.4.2 is invoked with fieldMbFlag,
picSamplesL, and, when ChromaArrayType is not equal to 0, picSamplesCb and picSamplesCr as the inputs and the outputs
are assigned to mbPredL and, when ChromaArrayType is not equal to 0, mbPredCb and mbPredCr.


The 16x16 array mbSamplesL is derived by:


mbSamplesL[ x, y ] = Clip1Y( mbPredL[ x, y ] + mbResL[ x, y ] )       with x, y = 0..15 (G-199)


When ChromaArrayType is not equal to 0, for CX being replaced by Cb and Cr, the (MbWidthC)x(MbHeightC) array
mbSamplesCX is derived by:


mbSamplesCX[ x, y ] = Clip1C( mbPredCX[ x, y ] + mbResCX[ x, y ] )    with x = 0..(MbWidthC − 1)
and y = 0..(MbHeightC − 1) (G-200)


The picture sample array construction process as specified in clause G.8.5.4.1 is invoked with fieldMbFlag, mbSamplesL,
picSamplesL, and, when ChromaArrayType is not equal to 0, mbSamplesCb, mbSamplesCr, picSamplesCb, and picSamplesCr
as inputs and the outputs are a modified version of picSamplesL and, when ChromaArrayType is not equal to 0, modified
versions of picSamplesCb and picSamplesCr.


**G.8.5.4.1** **Picture sample array construction process**


Inputs to this process are:


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a 16x16 array mbArrayL containing luma sample values for the current macroblock,


- a (PicWidthInSamplesL)x(PicWidthInHeightL) array picArrayL containing luma sample values for the current layer
representation,


- when ChromaArrayType is not equal to 0, two (MbWidthC)x(MbHeightC) arrays mbArrayCb and mbArrayCr
containing chroma sample values for the current macroblock,


- when ChromaArrayType is not equal to 0, two (PicWidthInSamplesC)x(PicHeightInSamplesC) arrays picArrayCb and
picArrayCr containing chroma sample values for the current layer representation.


Outputs of this process are:


- a modified version of the array picArrayL,


- when ChromaArrayType is not equal to 0, modified versions of the arrays picArrayCb and picArrayCr.


The picture sample array construction process for a colour component as specified in clause G.8.5.4.3 is invoked with
fieldMbFlag, mbW set equal to 16, mbH set equal to 16, mbArrayL, and picArrayL as the inputs and the output is a modified
version of the array picArrayL.


When ChromaArrayType is not equal to 0, for CX being replaced with Cr and Cb, the picture sample array construction
process for a colour component as specified in clause G.8.5.4.3 is invoked with fieldMbFlag, mbW set equal to MbWidthC,
mbH set equal to MbHeightC, mbArrayCX, and picArrayCX as the inputs and the output is a modified version of the array
picArrayCX.


**G.8.5.4.2** **Macroblock sample array extraction process**


Inputs to this process are:


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a (PicWidthInSamplesL)x(PicWidthInHeightL) array picArrayL containing luma sample values for the current layer
representation,


- when ChromaArrayType is not equal to 0, two (PicWidthInSamplesC)x(PicHeightInSamplesC) arrays picArrayCb and
picArrayCr containing chroma sample values for the current layer representation.


Outputs of this process are:


- a 16x16 array mbArrayL containing luma sample values for the current macroblock,


- when ChromaArrayType is not equal to 0, two (MbWidthC)x(MbHeightC) arrays mbArrayCb and mbArrayCr
containing chroma sample values for the current macroblock.


The macroblock sample array extraction process for a colour component as specified in clause G.8.5.4.4 is invoked with
fieldMbFlag, mbW set equal to 16, mbH set equal to 16, and picArrayL as the inputs and the output is assigned to mbArrayL.





When ChromaArrayType is not equal to 0, for CX being replaced with Cr and Cb, the macroblock sample array extraction
process for a colour component as specified in clause G.8.5.4.4 is invoked with fieldMbFlag, mbW set equal to MbWidthC,
mbH set equal to MbHeightC, and picArrayCX as the inputs and the output is assigned to mbArrayCX.


**G.8.5.4.3** **Picture sample array construction process for a colour component**


Inputs to this process are:


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a variable mbW specifying the width of a macroblock colour component in samples,


- a variable mbH specifying the height of a macroblock colour component in samples,


- an (mbW)x(mbH) array mbArray containing sample values for a colour component of the current macroblock,


- an (mbW * PicWidthInMbs)x(mbH * PicHeightInMbs) array picArray containing sample values for a colour
component of the current layer representation.


Output of this process is a modified version of the array picArray.


The inverse macroblock scanning process as specified in clause 6.4.1 is invoked with CurrMbAddr as the input and the
output is assigned to ( xO, yO ). During the process in clause 6.4.1, the current macroblock is treated as field macroblock
when fieldMbFlag is equal to 1, and it is treated as frame macroblock when fieldMbFlag is equal to 0.


The sample location ( xP, yP ) is derived by:


xP =  ( xO >> 4 ) * mbW (G-201)
yP = ( ( yO >> 4 ) * mbH ) + ( yO % 2 ) (G-202)


Depending on the variables MbaffFrameFlag and fieldMbFlag, the array picArray is modified as follows:


- If MbaffFrameFlag is equal to 1 and fieldMbFlag is equal to 1,


picArray[ xP + x, yP + 2 * y ] = mbArray[ x, y ]     with x = 0..(mbW − 1), y = 0..(mbH − 1) (G-203)


- Otherwise (MbaffFrameFlag is equal to 0 or fieldMbFlag is equal to 0),


picArray[ xP + x, yP + y ] = mbArray[ x, y ]        with x = 0..(mbW − 1), y = 0..(mbH − 1) (G-204)


**G.8.5.4.4** **Macroblock sample array extraction process for a colour component**


Inputs to this process are:


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a variable mbW specifying the width of a macroblock colour component in samples,


- a variable mbH specifying the height of a macroblock colour component in samples,


- an (mbW * PicWidthInMbs)x(mbH * PicHeightInMbs) array picArray containing sample values for a colour
component of the current layer representation.


Output of this process is an (mbW)x(mbH) array mbArray containing sample values for a colour component of the current
macroblock.


The inverse macroblock scanning process as specified in clause 6.4.1 is invoked with CurrMbAddr as the input and the
output is assigned to ( xO, yO ). During the process in clause 6.4.1, the current macroblock is treated as field macroblock
when fieldMbFlag is equal to 1, and it is treated as frame macroblock when fieldMbFlag is equal to 0.


The sample location ( xP, yP ) is derived by:


xP =  ( xO >> 4 ) * mbW (G-205)
yP = ( ( yO >> 4 ) * mbH ) + ( yO % 2 ) (G-206)


Depending on the variables MbaffFrameFlag and fieldMbFlag, the samples of the array mbArray are derived as follows:


- If MbaffFrameFlag is equal to 1 and fieldMbFlag is equal to 1,


mbArray[ x, y ] = picArray[ xP + x, yP + 2 * y ]     with x = 0..(mbW − 1), y = 0..(mbH − 1) (G-207)


- Otherwise (MbaffFrameFlag is equal to 0 or fieldMbFlag is equal to 0),


mbArray[ x, y ] = picArray[ xP + x, yP + y ]        with x = 0..(mbW − 1), y = 0..(mbH − 1) (G-208)





**G.8.5.5** **Sample array re-initialization process**


Inputs to this process are:


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a (PicWidthInSamplesL)x(PicHeightInSamplesL) array picSamplesL containing luma sample values for the current
layer representation,


- when ChromaArrayType is not equal to 0, two (PicWidthInSamplesC)x(PicHeightInSamplesC) arrays picSamplesCb
and picSamplesCr containing chroma sample values for the current layer representation.


Outputs of this process are:


- a modified version of the array picSamplesL,


- when ChromaArrayType is not equal to 0, modified versions of the arrays picSamplesCb and picSamplesCr.


The 16x16 array mbSamplesL is derived by:


mbSamplesL[ x, y ] = 0       with x, y = 0..15 (G-209)


When ChromaArrayType is not equal to 0, for CX being replaced by Cb and Cr, the (MbWidthC)x(MbHeightC) array
mbSamplesCX is derived by:


mbSamplesCX[ x, y ] = 0      with x = 0..(MbWidthC − 1) and y = 0..(MbHeightC − 1) (G-210)


The picture sample array construction process as specified in clause G.8.5.4.1 is invoked with fieldMbFlag, mbSamplesL,
picSamplesL, and, when ChromaArrayType is not equal to 0, mbSamplesCb, mbSamplesCr, picSamplesCb, and picSamplesCr
as inputs and the outputs are a modified version of picSamplesL and, when ChromaArrayType is not equal to 0, modified
versions of picSamplesCb and picSamplesCr.


**G.8.6** **Resampling processes for prediction data, intra samples, and residual samples**


Clause G.8.6.1 specifies the derivation process for inter-layer predictors for macroblock type, sub-macroblock type,
references indices, and motion vectors.


Clause G.8.6.2 specifies the resampling process for intra samples.


Clause G.8.6.3 specifies the resampling process for residual samples.


**G.8.6.1** **Derivation process for inter-layer predictors for macroblock type, sub-macroblock type, reference indices,**

**and motion vectors**


This process is only invoked when base_mode_flag is equal to 1 or any motion_prediction_flag_lX[ mbPartIdx ] with X
being replaced by 0 and 1 and mbPartIdx = 0..3 is equal to 1.


Inputs to this process are:


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a one-dimensional array refLayerFieldMbFlag with RefLayerPicSizeInMbs elements specifying which macroblocks
of the reference layer representation are field macroblocks and which macroblocks are frame macroblocks,


- a one-dimensional array refLayerMbType with RefLayerPicSizeInMbs elements specifying the macroblock types for
the macroblocks of the reference layer representation,


- a (RefLayerPicSizeInMbs)x4 array refLayerSubMbType specifying the sub-macroblock types for the macroblocks of
the reference layer representation,


- two (RefLayerPicSizeInMbs)x4 arrays refLayerPredFlagL0 and refLayerPredFlagL1 specifying prediction utilization
flags for the macroblocks of the reference layer representation,


- two (RefLayerPicSizeInMbs)x4 arrays refLayerRefIdxL0 and refLayerRefIdxL1 specifying reference indices for the
macroblocks of the reference layer representation,


- two (RefLayerPicSizeInMbs)x4x4x2 arrays refLayerMvL0 and refLayerMvL1 specifying motion vector components
for the macroblocks of the reference layer representation,


- when CroppingChangeFlag is equal to 1 and (slice_type % 5) is less than 2, the reference picture list refPicList0,


- when CroppingChangeFlag is equal to 1 and (slice_type % 5) is equal to 1, the reference picture list refPicList1.





Outputs of this process are:


- a variable mbTypeILPred specifying a predictor for the macroblock type of the current macroblock,


- a list subMbTypeILPred with 4 elements specifying predictors for sub-macroblock types of the current macroblock,


- two 2x2 arrays refIdxILPredL0 and refIdxILPredL1 specifying inter-layer predictors for the reference indices of the
current macroblock,


- two 4x4x2 arrays mvILPredL0 and mvILPredL1 specifying inter-layer predictors for the motion vector components
of the current macroblock.


The derivation process for reference layer partition identifications as specified in clause G.8.6.1.1 is invoked with
fieldMbFlag, refLayerFieldMbFlag, refLayerMbType, and refLayerSubMbType as the inputs and the outputs are a variable
intraILPredFlag and, when intraILPredFlag is equal to 0, reference layer partition identifications as a 4x4 array
refLayerPartIdc with elements refLayerPartIdc[ x, y ].


When slice_type is equal to EI, the bitstream shall not contain data that result in intraILPredFlag equal to 0.


Depending on intraILPredFlag, the 2x2 arrays refIdxILPredL0 and refIdxILPredL1 and the 4x4x2 array mvILPredL0 and
mvILPredL1 are derived as follows:


- If intraILPredFlag is equal to 1, all elements of the 2x2 arrays refIdxILPredL0 and refIdxILPredL1 are set equal to −1
and all elements of the 4x4x2 arrays mvILPredL0 and mvILPredL1 are set equal to 0.


- Otherwise (intraILPredFlag is equal to 0), the derivation process for inter-layer predictors for reference indices and
motion vectors as specified in clause G.8.6.1.2 is invoked with fieldMbFlag, refLayerFieldMbFlag,
refLayerPredFlagL0, refLayerPredFlagL1, refLayerRefIdxL0, refLayerRefIdxL1, refLayerMvL0, refLayerMvL1,
refLayerPartIdc, refPicList0 (when available), and refPicList1 (when available) as the inputs and the outputs are the
arrays refIdxILPredL0, refIdxILPredL1, mvILPredL0, and mvILPredL1.


Depending on intraILPredFlag, the variable mbTypeILPred and the list subMbTypeILPred are derived as follows:


- If intraILPredFlag is equal to 1, all elements subMbTypeILPred[ mbPartIdx ] of the list subMbTypeILPred with
mbPartIdx = 0..3 are marked as unspecified, and the variable mbTypeILPred is derived as follows:


   - If tcoeff_level_prediction_flag is equal to 1, mbTypeILPred is set equal to refLayerMbType[ CurrMbAddr ].


   - Otherwise (tcoeff_level_prediction_flag is equal to 0), mbTypeILPred is set equal to I_BL.


- Otherwise (intraILPredFlag is equal to 0), the derivation process for inter-layer predictors for P and B macroblock
and sub-macroblock types as specified in clause G.8.6.1.3 is invoked with refIdxILPredL0, refIdxILPredL1,
mvILPredL0, and mvILPredL1 as the inputs and the outputs are the variable mbTypeILPred and the list
subMbTypeILPred.


**G.8.6.1.1** **Derivation process for reference layer partition identifications**


Inputs to this process are:


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a one-dimensional array refLayerFieldMbFlag with RefLayerPicSizeInMbs elements specifying which macroblocks
of the reference layer representation are field macroblocks and which macroblocks are frame macroblocks,


- a one-dimensional array refLayerMbType with RefLayerPicSizeInMbs elements specifying the macroblock types for
the macroblocks of the reference layer representation,


- a (RefLayerPicSizeInMbs)x4 array refLayerSubMbType specifying the sub-macroblock types for the macroblocks of
the reference layer representation.


Outputs of this process are:


- a variable intraILPredFlag specifying whether the samples of the current macroblock in the current layer
representation can be predicted by inter-layer intra prediction or, in the case of tcoeff_level_prediction_flag equal
to 1, by a combination of intra-layer intra prediction and inter-layer prediction,


- when intraILPredFlag is equal to 0, reference layer partition identifications for the current macroblock as a 4x4 array
refLayerPartIdc with elements refLayerPartIdc[ x, y ].


When the 4x4 array refLayerPartIdc is output of this process, each of its elements refLayerPartIdc[ x, y ] specifies the
macroblock address, the macroblock partition index, and the sub-macroblock partition index of the partition in the
reference layer representation that can be used for inter-layer motion prediction of the macroblock or sub-macroblock
partition of the current macroblock that contains the 4x4 block with coordinates x and y.





For each 4x4 block with block coordinates x, y = 0..3, the element refLayerPartIdc[ x, y ] of the 4x4 array refLayerPartIdc
is derived by applying the following ordered steps:


1. The derivation process for reference layer partitions as specified in clause G.6.2 is invoked with the luma location

( 4 * x + 1, 4 * y + 1 ), fieldMbFlag, refLayerFieldMbFlag, refLayerMbType, and refLayerSubMbType as the
inputs and the outputs are a macroblock address refMbAddr, a macroblock partition index refMbPartIdx, and a
sub-macroblock partition index refSubMbPartIdx of a partition in the reference layer representation.


2. The element refLayerPartIdc[ x, y ] of the array refLayerPartIdc is derived as follows:


     - If refLayerMbType[ refMbAddr ] is equal to I_PCM, I_16x16, I_8x8, I_4x4, or I_BL,
refLayerPartIdc[ x, y ] is set equal to −1.


     - Otherwise (refLayerMbType[ refMbAddr ] is not equal to I_PCM, I_16x16, I_8x8, I_4x4, or I_BL),
refLayerPartIdc[ x, y ] is derived by


refLayerPartIdc[ x, y ] = 16 * refMbAddr + 4 * refMbPartIdx + refSubMbPartIdx (G-211)


The variable intraILPredFlag is derived as follows:


- If all elements refLayerPartIdc[ x, y ] with x, y = 0..3 are equal to −1, intraILPredFlag is set equal to 1.


- Otherwise (any element refLayerPartIdc[ x, y ] with x, y = 0..3 is not equal to −1), intraILPredFlag is set equal to 0.


When intraILPredFlag is equal to 0 and RestrictedSpatialResolutionChangeFlag is equal to 0, the 4x4 array refLayerPartIdc
is modified by the following ordered steps:


1. For each 8x8 block with block coordinates xP, yP = 0..1, the following ordered steps are specified.


a. The variables xO and yO are set equal to (2 * xP) and (2 * yP), respectively.


b. All elements procI4x4Blk[ xS, yS ] of the 2x2 array procI4x4Blk with xS, yS = 0..1 are set equal to 0.


c. The 4x4 blocks of the current 8x8 block with block coordinates xS, yS = 0..1 are processed in increasing
order of (2 * yS + xS), and when refLayerPartIdc[ xO + xS, yO + yS ] is equal to −1 for a 4x4 block, the
element procI4x4Blk[ xS, yS ] of the array procI4x4Blk is set equal to 1 and the following applies:


        - If procI4x4Blk[ 1 − xS, yS ] is equal to 0 and refLayerPartIdc[ xO + 1 − xS, yO + yS ] is not equal
to −1, the element refLayerPartIdc[ xO + xS, yO + yS ] is modified by


refLayerPartIdc[ xO + xS, yO + yS ] = refLayerPartIdc[ xO + 1 − xS, yO + yS ] (G-212)


        - Otherwise, if procI4x4Blk[ xS, 1 − yS ] is equal to 0 and refLayerPartIdc[ xO + xS, yO + 1 − yS ] is not
equal to −1, the element refLayerPartIdc[ xO + xS, yO + yS ] is modified by


refLayerPartIdc[ xO + xS, yO + yS ] = refLayerPartIdc[ xO + xS, yO + 1 − yS ] (G-213)


        - Otherwise, if procI4x4Blk[ 1 − xS, 1 − yS ] is equal to 0 and refLayerPartIdc[ xO + 1 − xS,
yO + 1 − yS ] is not equal to −1, the element refLayerPartIdc[ xO + xS, yO + yS ] is modified by


refLayerPartIdc[ xO + xS, yO + yS ] = refLayerPartIdc[ xO + 1 − xS, yO + 1 − yS ] (G-214)


        - Otherwise, the element refLayerPartIdc[ xO + xS, yO + yS ] is not modified.


2. All elements procI8x8Blk[ xP, yP ] of the 2x2 array procI8x8Blk with xP, yP = 0..1 are set equal to 0.


3. The 8x8 blocks with block coordinates xP, yP = 0..1 are processed in increasing order of (2 * yP + xP), and when

refLayerPartIdc[ 2 * xP, 2 * yP ] is equal to −1 for an 8x8 block, the element procI8x8Blk[ xP, yP ] of the array
procI8x8Blk is set equal to 1 and the following applies:


      - If procI8x8Blk[ 1 − xP, yP ] is equal to 0 and refLayerPartIdc[ 2 − xP, 2 * yP ] is not equal to −1, the
elements refLayerPartIdc[ 2 * xP + xS, 2 * yP + yS ] with xS, yS = 0..1 are modified by


refLayerPartIdc[ 2 * xP + xS, 2 * yP + yS ] = refLayerPartIdc[ 2 − xP, 2 * yP + yS ] (G-215)


      - Otherwise, if procI8x8Blk[ xP, 1 − yP ] is equal to 0 and refLayerPartIdc[ 2 * xP, 2 − yP ] is not equal
to −1, the elements refLayerPartIdc[ 2 * xP + xS, 2 * yP + yS ] with xS, yS = 0..1 are modified by


refLayerPartIdc[ 2 * xP + xS, 2 * yP + yS ] = refLayerPartIdc[ 2 * xP + xS, 2 − yP ] (G-216)





      - Otherwise, if procI8x8Blk[ 1 − xP, 1 − yP ] is equal to 0 and refLayerPartIdc[ 2 − xP, 2 − yP ] is not equal
to −1, the elements refLayerPartIdc[ 2 * xP + xS, 2 * yP + yS ] with xS, yS = 0..1 are modified by


refLayerPartIdc[ 2 * xP + xS, 2 * yP + yS ] = refLayerPartIdc[ 2 − xP, 2 − yP ] (G-217)


      - Otherwise, the elements refLayerPartIdc[ 2 * xP + xS, 2 * yP + yS ] with xS, yS = 0..1 are not modified.
NOTE – By the process specified above the elements refLayerPartIdc[ x, y ] that are equal to −1 are replaced by elements
refLayerPartIdc[ x, y ] that are not equal to −1. This process can also be applied when RestrictedSpatialResolutionChangeFlag is
equal to 1 or intraILPredFlag is equal to 1, but in this case, the 4x4 array refLayerPartIdc is not modified.


**G.8.6.1.2** **Derivation process for inter-layer predictors for reference indices and motion vectors**


Inputs to this process are:


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a one-dimensional array refLayerFieldMbFlag with RefLayerPicSizeInMbs elements specifying which macroblocks
of the reference layer representation are field macroblocks and which macroblocks are frame macroblocks,


- two (RefLayerPicSizeInMbs)x4 arrays refLayerPredFlagL0 and refLayerPredFlagL1 specifying prediction utilization
flags for the macroblocks of the reference layer representation,


- two (RefLayerPicSizeInMbs)x4 arrays refLayerRefIdxL0 and refLayerRefIdxL1 specifying reference indices for the
macroblocks of the reference layer representation,


- two (RefLayerPicSizeInMbs)x4x4x2 arrays refLayerMvL0 and refLayerMvL1 specifying motion vector components
for the macroblocks of the reference layer representation,


- a 4x4 array refLayerPartIdc specifying reference layer partition identifications for the 4x4 blocks of the current
macroblock,


- when CroppingChangeFlag is equal to 1 and (slice_type % 5) is less than 2, the reference picture list refPicList0,


- when CroppingChangeFlag is equal to 1 and (slice_type % 5) is equal to 1, the reference picture list refPicList1.


Outputs of this process are:


- two 2x2 arrays refIdxILPredL0 and refIdxILPredL1 specifying inter-layer predictors for the reference indices of the
current macroblock,


- two 4x4x2 arrays mvILPredL0 and mvILPredL1 specifying inter-layer predictors for the motion vector components
of the current macroblock.


Let tempRefIdxPredL0 and tempRefIdxPredL1 be two 4x4 arrays with elements tempRefIdxPredL0[ x, y ] and
tempRefIdxPredL1[ x, y ], respectively, that specify auxiliary inter-layer predictors for reference indices.


For each 4x4 block indexed by x, y = 0..3 and for X being replaced by 0 and 1, the auxiliary reference index predictor
tempRefIdxPredLX[ x, y ] and the motion vector predictor mvILPredLX[ x, y ] are derived as follows:


- If refLayerPredFlagLX[ refLayerPartIdc[ x, y ] / 16 ][ ( refLayerPartIdc[ x, y ] % 16 ) / 4 ] is equal 0, the reference
index predictor tempRefIdxPredLX[ x, y ] and the motion vector predictor mvILPredLX[ x, y ] are derived by:


tempRefIdxPredLX[ x, y ] = −1 (G-218)
mvILPredLX[ x, y ][ 0 ]  = 0 (G-219)
mvILPredLX[ x, y ][ 1 ]  = 0 (G-220)


- Otherwise (refLayerPredFlagLX[ refLayerPartIdc[ x, y ] / 16 ][ ( refLayerPartIdc[ x, y ] % 16 ) / 4 ] is equal to 1),
the following ordered steps are specified:


1. The variables refMbAddr, refMbPartIdx, and refSubMbPartIdx are derived by


refMbAddr     =  refLayerPartIdc[ x, y ] / 16 (G-221)
refMbPartIdx   = ( refLayerPartIdc[ x, y ] % 16 ) / 4 (G-222)
refSubMbPartIdx = refLayerPartIdc[ x, y ] % 4 (G-223)


2. The auxiliary reference index predictor tempRefIdxPredLX[ x, y ] is derived by:


tempRefIdxPredLX[ x, y ] = refLayerRefIdxLX[ refMbAddr ][ refMbPartIdx ]
          - ( 1 + fieldMbFlag − field_pic_flag ) (G-224)
/ ( 1 + refLayerFieldMbFlag[ refMbAddr ] − RefLayerFieldPicFlag )





3. The motion vector aMv is set equal to refLayerMvLX[ refMbAddr ][ refMbPartIdx ][ refSubMbPartIdx ], and

afterwards its vertical component aMv[ 1 ] is modified by:


aMv[ 1 ] = aMv[ 1 ] * ( 1 + refLayerFieldMbFlag[ refMbAddr ] ) (G-225)


4. The variables scaledW, scaledH, refLayerW, and refLayerH are derived by:


scaledW   = ScaledRefLayerPicWidthInSamplesL (G-226)
scaledH   = ScaledRefLayerPicHeightInSamplesL * ( 1 + field_pic_flag ) (G-227)
refLayerW = RefLayerPicWidthInSamplesL (G-228)
refLayerH = RefLayerPicHeightInSamplesL * ( 1 + RefLayerFieldPicFlag ) (G-229)


5. The variables dOX, dOY, dSW, and dSH are derived as follows:


   - If CroppingChangeFlag is equal to 0 or the reference picture refPicListX[ tempRefIdxPredLX[ x, y ] ] is not
available, dOX, dOY, dSW, and dSH are set equal to 0.


   - Otherwise (CroppingChangeFlag is equal to 1 and the reference picture
refPicListX[ tempRefIdxPredLX[ x, y ] ] is available), the variables refPicScaledRefLayerLeftOffset,
refPicScaledRefLayerRightOffset, refPicScaledRefLayerTopOffset, and
refPicScaledRefLayerBottomOffset are set equal to the variables ScaledRefLayerLeftOffset,
ScaledRefLayerRightOffset, ScaledRefLayerTopOffset, and ScaledRefLayerBottomOffset, respectively,
that are associated with the layer representation of the reference picture
refPicListX[ tempRefIdxPredLX[ x, y ] ] that has the same value of DQId as the current layer representation,
and the variables dOX, dOY, dSW, and dSH are derived by:


dOX = ScaledRefLayerLeftOffset   − refPicScaledRefLayerLeftOffset (G-230)
dOY = ScaledRefLayerTopOffset   − refPicScaledRefLayerTopOffset (G-231)
dSW = ScaledRefLayerRightOffset  − refPicScaledRefLayerRightOffset  + dOX (G-232)
dSH = ScaledRefLayerBottomOffset − refPicScaledRefLayerBottomOffset + dOY (G-233)


6. The variables scaleX and scaleY are derived by:


scaleX = ( ( ( scaledW + dSW ) << 16 ) + ( refLayerW >> 1 ) ) / refLayerW (G-234)
scaleY = ( ( ( scaledH + dSH ) << 16 ) + ( refLayerH >> 1 ) ) / refLayerH (G-235)


7. The motion vector aMv is scaled by:


aMv[ 0 ] = ( aMv[ 0 ] * scaleX + 32768 ) >> 16 (G-236)
aMv[ 1 ] = ( aMv[ 1 ] * scaleY + 32768 ) >> 16 (G-237)


8. When CroppingChangeFlag is equal to 1, the motion vector aMv is modified by applying the following ordered

steps:


a. The inverse macroblock scanning process as specified in clause 6.4.1 is invoked with CurrMbAddr as the
input and the output is a luma location ( xMbPic, yMbPic ). For this invocation of the process specified in
clause 6.4.1, the current macroblock is treated as field macroblock when fieldMbFlag is equal to 1, and it is
treated as frame macroblock when fieldMbFlag is equal to 0.


b. The luma location ( xFrm, yFrm ) is derived by:


xFrm = ( xMbPic + ( 4 * x + 1 ) ) (G-238)
yFrm = ( yMbPic + ( 4 * y + 1 ) * ( 1 + fieldMbFlag − field_pic_flag ) ) * ( 1 + field_pic_flag ) (G-239)


c. The variables scaleX and scaleY are modified by:


scaleX = ( ( ( 4 * dSW ) << 16 ) + ( scaledW >> 1 ) ) / scaledW (G-240)
scaleY = ( ( ( 4 * dSH ) << 16 ) + ( scaledH >> 1 ) ) / scaledH (G-241)





d. The motion vector aMv is modified by:


aMv[ 0 ] += ( ( ( xFrm − ScaledRefLayerLeftOffset ) * scaleX + 32768 ) >> 16 ) − 4 * dOX (G-242)
aMv[ 1 ] += ( ( ( yFrm − ScaledRefLayerTopOffset ) * scaleY + 32768 ) >> 16 ) − 4 * dOY (G-243)


9. The motion vector predictor mvILPredLX[ x, y ] is derived by:


mvILPredLX[ x, y ][ 0 ] = aMv[ 0 ] (G-244)
mvILPredLX[ x, y ][ 1 ] = aMv[ 1 ] / ( 1 + fieldMbFlag ) (G-245)


For each 8x8 block indexed by xP, yP = 0..1 and for X being replaced by 0 or 1, the reference index predictor
refIdxILPredLX[ xP, yP ] is set equal to tempRefIdxPredLX[ 2 * xP, 2 * yP ], and when
RestrictedSpatialResolutionChangeFlag is equal to 0, the following ordered steps are specified:


1. The 4x4 blocks indexed by xS, yS = 0..1 of the current 8x8 block are processed in increasing order of

(2 * yS + xS), and for each 4x4 block, the reference index predictor refIdxILPredLX[ xP, yP ] is modified by:


refIdxILPredLX[ xP, yP ] = MinPositive( refIdxILPredLX[ xP, yP ],
tempRefIdxPredLX[ 2 * xP + xS, 2 * yP + yS ] ) (G-246)


with


 Min(a, b ) if a = 0 and b = 0

MinPositive( a, ) b =  (G-247)

 Max( a, b ) otherwise


2. The 4x4 blocks indexed by xS, yS = 0..1 of the current 8x8 block are processed in increasing order of

(2 * yS + xS), and for each 4x4 block, when tempRefIdxPredLX[ 2 * xP + xS, 2 * yP + yS ] is not equal to the
reference index predictor refIdxILPredLX[ xP, yP ], the following applies:


      - If tempRefIdxPredLX[ 2 * xP + 1 − xS, 2 * yP + yS ] is equal to refIdxILPredLX[ xP, yP ], the motion
vector predictor mvILPredLX[ 2 * xP + xS, 2 * yP + yS ] is modified by:


mvILPredLX[ 2 * xP + xS, 2 * yP + yS ] = mvILPredLX[ 2 * xP + 1 − xS, 2 * yP + yS ] (G-248)


      - Otherwise, if tempRefIdxPredLX[ 2 * xP + xS, 2 * yP + 1 − yS ] is equal to refIdxILPredLX[ xP, yP ], the
motion vector predictor mvILPredLX[ 2 * xP + xS, 2 * yP + yS ] is modified by:


mvILPredLX[ 2 * xP + xS, 2 * yP + yS ] = mvILPredLX[ 2 * xP + xS, 2 * yP + 1 − yS ] (G-249)


      - Otherwise (tempRefIdxPredLX[ 2 * xP + 1 − xS, 2 * yP + 1 − yS ] is equal to refIdxILPredLX[ xP, yP ]),
the motion vector predictor mvILPredLX[ 2 * xP + xS, 2 * yP + yS ] is modified by:


mvILPredLX[ 2 * xP + xS, 2 * yP + yS ] = mvILPredLX[ 2 * xP + 1 − xS, 2 * yP + 1 − yS ] (G-250)


NOTE – The process specified above can also be applied when RestrictedSpatialResolutionChangeFlag is equal to 1, but in this
case, the reference index predictor refIdxILPredLX[ xP, yP ] and the motion vector predictors
mvILPredLX[ 2 * xP + xS, 2 * yP + yS ] with xS, yS = 0..1 will not be modified.


When RestrictedSpatialResolutionChangeFlag is equal to 0, slice_type is equal to EB, and direct_8x8_inference_flag is
equal to 1, for each 8x8 block indexed by xP, yP = 0..1 and for X being replaced by 0 or 1, the following ordered steps are
specified:


1. The motion vector tempMv with components tempMv[ 0 ] and tempMv[ 1 ] is derived by:


tempMv[ c ] = mvILPredX[ 3 * xP, 3 * yP ][ c ]   with c = 0..1 (G-251)


2. The array mvILPredLX is modified by:


mvILPredLX[ 2 * xP + xS, 2 * yP + yS ][ c ] = tempMv[ c ]  with xS, yS = 0..1 and c = 0..1 (G-252)


When RestrictedSpatialResolutionChangeFlag is equal to 0, for each 8x8 block indexed by xP, yP = 0..1, the motion vector
predictor arrays mvILPredL0 and mvILPredL1 are modified by applying the following ordered steps:


1. The variable maxX is derived as follows:


     - If slice_type is equal to EB, maxX is set equal to 1.





     - Otherwise (slice_type is equal to EP), maxX is set equal to 0.


2. The variables xO and yO are set equal to (2 * xP) and (2 * yP), respectively.


3. The function mvDiff( mv1, mv2 ) of two motion vectors mv1 and mv2 is defined by


mvDiff( mv1, mv2 ) = Abs( mv1[ 0 ] − mv2[ 0 ] ) + Abs( mv1[ 1 ] − mv2[ 1 ] ) (G-253)


4. The variable subPartSize is derived as follows:


      - If for X = 0..maxX, all of the following conditions are true, subPartSize is set equal to 8x8.


        - mvDiff( mvILPredLX[ xO, yO ], mvILPredLX[ xO + 1, yO ] ) is less than or equal to 1


        - mvDiff( mvILPredLX[ xO, yO ], mvILPredLX[ xO, yO + 1 ] ) is less than or equal to 1


        - mvDiff( mvILPredLX[ xO, yO ], mvILPredLX[ xO + 1, yO + 1 ] ) is less than or equal to 1


      - Otherwise, if for X = 0..maxX, all of the following conditions are true, subPartSize is set equal to 8x4.


        - mvDiff( mvILPredLX[ xO, yO ], mvILPredLX[ xO + 1, yO ] ) is less than or equal to 1


        - mvDiff( mvILPredLX[ xO, yO + 1 ], mvILPredLX[ xO + 1, yO + 1 ] ) is less than or equal to 1


      - Otherwise, if for X = 0..maxX, all of the following conditions are true, subPartSize is set equal to 4x8.


        - mvDiff( mvILPredLX[ xO, yO ], mvILPredLX[ xO, yO + 1 ] ) is less than or equal to 1


        - mvDiff( mvILPredLX[ xO + 1, yO ], mvILPredLX[ xO + 1, yO + 1 ] ) is less than or equal to 1


      - Otherwise, subPartSize is set equal to 4x4.


5. When subPartSize is not equal to 4x4, for X = 0..maxX, the motion vectors tempMvALX and tempMvBLX (when

subPartSize is equal to 8x4 or 4x8) are derived as follows:


      - If subPartSize is equal to 8x8, tempMvALX is derived by


tempMvALX[ c ] = ( mvILPredLX[ xO,    yO    ][ c ] +
mvILPredLX[ xO + 1, yO    ][ c ] +
mvILPredLX[ xO,    yO + 1 ][ c ] +
mvILPredLX[ xO + 1, yO + 1 ][ c ] + 2 ) >> 2   with c = 0..1 (G-254)


      - Otherwise, if subPartSize is equal to 8x4, tempMvALX and tempMvBLX are derived by


tempMvALX[ c ] = ( mvILPredLX[ xO,    yO ][ c ] +
mvILPredLX[ xO + 1, yO ][ c ] + 1 ) >> 1      with c = 0..1 (G-255)

tempMvBLX[ c ] = ( mvILPredLX[ xO,    yO + 1 ][ c ] +
mvILPredLX[ xO + 1, yO + 1 ][ c ] + 1 ) >> 1   with c = 0..1 (G-256)


      - Otherwise (subPartSize is equal to 4x8), tempMvALX and tempMvBLX are derived by


tempMvALX[ c ] = ( mvILPredLX[ xO, yO    ][ c ] +
mvILPredLX[ xO, yO + 1 ][ c ] + 1 ) >> 1      with c = 0..1 (G-257)

tempMvBLX[ c ] = ( mvILPredLX[ xO + 1, yO    ][ c ] +
mvILPredLX[ xO + 1, yO + 1 ][ c ] + 1 ) >> 1   with c = 0..1 (G-258)


6. When subPartSize is not equal to 4x4, for X = 0..maxX, the motion vector predictor array mvILPredLX is modified

as follows:


      - If subPartSize is equal to 8x8, the array mvILPredLX is modified by


mvILPredLX[ xO + xS, yO + yS ][ c ] = tempMvALX[ c ]  with xS, yS = 0..1 and c = 0..1 (G-259)


      - Otherwise, if subPartSize is equal to 8x4, the array mvILPredLX is modified by


mvILPredLX[ xO + xS, yO    ][ c ] = tempMvALX[ c ]    with xS = 0..1 and c = 0..1 (G-260)
mvILPredLX[ xO + xS, yO + 1 ][ c ] = tempMvBLX[ c ]    with xS = 0..1 and c = 0..1 (G-261)





      - Otherwise (subPartSize is equal to 4x8), the array mvILPredLX is modified by


mvILPredLX[ xO,   yO + yS ][ c ] = tempMvALX[ c ]    with yS = 0..1 and c = 0..1 (G-262)
mvILPredLX[ xO + 1, yO + yS ][ c ] = tempMvBLX[ c ]    with yS = 0..1 and c = 0..1 (G-263)


**G.8.6.1.3** **Derivation process for inter-layer predictors for P and B macroblock and sub-macroblock types**


This process is only invoked when slice_type is equal to EP or EB.


Inputs to this process are:


- two 2x2 arrays refIdxILPredL0 and refIdxILPredL1 specifying predictors for the reference indices of the current
macroblock,


- two 4x4x2 arrays mvILPredL0 and mvILPredL1 specifying predictors for the motion vectors of the current
macroblock.


Outputs of this process are:


- a variable mbTypeILPred specifying a predictor for the macroblock type of the current macroblock,


- a list subMbTypeILPred with 4 elements specifying predictors for sub-macroblock types of the current macroblock.


The variable maxX is derived as follows:


- If slice_type is equal to EB, maxX is set equal to 1.


- Otherwise (slice_type is equal to EP), maxX is set equal to 0.


The macroblock type predictor mbTypeILPred is derived by applying the following ordered steps:


1. The variable partitionSize is derived as follows:


     - If for X = 0..maxX, all of the following conditions are true, partitionSize is set equal to 16x16.


        - all elements refIdxILPredLX[ x, y ] with x, y = 0..1 are the same


        - all elements mvILPredLX[ x, y ] with x, y = 0..3 are the same


     - Otherwise, if for X = 0..maxX, all of the following conditions are true, partitionSize is set equal to 16x8.


        - refIdxILPredLX[ 0, 0 ] is equal to refIdxILPredLX[ 1, 0 ]


        - refIdxILPredLX[ 0, 1 ] is equal to refIdxILPredLX[ 1, 1 ]


        - all elements mvILPredLX[ x, y ] with x = 0..3 and y = 0..1 are the same


        - all elements mvILPredLX[ x, y ] with x = 0..3 and y = 2..3 are the same


     - Otherwise, if for X = 0..maxX, all of the following conditions are true, partitionSize is set equal to 8x16.


        - refIdxILPredLX[ 0, 0 ] is equal to refIdxILPredLX[ 0, 1 ]


        - refIdxILPredLX[ 1, 0 ] is equal to refIdxILPredLX[ 1, 1 ]


        - all elements mvILPredLX[ x, y ] with x = 0..1 and y = 0..3 are the same


        - all elements mvILPredLX[ x, y ] with x = 2..3 and y = 0..3 are the same


     - Otherwise, partitionSize is set equal to 8x8.


2. When slice_type is equal to EB and partitionSize is not equal to 8x8, the variable partPredModeA is derived by


partPredModeA = ( ( refIdxILPredL1[ 0, 0 ] >= 0 ) ? 2 : 0 ) +
( ( refIdxILPredL0[ 0, 0 ] >= 0 ) ? 1 : 0 ) (G-264)


3. When slice_type is equal to EB and partitionSize is equal to 16x8 or 8x16, the variable partPredModeB is derived

by


partPredModeB = ( ( refIdxILPredL1[ 1, 1 ] >= 0 ) ? 2 : 0 ) +
( ( refIdxILPredL0[ 1, 1 ] >= 0 ) ? 1 : 0 ) (G-265)


4. Depending on slice_type, partitionSize, partPredModeA (when applicable), and partPredModeB (when

applicable), the macroblock type predictor mbTypeILPred is derived as specified in Table G-7.





All elements subMbTypeILPred[ mbPartIdx ] of the list subMbTypeILPred with mbPartIdx = 0..3 are marked as
"unspecified".


When mbTypeILPred is equal to P_8x8 or B_8x8, each element subMbTypeILPred[ mbPartIdx ] with mbPartIdx = 0..3 is
modified by applying the following ordered steps:


1. The coordinate offset ( xO, yO ) is set equal to ( 2 * ( mbPartIdx % 2 ), 2 * ( mbPartIdx / 2 ) ).


2. The variable subPartitionSize is derived as follows:


     - If for X = 0..maxX, all elements mvILPredLX[ xO + xS, yO + yS ] with xS, yS = 0..1 are the same,
subPartitionSize is set equal to 8x8.


     - Otherwise, if for X = 0..maxX, mvILPredLX[ xO, yO ] is equal to mvILPredLX[ xO + 1, yO ] and
mvILPredLX[ xO, yO + 1 ] is equal to mvILPredLX[ xO + 1, yO + 1 ], subPartitionSize is set equal to 8x4.


     - Otherwise, if for X = 0..maxX, mvILPredLX[ xO, yO ] is equal to mvILPredLX[ xO, yO + 1 ] and
mvILPredLX[ xO + 1, yO ] is equal to mvILPredLX[ xO + 1, yO + 1 ], subPartitionSize is set equal to 4x8.


     - Otherwise, subPartitionSize is set equal to 4x4.


3. When slice_type is equal to EB, the variable partPredMode is derived by


partPredMode = ( ( refIdxILPredL1[ xO / 2, yO / 2 ] >= 0 ) ? 2 : 0 ) +
( ( refIdxILPredL0[ xO / 2, yO / 2 ] >= 0 ) ? 1 : 0 ) (G-266)


4. Depending on slice_type, subPartitionSize, and partPredMode (when applicable), the sub-macroblock type

predictor subMbTypeILPred[ mbPartIdx ] is derived as specified in Table G-8.


**Table G-7 – Macroblock type predictors mbTypeILPred**

|slice_type|partitionSize|partPredModeA|partPredModeB|mbTypeILPred|slice_type|partitionSize|partPredModeA|partPredModeB|mbTypeILPred|
|---|---|---|---|---|---|---|---|---|---|
|EB|16x16|1|na|B_L0_16x16|EB|16x8|2|3|B_L1_Bi_16x8|
|EB|16x16|2|na|B_L1_16x16|EB|8x16|2|3|B_L1_Bi_8x16|
|EB|16x16|3|na|B_Bi_16x16|EB|16x8|3|1|B_Bi_L0_16x8|
|EB|16x8|1|1|B_L0_L0_16x8|EB|8x16|3|1|B_Bi_L0_8x16|
|EB|8x16|1|1|B_L0_L0_8x16|EB|16x8|3|2|B_Bi_L1_16x8|
|EB|16x8|2|2|B_L1_L1_16x8|EB|8x16|3|2|B_Bi_L1_8x16|
|EB|8x16|2|2|B_L1_L1_8x16|EB|16x8|3|3|B_Bi_Bi_16x8|
|EB|16x8|1|2|B_L0_L1_16x8|EB|8x16|3|3|B_Bi_Bi_8x16|
|EB|8x16|1|2|B_L0_L1_8x16|EB|8x8|na|na|B_8x8|
|EB|16x8|2|1|B_L1_L0_16x8|EP|16x16|na|na|P_L0_16x16|
|EB|8x16|2|1|B_L1_L0_8x16|EP|16x8|na|na|P_L0_L0_16x8|
|EB|16x8|1|3|B_L0_Bi_16x8|EP|8x16|na|na|P_L0_L0_8x16|
|EB|8x16|1|3|B_L0_Bi_8x16|EP|8x8|na|na|P_8x8|






**Table G-8 – Sub-macroblock type predictors subMbTypeILPred[ mbPartIdx ]**

|slice_type|subPartitionSize|partPredMode|subMbTypeILPred [ mbPartIdx ]|slice_type|subPartitionSize|partPredMode|subMbTypeILPred [ mbPartIdx ]|
|---|---|---|---|---|---|---|---|
|EB|8x8|1|B_L0_8x8|EB|4x8|3|B_Bi_4x8|
|EB|8x8|2|B_L1_8x8|EB|4x4|1|B_L0_4x4|
|EB|8x8|3|B_Bi_8x8|EB|4x4|2|B_L1_4x4|
|EB|8x4|1|B_L0_8x4|EB|4x4|3|B_Bi_4x4|
|EB|8x4|2|B_L1_8x4|EP|8x8|na|P_L0_8x8|
|EB|8x4|3|B_Bi_8x4|EP|8x4|na|P_L0_8x4|
|EB|4x8|1|B_L0_4x8|EP|4x8|na|P_L0_4x8|
|EB|4x8|2|B_L1_4x8|EP|4x4|na|P_L0_4x4|



**G.8.6.2** **Resampling process for intra samples**


Inputs to this process are:


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a one-dimensional array refLayerSliceIdc with RefLayerPicSizeInMbs elements specifying slice identifications for
the macroblocks of the reference layer representation,


- a one-dimensional array refLayerFieldMbFlag with RefLayerPicSizeInMbs elements specifying which macroblocks
of the reference layer representation are field macroblocks and which macroblocks are frame macroblocks,


- a one-dimensional array refLayerMbType with RefLayerPicSizeInMbs elements specifying macroblock types for the
macroblocks of the reference layer representation,


- a (RefLayerPicWidthInSamplesL)x(RefLayerPicHeightInSamplesL) array refLayerPicSamplesL of luma samples for
the reference layer representation,


- a (PicWidthInSamplesL)x(PicHeightInSamplesL) array picSamplesL of luma samples,


- when ChromaArrayType is not equal to 0, two (RefLayerPicWidthInSamplesC)x(RefLayerPicHeightInSamplesC)
arrays refLayerPicSamplesCb and refLayerPicSamplesCr of chroma samples for the reference layer representation,


- when ChromaArrayType is not equal to 0, two (PicWidthInSamplesC)x(PicHeightInSamplesC) arrays picSamplesCb
and picSamplesCr of chroma samples.


Outputs of this process are:


- a modified version of the array picSamplesL of luma samples,


- when ChromaArrayType is not equal to 0, modified versions of the arrays picSamplesCb and picSamplesCr of chroma
samples.


The resampling process for intra samples of a macroblock colour component as specified in clause G.8.6.2.1 is invoked
with chromaFlag equal to 0, mbW equal to 16, mbH equal to 16, fieldMbFlag, refLayerPicSamplesL, refLayerSliceIdc,
refLayerFieldMbFlag, and refLayerMbType as the inputs and the output is the 16x16 array mbPredL of Intra_Base
prediction samples for the luma component of the current macroblock.


When ChromaArrayType is not equal to 0, for CX being replaced by Cb and Cr, the resampling process for intra samples
of a macroblock colour component as specified in clause G.8.6.2.1 is invoked with chromaFlag equal to 1, mbW equal to
MbWidthC, mbH equal to MbHeightC, fieldMbFlag, refLayerPicSamplesCX, refLayerSliceIdc, refLayerFieldMbFlag, and
refLayerMbType as the inputs and the output is the (MbWidthC)x(MbHeightC) array mbPredCX of Intra_Base prediction
samples for the CX component of the current macroblock.





The picture sample array construction process as specified in clause G.8.5.4.1 is invoked with fieldMbFlag, mbPredL,
picSamplesL and, when ChromaArrayType is not equal to 0, mbPredCb, mbPredCr, picSamplesCb, and picSamplesCr as the
inputs and the outputs are a modified version of picSamplesL and, when ChromaArrayType is not equal to 0, modified
versions of picSamplesCb, and picSamplesCr.


**G.8.6.2.1** **Resampling process for intra samples of a macroblock colour component**


Inputs to this process are:


- a variable chromaFlag specifying whether the luma or a chroma component is subject to the resampling process,


- two variables mbW and mbH specifying the width and height, respectively, of a macroblock for the considered colour
component,


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- an array refLayerPicSamples, which is a (RefLayerPicWidthInSamplesL)x(RefLayerPicHeightInSamplesL) array
containing constructed intra luma sample values for the reference layer representation when chromaFlag is equal to 0
or a (RefLayerPicWidthInSamplesC)x(RefLayerPicHeightInSamplesC) array containing constructed intra chroma
sample values for the reference layer representation when chromaFlag is equal to 1,


- a one-dimensional array refLayerSliceIdc with RefLayerPicSizeInMbs elements specifying slice identifications for
the macroblocks of the reference layer representation,


- a one-dimensional array refLayerFieldMbFlag with RefLayerPicSizeInMbs elements specifying which macroblocks
of the reference layer representation are field macroblocks and which macroblocks are frame macroblocks,


- a one-dimensional array refLayerMbType with RefLayerPicSizeInMbs elements specifying macroblock types for the
macroblocks of the reference layer representation.


Output of this process is an (mbW)x(mbH) array mbPred of Intra_Base prediction samples.


The variable botFieldFlag is derived as follows:


- If RefLayerFrameMbsOnlyFlag is equal to 1, botFieldFlag is set equal to 0.


- Otherwise, if field_pic_flag is equal to 1, botFieldFlag is set equal to bottom_field_flag.


- Otherwise, if RefLayerFieldPicFlag is equal to 1, botFieldFlag is set equal to RefLayerBottomFieldFlag.


- Otherwise, if fieldMbFlag is equal to 1, botFieldFlag is set equal to (CurrMbAddr % 2).


- Otherwise, botFieldFlag is set equal to 0.


The variable frameBasedResamplingFlag is derived as follows:


- If all of the following conditions are true, frameBasedResamplingFlag is set equal to 1:


   - RefLayerFrameMbsOnlyFlag is equal to 1,


   - frame_mbs_only_flag is equal to 1.


- Otherwise, frameBasedResamplingFlag is set equal to 0.


The variable topAndBotResamplingFlag is derived as follows:


- If all of the following conditions are true, topAndBotResamplingFlag is set equal to 1:


   - RefLayerFrameMbsOnlyFlag is equal to 0,


   - RefLayerFieldPicFlag is equal to 0,


   - frame_mbs_only_flag is equal to 0,


   - fieldMbFlag is equal to 0.


- Otherwise, topAndBotResamplingFlag is set equal to 0.


The variable botFieldFrameMbsOnlyRefFlag is derived as follows:


- If RefLayerFrameMbsOnlyFlag is equal to 1, fieldMbFlag is equal to 1, and any of the following conditions are true,
botFieldFrameMbsOnlyRefFlag is set equal to 1:


   - field_pic_flag is equal to 1 and bottom_field_flag is equal to 1,


   - field_pic_flag is equal to 0 and ( CurrMbAddr % 2 ) is equal to 1,





- Otherwise, botFieldFrameMbsOnlyRefFlag is set equal to 0.


The variable filteringModeFlag is derived as follows:


- If chromaFlag is equal to 0 or ChromaArrayType is equal to 3, filteringModeFlag is set equal to 0.


- Otherwise (chromaFlag is equal to 1 and ChromaArrayType is not equal to 3), filteringModeFlag is set equal to 1.


The array predArray is derived as specified in the following.


- If botFieldFrameMbsOnlyRefFlag is equal to 1, the following ordered steps are specified:


1. The reference layer sample array construction process prior to intra resampling as specified in clause G.8.6.2.2

is invoked with chromaFlag, mbW, mbH, fieldMbFlag, botFieldFlag, refLayerPicSamples, refLayerSliceIdc,
refLayerFieldMbFlag, and refLayerMbType as the inputs and the outputs are the variables refArrayW,
refArrayH, the array refSampleArray of reference layer sample values, and the variables xOffset and yOffset.


2. The variable yBorder is set equal to ( 2 − chromaFlag ).


3. The interpolation process for Intra_Base prediction as specified in clause G.8.6.2.3 is invoked with

filteringModeFlag, chromaFlag, mbW, mbH, fieldMbFlag, botFieldFlag, fldPrdInFrmMbFlag equal to 0,
yBorder, refArrayW, refArrayH, refSampleArray, xOffset, and yOffset as the inputs and the output is the
(mbW)x(mbH + 2 * yBorder) array topFldPredArray of top field prediction samples.


4. The vertical interpolation process for Intra_Base prediction as specified in clause G.8.6.2.4 is invoked with

filteringModeFlag, chromaFlag, mbW, mbH, botFieldFlag, yBorder, frameMbFlag equal to 0, and
topFldPredArray as the inputs and the output is the (mbW)x(mbH) array mbPred of Intra_Base prediction
samples.


- Otherwise, if frameBasedResamplingFlag is equal to 1 or fieldMbFlag is equal to 1, the following ordered steps are
specified:


1. The reference layer sample array construction process prior to intra resampling as specified in clause G.8.6.2.2

is invoked with chromaFlag, mbW, mbH, fieldMbFlag, botFieldFlag, refLayerPicSamples, refLayerSliceIdc,
refLayerFieldMbFlag, and refLayerMbType as the inputs and the outputs are the variables refArrayW,
refArrayH, the array refSampleArray of reference layer sample values, and the variables xOffset and yOffset.


2. The interpolation process for Intra_Base prediction as specified in clause G.8.6.2.3 is invoked with

filteringModeFlag, chromaFlag, mbW, mbH, fieldMbFlag, botFieldFlag, fldPrdInFrmMbFlag equal to 0,
yBorder equal to 0, refArrayW, refArrayH, refSampleArray, xOffset, and yOffset as the inputs and the output is
the (mbW)x(mbH) array mbPred of Intra_Base prediction samples.


- Otherwise, if topAndBotResamplingFlag is equal to 0, the following ordered steps are specified:


1. The reference layer sample array construction process prior to intra resampling as specified in clause G.8.6.2.2

is invoked with chromaFlag, mbW, mbH, fieldMbFlag, botFieldFlag, refLayerPicSamples, refLayerSliceIdc,
refLayerFieldMbFlag, and refLayerMbType as the inputs and the outputs are the variables refArrayW,
refArrayH, the array refSampleArray of reference layer sample values, and the variables xOffset and yOffset.


2. The variable yBorder is set equal to ( 2 − chromaFlag ).


3. The interpolation process for Intra_Base prediction as specified in clause G.8.6.2.3 is invoked with

filteringModeFlag, chromaFlag, mbW, mbH, fieldMbFlag, botFieldFlag, fldPrdInFrmMbFlag equal to 1,
yBorder, refArrayW, refArrayH, refSampleArray, xOffset, and yOffset as the inputs and the output is the
(mbW)x(mbH / 2 + 2* yBorder) array fieldPredArray of field prediction samples.


4. The vertical interpolation process for Intra_Base prediction as specified in clause G.8.6.2.4 is invoked with

filteringModeFlag, chromaFlag, mbW, mbH, botFieldFlag, yBorder, frameMbFlag equal to 1, and
fieldPredArray as the inputs and the output is the (mbW)x(mbH) array mbPred of Intra_Base prediction samples.


- Otherwise (topAndBotResamplingFlag is equal to 1), the following ordered steps are specified:


1. The reference layer sample array construction process prior to intra resampling as specified in clause G.8.6.2.2

is invoked with chromaFlag, mbW, mbH, fieldMbFlag, botFieldFlag equal to 0, refLayerPicSamples,
refLayerSliceIdc, refLayerFieldMbFlag, and refLayerMbType as the inputs and the outputs are the variables
refArrayTopW, refArrayTopH, the array refSampleArrayTop of top field reference layer sample values, and the
variables xOffsetTop and yOffsetTop.


2. The interpolation process for Intra_Base prediction as specified in clause G.8.6.2.3 is invoked with

filteringModeFlag, chromaFlag, mbW, mbH, fieldMbFlag, botFieldFlag equal to 0, fldPrdInFrmMbFlag equal





to 1, yBorder equal to 0, refArrayTopW, refArrayTopH, refSampleArrayTop, xOffsetTop, and yOffsetTop as the
inputs and the output is the (mbW)x(mbH / 2) array topFieldPredArray of top field prediction samples.


3. The reference layer sample array construction process prior to intra resampling as specified in clause G.8.6.2.2

is invoked with chromaFlag, mbW, mbH, fieldMbFlag, botFieldFlag equal to 1, refLayerPicSamples,
refLayerSliceIdc, refLayerFieldMbFlag, and refLayerMbType as the inputs and the outputs are the variables
refArrayBotW, refArrayBotH, the array refSampleArrayBot of bottom field reference layer sample values, and
the variables xOffsetBot and yOffsetBot.


4. The interpolation process for Intra_Base prediction as specified in clause G.8.6.2.3 is invoked with

filteringModeFlag, chromaFlag, mbW, mbH, fieldMbFlag, botFieldFlag equal to 1, fldPrdInFrmMbFlag equal
to 1, yBorder equal to 0, refArrayBotW, refArrayBotH, refSampleArrayBot, xOffsetBot, and yOffsetBot as the
inputs and the output is the (mbW)x(mbH / 2) array botFieldPredArray of bottom field prediction samples.


5. Each sample predArray[ x, y ] with x = 0..(mbW − 1) and y = 0..(mbH − 1) of the array mbPred of Intra_Base

prediction samples is derived by:


mbPred[ x, y ] = ( ( ( y % 2 ) = = 0 ) ? topFieldPredArray[ x, y >> 1 ]
: botFieldPredArray[ x, y >> 1 ] ) (G-267)


**G.8.6.2.2** **Reference layer sample array construction process prior to intra resampling**


Inputs to this process are:


- a variable chromaFlag specifying whether the luma or a chroma component is subject to the resampling process,


- two variables mbW and mbH specifying the width and height, respectively, of a macroblock for the considered colour
component,


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a variable botFieldFlag specifying whether a top or a bottom field is subject to the resampling process (when
RefLayerFrameMbsOnlyFlag is equal to 0 or frame_mbs_only_flag is equal to 0),


- an array refLayerPicSamples, which is a (RefLayerPicWidthInSamplesL)x(RefLayerPicHeightInSamplesL) array
containing constructed intra luma sample values for the reference layer representation when chromaFlag is equal to 0
or a (RefLayerPicWidthInSamplesC)x(RefLayerPicHeightInSamplesC) array containing constructed intra chroma
sample values for the reference layer representation when chromaFlag is equal to 1,


- a one-dimensional array refLayerSliceIdc with RefLayerPicSizeInMbs elements specifying slice identifications for
the macroblocks of the reference layer representation,


- a one-dimensional array refLayerFieldMbFlag with RefLayerPicSizeInMbs elements specifying which macroblocks
of the reference layer representation are field macroblocks and which macroblocks are frame macroblocks,


- a one-dimensional array refLayerMbType with RefLayerPicSizeInMbs elements specifying macroblock types for the
macroblocks of the reference layer representation.


Outputs of this process are:


- two variables refArrayW and refArrayH specifying the width and height, respectively, of the constructed array of
reference layer sample values,


- a (refArrayW)x(refArrayH) array refSampleArray of reference layer sample values,


- two variables xOffset and yOffset specifying the x and y coordinate, respectively, of the reference layer sample
location that corresponds to the sample refSampleArray[ 0, 0 ] of the array refSampleArray.


The variables refW, refH, refMbW, refMbH, xOffset, yOffset, refArrayW, refArrayH, xMin, yMin, xMax, yMax,
yRefScale, and yRefAdd are derived as specified in the following ordered steps:


1. The derivation process for reference layer sample locations in resampling as specified in clause G.6.3 is invoked

with chromaFlag, the sample location ( 0, 0 ), fieldMbFlag, and botFieldFlag as the inputs and the output is the
sample location ( xRefMin16, yRefMin16 ) in units of 1/16-th sample.


2. The derivation process for reference layer sample locations in resampling as specified in clause G.6.3 is invoked

with chromaFlag, the sample location ( mbW − 1, mbH − 1 ), fieldMbFlag, and botFieldFlag as the inputs and
the output is the sample location ( xRefMax16, yRefMax16 ) in units of 1/16-th sample.


3. With Z being replaced by L for chromaFlag equal to 0 and C for chromaFlag equal to 1, the variables refW, refH,

refMbW, and refMbH are derived by:





refW   = RefLayerPicWidthInSamplesZ (G-268)
refH    = RefLayerPicHeightInSamplesZ (G-269)
refMbW = ( ( chromaFlag = = 0 ) ? 16 : RefLayerMbWidthC ) (G-270)
refMbH = ( ( chromaFlag = = 0 ) ? 16 : RefLayerMbHeightC ) (G-271)


4. The variables xOffset, yOffset, refArrayW, and refArrayH are derived by:


xOffset   = ( ( ( xRefMin16 − 64 ) >> 8 ) << 4 ) − ( refMbW >> 1 ) (G-272)
yOffset   = ( ( ( yRefMin16 − 64 ) >> 8 ) << 4 ) − ( refMbH >> 1 ) (G-273)
refArrayW = ( ( ( xRefMax16 + 79 ) >> 8 ) << 4 ) + 3 * ( refMbW >> 1 ) − xOffset (G-274)
refArrayH = ( ( ( yRefMax16 + 79 ) >> 8 ) << 4 ) + 3 * ( refMbH >> 1 ) − yOffset (G-275)


NOTE 1 – The derived array size might be larger than the array size that is actually required by the interpolation process
for Intra_Base prediction specified in clause G.8.6.2.3.


5. The variables xMin, yMin, xMax, and yMax are derived by:


xMin = ( xRefMin16 >> 4 ) − xOffset (G-276)
yMin = ( yRefMin16 >> 4 ) − yOffset (G-277)
xMax = ( ( xRefMax16 + 15 ) >> 4 ) − xOffset (G-278)
yMax = ( ( yRefMax16 + 15 ) >> 4 ) − yOffset (G-279)


6. The variables yRefScale and yRefAdd are derived as follows:


     - If RefLayerFrameMbsOnlyFlag is equal to 1 or RefLayerFieldPicFlag is equal to 1, yRefScale is set equal
to 1 and yRefAdd is set equal to 0.


     - Otherwise (RefLayerFrameMbsOnlyFlag is equal to 0 and RefLayerFieldPicFlag is equal to 0), yRefScale
is set equal to 2 and yRefAdd is set equal to botFieldFlag.


The variable refSliceIdcMb is marked as "not available".


When constrained_intra_resampling_flag is equal to 1, the variable y proceeds over the values (yMin + 1)..(yMax − 1) and
for each value of y, the variable x proceeds over the values (xMin + 1)..(xMax − 1), and for each pair ( x, y ), the following
ordered steps are specified:


1. A reference layer sample location ( xRef, yRef ) is derived by:


xRef = Max( 0, Min( refW − 1, x + xOffset ) ) (G-280)
yRef = yRefScale * Max( 0, Min( refH / yRefScale − 1, y + yOffset ) ) + yRefAdd (G-281)


2. The derivation process for reference layer slice and intra macroblock identifications as specified in

clause G.8.6.2.2.1 is invoked with the reference layer sample location ( xRef, yRef ), refMbW, refMbH,
refLayerSliceIdc, refLayerFieldMbFlag, and refLayerMbType as the inputs and the outputs are the reference
layer slice identification refSliceIdc and the variable refIntraMbFlag.


3. When refIntraMbFlag is equal to 1 and refSliceIdcMb is marked as "not available", the variable refSliceIdcMb

is marked as "available" and set equal to refSliceIdc.


When constrained_intra_resampling_flag is equal to 1, the following ordered steps are specified:


1. The variable useIntraPredFlag is set equal to 0.


2. For x proceeding over the values 0..15 and y proceeding over the values 0..15, the following ordered steps are

specified:


a. The derivation process for reference layer macroblocks as specified in clause G.6.1 is invoked with the luma
location ( x, y ), fieldMbFlag, refLayerFieldMbFlag, and refLayerMbType as the inputs and the outputs are
assigned to mbAddrRefLayer and ( xRef, yRef ).


b. When refLayerMbType[ mbAddrRefLayer ] is equal to I_PCM, I_16x16, I_8x8, I_4x4, or I_BL, the variable

useIntraPredFlag is set equal to 1.


3. When useIntraPredFlag is equal to 1, it is a requirement of bitstream conformance that the bitstream shall not

contain data that result in refSliceIdcMb being marked as "not available".





Each sample refSampleArray[ x, y ] with x = 0..(refArrayW − 1) and y = 0..(refArrayH − 1) is derived as specified in the
following ordered steps:


1. A reference layer sample location ( xRef, yRef ) is derived by


xRef = Max( 0, Min( refW − 1, x + xOffset ) ) (G-282)
yRef = yRefScale * Max( 0, Min( refH / yRefScale − 1, y + yOffset ) ) + yRefAdd (G-283)


2. The derivation process for reference layer slice and intra macroblock identifications as specified in

clause G.8.6.2.2.1 is invoked with the reference layer sample location ( xRef, yRef ), refMbW, refMbH,
refLayerSliceIdc, refLayerFieldMbFlag, and refLayerMbType as the inputs and the outputs are the reference
layer slice identification refSliceIdc and the variable refIntraMbFlag.


3. When constrained_intra_resampling_flag is equal to 1, refIntraMbFlag is equal to 1, x is greater than xMin, x is

less than xMax, y is greater than yMin, and y is less than yMax, it is a requirement of bitstream conformance that
the bitstream shall not contain data that result in refSliceIdc being not equal to refSliceIdcMb.

NOTE 2 – This constraint specifies that a macroblock cannot be coded with base_mode_flag equal to 1 when it
covers intra-coded macroblocks of more than one slice in the reference layer representation,
constrained_intra_resampling_flag is equal to 1, and either the inferred macroblock type is equal to I_BL or the
conditions for invoking the intra-inter prediction combination process as specified in clause G.8.4.2.2 are fulfilled.


4. Depending on refIntraMbFlag, constrained_intra_resampling_flag, and refSliceIdc, the following applies:


     - If any of the following conditions are true, the sample refSampleArray[ x, y ] is marked as "not available for
Intra_Base prediction" and its value is set equal to 0:


        - refIntraMbFlag is equal to 0,


        - constrained_intra_resampling_flag is equal to 1 and refSliceIdcMb is marked as "not available",


        - constrained_intra_resampling_flag is equal to 1 and refSliceIdc is not equal to refSliceIdcMb.


     - Otherwise, the sample refSampleArray[ x, y ] is marked as "available for Intra_Base prediction" and its
value is derived by


refSampleArray[ x, y ] = refLayerPicSamples[ xRef, yRef ] (G-284)


The construction process for not available sample values prior to intra resampling as specified in clause G.8.6.2.2.2 is
invoked with refMbW, refMbH, refArrayW, refArrayH, refSampleArray, xOffset, and yOffset as the inputs and the output
is a modified version of the sample array refSampleArray.


**G.8.6.2.2.1** **Derivation process for reference layer slice and intra macroblock identifications**


Inputs to this process are:


- a reference layer sample location ( xRef, yRef ) relative to the upper-left sample of the considered colour component
of the reference layer picture,


- two variables refMbW and refMbH specifying the width and height, respectively, of a reference layer macroblock for
the considered colour component,


- a one-dimensional array refLayerSliceIdc with RefLayerPicSizeInMbs elements specifying slice identifications for
the macroblocks of the reference layer representation,


- a one-dimensional array refLayerFieldMbFlag with RefLayerPicSizeInMbs elements specifying which macroblocks
of the reference layer representation are field macroblocks and which macroblocks are frame macroblocks,


- a one-dimensional array refLayerMbType with RefLayerPicSizeInMbs elements specifying macroblock types for the
macroblocks of the reference layer representation.


Outputs of this process are:


- a reference layer slice identification refSliceIdc for the slice that covers the input reference layer sample location,


- a variable refIntraMbFlag specifying whether the reference layer macroblock that covers the input reference layer
sample location is intra coded.


The reference layer macroblock address refMbAddr is derived as follows:


- If RefLayerMbaffFrameFlag is equal to 0, the variable refMbAddr is derived by


refMbAddr = ( yRef / refMbH ) * RefLayerPicWidthInMbs + ( xRef / refMbW ) (G-285)





- Otherwise (RefLayerMbaffFrameFlag is equal to 1), the variable refMbAddr is derived as specified in the following
ordered steps:


1. A variable refMbAddrTop is derived by


refMbAddrTop = 2 * ( ( yRef / ( 2 * refMbH ) ) * RefLayerPicWidthInMbs
+ ( xRef / refMbW ) ) (G-286)


2. Depending on refLayerFieldMbFlag[ refMbAddrTop ], the variable refMbAddr is derived as follows:


     - If refLayerFieldMbFlag[ refMbAddrTop ] is equal to 0, the variable refMbAddr is derived by


refMbAddr = refMbAddrTop + ( yRef % ( 2 * refMbH ) ) / refMbH (G-287)


     - Otherwise (refLayerFieldMbFlag[ refMbAddrTop ] is equal to 1), the variable refMbAddr is derived by


refMbAddr = refMbAddrTop + ( yRef % 2 ) (G-288)


The reference layer slice identification refSliceIdc is set equal to refLayerSliceIdc[ refMbAddr ].


Depending on refLayerMbType[ refMbAddr ], the variable refIntraMbFlag is derived as follows:


- If refLayerMbType[ refMbAddr ] is equal to I_4x4, I_8x8, I_16x16, I_PCM, or I_BL, refIntraMbFlag is set equal
to 1.


- Otherwise (refLayerMbType[ refMbAddr ] is not equal to I_4x4, I_8x8, I_16x16, I_PCM, or I_BL), refIntraMbFlag
is set equal to 0.


**G.8.6.2.2.2** **Construction process for not available sample values prior to intra resampling**


Inputs to this process are:


- two variables refMbW and refMbH specifying the width and height, respectively, of a reference layer macroblock for
the considered colour component,


- two variables refArrayW and refArrayH specifying the width and height, respectively, of the array of reference layer
sample values,


- a (refArrayW)x(refArrayH) array refSampleArray of reference layer sample values,


- two variables xOffset and yOffset specifying the x and y coordinates, respectively, of the reference layer sample
location that corresponds to the sample refSampleArray[ 0, 0 ] of the array refSampleArray.


Output of this process is a modified version of the array refSampleArray.


For each sample refSampleArray[ x, y ] with x = (refMbW / 2)..(refArrayW − refMbW / 2 − 1) and
y = (refMbH / 2)..(refArrayH − refMbH / 2 − 1) that is marked as "not available for Intra_Base prediction", the following
ordered steps are specified:


1. The sample location difference ( xD, yD ) and the variable yA are derived by


xR = ( x + xOffset ) % refMbW (G-289)
yR = ( y + yOffset ) % refMbH (G-290)

xD = ( ( xR >= refMbW / 2 ) ? ( xR − refMbW ) : ( xR + 1 ) ) (G-291)
yD = ( ( yR >= refMbH / 2 ) ? ( yR − refMbH ) : ( yR + 1 ) ) (G-292)

yA = yD − ( refMbH / 2 + 1 ) * Sign( yD ) (G-293)


2. When any of the following conditions are true, yD is set equal to yA:


      - the sample refSampleArray[ x, y − yD ] is marked as "not available for Intra_Base prediction", the sample
refSampleArray[ x, y − yA ] is marked as "available for Intra_Base prediction", and the sample
refSampleArray[ x − xD, y ] is marked as "available for Intra_Base prediction",


      - all of the samples refSampleArray[ x − xD, y ], refSampleArray[ x, y − yD ], and
refSampleArray[ x − xD, y − yD ] are marked as "not available for Intra_Base prediction" and any of the
samples refSampleArray[ x, y − yA ] and refSampleArray[ x − xD, y − yA ] is marked as "available for
Intra_base prediction",


      - Abs( yA ) is less than Abs( yD ) and any of the following conditions are true:


        - both samples refSampleArray[ x, y − yD ] and refSampleArray[ x, y − yA ] are marked as "available
for Intra_Base prediction",





        - any of the samples refSampleArray[ x, y − yD ] and refSampleArray[ x − xD, y − yD ] is marked as
"available for Intra_Base prediction", any of the samples refSampleArray[ x, y − yA ] and
refSampleArray[ x − xD, y − yA ] is marked as "available for Intra_Base prediction", and the sample
refSampleArray[ x − xD, y ] is marked as "not available for Intra_Base prediction".
NOTE – The variable yD is never set equal to yA when RefLayerFrameMbsOnlyFlag is equal to 1 or RefLayerFieldPicFlag
is equal to 1.


3. The sample value refSampleArray[ x, y ] is derived as follows:


      - If the sample refSampleArray[ x − xD, y ] and the sample refSampleArray[ x, y − yD ] are marked as
"available for Intra_Base prediction", the following ordered steps are specified:


a. A variable cornerSampleAvailableFlag is derived as follows:


           - If the sample refSampleArray[ x − xD, y − yD ] is marked as "available for Intra_Base
prediction", the variable cornerSampleAvailableFlag is set equal to 1.


           - Otherwise (the sample refSampleArray[ x − xD, y − yD ] is marked as "not available for
Intra_Base prediction"), the variable cornerSampleAvailable is set equal to 0.


b. The diagonal construction process for not available sample values as specified in clause G.8.6.2.2.2.1

is invoked with refArrayW, refArrayH, refSampleArray, the sample location difference ( xD, yD ), the
sample location ( x, y ), and the variable cornerSampleAvailableFlag as the inputs and the output is the
sample array refSampleArray with a modified sample value at sample location ( x, y ).


      - Otherwise (the sample refSampleArray[ x − xD, y ] or the sample refSampleArray[ x, y − yD ] is marked
as "not available for Intra_Base prediction"), the following applies:


        - If the sample refSampleArray[ x − xD, y ] is marked as "available for Intra_Base prediction", the
sample value refSampleArray[ x, y ] is set equal to refSampleArray[ x − xD, y ].


        - Otherwise, if the sample refSampleArray[ x, y − yD ] is marked as "available for Intra_Base
prediction", the sample value refSampleArray[ x, y ] is set equal to refSampleArray[ x, y − yD ].


        - Otherwise, if the sample refSampleArray[ x − xD, y − yD ] is marked as "available for Intra_Base
prediction", the sample value refSampleArray[ x, y ] is set equal to refSampleArray[ x − xD, y − yD ].


        - Otherwise (the samples refSampleArray[ x − xD, y ], refSampleArray[ x, y − yD ], and
refSampleArray[ x − xD, y − yD ] are marked as "not available for Intra_Base prediction"), the sample
value refSampleArray[ x, y ] is not modified.


All samples refSampleArray[ x, y ] with x = 0..(refArrayW − 1) and y = 0..(refArrayH − 1) are marked as "available for
Intra_Base prediction".


**G.8.6.2.2.2.1** **Diagonal construction process for not available sample values**


Inputs to this process are:


- two variables refArrayW and refArrayH specifying the width and height, respectively, of the array of reference layer
sample values,


- a (refArrayW)x(refArrayH) array p of reference layer sample values,


- a sample location difference ( xD, yD ),


- a sample location ( x, y ) inside the reference layer sample array refSampleArray,


- a variable cornerSampleAvailableFlag.


Output of this process is the sample array p with a modified sample value at sample location ( x, y ).


The variables diffHorVer and sgnXY are derived by


diffHorVer = Abs( xD ) − Abs( yD ) (G-294)
sgnXY    = Sign( xD * yD ) (G-295)


When cornerSampleAvailableFlag is equal to 0, the following ordered steps are specified:


1. The variable cornerSample is set equal to p[ x − xD, y − yD ].


2. The sample location ( xC, yC ) is set equal to ( x − xD + Sign( xD ), y − yD + Sign( yD ) ) and the sample value

p[ x − xD, y − yD ] is modified by


p[ x − xD, y − yD ] = ( p[ x − xD, yC ] + p[ xC, y − yD ] + 1 ) >> 1 (G-296)





The sample value p[ x, y ] is derived as follows:


- If diffHorVer is greater than 0, the sample location ( xC, yC ) is set equal to ( x − sgnXY * yD, y − yD ) and the
sample value p[ x, y ] is derived by


p[ x, y ] = ( p[ xC − 1, yC ] + 2 * p[ xC, yC ] + p[ xC + 1, yC ] + 2 ) >> 2 (G-297)


- Otherwise, if diffHorVer is less than 0, the sample location ( xC, yC ) is set equal to ( x − xD, y − sgnXY * xD ) and
the sample value p[ x, y ] is derived by


p[ x, y ] = ( p[ xC, yC − 1 ] + 2 * p[ xC, yC ] + p[ xC, yC + 1 ] + 2 ) >> 2 (G-298)


- Otherwise (diffVerHor is equal to 0), the sample location ( xC, yC ) is set equal to
( x − xD +Sign( xD ), y − yD + Sign( yD ) ) and the sample value p[ x, y ] is derived by


p[ x, y ] = ( p[ xC, y − yD ] + 2 * p[ x − xD, y − yD ] + p[ x − xD, yC ] + 2 ) >> 2 (G-299)


When cornerSampleAvailableFlag is equal to 0, the sample value p[ x − xD, y − yD ] is set equal to cornerSample.


**G.8.6.2.3** **Interpolation process for Intra_Base prediction**


Inputs to this process are:


- a variable filteringModeFlag specifying the interpolation method,


- a variable chromaFlag specifying whether the luma or a chroma component is subject to the resampling process,


- two variables mbW and mbH specifying the width and height, respectively, of a macroblock for the considered colour
component,


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a variable botFieldFlag specifying whether a top or a bottom field is subject to the resampling process (when
RefLayerFrameMbsOnlyFlag is equal to 0 or frame_mbs_only_flag is equal to 0),


- a variable fldPrdInFrmMbFlag specifying whether field prediction for a frame macroblock is applied,


- a variable yBorder specifying the vertical border for the output sample array predSamples,


- two variables refArrayW and refArrayH specifying the width and height, respectively, of the array of reference layer
sample values,


- a (refArrayW)x(refArrayH) array refSampleArray of reference layer sample values,


- two variables xOffset and yOffset specifying the x and y coordinate, respectively, of the reference layer sample
location that corresponds to the sample refSampleArray[ 0, 0 ] of the array refSampleArray.


Output of this process is an (mbW)x(mbH / ( 1 + fldPrdInFrmMbFlag ) + 2 * yBorder) array predArray of interpolated
sample values.


Table G-9 specifies the filter coefficients eF[ p, x ] with p = 0..15 and x = 0..3 of the luma interpolation filter eF for
resampling in Intra_Base prediction.





**Table G-9 – 16-phase luma interpolation filter for resampling in Intra_Base prediction**

|phase p|interpolation filter coefficients|Col3|Col4|Col5|
|---|---|---|---|---|
|phase p|eF[ p, 0 ]|eF[ p, 1 ]|eF[ p, 2 ]|eF[ p, 3 ]|
|0|0|32|0|0|
|1|−1|32|2|−1|
|2|−2|31|4|−1|
|3|−3|30|6|−1|
|4|−3|28|8|−1|
|5|−4|26|11|−1|
|6|−4|24|14|−2|
|7|−3|22|16|−3|
|8|−3|19|19|−3|
|9|−3|16|22|−3|
|10|−2|14|24|−4|
|11|−1|11|26|−4|
|12|−1|8|28|−3|
|13|−1|6|30|−3|
|14|−1|4|31|−2|
|15|−1|2|32|−1|



Let tempArray be a (refArrayW)x(mbH / ( 1 + fldPrdInFrmMbFlag ) + 2 * yBorder) array of samples. Each sample
tempArray[ x, y ] with x = 0..(refArrayW − 1) and y = 0..(mbH / ( 1 + fldPrdInFrmMbFlag ) + 2 * yBorder − 1) is derived
as specified in the following ordered steps:


1. The variable yP is derived by


yP = ( y − yBorder ) * ( 1 + fldPrdInFrmMbFlag ) + botFieldFlag (G-300)


2. The derivation process for reference layer sample locations in resampling as specified in clause G.6.3 is invoked

with chromaFlag, the sample location ( 0, yP ), fieldMbFlag, and botFieldFlag as the inputs and the output is the
sample location ( xRef16, yRef16 ) in units of 1/16-th sample.
NOTE 1 – In this invocation of the process in clause G.6.3, only the vertical component yRef16 of the sample location needs
to be derived.


3. The variables yRef and yPhase are derived by


yRef   = ( yRef16 >> 4 ) − yOffset (G-301)
yPhase = ( yRef16 − 16 * yOffset ) % 16 (G-302)


4. Depending on filteringModeFlag, the sample value tempArray[ x, y ] is derived as follows:


     - If filteringModeFlag is equal to 0, the sample value tempArray[ x, y ] is derived by


tempArray[ x, y ] = eF[ yPhase, 0 ] * refSampleArray[ x, yRef − 1 ] +
eF[ yPhase, 1 ] * refSampleArray[ x, yRef ] +
eF[ yPhase, 2 ] * refSampleArray[ x, yRef + 1 ] + (G-303)
eF[ yPhase, 3 ] * refSampleArray[ x, yRef + 2 ]


     - Otherwise (filteringModeFlag is equal to 1), the sample value tempArray[ x, y ] is derived by


tempArray[ x, y ] = ( 16 − yPhase ) * refSampleArray[ x, yRef ] +
yPhase * refSampleArray[ x, yRef + 1 ] (G-304)


Each sample predArray[ x, y ] with x = 0..(mbW − 1) and y = 0..(mbH / ( 1 + fldPrdInFrmMbFlag ) + 2 * yBorder − 1) is
derived as specified in the following ordered steps:


1. The derivation process for reference layer sample locations in resampling as specified in clause G.6.3 is invoked

with chromaFlag, the sample location ( x, 0 ), fieldMbFlag, and botFieldFlag as the inputs and the output is the
sample location ( xRef16, yRef16 ) in units of 1/16-th sample.
NOTE 2 – In this invocation of the process in clause G.6.3, only the horizontal component xRef16 of the sample location
needs to be derived.





2. The variables xRef and xPhase are derived by


xRef   = ( xRef16 >> 4 ) − xOffset (G-305)
xPhase = ( xRef16 − 16 * xOffset ) % 16 (G-306)


3. Depending on filteringModeFlag, and with Clip1 being replaced by Clip1Y for chromaFlag equal to 0 and Clip1C

for chromaFlag equal to 1, the sample value predArray[ x, y ] is derived as follows:


     - If filteringModeFlag is equal to 0, the sample value tempArray[ x, y ] is derived by


predArray[ x, y ] = Clip1( ( eF[ xPhase, 0 ] * tempArray[ xRef − 1, y ] +
eF[ xPhase, 1 ] * tempArray[ xRef, y] +
eF[ xPhase, 2 ] * tempArray[ xRef + 1, y ] + (G-307)
eF[ xPhase, 3 ] * tempArray[ xRef + 2, y ] + 512 ) >> 10 )


     - Otherwise (filteringModeFlag is equal to 1), the sample value tempArray[ x, y ] is derived by


predArray[ x, y ] = ( ( 16 − xPhase ) * tempArray[ xRef, y ] +
xPhase * tempArray[ xRef + 1, y ] + 128 ) >> 8 (G-308)


**G.8.6.2.4** **Vertical interpolation process for Intra_Base prediction**


Inputs to this process are:


- a variable filteringModeFlag specifying the interpolation method,


- a variable chromaFlag specifying whether the luma or a chroma component is subject to the resampling process,


- two variables mbW and mbH specifying the width and height, respectively, of a macroblock for the considered colour
component,


- a variable botFieldFlag specifying whether the sample array fieldPredArray contains interpolated samples for the top
or bottom field,


- a variable yBorder specifying the vertical border for the sample array fieldPredArray,


- a variable frameMbFlag specifying whether the current macroblock is a frame or a field macroblock,


- an (mbW)x(mbH / ( 1 + frameMbFlag ) + 2 * yBorder) array fieldPredArray of sample values.


Output of this process is an (mbW)x(mbH) array predArray of interpolated sample values.


Each sample predArray[ x, y ] with x = 0..(mbW − 1) and y = 0..(mbH − 1) is derived as follows:


- If frameMbFlag is equal to 1 and ( y % 2 ) is equal to botFieldFlag, the sample value predArray[ x, y ] is derived by


predArray[ x, y ] = fieldPredArray[ x, ( y >> 1 ) + yBorder ] (G-309)


- Otherwise (frameMbFlag is equal to 0 or ( y % 2 ) is not equal to botFieldFlag), the following ordered steps are
specified:


1. The variable yFld is derived by


yFld = ( y >> frameMbFlag ) + yBorder − botFieldFlag (G-310)


2. Depending on filteringModeFlag, and with Clip1 being replaced by Clip1Y for chromaFlag equal to 0 and Clip1C

for chromaFlag equal to 1, the sample value predArray[ x, y ] is derived as follows:


     - If filteringModeFlag is equal to 0, the sample value predArray[ x, y ] is derived by


predArray[ x, y ] = Clip1( ( 19 * ( fieldPredArray[ x, yFld ] +
fieldPredArray[ x, yFld + 1 ] ) −
3 * ( fieldPredArray[ x, yFld − 1 ] +
fieldPredArray[ x, yFld + 2 ] ) + 16 ) >> 5 ) (G-311)


     - Otherwise (filteringModeFlag is equal to 1), the sample value predArray[ x, y ] is derived by


predArray[ x, y ] = ( fieldPredArray[ x, yFld ] +
fieldPredArray[ x, yFld + 1 ] + 1 ) >> 1 (G-312)


**G.8.6.2.5** **Derivation process for variables related to inter-layer intra prediction**


This clause is only invoked when MinNoInterLayerPredFlag is equal to 0.


Input to this process is a variable currDQId.


Outputs of this process are:





- a variable numILIntraPredSamples,


- a variable numRefLayerILIntraPredMbs.


Unless stated otherwise, all syntax elements and derived upper-case variables that are referred to inside this clause are
syntax elements and derived upper case variables for the layer representation with DQId equal to currDQId.


Inside this clause, the collective terms currentVars and refLayerVars are specified as follows:


- If SpatialResolutionChangeFlag is equal to 1, the following applies:


   - currentVars is the collective term currentVars after completion of the base decoding process for layer
representations with resolution change as specified in clause G.8.1.3.2 for the layer representation with DQId
equal to currDQId,


   - refLayerVars is the collective term refLayerVars after completion of the base decoding process for layer
representations with resolution change as specified in clause G.8.1.3.2 for the layer representation with DQId
equal to currDQId.


- Otherwise (SpatialResolutionChangeFlag is equal to 0), the following applies:


   - currentVars is the collective term currentVars after completion of the base decoding process for layer
representations without resolution change as specified in clause G.8.1.3.1 for the layer representation with DQId
equal to currDQId,


   - refLayerVars is of the collective term currentVars before invoking the base decoding process for layer
representations without resolution change as specified in clause G.8.1.3.1 for the layer representation with DQId
equal to currDQId.


Inside this clause, the arrays of the collective term currentVars are referred to by their names as specified in
clause G.8.1.2.1.


Inside this clause, the arrays fieldMbFlag and mbType of the collective term refLayerVars are referred to as
refLayerFieldMbFlag and refLayerMbType, respectively.


Let currILIntraPredFlag be a (PicWidthInSamplesL)x(PicHeightInSamplesL) array and let refILIntraPredFlag be a
one-dimensional array with RefLayerPicSizeInMbs elements. All elements of the arrays currILIntraPredFlag and
refILIntraPredFlag are initially set equal to 0.


The variable yC proceeds over the values 0..(PicHeightInSamplesL − 1). For each value of yC, the variable xC proceeds
over the values 0..(PicWidthInSamplesL − 1). For each combination of the values yC and xC, the following ordered steps
are specified:


1. The variable mbAddr is set equal to the address of the macroblock that contains the luma sample at location

( xC, yC ) relative to the upper-left sample of the layer picture.


2. Depending on SpatialResolutionChangeFlag, the following applies:


      - If SpatialResolutionChangeFlag is equal to 0, the following ordered steps are specified:


a. The array element currILIntraPredFlag[ xC, yC ] is derived as follows:


           - If mbType[ mbAddr ] is equal to I_BL, currILIntraPredFlag[ xC, yC ] is set equal to 1.


           - Otherwise (mbType[ mbAddr ] is not equal to I_BL), currILIntraPredFlag[ xC, yC ] is set equal
to 0.


b. When currILIntraPredFlag[ xC, yC ] is equal to 1, the following ordered steps are specified:


i. The variable refMbAddr is derived as specified in clause G.8.1.2.2 with mbAddr being the value

of mbAddr derived in step 1 of this clause.


ii. The array element refILIntraPredFlag[ refMbAddr ] is set equal to 1.


iii. When refLayerMbType[ refMbAddr ] is equal to I_16x16, I_8x8, or I_4x4, let setRefIntraMbs

be the set of macroblocks that contain luma or chroma samples that are directly (by the invocation
of clause G.8.3.2 for the macroblock with address refMbAddr) or indirectly (by multiple
invocations of clause G.8.3.2 for macroblocks with mbAddr less than or equal to refMbAddr)
used for construction of the intra prediction signal of the macroblock with address refMbAddr in
the layer representation with DQId equal to MaxRefLayerDQId.


iv. For refIntraMbAddr proceeding over the macroblock addresses for the macroblocks of the set

setRefIntraMbs, refILIntraPredFlag[ refIntraMbAddr ] is set equal to 1.





      - Otherwise (SpatialResolutionChangeFlag is equal to 1), the following ordered steps are specified:


a. When RestrictedSpatialResolutionFlag is equal to 0, MbaffFrameFlag is equal to 0,
RefLayerMbaffFrameFlag is equal to 0, and base_mode_flag for the macroblock with address mbAddr
is equal to 1, the derivation process for reference layer macroblocks as specified in clause G.6.1 is
invoked with the luma location ( xC % 16, yC % 16), fieldMbFlag, refLayerFieldMbFlag, and
refLayerMbType as the inputs and the outputs are assigned to mbAddrRefLayer and ( xRef, yRef ).
For this invocation of clause G.6.1, CurrMbAddr is set equal to mbAddr.


b. The element currILIntraPredFlag[ xC, yC ] is derived as follows:


           - If any of the following conditions are true, currILIntraPredFlag[ xC, yC ] is set equal to 1:


             - mbType[ mbAddr ] is equal to I_BL,


             - RestrictedSpatialResolutionFlag is equal to 0, MbaffFrameFlag is equal to 0,
RefLayerMbaffFrameFlag is equal to 0, base_mode_flag for the macroblock with address
mbAddr is equal to 1, and refLayerMbType[ mbAddrRefLayer ] is equal to I_PCM,
I_16x16, I_8x8, I_4x4, or I_BL.


           - Otherwise, currILIntraPredFlag[ xC, yC ] is set equal to 0.


c. When currILIntraPredFlag[ xC, yC ] is equal to 1, the following ordered steps are specified:


i. Let setOfRefSamples be the set of reference layer luma sample locations ( xR, yR ) of the luma

sample values that are used in the filtering processes specified in clause G.8.6.2.3 and, when
applicable, clause G.8.6.2.4 for deriving the inter-layer intra prediction sample for the luma
sample at location ( xC, yC ) relative to the upper-left luma sample of the layer picture.


ii. For each of the reference layer luma sample locations ( xR, yR ) of the set setOfRefSamples that

correspond to luma samples marked "available for Intra_Base prediction" in the invocation of
clause G.8.6.2.2 for the macroblock with address mbAddr of the layer representation with DQId
equal to currDQId, the following ordered steps are specified:


(1) Let refMbAddr be the macroblock address of the macroblock in the layer representation with

DQId equal to MaxRefLayerDQId that contains the luma sample at location ( xR, yR ).


(2) The array element refILIntraPredFlag[ refMbAddr ] is set equal to 1.


(3) When refLayerMbType[ refMbAddr ] is equal to I_16x16, I_8x8, or I_4x4, let

setRefIntraMbs be the set of macroblocks that contain luma or chroma samples that are
directly (by the invocation of clause G.8.3.2 for the macroblock with address refMbAddr) or
indirectly (by multiple invocations of clause G.8.3.2 for macroblocks with mbAddr less than
or equal to refMbAddr) used for construction of the intra prediction signal of the macroblock
with address refMbAddr in the layer representation with DQId equal to MaxRefLayerDQId.


(4) For refIntraMbAddr proceeding over the macroblock addresses for the macroblocks of the

set setRefIntraMbs, refILIntraPredFlag[ refIntraMbAddr ] is set equal to 1.


The variable numILIntraPredSamples is set equal to the number of elements of the
(PicWidthInSamplesL)x(PicHeightInSamplesL) array currILIntraPredFlag that are equal to 1.

NOTE 1 – The variable numILIntraPredSamples is a measure for the number of luma samples in the layer representation with
DQId equal to currDQId that are predicted by inter-layer intra prediction.


The variable numRefLayerILIntraPredMbs is set equal to the number of elements of the array refILIntraPredFlag that are
equal to 1.

NOTE 2 – The variable numRefLayerILIntraPredMbs is a measure for the number of intra-coded macroblocks in the reference
layer representation that need to be decoded for constructing the inter-layer intra prediction samples of the layer representation
with DQId equal to currDQId.


**G.8.6.3** **Resampling process for residual samples**


Inputs to this process are:


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a one-dimensional array refLayerFieldMbFlag with RefLayerPicSizeInMbs elements specifying which macroblocks
of the reference layer representation are field macroblocks and which macroblocks are frame macroblocks,


- a one-dimensional array refLayerCTrafo with RefLayerPicSizeInMbs elements specifying the luma transform types
for the macroblocks of the reference layer representation,





- a (RefLayerPicWidthInSamplesL)x(RefLayerPicHeightInSamplesL) array refLayerPicSamplesL of luma samples for
the reference layer representation,


- a (PicWidthInSamplesL)x(PicHeightInSamplesL) array picSamplesL of luma samples,


- when ChromaArrayType is not equal to 0, two (RefLayerPicWidthInSamplesC)x(RefLayerPicHeightInSamplesC)
arrays refLayerPicSamplesCb and refLayerPicSamplesCr of chroma samples for the reference layer representation,


- when ChromaArrayType is not equal to 0, two (PicWidthInSamplesC)x(PicHeightInSamplesC) arrays picSamplesCb
and picSamplesCr of chroma samples.


Outputs of this process are:


- a modified version of the array picSamplesL of luma samples,


- when ChromaArrayType is not equal to 0, modified versions of the arrays picSamplesCb and picSamplesCr of chroma
samples.


The resampling process for residual samples of a macroblock colour component as specified in clause G.8.6.3.1 is invoked
with chromaFlag equal to 0, mbW equal to 16, mbH equal to 16, fieldMbFlag, refLayerPicSamplesL, refLayerFieldMbFlag,
and refLayerCTrafo as the inputs and the output is the 16x16 array mbPredL of residual prediction samples for the luma
component of the current macroblock.


When ChromaArrayType is not equal to 0, for CX being replaced by Cb and Cr, the resampling process for residual
samples of a macroblock colour component as specified in clause G.8.6.3.1 is invoked with chromaFlag equal to 1, mbW
equal to MbWidthC, mbH equal to MbHeightC, fieldMbFlag, refLayerPicSamplesCX, refLayerFieldMbFlag, and
refLayerCTrafo as the inputs and the output is the (MbWidthC)x(MbHeightC) array mbPredCX of residual prediction
samples for the CX component of the current macroblock.


The picture sample array construction process as specified in clause G.8.5.4.1 is invoked with fieldMbFlag, mbPredL,
picSamplesL and, when ChromaArrayType is not equal to 0, mbPredCb, mbPredCr, picSamplesCb, and picSamplesCr as the
inputs and the outputs are a modified version of picSamplesL and, when ChromaArrayType is not equal to 0, modified
versions of picSamplesCb, and picSamplesCr.


**G.8.6.3.1** **Resampling process for residual samples of a macroblock colour component**


Inputs to this process are:


- a variable chromaFlag specifying whether the luma or a chroma component is subject to the resampling process,


- two variables mbW and mbH specifying the width and height, respectively, of a macroblock for the considered colour
component,


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- an array refLayerPicSamples, which is a (RefLayerPicWidthInSamplesL)x(RefLayerPicHeightInSamplesL) array
containing constructed residual luma sample values for the reference layer representation when chromaFlag is equal
to 0 or a (RefLayerPicWidthInSamplesC)x(RefLayerPicHeightInSamplesC) array containing constructed residual
chroma sample values for the reference layer representation when chromaFlag is equal to 1,


- a one-dimensional array refLayerFieldMbFlag with RefLayerPicSizeInMbs elements specifying which macroblocks
of the reference layer representation are field macroblocks and which macroblocks are frame macroblocks,


- a one-dimensional array refLayerCTrafo with RefLayerPicSizeInMbs elements specifying the luma transform types
for the macroblocks of the reference layer representation.


Output of this process is an (mbW)x(mbH) array mbPred of residual prediction samples.


The variable botFieldFlag is derived as follows:


- If RefLayerFrameMbsOnlyFlag is equal to 1, botFieldFlag is set equal to 0.


- Otherwise, if field_pic_flag is equal to 1, botFieldFlag is set equal to bottom_field_flag.


- Otherwise, if RefLayerFieldPicFlag is equal to 1, botFieldFlag is set equal to RefLayerBottomFieldFlag.


- Otherwise, if fieldMbFlag is equal to 1, botFieldFlag is set equal to (CurrMbAddr % 2).


- Otherwise, botFieldFlag is set equal to 0.


The variable frameBasedResamplingFlag is derived as follows:


- If all of the following conditions are true, frameBasedResamplingFlag is set equal to 1:





   - RefLayerFrameMbsOnlyFlag is equal to 1,


   - frame_mbs_only_flag is equal to 1.


- Otherwise, frameBasedResamplingFlag is set equal to 0.


The variable topAndBotResamplingFlag is derived as follows:


- If all of the following conditions are true, topAndBotResamplingFlag is set equal to 1:


   - RefLayerFrameMbsOnlyFlag is equal to 0,


   - RefLayerFieldPicFlag is equal to 0,


   - frame_mbs_only_flag is equal to 0,


   - fieldMbFlag is equal to 0.


- Otherwise, topAndBotResamplingFlag is set equal to 0.


The variable botFieldFrameMbsOnlyRefFlag is derived as follows:


- If RefLayerFrameMbsOnlyFlag is equal to 1, fieldMbFlag is equal to 1, and any of the following conditions are true,
botFieldFrameMbsOnlyRefFlag is set equal to 1:


   - field_pic_flag is equal to 1 and bottom_field_flag is equal to 1,


   - field_pic_flag is equal to 0 and ( CurrMbAddr % 2 ) is equal to 1.


- Otherwise, botFieldFrameMbsOnlyRefFlag is set equal to 0.


The array predArray is derived as specified in the following.


- If botFieldFrameMbsOnlyRefFlag is equal to 1, the following ordered steps are specified:


1. The reference layer sample array construction process prior to residual resampling as specified in clause G.8.6.3.2

is invoked with chromaFlag, mbW, mbH, fieldMbFlag, botFieldFlag, yBorder equal to 1, refLayerPicSamples,
refLayerFieldMbFlag, and refLayerCTrafo as the inputs and the outputs are the variables refArrayW, refArrayH,
the array refSampleArray of reference layer sample values, the array refTransBlkIdc of reference layer transform
block identifications, and the variables xOffset and yOffset.


2. The interpolation process for residual prediction as specified in clause G.8.6.3.3 is invoked with chromaFlag,

mbW, mbH, fieldMbFlag, botFieldFlag, fldPrdInFrmMbFlag equal to 0, yBorder equal to 1, refArrayW,
refArrayH, refSampleArray, refTransBlkIdc, xOffset, and yOffset as the inputs and the output is the
(mbW)x(mbH + 2) array topFldPredArray of top field prediction samples.


3. The vertical interpolation process for residual prediction as specified in clause G.8.6.3.4 is invoked with mbW,

mbH, botFieldFlag, yBorder equal to 1, frameMbFlag equal to 0, and topFldPredArray as the inputs and the
output is the (mbW)x(mbH) array mbPred of residual prediction samples.


- Otherwise, if frameBasedResamplingFlag is equal to 1 or fieldMbFlag is equal to 1, the following ordered steps are
specified:


1. The reference layer sample array construction process prior to residual resampling as specified in clause G.8.6.3.2

is invoked with chromaFlag, mbW, mbH, fieldMbFlag, botFieldFlag, yBorder equal to 0, refLayerPicSamples,
refLayerFieldMbFlag, and refLayerCTrafo as the inputs and the outputs are the variables refArrayW, refArrayH,
the array refSampleArray of reference layer sample values, the array refTransBlkIdc of reference layer transform
block identifications, and the variables xOffset and yOffset.


2. The interpolation process for residual prediction as specified in clause G.8.6.3.3 is invoked with chromaFlag,

mbW, mbH, fieldMbFlag, botFieldFlag, fldPrdInFrmMbFlag equal to 0, yBorder equal to 0, refArrayW,
refArrayH, refSampleArray, refTransBlkIdc, xOffset, and yOffset as the inputs and the output is the
(mbW)x(mbH) array mbPred of residual prediction samples.


- Otherwise, if topAndBotResamplingFlag is equal to 0, the following ordered steps are specified:


1. The reference layer sample array construction process prior to residual resampling as specified in clause G.8.6.3.2

is invoked with chromaFlag, mbW, mbH, fieldMbFlag, botFieldFlag, yBorder equal to 1, refLayerPicSamples,
refLayerFieldMbFlag, and refLayerCTrafo as the inputs and the outputs are the variables refArrayW, refArrayH,
the array refSampleArray of reference layer sample values, the array refTransBlkIdc of reference layer transform
block identifications, and the variables xOffset and yOffset.





2. The interpolation process for residual prediction as specified in clause G.8.6.3.3 is invoked with chromaFlag,

mbW, mbH, fieldMbFlag, botFieldFlag, fldPrdInFrmMbFlag equal to 1, yBorder equal to 1, refArrayW,
refArrayH, refSampleArray, refTransBlkIdc, xOffset, and yOffset as the inputs and the output is the
(mbW)x(mbH / 2 + 2) array fieldPredArray of field prediction samples.


3. The vertical interpolation process for residual prediction as specified in clause G.8.6.3.4 is invoked with mbW,

mbH, botFieldFlag, yBorder equal to 1, frameMbFlag equal to 1, and fieldPredArray as the inputs and the output
is the (mbW)x(mbH) array mbPred of residual prediction samples.


- Otherwise (topAndBotResamplingFlag is equal to 1), the following ordered steps are specified:


1. The reference layer sample array construction process prior to residual resampling as specified in clause G.8.6.3.2

is invoked with chromaFlag, mbW, mbH, fieldMbFlag, botFieldFlag equal to 0, yBorder equal to 0,
refLayerPicSamples, refLayerFieldMbFlag, and refLayerCTrafo as the inputs and the outputs are the variables
refArrayTopW, refArrayTopH, the array refSampleArrayTop of reference layer sample values, the array
refTransBlkIdcTop of reference layer transform block identifications, and the variables xOffsetTop and
yOffsetTop.


2. The interpolation process for residual prediction as specified in clause G.8.6.3.3 is invoked with chromaFlag,

mbW, mbH, fieldMbFlag, botFieldFlag equal to 0, fldPrdInFrmMbFlag equal to 1, yBorder equal to 0,
refArrayTopW, refArrayTopH, refSampleArrayTop, refTransBlkIdcTop, xOffsetTop, and yOffsetTop as the
inputs and the output is the (mbW)x(mbH / 2) array topFieldPredArray of top field prediction samples.


3. The reference layer sample array construction process prior to residual resampling as specified in clause G.8.6.3.2

is invoked with chromaFlag, mbW, mbH, fieldMbFlag, botFieldFlag equal to 1, yBorder equal to 0,
refLayerPicSamples, refLayerFieldMbFlag, and refLayerCTrafo as the inputs and the outputs are the variables
refArrayBotW, refArrayBotH, the array refSampleArrayBot of reference layer sample values, the array
refTransBlkIdcBot of reference layer transform block identifications, and the variables xOffsetBot and
yOffsetBot.


4. The interpolation process for residual prediction as specified in clause G.8.6.3.3 is invoked with chromaFlag,

mbW, mbH, fieldMbFlag, botFieldFlag equal to 1, fldPrdInFrmMbFlag equal to 1, yBorder equal to 0,
refArrayBotW, refArrayBotH, refSampleArrayBot, refTransBlkIdcBot, xOffsetBot, and yOffsetBot as the inputs
and the output is the (mbW)x(mbH / 2) array botFieldPredArray of bottom field prediction samples.


5. Each sample predArray[ x, y ] with x = 0..(mbW − 1) and y = 0..(mbH − 1) of the array mbPred of residual

prediction samples is derived by


mbPred[ x, y ] = ( ( ( y % 2 ) = = 0 ) ? topFieldPredArray[ x, y >> 1 ]
: botFieldPredArray[ x, y >> 1 ] ) (G-313)


**G.8.6.3.2** **Reference layer sample array construction process prior to residual resampling**


Inputs to this process are:


- a variable chromaFlag specifying whether the luma or a chroma component is subject to the resampling process,


- two variables mbW and mbH specifying the width and height, respectively, of a macroblock for the considered colour
component,


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a variable botFieldFlag specifying whether a top or a bottom field is subject to the resampling process (when
RefLayerFrameMbsOnlyFlag is equal to 0 or frame_mbs_only_flag is equal to 0),


- a variable yBorder specifying the vertical border for determining the vertical size of the output arrays,


- an array refLayerPicSamples, which is a (RefLayerPicWidthInSamplesL)x(RefLayerPicHeightInSamplesL) array
containing constructed residual luma sample values for the reference layer representation when chromaFlag is equal
to 0 or a (RefLayerPicWidthInSamplesC)x(RefLayerPicHeightInSamplesC) array containing constructed residual
chroma sample values for the reference layer representation when chromaFlag is equal to 1,


- a one-dimensional array refLayerFieldMbFlag with RefLayerPicSizeInMbs elements specifying which macroblocks
of the reference layer representation are field macroblocks and which macroblocks are frame macroblocks,


- a one-dimensional array refLayerCTrafo with RefLayerPicSizeInMbs elements specifying the luma transform types
for the macroblocks of the reference layer representation.


Outputs of this process are:


- two variables refArrayW and refArrayH specifying the width and height, respectively, of the constructed arrays of
reference layer sample values and reference layer transform block identification,





- a (refArrayW)x(refArrayH) array refSampleArray of reference layer sample values,


- a (refArrayW)x(refArrayH) array refTransBlkIdc of reference layer transform block identifications,


- two variables xOffset and yOffset specifying the x and y coordinate, respectively, of the reference layer sample
location that corresponds to the sample refSampleArray[ 0, 0 ] of the array refSampleArray and the transform block
identification refTransBlkIdc[ 0, 0 ] of the array refTransBlkIdc.


The variables refW, refH, refMbW, refMbH, xOffset, yOffset, refArrayW, refArrayH, yRefScale, and yRefAdd are derived
as specified in the following ordered steps:


1. The derivation process for reference layer sample locations in resampling as specified in clause G.6.3 is invoked

with chromaFlag, the sample location ( 0, −yBorder ), fieldMbFlag, and botFieldFlag as the inputs and the output
is the sample location ( xRefMin16, yRefMin16 ) in units of 1/16-th sample.


2. The derivation process for reference layer sample locations in resampling as specified in clause G.6.3 is invoked

with chromaFlag, the sample location ( mbW − 1, mbH − 1 + yBorder ), fieldMbFlag, and botFieldFlag as the
inputs and the output is the sample location ( xRefMax16, yRefMax16 ) in units of 1/16-th sample.


3. With Z being replaced by L for chromaFlag equal to 0 and C for chromaFlag equal to 1, the variables refW, refH,

refMbW, and refMbH are derived by


refW   = RefLayerPicWidthInSamplesZ (G-314)
refH    = RefLayerPicHeightInSamplesZ (G-315)
refMbW = ( ( chromaFlag = = 0 ) ? 16 : RefLayerMbWidthC ) (G-316)
refMbH = ( ( chromaFlag = = 0 ) ? 16 : RefLayerMbHeightC ) (G-317)


4. The variables xOffset, yOffset, refArrayW, and refArrayH are derived by


xOffset   = ( xRefMin16 >> 4 ) (G-318)
yOffset   = ( yRefMin16 >> 4 ) (G-319)
refArrayW = ( xRefMax16 >> 4 ) − xOffset + 2 (G-320)
refArrayH = ( yRefMax16 >> 4 ) − yOffset + 2 (G-321)


5. The variables yRefScale and yRefAdd are derived as follows:


     - If RefLayerFrameMbsOnlyFlag is equal to 1 or RefLayerFieldPicFlag is equal to 1, yRefScale is set equal
to 1 and yRefAdd is set equal to 0.


     - Otherwise (RefLayerFrameMbsOnlyFlag is equal to 0 and RefLayerFieldPicFlag is equal to 0), yRefScale
is set equal to 2 and yRefAdd is set equal to botFieldFlag.


Each sample refSampleArray[ x, y ] and each transform block identification refTransBlkIdc[ x, y ] with
x = 0..(refArrayW − 1) and y = 0..(refArrayH − 1) are derived as specified in the following ordered steps:


1. A reference layer sample location ( xRef, yRef ) is derived by:


xRef = Max( 0, Min( refW − 1, x + xOffset ) ) (G-322)
yRef = yRefScale * Max( 0, Min( refH / yRefScale − 1, y + yOffset ) ) + yRefAdd (G-323)


2. The sample refSampleArray[ x, y ] is derived by


refSampleArray[ x, y ] = refLayerPicSamples[ xRef, yRef ] (G-324)


3. The transform block identification refTransBlkIdc[ x, y ] is derived by invoking the derivation process for

reference layer transform block identifications as specified in clause G.8.6.3.2.1 with the reference layer sample
location ( xRef, yRef ), chromaFlag, refMbW, refMbH, refLayerFieldMbFlag, and refLayerCTrafo as the inputs
and assigning the output to refTransBlkIdc[ x, y ].


**G.8.6.3.2.1** **Derivation process for reference layer transform block identifications**


Inputs to this process are:


- a reference layer sample location ( xRef, yRef ) relative to the upper-left sample of the considered colour component
of the reference layer picture,


- a variable chromaFlag specifying whether the luma or a chroma component is subject to the resampling process,


- two variables refMbW and refMbH specifying the width and height, respectively, of a reference layer macroblock for
the considered colour component,


- a one-dimensional array refLayerFieldMbFlag with RefLayerPicSizeInMbs elements specifying which macroblocks
of the reference layer representation are field macroblocks and which macroblocks are frame macroblocks,





- a one-dimensional array refLayerCTrafo with RefLayerPicSizeInMbs elements specifying the luma transform types
for the macroblocks of the reference layer representation.


Output of this process is a variable refTransBlkIdc specifying an identification for the reference layer transform block that
contains the sample at location ( xRef, yRef ).


The reference layer macroblock address refMbAddr and the reference layer sample location ( xM, yM ) inside the reference
layer macroblock are derived as follows:


- If RefLayerMbaffFrameFlag is equal to 0, the variable refMbAddr and the sample location ( xM, yM ) are derived by


refMbAddr = ( yRef / refMbH ) * RefLayerPicWidthInMbs + ( xRef / refMbW ) (G-325)
xM       = xRef % refMbW (G-326)
yM       = yRef % refMbH (G-327)


- Otherwise (RefLayerMbaffFrameFlag is equal to 1), the variable refMbAddr is derived as specified in the following
ordered steps:


1. A variable refMbAddrTop and the horizontal sample location xM are derived by


refMbAddrTop = 2 * ( ( yRef / ( 2 * refMbH ) ) * RefLayerPicWidthInMbs
+ ( xRef / refMbW ) ) (G-328)
xM          = xRef % refMbW (G-329)


2. Depending on refLayerFieldMbFlag[ refMbAddrTop ], the variable refMbAddr and the vertical sample location

yM are derived as follows:


     - If refLayerFieldMbFlag[ refMbAddrTop ] is equal to 0, the variables refMbAddr and yM are derived by


refMbAddr = refMbAddrTop + ( yRef % ( 2 * refMbH) ) / refMbH (G-330)
yM       = yRef % refMbH (G-331)


     - Otherwise (refLayerFieldMbFlag[ refMbAddrTop ] is equal to 1), the variables refMbAddr and yM are
derived by


refMbAddr = refMbAddrTop + ( yRef % 2 ) (G-332)
yM       = ( yRef % ( 2 * refMbH ) ) >> 1 (G-333)


Depending on chromaFlag, RefLayerChromaArrayType, and refLayerCTrafo[ refMbAddr ], the following applies:


- If (chromaFlag is equal to 0 or RefLayerChromaArrayType is equal to 3) and refLayerCTrafo[ refMbAddr ] is equal
to T_8x8, the variable refTransBlkIdc is derived by:


refTransBlkIdc = 1 + 2 * ( 4 * refMbAddr + 2 * ( yM / 8 ) + ( xM / 8 ) ) (G-334)


- Otherwise ((chromaFlag is equal to 1 and RefLayerChromaArrayType is not equal to 3) or
refLayerCTrafo[ refMbAddr ] is not equal to T_8x8), the variable refTransBlkIdc is derived by


refTransBlkIdc = 2 * ( 16 * refMbAddr + 4 * ( yM / 4 ) + ( xM / 4 ) ) (G-335)


**G.8.6.3.3** **Interpolation process for residual prediction**


Inputs to this process are:


- a variable chromaFlag specifying whether the luma or a chroma component is subject to the resampling process,


- two variables mbW and mbH specifying the width and height, respectively, of a macroblock for the considered colour
component,


- a variable fieldMbFlag specifying whether the current macroblock is a field or a frame macroblock,


- a variable botFieldFlag specifying whether a top or a bottom field is subject to the resampling process (when
RefLayerFrameMbsOnlyFlag is equal to 0 or frame_mbs_only_flag is equal to 0),


- a variable fldPrdInFrmMbFlag specifying whether field prediction for a frame macroblock is applied,


- a variable yBorder specifying the vertical border for the output sample array predSamples,


- two variables refArrayW and refArrayH specifying the width and height, respectively, of the array of reference layer
sample values and the array of transform block identifications,


- a (refArrayW)x(refArrayH) array refSampleArray of reference layer sample values,


- a (refArrayW)x(refArrayH) array refTransBlkIdc of transform block identifications,





- two variables xOffset and yOffset specifying the x and y coordinate, respectively, of the reference layer sample
location that corresponds to the sample refSampleArray[ 0, 0 ] of the array refSampleArray and the transform block
identification refTransBlkIdc[ 0, 0 ] of the array refTransBlkIdc.


Output of this process is an (mbW)x(mbH / ( 1 + fldPrdInFrmMbFlag ) + 2 * yBorder) array predArray of interpolated
sample values.


Each sample predArray[ x, y ] with x = 0..(mbW − 1) and y = 0..(mbH / ( 1 + fldPrdInFrmMbFlag ) + 2 *yBorder − 1) is
derived as specified in the following ordered steps:


1. The variable yP is derived by:


yP = ( y − yBorder ) * ( 1 + fldPrdInFrmMbFlag ) + botFieldFlag (G-336)


2. The derivation process for reference layer sample locations in resampling as specified in clause G.6.3 is invoked

with chromaFlag, the sample location ( x, yP ), fieldMbFlag, and botFieldFlag as the inputs and the output is the
sample location ( xRef16, yRef16 ) in units of 1/16-th sample.


3. The variables xRef, yRef, xPhase, and yPhase are derived by:


xRef   = ( xRef16 >> 4 ) − xOffset (G-337)
yRef   = ( yRef16 >> 4 ) − yOffset (G-338)
xPhase = ( xRef16 − 16 * xOffset ) % 16 (G-339)
yPhase = ( yRef16 − 16 * yOffset ) % 16 (G-340)


4. Let tempPred be a one-dimensional array with 2 elements. Each sample value tempPred[ dY ] with dY = 0..1 is

derived as follows:


     - If refTransBlkIdc[ xRef, yRef + dY ] is equal to refTransBlkIdc[ xRef + 1, yRef + dY ], the sample value
tempPred[ dY ] is derived by:


tempPred[ dY ] = ( 16 − xPhase ) * refSampleArray[ xRef, yRef + dY ] +
xPhase  * refSampleArray[ xRef + 1, yRef + dY ] (G-341)


     - Otherwise (refTransBlkIdc[ xRef, yRef + dY ] is not equal to refTransBlkIdc[ xRef + 1, yRef + dY ]), the
sample value tempPred[ dY ] is derived by:


tempPred[ dY ] = ( ( xPhase < 8 ) ? refSampleArray[ xRef, yRef + dY ]
: refSampleArray[ xRef + 1, yRef + dY ] ) << 4 (G-342)


5. With xRefRound set equal to (xRef + ( xPhase / 8 )), the sample value predArray[ x, y ] is derived as follows:


     - If refTransBlkIdc[ xRefRound, yRef ] is equal to refTransBlkIdc[ xRefRound, yRef + 1 ], the sample value
predArray[ x, y ] is derived by:


predArray[ x, y ] = ( ( 16 − yPhase ) * tempPred[ 0 ] +
yPhase  * tempPred[ 1 ] + 128 ) >> 8 (G-343)


     - Otherwise (refTransBlkIdc[ xRefRound, yRef ] is not equal to refTransBlkIdc[ xRefRound, yRef + 1 ]), the
sample value predArray[ x, y ] is derived by:


predArray[ x, y ] = ( ( ( yPhase < 8 ) ? tempPred[ 0 ] : tempPred[ 1 ] ) + 8 ) >> 4 (G-344)


**G.8.6.3.4** **Vertical interpolation process for residual prediction**


Inputs to this process are:


- two variables mbW and mbH specifying the width and height, respectively, of a macroblock for the considered colour
component,


- a variable botFieldFlag specifying whether the sample array fieldPredArray contains interpolated samples for the top
or bottom field,


- a variable yBorder specifying the vertical border for the sample array fieldPredArray,


- a variable frameMbFlag specifying whether the current macroblock is a frame or a field macroblock,


- an (mbW)x(mbH / ( 1 + frameMbFlag ) + 2 * yBorder) array fieldPredArray of sample values.


Output of this process is an (mbW)x(mbH) array predArray of interpolated sample values.


Each sample predArray[ x, y ] with x = 0..(mbW − 1) and y = 0..(mbH − 1) is derived as follows:


- If frameMbFlag is equal to 1 and ( y % 2 ) is equal to botFieldFlag, the sample value predArray[ x, y ] is derived by





predArray[ x, y ] = fieldPredArray[ x, ( y >> 1 ) + yBorder ] (G-345)


- Otherwise (frameMbFlag is equal to 0 or ( y % 2 ) is not equal to botFieldFlag), the sample value predArray[ x, y ] is
derived by


predArray[ x, y ] = ( fieldPredArray[ x, ( y >> frameMbFlag ) + yBorder − botFieldFlag ] +
fieldPredArray[ x, ( y >> frameMbFlag ) + yBorder − botFieldFlag + 1 ] + 1 ) >> 1 (G-346)


**G.8.7** **SVC deblocking filter processes**


Clause G.8.7.1 specifies the deblocking filter process for Intra_Base prediction.


Clause G.8.7.2 specifies the deblocking filter process for target representations.


**G.8.7.1** **Deblocking filter process for Intra_Base prediction**


Inputs to the process are:


- the variable currDQId,


- the collective term currentVars.


Output of this process is a modified version of currentVars.


Let the variable refLayerDQId be equal to the value of the variable MaxRefLayerDQId of the layer representation with
DQId equal to currDQId.


Unless stated otherwise, the syntax elements and derived upper-case variables that are referred to by the process specified
in this clause and all child processes invoked from this process are the syntax elements and derived upper-case variables
for the layer representation with DQId equal to refLayerDQId.


Inside this clause, the arrays that are collectively referred to as currentVars are referred to by their names as specified in
clause G.8.1.2.1.


The derivation process for quantization parameters used in the deblocking filter process as specified in clause G.8.7.3 is
invoked with deblockingDQId set equal to refLayerDQId, mbType, tQPY, and tCoeffLevel as the inputs and the outputs
are a list qpDBY specifying luma quantization parameter that are used in the deblocking filter process and, when
ChromaArrayType is not equal to 0, two lists qpDBCb and qpDBCr specifying chroma quantization parameters that are used
in the deblocking filter process.


Let disableDeblockingFilterIdc, filterOffsetA, and filterOffsetB be equal to the values of
disable_inter_layer_deblocking_filter_idc, InterlayerFilterOffsetA, and InterlayerFilterOffsetB, respectively, for any slice
of the layer representation with DQId equal to currDQId, that has no_inter_layer_pred_flag equal to 0.


For the current macroblock address CurrMbAddr proceeding over values 0..(PicSizeInMbs − 1), the macroblock
deblocking filter process as specified in clause G.8.7.4 is invoked with interLayerDeblockingFlag = 1,
disableDeblockingFilterIdc, filterOffsetA, filterOffsetB, sliceBoundariesOnlyFlag = 0, currentVars, qpDBY and, when
ChromaArrayType is not equal to 0, qpDBCb and qpDBCr as the inputs and the output is a modified version of currentVars.


When disableDeblockingFilterIdc is equal to 3 or 6, for the current macroblock address CurrMbAddr proceeding over
values 0..(PicSizeInMbs − 1), the macroblock deblocking filter process as specified in clause G.8.7.4 is invoked with
interLayerDeblockingFlag = 1, disableDeblockingFilterIdc, filterOffsetA, filterOffsetB, sliceBoundariesOnlyFlag = 1,
currentVars, qpDBY and, when ChromaArrayType is not equal to 0, qpDBCb and qpDBCr as the inputs and the output is a
modified version of currentVars.


**G.8.7.2** **Deblocking filter process for target representations**


Inputs to the process are:


- the variable currDQId,


- the collective term currentVars.


Output of this process is a modified version of currentVars.


Unless stated otherwise, the syntax elements and derived upper-case variables that are referred to by the process specified
in this clause and all child processes invoked from this process are the syntax elements and derived upper-case variables
for the layer representation with DQId equal to currDQId.


Inside this clause, the arrays that are collectively referred to as currentVars are referred to by their names as specified in
clause G.8.1.2.1.





The derivation process for quantization parameters used in the deblocking filter process as specified in clause G.8.7.3 is
invoked with deblockingDQId set equal to currDQId, mbType, tQPY, and tCoeffLevel as the inputs and the outputs are a
list qpDBY specifying luma quantization parameter that are used in the deblocking filter process and, when
ChromaArrayType is not equal to 0, two lists qpDBCb and qpDBCr specifying chroma quantization parameters that are used
in the deblocking filter process.


For the current macroblock address CurrMbAddr proceeding over values 0..(PicSizeInMbs − 1), the following ordered
steps are specified:


1. Let disableDeblockingFilterIdc, filterOffsetA, and filterOffsetB be equal to the value of
disable_deblocking_filter_idc, FilterOffsetA, and FilterOffsetB, respectively, for the slice with DQId equal to
(sliceIdc[ CurrMbAddr ] & 127) and first_mb_in_slice equal to (sliceIdc[ CurrMbAddr ] >> 7).


2. The macroblock deblocking filter process as specified in clause G.8.7.4 is invoked with
interLayerDeblockingFlag = 0, disableDeblockingFilterIdc, filterOffsetA, filterOffsetB,
sliceBoundariesOnlyFlag = 0, currentVars, qpDBY and, when ChromaArrayType is not equal to 0, qpDBCb and
qpDBCr as the inputs and the output is a modified version of currentVars.


For the current macroblock address CurrMbAddr proceeding over values 0..(PicSizeInMbs − 1), the following ordered
steps are specified:


1. Let disableDeblockingFilterIdc, filterOffsetA, and filterOffsetB be equal to the value of
disable_deblocking_filter_idc, FilterOffsetA, and FilterOffsetB, respectively, for the slice with DQId equal to
(sliceIdc[ CurrMbAddr ] & 127) and first_mb_in_slice equal to (sliceIdc[ CurrMbAddr ] >> 7).


2. When disableDeblockingFilterIdc is equal to 3 or 6, the macroblock deblocking filter process as specified in

clause G.8.7.4 is invoked with interLayerDeblockingFlag = 0, disableDeblockingFilterIdc, filterOffsetA,
filterOffsetB, sliceBoundariesOnlyFlag = 1, currentVars, qpDBY and, when ChromaArrayType is not equal to 0,
qpDBCb and qpDBCr as inputs and the output is a modified version of currentVars.


**G.8.7.3** **Derivation process for quantization parameters used in the deblocking filter process**


Inputs to this process are:


- a variable deblockingDQId,


- a one-dimensional array mbType with PicSizeInMbs elements specifying macroblock types for the macroblocks of
the current decoded or partly decoded dependency representation,


- a one-dimensional array tQPY with PicSizeInMbs elements specifying luma quantization parameters for the
macroblocks of the current decoded or partly decoded dependency representation,


- a (PicSizeInMbs)x(256 + 2 * MbWidthC * MbHeightC) array tCoeffLevel specifying transform coefficient level
values for the macroblocks of the current decoded or partly decoded dependency representation.


Outputs of this process are:


- a one-dimensional array qpDBY with PicSizeInMbs elements specifying luma quantization parameters used in the
deblocking filter process for the macroblocks of the current decoded or partly decoded dependency representation,


- when ChromaArrayType is not equal to 0, two one-dimensional arrays qpDBCb and qpDBCr with PicSizeInMbs
elements specifying chroma quantization parameters used in the deblocking filter process for the macroblocks of the
current decoded or partly decoded dependency representation.


The syntax elements and derived upper-case variables that are referred to by the process specified in this clause are the
syntax elements and derived upper-case variables for the layer representation with DQId equal to deblockingDQId.


Let tempQP be a one-dimensional array with PicSizeInMbs elements. All elements tempQP[ i ] with
i = 0..(PicSizeInMbs − 1) are set equal to tQPY[ i ].


When MaxTCoeffLevelPredFlag is equal to 1, the following ordered steps are specified:


1. Let firstMbInSliceGroup and numMbsInSliceGroup be two one-dimensional arrays with
(num_slice_groups_minus1 + 1) elements. The array elements are derived as specified by the following pseudo
code.


for( iGroup = 0; iGroup <= num_slice_groups_minus1; iGroup++ ) {
firstMbInSliceGroup[ iGroup ] = −1
numMbsInSliceGroup[ iGroup ] = 0
}
for( i = 0; i < PicSizeInMbs; i++ ) { (G-347)





if( firstMbInSliceGroup[ MbToSliceGroupMap[ i ] ] = = −1 )
firstMbInSliceGroup[ MbToSliceGroupMap[ i ] ] = i
numMbsInSliceGroup[ MbToSliceGroupMap[ i ] ]++
}


2. The variable iGroup proceeds over the values 0..num_slice_groups_minus1. For each value of iGroup, the

variable lastMbAddr is set equal to firstMbInSliceGroup[ iGroup ] and the variable mbIdx proceeds over the
values 1..(numMbsInSliceGroup[ iGroup ] − 1). For each value of mbIdx, the following ordered steps are
specified.


a. The variable mbAddr is derived as specified by the following pseudo-code:


mbAddr = lastMbAddr + 1
while( MbToSliceGroupMap[ mbAddr ] != MbToSliceGroupMap[ lastMbAddr ] ) (G-348)
mbAddr++


b. When mbType[ mbAddr ] is not equal to I_16x16 and all elements tCoeffLevel[ mbAddr ][ i ] with

i = 0..(255 + 2 * MbWidthC * MbHeightC) are equal to 0, tempQP[ mbAddr ] is set equal to
tempQP[ lastMbAddr ].


c. The variable lastMbAddr is set equal to mbAddr.


The macroblock address mbAddr proceeds over the values 0..(PicSizeInMbs − 1), and for each value of mbAddr, the
following ordered steps are specified:


1. The variable qpDBY[ mbAddr ] is derived as follows:


     - If mbType[ mbAddr ] is equal to I_PCM, qpDBY[ mbAddr ] is set equal to 0.


     - Otherwise (mbType[ mbAddr ] is not equal to I_PCM), qpDBY[ mbAddr ] is set equal to
tempQP[ mbAddr ].


2. When ChromaArrayType is not equal to 0, for C being replaced by Cb and Cr, the variable qpDBC[ mbAddr ] is

set equal to the value of QPC that corresponds to a value of qpDBY[ mbAddr ] for QPY as specified in clause 8.5.8.
During this invocation of the process in clause 8.5.8, the syntax elements chroma_qp_index_offset and
second_chroma_qp_index_offset of the layer representation with DQId equal to deblockingFilterDQId are used.


**G.8.7.4** **Macroblock deblocking filter process**


Inputs to this process are:


- the variables interLayerDeblockingFlag, disableDeblockingFilterIdc, filterOffsetA, filterOffsetB, and
sliceBoundariesOnlyFlag,


- the collective term currentVars,


- a one-dimensional array qpDBY with PicSizeInMbs elements specifying luma quantization parameters used in the
deblocking filter process for the macroblocks of the current decoded or partly decoded dependency representation,


- when ChromaArrayType is not equal to 0, two one-dimensional arrays qpDBCb and qpDBCr with PicSizeInMbs
elements specifying chroma quantization parameters used in the deblocking filter process for the macroblocks of the
current decoded or partly decoded dependency representation.


Output of this process is a modified version of currentVars.


In the following of this clause, the arrays that are collectively referred to as currentVars are referred to by their names as
specified in clause G.8.1.2.1.


The derivation process for neighbouring macroblocks specified in clause 6.4.11.1 is invoked and the output is assigned to
mbAddrA and mbAddrB. For this invocation of the process in clause 6.4.11.1, the current macroblock is treated as field
macroblock when fieldMbFlag[ CurrMbAddr ] is equal to 1, and it is treated as frame macroblock when
fieldMbFlag[ CurrMbAddr ] is equal to 0.

NOTE 1 – The availability status of the macroblocks mbAddrA and mbAddrB is not used inside this clause. Slice boundaries are
detected using the array sliceIdc.


The variable filterLeftLumaMbEdgeFlag is derived as follows:


- If any of the following conditions are true, the variable filterLeftLumaMbEdgeFlag is set equal to 0:





   - MbaffFrameFlag is equal to 0 and CurrMbAddr % PicWidthInMbs is equal to 0,


   - MbaffFrameFlag is equal to 1 and ( CurrMbAddr >> 1 ) % PicWidthInMbs is equal to 0,


   - disableDeblockingFilterIdc is equal to 1,


   - disableDeblockingFilterIdc is equal to 2 or 5 and sliceIdc[ mbAddrA ] is different than sliceIdc[ CurrMbAddr ],


   - disableDeblockingFilterIdc is equal to 3 or 6, sliceBoundariesOnlyFlag is equal to 0, and sliceIdc[ mbAddrA ]
is different than sliceIdc[ CurrMbAddr ],


   - disableDeblockingFilterIdc is equal to 3 or 6, sliceBoundariesOnlyFlag is equal to 1, and sliceIdc[ mbAddrA ]
is equal to sliceIdc[ CurrMbAddr ],


   - interLayerDeblockingFlag is equal to 1 and mbType[ CurrMbAddr ] specifies an Inter macroblock prediction
mode.


- Otherwise, the variable filterLeftLumaMbEdgeFlag is set equal to 1.


The variable filterTopLumaMbEdgeFlag is derived as follows:


- If any of the following conditions are true, the variable filterTopLumaMbEdgeFlag is set equal to 0:


   - MbaffFrameFlag is equal to 0 and CurrMbAddr is less than PicWidthInMbs,


   - MbaffFrameFlag is equal to 1, ( CurrMbAddr >> 1 ) is less than PicWidthInMbs, and
fieldMbFlag[ CurrMbAddr ] is equal to 1,


   - MbaffFrameFlag is equal to 1, ( CurrMbAddr >> 1 ) is less than PicWidthInMbs, fieldMbFlag[ CurrMbAddr ]
is equal to 0, and CurrMbAddr % 2 is equal to 0,


   - disableDeblockingFilterIdc is equal to 1,


   - disableDeblockingFilterIdc is equal to 2 or 5 and sliceIdc[ mbAddrB ] is different than sliceIdc[ CurrMbAddr ],


   - disableDeblockingFilterIdc is equal to 3 or 6, sliceBoundariesOnlyFlag is equal to 0, and sliceIdc[ mbAddrB ]
is different than sliceIdc[ CurrMbAddr ],


   - disableDeblockingFilterIdc is equal to 3 or 6, sliceBoundariesOnlyFlag is equal to 1, and sliceIdc[ mbAddrB ]
is equal to sliceIdc[ CurrMbAddr ],


   - interLayerDeblockingFlag is equal to 1 and mbType[ CurrMbAddr ] specifies an Inter macroblock prediction
mode.


- Otherwise, the variable filterTopLumaMbEdgeFlag is set equal to 1.


The variable filterInternalLumaEdgesFlag is derived as follows:


- If any of the following conditions are true, the variable filterInternalLumaEdgesFlag is set equal to 0:


   - disableDeblockingFilterIdc is equal to 1,


   - disableDeblockingFilterIdc is equal to 3 or 6 and sliceBoundariesOnlyFlag is equal to 1,


   - interLayerDeblockingFlag is equal to 1 mbType[ CurrMbAddr ] specifies an Inter macroblock prediction mode.


- Otherwise the variable filterInternalLumaEdgesFlag is set equal to 1.


The variables filterLeftChromaMbEdgeFlag, filterTopChromaMbEdgeFlag, and filterInternalChromaEdgesFlag are
derived as follows:


- If disableDeblockingFilterIdc is greater than 3, filterLeftChromaMbEdgeFlag, filterTopChromaMbEdgeFlag, and
filterInternalChromaEdgesFlag are set equal to 0.


- Otherwise (disableDeblockingFilterIdc is less than 4), filterLeftChromaMbEdgeFlag, filterTopChromaMbEdgeFlag,
and filterInternalChromaEdgesFlag are set equal to filterLeftLumaMbEdgeFlag, filterTopLumaMbEdgeFlag, and
filterInternalLumaEdgesFlag, respectively.


The variable fieldMbInFrameFlag is derived as follows:


- If MbaffFrameFlag is equal to 1 and fieldMbFlag[ CurrMbAddr ] is equal to 1, fieldMbInFrameFlag is set equal to 1.


- Otherwise (MbaffFrameFlag is equal to 0 or fieldMbFlag[ CurrMbAddr ] is equal to 0), fieldMbInFrameFlag is set
equal to 0.





When filterLeftLumaMbEdgeFlag is equal to 1, the left vertical luma edge is filtered by invoking the process specified in
clause G.8.7.4.1 with interLayerDeblockingFlag, chromaEdgeFlag set equal to 0, verticalEdgeFlag set equal to 1,
fieldModeInFrameFilteringFlag set equal to fieldMbInFrameFlag, filterOffsetA, filterOffsetB, qpDB set equal to qpDBY,
currentVars, (xEk, yEk) set equal to (0, k) with k = 0..15, and cSL as the inputs and cSL as the output.


When filterInternalLumaEdgesFlag is equal to 1, the filtering of the internal vertical luma edges is specified by the
following ordered steps:


1. When cTrafo[ CurrMbAddr ] is not equal to T_8x8, the process specified in clause G.8.7.4.1 is invoked with

interLayerDeblockingFlag, chromaEdgeFlag set equal to 0, verticalEdgeFlag set equal to 1,
fieldModeInFrameFilteringFlag set equal to fieldMbInFrameFlag, filterOffsetA, filterOffsetB, qpDB set equal to
qpDBY, currentVars, (xEk, yEk) set equal to (4, k) with k = 0..15, and cSL as the inputs and cSL as the output.


2. The process specified in clause G.8.7.4.1 is invoked with interLayerDeblockingFlag, chromaEdgeFlag set equal

to 0, verticalEdgeFlag set equal to 1, fieldModeInFrameFilteringFlag set equal to fieldMbInFrameFlag,
filterOffsetA, filterOffsetB, qpDB set equal to qpDBY, currentVars, (xEk, yEk) set equal to (8, k) with k = 0..15,
and cSL as the inputs and cSL as the output.


3. When cTrafo[ CurrMbAddr ] is not equal to T_8x8, the process specified in clause G.8.7.4.1 is invoked with

interLayerDeblockingFlag, chromaEdgeFlag set equal to 0, verticalEdgeFlag set equal to 1,
fieldModeInFrameFilteringFlag set equal to fieldMbInFrameFlag, filterOffsetA, filterOffsetB, qpDB set equal to
qpDBY, currentVars, (xEk, yEk) set equal to (12, k) with k = 0..15, and cSL as the inputs and cSL as the output.


When filterTopLumaMbEdgeFlag is equal to 1, the filtering of the top horizontal luma edge is specified as follows:


- If MbaffFrameFlag is equal to 1, (CurrMbAddr % 2) is equal to 0, CurrMbAddr is greater than or equal to
(2 * PicWidthInMbs), fieldMbFlag[ CurrMbAddr ] is equal to 0, and
fieldMbFlag[ CurrMbAddr − 2 * PicWidthInMbs + 1 ] is equal to 1, the following ordered steps are specified:


1. The process specified in clause G.8.7.4.1 is invoked with interLayerDeblockingFlag, chromaEdgeFlag set equal

to 0, verticalEdgeFlag set equal to 0, fieldModeInFrameFilteringFlag set equal to 1, filterOffsetA, filterOffsetB,
qpDB set equal to qpDBY, currentVars, (xEk, yEk) set equal to (k, 0) with k = 0..15, and cSL as the inputs and cSL
as the output.


2. The process specified in clause G.8.7.4.1 is invoked with interLayerDeblockingFlag, chromaEdgeFlag set equal

to 0, verticalEdgeFlag set equal to 0, fieldModeInFrameFilteringFlag set equal to 1, filterOffsetA, filterOffsetB,
qpDB set equal to qpDBY, currentVars, (xEk, yEk) set equal to (k, 1) with k = 0..15, and cSL as the inputs and cSL
as the output.


- Otherwise, the process specified in clause G.8.7.4.1 is invoked with interLayerDeblockingFlag, chromaEdgeFlag set
equal to 0, verticalEdgeFlag set equal to 0, fieldModeInFrameFilteringFlag set equal to fieldMbInFrameFlag,
filterOffsetA, filterOffsetB, qpDB set equal to qpDBY, currentVars, (xEk, yEk) set equal to (k, 0) with k = 0..15, and
cSL as the inputs and cSL as the output.


When filterInternalLumaEdgesFlag is equal to 1, the filtering of the internal horizontal luma edges is specified by the
following ordered steps:


1. When cTrafo[ CurrMbAddr ] is not equal to T_8x8, the process specified in clause G.8.7.4.1 is invoked with

interLayerDeblockingFlag, chromaEdgeFlag set equal to 0, verticalEdgeFlag set equal to 0,
fieldModeInFrameFilteringFlag set equal to fieldMbInFrameFlag, filterOffsetA, filterOffsetB, qpDB set equal to
qpDBY, currentVars, (xEk, yEk) set equal to (k, 4) with k = 0..15, and cSL as the inputs and cSL as the output.


2. The process specified in clause G.8.7.4.1 is invoked with interLayerDeblockingFlag, chromaEdgeFlag set equal

to 0, verticalEdgeFlag set equal to 0, fieldModeInFrameFilteringFlag set equal to fieldMbInFrameFlag,
filterOffsetA, filterOffsetB, qpDB set equal to qpDBY, currentVars, (xEk, yEk) set equal to (k, 8) with k = 0..15,
and cSL as the inputs and cSL as the output.


3. When cTrafo[ CurrMbAddr ] is not equal to T_8x8, the process specified in clause G.8.7.4.1 is invoked with

interLayerDeblockingFlag, chromaEdgeFlag set equal to 0, verticalEdgeFlag set equal to 0,
fieldModeInFrameFilteringFlag set equal to fieldMbInFrameFlag, filterOffsetA, filterOffsetB, qpDB set equal to
qpDBY, currentVars, (xEk, yEk) set equal to (k, 12) with k = 0..15, and cSL as the inputs and cSL as the output.


When ChromaArrayType is not equal to 0, for the filtering of both chroma components with C being replaced by Cb and
Cr in qpDBC and cSC, the following ordered steps are specified:


1. When filterLeftChromaMbEdgeFlag is equal to 1, the left vertical chroma edge is filtered by invoking the process

specified in clause G.8.7.4.1 with interLayerDeblockingFlag, chromaEdgeFlag set equal to 1, verticalEdgeFlag
set equal to 1, fieldModeInFrameFilteringFlag set equal to fieldMbInFrameFlag, filterOffsetA, filterOffsetB,





qpDB set equal to qpDBC, currentVars, (xEk, yEk) set equal to (0, k) with k = 0..(MbHeightC − 1), and cSC as the
inputs and cSC as the output.


2. When filterInternalChromaEdgesFlag is equal to 1, the filtering of the internal vertical chroma edge is specified

by the following ordered steps:


a. When ChromaArrayType is not equal to 3 or cTrafo[ CurrMbAddr ] is not equal to T_8x8, the process
specified in clause G.8.7.4.1 is invoked with interLayerDeblockingFlag, chromaEdgeFlag set equal to 1,
verticalEdgeFlag set equal to 1, fieldModeInFrameFilteringFlag set equal to fieldMbInFrameFlag,
filterOffsetA, filterOffsetB, qpDB set equal to qpDBC, currentVars, (xEk, yEk) set equal to (4, k) with
k = 0..(MbHeightC − 1), and cSC as the inputs and cSC as the output.


b. When ChromaArrayType is equal to 3, the process specified in clause G.8.7.4.1 is invoked with

interLayerDeblockingFlag, chromaEdgeFlag set equal to 1, verticalEdgeFlag set equal to 1,
fieldModeInFrameFilteringFlag set equal to fieldMbInFrameFlag, filterOffsetA, filterOffsetB, qpDB set
equal to qpDBC, currentVars, (xEk, yEk) set equal to (8, k) with k = 0..(MbHeightC − 1), and cSC as the
inputs and cSC as the output.


c. When ChromaArrayType is equal to 3 and cTrafo[ CurrMbAddr ] is not equal to T_8x8, the process
specified in clause G.8.7.4.1 is invoked with interLayerDeblockingFlag, chromaEdgeFlag set equal to 1,
verticalEdgeFlag set equal to 1, fieldModeInFrameFilteringFlag set equal to fieldMbInFrameFlag,
filterOffsetA, filterOffsetB, qpDB set equal to qpDBC, currentVars, (xEk, yEk) set equal to (12, k) with
k = 0..(MbHeightC − 1), and cSC as the inputs and cSC as the output.


3. When filterTopChromaMbEdgeFlag is equal to 1, the filtering of the top horizontal chroma edge is specified as

follows:


     - If MbaffFrameFlag is equal to 1, (CurrMbAddr % 2) is equal to 0, CurrMbAddr is greater than or equal to
(2 * PicWidthInMbs), fieldMbFlag[ CurrMbAddr ] is equal to 0,
fieldMbFlag[ CurrMbAddr − 2 * PicWidthInMbs + 1 ] is equal to 1, the following ordered steps are
specified:


a. The process specified in clause G.8.7.4.1 is invoked with interLayerDeblockingFlag, chromaEdgeFlag
set equal to 1, verticalEdgeFlag set equal to 0, fieldModeInFrameFilteringFlag set equal to 1,
filterOffsetA, filterOffsetB, qpDB set equal to qpDBC, currentVars, (xEk, yEk) set equal to (k, 0) with
k = 0..(MbWidthC − 1), and cSC as the inputs and cSC as the output.


b. The process specified in clause G.8.7.4.1 is invoked with interLayerDeblockingFlag, chromaEdgeFlag

set equal to 1, verticalEdgeFlag set equal to 0, fieldModeInFrameFilteringFlag set equal to 1,
filterOffsetA, filterOffsetB, qpDB set equal to qpDBC, currentVars, (xEk, yEk) set equal to (k, 1) with
k = 0..(MbWidthC − 1), and cSC as the inputs and cSC as the output.


     - Otherwise, the process specified in clause G.8.7.4.1 is invoked with interLayerDeblockingFlag,
chromaEdgeFlag set equal to 1, verticalEdgeFlag set equal to 0, fieldModeInFrameFilteringFlag set equal
to fieldMbInFrameFlag, filterOffsetA, filterOffsetB, qpDB set equal to qpDBC, currentVars, (xEk, yEk) set
equal to (k, 0) with k = 0..(MbWidthC − 1), and cSC as the inputs and cSC as the output.


4. When filterInternalChromaEdgesFlag is equal to 1, the filtering of the internal horizontal chroma edge is

specified by the following ordered steps:


a. When ChromaArrayType is not equal to 3 or cTrafo[ CurrMbAddr ] is not equal to T_8x8, the process
specified in clause G.8.7.4.1 is invoked with interLayerDeblockingFlag, chromaEdgeFlag set equal to 1,
verticalEdgeFlag set equal to 0, fieldModeInFrameFilteringFlag set equal to fieldMbInFrameFlag,
filterOffsetA, filterOffsetB, qpDB set equal to qpDBC, currentVars, (xEk, yEk) set equal to (k, 4) with
k = 0..(MbWidthC − 1), and cSC as the inputs and cSC as the output.


b. When ChromaArrayType is not equal to 1, the process specified in clause G.8.7.4.1 is invoked with

interLayerDeblockingFlag, chromaEdgeFlag set equal to 1, verticalEdgeFlag set equal to 0,
fieldModeInFrameFilteringFlag set equal to fieldMbInFrameFlag, filterOffsetA, filterOffsetB, qpDB set
equal to qpDBC, currentVars, (xEk, yEk) set equal to (k, 8) with k = 0..(MbWidthC − 1), and cSC as the inputs
and cSC as the output.


c. When ChromaArrayType is equal to 2, the process specified in clause G.8.7.4.1 is invoked with
interLayerDeblockingFlag, chromaEdgeFlag set equal to 1, verticalEdgeFlag set equal to 0,
fieldModeInFrameFilteringFlag set equal to fieldMbInFrameFlag, filterOffsetA, filterOffsetB, qpDB set
equal to qpDBC, currentVars, (xEk, yEk) set equal to (k, 12) with k = 0..(MbWidthC − 1), and cSC as the
inputs and cSC as the output.





d. When ChromaArrayType is equal to 3 and cTrafo[ CurrMbAddr ] is not equal to T_8x8, the process

specified in clause G.8.7.4.1 is invoked with interLayerDeblockingFlag, chromaEdgeFlag set equal to 1,
verticalEdgeFlag set equal to 0, fieldModeInFrameFilteringFlag set equal to fieldMbInFrameFlag,
filterOffsetA, filterOffsetB, qpDB set equal to qpDBC, currentVars, (xEk, yEk) set equal to (k, 12) with
k = 0..(MbWidthC − 1), and cSC as the inputs and cSC as the output.
NOTE 2 – When field mode filtering (fieldModeInFrameFilteringFlag is equal to 1) is applied across the top horizontal edges
of a frame macroblock, this vertical filtering across the top or bottom macroblock boundary may involve some samples that
extend across an internal block edge that is also filtered internally in frame mode.
NOTE 3 – For example, in 4:2:0 chroma format when cTrafo[ CurrMbAddr ] is not equal to T_8x8, the following applies. 3
horizontal luma edges, 1 horizontal chroma edge for Cb, and 1 horizontal chroma edge for Cr are filtered that are internal to
a macroblock. When field mode filtering (fieldModeInFrameFilteringFlag is equal to 1) is applied to the top edges of a frame
macroblock, 2 horizontal luma, 2 horizontal chroma edges for Cb, and 2 horizontal chroma edges for Cr between the frame
macroblock and the above macroblock pair are filtered using field mode filtering, for a total of up to 5 horizontal luma edges,
3 horizontal chroma edges for Cb, and 3 horizontal chroma edges for Cr filtered that are considered to be controlled by the
frame macroblock. In all other cases, at most 4 horizontal luma, 2 horizontal chroma edges for Cb, and 2 horizontal chroma
edges for Cr are filtered that are considered to be controlled by a particular macroblock.


**G.8.7.4.1** **SVC filtering process for block edges**


Inputs to this process are:


- the variable interLayerDeblockingFlag,


- the variable chromaEdgeFlag,


- the variable verticalEdgeFlag,


- the variable fieldModeInFrameFilteringFlag,


- the variables filterOffsetA and filterOffsetB,


- the one-dimensional array qpDB with PicSizeInMbs elements specifying quantization parameters,


- the collective term currentVars,


- a set of nE sample locations (xEk, yEk), with k = 0..(nE − 1), expressed relative to the upper left corner of the
macroblock CurrMbAddr. The set of sample locations (xEk, yEk) represent the sample locations immediately to the
right of a vertical edge (when verticalEdgeFlag is equal to 1) or immediately below a horizontal edge (when
verticalEdgeFlag is equal to 0),


- an array of samples s′.


Output of this process is a modified version of the array s′.


The variable nE is derived as follows:


- If chromaEdgeFlag is equal to 0, nE is set equal to 16.


- Otherwise (chromaEdgeFlag is equal to 1), nE is set equal to
( ( verticalEdgeFlag = = 1 ) ? MbHeightC : MbWidthC ).


Inside this clause, the arrays that are collectively referred to as currentVars are referred to by their names as specified in
clause G.8.1.2.1.


The variable dy is set equal to (1 + fieldModeInFrameFilteringFlag).


The position of the upper-left luma sample of the macroblock CurrMbAddr is derived by invoking the inverse macroblock
scanning process in clause 6.4.1 with mbAddr = CurrMbAddr as input and the output being assigned to ( xI, yI ). During
the process in clause 6.4.1, the current macroblock is treated as field macroblock when fieldMbFlag[ CurrMbAddr ] is
equal to 1, and it is treated as frame macroblock when fieldMbFlag[ CurrMbAddr ] is equal to 0.


The variables xP and yP are derived as follows:


- If chromaEdgeFlag is equal to 0, xP is set equal to xI and yP is set equal to yI.


- Otherwise (chromaEdgeFlag is equal to 1), xP is set equal to (xI / SubWidthC) and yP is set equal to
( ( yI + SubHeightC − 1 ) / SubHeightC ).


For each sample location ( xEk, yEk ), k = 0..(nE − 1), the following ordered steps are specified:


1. The filtering process is applied to a set of eight samples across a 4x4 block horizontal or vertical edge denoted as

pi and qi with i = 0..3 as shown in Figure 8-11 with the edge lying between p0 and q0. pi and qi with i = 0..3 are
specified as follows:





    - If verticalEdgeFlag is equal to 1,


qi = s′[ xP + xEk + i, yP + dy * yEk ] (G-349)
pi = s′[ xP + xEk − i − 1, yP + dy * yEk ] (G-350)


    - Otherwise (verticalEdgeFlag is equal to 0),


qi = s′[ xP + xEk, yP + dy * ( yEk + i ) − (yEk % 2 ) ] (G-351)
pi = s′[ xP + xEk, yP + dy * ( yEk − i − 1 ) − (yEk % 2 ) ] (G-352)


2. Let mbAddrP and mbAddrQ specify the addresses of the macroblocks that contain the samples p0 and q0,

respectively.


3. The process specified in clause G.8.7.4.2 is invoked with the sample values pi and qi (i = 0..3),

interLayerDeblockingFlag, chromaEdgeFlag, verticalEdgeFlag, filterOffsetA, filterOffsetB, qPp set equal to
qpDB[ mbAddrP ], qPq set equal to qpDB[ mbAddrQ ], sliceIdc, fieldMbFlag, mbType, cTrafo, predFlagL0,
predFlagL1, refIdxL0, refIdxL1, mvL0, mvL1, and rSL as inputs, and the output is assigned to the filtered result
sample values p′i and q′i with i = 0..2.


4. The input sample values pi and qi with i = 0..2 are replaced by the corresponding filtered result sample values p′i

and q′i with i = 0..2 inside the sample array s′ as follows:


    - If verticalEdgeFlag is equal to 1,


s′[ xP + xEk + i, yP + dy * yEk ] = q′i (G-353)
s′[ xP + xEk − i − 1, yP + dy * yEk ] = p′i (G-354)


    - Otherwise (verticalEdgeFlag is equal to 0),


s′[ xP + xEk, yP + dy * ( yEk + i ) − ( yEk % 2 ) ] = q′i (G-355)
s′[ xP + xEk, yP + dy * ( yEk − i − 1 ) − ( yEk % 2 ) ] = p′i (G-356)


**G.8.7.4.2** **SVC filtering process for a set of samples across a horizontal or vertical block edge**


Inputs to this process are:


- the input sample values pi and qi with i = 0..3 of a single set of samples across an edge that is to be filtered,


- the variable interLayerDeblockingFlag,


- the variable chromaEdgeFlag,


- the variable verticalEdgeFlag,


- the variables filterOffsetA and filterOffsetB,


- the variables qPp and qPq,


- the arrays sliceIdc, fieldMbFlag, mbType, cTrafo, predFlagL0, predFlagL1, refIdxL0, refIdxL1, mvL0, and mvL1,


- an array rSL containing residual sample values.


Outputs of this process are the filtered result sample values p′i and q′i with i in the range of 0..2.


The content dependent boundary filtering strength variable bS is derived as follows:


- If chromaEdgeFlag is equal to 0, the SVC derivation process for the luma content dependent boundary filtering
strength specified in clause G.8.7.4.3 is invoked with p0, q0, interLayerDeblockingFlag, verticalEdgeFlag, sliceIdc,
fieldMbFlag, mbType, cTrafo, predFlagL0, predFlagL1, refIdxL0, refIdxL1, mvL0, mvL1, and rSL as inputs, and the
output is assigned to bS.


- Otherwise (chromaEdgeFlag is equal to 1), the bS used for filtering a set of samples of a horizontal or vertical chroma
edge is set equal to the value of bS for filtering the set of samples of a horizontal or vertical luma edge, respectively,
that contains the luma sample at location ( SubWidthC * x, SubHeightC * y ) inside the luma array of the same field,
where ( x, y ) is the location of the chroma sample q0 inside the chroma array for that field.


The process specified in clause 8.7.2.2 is invoked with p0, q0, p1, q1, chromaEdgeFlag, bS, filterOffsetA, filterOffsetB, qPp,
and qPq as inputs, and the output is assigned to filterSamplesFlag, indexA, , and  .


Depending on the variable filterSamplesFlag, the following applies:





- If filterSamplesFlag is equal to 1, the following applies:


   - If bS is less than 4, the process specified in clause 8.7.2.3 is invoked with pi and qi (i = 0..2), chromaEdgeFlag,
bS, , and indexA given as input, and the output is assigned to p′i and q′i (i = 0..2).


   - Otherwise (bS is equal to 4), the process specified in clause 8.7.2.4 is invoked with pi and qi (i = 0..3),
chromaEdgeFlag, , and  given as input, and the output is assigned to p′i and q′i (i = 0..2).


- Otherwise (filterSamplesFlag is equal to 0), the filtered result samples p′i and q′i (i = 0..2) are replaced by the
corresponding input samples pi and qi:


for i = 0..2, p′i = pi (G-357)
for i = 0..2, q′i = qi (G-358)


**G.8.7.4.3** **SVC derivation process for the luma content dependent boundary filtering strength**


Inputs to this process are:


- the input sample values p0 and q0 of a single set of samples across an edge that is to be filtered,


- the variable interLayerDeblockingFlag,


- the variable verticalEdgeFlag,


- the arrays sliceIdc, fieldMbFlag, mbType, cTrafo, predFlagL0, predFlagL1, refIdxL0, refIdxL1, mvL0, and mvL1,


- the array rSL containing residual sample values.


Output of this process is the variable bS.


The following variables are derived as specified in the following:


- mbAddrP and mbAddrQ specify the macroblocks containing the samples p0 and q0, respectively.


- mbPartIdxP and mbPartIdxQ specify the macroblock partitions containing the samples p0 and q0, respectively.


- subMbPartIdxP and subMbPartIdxQ specify the sub-macroblock partitions containing the samples p0 and q0,
respectively.


- pFLXP and pFLXQ with X being replaced by 0 and 1 are equal to predFlagLX[ mbAddrP ][ mbPartIdxP ] and
predFlagLX[ mbAddrQ ][ mbPartIdxQ ], respectively.


- refLXP and refLXQ with X being replaced by 0 and 1 are equal to refIdxLX[ mbAddrP ][ mbPartIdxP ] and
refIdxLX[ mbAddrQ ][ mbPartIdxQ ], respectively.


- mvLXP and mvLXQ with X being replaced by 0 and 1 are equal to mvLX[ mbAddrP ][ mbPartIdxP ][ subMbPartP ]
and mvLX[ mbAddrQ ][ mbPartIdxQ ][ subMbPartQ ], respectively.


- numMvP and numMvQ are equal to (pFL0P + pFL1P) and (pFL0Q + pFL1Q), respectively.


- When numMvP and numMvQ are both equal to 1, the variables refX and mvX with X being replaced by P and Q are
derived as follows:


   - If pFL0X is equal to 1, refX is set equal to refL0X and mvX is set equal to mvL0X.


   - Otherwise (pFL1X is equal to 1), refX is set equal to refL1X and mvX is set equal to mvL1X.


- sliceX with X being replaced by P and Q is the slice with DQId equal to (sliceIdc[ mbAddrX ] & 127) and
first_mb_in_slice equal to (sliceIdc[ mbAddrX ] >> 7)


Let the variable mixedModeEdgeFlag be derived as follows:


- If MbaffFrameFlag is equal to 1 and fieldMbFlag[ mbAddrP ] is not equal to fieldMbFlag[ mbAddrQ ],
mixedModeEdgeFlag is set equal to 1.


- Otherwise, mixedModeEdgeFlag is set equal to 0.


The variable bS is derived as follows:


- If interLayerDeblockingFlag is equal to 1 and mbType[ mbAddrP ] specifies an Inter macroblock prediction mode,
bS is set equal to 0.

NOTE 1 – This clause is not invoked when interLayerDeblockingFlag is equal to 1 and mbType[ mbAddrQ ]
specifies an Inter macroblock prediction mode.





- Otherwise, if SpatialResolutionChangeFlag is equal to 1 and either or both mbType[ mbAddrP ] or
mbType[ mbAddrQ ] is equal to I_BL, the following applies:


   - If either mbType[ mbAddrP ] or mbType[ mbAddrQ ] specifies an Intra macroblock prediction mode other than
I_BL, the following applies:


      - If verticalEdgeFlag is equal to 1 or both fieldMbFlag[ mbAddrP ] and fieldMbFlag[ mbAddrQ ] are equal
to 0, bS is set equal to 4.


      - Otherwise (verticalEdgeFlag is equal to 0 and either or both fieldMbFlag[ mbAddrP ] or
fieldMbFlag[ mbAddrQ ] is equal to 1), bS is set equal to 3.


   - Otherwise, if mbType[ mbAddrP ] is equal to I_BL and mbType[ mbAddrQ ] is equal to I_BL, the following
applies:


      - If any of the following conditions are true, bS is set equal to 1:


        - cTrafo[ mbAddrP ] is equal to T_8x8 and the 8x8 luma transform block coded in sliceP and associated
with the 8x8 luma block containing sample p0 contains non-zero transform coefficient levels,


        - cTrafo[ mbAddrP ] is equal to T_4x4 and the 4x4 luma transform block coded in sliceP and associated
with the 4x4 luma block containing sample p0 contains non-zero transform coefficient levels,


        - cTrafo[ mbAddrQ ] is equal to T_8x8 and the 8x8 luma transform block coded in sliceQ and associated
with the 8x8 luma block containing sample q0 contains non-zero transform coefficient levels,


        - cTrafo[ mbAddrQ ] is equal to T_4x4 and the 4x4 luma transform block coded in sliceQ and associated
with the 4x4 luma block containing sample q0 contains non-zero transform coefficient levels.
NOTE 2 – A luma transform block coded in a particular slice is considered to contain non-zero transform coefficient
levels, if non-zero transform coefficients are transmitted in the macroblock layer of the slice for the considered luma
transform block. Transform coefficient levels that are transmitted in layers that are used for inter-layer prediction
are not taken into account.


      - Otherwise, bS is set equal to 0.


   - Otherwise (either mbType[ mbAddrP ] or mbType[ mbAddrQ ] specifies an Inter macroblock prediction mode),
the following applies:


      - If any of the following conditions are true, bS is set equal to 2:


        - mbType[ mbAddrP ] specifies an Inter macroblock prediction type, cTrafo[ mbAddrP ] is equal
to T_8x8, and the array rSL contains non-zero samples for the 8x8 luma block containing sample p0,


        - mbType[ mbAddrP ] specifies an Inter macroblock prediction type, cTrafo[ mbAddrP ] is equal
to T_4x4, and the array rSL contains non-zero samples for the 4x4 luma block containing sample p0,


        - mbType[ mbAddrQ ] specifies an Inter macroblock prediction type, cTrafo[ mbAddrQ ] is equal
to T_8x8, and the array rSL contains non-zero samples for the 8x8 luma block containing sample q0,


        - mbType[ mbAddrQ ] specifies an Inter macroblock prediction type, cTrafo[ mbAddrQ ] is equal
to T_4x4, and the array rSL contains non-zero samples for the 4x4 luma block containing sample q0.
NOTE 3 – The array rSL contains samples for the accumulated residual signal. Transform coefficient values of layer
representations that are used for inter-layer prediction are taken into account.


      - Otherwise, bS is set equal to 1.


- Otherwise, if the block edge is also a macroblock edge and any of the following conditions are true, bS is set equal
to 4:


   - fieldMbFlag[ mbAddrP ] is equal to 0 and fieldMbFlag[ mbAddrQ ] is equal to 0 and either or both
mbType[ mbAddrP ] or mbType[ mbAddrQ ] specify an Intra macroblock prediction mode,


   - MbaffFrameFlag is equal to 1 or field_pic_flag is equal to 1, and verticalEdgeFlag is equal to 1, and either or
both mbType[ mbAddrP ] or mbType[ mbAddrQ ] specify an Intra macroblock prediction mode.


- Otherwise, if any of the following conditions are true, bS is set equal to 3:


   - mixedModeEdgeFlag is equal to 0 and either or both mbType[ mbAddrP ] or mbType[ mbAddrQ ] specify an
Intra macroblock prediction mode,


   - mixedModeEdgeFlag is equal to 1, verticalEdgeFlag is equal to 0, and either or both mbType[ mbAddrP ] or
mbType[ mbAddrQ ] specify an Intra macroblock prediction mode.





- Otherwise, if any of the following conditions are true, bS is set equal to 2:


   - cTrafo[ mbAddrP ] is equal to T_8x8 and either the array rSL contains non-zero samples for the 8x8 luma block
containing sample p0 or ((sliceIdc[ mbAddrP ] & 127) is equal to 0 and the 8x8 luma transform block coded in
sliceP and associated with the 8x8 luma block containing sample p0 contains non-zero transform coefficient
levels),


   - cTrafo[ mbAddrP ] is equal to T_4x4 and either the array rSL contains non-zero samples for the 4x4 luma block
containing sample p0 or ((sliceIdc[ mbAddrP ] & 127) is equal to 0 and the 4x4 luma transform block coded in
sliceP and associated with the 4x4 luma block containing sample p0 contains non-zero transform coefficient
levels),


   - cTrafo[ mbAddrQ ] is equal to T_8x8 and either the array rSL contains non-zero samples for the 8x8 luma block
containing sample q0 or ((sliceIdc[ mbAddrQ ] & 127) is equal to 0 and the 8x8 luma transform block coded in
sliceQ and associated with the 8x8 luma block containing sample q0 contains non-zero transform coefficient
levels),


   - cTrafo[ mbAddrQ ] is equal to T_4x4 and either the array rSL contains non-zero samples for the 4x4 luma block
containing sample q0 or ((sliceIdc[ mbAddrQ ] & 127) is equal to 0 and the 4x4 luma transform block coded in
sliceQ and associated with the 4x4 luma block containing sample q0 contains non-zero transform coefficient
levels).

NOTE 4 – The array rSL contains samples for the accumulated residual signal. Transform coefficient values of layer
representations that are used for inter-layer prediction are taken into account.


- Otherwise, if profile_idc is equal to 83 and any of the following conditions are true, bS is set equal to 2:


   - cTrafo[ mbAddrP ] is equal to T_8x8 and the array sTCoeff[ mbAddrP ] contains non-zero scaled transform
coefficient values for the 8x8 luma transform block associated with the 8x8 luma block containing sample p0,


   - cTrafo[ mbAddrP ] is equal to T_4x4 and the array sTCoeff[ mbAddrP ] contains non-zero scaled transform
coefficient values for the 4x4 luma transform block associated with the 4x4 luma block containing sample p0,


   - cTrafo[ mbAddrQ ] is equal to T_8x8 and the array sTCoeff[ mbAddrQ ] contains non-zero scaled transform
coefficient values for the 8x8 luma transform block associated with the 8x8 luma block containing sample q0,


   - cTrafo[ mbAddrQ ] is equal to T_4x4 and the array sTCoeff[ mbAddrQ ] contains non-zero scaled transform
coefficient values for the 4x4 luma transform block associated with the 4x4 luma block containing sample q0.


- Otherwise, if mixedModeEdgeFlag is equal to 1 or any of the following conditions are true, bS is set equal to 1:


1. numMvP is not equal to numMvQ.


2. numMvP and numMvQ are both equal to 1 and any of the following conditions are true:


      - refP and refQ specify different reference pictures,


      - the absolute difference between the horizontal or vertical components of the motion vectors mvP and mvQ
is greater than or equal to 4 in units of quarter luma frame samples.


3. numRefP and numRefQ are both equal to 2 and any of the following conditions are true:


a. refL0P and refL1P specify different reference pictures and any of the following conditions are true:


i. both of the following conditions are true:


           - refL0P and refL0Q specify different reference pictures or refL1P and refL1Q specify different
reference pictures,


           - refL0P and refL1Q specify different reference pictures or refL1P and refL0Q specify different
reference pictures.


ii. refL0P and refL0Q specifies the same reference picture, refL1P and refL1Q specify the same reference

picture, and any of the following conditions are true:


           - the absolute difference between the horizontal or vertical components of the motion vectors
mvL0P and mvL0Q is greater than or equal to 4 in units of quarter luma frame samples,


           - the absolute difference between the horizontal or vertical components of the motion vectors
mvL1P and mvL1Q is greater than or equal to 4 in units of quarter luma frame samples.


iii. refL0P and refL1Q specifies the same reference picture, refL1P and refL0Q specify the same reference

picture, and any of the following conditions are true:





           - the absolute difference between the horizontal or vertical components of the motion vectors
mvL0P and mvL1Q is greater than or equal to 4 in units of quarter luma frame samples,


           - the absolute difference between the horizontal or vertical components of the motion vectors
mvL1P and mvL0Q is greater than or equal to 4 in units of quarter luma frame samples.


b. refL0P and refL1P specify the same reference picture and any of the following conditions are true:


i. refL0Q or refL1Q specify a different reference picture than refL0P (or refL1P).


ii. refL0Q and refL1Q specify the same reference picture as refL0P (and refL1P) and both of the

following conditions are true:


           - the absolute difference between the horizontal or vertical components of the motion vectors
mvL0P and mvL0Q is greater than or equal to 4 in units of quarter luma frame samples or the
absolute difference between the horizontal or vertical components of the motion vectors mvL1P
and mvL1Q is greater than or equal to 4 in units of quarter luma frame samples,


           - the absolute difference between the horizontal or vertical components of the motion vectors
mvL0P and mvL1Q is greater than or equal to 4 in units of quarter luma frame samples or the
absolute difference between the horizontal or vertical components of the motion vectors mvL1P
and mvL0Q is greater than or equal to 4 in units of quarter luma frame samples.
NOTE 5 – The determination of whether the reference pictures used for the two macroblock/sub-macroblock partitions
are the same or different is based only on which pictures are referenced, without regard to whether a prediction is formed
using an index into reference picture list 0 or an index into reference picture list 1, and also without regard to whether the
index position within a reference picture list is different.
NOTE 6 – A vertical difference of 4 in units of quarter luma frame samples is a difference of 2 in units of quarter luma
field samples


- Otherwise, bS is set equal to 0.


The variable interProfileConformanceFlag is derived as follows:


- If DQId is greater than 0, interLayerDeblockingFlag is equal to 0, and any of the following conditions are true,
interProfileConformanceFlag is set equal to 1:


   - profile_idc is equal to 83 and constraint_set1_flag is equal to 1,


   - profile_idc is equal to 86 and constraint_set0_flag is equal to 1.


- Otherwise, interProfileConformanceFlag is set equal to 0.


When interProfileConformanceFlag is equal to 1 and both mbType[ mbAddrP ] and mbType[ mbAddrQ ] specify an Inter
macroblock prediction mode, it is a requirement of bitstream conformance that the following constraints are obeyed:


- When cTrafo[ mbAddrP ] is equal to T_8x8 and the array sTCoeff[ mbAddrP ] contains at least one non-zero scaled
transform coefficient value for the 8x8 luma transform block associated with the 8x8 luma block containing sample
p0, the bitstream shall not contain data that result in an array rSL for which all sample values are equal to 0 for the 8x8
luma block containing sample p0.


- When cTrafo[ mbAddrP ] is equal to T_4x4 and the array sTCoeff[ mbAddrP ] contains at least one non-zero scaled
transform coefficient value for the 4x4 luma transform block associated with the 4x4 luma block containing sample
p0, the bitstream shall not contain data that result in an array rSL for which all sample values are equal to 0 for the 4x4
luma block containing sample p0.


- When cTrafo[ mbAddrQ ] is equal to T_8x8 and the array sTCoeff[ mbAddrQ ] contains at least one non-zero scaled
transform coefficient value for the 8x8 luma transform block associated with the 8x8 luma block containing sample
q0, the bitstream shall not contain data that result in an array rSL for which all sample values are equal to 0 for the 8x8
luma block containing sample q0.


- When cTrafo[ mbAddrQ ] is equal to T_4x4 and the array sTCoeff[ mbAddrQ ] contains at least one non-zero scaled
transform coefficient value for the 4x4 luma transform block associated with the 4x4 luma block containing sample
q0, the bitstream shall not contain data that result in an array rSL for which all sample values are equal to 0 for the 4x4
luma block containing sample q0.


**G.8.8** **Specification of bitstream subsets**


Clause G.8.8.1 specifies the sub-bitstream extraction process.


Clause G.8.8.2 specifies the base layer bitstream.





**G.8.8.1** **Sub-bitstream extraction process**


It is requirement of bitstream conformance that any sub-bitstream that is the output of the process specified in this clause
with pIdTarget equal to any value in the range of 0 to 63, inclusive, tIdTarget equal to any value in the range of 0 to 7,
inclusive, dIdTarget equal to any value in the range of 0 to 7, inclusive, and qIdTarget equal to any value in the range of 0
to 15, inclusive, shall be conforming to this Recommendation | International Standard.

NOTE – A conforming bitstream contains one or more coded slice NAL units with priority_id equal to 0, dependency_id equal to 0,
quality_id equal to 0, and temporal_id equal to 0.


Inputs to this process are:


- a variable pIdTarget (when present),


- a variable tIdTarget (when present),


- a variable dIdTarget (when present),


- a variable qIdTarget (when present).


Output of this process is a sub-bitstream.


When pIdTarget is not present as input to this clause, pIdTarget is inferred to be equal to 63.


When tIdTarget is not present as input to this clause, tIdTarget is inferred to be equal to 7.


When dIdTarget is not present as input to this clause, dIdTarget is inferred to be equal to 7.


When qIdTarget is not present as input to this clause, qIdTarget is inferred to be equal to 15.


The sub-bitstream is derived by applying the following operations in sequential order:


1. Mark all VCL NAL units and filler data NAL units for which any of the following conditions are true as "to be

removed from the bitstream":


     - priority_id is greater than pIdTarget,


     - temporal_id is greater than tIdTarget,


     - dependency_id is greater than dIdTarget,


     - dependency_id is equal to dIdTarget and quality_id is greater than qIdTarget.


2. Remove all access units for which all VCL NAL units are marked as "to be removed from the bitstream".


3. Remove all VCL NAL units and filler data NAL units that are marked as "to be removed from the bitstream".


4. When dIdTarget is equal to 0 and qIdTarget is equal to 0, remove the following NAL units:


     - all NAL units with nal_unit_type equal to 14 or 15,


     - all NAL units with nal_unit_type equal to 6 in which the first SEI message has payloadType in the range of
24 to 35, inclusive.


5. Remove all NAL units with nal_unit_type equal to 6 that only contain SEI messages that are part of a scalable

nesting SEI message with any of the following properties:


     - sei_temporal_id is greater than tIdTarget,


     - the minimum value of ( sei_dependency_id[ i ] << 4 ) + sei_quality_id[ i ] for all i in the range of 0 to
num_layer_representations_minus1, inclusive, is greater than ( dIdTarget << 4 ) + qIdTarget.


6. Remove all NAL units with nal_unit_type equal to 6 that contain SEI messages with payloadType equal to 24,

28, or 29.


**G.8.8.2** **Specification of the base layer bitstream**


Each scalable bitstream that conforms to this specification shall contain a base layer bitstream that conforms to one or
more of the profiles specified in Annex A. This base layer bitstream is derived by invoking the sub-bitstream extraction
process as specified in clause G.8.8.1 with dIdTarget being equal to 0 and qIdTarget being equal to 0 and the base layer
bitstream being the output.

NOTE – Although all scalable bitstreams that conform to one or more of the profiles specified in this annex contain a base layer
bitstream that conforms to one or more of the profiles specified in Annex A, the complete scalable bitstream (prior to operation of
the base layer extraction process specified in this clause) may not conform to any profile specified in Annex A.
