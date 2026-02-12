**G.3** **Definitions**


For the purpose of this annex, the following definitions apply in addition to the definitions in clause 3. These definitions
are either not present in clause 3 or replace definitions in clause 3.


**G.3.1** **arbitrary slice order (ASO)** : A _decoding order_ of _slices_ in which the _macroblock address_ of the first _macroblock_
of some _slice_ of a _slice group_ within a _layer representation_ may be less than the _macroblock address_ of the first
_macroblock_ of some other preceding _slice_ of the same _slice group_ within the same _layer representation_ or in
which the _slices_ of a _slice group_ within a _layer representation_ may be interleaved with the _slices_ of one or more
other _slices groups_ within the same _layer representation_ .


**G.3.2** **associated NAL unit** : A _NAL unit_ that directly succeeds a _prefix NAL unit_ in _decoding order_ .


**G.3.3** **B slice** : A _slice_ that may be decoded using _intra-layer intra_ _prediction_ or _inter prediction_ using at most two _motion_
_vectors_ and _reference indices_ to _predict_ the sample values of each _block_ .


**G.3.4** **base layer** : A _bitstream subset_ that contains all _NAL units_ with the nal_unit_type _syntax element_ equal to 1 and 5
of the _bitstream_ and does not contain any _NAL unit_ with the nal_unit_type _syntax element_ equal to 14, 15, or 20
and conforms to one or more of the profiles specified in Annex A.


**G.3.5** **base quality layer representation** : The _layer representation_ of the _target dependency representation_ of an _access_
_unit_ that is associated with the quality_id _syntax element_ equal to 0.


**G.3.6** **bitstream subset** : A _bitstream_ that is derived as a _subset_ from a _bitstream_ by discarding zero or more _NAL units_ .
A _bitstream subset_ is also referred to as _sub-bitstream_ .


**G.3.7** **bottom macroblock (of a macroblock pair)** : The _macroblock_ within a _macroblock pair_ that contains the samples
in the bottom row of samples for the _macroblock pair._ For a _field macroblock pair_, the bottom macroblock
represents the samples from the region of the _bottom field_ or _layer bottom field_ of the _frame_ or _layer frame_,
respectively, that lie within the spatial region of the _macroblock pair._ For a _frame macroblock pair_, the bottom
macroblock represents the samples of the _frame_ or _layer frame_ that lie within the bottom half of the spatial region
of the _macroblock pair._


**G.3.8** **coded slice in scalable extension NAL unit** : A _coded slice NAL unit_ that contains an _EI slice_, _EP slice_, or an _EB_
_slice_ .


**G.3.9** **complementary reference field pair** : A collective term for two _reference fields_ that are in consecutive _access_
_units_ in _decoding order_ as two _coded fields_, where the _target dependency representations_ of the _fields_ share the
same value of the frame_num _syntax element_ and where the second _field_ in _decoding order_ is not an _IDR picture_
and the _target_ _dependency_ _representation_ of the second _field_ does not include a
memory_management_control_operation _syntax element_ equal to 5, or a _complementary reference base field pair_ .


**G.3.10** **complementary reference base field pair** : Two _reference base fields_ that are associated with two _coded fields_

that are in consecutive _access units_ in _decoding order_, where the _target dependency representations_ of the _coded_
_fields_ share the same value of the frame_num _syntax element_ and where the second _coded field_ in _decoding order_
is not an _IDR picture_ and the _target dependency representation_ of the second _coded field_ does not include a





memory_management_control_operation _syntax element_ equal to 5. A _complementary reference base field pair_
is a _complementary reference field pair_ .


**G.3.11** **dependency representation** : A _subset_ of _VCL NAL units_ within an _access unit_ that are associated with the same

value of the dependency_id _syntax element_, which is provided as part of the _NAL unit_ header or by an associated
_prefix NAL unit,_ and the same value of the redundant_pic_cnt _syntax element_ . A _dependency representation_
consists of one or more _layer representations_ .


**G.3.12** **EB slice** : A _slice_ that may be decoded using _intra_ _prediction_ or _inter prediction_ or _inter-layer prediction_ from

