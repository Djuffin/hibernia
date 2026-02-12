**C.1** **Operation of coded picture buffer (CPB)**


The specifications in this clause apply independently to each set of CPB parameters that is present and to both the Type I
and Type II conformance points shown in Figure C-1.





**C.1.1** **Timing of bitstream arrival**


The HRD may be initialized at any one of the buffering period SEI messages. Prior to initialization, the CPB is empty.

NOTE – After initialization, the HRD is not initialized again by subsequent buffering period SEI messages.


Each access unit is referred to as access unit n, where the number n identifies the particular access unit. The access unit
that is associated with the buffering period SEI message that initializes the CPB is referred to as access unit 0. The value
of n is incremented by 1 for each subsequent access unit in decoding order.


The time at which the first bit of access unit n begins to enter the CPB is referred to as the initial arrival time tai( n ).


The initial arrival time of access units is derived as follows:


- If the access unit is access unit 0, tai( 0 ) = 0,


- Otherwise (the access unit is access unit n with n > 0), the following applies:


   - If cbr_flag[ SchedSelIdx ] is equal to 1, the initial arrival time for access unit n, is equal to the final arrival time
(which is derived below) of access unit n − 1, i.e.,


tai( n ) = taf( n − 1 ) (C-2)


   - Otherwise (cbr_flag[ SchedSelIdx ] is equal to 0), the initial arrival time for access unit n is derived by


tai( n ) = Max( taf( n − 1 ), tai,earliest( n ) ) (C-3)


where tai,earliest( n ) is derived as follows:


      - If access unit n is not the first access unit of a subsequent buffering period, tai,earliest( n ) is derived as


tai,earliest( n ) = tr,n( n ) − ( initial_cpb_removal_delay[ SchedSelIdx ] +
initial_cpb_removal_delay_offset[ SchedSelIdx ] ) ÷ 90000 (C-4)


with tr,n( n ) being the nominal removal time of access unit n from the CPB as specified in clause C.1.2 and
initial_cpb_removal_delay[ SchedSelIdx ] and initial_cpb_removal_delay_offset[ SchedSelIdx ] being
specified in the previous buffering period SEI message.


      - Otherwise (access unit n is the first access unit of a subsequent buffering period), tai,earliest( n ) is derived as


tai,earliest( n ) = tr,n( n ) − ( initial_cpb_removal_delay[ SchedSelIdx ] ÷ 90000 ) (C-5)


with initial_cpb_removal_delay[ SchedSelIdx ] being specified in the buffering period SEI message
associated with access unit n.


The final arrival time for access unit n is derived by


taf( n ) = tai( n ) + b( n ) ÷ BitRate[ SchedSelIdx ] (C-6)


where b( n ) is the size in bits of access unit n, counting the bits of the VCL NAL units and the filler data NAL units for
the Type I conformance point or all bits of the Type II bitstream for the Type II conformance point, where the Type I and
Type II conformance points are as shown in Figure C-1.


The values of SchedSelIdx, BitRate[ SchedSelIdx ], and CpbSize[ SchedSelIdx ] are constrained as follows:


- If the content of the active sequence parameter sets for access unit n and access unit n − 1 differ, the HSS selects a
value SchedSelIdx1 of SchedSelIdx from among the values of SchedSelIdx provided in the active sequence parameter
set for access unit n that results in a BitRate[ SchedSelIdx1 ] or CpbSize[ SchedSelIdx1 ] for access unit n. The value
of BitRate[ SchedSelIdx1 ] or CpbSize[ SchedSelIdx1 ] may differ from the value of BitRate[ SchedSelIdx0 ] or
CpbSize[ SchedSelIdx0 ] for the value SchedSelIdx0 of SchedSelIdx that was in use for access unit n − 1.


- Otherwise, the HSS continues to operate with the previous values of SchedSelIdx, BitRate[ SchedSelIdx ] and
CpbSize[ SchedSelIdx ].


When the HSS selects values of BitRate[ SchedSelIdx ] or CpbSize[ SchedSelIdx ] that differ from those of the previous
access unit, the following applies:


- the variable BitRate[ SchedSelIdx ] comes into effect at time tai( n )


- the variable CpbSize[ SchedSelIdx ] comes into effect as follows:





   - If the new value of CpbSize[ SchedSelIdx ] exceeds the old CPB size, it comes into effect at time tai( n ),


   - Otherwise, the new value of CpbSize[ SchedSelIdx ] comes into effect at the time tr( n ).


**C.1.2** **Timing of coded picture removal**


When an access unit n is the access unit with n equal to 0 (the access unit that initializes the HRD), the nominal removal
time of the access unit from the CPB is specified by


tr,n( 0 ) = initial_cpb_removal_delay[ SchedSelIdx ] ÷ 90000 (C-7)


When an access unit n is the first access unit of a buffering period that does not initialize the HRD, the nominal removal
time of the access unit from the CPB is specified by


tr,n( n ) = tr,n( nb ) + tc * cpb_removal_delay( n ) (C-8)


where tr,n( nb ) is the nominal removal time of the first access unit of the previous buffering period and
cpb_removal_delay( n ) is the value of cpb_removal_delay specified in the picture timing SEI message associated with
access unit n.


The nominal removal time tr,n(n) of an access unit n that is not the first access unit of a buffering period is given by


tr,n( n ) = tr,n( nb ) + tc * cpb_removal_delay( n ) (C-9)


where tr,n( nb ) is the nominal removal time of the first access unit of the current buffering period and
cpb_removal_delay( n ) is the value of cpb_removal_delay specified in the picture timing SEI message associated with
access unit n.


The removal time of access unit n is specified as follows:


- If low_delay_hrd_flag is equal to 0 or tr,n( n ) >= taf( n ), the removal time of access unit n is specified by


tr( n ) = tr,n( n ) (C-10)


- Otherwise (low_delay_hrd_flag is equal to 1 and tr,n( n ) < taf( n )), the removal time of access unit n is specified by


tr( n ) = tr,n( n ) + tc * Ceil( ( taf( n ) − tr,n( n ) ) ÷ tc ) (C-11)


NOTE – The latter case indicates that the size of access unit n, b( n ), is so large that it prevents removal at the nominal removal
time.


When an access unit n is the first access unit of a buffering period, nb is set equal to n at the removal time tr( n ) of the
access unit n.
