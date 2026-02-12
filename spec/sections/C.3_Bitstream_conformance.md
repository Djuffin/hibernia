**C.3** **Bitstream conformance**


A bitstream of coded data conforming to this Recommendation | International Standard fulfils the following requirements.


The bitstream is constructed according to the syntax, semantics, and constraints specified in this
Recommendation | International Standard outside of this annex.


The bitstream is tested by the HRD as specified below:


For Type I bitstreams, the number of tests carried out is equal to cpb_cnt_minus1 + 1 where cpb_cnt_minus1 is either the
syntax element of hrd_parameters( ) following the vcl_hrd_parameters_present_flag or is determined by the application
by other means not specified in this Recommendation | International Standard. One test is carried out for each bit rate and
CPB size combination specified by hrd_parameters( ) following the vcl_hrd_parameters_present_flag. Each of these tests
is conducted at the Type I conformance point shown in Figure C-1.


For Type II bitstreams there are two sets of tests. The number of tests of the first set is equal to cpb_cnt_minus1 + 1 where
cpb_cnt_minus1 is either the syntax element of hrd_parameters( ) following the vcl_hrd_parameters_present_flag or is
determined by the application by other means not specified in this Recommendation | International Standard. One test is
carried out for each bit rate and CPB size combination. Each of these tests is conducted at the Type I conformance point
shown in Figure C-1. For these tests, only VCL and filler data NAL units are counted for the input bit rate and CPB storage.


The number of tests of the second set, for Type II bitstreams, is equal to cpb_cnt_minus1 + 1 where cpb_cnt_minus1 is
either the syntax element of hrd_parameters( ) following the nal_hrd_parameters_present_flag or is determined by the
application by other means not specified in this Recommendation | International Standard. One test is carried out for each
bit rate and CPB size combination specified by hrd_parameters( ) following the nal_hrd_parameters_present_flag. Each of
these tests is conducted at the Type II conformance point shown in Figure C-1. For these tests, all NAL units (of a Type II
NAL unit stream) or all bytes (of a byte stream) are counted for the input bit rate and CPB storage.

NOTE 1 – NAL HRD parameters established by a value of SchedSelIdx for the Type II conformance point shown in Figure C-1 are
sufficient to also establish VCL HRD conformance for the Type I conformance point shown in Figure C-1 for the same values of
initial_cpb_removal_delay[ SchedSelIdx ], BitRate[ SchedSelIdx ], and CpbSize[ SchedSelIdx ] for the VBR case
(cbr_flag[ SchedSelIdx ] equal to 0). This is because the data flow into the Type I conformance point is a subset of the data flow
into the Type II conformance point and because, for the VBR case, the CPB is allowed to become empty and stay empty until the
time a next picture is scheduled to begin to arrive. For example, when a coded video sequence conforming to one or more of the
profiles specified in Annex A is decoded by applying the decoding process specified in clauses 2 to 9, when NAL HRD parameters
are provided for the Type II conformance point that not only fall within the bounds set for NAL HRD parameters for profile
conformance in item j) of clause A.3.1 or item h) of clause A.3.3 (depending on the profile in use) but also fall within the bounds
set for VCL HRD parameters for profile conformance in item i) of clause A.3.1 or item g) of clause A.3.3 (depending on the profile
in use), conformance of the VCL HRD for the Type I conformance point is also assured to fall within the bounds of item i) of
clause A.3.1.


For conforming bitstreams, all of the following conditions shall be fulfilled for each of the tests:


1. For each access unit n, with n>0, associated with a buffering period SEI message, with  tg,90( n ) specified by


 tg,90( n ) = 90000 * ( tr,n( n ) − taf( n − 1 ) ) (C-14)


the value of initial_cpb_removal_delay[ SchedSelIdx ] shall be constrained as follows:

    - If cbr_flag[ SchedSelIdx ] is equal to 0,





initial_cpb_removal_delay[ SchedSelIdx ] <= Ceil(  tg,90( n ) ) (C-15)


    - Otherwise (cbr_flag[ SchedSelIdx ] is equal to 1),


Floor(  tg,90( n ) ) <= initial_cpb_removal_delay[ SchedSelIdx ] <= Ceil(  tg,90( n ) ) (C-16)


NOTE 2 – The exact number of bits in the CPB at the removal time of each picture may depend on which buffering
period SEI message is selected to initialize the HRD. Encoders must take this into account to ensure that all specified
constraints must be obeyed regardless of which buffering period SEI message is selected to initialize the HRD, as the
HRD may be initialized at any one of the buffering period SEI messages.


2. A CPB overflow is specified as the condition in which the total number of bits in the CPB is larger than the CPB

size. The CPB shall never overflow.


3. A CPB underflow is specified as the condition in which tr,n( n ) is less than taf( n ). When low_delay_hrd_flag is

equal to 0, the CPB shall never underflow.


4. The nominal removal times of pictures from the CPB (starting from the second picture in decoding order), shall

satisfy the constraints on tr,n( n ) and tr( n ) expressed in clauses A.3.1 through A.3.3 for the profile and level
specified in the bitstream when a coded video sequence conforming to one or more of the profiles specified in
Annex A is decoded by applying the decoding process specified in clauses 2 to 9, and they shall satisfy the
constraints on tr,n( n ) and tr( n ) expressed in clauses G.10.2.1 and G.10.2.2 for profile and level specified in the
bitstream when a coded video sequence conforming to one or more of the profiles specified in Annex G is decoded
by applying the decoding process specified in Annex G, and they shall satisfy the constraints on tr,n( n ) and tr( n )
expressed in clause H.10.2 for the profile and level specified in the bitstream when a coded video sequence
conforming to one or more of the profiles specified in Annex H is decoded by applying the decoding process
specified in Annex H, and they shall satisfy the constraints on tr,n( n ) and tr( n ) expressed in clause I.10.2 for the
profile and level specified in the bitstream when a coded video sequence conforming to one or more of the profiles
specified in Annex I is decoded by applying the decoding process specified in Annex I.


5. Immediately after any decoded picture is added to the DPB, the fullness of the DPB shall be less than or equal to

the DPB size as constrained by Annexes A, D, and E and clauses G.10, G.13, G.14, H.10, H.13, H.14, and I.14 for
the profile and level specified in the bitstream.


6. All reference pictures shall be present in the DPB when needed for prediction. Each picture shall be present in the

DPB at its DPB output time unless it is not stored in the DPB at all, or is removed from the DPB before its output
time by one of the processes specified in clause C.2.


7. The value of  to,dpb( n ) as given by Equation C- **13**, which is the difference between the output time of a picture and

that of the first picture following it in output order and having OutputFlag equal to 1, shall satisfy the constraint
expressed in clause A.3.1 for the profile and level specified in the bitstream when a coded video sequence
conforming to one or more of the profiles specified in Annex A is decoded by applying the decoding process
specified in clauses 2 to 9, and it shall satisfy the constraint expressed in clause G.10.2.1 for profile and level
specified in the bitstream when a coded video sequence conforming to one or more of the profiles specified in
Annex G is decoded by applying the decoding process specified in Annex G, and it shall satisfy the constraints
expressed in clause H.10.2 for the profile and level specified in the bitstream when a coded video sequence
conforming to one or more of the profiles specified in Annex H is decoded by applying the decoding process
specified in Annex H, and it shall satisfy the constraints expressed in clause I.10.2 for the profile and level specified
in the bitstream when a coded video sequence conforming to one or more of the profiles specified in Annex I is
decoded by applying the decoding process specified in Annex I.
