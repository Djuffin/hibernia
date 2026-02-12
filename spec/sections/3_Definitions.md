**3** **Definitions**


For the purposes of this Recommendation | International Standard, the following definitions apply:


**3.1** **access unit** : A set of _NAL units_ that are consecutive in _decoding order_ and contain exactly one _primary coded_
_picture_ . In addition to the _primary coded picture_, an access unit may also contain one or more _redundant coded_
_pictures_, one _auxiliary coded picture_, or other _NAL units_ not containing _slices_ or _slice data partitions_ of a _coded_
_picture_ . The decoding of an access unit always results in a _decoded picture_ .


**3.2** **AC transform coefficient** : Any _transform coefficient_ for which the _frequency index_ in one or both dimensions
is non-zero.


**3.3** **adaptive binary arithmetic decoding process** : An entropy _decoding process_ that derives the values of _bins_
from a _bitstream_ produced by an _adaptive binary arithmetic encoding process_ .


**3.4** **adaptive binary arithmetic encoding process** : An entropy _encoding process_, not normatively specified in this
Recommendation | International Standard, that codes a sequence of _bins_ and produces a _bitstream_ that can be
decoded using the _adaptive binary arithmetic decoding process_ .


**3.5** **alpha blending** : A process not specified by this Recommendation | International Standard, in which an _auxiliary_
_coded_ _picture_ is used in combination with a _primary coded picture_ and with other data not specified by this
Recommendation | International Standard in the _display process._ In an alpha blending process, the samples of an
_auxiliary coded picture_ are interpreted as indications of the degree of opacity (or, equivalently, the degrees of
transparency) associated with the corresponding _luma_ samples of the _primary coded picture._


**3.6** **arbitrary slice order (ASO)** : A _decoding order_ of _slices_ in which the _macroblock address_ of the first
_macroblock_ of some _slice_ of a _slice group_ may be less than the _macroblock address_ of the first _macroblock_ of
some other preceding _slice_ of the same _slice group_ or, in the case of a _picture_ that is coded using three separate
colour planes, some other preceding _slice_ of the same _slice group_ within the same colour plane, or in which the
_slices_ of a _slice group_ of a picture may be interleaved with the _slices_ of one or more other _slice groups_ of the
_picture_ or, in the case of a _picture_ that is coded using three separate colour planes, with the _slices_ of one or more
other _slice groups_ within the same colour plane.


**3.7** **auxiliary coded picture** : A _picture_ that supplements the _primary coded picture_ that may be used in combination
with other data not specified by this Recommendation | International Standard in the _display process._ An
auxiliary coded picture has the same syntactic and semantic restrictions as a monochrome _redundant coded_
_picture._ An auxiliary coded picture must contain the same number of _macroblocks_ as the _primary coded picture_ .
Auxiliary coded pictures have no normative effect on the _decoding process_ . See also _primary coded picture_ and
_redundant coded picture._


**3.8** **azimuth circle** : circle on a sphere connecting all points with the same azimuth value.

NOTE – An azimuth circle is always a _great circle_ like a longitude line on the earth.


**3.9** **B slice** : A _slice_ that may be decoded using _intra_ _prediction_ or _inter prediction_ using at most two _motion vectors_
and _reference indices_ to _predict_ the sample values of each _block_ .


**3.10** **bin** : One bit of a _bin string_ .


**3.11** **binarization** : A set of _bin strings_ for all possible values of a _syntax element_ .


**3.12** **binarization process** : A unique mapping process of all possible values of a _syntax element_ onto a set of _bin_
_strings_ .


**3.13** **bin string** : A string of _bins_ . A bin string is an intermediate binary representation of values of _syntax elements_
from the _binarization_ of the _syntax element_ .


**3.14** **bi-predictive slice** : See _B slice_ **.**


**3.15** **bitstream** : A sequence of bits that forms the representation of _coded pictures_ and associated data forming one
or more _coded video sequences_ . Bitstream is a collective term used to refer either to a _NAL unit stream_ or a _byte_
_stream_ .


**3.16** **block** : An MxN (M-column by N-row) array of samples, or an MxN array of _transform coefficients_ .


**3.17** **bottom field** : One of two _fields_ that comprise a _frame_ . Each row of a _bottom field_ is spatially located immediately
below a corresponding row of a _top field_ .





**3.18** **bottom macroblock (of a macroblock pair)** : The _macroblock_ within a _macroblock pair_ that contains the
samples in the bottom row of samples for the _macroblock pair._ For a _field macroblock pair_, the bottom
macroblock represents the samples from the region of the _bottom field_ of the _frame_ that lie within the spatial
region of the _macroblock pair._ For a _frame macroblock pair_, the bottom macroblock represents the samples of
the _frame_ that lie within the bottom half of the spatial region of the _macroblock pair._


