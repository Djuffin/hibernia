**H.3** **Definitions**


For the purpose of this annex, the following definitions apply in addition to the definitions in clause 3. These definitions
are either not present in clause 3 or replace definitions in clause 3.


**H.3.1** **access unit** : A set of _NAL units_ that are consecutive in _decoding order_ and contain exactly one _primary coded_
_picture_ consisting of one or more _view components_ . In addition to the _primary coded picture_, an access unit may
also contain one or more _redundant coded pictures_, one _auxiliary coded picture_, or other _NAL units_ not containing
_slices_ or _slice data partitions_ of a _coded picture_ . The decoding of an access unit always results in one _decoded_
_picture_ consisting of one or more _decoded view components_ .


**H.3.2** **anchor access unit** : An _access unit_ in which the _primary_ _coded picture_ is an _anchor picture_ .


**H.3.3** **anchor picture** : A _coded picture_ in which all _slices_ may reference only _slices_ within the same _access unit_, i.e.,
_inter-view prediction_ may be used, but no _inter prediction_ is used, and all following _coded pictures_ in output order
do not use _inter prediction_ from any _picture_ prior to the _coded picture_ in _decoding order_ . The value of
anchor_pic_flag is equal to 1 for all the _prefix NAL units_ (when present) and all the slice extension NAL units
that are contained in an anchor picture.


**H.3.4** **anchor view component** : A _view component_ in an _anchor picture_ . All _view components_ in an _anchor picture_ are
anchor view components.


**H.3.5** **associated NAL unit** : A _NAL unit_ that immediately follows a _prefix NAL unit_ in _decoding order_ .


**H.3.6** **base view** : A _view_ that has the minimum value of _view order index_ in a _coded video sequence_ . The base view can
be decoded independently of other _views_, does not use _inter-view prediction,_ and contains _VCL NAL units_ only
with nal_unit_type equal to 1, 5, or 14. The _bitstream subset_ corresponding to the base view conforms to one or
more of the _profiles_ specified in Annex A. There is only one base view in a _coded video sequence_ .


**H.3.7** **bitstream subset** : A _bitstream_ that is derived as a _subset_ from a _bitstream_ by discarding zero or more _NAL units_ .
A _bitstream subset_ is also referred to as a _sub-bitstream_ .


**H.3.8** **coded slice MVC extension NAL unit** : A _coded slice NAL unit_ that has nal_unit_type equal to 20.


**H.3.9** **decoded view component** : A decoded view component is derived by decoding a _view component_ . A decoded
view component is either a decoded _frame view component_, or a decoded _field view component_ .


**H.3.10** **direct prediction** : An _inter prediction_ or _inter-view prediction_ for a _block_ for which no _motion vector_ is decoded.

Two direct prediction modes are specified that are referred to as spatial direct prediction mode and temporal direct
prediction mode.


**H.3.11** **field view component** : A _view component_ of a _field_ .


**H.3.12** **frame view component** : A _view component_ of a _frame_ .


**H.3.13** **instantaneous decoding refresh (IDR) view component** : A _view component_ in an _IDR picture_ . All _view_

_components_ in an _IDR picture_ are IDR view components. IDR view components are also _anchor view components_,
and _inter-view prediction_ may be used for IDR view components that are part of a _non-base view_ .


**H.3.14** **inter-view coding** : Coding of a _block_, _macroblock_, _slice_, or _picture_ that uses _inter-view prediction_ .





**H.3.15** **inter-view only reference component** : A _view component_ coded with nal_ref_idc equal to 0 and inter_view_flag

equal to 1. An inter-view only reference component contains samples that may be used for _inter-view prediction_
in the _decoding process_ of subsequent _view components_ in _decoding order_, but are not used for _inter prediction_
by any _view components_ . Inter-view only reference components are _non-reference pictures_ .


**H.3.16** **inter-view prediction** : A _prediction_ derived from decoded samples of _inter-view reference components_ or

_inter-view only reference components_ for decoding another _view component_ in the same _access unit_ .