_syntax elements_ and derived variables of the _reference layer representation_ . For _inter-prediction_ of EB slices at
most two _motion vectors_ and _reference indices_ are used to _predict_ the sample values of each _block_ .


**G.3.13** **EI slice** : A _slice_ that is not an _I slice_ or _SI slice_ that is decoded using _intra_ _prediction_ only.


**G.3.14** **EP slice** : A _slice_ that may be decoded using _intra_ _prediction_ or _inter prediction_ or _inter-layer prediction_ from

_syntax elements_ and derived variables of the _reference layer representation_ . For _inter-prediction_ of EP slices at
most one _motion vector_ and _reference index_ is used to _predict_ the sample values of each _block._


**G.3.15** **field macroblock** : A _macroblock_ containing samples from a single _field_ or _layer field_ .


**G.3.16** **frame macroblock** : A _macroblock_ containing samples from the two _fields_ or _layer fields_ of a _frame_ or _layer_

_frame_, respectively.


**G.3.17** **I slice** : A _slice_ that is decoded using _intra-layer intra_ _prediction_ only.


**G.3.18** **instantaneous decoding refresh (IDR) picture** : A _coded_ _picture_ for which the variable IdrPicFlag is equal to 1

for the _target dependency representation_ . An IDR picture causes the _decoding process_ to mark all _reference_
_pictures_ as "unused for reference" immediately after the decoding of the IDR picture. All _coded pictures_ that
follow an IDR picture in _decoding order_ can be decoded without _inter prediction_ from any _picture_ that precedes
the IDR picture in _decoding order_ . The first _picture_ of each _coded video sequence_ in _decoding order_ is an IDR
picture.


**G.3.19** **inter-layer intra prediction** : An _inter-layer prediction_ derived from decoded samples of _intra-coded_

_macroblocks_ of the _reference layer representation_ .


**G.3.20** **inter-layer prediction** : A _prediction_ derived from _syntax elements_, derived variables, or decoded samples of the

_reference layer representation_ .


**G.3.21** **intra-layer intra prediction** : A _prediction_ derived from decoded samples of the same decoded _slice_ .


**G.3.22** **intra prediction** : A collective term for _intra-layer intra prediction_ or _inter-layer intra prediction_ or a combination

of _intra-layer intra prediction_ together with _inter-layer prediction_ from _syntax elements_ and derived variables of
the _reference layer representation_ .


**G.3.23** **intra slice** : A collective term for _I slice_ or _EI slice_ .


**G.3.24** **layer bottom field** : One of two _layer fields_ that comprise a _layer frame_ . Each row of a layer bottom field is

spatially located immediately below a corresponding row of a _layer top field_ .


**G.3.25** **layer field** : An assembly of alternate rows of a _layer frame_ . A _layer frame_ is composed of two _layer fields_, a _layer_

_top field_ and a _layer bottom field_ .


**G.3.26** **layer frame** : A _layer frame_ contains an array of _luma_ samples that represents an intermediate decoding result for

a _field_ or a _frame_ in monochrome format or an array of _luma_ samples and two corresponding arrays of _chroma_
samples that represent an intermediate decoding result for a _field_ or a _frame_ in 4:2:0, 4:2:2, and 4:4:4 colour
format. A _layer frame_ consists of two _layer fields_, a _layer top field_ and a _layer bottom field_ .


**G.3.27** **layer picture** : A collective term for a _layer field_ or a _layer frame_ .


**G.3.28** **layer top field** : One of two _layer fields_ that comprise a _layer frame_ . Each row of a _layer top field_ is spatially

located immediately above a corresponding row of a _layer bottom field_ .


**G.3.29** **layer representation** : A subset of _VCL NAL units_ within an _access unit_ that are associated with the same values

of the dependency_id and quality_id _syntax elements_, which are provided as part of the _VCL NAL unit_ header or
by an associated _prefix NAL unit_, and the same value of the redundant_pic_cnt _syntax element_ . One or more _layer_
_representations_ represent a _dependency representation_ .


