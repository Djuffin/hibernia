**J.11** **Byte stream format**


The specifications in Annex B apply.


**J.12** **3D-AVC hypothetical reference decoder**


The specifications in Annex C apply with substituting 3D-AVC sequence parameter set for MVC sequence parameter set.


**J.13** **3D-AVC SEI messages**


The specifications in Annex D and clause I.13 together with the extensions and modifications specified in this clause apply.


**J.13.1** **SEI message syntax**


**J.13.1.1** **Constrained depth parameter set identifier SEI message syntax**

|constrained_depth_parameter_set_identifier( payloadSize ) {|C|Descriptor|
|---|---|---|
|<br>**max_dps_id**|5|ue(v)|
|<br>**max_dps_id_diff**|5|ue(v)|
|}|||



**J.13.2** **SEI message semantics**


**J.13.2.1** **Constrained depth parameter set identifier SEI message semantics**


When present, this message shall be associated with an IDR access unit. The semantics of the message are valid for the
current coded video sequence. A constrained depth parameter set identifier SEI message indicates that
depth_parameter_set_id and dps_id values present in the coded video sequence are constrained as specified below.


NOTE 1 – When a constrained depth parameter set identifier SEI message is present, decoders are able to conclude losses of depth
parameter set NAL units.


**max_dps_id** plus 1 specifies the maximum allowed depth_range_parameter_set_id value.


**max_dps_id_diff** specifies the value range of depth_range_parameter_set_id values marked as "used". max_dps_id_diff

- 2 shall be less than max_dps_id.


For each coded slice, the following applies:


- For the first coded slice of an IDR access unit, MaxUsedDpsId is set equal to "no value", UsedDpsIdSet is an empty
set of values, and all depth range parameter set RBSPs included in the bitstream or made available to the decoding
process through external means prior to the access unit containing the IDR picture are marked unavailable.


- When MaxUsedDpsId is not equal to "no value", the value of dps_id of the slice header is constrained and the variable
updateMaxUsedDpsIdFlag is set as follows:


   - If dps_id is equal to 0, the variable updateMaxUsedDpsIdFlag is set equal to 0.


   - Otherwise, the variable zeroBasedDpsId is equal to dps_id − 1 and dps_id is constrained so that zeroBasedDpsId

fulfils the following:


      - If MaxUsedDpsId >= max_dps_id_diff and MaxUsedDpsId <= max_dps_id − max_dps_id_diff,

zeroBasedDpsId is in the range of MaxUsedDpsId       - max_dps_id_diff to
MaxUsedDpsId + max_dps_id_diff, inclusive. The variable updateMaxUsedDpsIdFlag is set equal to
( zeroBasedDpsId > MaxUsedDpsId ).


      - Otherwise, if MaxUsedDpsId < max_dps_id_diff, zeroBasedDpsId is either in the range of 0 to

MaxUsedDpsId + max_dps_id_diff, inclusive, or in the range of max_dps_id − ( max_dps_id_diff −
MaxUsedDpsId − 1 ) to max_dps_id, inclusive. The updateMaxUsedDpsIdFlag is set equal to
( zeroBasedDpsId > MaxUsedDpsId && zeroBasedDpsId < max_dps_id − ( max_dps_id_diff −
MaxUsedDpsId − 1 ) ).


      - Otherwise (MaxUsedDpsId > max_dps_id − max_dps_id_diff), zeroBasedDpsId is either in the range of 0

to max_dps_id_diff − ( max_dps_id − MaxUsedDpsId ) − 1, inclusive, or in the range of MaxUsedDpsId −
max_dps_id_diff to max_dps_id, inclusive. The updateMaxUsedDpsIdFlag is set equal to





( zeroBasedDpsId > MaxUsedDpsId | | zeroBasedDpsId < max_dps_id_diff − ( max_dps_id −
MaxUsedDpsId ) ).


- When updateMaxUsedDpsIdFlag is equal to 1, depth range parameter set RBSPs are marked as unavailable as
follows:


prevMinUsedDpsId = MaxUsedDpsId − max_dps_id_diff
if( prevMinUsedDpsId < 0 )
prevMinUsedDpsId += max_dps_id
minUsedDpsId = dps_id − 1 − max_dps_id_diff
if( minUsedDpsId < 0 )
minUsedDpsId += max_dps_id (J-86)
i = prevMinUsedDpsId
do {
Mark depth range parameter set RBSP with depth_range_parameter_set_id equal to i + 1,
if present, as unavailable.
i = ( i + 1 ) % ( max_dps_id + 1 )
} while( i != minUsedDpsId )


- When updateMaxUsedDpsIdFlag is equal to 1 or MaxUsedDpsId is equal to "no value", the following applies:


   - MaxUsedDpsId is set equal to dps_id − 1.


   - If MaxUsedDpsId is greater than or equal to max_dps_id_diff, UsedDpsIdSet is set to the values in the range of

MaxUsedDpsId − max_dps_id_diff to MaxUsedDpsId, inclusive.


   - Otherwise (MaxUsedDpsId is smaller than max_dps_id_diff), UsedDpsIdSet is set to the values in the range of

0 to MaxUsedDpsId, inclusive, and in the range of max_dps_id − ( max_dps_id_diff − MaxUsedDpsId ) to
max_dps_id, inclusive.


Any depth parameter set RBSP included in the bitstream or made available to the decoding process through external means
and having depth_parameter_set_id equal to any value included in UsedDpsIdSet + 1 has the same content as the previous
depth parameter set RBSP included in the bitstream or made available to the decoding process through external means
having the same depth_range_parameter_id value.


NOTE 2 – If a slice header includes a dps_id value marked as unavailable, a decoder should infer an unintentional loss of a depth
parameter set with depth_parameter_set_id value equal to the dps_id value of the slice header.
