**I.8** **MVCD decoding process**


This clause specifies the decoding process for an access unit of a coded video sequence conforming to one or more of the
profiles specified in Annex I. Specifically, this clause specifies how the decoded picture with multiple texture view
components and multiple depth view components is derived from syntax elements and global variables that are derived
from NAL units in an access unit when the decoder is decoding the operation point identified by the target temporal level
and the target output texture and depth views.


The decoding process is specified such that all decoders shall produce numerically identical results for the target output
texture and depth views. Any decoding process that produces identical results for the target output texture and depth views
to the process described here conforms to the decoding process requirements of this Recommendation | International
Standard.


Unless stated otherwise, the syntax elements and derived upper-case variables that are referred to by the decoding process
specified in this clause and all child processes invoked from the process specified in this clause are the syntax elements
and derived upper-case variables for the current access unit.


The target output texture and depth views are either specified by external means not specified in this Specification, or,
when not specified by external means, there shall be one target output texture view which is the base texture view.


NOTE – The association of VOIdx values to view_id values according to the decoding process of clause I.8 may differ from that
of the decoding process of clause H.8.


A target output view may include only a texture view, only a depth view, or both the texture view and the depth view,
which have the same view_id value.


All sub-bitstreams that can be derived using the sub-bitstream extraction process with depthPresentFlagTarget equal to 0
or 1, pIdTarget equal to any value in the range of 0 to 63, inclusive, tIdTarget equal to any value in the range of 0 to 7,
inclusive, viewIdTargetList consisting of any one or more viewIdTarget's identifying the views in the bitstream as inputs
as specified in clause I.8.5 shall result in a set of coded video sequences, with each coded video sequence conforming to
one or more of the profiles specified in Annex A, Annex H and Annex I.


Let vOIdxList be a list of integer values specifying the VOIdx values of the view components of the access unit. The
variable VOIdxMax is set equal to the maximum value of the entries in the list vOIdxList, and the variable vOIdxMin is
set to the minimum value of the entries in the list vOIdxList. When the current access unit is an anchor access unit, the
variable VOIdxMin is set to vOIdxMin.


The MVCD video decoding process specified in this clause is repeatedly invoked for each texture and depth view
component with VOIdx from vOIdxMin to VOIdxMax, inclusive, which is present in the list vOIdxList, in increasing
order of VOIdx and in decoding order of texture or depth view components as specified in clause I.7.4.1.2.5.


Outputs of the MVCD video decoding process are decoded samples of the current primary coded picture including all
decoded texture and depth view components of the target output texture and depth views.


For each texture view component and each depth view component, the specifications in clause H.8 apply, with the decoding
processes for picture order count, reference picture lists construction and decoded reference picture marking being
modified in clauses I.8.1, I.8.2, I.8.3, and I.8.4, respectively. The MVCD inter prediction and inter-view prediction process
is specified in clause I.8.4.





**I.8.1** **MVCD decoding process for picture order count**


The specifications in clause 8.2.1 apply independently for each texture view or depth view.


**I.8.2** **MVC decoding process for reference picture lists construction**


The specification of clause H.8.2 apply with substituting "view component" as either "texture view component" or "depth
view component", and "frame view component"" as either "depth frame view component" or "texture frame view
component ", and "field view component" as "texture field view component" or "depth field view component".


Additionally, an inter-view reference component or the inter-view only reference component is identified by the view_id
and a depth view component when the current slice is a part of a coded depth view component or a texture view component
if the current slice is a part of a coded texture view component.


**I.8.2.1** **Initialization process for reference picture list for inter-view prediction references**


The specifications of clause H.8.2.1 apply.


**I.8.2.2** **Modification process for reference picture lists**


The specifications of clause H.8.2.2 apply.


**I.8.2.2.1** **Modification process of reference picture lists for short-term reference pictures for inter prediction**


The specifications of clause H.8.2.2.1 apply.


**I.8.2.2.2** **Modification process of reference picture lists for long-term reference pictures for inter prediction**


The specifications of clause H.8.2.2.2 apply.


**I.8.2.2.3** **Modification process for reference picture lists for inter-view prediction references**


The specifications of clause H.8.2.2.3 apply.


**I.8.3** **MVCD decoded reference picture marking process**


The specifications of clause H.8.3 apply. Additionally, the following applies.


The process specified in this clause is invoked for a particular texture view or depth view with view order index VOIdx.
The specifications in clause H.8.3 apply with "view component" being replaced by either "texture view component" or
"depth view component", "frame view component" being replaced by either "texture frame view component" or "depth
frame view component", and "field view component" being replaced by either "texture field view component" or "depth
field view component". During the invocation of the process for a particular texture view, only texture view components
of the particular view are considered. During the invocation of the process for a particular depth view, only depth view
components of the particular view are considered. The marking of view components of other views is not changed.