**G.3.30** **layer representation identifier** : An integer value by which a particular _layer representation_ inside a _coded_

_picture_ is uniquely identified.





**G.3.31** **macroblock** : A 16x16 _block_ of _luma_ samples and two corresponding _blocks_ of _chroma_ samples of a _picture_ or

_layer picture_ that has three sample arrays, or a 16x16 _block_ of samples of a monochrome _picture_ or _layer picture_ .
The division of a _slice_ or a _macroblock pair_ into macroblocks is a _partitioning_ .


**G.3.32** **macroblock-adaptive frame/field decoding** : A _decoding process_ for _coded frames_ or _layer representations_ in

which some _macroblocks_ may be decoded as _frame macroblocks_ and others may be decoded as _field macroblocks._


**G.3.33** **macroblock address** : When _macroblock-adaptive frame/field decoding_ is not in use, a macroblock address is the

index of a macroblock in a _macroblock_ _raster scan_ of the _picture_ or _layer picture_ starting with zero for the topleft _macroblock_ in a _picture_ or _layer picture_ . When _macroblock-adaptive frame/field decoding_ is in use, the
macroblock address of the _top macroblock_ of a _macroblock pair_ is two times the index of the _macroblock pair_ in
a _macroblock pair_ _raster scan_ of the _picture_ or _layer picture_, and the macroblock address of the _bottom_
_macroblock_ of a _macroblock pair_ is the macroblock address of the corresponding _top macroblock_ plus 1. The
macroblock address of the _top macroblock_ of each _macroblock pair_ is an even number and the macroblock address
of the _bottom macroblock_ of each _macroblock pair_ is an odd number.


**G.3.34** **macroblock location** : The two-dimensional coordinates of a _macroblock_ in a _picture_ or _layer picture_ denoted by

( x, y ). For the top left _macroblock_ of the _picture_ or _layer picture_ ( x, y ) is equal to ( 0, 0 ). x is incremented by
1 for each _macroblock_ column from left to right. When _macroblock-adaptive frame/field decoding_ is not in use,
y is incremented by 1 for each _macroblock_ row from top to bottom. When _macroblock-adaptive frame/field_
_decoding_ is in use, y is incremented by 2 for each _macroblock pair_ row from top to bottom, and is incremented
by an additional 1 when a macroblock is a _bottom macroblock_ .


**G.3.35** **macroblock pair** : A pair of vertically contiguous _macroblocks_ in a _frame_ or _layer frame_ that is coupled for use

in _macroblock-adaptive frame/field decoding_ . The division of a _slice_ into macroblock pairs is a _partitioning_ .


**G.3.36** **macroblock to slice group map** : A means of mapping _macroblocks_ of a _picture_ or _layer picture_ into _slice groups_ .

The macroblock to slice group map consists of a list of numbers, one for each coded _macroblock_, specifying the
_slice group_ to which each coded _macroblock_ belongs.


**G.3.37** **map unit to slice group map** : A means of mapping _slice group map units_ of a _picture_ or _layer picture_ into _slice_

_groups_ . The map unit to slice group map consists of a list of numbers, one for each _slice group map unit_, specifying
the _slice group_ to which each coded _slice group map unit_ belongs to.


**G.3.38** **non-paired reference base field** : A _reference base field_ that is not part of a _complementary reference base field_

_pair_ . A non-paired reference base field is a _non-paired reference field_ .


**G.3.39** **P slice** : A _slice_ that may be decoded using _intra-layer intra_ _prediction_ or _inter prediction_ using at most one _motion_

_vector_ and _reference index_ to _predict_ the sample values of each _block_ .


**G.3.40** **parameter** : A _syntax element_ of an _SVC sequence parameter set_ or a _picture parameter set_ . Parameter is also

used as part of the defined term _quantization parameter_ .


**G.3.41** **picture parameter set** : A _syntax structure_ containing _syntax elements_ that apply to zero or more _layer_

_representations_ as determined by the pic_parameter_set_id _syntax element_ found in each _slice header._


**G.3.42** **prefix NAL unit** : A _NAL unit_ with nal_unit_type equal to 14 that immediately precedes in _decoding order_ a _NAL_

