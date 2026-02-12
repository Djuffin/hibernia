**B.1** **Byte stream NAL unit syntax and semantics**


**B.1.1** **Byte stream NAL unit syntax**

|byte_stream_nal_unit( NumBytesInNALunit ) {|C|Descriptor|
|---|---|---|
|<br>while( next_bits( 24 ) != 0x000001 &&<br> <br> <br> <br> <br>next_bits( 32 ) != 0x00000001 )|||
|<br> <br>**leading_zero_8bits** /* equal to 0x00 */||f(8)|
|<br>if( next_bits( 24 ) != 0x000001 )|||
|<br> <br>**zero_byte** /* equal to 0x00 */||f(8)|
|<br>**start_code_prefix_one_3bytes** /* equal to 0x000001 */||f(24)|
|<br>nal_unit( NumBytesInNALunit )|||
|<br>while( more_data_in_byte_stream( ) &&<br> <br> <br> <br> <br>next_bits( 24 ) != 0x000001 &&<br> <br> <br> <br> <br>next_bits( 32 ) != 0x00000001 )|||
|<br> <br>**trailing_zero_8bits** /* equal to 0x00 */||f(8)|
|}|||



**B.1.2** **Byte stream NAL unit semantics**


The order of byte stream NAL units in the byte stream shall follow the decoding order of the NAL units contained in the
byte stream NAL units (see clause 7.4.1.2). The content of each byte stream NAL unit is associated with the same access
unit as the NAL unit contained in the byte stream NAL unit (see clause 7.4.1.2.3).


**leading_zero_8bits** is a byte equal to 0x00.

NOTE – The leading_zero_8bits syntax element can only be present in the first byte stream NAL unit of the bitstream, because (as
shown in the syntax diagram of clause B.1.1) any bytes equal to 0x00 that follow a NAL unit syntax structure and precede the fourbyte sequence 0x00000001 (which is to be interpreted as a zero_byte followed by a start_code_prefix_one_3bytes) will be
considered to be trailing_zero_8bits syntax elements that are part of the preceding byte stream NAL unit.


**zero_byte** is a single byte equal to 0x00.


When any of the following conditions are true, the zero_byte syntax element shall be present:

- the nal_unit_type within the nal_unit( ) is equal to 7 (sequence parameter set) or 8 (picture parameter set),

- the byte stream NAL unit syntax structure contains the first NAL unit of an access unit in decoding order, as specified
in clause 7.4.1.2.3.

**start_code_prefix_one_3bytes** is a fixed-value sequence of 3 bytes equal to 0x000001. This syntax element is called a
start code prefix.


**trailing_zero_8bits** is a byte equal to 0x00.