**3.19** **broken link** : A location in a _bitstream_ at which it is indicated that some subsequent _pictures_ in _decoding order_
may contain serious visual artefacts due to unspecified operations performed in the generation of the _bitstream_ .


**3.20** **byte** : A sequence of 8 bits, written and read with the most significant bit on the left and the least significant bit
on the right. When represented in a sequence of data bits, the most significant bit of a byte is first.


**3.21** **byte-aligned** : A position in a _bitstream_ is byte-aligned when the position is an integer multiple of 8 bits from
the position of the first bit in the _bitstream_ . A bit or _byte_ or _syntax element_ is said to be byte-aligned when the
position at which it appears in a _bitstream_ is byte-aligned.


**3.22** **byte stream** : An encapsulation of a _NAL unit stream_ containing _start code prefixes_ and _NAL units_ as specified
in Annex B.


**3.23** **can** : A term used to refer to behaviour that is allowed, but not necessarily required _._


**3.24** **category** : A number associated with each _syntax element_ . The category is used to specify the allocation of _syntax_
_elements_ to _NAL units_ for _slice data partitioning_ . It may also be used in a manner determined by the application
to refer to classes of _syntax elements_ in a manner not specified in this Recommendation | International Standard.


**3.25** **chroma** : An adjective specifying that a sample array or single sample is representing one of the two colour
difference signals related to the primary colours. The symbols used for a chroma array or sample are Cb and Cr.

NOTE – The term chroma is used rather than the term chrominance in order to avoid the implication of the use of linear
light transfer characteristics that is often associated with the term chrominance.


**3.26** **coded field** : A _coded representation_ of a _field_ .


**3.27** **coded frame** : A _coded representation_ of a _frame_ .


**3.28** **coded picture** : A _coded representation_ of a _picture_ . A coded picture may be either a _coded field_ or a _coded_
_frame_ . Coded picture is a collective term referring to a _primary coded picture_ or a _redundant coded picture_, but
not to both together.


**3.29** **coded picture buffer (CPB)** : A first-in first-out buffer containing _access units_ in _decoding order_ specified in
the _hypothetical reference decoder_ in Annex C.


**3.30** **coded representation** : A data element as represented in its coded form.


**3.31** **coded slice data partition NAL unit** : A _NAL unit_ containing a _slice data partition_ .


**3.32** **coded slice NAL unit** : A _NAL unit_ containing a _slice_ that is not a _slice_ of an _auxiliary coded picture_ .


**3.33** **coded video sequence** : A sequence of _access units_ that consists, in decoding order, of an _IDR access unit_
followed by zero or more non-IDR _access_ _units_ including all subsequent _access units_ up to but not including any
subsequent _IDR access unit_ .


**3.34** **component** : An array or single sample from one of the three arrays ( _luma_ and two _chroma_ ) that make up a _field_
or _frame_ in 4:2:0, 4:2:2, or 4:4:4 colour format or the array or a single sample of the array that make up a _field_
or _frame_ in monochrome format.


**3.35** **complementary field pair:** A collective term for a _complementary reference field pair_ or a _complementary non-_
_reference field pair_ .


**3.36** **complementary non-reference field pair** : Two _non-reference fields_ that are in consecutive _access units_ in
_decoding order_ as two _coded fields_ of opposite parity and share the same value of the frame_num _syntax element_,
where the first _field_ is not already a paired _field._


**3.37** **complementary reference field pair** : Two _reference fields_ that are in consecutive _access units_ in _decoding_
_order_ as two _coded fields_ and share the same value of the frame_num _syntax element_, where the second _field_ in
_decoding order_ is not an _IDR picture_ and does not include a memory_management_control_operation _syntax_
_element_ equal to 5 _._


**3.38** **constituent picture** : part of a spatially frame-packed stereoscopic video picture that corresponds to one view, or
a picture itself when frame packing is not in use or the temporal interleaving frame packing arrangement is in
use.





**3.39** **context variable** : A variable specified for the _adaptive binary arithmetic decoding_ _process_ of a _bin_ by an
equation containing recently decoded _bins_ .


**3.40** **DC transform coefficient** : A _transform coefficient_ for which the _frequency index_ is zero in all dimensions.


**3.41** **decoded picture** : A _decoded picture_ is derived by decoding a _coded picture_ . A _decoded picture_ is either a
decoded _frame_, or a decoded _field_ . A decoded _field_ is either a decoded _top field_ or a decoded _bottom field_ .


**3.42** **decoded picture buffer (DPB)** : A buffer holding _decoded pictures_ for reference, output reordering, or output
delay specified for the _hypothetical reference decoder_ in Annex C.


**3.43** **decoder** : An embodiment of a _decoding process_ .


**3.44** **decoder under test (DUT)** : A _decoder_ that is tested for conformance to this Recommendation | International
Standard by operating the _hypothetical stream scheduler_ to deliver a conforming _bitstream_ to the _decoder_ and to
the _hypothetical reference decoder_ and comparing the values and timing of the output of the two _decoders_ .