_unit_ with nal_unit_type equal to 1 or 5. The _NAL unit_ that immediately succeeds the prefix NAL unit in _decoding_
_order_ is referred to as the _associated NAL unit_ . The prefix NAL unit contains data associated with the _associated_
_NAL unit_, which are considered to be part of the _associated NAL unit_ .


**G.3.43** **reference base field** : A _reference field_ that is obtained by decoding a _base quality_ _layer representation_ with the

nal_ref_idc _syntax element_ not equal to 0, the store_ref_base_pic_flag _syntax element_ equal to 1, and the
field_pic_flag _syntax element_ equal to 1 of a _coded picture_ and all _layer representations_ of the _coded picture_ that
are referred to by _inter-layer prediction_ in the _base quality layer representation_ . A reference base field is not a
_decoded picture_ and it is not an output of the _decoding process_, but may be used for _inter prediction_ when _P_, _B_,
_EP_, and _EB slices_ of a _coded field_ or a _field macroblock_ of a _coded frame_ are decoded. See also _reference base_
_picture_ .


**G.3.44** **reference base frame** : A _reference frame_ that is obtained by decoding a _base quality_ _layer representation_ with

the nal_ref_idc _syntax element_ not equal to 0, the store_ref_base_pic_flag _syntax element_ equal to 1, and the
field_pic_flag _syntax element_ equal to 0 of a _coded picture_ and all _layer representations_ of the _coded picture_ that
are referred to by _inter-layer prediction_ of the _base quality layer representation_ . A reference base frame is not a
_decoded picture_ and it is not an output of the _decoding process_, but may be used for _inter prediction_ when _P_, _B_,
_EP_, and _EB slices_ of a _coded frame_ are decoded. See also _reference base picture_ .


**G.3.45** **reference base picture** : A collective term for a _reference base field_ or a _reference base frame_ .





**G.3.46** **reference field** : A _reference field_ may be used for _inter prediction_ when _P_, _B, EP,_ or _EB slices_ of a _coded field_

or _field macroblocks_ of a _coded frame_ are decoded. See also _reference picture_ .


**G.3.47** **reference frame** : A _reference frame_ may be used for _inter prediction_ when _P_, _B, EP,_ or _EB slices_ of a _coded_

_frame_ are decoded. See also _reference picture_ .


**G.3.48** **reference layer macroblock** : A _macroblock_ of a _reference layer representation_ .


**G.3.49** **reference layer representation** : A reference layer representation for a particular _layer representation_ of a _coded_

_picture_ is the _layer representation_ that is used for _inter-layer prediction_ of the particular _layer representation_ .
The reference layer representation belongs to the same _access unit_ as the _layer representation_ that uses the
reference layer representation for _inter-layer prediction_ .


**G.3.50** **reference picture** : A collective term for a _decoded picture_ that is obtained by decoding a _coded picture_ for which

the nal_ref_idc _syntax element_ that is associated with the _target dependency representation_ is not equal to 0 or a
_reference base picture_ . A reference picture contains samples that may be used for _inter prediction_ in the _decoding_
_process_ of subsequent _pictures_ in _decoding order_ .


**G.3.51** **reference picture list** : A list of _reference pictures_ that is used for _inter prediction_ of a _P, B, EP,_ or _EB slice._ For

the _decoding process_ of a _P_ or _EP slice,_ there is one reference picture list _._ For the _decoding process_ of a _B_ or _EB_
_slice_, there are two reference picture lists _._


**G.3.52** **reference picture list 0** : A _reference picture list_ used for _inter prediction_ of a _P_, _B_, _EP,_ or _EB_ _slice_ . All _inter_

_prediction_ used for _P_ and _EP_ _slices_ uses reference picture list 0. Reference picture list 0 is one of two _reference_
_picture lists_ used for _inter prediction_ for a _B_ or _EB slice_, with the other being _reference picture list 1_ .


**G.3.53** **reference picture list 1** : A _reference picture list_ used for _inter_ _prediction_ of a _B_ or _EB slice_ . Reference picture

