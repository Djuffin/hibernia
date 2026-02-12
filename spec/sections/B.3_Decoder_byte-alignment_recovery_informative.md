**B.3** **Decoder byte-alignment recovery (informative)**


This clause does not form an integral part of this Recommendation | International Standard.


Many applications provide data to a decoder in a manner that is inherently byte aligned, and thus have no need for the bitoriented byte alignment detection procedure described in this clause.


A decoder is said to have byte-alignment with a bitstream when the decoder is able to determine whether or not the positions
of data in the bitstream are byte-aligned. When a decoder does not have byte alignment with the encoder's byte stream, the
decoder may examine the incoming bitstream for the binary pattern '00000000 00000000 00000000 00000001' (31
consecutive bits equal to 0 followed by a bit equal to 1). The bit immediately following this pattern is the first bit of an
aligned byte following a start code prefix. Upon detecting this pattern, the decoder will be byte aligned with the encoder
and positioned at the start of a NAL unit in the byte stream.


Once byte aligned with the encoder, the decoder can examine the incoming byte stream for subsequent three-byte sequences
0x000001 and 0x000003.


When the three-byte sequence 0x000001 is detected, this is a start code prefix.


When the three-byte sequence 0x000003 is detected, the third byte (0x03) is an emulation_prevention_three_byte to be
discarded as specified in clause 7.4.1.


When an error in the bitstream syntax is detected (e.g., a non-zero value of the forbidden_zero_bit or one of the three-byte
or four-byte sequences that are prohibited in clause 7.4.1), the decoder may consider the detected condition as an indication
that byte alignment may have been lost and may discard all bitstream data until the detection of byte alignment at a later
position in the bitstream as described in this clause.