**3.45** **decoding order** : The order in which _syntax elements_ are processed by the _decoding process_ .


**3.46** **decoding process** : The process specified in this Recommendation | International Standard that reads a _bitstream_
and derives _decoded_ _pictures_ from it.


**3.47** **direct prediction** : An _inter prediction_ for a _block_ for which no _motion vector_ is decoded. Two direct _prediction_
modes are specified that are referred to as spatial direct _prediction_ and temporal _prediction_ mode.


**3.48** **display process** : A process not specified in this Recommendation | International Standard having, as its input,
the cropped decoded _pictures_ that are the output of the _decoding process._


**3.49** **elevation circle** : circle on a sphere connecting all points with the same elevation value.

NOTE – An elevation circle is similar to a lattitude line on the earth. Except when the elevation value is zero, an elevation
circle is not a _great circle_ like a longitude circle on the earth.


**3.50** **emulation prevention byte** : A _byte_ equal to 0x03 that may be present within a _NAL unit_ . The presence of
emulation prevention bytes ensures that no sequence of consecutive _byte-aligned_ _bytes_ in the _NAL unit_ contains
a _start code prefix_ .


**3.51** **encoder** : An embodiment of an _encoding process_ .


**3.52** **encoding process** : A process, not specified in this Recommendation | International Standard, that produces a
_bitstream_ conforming to this Recommendation | International Standard.


**3.53** **field** : An assembly of alternate rows of a _frame_ . A _frame_ is composed of two _fields_, a _top field_ and a _bottom field_ .


**3.54** **field macroblock** : A _macroblock_ containing samples from a single _field_ . All _macroblocks_ of a _coded field_ are
field macroblocks. When _macroblock-adaptive frame/field decoding_ is in use, some _macroblocks_ of a _coded_
_frame_ may be field macroblocks.


**3.55** **field macroblock pair** : A _macroblock pair_ decoded as two _field macroblocks._


**3.56** **field scan** : A specific sequential ordering of _transform coefficients_ that differs from the _zig-zag_ _scan_ by scanning
columns more rapidly than rows. Field scan is used for _transform coefficients_ in _field macroblocks._


**3.57** **flag** : A variable that can take one of the two possible values 0 and 1.


**3.58** **frame** : A _frame_ contains an array of _luma_ samples in monochrome format or an array of _luma_ samples and two
corresponding arrays of _chroma_ samples in 4:2:0, 4:2:2, and 4:4:4 colour format. A _frame_ consists of two _fields_,
a _top field_ and a _bottom field_ .


**3.59** **frame macroblock** : A _macroblock_ representing samples from the two _fields_ of a _coded frame_ . When
_macroblock-adaptive frame/field decoding_ is not in use, all _macroblocks_ of a _coded frame_ are frame
macroblocks. When _macroblock-adaptive frame/field decoding_ is in use, some _macroblocks_ of a _coded frame_
may be frame macroblocks.


**3.60** **frame macroblock pair** : A _macroblock pair_ decoded as two _frame macroblocks._


**3.61** **frequency index** : A one-dimensional or two-dimensional index associated with a _transform coefficient_ prior to
an _inverse transform_ part of the _decoding process._


**3.62** **global coordinate axes** : coordinate axes associated with _omnidirectional video_ that are associated with an
externally referenceable position and orientation.

NOTE – The global coordinate axes may correspond to the position and orientation of a device or rig used for
omnidirectional audio/video acquisition as well as the position of an observer's head in the three-dimensional space of
the _omnidirectional video_ rendering environment.





**3.63** **great circle** : intersection of a sphere and a plane that passes through the centre point of the sphere.

NOTE – A great circle is also known as an orthodrome or Riemannian circle.


**3.64** **hypothetical reference decoder (HRD)** : A hypothetical _decoder_ model that specifies constraints on the
variability of conforming _NAL unit streams_ or conforming _byte streams_ that an encoding process may produce.


**3.65** **hypothetical stream scheduler (HSS)** : A hypothetical delivery mechanism for the timing and data flow of the
input of a _bitstream_ into the _hypothetical reference decoder_ . The HSS is used for checking the conformance of
a _bitstream_ or a _decoder_ .


**3.66** **I slice** : A _slice_ that is not an _SI slice_ that is decoded using _intra prediction_ only.


**3.67** **informative** : A term used to refer to content provided in this Recommendation | International Standard that is
not an integral part of this Recommendation | International Standard. Informative content does not establish any
mandatory requirements for conformance to this Recommendation | International Standard.


**3.68** **instantaneous decoding refresh (IDR) access unit** : An _access unit_ in which the _primary coded picture_ is an
_IDR picture_ .


