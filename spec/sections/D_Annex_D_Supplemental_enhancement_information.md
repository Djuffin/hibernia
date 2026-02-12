**Annex D**


**Supplemental enhancement information**


(This annex forms an integral part of this Recommendation | International Standard.)


This annex specifies syntax and semantics for SEI message payloads.


SEI messages assist in processes related to decoding, display or other purposes. However, SEI messages are not required
for constructing the luma or chroma samples by the decoding process. Conforming decoders are not required to process
this information for output order conformance to this Recommendation | International Standard (see Annex C for the
specification of conformance). Some SEI message information is required to check bitstream conformance and for output
timing decoder conformance.


In Annex D, specification for presence of SEI messages are also satisfied when those messages (or some subset of them)
are conveyed to decoders (or to the HRD) by other means not specified by this Recommendation | International Standard.
When present in the bitstream, SEI messages shall obey the syntax and semantics specified in clauses 7.3.2.3 and 7.4.2.3
and this annex. When the content of an SEI message is conveyed for the application by some means other than presence
within the bitstream, the representation of the content of the SEI message is not required to use the same syntax specified
in this annex. For the purpose of counting bits, only the appropriate bits that are actually present in the bitstream are
counted.