NOTE – A texture view component of a picture may have a different marking status than other texture view components of the
same picture. A depth view component of a picture may have a different marking status than other depth view components of the
same picture. A texture view component of a picture may have a different marking status than a depth view component.


**I.8.4** **MVCD inter prediction and inter-view prediction process**


The specifications of clause H.8.4 apply.


**I.8.5** **Specification of bitstream subsets**


The specifications of clause H.8.5 apply.


**I.8.5.1** **Derivation process for required anchor view components**


When invoked for a depth view, the specification of clause H.8.5.1 apply with substituting "view component" with "depth
view component" and "view" with "depth view".


When invoked for a texture view, the specification of clause H.8.5.1 apply with substituting "view component" with
"texture view component" and "view" with "texture view".


**I.8.5.2** **Derivation process for required non-anchor view components**


When invoked for a depth view, the specification of clause H.8.5.2 apply with substituting "view component" with "depth
view component" and "view" with "depth view".


When invoked for a texture view, the specification of clause H.8.5.2 apply with substituting "view component" with
"texture view component" and "view" with "texture depth view".





**I.8.5.3** **Sub-bitstream extraction process**


It is requirement of bitstream conformance that any sub-bitstream that is the output of the process specified in this clause
with depthPresentFlagTarget equal to 0 or 1, pIdTarget equal to any value in the range of 0 to 63, inclusive, tIdTarget equal
to any value in the range of 0 to 7, inclusive, viewIdTargetList consisting of any one or more values of viewIdTarget
identifying the views in the bitstream, shall be conforming to this Recommendation | International Standard.

NOTE 1 – A conforming bitstream contains one or more coded slice NAL units with priority_id equal to 0 and temporal_id equal
to 0.
NOTE 2 – It is possible that not all operation points of sub-bitstreams resulting from the sub-bitstream extraction process have an
applicable level_idc or level_idc[ i ]. In this case, each coded video sequence in a sub-bitstream must still conform to one or more
of the profiles specified in Annex A, Annex H and Annex I, but may not satisfy the level constraints specified in clauses A.3, H.10.2
and I.10.2, respectively.


Inputs to this process are:


- a variable depthPresentFlagTarget (when present),


- a variable pIdTarget (when present),


- a variable tIdTarget (when present),


- a list viewIdTargetList consisting of one or more values of viewIdTarget (when present).


- a list viewIdDepthTargetList consisting of one or more value of viewIdDepthTarget (when present).


Outputs of this process are a sub-bitstream and a list of VOIdx values VOIdxList.


When depthPresentFlagTarget is not present as input to this clause, depthPresentFlagTarget is inferred to be equal to 0.


When pIdTarget is not present as input to this clause, pIdTarget is inferred to be equal to 63.


When tIdTarget is not present as input to this clause, tIdTarget is inferred to be equal to 7.


When viewIdTargetList is not present as input to this clause, there shall be one value of viewIdTarget inferred in
viewIdTargetList and the value of viewIdTarget is inferred to be equal to view_id of the base view.


When viewIdDepthTargetList is not present as input to this clause, the viewIdDepthTargetList is inferred to be identical
to viewIdTargetList. viewIdDepthTargetList shall not be present as input if depthPresentFlagTarget is equal to 0.


The sub-bitstream is derived by applying the following operations in sequential order:


1. Let VOIdxList be empty and minVOIdx be the VOIdx value of the base view.


2. For each value of viewIdTarget included in viewIdTargetList, invoke the process specified in clause I.8.5.1 for

texture views with the viewIdTarget as input.


3. If depthPresentFlagTarget is equal to 1, for each value of viewIdTarget included in viewIdDepthTargetList,

invoke the process specified in clause I.8.5.1 for depth views with the viewIdTarget as input.


4. For each value of viewIdTarget included in viewIdTargetList, invoke the process specified in clause I.8.5.2 for

texture views with the value of viewIdTarget as input.


5. If depthPresentFlagTarget is equal to 1, for each value of viewIdTarget included in viewIdDepthTargetList,

invoke the process specified in clause I.8.5.2 for depth views with the viewIdTarget as input.


6. Mark all VCL NAL units and filler data NAL units for which any of the following conditions are true as "to be

removed from the bitstream":


     - priority_id is greater than pIdTarget,


     - temporal_id is greater than tIdTarget,


     - nal_unit_type is not equal to 21 and view_id is not in the viewIdTargetList,


     - nal_unit_type is equal to 21 and view_id is not in the viewIdDepthTargetList,


     - nal_unit_type is equal to 21 and depthPresentFlagTarget is equal to 0.


7. Remove all access units for which all VCL NAL units are marked as "to be removed from the bitstream".


8. Remove all VCL NAL units and filler data NAL units that are marked as "to be removed from the bitstream".


9. When VOIdxList contains only one value of VOIdx that is equal to minVOIdx, remove the following NAL units:


     - all NAL units with nal_unit_type equal to 14 or 15,





     - all NAL units with nal_unit_type equal to 6 in which the first SEI message has payloadType in the range