**3.69** **instantaneous decoding refresh (IDR) picture** : A _coded_ _picture_ for which the variable IdrPicFlag is equal to 1.
An IDR picture causes the _decoding process_ to mark all _reference pictures_ as "unused for reference" immediately
after the decoding of the IDR picture. All _coded pictures_ that follow an IDR picture in _decoding order_ can be
decoded without _inter prediction_ from any _picture_ that precedes the IDR picture in _decoding order_ . The first
_picture_ of each _coded video sequence_ in _decoding order_ is an IDR picture.


**3.70** **inter coding** : Coding of a _block_, _macroblock_, _slice_, or _picture_ that uses _inter prediction_ .


**3.71** **inter prediction** : A _prediction_ derived from decoded samples of _reference pictures_ other than the current
_decoded picture_ .


**3.72** **interpretation sample value** : A possibly-altered value corresponding to a decoded sample value of an _auxiliary_
_coded picture_ that may be generated for use in the _display process_ . Interpretation sample values are not used in
the _decoding process_ and have no normative effect on the _decoding process_ .


**3.73** **intra coding** : Coding of a _block, macroblock_, _slice_, or _picture_ that uses _intra prediction_ .


**3.74** **intra prediction** : A _prediction_ derived from the decoded samples of the same decoded _slice_ .


**3.75** **intra slice** : See _I slice_ .


**3.76** **inverse transform** : A part of the _decoding process_ by which a set of _transform coefficients_ are converted into
spatial-domain values, or by which a set of _transform coefficients_ are converted into _DC transform coefficients_ .


**3.77** **layer** : One of a set of syntactical structures in a non-branching hierarchical relationship. Higher layers contain
lower layers. The coding layers are the _coded video sequence_, _picture_, _slice_, and _macroblock_ layers.


**3.78** **level** : A defined set of constraints on the values that may be taken by the _syntax elements_ and variables of this
Recommendation | International Standard. The same set of levels is defined for all _profiles_, with most aspects of
the definition of each level being in common across different _profiles._ Individual implementations may, within
specified constraints, support a different level for each supported _profile_ . In a different context, a level is the
value of a _transform coefficient_ prior to _scaling_ (see the definition of _transform coefficient level_ ).


**3.79** **list:** A one-dimensional array of _syntax elements_ or variables.


**3.80** **list 0 (list 1) motion vector** : A _motion vector_ associated with a _reference index_ pointing into _reference picture_
_list 0_ ( _list 1_ ).


**3.81** **list 0 (list 1) prediction** : _Inter prediction_ of the content of a _slice_ using a _reference index_ pointing into _reference_
_picture list 0_ ( _list 1_ ).


**3.82** **local coordinate axes** : coordinate axes having a specified rotation relationship relative to the _global coordinate_
_axes_ .


**3.83** **luma** : An adjective specifying that a sample array or single sample is representing the monochrome signal related
to the primary colours. The symbol or subscript used for luma is Y or L.

NOTE – The term luma is used rather than the term luminance in order to avoid the implication of the use of linear light
transfer characteristics that is often associated with the term luminance. The symbol L is sometimes used instead of the
symbol Y to avoid confusion with the symbol y as used for vertical location.


**3.84** **macroblock** : A 16x16 _block_ of _luma_ samples and two corresponding _blocks_ of _chroma_ samples of a _picture_ that
has three sample arrays, or a 16x16 _block_ of samples of a monochrome _picture_ or a _picture_ that is coded using
three separate colour planes. The division of a _slice_ or a _macroblock pair_ into macroblocks is a _partitioning_ .





**3.85** **macroblock-adaptive frame/field decoding** : A _decoding process_ for _coded frames_ in which some _macroblocks_
may be decoded as _frame macroblocks_ and others may be decoded as _field macroblocks._


**3.86** **macroblock address** : When _macroblock-adaptive frame/field decoding_ is not in use, a macroblock address is
the index of a _macroblock_ in a _macroblock_ _raster scan_ of the _picture_ starting with zero for the top-left _macroblock_
in a _picture_ . When _macroblock-adaptive frame/field decoding_ is in use, the macroblock address of the _top_
_macroblock_ of a _macroblock pair_ is two times the index of the _macroblock pair_ in a _macroblock pair_ _raster scan_
of the _picture_, and the macroblock address of the _bottom macroblock_ of a _macroblock pair_ is the macroblock
address of the corresponding _top macroblock_ plus 1. The macroblock address of the _top macroblock_ of each
_macroblock pair_ is an even number and the macroblock address of the _bottom macroblock_ of each _macroblock_
_pair_ is an odd number.


**3.87** **macroblock location** : The two-dimensional coordinates of a _macroblock_ in a _picture_ denoted by ( x, y ). For the
top left _macroblock_ of the _picture_ ( x, y ) is equal to ( 0, 0 ). x is incremented by 1 for each _macroblock_ column
from left to right. When _macroblock-adaptive frame/field decoding_ is not in use, y is incremented by 1 for each
_macroblock_ row from top to bottom. When _macroblock-adaptive frame/field decoding_ is in use, y is incremented
by 2 for each _macroblock pair_ row from top to bottom, and is incremented by an additional 1 when a _macroblock_
is a _bottom macroblock_ .


