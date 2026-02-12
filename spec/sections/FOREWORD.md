FOREWORD


The International Telecommunication Union (ITU) is the United Nations specialized agency in the field of
telecommunications, information and communication technologies (ICTs). The ITU Telecommunication
Standardization Sector (ITU-T) is a permanent organ of ITU. ITU-T is responsible for studying technical,
operating and tariff questions and issuing Recommendations on them with a view to standardizing
telecommunications on a worldwide basis.


The World Telecommunication Standardization Assembly (WTSA), which meets every four years, establishes
the topics for study by the ITU-T study groups which, in turn, produce Recommendations on these topics.


The approval of ITU-T Recommendations is covered by the procedure laid down in WTSA Resolution 1.


In some areas of information technology which fall within ITU-T's purview, the necessary standards are
prepared on a collaborative basis with ISO and IEC.


NOTE


In this Recommendation, the expression "Administration" is used for conciseness to indicate both a
telecommunication administration and a recognized operating agency.


Compliance with this Recommendation is voluntary. However, the Recommendation may contain certain
mandatory provisions (to ensure, e.g., interoperability or applicability) and compliance with the
Recommendation is achieved when all of these mandatory provisions are met. The words "shall" or some other
obligatory language such as "must" and the negative equivalents are used to express requirements. The use of
such words does not suggest that compliance with the Recommendation is required of any party.


INTELLECTUAL PROPERTY RIGHTS


ITU draws attention to the possibility that the practice or implementation of this Recommendation may involve
the use of a claimed Intellectual Property Right. ITU takes no position concerning the evidence, validity or
applicability of claimed Intellectual Property Rights, whether asserted by ITU members or others outside of
the Recommendation development process.


