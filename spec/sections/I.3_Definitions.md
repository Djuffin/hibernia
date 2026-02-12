**I.3** **Definitions**


For the purpose of this annex, the following definitions apply in addition to the definitions in clause H.3. These definitions
are either not present in clause H.3 or replace definitions in clause H.3.


**I.3.1** **depth field view** : A _depth view component_ of a _field_ .


**I.3.2** **depth frame view** : A _depth view component_ of a _frame_ .


**I.3.3** **depth view** : A sequence of _depth view components_ associated with an identical value of view_id.


**I.3.4** **depth view component** : A _coded representation_ of the depth of a view in a single _access unit_ .


**I.3.5** **inter-view only reference component** : A _view component_, _texture view component, or depth view component_
coded with nal_ref_idc equal to 0 and inter_view_flag equal to 1. An _inter-view only reference component_
contains samples that may be used for _inter-view prediction_ in the _decoding process_ of subsequent _view_
_components_ in _decoding order_, but are not used for _inter prediction_ by any _view components_ . _Inter-view only_
_reference components_ are _non-reference pictures_ .


**I.3.6** **inter-view reference component** : A _view component_, _texture view component, or depth view component_ coded
with nal_ref_idc greater than 0 and inter_view_flag equal to 1. An _inter-view reference component_ contains
samples that may be used for _inter prediction_ of subsequent _pictures_ in _decoding order_ and _inter-view prediction_
of subsequent _view components_ in _decoding order_ . _Inter-view reference components_ are _reference pictures_ .


**I.3.7** **MVCD operation point** : An operation point for which each target output view includes a texture view or a depth
view or both a texture view and a depth view.


**I.3.8** **MVCD sequence parameter set** : A collective term for _sequence parameter set_ or _subset sequence parameter_
_set_ .


**I.3.9** **MVCD sequence parameter set RBSP** : A collective term for _sequence parameter set RBSP_ or _subset sequence_
_parameter set RBSP_ .


**I.3.10** **reference picture** : A _view component_, _texture view component, or depth view component_ coded with nal_ref_idc
greater than 0. A _reference picture_ contains samples that may be used for _inter prediction_ in the _decoding process_
of subsequent _view components_ in _decoding order_ . A _reference picture_ may be an _inter-view reference component_,
in which case the samples contained in the _reference picture_ may also be used for _inter-view prediction_ in the
_decoding process_ of subsequent _view components_ in _decoding order_ .


**I.3.11** **stereoscopic texture bitstream** : A _bitstream_ containing two _texture_ _views_ and conforming to one of the _profiles_
specified in Annex H.


**I.3.12** **texture field view component** : A _texture view component_ of a _field_ .


**I.3.13** **texture frame view component** : A _texture view component_ of a _frame_ .


**I.3.14** **texture view** : A sequence of _texture view components_ associated with an identical value of view_id.


**I.3.15** **texture view component** : A _coded representation_ of the texture of a view in a single _access unit_ .


**I.3.16** **view** : A _texture view_ and a _depth view_ with the same value of view_id, unless explicitly limited to either _texture_





_view_ or _depth view_ .


**I.3.17** **view component** : A _coded representation_ of a _view_ in a single _access unit_ . A _view component_ may consist of a
_texture view component_ and a _depth view component_ .


**I.3.18** **view component pair** : A _texture view component_ and a _depth view component_ of the same _view_ within the same
_access unit_ .