**3.88** **macroblock pair** : A pair of vertically contiguous _macroblocks_ in a _frame_ that is coupled for use in _macroblock-_
_adaptive frame/field decoding_ . The division of a _slice_ into macroblock pairs is a _partitioning_ .


**3.89** **macroblock partition** : A _block_ of _luma_ samples and two corresponding _blocks_ of _chroma_ samples resulting
from a _partitioning_ of a _macroblock_ for _inter prediction_ for a _picture_ that has three sample arrays or a _block_ of
_luma_ samples resulting from a _partitioning_ of a _macroblock_ for _inter prediction_ for a monochrome _picture_ or a
_picture_ that is coded using three separate colour planes.


**3.90** **macroblock to slice group map** : A means of mapping _macroblocks_ of a _picture_ into _slice groups_ . The
macroblock to slice group map consists of a list of numbers, one for each coded _macroblock_, specifying the _slice_
_group_ to which each coded _macroblock_ belongs.


**3.91** **map unit to slice group map** : A means of mapping _slice group map units_ of a _picture_ into _slice groups_ . The
map unit to slice group map consists of a list of numbers, one for each _slice group map unit_, specifying the _slice_
_group_ to which each coded _slice group map unit_ belongs.


**3.92** **matrix** : A two-dimensional array of _syntax elements_ or variables.


**3.93** **may** : A term used to refer to behaviour that is allowed, but not necessarily required _._ In some places where the
optional nature of the described behaviour is intended to be emphasized, the phrase "may or may not" is used to
provide emphasis.


**3.94** **memory management control operation** : Seven operations that control _reference picture marking_ .


**3.95** **motion vector** : A two-dimensional vector used for _inter prediction_ that provides an offset from the coordinates
in the _decoded picture_ to the coordinates in a _reference picture_ .


**3.96** **must** : A term used in expressing an observation about a requirement or an implication of a requirement that is
specified elsewhere in this Recommendation | International Standard. This term is used exclusively in an
_informative_ context.


**3.97** **NAL unit** : A _syntax structure_ containing an indication of the type of data to follow and _bytes_ containing that
data in the form of an _RBSP_ interspersed as necessary with _emulation prevention bytes_ .


**3.98** **NAL unit stream** : A sequence of _NAL units_ .


**3.99** **non-paired field** : A collective term for a _non-paired reference field_ or a _non-paired non-reference field_ .


**3.100** **non-paired non-reference field** : A decoded _non-reference field_ that is not part of a _complementary_
_non-reference field pair._


**3.101** **non-paired reference field** : A decoded _reference field_ that is not part of a _complementary reference field pair._


**3.102** **non-reference field** : A _field_ coded with nal_ref_idc equal to 0.


**3.103** **non-reference frame** : A _frame_ coded with nal_ref_idc equal to 0.


**3.104** **non-reference picture** : A _picture_ coded with nal_ref_idc equal to 0. A _non-reference picture_ is not used for
_inter prediction_ of any other _pictures_ .


**3.105** **note** : A term used to prefix _informative_ remarks. This term is used exclusively in an _informative_ context.





**3.106** **omnidirectional video** : video content in a format that enables rendering according to the user's viewing
orientation, e.g., if viewed using a head-mounted device, or according to a user's desired _viewport_, reflecting a
potentially rotated viewing position.


**3.107** **opposite parity** : The _opposite parity_ of _top_ is _bottom_, and vice versa.


**3.108** **output order** : The order in which the _decoded_ _pictures_ are output from the _decoded picture buffer_ .


**3.109** **P slice** : A _slice_ that is not an _SP slice_ that may be decoded using _intra_ _prediction_ or _inter prediction_ using at
most one _motion vector_ and _reference index_ to _predict_ the sample values of each _block_ .


**3.110** **packed region** : region in a _region-wise packed picture_ that is mapped to a _projected region_ according to a _region-_
_wise packing_ .


**3.111** **parameter** : A _syntax element_ of a _sequence parameter set_ or a _picture parameter set_ . Parameter is also used as
part of the defined term _quantization parameter_ .


**3.112** **parity** : The parity of a _field_ can be _top_ or _bottom_ .


**3.113** **partitioning** : The division of a set into subsets such that each element of the set is in exactly one of the subsets.


**3.114** **picture** : A collective term for a _field_ or a _frame_ .


**3.115** **picture parameter set** : A _syntax structure_ containing _syntax elements_ that apply to zero or more entire _coded_
_pictures_ as determined by the pic_parameter_set_id _syntax element_ found in each _slice header._


