**J.5** **Conventions**


The specifications in clause 5 apply.


**J.6** **Source, coded, decoded and output data formats, scanning processes, and neighbouring**
**relationships**


The specifications in clause 6 apply with substitution of 3D-AVC sequence parameter set for sequence parameter set and
by replacing references to clause 6.4.2.2 with reference to clause J.6.1.


**J.6.1** **Inverse sub-macroblock partition scanning process**


Inputs to this process are the index of a macroblock partition mbPartIdx and the index of a sub-macroblock partition
subMbPartIdx.





Output of this process is the location ( x, y ) of the upper-left luma sample for the sub-macroblock partition subMbPartIdx
relative to the upper-left sample of the sub-macroblock.


The inverse sub-macroblock partition scanning process is specified as follows:

- If mb_type is equal to P_8x8, P_8x8ref0, or B_8x8,


x = InverseRasterScan( subMbPartIdx, SubMbPartWidth( sub_mb_type[ mbPartIdx ] ),
SubMbPartHeight( sub_mb_type[ mbPartIdx ] ), 8, 0 ) (J-1)


y = InverseRasterScan( subMbPartIdx, SubMbPartWidth( sub_mb_type[ mbPartIdx ] ),
SubMbPartHeight( sub_mb_type[ mbPartIdx ] ), 8, 1 ) (J-2)


- Otherwise, if both of the following are true:

   - mb_type is equal to B_8x8 and sub_mb_type[ mbPartIdx ] is equal to B_Direct_8x8, or mb_type is equal to
B_Skip, or mb_type is equal to B_Direct_16x16; and

   - MbVSSkipFlag is equal to 1 or mb_direct_type_flag is equal to 1,

the following applies:


x = InverseRasterScan( subMbPartIdx, 8, 8, 8, 0 ) (J-3)


y = InverseRasterScan( subMbPartIdx, 8, 8, 8, 1 ) (J-4)


- Otherwise,


x = InverseRasterScan( subMbPartIdx, 4, 4, 8, 0 ) (J-5)


y = InverseRasterScan( subMbPartIdx, 4, 4, 8, 1 ) (J-6)