of 36 to 44, inclusive, or equal to 46, or in the range of 48 to 53, inclusive.


NOTE 3 – When VOIdxList contains only one value of VOIdx equal to minVOIdx, the sub-bitstream contains only the base
view or only a temporal subset of the base view.
10. Remove all NAL units with nal_unit_type equal to 6 in which the first SEI message has payloadType equal to 0

or 1, or the first SEI message has payloadType equal to 37 (MVC scalable nesting SEI message) and
operation_point_flag in the first SEI message is equal to 1.


NOTE 4 – The buffering period SEI and picture timing SEI messages, when not nested or nested in the MVC scalable
nesting SEI message, apply for a sub-bitstream obtained with the sub-bitstream extraction process of clause H.8.5.3,
which does not process NAL units of nal_unit_type equal to 21.


11. When depthPresentFlagTarget is equal to 0, the following applies in sequential order.


     - Replace each NAL unit with nal_unit_type equal to 6 in which payloadType indicates an MVCD scalable
nesting SEI message with sei_op_texture_only_flag equal to 0 with a NAL unit containing an MVC scalable
nesting SEI message with the same values of num_view_components_op_minus1, sei_op_view_id[ i ] and
sei_op_temporal_id and the same nested SEI messages.


     - Remove all NAL units with nal_unit_type equal to 6 in which payloadType indicates an MVCD texture
scalable nesting SEI message.


     - The following applies for each active texture MVCD sequence parameter set RBSP.


        - Replace mvc_vui_parameters_extension( ) syntax structure in an active texture MVCD sequence
parameter set RBSPs with the mvc_vui_parameters_extension( ) syntax structure of the MVCD texture
sub-bitstream VUI parameters extension, if both mvc_vui_parameters_extension( ) syntax structures
apply to the same views.


        - Otherwise, remove mvc_vui_parameters_extension( ) syntax structure in an active texture MVCD
sequence parameter set RBSP.


     - Remove all NAL units with nal_unit_type equal to 6 in which the first SEI message has payloadType in the
range of 48 to 53, inclusive.


12. Let maxTId be the maximum temporal_id of all the remaining VCL NAL units. Remove all NAL units with

nal_unit_type equal to 6 that only contain SEI messages that are part of an MVC scalable nesting SEI message or
MVCD scalable nesting SEI message with any of the following properties:


     - operation_point_flag is equal to 0 and all_view_components_in_au_flag is equal to 0 and none of
sei_view_id[ i ] for all i in the range of 0 to num_view_components_minus1, inclusive, corresponds to a
VOIdx value included in VOIdxList,


     - operation_point_flag is equal to 1 and either sei_op_temporal_id is greater than maxTId or the list of
sei_op_view_id[ i ] for all i in the range of 0 to num_view_components_op_minus1, inclusive, is not a subset
of viewIdTargetList (i.e., it is not true that sei_op_view_id[ i ] for any i in the range of 0 to
num_view_components_op_minus1, inclusive, is equal to a value in viewIdTargetList).


13. Remove each view scalability information SEI message and each operation point not present SEI message, when

present.


14. When VOIdxList does not contain a value of VOIdx equal to minVOIdx, the view with VOIdx equal to the

minimum VOIdx value included in VOIdxList is converted to the base view of the extracted sub-bitstream.


NOTE 5 – When VOIdxList does not contain a value of VOIdx equal to minVOIdx, the resulting sub-bitstream
according to the operation steps 1-9 above does not contain a base view that conforms to one or more profiles specified
in Annex A. In this case, by this operation step, the remaining view with the new minimum VOIdx value is converted
to be the new base view that conforms to one or more profiles specified in Annex A and Annex H.


**I.8.5.4** **Specification of the base view bitstream**


A bitstream that conforms to one or more profiles as specified in Annex I shall contain a base view bitstream that conforms
to one or more of the profiles specified in Annex A. This base view bitstream is derived by invoking the sub-bitstream
extraction process as specified in clause I.8.5.3 with no input and the base view bitstream being the output.

NOTE – Although all multiview bitstreams that conform to one or more of the profiles specified in this annex contain a base view
bitstream that conforms to one or more of the profiles specified in Annex A, the complete multiview bitstream (prior to operation of
the base view extraction process specified in this clause) may not conform to any profile specified in Annex A.





**I.8.5.5** **Specification of the stereoscopic texture bitstream**


A bitstream that conforms to a profile as specified in Annex I shall contain at least one sub-bitstream that conforms to one
or more of the profiles specified in Annex H with number of views equal to 2. This stereoscopic texture bitstream is derived
by invoking the sub-bitstream extraction process as specified in clause I.8.5.3 with depthPresentFlagTarget equal to 0 and
viewIdTargetList containing the view_id values of the base view and a non-base view, the texture of which does not depend
on any other non-base view for decoding.