**3.116** **picture order count** : A variable that is associated with each _coded field_ and each _field_ of a _coded frame_ and has
a value that is non-decreasing with increasing _field_ position in _output order_ relative to the first output _field_ of the
previous _IDR picture_ in _decoding order_ or relative to the first output _field_ of the previous _picture_, in _decoding_
_order_, that contains a _memory management control operation_ that marks all _reference pictures_ as "unused for
reference".


**3.117** **prediction** : An embodiment of the _prediction process_ .


**3.118** **prediction process** : The use of a _predictor_ to provide an estimate of the sample value or data element currently
being decoded.


**3.119** **predictive slice** : See _P slice_ .


**3.120** **predictor** : A combination of specified values or previously decoded sample values or data elements used in the
_decoding process_ of subsequent sample values or data elements.


**3.121** **primary coded picture** : The coded representation of a _picture_ to be used by the _decoding process_ for a bitstream
conforming to this Recommendation | International Standard. The primary coded picture contains all
_macroblocks_ of the _picture._ The only _pictures_ that have a normative effect on the _decoding process_ are primary
coded pictures. See also _redundant coded picture._


**3.122** **profile** : A specified subset of the syntax of this Recommendation | International Standard.


**3.123** **projected picture** : picture that uses a _projection_ format for _omnidirectional video_ .


**3.124** **projected region** : region in a _projected picture_ that is mapped to a _packed region_ according to a _region-wise_
_packing_ .


**3.125** **projection** : specified correspondence between the colour samples of a _projected picture_ and azimuth and
elevation positions on a sphere.


**3.126** **quantization parameter** : A variable used by the _decoding process_ for _scaling_ of _transform coefficient levels_ .


**3.127** **random access** : The act of starting the decoding process for a _bitstream_ at a point other than the beginning of
the stream.


**3.128** **raster scan** : A mapping of a rectangular two-dimensional pattern to a one-dimensional pattern such that the first
entries in the one-dimensional pattern are from the first top row of the two-dimensional pattern scanned from
left to right, followed similarly by the second, third, etc., rows of the pattern (going down) each scanned from
left to right.


**3.129** **raw byte sequence payload (RBSP)** : A _syntax structure_ containing an integer number of _bytes_ that is
encapsulated in a _NAL unit_ . An RBSP is either empty or has the form of a _string of data bits_ containing _syntax_
_elements_ followed by an _RBSP stop bit_ and followed by zero or more subsequent bits equal to 0.





**3.130** **raw byte sequence payload (RBSP) stop bit** : A bit equal to 1 present within a _raw byte sequence payload_
_(RBSP)_ after a _string of data bits_ . The location of the end of the _string of data bits_ within an _RBSP_ can be
identified by searching from the end of the _RBSP_ for the _RBSP stop bit_, which is the last non-zero bit in the
_RBSP._


**3.131** **recovery point** : A point in the _bitstream_ at which the recovery of an exact or an approximate representation of
the _decoded pictures_ represented by the _bitstream_ is achieved after a _random access_ or _broken link_ .


**3.132** **redundant coded picture** : A coded representation of a _picture_ or a part of a _picture._ The content of a redundant
coded picture shall not be used by the _decoding process_ for a _bitstream_ conforming to this
Recommendation | International Standard. A _redundant coded picture_ is not required to contain all _macroblocks_
in the _primary coded picture_ . Redundant coded pictures have no normative effect on the _decoding process_ . See
also _primary coded picture_ .


**3.133** **reference field** : A _reference field_ may be used for _inter prediction_ when _P_, _SP_, and _B slices_ of a _coded field_ or
_field macroblocks_ of a _coded frame_ are decoded. See also _reference picture_ .


**3.134** **reference frame** : A _reference frame_ may be used for _inter prediction_ when _P_, _SP_, and _B slices_ of a _coded frame_
are decoded. See also _reference picture_ .


**3.135** **reference index** : An index into a _reference picture list_ .


**3.136** **reference picture** : A _picture_ with nal_ref_idc not equal to 0. A _reference picture_ contains samples that may be
used for _inter prediction_ in the _decoding process_ of subsequent _pictures_ in _decoding order_ .


**3.137** **reference picture list** : A list of _reference pictures_ that is used for _inter prediction_ of a _P, B,_ or _SP slice._ For the
_decoding process_ of a _P_ or _SP slice,_ there is one reference picture list _._ For the _decoding process_ of a _B slice_,
there are two reference picture lists _._


**3.138** **reference picture list 0** : A _reference picture list_ used for _inter prediction_ of a _P_, _B_, or _SP_ _slice_ . All _inter_
_prediction_ used for _P_ and _SP_ _slices_ uses reference picture list 0. Reference picture list 0 is one of two _reference_
_picture lists_ used for _inter prediction_ for a _B slice_, with the other being _reference picture list 1_ .


**3.139** **reference picture list 1** : A _reference picture list_ used for _inter_ _prediction_ of a _B slice_ . Reference picture list 1 is
one of two _reference picture lists_ used for _inter prediction_ for a _B slice_, with the other being _reference picture_
_list 0_ .


