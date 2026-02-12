**B.2** **Byte stream NAL unit decoding process**


Input to this process consists of an ordered stream of bytes consisting of a sequence of byte stream NAL unit syntax
structures.


Output of this process consists of a sequence of NAL unit syntax structures.


At the beginning of the decoding process, the decoder initializes its current position in the byte stream to the beginning of
the byte stream. It then extracts and discards each leading_zero_8bits syntax element (if present), moving the current
position in the byte stream forward one byte at a time, until the current position in the byte stream is such that the next four
bytes in the bitstream form the four-byte sequence 0x00000001.


The decoder then performs the following step-wise process repeatedly to extract and decode each NAL unit syntax structure
in the byte stream until the end of the byte stream has been encountered (as determined by unspecified means) and the last
NAL unit in the byte stream has been decoded:


1. When the next four bytes in the bitstream form the four-byte sequence 0x00000001, the next byte in the byte

stream (which is a zero_byte syntax element) is extracted and discarded and the current position in the byte stream
is set equal to the position of the byte following this discarded byte.


2. The next three-byte sequence in the byte stream (which is a start_code_prefix_one_3bytes) is extracted and

discarded and the current position in the byte stream is set equal to the position of the byte following this three-byte
sequence.


3. NumBytesInNALunit is set equal to the number of bytes starting with the byte at the current position in the byte

stream up to and including the last byte that precedes the location of any of the following:

      - A subsequent byte-aligned three-byte sequence equal to 0x000000,

      - A subsequent byte-aligned three-byte sequence equal to 0x000001,

      - The end of the byte stream, as determined by unspecified means.


4. NumBytesInNALunit bytes are removed from the bitstream and the current position in the byte stream is advanced

by NumBytesInNALunit bytes. This sequence of bytes is nal_unit( NumBytesInNALunit ) and is decoded using
the NAL unit decoding process.


5. When the current position in the byte stream is not at the end of the byte stream (as determined by unspecified

means) and the next bytes in the byte stream do not start with a three-byte sequence equal to 0x000001 and the
next bytes in the byte stream do not start with a four byte sequence equal to 0x00000001, the decoder extracts and
discards each trailing_zero_8bits syntax element, moving the current position in the byte stream forward one byte
at a time, until the current position in the byte stream is such that the next bytes in the byte stream form the fourbyte sequence 0x00000001 or the end of the byte stream has been encountered (as determined by unspecified
means).