**H.3.17** **inter-view prediction reference** : A collective term for _inter-view reference components_ or _inter-view only_

_reference components_ .


**H.3.18** **inter-view reference component** : A _view component_ coded with nal_ref_idc greater than 0 and inter_view_flag

equal to 1. An inter-view reference component contains samples that may be used for _inter prediction_ of
subsequent _pictures_ in _decoding order_ and _inter-view prediction_ of subsequent _view components_ in _decoding_
_order_ . Inter-view reference components are _reference pictures_ .


**H.3.19** **left view:** The left part of a picture coded in a frame-packed manner with the side-by-side frame packing

arrangement type or the top part of a picture coded in a frame-packed manner with the top-bottom frame packing
arrangement type.


**H.3.20** **list 0 (list 1) prediction** : _Inter prediction or inter-view prediction_ of the content of a _slice_ using a _reference index_

pointing into _reference picture list 0 (list 1)_ .


**H.3.21** **macroblock partition** : A _block_ of _luma_ samples and two corresponding _blocks_ of _chroma_ samples resulting from

a _partitioning_ of a _macroblock_ for _inter prediction or inter-view prediction_ .


**H.3.22** **motion vector** : A two-dimensional vector used for _inter prediction or inter-view_ prediction that provides an offset

from the coordinates in the _decoded view component_ to the coordinates in a _reference picture_ or _inter-view only_
_reference component_ .


**H.3.23** **MVC sequence parameter set** : A collective term for _sequence parameter set_ or _subset sequence parameter set_ .


**H.3.24** **MVC sequence parameter set RBSP** : A collective term for sequence parameter set RBSP or subset sequence

parameter set RBSP.


**H.3.25** **non-anchor access unit** : An _access unit_ that is not an _anchor access unit_ .


**H.3.26** **non-anchor picture** : A _coded picture_ that is not an _anchor picture_ .


**H.3.27** **non-anchor view component** : A _view component_ that is not an _anchor view component_ .


**H.3.28** **non-base view** : A _view_ that is not the _base view_ . _VCL NAL units_ of a non-base view have nal_unit_type equal

to 20. Decoding of _view components_ in a non-base view may require the use of _inter-view prediction_ .


**H.3.29** **non-reference picture** : A _view component_ coded with nal_ref_idc equal to 0. A non-reference picture is not used

for _inter prediction_ in the _decoding process_ of any other _view components_ .


**H.3.30** **operation point** : An operation point is identified by a temporal_id value representing the _target temporal level_

and a set of view_id values representing the _target output views_ . One _operation point_ is associated with a _bitstream_
_subset_, which consists of the _target output views_ and all other _views_ the _target output views_ depend on, that is
derived using the _sub-bitstream_ extraction process as specified in clause H.8.5.3 with tIdTarget equal to the
temporal_id value and viewIdTargetList consisting of the set of view_id values as inputs. More than one _operation_
_point_ may be associated with the same _bitstream subset_ . When the specification states "an _operation point_ is
decoded" it refers to the decoding of a _bitstream subset_ corresponding to the _operation point_ and subsequent
output of the _target output views_ .


**H.3.31** **picture order count** : A variable that is associated with each _field view component_ and each _field_ of a _frame view_

_component_ and has a value that is non-decreasing with increasing _field_ position in _output order_ in the same _view_
relative to the first output _field_ of the previous _IDR view component_ in _decoding order_ in the same _view_ or relative
to the first output _field of the_ previous _view component_, in _decoding order_ in the same _view_, that contains a _memory_
_management control operation_ that marks all _reference pictures_ in the _view_ as "unused for reference".


**H.3.32** **prefix NAL unit** : A _NAL unit_ with nal_unit_type equal to 14 that immediately precedes in _decoding order_ a _NAL_

_unit_ with nal_unit_type equal to 1 or 5. The _NAL unit_ that immediately follows in _decoding order_ the prefix NAL
unit is referred to as the _associated NAL unit_ . The prefix NAL unit contains data associated with the _associated_
_NAL unit_, which are considered to be part of the _associated NAL unit_ .


