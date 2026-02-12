**8** **Decoding process**


Outputs of this process are decoded samples of the current picture (sometimes referred to by the variable CurrPic).


Depending on the value of chroma_format_idc, the number of sample arrays of the current picture is as follows:


- If chroma_format_idc is equal to 0, the current picture consists of 1 sample array SL.


- Otherwise (chroma_format_idc is not equal to 0), the current picture consists of 3 sample arrays SL, SCb, SCr.


This clause describes the decoding process, given syntax elements and upper-case variables from clause 7.


The decoding process is specified such that all decoders shall produce numerically identical results. Any decoding process
that produces identical results to the process described here conforms to the decoding process requirements of this
Recommendation | International Standard.


Each picture referred to in this clause is a complete primary coded picture or part of a primary coded picture. Each slice
referred to in this clause is a slice of a primary coded picture. Each slice data partition referred to in this clause is a slice
data partition of a primary coded picture.


Depending on the value of separate_colour_plane_flag, the decoding process is structured as follows:


- If separate_colour_plane_flag is equal to 0, the decoding process is invoked a single time with the current picture
being the output.


- Otherwise (separate_colour_plane_flag is equal to 1), the decoding process is invoked three times. Inputs to the
decoding process are all NAL units of the primary coded picture with identical value of colour_plane_id. The decoding
process of NAL units with a particular value of colour_plane_id is specified as if only a coded video sequence with
monochrome colour format with that particular value of colour_plane_id would be present in the bitstream. The output
of each of the three decoding processes is assigned to the 3 sample arrays of the current picture with the NAL units
with colour_plane_id equal to 0 being assigned to SL, the NAL units with colour_plane_id equal to 1 being assigned
to SCb, and the NAL units with colour_plane_id equal to 2 being assigned to SCr.

NOTE – The variable ChromaArrayType is derived as 0 when separate_colour_plane_flag is equal to 1 and
chroma_format_idc is equal to 3. In the decoding process, the value of this variable is evaluated resulting in operations
identical to that of monochrome pictures with chroma_format_idc being equal to 0.


An overview of the decoding process is given as follows:


1. The decoding of NAL units is specified in clause 8.1.


2. The processes in clause 8.2 specify decoding processes using syntax elements in the slice layer and above:


      - Variables and functions relating to picture order count are derived in clause 8.2.1. (only needed to be

invoked for one slice of a picture)


      - Variables and functions relating to the macroblock to slice group map are derived in clause 8.2.2. (only

needed to be invoked for one slice of a picture)


      - The method of combining the various slice data partitions when slice data partitioning is used is described

in clause 8.2.3.





      - When the frame_num of the current picture is not equal to PrevRefFrameNum and is not equal to

( PrevRefFrameNum + 1 ) % MaxFrameNum, the decoding process for gaps in frame_num is performed
according to clause 8.2.5.2 prior to the decoding of any slices of the current picture.


      - At the beginning of the decoding process for each P, SP, or B slice, the decoding process for reference

picture lists construction specified in clause 8.2.4 is invoked for derivation of reference picture list 0
(RefPicList0), and when decoding a B slice, reference picture list 1 (RefPicList1).


      - When the current picture is a reference picture and after all slices of the current picture have been decoded,

the decoded reference picture marking process in clause 8.2.5 specifies how the current picture is used in
the decoding process of inter prediction in later decoded pictures.


3. The processes in clauses 8.3, 8.4, 8.5, 8.6, and 8.7 specify decoding processes using syntax elements in the

macroblock layer and above.


      - The intra prediction process for I and SI macroblocks, except for I_PCM macroblocks as specified in

clause 8.3, has intra prediction samples as its output. For I_PCM macroblocks clause 8.3 directly specifies
a picture construction process. The output are constructed samples prior to the deblocking filter process.


      - The inter prediction process for P and B macroblocks is specified in clause 8.4 with inter prediction samples

being the output.


      - The transform coefficient decoding process and picture construction process prior to deblocking filter

process are specified in clause 8.5. That process derives samples for I and B macroblocks and for P
macroblocks in P slices. The output are constructed samples prior to the deblocking filter process.


      - The decoding process for P macroblocks in SP slices or SI macroblocks is specified in clause 8.6. That

process derives samples for P macroblocks in SP slices and for SI macroblocks. The output are constructed
samples prior to the deblocking filter process.


      - The constructed samples prior to the deblocking filter process that are next to the edges of blocks and

macroblocks are processed by a deblocking filter as specified in clause 8.7 with the output being the
decoded samples.
