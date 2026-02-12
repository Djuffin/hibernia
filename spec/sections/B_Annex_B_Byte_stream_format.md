**Annex B**


**Byte stream format**


(This annex forms an integral part of this Recommendation | International Standard.)


This annex specifies syntax and semantics of a byte stream format specified for use by applications that deliver some or
all of the NAL unit stream as an ordered stream of bytes or bits within which the locations of NAL unit boundaries need
to be identifiable from patterns in the data, such as Rec. ITU-T H.222.0 | ISO/IEC 13818-1 systems or Rec. ITU-T H.320
systems. For bit-oriented delivery, the bit order for the byte stream format is specified to start with the MSB of the first
byte, proceed to the LSB of the first byte, followed by the MSB of the second byte, etc.


The byte stream format consists of a sequence of byte stream NAL unit syntax structures. Each byte stream NAL unit
syntax structure contains one start code prefix followed by one nal_unit( NumBytesInNALunit ) syntax structure. It may
(and under some circumstances, it shall) also contain an additional zero_byte syntax element. It may also contain one or
more additional trailing_zero_8bits syntax elements. When it is the first byte stream NAL unit in the bitstream, it may also
contain one or more additional leading_zero_8bits syntax elements.