As of the date of approval of this Recommendation, ITU had received notice of intellectual property, protected
by patents/software copyrights, which may be required to implement this Recommendation. However,
implementers are cautioned that this may not represent the latest information and are therefore strongly urged
[to consult the appropriate ITU-T databases available via the ITU-T website at http://www.itu.int/ITU-T/ipr/.](http://www.itu.int/ITU-T/ipr/)


© ITU 2021


All rights reserved. No part of this publication may be reproduced, by any means whatsoever, without the prior
written permission of ITU.


**Rec. ITU-T H.264 (08/2021)** iii


**Table of Contents**


_Page_

0 Introduction ............................................................................................................................................................1
0.1 Prologue ..........................................................................................................................................................1
0.2 Purpose ...........................................................................................................................................................1
0.3 Applications ....................................................................................................................................................1
0.4 Publication and versions of this Specification ..................................................................................................1
0.5 Profiles and levels ...........................................................................................................................................3
0.6 Overview of the design characteristics .............................................................................................................4
0.6.1 Predictive coding .....................................................................................................................................5
0.6.2 Coding of progressive and interlaced video..............................................................................................5
0.6.3 Picture partitioning into macroblocks and smaller partitions ....................................................................5
0.6.4 Spatial redundancy reduction ...................................................................................................................5
0.7 How to read this Specification .........................................................................................................................5

1 Scope ......................................................................................................................................................................6

2 Normative references ..............................................................................................................................................6

3 Definitions ..............................................................................................................................................................7

4 Abbreviations ....................................................................................................................................................... 15

5 Conventions .......................................................................................................................................................... 16
5.1 Arithmetic operators ...................................................................................................................................... 16
5.2 Logical operators........................................................................................................................................... 16
5.3 Relational operators ...................................................................................................................................... 17
5.4 Bit-wise operators ......................................................................................................................................... 17
5.5 Assignment operators .................................................................................................................................... 17
5.6 Range notation .............................................................................................................................................. 17
5.7 Mathematical functions ................................................................................................................................. 18
5.8 Order of operation precedence ....................................................................................................................... 19
5.9 Variables, syntax elements, and tables ........................................................................................................... 19
5.10 Text description of logical operations ............................................................................................................ 20
5.11 Processes ....................................................................................................................................................... 21

6 Source, coded, decoded and output data formats, scanning processes, and neighbouring relationships .................. 21
6.1 Bitstream formats .......................................................................................................................................... 21
6.2 Source, decoded, and output picture formats.................................................................................................. 22
6.3 Spatial subdivision of pictures and slices ....................................................................................................... 26
6.4 Inverse scanning processes and derivation processes for neighbours .............................................................. 27
6.4.1 Inverse macroblock scanning process .................................................................................................... 27
6.4.2 Inverse macroblock partition and sub-macroblock partition scanning process ........................................ 27
6.4.2.1 Inverse macroblock partition scanning process ................................................................................ 28
6.4.2.2 Inverse sub-macroblock partition scanning process .......................................................................... 28
6.4.3 Inverse 4x4 luma block scanning process............................................................................................... 29
6.4.4 Inverse 4x4 Cb or Cr block scanning process for ChromaArrayType equal to 3 ..................................... 29
6.4.5 Inverse 8x8 luma block scanning process............................................................................................... 29
6.4.6 Inverse 8x8 Cb or Cr block scanning process for ChromaArrayType equal to 3 ..................................... 30
6.4.7 Inverse 4x4 chroma block scanning process ........................................................................................... 30
6.4.8 Derivation process of the availability for macroblock addresses............................................................. 30
6.4.9 Derivation process for neighbouring macroblock addresses and their availability .................................. 30
6.4.10 Derivation process for neighbouring macroblock addresses and their availability in MBAFF frames ..... 31
6.4.11 Derivation processes for neighbouring macroblocks, blocks, and partitions ........................................... 32
6.4.11.1 Derivation process for neighbouring macroblocks ........................................................................... 33
6.4.11.2 Derivation process for neighbouring 8x8 luma block ....................................................................... 33
6.4.11.3 Derivation process for neighbouring 8x8 chroma blocks for ChromaArrayType equal to 3 .............. 34
6.4.11.4 Derivation process for neighbouring 4x4 luma blocks...................................................................... 34
6.4.11.5 Derivation process for neighbouring 4x4 chroma blocks .................................................................. 34
6.4.11.6 Derivation process for neighbouring 4x4 chroma blocks for ChromaArrayType equal to 3 .............. 35
6.4.11.7 Derivation process for neighbouring partitions ................................................................................ 35
6.4.12 Derivation process for neighbouring locations ....................................................................................... 36
6.4.12.1 Specification for neighbouring locations in fields and non-MBAFF frames ..................................... 37


iv **Rec. ITU-T H.264 (08/2021)**


6.4.12.2 Specification for neighbouring locations in MBAFF frames ............................................................ 37
6.4.13 Derivation processes for block and partition indices .............................................................................. 39
6.4.13.1 Derivation process for 4x4 luma block indices ................................................................................. 39
6.4.13.2 Derivation process for 4x4 chroma block indices ............................................................................. 39
6.4.13.3 Derivation process for 8x8 luma block indices ................................................................................. 39
6.4.13.4 Derivation process for macroblock and sub-macroblock partition indices ........................................ 39

7 Syntax and semantics ............................................................................................................................................ 40
7.1 Method of specifying syntax in tabular form ................................................................................................. 40
7.2 Specification of syntax functions, categories, and descriptors ........................................................................ 41
7.3 Syntax in tabular form ................................................................................................................................... 43
7.3.1 NAL unit syntax .................................................................................................................................... 43
7.3.2 Raw byte sequence payloads and RBSP trailing bits syntax ................................................................... 43
7.3.2.1 Sequence parameter set RBSP syntax .............................................................................................. 43
7.3.2.2 Picture parameter set RBSP syntax .................................................................................................. 47
7.3.2.3 Supplemental enhancement information RBSP syntax ..................................................................... 48
7.3.2.4 Access unit delimiter RBSP syntax .................................................................................................. 48
7.3.2.5 End of sequence RBSP syntax ......................................................................................................... 49
7.3.2.6 End of stream RBSP syntax ............................................................................................................. 49
7.3.2.7 Filler data RBSP syntax ................................................................................................................... 49
7.3.2.8 Slice layer without partitioning RBSP syntax................................................................................... 49
7.3.2.9 Slice data partition RBSP syntax ..................................................................................................... 49
7.3.2.10 RBSP slice trailing bits syntax ......................................................................................................... 50
7.3.2.11 RBSP trailing bits syntax ................................................................................................................. 50
7.3.2.12 Prefix NAL unit RBSP syntax ......................................................................................................... 51
7.3.2.13 Slice layer extension RBSP syntax .................................................................................................. 51
7.3.3 Slice header syntax ................................................................................................................................ 51
7.3.3.1 Reference picture list modification syntax ....................................................................................... 53
7.3.3.2 Prediction weight table syntax ......................................................................................................... 54
7.3.3.3 Decoded reference picture marking syntax ...................................................................................... 55
7.3.4 Slice data syntax .................................................................................................................................... 56
7.3.5 Macroblock layer syntax ........................................................................................................................ 57
7.3.5.1 Macroblock prediction syntax .......................................................................................................... 58
7.3.5.2 Sub-macroblock prediction syntax ................................................................................................... 59
7.3.5.3 Residual data syntax ........................................................................................................................ 60
7.4 Semantics ...................................................................................................................................................... 63
7.4.1 NAL unit semantics ............................................................................................................................... 63
7.4.1.1 Encapsulation of an SODB within an RBSP (informative) ............................................................... 67
7.4.1.2 Order of NAL units and association to coded pictures, access units, and video sequences ................ 68
7.4.2 Raw byte sequence payloads and RBSP trailing bits semantics .............................................................. 73
7.4.2.1 Sequence parameter set RBSP semantics ......................................................................................... 73
7.4.2.2 Picture parameter set RBSP semantics ............................................................................................. 81
7.4.2.3 Supplemental enhancement information RBSP semantics ................................................................ 84
7.4.2.4 Access unit delimiter RBSP semantics ............................................................................................. 84
7.4.2.5 End of sequence RBSP semantics .................................................................................................... 84
7.4.2.6 End of stream RBSP semantics ........................................................................................................ 84
7.4.2.7 Filler data RBSP semantics .............................................................................................................. 85
7.4.2.8 Slice layer without partitioning RBSP semantics ............................................................................. 85
7.4.2.9 Slice data partition RBSP semantics ................................................................................................ 85
7.4.2.10 RBSP slice trailing bits semantics .................................................................................................... 86
7.4.2.11 RBSP trailing bits semantics ............................................................................................................ 86
7.4.2.12 Prefix NAL unit RBSP semantics .................................................................................................... 86
7.4.2.13 Slice layer extension RBSP semantics ............................................................................................. 86
7.4.3 Slice header semantics ........................................................................................................................... 86
7.4.3.1 Reference picture list modification semantics .................................................................................. 92
7.4.3.2 Prediction weight table semantics .................................................................................................... 93
7.4.3.3 Decoded reference picture marking semantics ................................................................................. 93
7.4.4 Slice data semantics ............................................................................................................................... 97
7.4.5 Macroblock layer semantics .................................................................................................................. 98
7.4.5.1 Macroblock prediction semantics ................................................................................................... 106
7.4.5.2 Sub-macroblock prediction semantics ............................................................................................ 107
7.4.5.3 Residual data semantics .................................................................................................................  v


8 Decoding process ................................................................................................................................................ 111
8.1 NAL unit decoding process ......................................................................................................................... 112
8.2 Slice decoding process ................................................................................................................................ 113
8.2.1 Decoding process for picture order count ............................................................................................. 113
8.2.1.1 Decoding process for picture order count type 0 ............................................................................ 114
8.2.1.2 Decoding process for picture order count type 1 ............................................................................ 115
8.2.1.3 Decoding process for picture order count type 2 ............................................................................ 116
8.2.2 Decoding process for macroblock to slice group map .......................................................................... 117
8.2.2.1 Specification for interleaved slice group map type ......................................................................... 118
8.2.2.2 Specification for dispersed slice group map type............................................................................ 118
8.2.2.3 Specification for foreground with left-over slice group map type ................................................... 118
8.2.2.4 Specification for box-out slice group map types............................................................................. 119
8.2.2.5 Specification for raster scan slice group map types ........................................................................ 119
8.2.2.6 Specification for wipe slice group map types ................................................................................. 119
8.2.2.7 Specification for explicit slice group map type............................................................................... 120
8.2.2.8 Specification for conversion of map unit to slice group map to macroblock to slice group map...... 120
8.2.3 Decoding process for slice data partitions ............................................................................................ 120
8.2.4 Decoding process for reference picture lists construction ..................................................................... 121
8.2.4.1 Decoding process for picture numbers ........................................................................................... 121
8.2.4.2 Initialization process for reference picture lists .............................................................................. 122
8.2.4.3 Modification process for reference picture lists .............................................................................. 125
8.2.5 Decoded reference picture marking process ......................................................................................... 127
8.2.5.1 Sequence of operations for decoded reference picture marking process .......................................... 128
8.2.5.2 Decoding process for gaps in frame_num ...................................................................................... 128
8.2.5.3 Sliding window decoded reference picture marking process .......................................................... 129
8.2.5.4 Adaptive memory control decoded reference picture marking process ........................................... 129
8.3 Intra prediction process ............................................................................................................................... 131
8.3.1 Intra_4x4 prediction process for luma samples .................................................................................... 132
8.3.1.1 Derivation process for Intra4x4PredMode ..................................................................................... 132
8.3.1.2 Intra_4x4 sample prediction .......................................................................................................... 134
8.3.2 Intra_8x8 prediction process for luma samples .................................................................................... 137
8.3.2.1 Derivation process for Intra8x8PredMode ..................................................................................... 138
8.3.2.2 Intra_8x8 sample prediction .......................................................................................................... 139
8.3.3 Intra_16x16 prediction process for luma samples................................................................................. 144
8.3.3.1 Specification of Intra_16x16_Vertical prediction mode ................................................................. 145
8.3.3.2 Specification of Intra_16x16_Horizontal prediction mode ............................................................. 145
8.3.3.3 Specification of Intra_16x16_DC prediction mode ........................................................................ 145
8.3.3.4 Specification of Intra_16x16_Plane prediction mode ..................................................................... 146
8.3.4 Intra prediction process for chroma samples ........................................................................................ 146
8.3.4.1 Specification of Intra_Chroma_DC prediction mode...................................................................... 147
8.3.4.2 Specification of Intra_Chroma_Horizontal prediction mode .......................................................... 149
8.3.4.3 Specification of Intra_Chroma_Vertical prediction mode .............................................................. 149
8.3.4.4 Specification of Intra_Chroma_Plane prediction mode .................................................................. 149
8.3.4.5 Intra prediction for chroma samples with ChromaArrayType equal to 3 ......................................... 149
8.3.5 Sample construction process for I_PCM macroblocks ......................................................................... 150
8.4 Inter prediction process ............................................................................................................................... 151
8.4.1 Derivation process for motion vector components and reference indices .............................................. 153
8.4.1.1 Derivation process for luma motion vectors for skipped macroblocks in P and SP slices ................ 154
8.4.1.2 Derivation process for luma motion vectors for B_Skip, B_Direct_16x16, and B_Direct_8x8 ....... 154
8.4.1.3 Derivation process for luma motion vector prediction .................................................................... 161
8.4.1.4 Derivation process for chroma motion vectors ............................................................................... 164
8.4.2 Decoding process for Inter prediction samples ..................................................................................... 165
8.4.2.1 Reference picture selection process ............................................................................................... 165
8.4.2.2 Fractional sample interpolation process ......................................................................................... 166
8.4.2.3 Weighted sample prediction process .............................................................................................. 172
8.4.3 Derivation process for prediction weights ............................................................................................ 173
8.5 Transform coefficient decoding process and picture construction process prior to deblocking filter process 175
8.5.1 Specification of transform decoding process for 4x4 luma residual blocks ........................................... 176
8.5.2 Specification of transform decoding process for luma samples of Intra_16x16 macroblock prediction
mode ................................................................................................................................................... 176
8.5.3 Specification of transform decoding process for 8x8 luma residual blocks ........................................... 177
8.5.4 Specification of transform decoding process for chroma samples ......................................................... 178


vi **Rec. ITU-T H.264 (08/2021)**


8.5.5 Specification of transform decoding process for chroma samples with ChromaArrayType equal to 3 .. 180
8.5.6 Inverse scanning process for 4x4 transform coefficients and scaling lists ............................................. 180
8.5.7 Inverse scanning process for 8x8 transform coefficients and scaling lists ............................................. 181
8.5.8 Derivation process for chroma quantization parameters ....................................................................... 182
8.5.9 Derivation process for scaling functions .............................................................................................. 183
8.5.10 Scaling and transformation process for DC transform coefficients for Intra_16x16 macroblock type ... 184
8.5.11 Scaling and transformation process for chroma DC transform coefficients ........................................... 185
8.5.11.1 Transformation process for chroma DC transform coefficients ...................................................... 185
8.5.11.2 Scaling process for chroma DC transform coefficients ................................................................... 185
8.5.12 Scaling and transformation process for residual 4x4 blocks ................................................................. 186
8.5.12.1 Scaling process for residual 4x4 blocks ......................................................................................... 187
8.5.12.2 Transformation process for residual 4x4 blocks ............................................................................. 187
8.5.13 Scaling and transformation process for residual 8x8 blocks ................................................................. 189
8.5.13.1 Scaling process for residual 8x8 blocks ......................................................................................... 189
8.5.13.2 Transformation process for residual 8x8 blocks ............................................................................. 189
8.5.14 Picture construction process prior to deblocking filter process ............................................................. 192
8.5.15 Intra residual transform-bypass decoding process ................................................................................ 193
8.6 Decoding process for P macroblocks in SP slices or SI macroblocks ........................................................... 193
8.6.1 SP decoding process for non-switching pictures .................................................................................. 194
8.6.1.1 Luma transform coefficient decoding process ................................................................................ 194
8.6.1.2 Chroma transform coefficient decoding process ............................................................................. 195
8.6.2 SP and SI slice decoding process for switching pictures ....................................................................... 196
8.6.2.1 Luma transform coefficient decoding process ................................................................................ 196
8.6.2.2 Chroma transform coefficient decoding process ............................................................................. 197
8.7 Deblocking filter process ............................................................................................................................. 198
8.7.1 Filtering process for block edges ......................................................................................................... 202
8.7.2 Filtering process for a set of samples across a horizontal or vertical block edge ................................... 203
8.7.2.1 Derivation process for the luma content dependent boundary filtering strength .............................. 204
8.7.2.2 Derivation process for the thresholds for each block edge .............................................................. 206
8.7.2.3 Filtering process for edges with bS less than 4 ............................................................................... 207
8.7.2.4 Filtering process for edges for bS equal to 4 .................................................................................. 208

9 Parsing process ................................................................................................................................................... 209
9.1 Parsing process for Exp-Golomb codes ....................................................................................................... 209
9.1.1 Mapping process for signed Exp-Golomb codes .................................................................................. 211
9.1.2 Mapping process for coded block pattern ............................................................................................. 211
9.2 CAVLC parsing process for transform coefficient levels ............................................................................. 214
9.2.1 Parsing process for total number of non-zero transform coefficient levels and number of trailing ones 214
9.2.2 Parsing process for level information ................................................................................................... 218
9.2.2.1 Parsing process for level_prefix ..................................................................................................... 219
9.2.3 Parsing process for run information ..................................................................................................... 220
9.2.4 Combining level and run information .................................................................................................. 223
9.3 CABAC parsing process for slice data ......................................................................................................... 223
9.3.1 Initialization process ............................................................................................................................ 225
9.3.1.1 Initialization process for context variables ..................................................................................... 225
9.3.1.2 Initialization process for the arithmetic decoding engine ................................................................ 248
9.3.2 Binarization process ............................................................................................................................ 248
9.3.2.1 Unary (U) binarization process ...................................................................................................... 251
9.3.2.2 Truncated unary (TU) binarization process .................................................................................... 252
9.3.2.3 Concatenated unary/ k-th order Exp-Golomb (UEGk) binarization process .................................... 252
9.3.2.4 Fixed-length (FL) binarization process .......................................................................................... 253
9.3.2.5 Binarization process for macroblock type and sub-macroblock type .............................................. 253
9.3.2.6 Binarization process for coded block pattern .................................................................................. 256
9.3.2.7 Binarization process for mb_qp_delta ............................................................................................ 256
9.3.3 Decoding process flow ........................................................................................................................ 256
9.3.3.1 Derivation process for ctxIdx ......................................................................................................... 257
9.3.3.2 Arithmetic decoding process .......................................................................................................... 270
9.3.4 Arithmetic encoding process (informative) .......................................................................................... 277
9.3.4.1 Initialization process for the arithmetic encoding engine (informative) .......................................... 277
9.3.4.2 Encoding process for a binary decision (informative) .................................................................... 278
9.3.4.3 Renormalization process in the arithmetic encoding engine (informative) ...................................... 279
9.3.4.4 Bypass encoding process for binary decisions (informative) .......................................................... 281
9.3.4.5 Encoding process for a binary decision before termination (informative) .......................................  vii


9.3.4.6 Byte stuffing process (informative) ................................................................................................ 284

Annex A – Profiles and levels..................................................................................................................................... 285

A.1 Requirements on video decoder capability ................................................................................................... 285
A.2 Profiles........................................................................................................................................................ 285
A.2.1 Baseline profile ................................................................................................................................... 285
A.2.1.1 Constrained Baseline profile .......................................................................................................... 286
A.2.2 Main profile......................................................................................................................................... 286
A.2.3 Extended profile .................................................................................................................................. 286
A.2.4 High profile ......................................................................................................................................... 287
A.2.4.1 Progressive High profile ................................................................................................................ 287
A.2.4.2 Constrained High profile ............................................................................................................... 288
A.2.5 High 10 profile .................................................................................................................................... 288
A.2.5.1 Progressive High 10 profile ........................................................................................................... 288
A.2.6 High 4:2:2 profile ................................................................................................................................ 289
A.2.7 High 4:4:4 Predictive profile................................................................................................................ 289
A.2.8 High 10 Intra profile ............................................................................................................................ 289
A.2.9 High 4:2:2 Intra profile ........................................................................................................................ 290
A.2.10 High 4:4:4 Intra profile ........................................................................................................................ 291
A.2.11 CAVLC 4:4:4 Intra profile................................................................................................................... 291
A.3 Levels ......................................................................................................................................................... 291
A.3.1 Level limits common to the Baseline, Constrained Baseline, Main, and Extended profiles ................... 292
A.3.2 Level limits common to the High, Progressive High, Constrained High, High 10, Progressive High 10,
High 4:2:2, High 4:4:4 Predictive, High 10 Intra, High 4:2:2 Intra, High 4:4:4 Intra, and
CAVLC 4:4:4 Intra profiles ................................................................................................................. 294
A.3.3 Profile-specific level limits .................................................................................................................. 296
A.3.3.1 Level limits of the Baseline and Constrained Baseline profile ........................................................ 298
A.3.3.2 Level limits of the Main, High, Progressive High, Constrained High, High 10, Progressive High 10,
High 4:2:2, High 4:4:4 Predictive, High 10 Intra, High 4:2:2 Intra, High 4:4:4 Intra, and
CAVLC 4:4:4 Intra profile............................................................................................................. 298
A.3.3.3 Level limits of the Extended profile ............................................................................................... 299
A.3.4 Effect of level limits on frame rate (informative) ................................................................................. 301
A.3.5 Effect of level limits on maximum DPB size in units of frames (informative) ...................................... 304

Annex B – Byte stream format ................................................................................................................................... 307

B.1 Byte stream NAL unit syntax and semantics ................................................................................................ 307
B.1.1 Byte stream NAL unit syntax............................................................................................................... 307
B.1.2 Byte stream NAL unit semantics ......................................................................................................... 307
B.2 Byte stream NAL unit decoding process ...................................................................................................... 308
B.3 Decoder byte-alignment recovery (informative) .......................................................................................... 308

Annex C – Hypothetical reference decoder ................................................................................................................. 309

C.1 Operation of coded picture buffer (CPB) ..................................................................................................... 313
C.1.1 Timing of bitstream arrival .................................................................................................................. 314
C.1.2 Timing of coded picture removal ......................................................................................................... 315
C.2 Operation of the decoded picture buffer (DPB)............................................................................................ 315
C.2.1 Decoding of gaps in frame_num and storage of "non-existing" frames................................................. 316
C.2.2 Picture decoding and output ................................................................................................................. 316
C.2.3 Removal of pictures from the DPB before possible insertion of the current picture .............................. 317
C.2.4 Current decoded picture marking and storage ...................................................................................... 319
C.2.4.1 Marking and storage of a reference picture into the DPB ............................................................... 319
C.2.4.2 Storage of a non-reference picture into the DPB ............................................................................ 319
C.3 Bitstream conformance................................................................................................................................ 320
C.4 Decoder conformance ................................................................................................................................. 321
C.4.1 Operation of the output order DPB ...................................................................................................... 322
C.4.2 Decoding of gaps in frame_num and storage of "non-existing" pictures ............................................... 323
C.4.3 Picture decoding .................................................................................................................................. 323
C.4.4 Removal of pictures from the DPB before possible insertion of the current picture .............................. 324
C.4.5 Current decoded picture marking and storage ...................................................................................... 325
C.4.5.1 Storage and marking of a reference decoded picture into the DPB ................................................. 325
C.4.5.2 Storage and marking of a non-reference decoded picture into the DPB .......................................... 326
C.4.5.3 "Bumping" process ........................................................................................................................ 327


viii **Rec. ITU-T H.264 (08/2021)**


Annex D – Supplemental enhancement information ................................................................................................... 329

D.1 SEI payload syntax ...................................................................................................................................... 329
D.1.1 General SEI message syntax ................................................................................................................ 329
D.1.2 Buffering period SEI message syntax .................................................................................................. 333
D.1.3 Picture timing SEI message syntax ...................................................................................................... 333
D.1.4 Pan-scan rectangle SEI message syntax ............................................................................................... 334
D.1.5 Filler payload SEI message syntax ....................................................................................................... 334
D.1.6 User data registered by Rec. ITU-T T.35 SEI message syntax.............................................................. 335
D.1.7 User data unregistered SEI message syntax.......................................................................................... 335
D.1.8 Recovery point SEI message syntax..................................................................................................... 335
D.1.9 Decoded reference picture marking repetition SEI message syntax ...................................................... 335
D.1.10 Spare picture SEI message syntax ........................................................................................................ 336
D.1.11 Scene information SEI message syntax ................................................................................................ 336
D.1.12 Sub-sequence information SEI message syntax .................................................................................... 337
D.1.13 Sub-sequence layer characteristics SEI message syntax ....................................................................... 337
D.1.14 Sub-sequence characteristics SEI message syntax ................................................................................ 337
D.1.15 Full-frame freeze SEI message syntax ................................................................................................. 338
D.1.16 Full-frame freeze release SEI message syntax ...................................................................................... 338
D.1.17 Full-frame snapshot SEI message syntax ............................................................................................. 338
D.1.18 Progressive refinement segment start SEI message syntax ................................................................... 338
D.1.19 Progressive refinement segment end SEI message syntax .................................................................... 338
D.1.20 Motion-constrained slice group set SEI message syntax ....................................................................... 338
D.1.21 Film grain characteristics SEI message syntax ..................................................................................... 339
D.1.22 Deblocking filter display preference SEI message syntax .................................................................... 339
D.1.23 Stereo video information SEI message syntax ...................................................................................... 340
D.1.24 Post-filter hint SEI message syntax ...................................................................................................... 340
D.1.25 Tone mapping information SEI message syntax ................................................................................... 341
D.1.26 Frame packing arrangement SEI message syntax ................................................................................. 342
D.1.27 Display orientation SEI message syntax............................................................................................... 342
D.1.28 Green metadata SEI message syntax .................................................................................................... 342
D.1.29 Mastering display colour volume SEI message syntax ......................................................................... 343
D.1.30 Colour remapping information SEI message syntax ............................................................................. 344
D.1.31 Content light level information SEI message syntax ............................................................................ 345
D.1.32 Alternative transfer characteristics SEI message syntax ....................................................................... 345
D.1.33 Content colour volume SEI message syntax ......................................................................................... 345
D.1.34 Ambient viewing environment SEI message syntax ............................................................................. 345
D.1.35 Syntax of omnidirectional video specific SEI messages ....................................................................... 346
D.1.35.1 Equirectangular projection SEI message syntax ............................................................................. 346
D.1.35.2 Cubemap projection SEI message syntax ....................................................................................... 346
D.1.35.3 Sphere rotation SEI message syntax............................................................................................... 346
D.1.35.4 Region-wise packing SEI message syntax...................................................................................... 347
D.1.35.5 Omnidirectional viewport SEI message syntax .............................................................................. 348
D.1.36 SEI manifest SEI message syntax ........................................................................................................ 348
D.1.37 SEI prefix indication SEI message syntax ............................................................................................ 348
D.1.38 Annotated regions SEI message syntax ................................................................................................ 349
D.1.39 Shutter interval information SEI message syntax ................................................................................. 350
D.1.40 Reserved SEI message syntax .............................................................................................................. 350
D.2 SEI payload semantics................................................................................................................................. 350
D.2.1 General SEI payload semantics ............................................................................................................ 350
D.2.2 Buffering period SEI message semantics ............................................................................................. 351
D.2.3 Picture timing SEI message semantics ................................................................................................. 351
D.2.4 Pan-scan rectangle SEI message semantics .......................................................................................... 356
D.2.5 Filler payload SEI message semantics.................................................................................................. 357
D.2.6 User data registered by Rec. ITU-T T.35 SEI message semantics ........................................................ 357
D.2.7 User data unregistered SEI message semantics .................................................................................... 357
D.2.8 Recovery point SEI message semantics ............................................................................................... 358
D.2.9 Decoded reference picture marking repetition SEI message semantics ................................................. 360
D.2.10 Spare picture SEI message semantics ................................................................................................... 360
D.2.11 Scene information SEI message semantics ........................................................................................... 362
D.2.12 Sub-sequence information SEI message semantics............................................................................... 363
D.2.13 Sub-sequence layer characteristics SEI message semantics .................................................................. 365
D.2.14 Sub-sequence characteristics SEI message semantics ........................................................................... 366
D.2.15 Full-frame freeze SEI message semantics ............................................................................................  ix


D.2.16 Full-frame freeze release SEI message semantics................................................................................. 367
D.2.17 Full-frame snapshot SEI message semantics ........................................................................................ 367
D.2.18 Progressive refinement segment start SEI message semantics .............................................................. 368
D.2.19 Progressive refinement segment end SEI message semantics ............................................................... 368
D.2.20 Motion-constrained slice group set SEI message semantics ................................................................. 368
D.2.21 Film grain characteristics SEI message semantics ................................................................................ 369
D.2.22 Deblocking filter display preference SEI message semantics ............................................................... 375
D.2.23 Stereo video information SEI message semantics................................................................................. 377
D.2.24 Post-filter hint SEI message semantics ................................................................................................. 378
D.2.25 Tone mapping information SEI message semantics.............................................................................. 379
D.2.26 Frame packing arrangement SEI message semantics ............................................................................ 383
D.2.27 Display orientation SEI message semantics ......................................................................................... 394
D.2.28 Green metadata SEI message semantics ............................................................................................... 395
D.2.29 Mastering display colour volume SEI message semantics .................................................................... 396
D.2.30 Colour remapping information SEI message semantics ........................................................................ 397
D.2.31 Content light level information SEI message semantics ....................................................................... 399
D.2.32 Alternative transfer characteristics SEI message semantics .................................................................. 400
D.2.33 Content colour volume SEI message semantics.................................................................................... 400
D.2.34 Ambient viewing environment SEI message semantics ........................................................................ 402
D.2.35 Semantics of omnidirectional video specific SEI messages .................................................................. 403
D.2.35.1 Equirectangular projection SEI message semantics ........................................................................ 403
D.2.35.2 Cubemap projection SEI message semantics .................................................................................. 404
D.2.35.3 Sphere rotation SEI message semantics ......................................................................................... 405
D.2.35.4 Region-wise packing SEI message semantics ................................................................................ 405
D.2.35.5 Omnidirectional viewport SEI message semantics ......................................................................... 410
D.2.35.6 Sample location remapping process ............................................................................................... 411
D.2.36 SEI manifest SEI message semantics ................................................................................................... 416
D.2.37 SEI prefix indication SEI message semantics ....................................................................................... 417
D.2.38 Annotated regions SEI message semantics ........................................................................................... 418
D.2.39 Shutter interval information SEI message semantics ............................................................................ 420
D.2.40 Reserved SEI message semantics ......................................................................................................... 421

Annex E – Video usability information ....................................................................................................................... 422

E.1 VUI syntax .................................................................................................................................................. 422
E.1.1 VUI parameters syntax ........................................................................................................................ 422
E.1.2 HRD parameters syntax ....................................................................................................................... 423
E.2 VUI semantics............................................................................................................................................. 424
E.2.1 VUI parameters semantics ................................................................................................................... 424
E.2.2 HRD parameters semantics .................................................................................................................. 439

Annex F – Deprecated ................................................................................................................................................ 442

Annex G – Scalable video coding ............................................................................................................................... 443

G.1 Scope .......................................................................................................................................................... 443
G.2 Normative references .................................................................................................................................. 443
G.3 Definitions .................................................................................................................................................. 443
G.4 Abbreviations .............................................................................................................................................. 447
G.5 Conventions ................................................................................................................................................ 447
G.6 Source, coded, decoded and output data formats, scanning processes, neighbouring and reference layer
relationships ................................................................................................................................................ 447
G.6.1 Derivation process for reference layer macroblocks ............................................................................. 447
G.6.1.1 Field-to-frame reference layer macroblock conversion process ...................................................... 449
G.6.1.2 Frame-to-field reference layer macroblock conversion process ...................................................... 449
G.6.2 Derivation process for reference layer partitions .................................................................................. 449
G.6.3 Derivation process for reference layer sample locations in resampling ................................................. 450
G.6.4 SVC derivation process for macroblock and sub-macroblock partition indices ..................................... 452
G.7 Syntax and semantics .................................................................................................................................. 452
G.7.1 Method of specifying syntax in tabular form ........................................................................................ 452
G.7.2 Specification of syntax functions, categories, and descriptors .............................................................. 453
G.7.3 Syntax in tabular form ......................................................................................................................... 453
G.7.3.1 NAL unit syntax ............................................................................................................................ 453
G.7.3.2 Raw byte sequence payloads and RBSP trailing bits syntax ........................................................... 453
G.7.3.3 Slice header syntax ........................................................................................................................ 455
G.7.3.4 Slice data syntax ............................................................................................................................ 458


x **Rec. ITU-T H.264 (08/2021)**


G.7.3.5 Macroblock layer syntax ................................................................................................................ 459
G.7.3.6 Macroblock layer in scalable extension syntax ............................................................................... 460
G.7.4 Semantics ............................................................................................................................................ 463
G.7.4.1 NAL unit semantics ....................................................................................................................... 464
G.7.4.2 Raw byte sequence payloads and RBSP trailing bits semantics ...................................................... 472
G.7.4.3 Slice header semantics ................................................................................................................... 477
G.7.4.4 Slice data semantics ....................................................................................................................... 491
G.7.4.5 Macroblock layer semantics .......................................................................................................... 491
G.7.4.6 Macroblock layer in scalable extension semantics ......................................................................... 492
G.8 SVC decoding process................................................................................................................................. 495
G.8.1 SVC initialization and decoding processes ........................................................................................... 496
G.8.1.1 Derivation process for the set of layer representations required for decoding ................................. 496
G.8.1.2 Array assignment, initialization, and restructuring processes ......................................................... 496
G.8.1.3 Layer representation decoding processes ....................................................................................... 499
G.8.1.4 Slice decoding processes ............................................................................................................... 502
G.8.1.5 Macroblock initialization and decoding processes .......................................................................... 503
G.8.2 SVC reference picture lists construction and decoded reference picture marking process ..................... 514
G.8.2.1 SVC decoding process for picture order count ............................................................................... 516
G.8.2.2 SVC decoding process for picture numbers.................................................................................... 517
G.8.2.3 SVC decoding process for reference picture lists construction ....................................................... 517
G.8.2.4 SVC decoded reference picture marking process ........................................................................... 519
G.8.2.5 SVC decoding process for gaps in frame_num ............................................................................... 524
G.8.3 SVC intra decoding processes .............................................................................................................. 525
G.8.3.1 SVC derivation process for intra prediction modes ........................................................................ 525
G.8.3.2 SVC intra sample prediction and construction process ................................................................... 528
G.8.4 SVC Inter prediction process ............................................................................................................... 534
G.8.4.1 SVC derivation process for motion vector components and reference indices ................................ 534
G.8.4.2 SVC decoding process for Inter prediction samples ....................................................................... 540
G.8.5 SVC transform coefficient decoding and sample array construction processes ..................................... 545
G.8.5.1 Transform coefficient scaling and refinement process .................................................................... 546
G.8.5.2 Transform coefficient level scaling process prior to transform coefficient refinement .................... 553
G.8.5.3 Residual construction and accumulation process ............................................................................ 554
G.8.5.4 Sample array accumulation process ............................................................................................... 558
G.8.5.5 Sample array re-initialization process ............................................................................................ 561
G.8.6 Resampling processes for prediction data, intra samples, and residual samples .................................... 561
G.8.6.1 Derivation process for inter-layer predictors for macroblock type, sub-macroblock type, reference
indices, and motion vectors............................................................................................................ 561
G.8.6.2 Resampling process for intra samples ............................................................................................ 570
G.8.6.3 Resampling process for residual samples ....................................................................................... 582
G.8.7 SVC deblocking filter processes .......................................................................................................... 589
G.8.7.1 Deblocking filter process for Intra_Base prediction ....................................................................... 589
G.8.7.2 Deblocking filter process for target representations........................................................................ 589
G.8.7.3 Derivation process for quantization parameters used in the deblocking filter process ..................... 590
G.8.7.4 Macroblock deblocking filter process ............................................................................................ 591
G.8.8 Specification of bitstream subsets ........................................................................................................ 600
G.8.8.1 Sub-bitstream extraction process ................................................................................................... 601
G.8.8.2 Specification of the base layer bitstream ........................................................................................ 601
G.9 Parsing process............................................................................................................................................ 602
G.9.1 Alternative parsing process for coded block pattern ............................................................................. 602
G.9.2 Alternative CAVLC parsing process for transform coefficient levels ................................................... 603
G.9.2.1 Additional parsing process for total number of non-zero transform coefficient levels and number of
trailing ones ................................................................................................................................... 603
G.9.2.2 Alternative parsing process for run information ............................................................................. 606
G.9.3 Alternative CABAC parsing process for slice data in scalable extension .............................................. 607
G.9.3.1 Initialization process ...................................................................................................................... 607
G.9.3.2 Binarization process ...................................................................................................................... 608
G.9.3.3 Decoding process flow .................................................................................................................. 608
G.10 Profiles and levels ....................................................................................................................................... 610

G.10.1 Profiles ................................................................................................................................................ 610
G.10.1.1 Scalable Baseline profile ............................................................................................................... 610
G.10.1.2 Scalable High profile ..................................................................................................................... 613
G.10.1.3 Scalable High Intra profile .............................................................................................................  xi


G.10.2 Levels .................................................................................................................................................. 615
G.10.2.1 Level limits common to Scalable Baseline, Scalable Constrained Baseline, Scalable High, Scalable
Constrained High, and Scalable High Intra profiles........................................................................ 616
G.10.2.2 Profile specific level limits ............................................................................................................ 618
G.11 Byte stream format ...................................................................................................................................... 620
G.12 Hypothetical reference decoder ................................................................................................................... 620
G.13 Supplemental enhancement information ...................................................................................................... 620

G.13.1 SEI payload syntax .............................................................................................................................. 620
G.13.1.1 Scalability information SEI message syntax................................................................................... 621
G.13.1.2 Sub-picture scalable layer SEI message syntax .............................................................................. 623
G.13.1.3 Non-required layer representation SEI message syntax .................................................................. 624
G.13.1.4 Priority layer information SEI message syntax ............................................................................... 624
G.13.1.5 Layers not present SEI message syntax .......................................................................................... 624
G.13.1.6 Layer dependency change SEI message syntax .............................................................................. 624
G.13.1.7 Scalable nesting SEI message syntax ............................................................................................. 625
G.13.1.8 Base layer temporal HRD SEI message syntax .............................................................................. 625
G.13.1.9 Quality layer integrity check SEI message syntax .......................................................................... 626
G.13.1.10 Redundant picture property SEI message syntax ............................................................................ 626
G.13.1.11 Temporal level zero dependency representation index SEI message syntax.................................... 626
G.13.1.12 Temporal level switching point SEI message syntax ...................................................................... 627
G.13.2 SEI payload semantics ......................................................................................................................... 627
G.13.2.1 Scalability information SEI message semantics ............................................................................. 629
G.13.2.2 Sub-picture scalable layer SEI message semantics ......................................................................... 643
G.13.2.3 Non-required layer representation SEI message semantics ............................................................. 644
G.13.2.4 Priority layer information SEI message semantics ......................................................................... 645
G.13.2.5 Layers not present SEI message semantics ..................................................................................... 645
G.13.2.6 Layer dependency change SEI message semantics ......................................................................... 645
G.13.2.7 Scalable nesting SEI message semantics ........................................................................................ 647
G.13.2.8 Base layer temporal HRD SEI message semantics ......................................................................... 648
G.13.2.9 Quality layer integrity check SEI message semantics ..................................................................... 649
G.13.2.10 Redundant picture property SEI message semantics ....................................................................... 649
G.13.2.11 Temporal level zero dependency representation index SEI message semantics .............................. 650
G.13.2.12 Temporal level switching point SEI message semantics ................................................................. 652
G.14 Video usability information ......................................................................................................................... 653

G.14.1 SVC VUI parameters extension syntax ................................................................................................ 653
G.14.2 SVC VUI parameters extension semantics ........................................................................................... 653



Annex H – Multiview video coding ............................................................................................................................ 656

H.1 Scope .......................................................................................................................................................... 656
H.2 Normative references .................................................................................................................................. 656
H.3 Definitions .................................................................................................................................................. 656
H.4 Abbreviations .............................................................................................................................................. 658
H.5 Conventions ................................................................................................................................................ 658
H.6 Source, coded, decoded and output data formats, scanning processes, and neighbouring relationships ......... 659
H.7 Syntax and semantics .................................................................................................................................. 659
H.7.1 Method of specifying syntax in tabular form ........................................................................................ 659
H.7.2 Specification of syntax functions, categories, and descriptors .............................................................. 659
H.7.3 Syntax in tabular form ......................................................................................................................... 659
H.7.3.1 NAL unit syntax ............................................................................................................................ 659
H.7.3.2 Raw byte sequence payloads and RBSP trailing bits syntax ........................................................... 659
H.7.3.3 Slice header syntax ........................................................................................................................ 661
H.7.3.4 Slice data syntax ............................................................................................................................ 662
H.7.3.5 Macroblock layer syntax ................................................................................................................ 662
H.7.4 Semantics ............................................................................................................................................ 663
H.7.4.1 NAL unit semantics ....................................................................................................................... 663
H.7.4.2 Raw byte sequence payloads and RBSP trailing bits semantics ...................................................... 670
H.7.4.3 Slice header semantics ................................................................................................................... 674
H.7.4.4 Slice data semantics ....................................................................................................................... 676
H.7.4.5 Macroblock layer semantics .......................................................................................................... 676
H.8 MVC decoding process ............................................................................................................................... 677
H.8.1 MVC decoding process for picture order count .................................................................................... 677
H.8.2 MVC decoding process for reference picture lists construction ............................................................ 678


xii **Rec. ITU-T H.264 (08/2021)**


H.8.2.1 Initialization process for reference picture list for inter-view prediction references ........................ 679
H.8.2.2 Modification process for reference picture lists .............................................................................. 679
H.8.3 MVC decoded reference picture marking process ................................................................................ 682
H.8.4 MVC inter prediction and inter-view prediction process ...................................................................... 682
H.8.4.1 Additional processing for an inter-view prediction reference ......................................................... 682
H.8.5 Specification of bitstream subsets ........................................................................................................ 686
H.8.5.1 Derivation process for required anchor view components .............................................................. 686
H.8.5.2 Derivation process for required non-anchor view components ....................................................... 687
H.8.5.3 Sub-bitstream extraction process ................................................................................................... 687
H.8.5.4 Specification of the base view bitstream ........................................................................................ 689
H.8.5.5 Creation of a base view during sub-bitstream extraction (informative) ........................................... 689
H.8.6 MFC enhanced resolution picture reconstruction ................................................................................. 690
H.9 Parsing process............................................................................................................................................ 694
H.10 Profiles and levels ....................................................................................................................................... 694

H.10.1 Profiles ................................................................................................................................................ 694
H.10.1.1 Multiview High profile .................................................................................................................. 694
H.10.1.2 Stereo High profile ........................................................................................................................ 695
H.10.1.3 MFC High profile .......................................................................................................................... 696
H.10.2 Levels .................................................................................................................................................. 696
H.10.2.1 Level limits common to Multiview High, Stereo High, and MFC High profiles ............................. 697
H.10.2.2 Profile specific level limits ............................................................................................................ 699
H.11 Byte stream format ...................................................................................................................................... 699
H.12 MVC hypothetical reference decoder .......................................................................................................... 700
H.13 MVC SEI messages..................................................................................................................................... 700

H.13.1 SEI message syntax ............................................................................................................................. 700
H.13.1.1 Parallel decoding information SEI message syntax ........................................................................ 700
H.13.1.2 MVC scalable nesting SEI message syntax .................................................................................... 700
H.13.1.3 View scalability information SEI message syntax .......................................................................... 701
H.13.1.4 Multiview scene information SEI message syntax ......................................................................... 702
H.13.1.5 Multiview acquisition information SEI message syntax ................................................................. 702
H.13.1.6 Non-required view component SEI message syntax ....................................................................... 703
H.13.1.7 View dependency change SEI message syntax ............................................................................... 704
H.13.1.8 Operation point not present SEI message syntax ............................................................................ 704
H.13.1.9 Base view temporal HRD SEI message syntax .............................................................................. 704
H.13.1.10 Multiview view position SEI message syntax ................................................................................ 705
H.13.2 SEI message semantics ........................................................................................................................ 705
H.13.2.1 Parallel decoding information SEI message semantics ................................................................... 706
H.13.2.2 MVC scalable nesting SEI message semantics ............................................................................... 707
H.13.2.3 View scalability information SEI message semantics ..................................................................... 708
H.13.2.4 Multiview scene information SEI message semantics .................................................................... 711
H.13.2.5 Multiview acquisition information SEI message semantics ............................................................ 711
H.13.2.6 Non-required view component SEI message semantics .................................................................. 714
H.13.2.7 View dependency change SEI message semantics ......................................................................... 715
H.13.2.8 Operation point not present SEI message semantics ....................................................................... 716
H.13.2.9 Base view temporal HRD SEI message semantics ......................................................................... 716
H.13.2.10 Multiview view position SEI message semantics ........................................................................... 717
H.14 Video usability information ......................................................................................................................... 717

H.14.1 MVC VUI parameters extension syntax ............................................................................................... 717
H.14.2 MVC VUI parameters extension semantics .......................................................................................... 718



Annex I – Multiview and depth video coding ............................................................................................................. 720

I.1 Scope .......................................................................................................................................................... 720
I.2 Normative references .................................................................................................................................. 720
I.3 Definitions .................................................................................................................................................. 720
I.4 Abbreviations .............................................................................................................................................. 721
I.5 Conventions ................................................................................................................................................ 721
I.6 Source, coded, decoded and output data formats, scanning processes, and neighbouring relationships ......... 721
I.7 Syntax and semantics .................................................................................................................................. 721
I.7.1 Method of specifying syntax in tabular form ........................................................................................ 721
I.7.2 Specification of syntax functions, categories, and descriptors .............................................................. 721
I.7.3 Syntax in tabular form ......................................................................................................................... 721
I.7.3.1 NAL unit syntax ............................................................................................................................  xiii


I.7.3.2 Raw byte sequence payloads and RBSP trailing bits syntax ........................................................... 721
I.7.3.3 Slice header syntax ........................................................................................................................ 723
I.7.3.4 Slice data syntax ............................................................................................................................ 723
I.7.3.5 Macroblock layer syntax ................................................................................................................ 723
I.7.4 Semantics ............................................................................................................................................ 724
I.7.4.1 NAL unit semantics ....................................................................................................................... 724
I.7.4.2 Raw byte sequence payloads and RBSP trailing bits semantics ...................................................... 731
I.7.4.3 Slice header semantics ................................................................................................................... 733
I.7.4.4 Slice data semantics ....................................................................................................................... 733
I.7.4.5 Macroblock layer semantics .......................................................................................................... 733
I.8 MVCD decoding process ............................................................................................................................ 734
I.8.1 MVCD decoding process for picture order count ................................................................................. 735
I.8.2 MVC decoding process for reference picture lists construction ............................................................ 735
I.8.2.1 Initialization process for reference picture list for inter-view prediction references ........................ 735
I.8.2.2 Modification process for reference picture lists .............................................................................. 735
I.8.3 MVCD decoded reference picture marking process ............................................................................. 735
I.8.4 MVCD inter prediction and inter-view prediction process ................................................................... 735
I.8.5 Specification of bitstream subsets ........................................................................................................ 735
I.8.5.1 Derivation process for required anchor view components .............................................................. 735
I.8.5.2 Derivation process for required non-anchor view components ....................................................... 735
I.8.5.3 Sub-bitstream extraction process ................................................................................................... 736
I.8.5.4 Specification of the base view bitstream ........................................................................................ 737
I.8.5.5 Specification of the stereoscopic texture bitstream ......................................................................... 738
I.9 Parsing process............................................................................................................................................ 738
I.10 Profiles and levels ....................................................................................................................................... 738
I.10.1 Profiles ................................................................................................................................................ 738
I.10.1.1 Multiview Depth High profile ........................................................................................................ 738
I.10.1.2 MFC Depth High profile ............................................................................................................... 739
I.10.2 Levels .................................................................................................................................................. 740
I.10.2.1 Level limits common to Multiview Depth High profiles ................................................................ 740
I.10.2.2 Profile specific level limits ............................................................................................................ 743
I.11 Byte stream format ...................................................................................................................................... 743
I.12 MVCD hypothetical reference decoder ........................................................................................................ 743
I.13 MVCD SEI messages .................................................................................................................................. 743
I.13.1 SEI message syntax ............................................................................................................................. 743
I.13.1.1 MVCD view scalability information SEI message syntax .............................................................. 743
I.13.1.2 MVCD scalable nesting SEI message syntax ................................................................................. 745
I.13.1.3 Depth representation information SEI message syntax ................................................................... 746
I.13.1.4 3D reference displays information SEI message syntax ................................................................. 747
I.13.1.5 Depth timing SEI message syntax .................................................................................................. 747
I.13.1.6 Alternative depth information SEI message syntax ........................................................................ 748
I.13.1.7 Depth sampling information SEI message syntax........................................................................... 749
I.13.2 SEI message semantics ........................................................................................................................ 750
I.13.2.1 MVCD view scalability information SEI message semantics ......................................................... 752
I.13.2.2 MVCD scalable nesting SEI message semantics ............................................................................ 753
I.13.2.3 Depth representation information SEI message semantics .............................................................. 753
I.13.2.4 3D reference displays information SEI message semantics ............................................................ 756
I.13.2.5 Depth timing SEI message semantics ............................................................................................. 759
I.13.2.6 Alternative depth information SEI message semantics ................................................................... 759
I.13.2.7 Depth sampling information SEI message semantics ..................................................................... 763
I.14 Video usability information ......................................................................................................................... 764
I.14.1 MVCD VUI parameters extension syntax ............................................................................................ 764
I.14.2 MVCD VUI parameters extension semantics ....................................................................................... 764

Annex J – Multiview and depth video with enhanced non-base view coding ............................................................... 767

J.1 Scope .......................................................................................................................................................... 767
J.2 Normative references .................................................................................................................................. 767
J.3 Definitions .................................................................................................................................................. 767
J.4 Abbreviations .............................................................................................................................................. 767
J.5 Conventions ................................................................................................................................................ 767
J.6 Source, coded, decoded and output data formats, scanning processes, and neighbouring relationships ......... 767
J.6.1 Inverse sub-macroblock partition scanning process .............................................................................. 767


xiv **Rec. ITU-T H.264 (08/2021)**


J.7 Syntax and semantics .................................................................................................................................. 768
J.7.1 Method of specifying syntax in tabular form ........................................................................................ 768
J.7.2 Specification of syntax functions, categories, and descriptors .............................................................. 768
J.7.3 Syntax in tabular form ......................................................................................................................... 768
J.7.3.1 NAL unit syntax ............................................................................................................................ 768
J.7.3.2 Raw byte sequence payloads and RBSP trailing bits syntax ........................................................... 769
J.7.3.3 Slice header syntax ........................................................................................................................ 773
J.7.3.4 Slice data syntax ............................................................................................................................ 776
J.7.3.5 Macroblock layer syntax ................................................................................................................ 777
J.7.3.6 Macroblock layer in 3D-AVC extension syntax ............................................................................. 778
J.7.4 Semantics ............................................................................................................................................ 781
J.7.4.1 NAL unit semantics ....................................................................................................................... 781
J.7.4.2 Raw byte sequence payloads and RBSP trailing bits semantics ...................................................... 782
J.7.4.3 Slice header semantics ................................................................................................................... 788
J.7.4.4 Slice data semantics ....................................................................................................................... 790
J.7.4.5 Macroblock layer semantics .......................................................................................................... 790
J.7.4.6 Macroblock layer in 3D-AVC extension semantics ........................................................................ 791
J.8 3D-AVC decoding process .......................................................................................................................... 792
J.8.1 3D-AVC decoding process for reference picture lists construction ....................................................... 792
J.8.2 3D-AVC inter prediction, inter-view prediction, view synthesis prediction and adaptive luminance
compensation ...................................................................................................................................... 793
J.8.2.1 Derivation process for motion vector components and reference indices ........................................ 796
J.8.2.2 Derivation of prediction weights in depth-range-based weighted prediction ................................... 805
J.8.2.3 Derivation process for motion vectors and reference indices for adaptive luminance compensation807
J.8.2.4 Derivation process for prediction weights in adaptive luminance compensation ............................. 807
J.8.3 Specification of bitstream subsets ........................................................................................................ 809
J.8.4 Decoding process for depth range parameters ...................................................................................... 809
J.9 Parsing process............................................................................................................................................ 810
J.9.1 Alternative CABAC parsing process for slice data and macroblock layer in depth extension ............... 810
J.9.1.1 Initialization process ...................................................................................................................... 810
J.9.1.2 Binarization process ...................................................................................................................... 811
J.9.1.3 Decoding process flow .................................................................................................................. 812
J.10 Profiles and levels ....................................................................................................................................... 814
J.10.1 Profiles ................................................................................................................................................ 814
J.10.1.1 Enhanced Multiview Depth High profile ....................................................................................... 815
J.10.2 Levels .................................................................................................................................................. 815
J.10.2.1 Level limits for Enhanced Multiview Depth High profile ............................................................... 815
J.10.2.2 Profile specific level limits ............................................................................................................ 815
J.11 Byte stream format ...................................................................................................................................... 816
J.12 3D-AVC hypothetical reference decoder ..................................................................................................... 816
J.13 3D-AVC SEI messages ............................................................................................................................... 816
J.13.1 SEI message syntax ............................................................................................................................. 816
J.13.1.1 Constrained depth parameter set identifier SEI message syntax ..................................................... 816
J.13.2 SEI message semantics ........................................................................................................................ 816
J.13.2.1 Constrained depth parameter set identifier SEI message semantics ................................................ 816
J.14 Video usability information ......................................................................................................................... 817


**List of Figures**


Figure 6-1 – Nominal vertical and horizontal locations of 4:2:0 luma and chroma samples in a frame .......................... 23


Figure 6-2 – Nominal vertical and horizontal sampling locations of 4:2:0 samples in top and bottom fields .................. 24


Figure 6-4 – Nominal vertical and horizontal sampling locations of 4:2:2 samples top and bottom fields ...................... 25


Figure 6-5 – Nominal vertical and horizontal locations of 4:4:4 luma and chroma samples in a frame .......................... 25


Figure 6-6 – Nominal vertical and horizontal sampling locations of 4:4:4 samples top and bottom fields ...................... 25


Figure 6-7 – A picture with 11 by 9 macroblocks that is partitioned into two slices ...................................................... 26


Figure 6-8 – Partitioning of the decoded frame into macroblock pairs ...........................................................................  xv


Figure 6-9 – Macroblock partitions, sub-macroblock partitions, macroblock partition scans, and sub-macroblock

partition scans ....................................................................................................................................................... 28


Figure 6-10 – Scan for 4x4 luma blocks........................................................................................................................ 29


Figure 6-11 – Scan for 8x8 luma blocks........................................................................................................................ 29


Figure 6-12 – Neighbouring macroblocks for a given macroblock ................................................................................ 31


Figure 6-13 – Neighbouring macroblocks for a given macroblock in MBAFF frames ................................................... 32


Figure 6-14 – Determination of the neighbouring macroblock, blocks, and partitions (informative) .............................. 33


Figure 7-1 – Structure of an access unit not containing any NAL units with nal_unit_type equal to 0, 7, 8,

or in the range of 12 to 18, inclusive, or in the range of 20 to 31, inclusive ........................................................... 71


Figure 8-1 – Intra_4x4 prediction mode directions (informative) ................................................................................ 133


Figure 8-2 – Example for temporal direct-mode motion vector inference (informative) .............................................. 161


Figure 8-3 – Directional segmentation prediction (informative) .................................................................................. 162


Figure 8-4 – Integer samples (shaded blocks with upper-case letters) and fractional sample positions

(un-shaded blocks with lower-case letters) for quarter sample luma interpolation................................................ 169


Figure 8-5 – Fractional sample position dependent variables in chroma interpolation and surrounding integer

position samples A, B, C, and D ......................................................................................................................... 171


Figure 8-6 – Assignment of the indices of dcY to luma4x4BlkIdx .............................................................................. 177


Figure 8-7 – Assignment of the indices of dcC to chroma4x4BlkIdx: (a) ChromaArrayType equal to 1,

(b) ChromaArrayType equal to 2 ........................................................................................................................ 179


Figure 8-8 – 4x4 block scans. (a) Zig-zag scan. (b) Field scan (informative) ............................................................... 180


Figure 8-9 – 8x8 block scans. (a) 8x8 zig-zag scan. (b) 8x8 field scan (informative) ................................................... 181


Figure 8-10 – Boundaries in a macroblock to be filtered ............................................................................................. 199


Figure 8-11 – Convention for describing samples across a 4x4 block horizontal or vertical boundary ......................... 203


Figure 9-1 – Illustration of CABAC parsing process for a syntax element SE (informative)........................................ 225


Figure 9-2 – Overview of the arithmetic decoding process for a single bin (informative) ............................................ 271


Figure C-2 – HRD buffer model ................................................................................................................................. 311


Figure D-1 – Rearrangement and upconversion of checkerboard interleaving (frame_packing_arrangement_

type equal to 0) ................................................................................................................................................... 389


Figure D-2 – Rearrangement and upconversion of column interleaving with frame_packing_arrangement_type

equal to 1, quincunx_sampling_flag equal to 0, and (x, y) equal to (0, 0) or (4, 8) for both constituent frames .... 389


Figure D-3 – Rearrangement and upconversion of column interleaving with frame_packing_arrangement_type

equal to 1, quincunx_sampling_flag equal to 0, (x, y) equal to (0, 0) or (4, 8) for constituent frame 0 and
(x, y) equal to (12, 8) for constituent frame 1 ...................................................................................................... 390


Figure D-4 – Rearrangement and upconversion of row interleaving with frame_packing_arrangement_type

equal to 2, quincunx_sampling_flag equal to 0, and (x, y) equal to (0, 0) or (8, 4) for both constituent frames .... 390


Figure D-5 – Rearrangement and upconversion of row interleaving with frame_packing_arrangement_type

equal to 2, quincunx_sampling_flag equal to 0, (x, y) equal to (0, 0) or (8, 4) for constituent frame 0, and
(x, y) equal to (8, 12) for constituent frame 1 ...................................................................................................... 391


Figure D-6 – Rearrangement and upconversion of side-by-side packing arrangement with frame_packing_

arrangement_type equal to 3, quincunx_sampling_flag equal to 0, and (x, y) equal to (0, 0) or (4, 8) for both
constituent frames ............................................................................................................................................... 391


Figure D-7 – Rearrangement and upconversion of side-by-side packing arrangement with frame_packing_

arrangement_type equal to 3, quincunx_sampling_flag equal to 0, (x, y) equal to (12, 8) for constituent
frame 0, and (x, y) equal to (0, 0) or (4, 8) for constituent frame 1 ...................................................................... 392


xvi **Rec. ITU-T H.264 (08/2021)**


Figure D-8 – Rearrangement and upconversion of top-bottom packing arrangement with frame_packing_

arrangement_type equal to 4, quincunx_sampling_flag equal to 0, and (x, y) equal to (0, 0) or (8, 4) for
both constituent frames ....................................................................................................................................... 392


Figure D-9 – Rearrangement and upconversion of top-bottom packing arrangement with frame_packing_

arrangement_type equal to 4, quincunx_sampling_flag equal to 0, (x, y) equal to (8, 12) for constituent
frame 0, and (x, y) equal to (0, 0) or (8, 4) for constituent frame 1 ...................................................................... 393


**List of Tables**


Table 5-1 – Operation precedence from highest (at top of table) to lowest (at bottom of table) ..................................... 19


Table 6-1 – SubWidthC, and SubHeightC values derived from chroma_format_idc and separate_colour_plane_flag .. 22


Table 6-2 – Specification of input and output assignments for clauses 6.4.11.1 to 6.4.11.7 ........................................... 32


Table 6-3 – Specification of mbAddrN ......................................................................................................................... 37


Table 6-4 – Specification of mbAddrN and yM ............................................................................................................ 38


Table 7-1 – NAL unit type codes, syntax element categories, and NAL unit type classes .............................................. 65


Table 7-2 – Assignment of mnemonic names to scaling list indices and specification of fall-back rule ......................... 76


Table 7-3 – Specification of default scaling lists Default_4x4_Intra and Default_4x4_Inter .......................................... 76


Table 7-4 – Specification of default scaling lists Default_8x8_Intra and Default_8x8_Inter .......................................... 77


Table 7-5 – Meaning of primary_pic_type .................................................................................................................... 84


Table 7-6 – Name association to slice_type .................................................................................................................. 87


Table 7-7 – modification_of_pic_nums_idc operations for modification of reference picture lists................................. 93


Table 7-8 – Interpretation of adaptive_ref_pic_marking_mode_flag ............................................................................. 94


Table 7-9 – Memory management control operation (memory_management_control_operation) values ....................... 96


Table 7-10 – Allowed collective macroblock types for slice_type ................................................................................. 98


Table 7-11 – Macroblock types for I slices ................................................................................................................. 100


Table 7-12 – Macroblock type with value 0 for SI slices ............................................................................................. 101


Table 7-13 – Macroblock type values 0 to 4 for P and SP slices .................................................................................. 102


Table 7-14 – Macroblock type values 0 to 22 for B slices ........................................................................................... 103


Table 7-15 – Specification of CodedBlockPatternChroma values ............................................................................... 105


Table 7-16 – Relationship between intra_chroma_pred_mode and spatial prediction modes ....................................... 106


Table 7-17 – Sub-macroblock types in P macroblocks ................................................................................................ 107


Table 7-18 – Sub-macroblock types in B macroblocks................................................................................................ 108


Table 8-1 – Refined slice group map type ................................................................................................................... 117


Table 8-2 – Specification of Intra4x4PredMode[ luma4x4BlkIdx ] and associated names ........................................... 132


Table 8-3 – Specification of Intra8x8PredMode[ luma8x8BlkIdx ] and associated names ........................................... 138


Table 8-4 – Specification of Intra16x16PredMode and associated names .................................................................... 145


Table 8-5 – Specification of Intra chroma prediction modes and associated names ..................................................... 147


Table 8-6 – Specification of the variable colPic .......................................................................................................... 155


Table 8-7 – Specification of PicCodingStruct( X ) ...................................................................................................... 155


Table 8-8 – Specification of mbAddrCol, yM, and vertMvScale ................................................................................. 157


Table 8-9 – Assignment of prediction utilization flags ................................................................................................ 159


Table 8-10 – Derivation of the vertical component of the chroma vector in field coding mode....................................  xvii


Table 8-11 – Differential full-sample luma locations .................................................................................................. 169


Table 8-12 – Assignment of the luma prediction sample predPartLXL[ xL, yL ] ........................................................... 171


Table 8-13 – Specification of mapping of idx to cij for zig-zag and field scan ............................................................. 181


Table 8-14 – Specification of mapping of idx to cij for 8x8 zig-zag and 8x8 field scan ................................................ 182


Table 8-15 – Specification of QP **C** as a function of qP **I** ............................................................................................... 183


Table 8-16 – Derivation of offset dependent threshold variables  ´ and  ´ from indexA and indexB .......................... 206


Table 8-17 – Value of variable t´C0 as a function of indexA and bS ............................................................................. 207


Table 9-1 – Bit strings with "prefix" and "suffix" bits and assignment to codeNum ranges (informative) .................... 210


Table 9-2 – Exp-Golomb bit strings and codeNum in explicit form and used as ue(v) (informative) ........................... 210


Table 9-3 – Assignment of syntax element to codeNum for signed Exp-Golomb coded syntax elements se(v) ............ 211


Table 9-4 – Assignment of codeNum to values of coded_block_pattern for macroblock prediction modes ................. 212


Table 9-5 – coeff_token mapping to TotalCoeff( coeff_token ) and TrailingOnes( coeff_token ) ................................ 216


Table 9-6 – Codeword table for level_prefix (informative) ......................................................................................... 219


Table 9-7 – total_zeros tables for 4x4 blocks with tzVlcIndex 1 to 7 .......................................................................... 221


Table 9-8 – total_zeros tables for 4x4 blocks with tzVlcIndex 8 to 15......................................................................... 221


Table 9-9 – total_zeros tables for chroma DC 2x2 and 2x4 blocks .............................................................................. 222


Table 9-10 – Tables for run_before ............................................................................................................................. 223


Table 9-11 – Association of ctxIdx and syntax elements for each slice type in the initialization process ..................... 227


Table 9-12 – Values of variables m and n for ctxIdx from 0 to 10 ............................................................................... 228


Table 9-13 – Values of variables m and n for ctxIdx from 11 to 23 ............................................................................. 228


Table 9-14 – Values of variables m and n for ctxIdx from 24 to 39 ............................................................................. 229


Table 9-15 – Values of variables m and n for ctxIdx from 40 to 53 ............................................................................. 229


Table 9-16 – Values of variables m and n for ctxIdx from 54 to 59, and 399 to 401 .................................................... 229


Table 9-17 – Values of variables m and n for ctxIdx from 60 to 69 ............................................................................. 230


Table 9-18 – Values of variables m and n for ctxIdx from 70 to 104 ........................................................................... 230


Table 9-19 – Values of variables m and n for ctxIdx from 105 to 165 ......................................................................... 231


Table 9-20 – Values of variables m and n for ctxIdx from 166 to 226 ......................................................................... 232


Table 9-21 – Values of variables m and n for ctxIdx from 227 to 275 ......................................................................... 233


Table 9-22 – Values of variables m and n for ctxIdx from 277 to 337 ......................................................................... 234


Table 9-23 – Values of variables m and n for ctxIdx from 338 to 398 ......................................................................... 235


Table 9-24 – Values of variables m and n for ctxIdx from 402 to 459 ......................................................................... 236


Table 9-25 – Values of variables m and n for ctxIdx from 460 to 483 ......................................................................... 237


Table 9-26 – Values of variables m and n for ctxIdx from 484 to 571 ......................................................................... 237


Table 9-27 – Values of variables m and n for ctxIdx from 572 to 659 ......................................................................... 239


Table 9-28 – Values of variables m and n for ctxIdx from 660 to 717 ......................................................................... 241


Table 9-29 – Values of variables m and n for ctxIdx from 718 to 775 ......................................................................... 242


Table 9-30 – Values of variables m and n for ctxIdx from 776 to 863 ......................................................................... 243


Table 9-31 – Values of variables m and n for ctxIdx from 864 to 951 ......................................................................... 245


Table 9-32 – Values of variables m and n for ctxIdx from 952 to 1011 ....................................................................... 247


Table 9-33 – Values of variables m and n for ctxIdx from 1012 to 1023 ..................................................................... 248


xviii **Rec. ITU-T H.264 (08/2021)**


Table 9-34 – Syntax elements and associated types of binarization, maxBinIdxCtx, and ctxIdxOffset ........................ 249


Table 9-35 – Bin string of the unary binarization (informative)................................................................................... 252


Table 9-36 – Binarization for macroblock types in I slices .......................................................................................... 254


Table 9-37 – Binarization for macroblock types in P, SP, and B slices ........................................................................ 255


Table 9-38 – Binarization for sub-macroblock types in P, SP, and B slices ................................................................. 256


Table 9-39 – Assignment of ctxIdxInc to binIdx for all ctxIdxOffset values except those related to the syntax

elements coded_block_flag, significant_coeff_flag, last_significant_coeff_flag, and coeff_abs_level_minus1 .... 258


Table 9-40 – Assignment of ctxIdxBlockCatOffset to ctxBlockCat for syntax elements coded_block_flag,

significant_coeff_flag, last_significant_coeff_flag, and coeff_abs_level_minus1 ................................................ 259


Table 9-41 – Specification of ctxIdxInc for specific values of ctxIdxOffset and binIdx ............................................... 267


Table 9-42 – Specification of ctxBlockCat for the different blocks ............................................................................. 268


Table 9-43 – Mapping of scanning position to ctxIdxInc for ctxBlockCat = = 5, 9, or 13 .......................................... 269


Table 9-44 – Specification of rangeTabLPS depending on pStateIdx and qCodIRangeIdx .......................................... 274


Table 9-45 – State transition table............................................................................................................................... 275


Table A-1 – Level limits ............................................................................................................................................. 294


Table A-2 – Specification of cpbBrVclFactor and cpbBrNalFactor ............................................................................ 297


Table A-3 – Baseline and Constrained Baseline profile level limits ........................................................................... 298


Table A-4 – Main, High, Progressive High, Constrained High, High 10, Progressive High 10, High 4:2:2,

High 4:4:4 Predictive, High 10 Intra, High 4:2:2 Intra, High 4:4:4 Intra, and CAVLC 4:4:4 Intra profile
level limits .......................................................................................................................................................... 299


Table A-5 – Extended profile level limits ................................................................................................................... 300


Table A-6 – Maximum frame rates (frames per second) for some example frame sizes ............................................... 301


Table A-7 – Maximum DPB size (frames) for some example frame sizes ................................................................... 304


Table D-1 – Interpretation of pic_struct ...................................................................................................................... 353


Table D-2 – Mapping of ct_type to source picture scan .............................................................................................. 354


Table D-3 – Definition of counting_type values ......................................................................................................... 355


Table D-4 – scene_transition_type values ................................................................................................................... 362


Table D-5 – film_grain_model_id values .................................................................................................................... 370


Table D-6 – blending_mode_id values ........................................................................................................................ 371


Table D-7 – filter_hint_type values............................................................................................................................. 378


Table D-8 – Interpretation of camera_iso_speed_idc and exposure_index_idc ............................................................ 382


Table D-9 – Definition of frame_packing_arrangement_type...................................................................................... 384


Table D-10 – Definition of content_interpretation_type .............................................................................................. 386


Table D-11 – transform_type[ i ] values ..................................................................................................................... 407


Table D-12– manifest_sei_description[ i ] values ....................................................................................................... 417


Table E-1 – Meaning of sample aspect ratio indicator ................................................................................................. 424


Table E-2 – Meaning of video_format ........................................................................................................................ 425


Table E-3 – Colour primaries interpretation using colour_primaries syntax element ................................................... 426


Table E-4 – Transfer characteristics interpretation using transfer_characteristics syntax element ................................ 427


Table E-5 – Matrix coefficients interpretation using matrix_coefficients syntax element ............................................ 434


Table E-6 – Divisor for computation of  tfi,dpb( n ) ......................................................................................................  xix


Table G-1 – Name association to slice_type for NAL units with nal_unit_type equal to 20 ......................................... 480


Table G-2 – Interpretation of adaptive_ref_base_pic_marking_mode_flag ................................................................. 489


Table G-3 – Memory management base control operation (memory_management_base_control_operation) values.... 490


Table G-4 – Allowed collective macroblock types for slice_type ................................................................................ 493


Table G-5 – Inferred macroblock type I_BL for EI slices............................................................................................ 493


Table G-6 – Scale values cS for transform coefficient level scaling ............................................................................ 553


Table G-7 – Macroblock type predictors mbTypeILPred ............................................................................................ 569


Table G-8 – Sub-macroblock type predictors subMbTypeILPred[ mbPartIdx ] ........................................................... 570


Table G-9 – 16-phase luma interpolation filter for resampling in Intra_Base prediction .............................................. 579


Table G-10 – Mapping of ( nX, nY ) to coeffTokenIdx and vice versa ........................................................................ 604


Table G-11 – Association of ctxIdx and syntax elements for each slice type in the initialization process .................... 607


Table G-12 – Values of variables m and n for ctxIdx from 1024 to 1026 .................................................................... 608


Table G-13 – Values of variables m and n for ctxIdx from 1027 to 1030 .................................................................... 608


Table G-14 – Syntax elements and associated types of binarization, maxBinIdxCtx, and ctxIdxOffset........................ 608


Table G-15 – Assignment of ctxIdxInc to binIdx for the ctxIdxOffset values related to the syntax elements

base_mode_flag and residual_prediction_flag ..................................................................................................... 609


Table G-16 – Scalable Baseline and Scalable Constrained Baseline profile level limits .............................................. 620


Table G-17 – Specification of cpbBrVclFactor and cpbBrNalFactor ........................................................................... 620


Table H-1 – Association between frame packing arrangement type and syntax elements ............................................ 672


Table H-2 – modification_of_pic_nums_idc operations for modification of reference picture lists .............................. 676


Table H-3 – Association between camera parameter variables and syntax elements. ................................................... 714


Table I-1 – Definition of depth_representation_type ................................................................................................... 754


Table I-2 – Association between depth parameter variables and syntax elements ........................................................ 755


Table I-3 – Association between camera parameter variables and syntax elements ..................................................... 758


Table J-1 – Respective syntax elements for pre_slice_header_src, pre_ref_lists_src, pre_pred_weight_table_src

and pre_dec_ref_pic_marking_src ...................................................................................................................... 789


Table J-2 – Semantics of the values of pre_slice_header_src, pre_ref_lists_src, pre_pred_weight_table_src and

pre_dec_ref_pic_marking_src ............................................................................................................................. 789


Table J-3 – Macroblock type values 0 to 4 for P and SP slices .................................................................................... 791


Table J-4 – Association between depth parameter variables and syntax elements ........................................................ 810


Table J-5 – Association of ctxIdx and syntax elements in the initialization process ..................................................... 810


Table J-6 – Values of variables m and n for ctxIdx from 1031 to 1039 ....................................................................... 811


Table J-7 – Values of variables m and n for ctxIdx from 1040 to 1052 ....................................................................... 811


Table J-8 – Syntax elements and associated types of binarization, maxBinIdxCtx, and ctxIdxOffset ........................... 812


Table J-9 – Assignment of ctxIdxInc to binIdx for the ctxIdxOffset values related to the syntax elements

mb_vsskip_flag, mb_direct_type_flag, mb_alc_skip_flag, mb_alc_flag and mb_vsp_flag .................................. 813


xx **Rec. ITU-T H.264 (08/2021)**


**Recommendation ITU-T H.264**


**Advanced video coding for generic audiovisual services**