**3.140** **reference picture marking** : Specifies, in the bitstream, how the _decoded pictures_ are marked for _inter_
_prediction_ .


**3.141** **region-wise packed picture** : decoded picture that contains one or more _packed regions_ .

NOTE – A packed picture may contain a _region-wise packing_ of a _projected picture_ .


**3.142** **region-wise packing** : transformation, resizing, and relocation of _packed regions_ of a _region-wise packed picture_
to remap the _packed regions_ to _projected region_ s of a _projected picture_ .


**3.143** **reserved** : The term reserved, when used in the clauses specifying some values of a particular _syntax element_, are
for future use by ITU-T | ISO/IEC. These values shall not be used in _bitstreams_ conforming to this
Recommendation | International Standard, but may be used in future extensions of this
Recommendation | International Standard by ITU-T | ISO/IEC.


**3.144** **residual** : The decoded difference between a _prediction_ of a sample or data element and its decoded value.


**3.145** **run** : A number of consecutive data elements represented in the decoding process. In one context, the number of
zero-valued _transform coefficient levels_ preceding a non-zero _transform coefficient level_ in the list of _transform_
_coefficient levels_ generated by a _zig-zag scan_ or a _field scan_ . In other contexts, run refers to a number of
_macroblocks_ .


**3.146** **sample aspect ratio** : Specifies, for assisting the _display process_, which is not specified in this
Recommendation | International Standard, the ratio between the intended horizontal distance between the
columns and the intended vertical distance between the rows of the _luma_ sample array in a _frame_ . Sample aspect
ratio is expressed as _h_ : _v_, where _h_ is horizontal width and _v_ is vertical height (in arbitrary units of spatial distance).


**3.147** **scaling** : The process of multiplying _transform coefficient levels_ by a factor, resulting in _transform coefficients_ .


**3.148** **sequence parameter set** : A _syntax structure_ containing _syntax elements_ that apply to zero or more entire _coded_
_video sequences_ as determined by the content of a seq_parameter_set_id _syntax element_ found in the _picture_
_parameter set_ referred to by the pic_parameter_set_id _syntax element_ found in each _slice header._





**3.149** **shall** : A term used to express mandatory requirements for conformance to this Recommendation | International
Standard. When used to express a mandatory constraint on the values of _syntax elements_ or on the results obtained
by operation of the specified _decoding process_, it is the responsibility of the _encoder_ to ensure that the constraint
is fulfilled. When used in reference to operations performed by the _decoding process_, any _decoding process_ that
produces identical results to the _decoding process_ described herein conforms to the _decoding process_
requirements of this Recommendation | International Standard _._


**3.150** **should** : A term used to refer to behaviour of an implementation that is encouraged to be followed under
anticipated ordinary circumstances, but is not a mandatory requirement for conformance to this Recommendation
| International Standard.


**3.151** **SI slice** : A _slice_ that is coded using _intra prediction_ only and using quantization of the _prediction_ samples. An
SI slice can be coded such that its decoded samples can be constructed identically to an _SP slice_ .


**3.152** **skipped macroblock** : A _macroblock_ for which no data is coded other than an indication that the _macroblock_ is
to be decoded as "skipped". This indication may be common to several _macroblocks_ .


**3.153** **slice** : An integer number of _macroblocks_ or _macroblock pairs_ ordered consecutively in the _raster scan_ within a
particular _slice group_ . For the _primary coded picture_, the division of each _slice group_ into slices is a _partitioning_ .
Although a slice contains _macroblocks_ or _macroblock pairs_ that are consecutive in the _raster scan_ within a _slice_
_group_, these _macroblocks_ or _macroblock pairs_ are not necessarily consecutive in the _raster scan_ within the
_picture_ . The _macroblock addresses_ are derived from the first _macroblock address_ in a slice (as represented in
the _slice header_ ) and the _macroblock to slice group map_, and, when a _picture_ is coded using three separate colour
planes, a colour plane identifier _._


**3.154** **slice data partition** : A non-empty subset of the _syntax elements_ of the slice data _syntax structure_ for a _slice_ . The
_syntax elements_ of a slice data partition are associated with the same _category_ .


**3.155** **slice data partitioning** : A method of _partitioning_ selected _syntax elements_ into _syntax structures_ based on a
_category_ associated with each _syntax element_ .


**3.156** **slice group** : A subset of the _macroblocks_ or _macroblock pairs_ of a _picture_ . The division of the _picture_ into slice
groups is a _partitioning_ of the _picture._ The _partitioning_ is specified by the _macroblock to slice group map_ .


**3.157** **slice group map units** : The units of the _map unit to slice group map._


**3.158** **slice header** : A part of a coded _slice_ containing the data elements pertaining to the first or all _macroblocks_
represented in the _slice_ .


**3.159** **source** : Term used to describe the video material or some of its attributes before encoding.