**H.3.33** **reference picture** : A _view component_ coded with nal_ref_idc greater than 0. A reference picture contains samples

that may be used for _inter prediction_ in the _decoding process_ of subsequent _view components_ in _decoding order_ .
A reference picture may be an _inter-view reference component_, in which case the samples contained in the
reference picture may also be used for _inter-view prediction_ in the _decoding process_ of subsequent _view_





_components_ in _decoding order_ .


**H.3.34** **reference picture list** : A list of _reference pictures_ and _inter-view only reference components_ that are used for

_inter prediction_ or _inter-view prediction_ of a _P_ or _B slice._ For the _decoding process_ of a _P slice,_ there is one
reference picture list _._ For the _decoding process_ of a _B slice_, there are two reference picture lists _._


**H.3.35** **reference picture list 0** : A _reference picture list_ used for _inter prediction_ or _inter-view prediction_ of a _P_ or _B_

_slice_ . All _inter prediction_ or _inter-view prediction_ used for _P slices_ uses reference picture list 0. Reference picture
list 0 is one of two _reference picture lists_ used for _inter prediction_ or _inter-view prediction_ for a _B slice_, with the
other being _reference picture list 1_ .


**H.3.36** **reference picture list 1** : A _reference picture list_ used for _inter prediction_ or _inter-view prediction_ of a _B slice_ .

Reference picture list 1 is one of two _reference picture lists_ used for _inter prediction_ or _inter-view prediction_ for
a _B slice_, with the other being _reference picture list 0_ .


**H.3.37** **reference picture marking** : Specifies, in the bitstream, how the _decoded view components_ are marked for _inter_

_prediction_ or _inter-view prediction_ .


**H.3.38** **reference processing unit:** A functional unit that processes an _inter-view_ _prediction reference_ before the _inter-_

_view_ _prediction reference_ is used for _inter-view prediction_ in the _decoding process_ of subsequent _view components_
in _decoding order_ .


**H.3.39** **reference view index** : An index into a list of _anchor view components_ or a list of _non-anchor view components_

that are specified in the sequence parameter set MVC extension syntax structure and can be used for _inter-view_
_prediction_ as _list 0 prediction_ or _list 1 prediction_ .


**H.3.40** **right view:** The right part of a picture coded in a frame-packed manner with the side-by-side frame packing

arrangement type or the bottom part of a picture coded in a frame-packed manner with the top-bottom frame
packing arrangement type.


**H.3.41** **sub-bitstream** : A _subset_ of a _bitstream_ . A _sub-bitstream_ is also referred to as a _bitstream subset_ .


**H.3.42** **subset** : A subset contains only elements that are also contained in the set from which the subset is derived. The

subset may be identical to the set from which it is derived.


**H.3.43** **subset sequence parameter set** : A _syntax structure_ containing _syntax elements_ that apply to zero or more

_non-base views_ as determined by the content of a seq_parameter_set_id _syntax element_ found in the _picture_
_parameter set_ referred to by the pic_parameter_set_id _syntax element_ found in each _slice header_ of _I, P, and B_
_slices_ of a _non-base view component_ .


**H.3.44** **target output view** : A _view_ that is to be output. The target output views are either indicated by external means

or, when not indicated by external means, the target output view is the _base view_ .


NOTE – The output views may be requested by a receiver and may be negotiated between the receiver and the sender.


**H.3.45** **target temporal level** : The _target temporal level_ of an _operation point_ is the greatest value of temporal_id of all

_VCL NAL units_ in the _bitstream subset_ associated with the _operation point_ .


**H.3.46** **view:** A sequence of _view components_ associated with an identical value of view_id.


**H.3.47** **view component:** A _coded representation_ of a _view_ in a single _access unit_ . When profile_idc is equal to 134, a

view contains samples of two distinct spatially packed constituent frames that are packed into one frame using
one of the frame packing arrangement schemes as specified in clause D.2.26.


**H.3.48** **view order index:** An index that indicates the _decoding order_ of _view components_ in an _access unit_ .