list 1 is one of two _reference picture lists_ used for _inter prediction_ for a _B_ or _EB slice_, with the other being
_reference picture list 0_ .


**G.3.54** **scalable bitstream** : A _bitstream_ with the property that one or more _bitstream subsets_ that are not identical to the

scalable bitstream form another _bitstream_ that conforms to this specification.


**G.3.55** **sequence parameter set** : A _syntax structure_ containing _syntax elements_ that apply to zero or more _layer_

_representations_ with the dependency_id _syntax element_ equal to 0 and the quality_id _syntax element_ equal to 0 as
determined by the content of a seq_parameter_set_id _syntax element_ found in the _picture parameter set_ referred
to by the pic_parameter_set_id _syntax element_ found in each _slice header_ of _I_, _P_, and _B slices._


**G.3.56** **slice** : An integer number of _macroblocks_ or _macroblock pairs_ ordered consecutively in the _raster scan_ within a

particular _slice group_ . Each _macroblock_ or _macroblock pair_ of a _picture_ or _layer picture_ shall not be contained in
more than one slice of a particular _layer representation_ . Although a slice contains _macroblocks_ or _macroblock_
_pairs_ that are consecutive in the _raster scan_ within a _slice group_, these _macroblocks_ or _macroblock pairs_ are not
necessarily consecutive in the _raster scan_ within the _picture_ or _layer picture_ . The _macroblock addresses_ are
derived from the first _macroblock address_ in a slice (as represented in the _slice header_ ) and the _macroblock to_
_slice group map._


**G.3.57** **slice group** : A subset of the _macroblocks_ or _macroblock pairs_ of a _picture_ or _layer picture_ . The division of the

_picture_ or _layer picture_ into slice groups is a _partitioning_ of the _picture_ or _layer picture._ The partitioning is
specified by the _macroblock to slice group map_ .


**G.3.58** **spatial intra prediction** : See _intra-layer intra prediction_ .


**G.3.59** **sub-bitstream** : A _subset_ of a _bitstream_ . A sub-bitstream is also referred to as _bitstream subset_ .


**G.3.60** **subset** : A subset contains only elements that are also contained in the set from which the subset is derived. The

subset may be identical to the set from which it is derived.


**G.3.61** **subset sequence parameter set** : A _syntax structure_ containing _syntax elements_ that apply to zero or more _layer_

_representations_ with the dependency_id _syntax element_ not equal to 0 or the quality_id _syntax element_ not equal
to 0 as determined by the content of a seq_parameter_set_id _syntax element_ found in the _picture parameter set_
referred to by the pic_parameter_set_id _syntax element_ found in each _slice header_ of _EI_, _EP_, and _EB slices._


**G.3.62** **SVC sequence parameter set** : A collective term for _sequence parameter set_ or _subset sequence parameter set_ .


**G.3.63** **SVC sequence parameter set RBSP** : A collective term for sequence parameter set RBSP or subset sequence

parameter set RBSP.


**G.3.64** **target dependency representation** : The _dependency representation_ of a _coded picture_ that is associated with the

largest value of the dependency_id _syntax element_ for all _dependency representations_ of the _coded picture_ .





**G.3.65** **target layer representation** : The _layer representation_ of the _target dependency representation_ of a _coded picture_

that is associated with the largest value of the quality_id _syntax element_ for all _layer representations_ of the _target_
_dependency representation_ of the _coded picture_ .


**G.3.66** **top macroblock (of a macroblock pair)** : The _macroblock_ within a _macroblock pair_ that contains the samples in

the top row of samples for the _macroblock pair._ For a _field macroblock pair_, the top macroblock represents the
samples from the region of the _top field_ or _layer top field_ of the _frame_ or _layer frame_ that lie within the spatial
region of the _macroblock pair._ For a _frame macroblock pair_, the top macroblock represents the samples of the
_frame_ or _layer frame_ that lie within the top half of the spatial region of the _macroblock pair._


**G.3.67** **VCL NAL unit** : A collective term for _coded slice NAL units_ and _prefix NAL units_ .