**3.160** **SP slice** : A _slice_ that may be coded using _intra prediction_ or _inter prediction_ with quantization of the _prediction_
samples using at most one _motion vector_ and _reference index_ to _predict_ the sample values of each _block._ An SP
slice can be coded such that its decoded samples can be constructed identically to another SP slice or an _SI slice_ .


**3.161** **sphere coordinates** : azimuth and elevation angles identifying a location of a point on a sphere.


**3.162** **sphere region** : region on a sphere, specified either by four _great circles_ or by two _azimuth circles_ and two
_elevation circles_, or such a region on a rotated sphere after applying yaw, pitch, and roll rotations.


**3.163** **start code prefix** : A unique sequence of three _bytes_ equal to 0x000001 embedded in the _byte stream_ as a prefix
to each _NAL unit._ The location of a start code prefix can be used by a _decoder_ to identify the beginning of a new
_NAL unit_ and the end of a previous _NAL unit_ . Emulation of start code prefixes is prevented within _NAL units_ by
the inclusion of _emulation prevention bytes_ .


**3.164** **string of data bits (SODB)** : A sequence of some number of bits representing _syntax elements_ present within a
_raw byte sequence payload_ prior to the _raw byte sequence payload stop bit._ Within an SODB, the left-most bit
is considered to be the first and most significant bit, and the right-most bit is considered to be the last and least
significant bit.


**3.165** **sub-macroblock** : One quarter of the samples of a _macroblock_, i.e., an 8x8 _luma_ _block_ and two corresponding
_chroma_ _blocks_ of which one corner is located at a corner of the _macroblock_ for a _picture_ that has three sample
arrays or an 8x8 _luma block_ of which one corner is located at a corner of the _macroblock_ for a monochrome
_picture_ or a _picture_ that is coded using three separate colour planes.


**3.166** **sub-macroblock partition** : A _block_ of _luma_ samples and two corresponding _blocks_ of _chroma_ samples resulting
from a _partitioning_ of a _sub-macroblock_ for _inter prediction_ for a _picture_ that has three sample arrays or a _block_
of _luma_ samples resulting from a _partitioning_ of a _sub-macroblock_ for _inter prediction_ for a monochrome _picture_
or a _picture_ that is coded using three separate colour planes.


**3.167** **switching I slice** : See _SI slice_ .





**3.168** **switching P slice** : See _SP slice_ .


**3.169** **syntax element** : An element of data represented in the _bitstream_ .


**3.170** **syntax structure** : Zero or more _syntax elements_ present together in the _bitstream_ in a specified order _._


**3.171** **tilt angle** : angle indicating the amount of tilt of a _sphere region_, measured as the amount of rotation of a _sphere_
_region_ along the axis originating from the sphere origin passing through the centre point of the _sphere region_,
where the angle value increases clockwise when looking from the origin towards the positive end of the axis.


**3.172** **top field** : One of two _fields_ that comprise a _frame_ . Each row of a _top field_ is spatially located immediately above
the corresponding row of the _bottom field_ .


**3.173** **top macroblock (of a macroblock pair)** : The _macroblock_ within a _macroblock pair_ that contains the samples
in the top row of samples for the _macroblock pair._ For a _field macroblock pair_, the top macroblock represents
the samples from the region of the _top field_ of the _frame_ that lie within the spatial region of the _macroblock pair._
For a _frame macroblock pair_, the top macroblock represents the samples of the _frame_ that lie within the top half
of the spatial region of the _macroblock pair._


**3.174** **transform coefficient** : A scalar quantity, considered to be in a frequency domain, that is associated with a
particular one-dimensional or two-dimensional _frequency index_ in an _inverse transform_ part of the _decoding_
_process_ .


**3.175** **transform coefficient level** : An integer quantity representing the value associated with a particular
two-dimensional frequency index in the _decoding process_ prior to _scaling_ for computation of a _transform_
_coefficient_ value.


**3.176** **universal unique identifier (UUID)** : An identifier that is unique with respect to the space of all universal unique
identifiers.


**3.177** **unspecified** : The term unspecified, when used in the clauses specifying some values of a particular _syntax_
_element_, indicates that the values have no specified meaning in this Recommendation | International Standard
and will not have a specified meaning in the future as an integral part of this Recommendation | International
Standard.


**3.178** **variable length coding (VLC)** : A reversible procedure for entropy coding that assigns shorter bit strings to
_symbols_ expected to be more frequent and longer bit strings to _symbols_ expected to be less frequent.


**3.179** **VCL NAL unit** : A collective term for _coded slice NAL units_ and _coded slice data partition NAL units_ .


**3.180** **viewport** : region of _omnidirectional video_ content suitable for display and viewing by the user.


**3.181** **zig-zag scan** : A specific sequential ordering of _transform coefficient levels_ from (approximately) the lowest
spatial frequency to the highest. Zig-zag scan is used for _transform coefficient levels_ in _frame macroblocks._
