**E.2** **VUI semantics**


**E.2.1** **VUI parameters semantics**


**aspect_ratio_info_present_flag** equal to 1 specifies that aspect_ratio_idc is present. aspect_ratio_info_present_flag equal
to 0 specifies that aspect_ratio_idc is not present.


**aspect_ratio_idc** specifies the value of the sample aspect ratio of the luma samples. Table E-1 shows the meaning of the
code. When aspect_ratio_idc indicates Extended_SAR, the sample aspect ratio is represented by sar_width : sar_height.
When the aspect_ratio_idc syntax element is not present, aspect_ratio_idc value shall be inferred to be equal to 0.


**Table E-1 – Meaning of sample aspect ratio indicator**







|aspect_ratio_idc|Sample aspect<br>ratio|(informative)<br>Examples of use|
|---|---|---|
|0|Unspecified<br>||
|1|1:1<br>("square")|7680x4320 16:9 frame without horizontal overscan<br>3840x2160 16:9 frame without horizontal overscan<br>1280x720 16:9 frame without horizontal overscan<br>1920x1080 16:9 frame without horizontal overscan (cropped from<br>1920x1088)<br>640x480 4:3 frame without horizontal overscan|
|2|12:11|720x576 4:3 frame with horizontal overscan<br>352x288 4:3 frame without horizontal overscan|
|3|10:11|720x480 4:3 frame with horizontal overscan<br>352x240 4:3 frame without horizontal overscan|
|4|16:11|720x576 16:9 frame with horizontal overscan<br>528x576 4:3 frame without horizontal overscan|
|5|40:33|720x480 16:9 frame with horizontal overscan<br>528x480 4:3 frame without horizontal overscan|
|6|24:11|352x576 4:3 frame without horizontal overscan<br>480x576 16:9 frame with horizontal overscan|
|7|20:11|352x480 4:3 frame without horizontal overscan<br>480x480 16:9 frame with horizontal overscan|
|8|32:11|352x576 16:9 frame without horizontal overscan|
|9|80:33|352x480 16:9 frame without horizontal overscan|
|10|18:11|480x576 4:3 frame with horizontal overscan|
|11|15:11|480x480 4:3 frame with horizontal overscan|
|12|64:33|528x576 16:9 frame without horizontal overscan|
|13|160:99|528x480 16:9 frame without horizontal overscan|
|14|4:3|1440x1080 16:9 frame without horizontal overscan|
|15|3:2|1280x1080 16:9 frame without horizontal overscan|
|16|2:1|960x1080 16:9 frame without horizontal overscan|
|17..254|Reserved||
|255|Extended_SAR||


NOTE 1 – For the examples in Table E-1, the term "without horizontal overscan" refers to display processes in which the display
area matches the area of the cropped decoded pictures and the term "with horizontal overscan" refers to display processes in which
some parts near the left and/or right border of the cropped decoded pictures are not visible in the display area. As an example, the
entry "720x576 4:3 frame with horizontal overscan" for aspect_ratio_idc equal to 2 refers to having an area of 704x576 luma samples
(which has an aspect ratio of 4:3) of the cropped decoded frame (720x576 luma samples) that is visible in the display area.



**sar_width** indicates the horizontal size of the sample aspect ratio (in arbitrary units).


**sar_height** indicates the vertical size of the sample aspect ratio (in the same arbitrary units as sar_width).


sar_width and sar_height shall be relatively prime or equal to 0. When aspect_ratio_idc is equal to 0 or sar_width is equal
to 0 or sar_height is equal to 0, the sample aspect ratio shall be considered unspecified by this Recommendation |
International Standard.


**overscan_info_present_flag** equal to 1 specifies that the overscan_appropriate_flag is present. When
overscan_info_present_flag is equal to 0 or is not present, the preferred display method for the video signal is unspecified.





**overscan_appropriate_flag** equal to 1 indicates that the cropped decoded pictures output are suitable for display using
overscan. overscan_appropriate_flag equal to 0 indicates that the cropped decoded pictures output contain visually
important information in the entire region out to the edges of the cropping rectangle of the picture, such that the cropped
decoded pictures output should not be displayed using overscan. Instead, they should be displayed using either an exact
match between the display area and the cropping rectangle, or using underscan. As used in this paragraph, the term
"overscan" refers to display processes in which some parts near the borders of the cropped decoded pictures are not visible
in the display area. The term "underscan" describes display processes in which the entire cropped decoded pictures are
visible in the display area, but they do not cover the entire display area. For display processes that neither use overscan nor
underscan, the display area exactly matches the area of the cropped decoded pictures.

NOTE 2 – For example, overscan_appropriate_flag equal to 1 might be used for entertainment television programming, or for a live
view of people in a videoconference, and overscan_appropriate_flag equal to 0 might be used for computer screen capture or security
camera content.


**video_signal_type_present_flag** equal to 1 specifies that video_format, video_full_range_flag and
colour_description_present_flag are present. video_signal_type_present_flag equal to 0, specify that video_format,
video_full_range_flag and colour_description_present_flag are not present.


**video_format** indicates the representation of the pictures as specified in Table E-2, before being coded in accordance with
this Recommendation | International Standard. When the video_format syntax element is not present, video_format value
shall be inferred to be equal to 5.


**Table E-2 – Meaning of video_format**

|video_format|Meaning|
|---|---|
|0|Component|
|1|PAL|
|2|NTSC|
|3|SECAM|
|4|MAC|
|5|Unspecified video format|
|6|Reserved|
|7|Reserved|



**video_full_range_flag** indicates the black level and range of the luma and chroma signals as derived from E′Y, E′PB, and
E′PR or E′R, E′G, and E′B real-valued component signals.


When the video_full_range_flag syntax element is not present, the value of video_full_range_flag shall be inferred to be
equal to 0.


**colour_description_present_flag** equal to 1 specifies that colour_primaries, transfer_characteristics and
matrix_coefficients are present. colour_description_present_flag equal to 0 specifies that colour_primaries,
transfer_characteristics and matrix_coefficients are not present.


**colour_primaries** indicates the chromaticity coordinates of the source primaries as specified in Table E-3 in terms of the
CIE 1931 definition of x and y as specified by ISO 11664-1.


When the colour_primaries syntax element is not present, the value of colour_primaries shall be inferred to be equal to 2
(the chromaticity is unspecified or is determined by the application).





**Table E-3 – Colour primaries interpretation using colour_primaries syntax element**









|Value|Primaries|Informative remark|
|---|---|---|
|0|Reserved|For future use by ITU-T | ISO/IEC|
|1|primary<br>x <br>y <br>green<br>0.300<br>0.600<br>blue<br>0.150<br>0.060<br>red<br>0.640<br>0.330<br>white D65<br>0.3127<br>0.3290|Rec. ITU-R BT.709-6<br>Rec. ITU-R BT.1361-0 conventional colour gamut<br>system and extended colour gamut system (historical)<br>IEC 61966-2-1 sRGB or sYCC<br>IEC 61966-2-4<br>Society of Motion Picture and Television Engineers RP<br>177 (1993) Annex B|
|2|Unspecified|Image characteristics are unknown or are determined by<br>the application.|
|3|Reserved|For future use by ITU-T | ISO/IEC|
|4|primary<br>x <br>y <br>green<br>0.21<br>0.71<br>blue<br>0.14<br>0.08<br>red<br>0.67<br>0.33<br>white C<br>0.310<br>0.316|Rec. ITU-R BT.470-7 System M (historical)<br>United States National Television System Committee<br>1953 Recommendation for transmission standards for<br>colour television<br>United States Federal Communications Commission<br>Title 47 Code of Federal Regulations (2016) 73.682 (a)<br>(20)|
|5|primary<br>x <br>y <br>green<br>0.29<br>0.60<br>blue<br>0.15<br>0.06<br>red<br>0.64<br>0.33<br>white D65<br>0.3127<br>0.3290|Rec. ITU-R BT.470-7 System B, G (historical)<br>Rec. ITU-R BT.601-7 625<br>Rec. ITU-R BT.1358-0 625 (historical)<br>Rec. ITU-R BT.1700-0 625 PAL and 625 SECAM|
|6|primary<br>x <br>y <br>green<br>0.310<br>0.595<br>blue<br>0.155<br>0.070<br>red<br>0.630<br>0.340<br>white D65<br>0.3127<br>0.3290|Rec. ITU-R BT.601-7 525<br>Rec. ITU-R BT.1358-1 525 or 625 (historical)<br>Rec. ITU-R BT.1700-0 NTSC<br>Society of Motion Picture and Television Engineers<br>170M (2004)<br>(functionally the same as the value 7)|
|7|primary<br>x <br>y <br>green<br>0.310<br>0.595<br>blue<br>0.155<br>0.070<br>red<br>0.630<br>0.340<br>white D65<br>0.3127<br>0.3290|Society of Motion Picture and Television Engineers<br>240M (1999, historical)<br>(functionally the same as the value 6)|
|8|primary<br>x <br>y <br>green<br>0.243<br>0.692 (Wratten 58)<br>blue<br>0.145<br>0.049 (Wratten 47)<br>red<br>0.681<br>0.319 (Wratten 25)<br>white C<br>0.310<br>0.316|Generic film (colour filters using Illuminant C)|
|9|primary<br>x <br>y <br>green<br>0.170<br>0.797<br>blue<br>0.131<br>0.046<br>red<br>0.708<br>0.292<br>white D65<br>0.3127<br>0.3290|Rec. ITU-R BT.2020-2<br>Rec. ITU-R BT.2100-2|
|10|primary<br>x <br>y <br>Y <br>0.0<br>1.0<br>Z <br>0.0<br>0.0<br>X <br>1.0<br>0.0<br>centre white<br>1 ÷ 3<br>1 ÷ 3|Society of Motion Picture and Television Engineers ST<br>428-1 (2006)<br>(CIE 1931 XYZ as in ISO 11664-1)|
|11|primary<br>x <br>y <br>green<br>0.265<br>0.690<br>blue<br>0.150<br>0.060<br>red<br>0.680<br>0.320<br>white<br>0.314<br>0.351|Society of Motion Picture and Television Engineers RP<br>431-2 (2011)<br>Society of Motion Picture and Television Engineers RP<br>ST 2113 (2018) "P3DCI"|







|12|primary x y<br>green 0.265 0.690<br>blue 0.150 0.060<br>red 0.680 0.320<br>white D65 0.3127 0.3290|Society of Motion Picture and Television Engineers EG<br>432-1 (2010)<br>Society of Motion Picture and Television Engineers ST<br>2113 (2018) "P3D65"|
|---|---|---|
|13..21|Reserved|For future use by ITU-T | ISO/IEC|
|22|primary<br>x <br>y <br>green<br>0.295<br>0.605<br>blue<br>0.155<br>0.077<br>red<br>0.630<br>0.340<br>white D65<br>0.3127<br>0.3290|EBU Tech. 3213-E (1975)|
|23..255|Reserved|For future use by ITU-T | ISO/IEC|


**transfer_characteristics**, as specified in Table E-4, either indicates the reference opto-electronic transfer characteristic
function of the source picture as a function of a source input linear optical intensity input Lc with a nominal real-valued
range of 0 to 1 or indicates the inverse of the reference electro-optical transfer characteristic function as a function of an
output linear optical intensity Lo with a nominal real-valued range of 0 to 1. For interpretation of entries in Table E-4 that
are expressed in terms of multiple curve segments parameterized by the variable  over a region bounded by the variable
 or by the variables  and , the values of  and  are defined to be the positive constants necessary for the curve segments
that meet at the value  to have continuity of both value and slope at the value  . The value of , when applicable, is defined
to be the positive constant necessary for the associated curve segments to meet at the value  . For example, for
transfer_characteristics equal to 1, 6, 11, 14, or 15,  has the value 1 + 5.5 *  _=_ 1.099 296 826 809 442... and  has the
value 0.018 053 968 510 807....


When video_full_range_flag is equal to 1 and either or both of the following conditions apply, transfer_characteristics
shall not be equal to 16 or 18:


- BitDepthY is less than 10.


- chroma_format_idc is not equal to 0 (monochrome) and BitDepthC is less than 10.


When the transfer_characteristics syntax element is not present, the value of transfer_characteristics shall be inferred to be
equal to 2 (the transfer characteristics are unspecified or are determined by the application). Values of
transfer_characteristics that are identified as reserved in Table E-4 are reserved for future use by ITU-T | ISO/IEC and
shall not be present in bitstreams conforming to this version of this Specification. Decoders shall interpret reserved values
of transfer_characteristics as equivalent to the value 2.

NOTE 3 – As indicated in Table E-4, some values of transfer_characteristics are defined in terms of a reference opto-electronic
transfer characteristic function and others are defined in terms of a reference electro-optical transfer characteristic function,
according to the convention that has been applied in other Specifications. In the cases of Rec. ITU-R BT.709-6 and Rec. ITU-R
BT.2020-2 (which may be indicated by transfer_characteristics equal to 1, 6, 14, or 15), although the value is defined in terms of a
reference opto-electronic transfer characteristic function, a suggested corresponding reference electro-optical transfer characteristic
function for flat panel displays used in HDTV studio production has been specified in Rec. ITU-R BT.1886-0.


**Table E-4 – Transfer characteristics interpretation using transfer_characteristics syntax element**







|Value|Transfer characteristic|Informative remark|
|---|---|---|
|0|Reserved|For future use by ITU-T | ISO/IEC|
|1|V = * Lc0.45 − ( − 1 )<br>for 1 >= Lc >= <br>V = 4.500 * Lc <br>for > Lc >= 0|Rec. ITU-R BT.709-6<br>Rec. ITU-R BT.1361-0 conventional<br>colour gamut system (historical)<br>(functionally the same as the value 6,<br>14, and 15)|
|2|Unspecified|Image characteristics are unknown or<br>are determined by the application.|
|3|Reserved|For future use by ITU-T | ISO/IEC|





|4|Assumed display gamma 2.2|Rec. ITU-R BT.470-7 System M<br>(historical)<br>United States National Television<br>System Committee 1953<br>Recommendation for transmission<br>standards for colour television<br>United States Federal<br>Communications Commission Title 47<br>Code of Federal Regulations (2016)<br>73.682 (a) (20)<br>Rec. ITU-R BT.1700-0 625 PAL and<br>625 SECAM|
|---|---|---|
|5|Assumed display gamma 2.8|Rec. ITU-R BT.470-7 System B, G<br>(historical)|
|6|V = * Lc0.45 − ( − 1 )<br>for 1 >= Lc >= <br>V = 4.500 * Lc <br>for > Lc >= 0|Rec. ITU-R BT.601-7 525 or 625<br>Rec. ITU-R BT.1358-1 525 or 625<br>(historical)<br>Rec. ITU-R BT.1700-0 NTSC<br>Society of Motion Picture and<br>Television Engineers 170M (2004)<br>(functionally the same as the value 1)|
|7|V = * Lc0.45 − ( − 1 )<br>for 1 >= Lc >= <br>V = 4.0 * Lc <br>for > Lc >= 0|Society of Motion Picture and<br>Television Engineers 240M (1999,<br>historical)|
|8|V = Lc <br>for 1 > Lc >= 0|Linear transfer characteristics|
|9|V = 1.0 + Log10( Lc ) ÷ 2<br>for 1 >= Lc >= 0.01<br>V = 0.0<br>for 0.01 > Lc >= 0|Logarithmic transfer characteristic<br>(100:1 range)|
|10|V = 1.0 + Log10( Lc ) ÷ 2.5<br>for 1 >= Lc >= Sqrt( 10 ) ÷ 1000<br>V = 0.0<br>for Sqrt( 10 ) ÷ 1000 > Lc >= 0|Logarithmic transfer characteristic<br>(100 * Sqrt( 10 ) : 1 range)|
|11|V = * Lc0.45 − ( − 1 )<br>for Lc >= <br>V = 4.500 * Lc <br>for > Lc > − <br>V = − * ( −Lc )0.45 + ( − 1 )<br>for − >= Lc|IEC 61966-2-4|
|12|V = * Lc0.45 − ( − 1 ) <br>for 1.33 > Lc >= <br>V = 4.500 * Lc <br>for > Lc >= − <br>V = −( * ( −4 * Lc )0.45 − ( − 1 ) ) ÷ 4<br>for − >= Lc >= −0.25|Rec. ITU-R BT.1361-0 extended<br>colour gamut system (historical)|
|13|– If matrix_coefficients is equal to 0<br>     V = * Lc( 1 ÷ 2.4) − ( − 1 )<br>for 1 > Lc >= <br>     V = 12.92 * Lc <br>for > Lc >= 0<br>– Otherwise <br>     V =_α_ * Lc( 1 ÷ 2.4 ) − (_α_ − 1 )<br>for Lc >=_β_ <br>     V = 12.92 * Lc <br>for_β_ > Lc > −_β_ <br>     V = −_α_ * ( −Lc )( 1 ÷ 2.4 ) + (_α_ − 1 )<br>for −_β_ >= Lc|IEC 61966-2-1 sRGB (with<br>matrix_coefficients equal to 0)<br>IEC 61966-2-1 sYCC (with<br>matrix_coefficients equal to 5)|
|14|V = * Lc0.45 − ( − 1 )<br>for 1 >= Lc >= <br>V = 4.500 * Lc <br>for > Lc >= 0|Rec. ITU-R BT.2020-2 (10 bit system)<br>(functionally the same as the values 1,<br>6, and 15)|
|15|V = * Lc0.45 − ( − 1 )<br>for 1 >= Lc >= <br>V = 4.500 * Lc <br>for > Lc >= 0|Rec. ITU-R BT.2020-2 (12 bit system)<br>(functionally the same as the values 1,<br>6, and 15)|
|16|V = ( ( c1 + c2 * Lon ) ÷ ( 1 + c3 * Lon ) )m <br>for all values of Lo <br>c1 = c3 − c2 + 1 = 107 ÷ 128 = 0.8359375<br>c2 = 2413 ÷ 128 = 18.8515625<br>c3 = 2392 ÷ 128 = 18.6875<br>m = 2523 ÷ 32 = 78.84375<br>n = 653 ÷ 4096 = 0.1593017578125<br>for which Lo equal to 1 for peak white is ordinarily intended to correspond to a<br>reference output luminance level of 10 000 candelas per square metre|Society of Motion Picture and<br>Television Engineers ST 2084 for 10,<br>12, 14, and 16-bit systems (2014)<br>Rec. ITU-R BT.2100-2 perceptual<br>quantization (PQ) system|







|17|V = ( 48 * L ÷ 52.37 )( 1 ÷ 2.6 ) for all values of L<br>o o<br>for which L equal to 1 for peak white is ordinarily intended to correspond to a<br>o<br>reference output luminance level of 48 candelas per square metre|Society of Motion Picture and<br>Television Engineers ST 428-1 (2006)|
|---|---|---|
|18|V = a * Ln( 12 * Lc − b ) + c<br>for 1 >= Lc > 1 ÷ 12<br>V = Sqrt( 3 ) * Lc0.5 <br>for 1 ÷ 12 >= Lc >= 0<br>a = 0.17883277<br>b = 0.28466892<br>c = 0.55991073|Association of Radio Industries and<br>Businesses (ARIB) STD-B67<br>Rec. ITU-R BT.2100-2 hybrid log-<br>gamma (HLG) system|
|19..255|Reserved<br>|For future use by ITU-T | ISO/IEC|


NOTE 4 – For transfer_characteristics equal to 18, the equations given in Table E-4 are normalized for a source input linear optical
intensity Lc with a nominal real-valued range of 0 to 1. An alternative scaling that is mathematically equivalent is used in ARIB
STD-B67 with the source input linear optical intensity having a nominal real-valued range of 0 to 12.


**matrix_coefficients** describes the matrix coefficients used in deriving luma and chroma signals from the green, blue, and
red, or Y, Z, and X primaries, as specified in Table E-5.


matrix_coefficients shall not be equal to 0 unless both of the following conditions are true:


- BitDepthC is equal to BitDepthY,


- chroma_format_idc is equal to 3 (4:4:4).


The specification of the use of matrix_coefficients equal to 0 under all other conditions is reserved for future use by
ITU-T | ISO/IEC.


matrix_coefficients shall not be equal to 8 unless one of the following conditions is true:


- BitDepthC is equal to BitDepthY,


- BitDepthC is equal to BitDepthY + 1 and chroma_format_idc is equal to 3 (4:4:4).


The specification of the use of matrix_coefficients equal to 8 under all other conditions is reserved for future use by
ITU-T | ISO/IEC.


When the matrix_coefficients syntax element is not present, the value of matrix_coefficients is inferred to be equal to 2
(unspecified).


The interpretation of matrix_coefficients, together with colour_primaries and transfer_characteristics, is specified by the
equations below.

NOTE 5 – For purposes of YZX representation when matrix_coefficients is equal to 0, the symbols R, G, and B would be substituted
for X, Y, and Z, respectively, in the following descriptions of Equations E-1 to E-3, E-13 to E-15, E-19 to E-21, E-28 to E-30, and
E-31 to E-33.


ER, EG, and EB are defined as "linear-domain" real-valued signals based on the indicated colour primaries before application
of the transfer characteristics function.


Nominal white is specified as having ER equal to 1, EG equal to 1, and EB equal to 1.


Nominal black is specified as having ER equal to 0, EG equal to 0, and EB equal to 0.


The application of the transfer characteristics function is denoted by ( x )′ for an argument x.

- If matrix_coefficients is not equal to 14, the signals E′R, E′G, and E′B are determined by application of the transfer
characteristics function as follows:


E′R = ( ER )′ (E-1)


E′G = ( EG )′ (E-2)


E′B = ( EB )′ (E-3)


In this case, the range of E′R, E′G, and E′B is specified as follows:

   - If transfer_characteristics is equal to 11 or 12, or transfer_characteristics is equal to 13 and matrix_coefficients
is not equal to 0, E′R, E′G, and E′B are real numbers with values that have a larger range than the range of 0 to 1,
and their range is not specified in this Specification.

   - Otherwise, E′R, E′G and E′B are real numbers with values in the range of 0 to 1.





- Otherwise (matrix_coefficients is equal to 14), the "linear-domain" real-valued signals EL, EM, and ES are determined
as follows:


EL = ( 1688 * ER + 2146 * EG + 262 * EB ) ÷ 4096 (E-4)


EM = ( 683 * ER + 2951 * EG + 462 * EB ) ÷ 4096 (E-5)


ES = ( 99 * ER + 309 * EG + 3688 * EB ) ÷ 4096 (E-6)


In this case, the signals E′L, E′M, and E′S are determined by application of the transfer characteristics function as
follows:


E′L = ( EL )′ (E-7)


E′M = ( EM )′ (E-8)


E′S = ( ES )′ (E-9)


The interpretation of matrix_coefficients is specified as follows:

- If video_full_range_flag is equal to 0, the following applies:

   - If matrix_coefficients is equal to 1, 4, 5, 6, 7, 9, 10, 11, 12, 13, or 14, the following equations apply:


Y = Clip1Y( Round( ( 1 << ( BitDepthY − 8 ) ) * ( 219 * E′Y + 16 ) ) ) (E-10)


Cb = Clip1C( Round( ( 1 << ( BitDepthC − 8 ) ) * ( 224 * E′PB + 128 ) ) ) (E-11)


Cr = Clip1C( Round( ( 1 << ( BitDepthC − 8 ) ) * ( 224 * E′PR + 128 ) ) ) (E-12)


   - Otherwise, if matrix_coefficients is equal to 0 or 8, the following equations apply:


R = Clip1Y( ( 1 << ( BitDepthY − 8 ) ) * ( 219 * E′R + 16 ) ) (E-13)


G = Clip1Y( ( 1 << ( BitDepthY − 8 ) ) * ( 219 * E′G + 16 ) ) (E-14)


B = Clip1Y( ( 1 << ( BitDepthY − 8 ) ) * ( 219 * E′B + 16 ) ) (E-15)


   - Otherwise, if matrix_coefficients is equal to 2, the interpretation of the matrix_coefficients syntax element is
unknown or is determined by the application.

   - Otherwise (matrix_coefficients is not equal to 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, or 14), the interpretation of
the matrix_coefficients syntax element is reserved for future definition by ITU-T | ISO/IEC.

- Otherwise (video_full_range_flag is equal to 1), the following applies:

   - If matrix_coefficients is equal to 1, 4, 5, 6, 7, 9, 10, 11, 12, 13, or 14, the following equations apply:


Y = Clip1Y( Round( ( ( 1 << BitDepthY ) − 1 ) * E′Y ) ) (E-16)


Cb = Clip1C( Round( ( ( 1 << BitDepthC ) − 1 ) * E′PB + ( 1 << ( BitDepthC − 1 ) ) ) ) (E-17)


Cr = Clip1C( Round( ( ( 1 << BitDepthC ) − 1 ) * E′PR + ( 1 << ( BitDepthC − 1 ) ) ) ) (E-18)


   - Otherwise, if matrix_coefficients is equal to 0 or 8, the following equations apply:


R = Clip1Y( ( ( 1 << BitDepthY ) − 1 ) * E′R ) (E-19)


G = Clip1Y( ( ( 1 << BitDepthY ) − 1 ) * E′G ) (E-20)


B = Clip1Y( ( ( 1 << BitDepthY ) − 1 ) * E′B ) (E-21)


   - Otherwise, if matrix_coefficients is equal to 2, the interpretation of the matrix_coefficients syntax element is
unknown or is determined by the application.





   - Otherwise (matrix_coefficients is not equal to 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, or 14), the interpretation of
the matrix_coefficients syntax element is reserved for future definition by ITU-T | ISO/IEC.


Reserved values for matrix_coefficients shall not be present in bitstreams conforming to this version of this Specification.
Decoders shall interpret reserved values of matrix_coefficients as equivalent to the value 2.


It is a requirement of bitstream conformance to this version of this Specification that when colour_primaries is not equal
to 1, 4, 5, 6, 7, 8, 9, 10, 11, 12, or 22, matrix_coefficients shall not be equal to 12 or 13.


When matrix_coefficients is equal to 1, 4, 5, 6, 7, 9, 10, 11, 12, or 13, the constants KR and KB are specified as follows:

- If matrix_coefficients is not equal to 12 or 13, the constants KR and KB are specified in Table E-5.

- Otherwise (matrix_coefficients is equal to 12 or 13), the constants KR and KB are computed as follows, using the
chromaticity coordinates (xR, yR), (xG, yG), (xB, yB), and (xW, yW) specified by Table E-3 for the colour_primaries
syntax element for the red, green, blue, and white colour primaries, respectively:




- yB - zG ) + yW - (xB - zG - xG - zB) + zW - (xG - yB
- y - z ) + x - (y - z - y - z ) + x - (y - z


y - (x - (y - z - y - z ) + y - (x - z - x - z ) + z - (x - y - x - y ))



KR = R W G B B G W B G G B W G B B G (E-22)



R W G B B G W B G G B W G B B G



y - (x - (y - z - y - z ) + x - (y - z - y - z ) + x - (y - z - y - z ))



W R G B B G G B R R B B R G G R




- y - z ) + y - (x - z - x - z ) + z - (x - y


y - (x - (y - z - y - z ) + y - (x - z - x - z ) + z - (x - y - x - y ))



KB = B W R G G R W G R R G W R G G R (E-23)



B W R G G R W G R R G W R G G R




- y - z ) + x - (y - z - y - z ) + x - (y - z


y - (x - (y - z - y - z ) + x - (y - z - y - z ) + x - (y - z - y - z ))



W R G B B G G B R R B B R G G R



where the values of zR, zG, zB, and zW, are given by.


zR = 1 − ( xR + yR ) (E-24)


zG = 1 − ( xG + yG ) (E-25)


zB = 1 − ( xB + yB ) (E-26)


zW = 1 − ( xW + yW ) (E-27)


The variables E′Y, E′PB, and E′PR (for matrix_coefficients not equal to 0 or 8) or Y, Cb, and Cr (for matrix_coefficients
equal to 0 or 8) are specified as follows:

- If matrix_coefficients is not equal to 0, 8, 10, 11, 13, or 14, the following equations apply:


E′Y = KR * ′ER + ( 1 − KR − KB ) * E′G + KB * E′B (E-28)


E′PB = 0.5 * ( E′B − E′Y ) ÷ ( 1 − KB ) (E-29)


E′PR = 0.5 * ( E′R − E′Y ) ÷ ( 1 − KR ) (E-30)


E′Y is a real number with the value 0 associated with nominal black and the value 1 associated with nominal white.
E′PB and E′PR are real numbers with the value 0 associated with both nominal black and nominal white. When
transfer_characteristics is not equal to 11 or 12, E′Y is a real number with values in the range of 0 to 1. When
transfer_characteristics is not equal to 11 or 12, E′PB and E′PR are real numbers with values in the range of −0.5 to 0.5.
When transfer_characteristics is equal to 11, or 12, E′Y, E′PB and E′PR are real numbers with a larger range not specified
in this Specification.

- Otherwise, if matrix_coefficients is equal to 0, the following equations apply:


Y  = Round( G ) (E-31)


Cb = Round( B ) (E-32)


Cr = Round( R ) (E-33)


- Otherwise, if matrix_coefficients is equal to 8, the following applies:

   - If BitDepthC is equal to BitDepthY, the following equations apply:


Y  = Round( 0.5 * G + 0.25 * ( R + B ) ) (E-34)


Cb = Round( 0.5 * G − 0.25 * ( R + B ) ) + ( 1 << ( BitDepthC − 1 ) ) (E-35)





Cr = Round( 0.5 * ( R − B ) ) + ( 1 << ( BitDepthC − 1 ) ) (E-36)


NOTE 6 – In this case, for purposes of the YCgCo nomenclature used in Table E-5, Cb and Cr of Equations E-35 and E-36
would be referred to as Cg and Co, respectively. An appropriate inverse conversion for Equations E-34 to E-36 is as follows:

t  = Y − ( Cb − ( 1 << ( BitDepthC − 1 ) ) ) (E-37)
G = Clip1Y( Y + ( Cb − ( 1 << ( BitDepthC − 1 ) ) ) ) (E-38)
B = Clip1Y( t − ( Cr − ( 1 << ( BitDepthC − 1 ) ) ) ) (E-39)
R = Clip1Y( t + ( Cr − ( 1 << ( BitDepthC − 1 ) ) ) ) (E-40)


   - Otherwise (BitDepthC is not equal to BitDepthY), the following equations apply:


Cr = Round( R ) − Round( B ) + ( 1 << ( BitDepthC − 1 ) ) (E-41)


t = Round( B ) + ( ( Cr − ( 1 << ( BitDepthC − 1 ) ) ) >> 1 ) (E-42)


Cb = Round( G ) − t + ( 1 << ( BitDepthC − 1 ) ) (E-43)


Y = t + ( ( Cb − ( 1 << ( BitDepthC − 1 ) ) ) >> 1 ) (E-44)


NOTE 7 – In this case, for purposes of the YCgCo nomenclature used in Table E-5, Cb and Cr of Equations E-43 and E-41
would be referred to as Cg and Co, respectively. An appropriate inverse conversion for Equations E-41 to E-44 is as follows:

t  = Y − ( ( Cb − ( 1 << ( BitDepthC − 1 ) ) ) >> 1 ) (E-45)
G = Clip1Y( t + ( Cb − ( 1 << ( BitDepthC − 1 ) ) ) ) (E-46)
B = Clip1Y( t − ( ( Cr − ( 1 << ( BitDepthC − 1 ) ) ) >> 1 ) ) (E-47)
R = Clip1Y( B + ( Cr − ( 1 << ( BitDepthC − 1 ) ) ) ) (E-48)

- Otherwise, if matrix_coefficients is equal to 10 or 13, the signal E′Y is determined by application of the transfer
characteristics function as follows:


EY = KR * ER + ( 1 − KR − KB ) * EG + KB * EB (E-49)


E′Y = ( EY )′ (E-50)


In this case, EY is defined from the "linear-domain" signals for ER, EG, and EB, prior to application of the transfer
characteristics function, which is then applied to produce the signal E′Y. EY and E′Y are real values with the value 0
associated with nominal black and the value 1 associated with nominal white.

In this case, the signals E′PB and E′PR are determined as follows:


E′PB = ( E′B − E′Y ) ÷ ( 2 * NB )   for −NB <= E′B − E′Y <= 0 (E-51)


E′PB = ( E′B − E′Y ) ÷ ( 2 * PB )   for 0 < E′B − E′Y <= PB (E-52)


E′PR = ( E′R − E′Y ) ÷ ( 2 * NR )   for −NR <= E′R − E′Y <= 0 (E-53)


E′PR = ( E′R − E′Y ) ÷ ( 2 * PR )   for 0 < E′R − E′Y <= PR (E-54)


where the constants NB, PB, NR, and PR are determined by application of the transfer characteristics function to
expressions involving the constants KB and KR as follows:


NB = ( 1 − KB )′ (E-55)


PB = 1 − ( KB )′ (E-56)


NR = ( 1 − KR )′ (E-57)


PR = 1 − ( KR )′ (E-58)

- Otherwise, if matrix_coefficients is equal to 11, the following equations apply:


E′Y = E′G (E-59)


E′PB = ( 0.986566 * E′B − E′Y ) ÷ 2 (E-60)





E′PR = ( E′R − 0.991902 * E′Y ) ÷ 2 (E-61)


NOTE 8 – In this case, for purposes of the Y′D′ZD′X nomenclature used in Table E-5, E′PB and E′PR of Equations E-60 and E-61
would be referred to as D′Z and D′X, respectively.

- Otherwise (matrix_coefficients is equal to 14), the following applies:

   - If transfer_characteristics is not equal to 18, the following equations apply:


E′Y = 0.5 * ( E′L + E′M ) (E-62)


E′PB = ( 6610 * E′L − 13613 * E′M + 7003 * E′S ) ÷ 4096 (E-63)


E′PR = ( 17933 * E′L − 17390 * E′M − 543 * E′S ) ÷ 4096 (E-64)

   - Otherwise, the following equations apply:


E′Y = 0.5 * ( E′L + E′M ) (E-65)


E′PB = ( 3625 * E′L − 7465 * E′M + 3840 * E′S ) ÷ 4096 (E-66)


E′PR = ( 9500 * E′L − 9212 * E′M − 288 * E′S ) ÷ 4096 (E-67)


NOTE 9 – In this case, for purposes of the ICTCP nomenclature used in Table E-5, E ′ Y, E′PB, and E′PR of Equations E-62, E-63 and
E-64 may be referred to as I, CT, and CP, respectively. Equations E-62 through E-64 were designed specifically for use with
transfer_characteristics equal to 16 (PQ), and Equations E-65 through E-67 were designed specifically for use with
transfer_characteristics equal to 18 (HLG).





**Table E-5 – Matrix coefficients interpretation using matrix_coefficients syntax element**

|Value|Matrix|Informative remark|
|---|---|---|
|0|GBR|The identity matrix.<br>Typically used for GBR (often referred to as RGB); however, may also be<br>used for YZX (often referred to as XYZ)<br>IEC 61966-2-1 (sRGB)<br>Society of Motion Picture and Television Engineers ST 428-1 XYZ (2006)<br>See Equations E-1 to E-3|
|1|KR = 0.2126; KB = 0.0722|Rec. ITU-R BT.709-6<br>Rec. ITU-R BT.1361-0 conventional colour gamut system and extended<br>colour gamut system (historical)<br>IEC 61966-2-4 xvYCC709 <br>Society of Motion Picture and Television Engineers RP 177 (1993)<br>Annex B<br>See Equations E-28 to E-30|
|2|Unspecified|Image characteristics are unknown or are determined by the application.|
|3|Reserved|For future use by ITU-T | ISO/IEC|
|4|KR = 0.30;  KB = 0.11|United States Federal Communications Commission Title 47 Code of<br>Federal Regulations (2016) 73.682 (a) (20)<br>See Equations E-28 to E-30|
|5|KR = 0.299; KB = 0.114|Rec. ITU-R BT.470-7 System B, G (historical)<br>Rec. ITU-R BT.601-7 625<br>Rec. ITU-R BT.1358-0 625 (historical)<br>Rec. ITU-R BT.1700-0 625 PAL and 625 SECAM<br>IEC 61966-2-1 (sYCC)<br>IEC 61966-2-4 xvYCC601 <br>(functionally the same as the value 6)<br>See Equations E-28 to E-30|
|6|KR = 0.299; KB = 0.114|Rec. ITU-R BT.601-7 525<br>Rec. ITU-R BT.1358-1 525 or 625<br>Rec. ITU-R BT.1700-0 NTSC<br>Society of Motion Picture and Television Engineers 170M (2004)<br>(functionally the same as the value 5)<br>See Equations E-28 to E-30|
|7|KR = 0.212; KB = 0.087|Society of Motion Picture and Television Engineers 240M (1999, historical)<br>See Equations E-28 to E-30|
|8|YCgCo|See Equations E-34 to E-48|
|9|KR = 0.2627; KB = 0.0593|Rec. ITU-R BT.2020-2 non-constant luminance system<br>Rec. ITU-R BT.2100-2 Y′CbCr<br>See Equations E-28 to E-30|
|10|KR = 0.2627; KB = 0.0593|Rec. ITU-R BT.2020-2 constant luminance system<br>See Equations E-49 to E-58|
|11|Y′D′ZD′X|Society of Motion Picture and Television Engineers ST 2085 (2015)<br>See Equations E-59 to E-61|
|12|See Equations E-22 to E-27.|Chromaticity-derived non-constant luminance system<br>See Equations E-28 to E-30.|
|13|See Equations E-22 to E-27.|Chromaticity-derived constant luminance system<br>See Equations E-49 to E-58.|
|14|ICTCP|Rec. ITU-R BT.2100-2 ICTCP <br>See EquationsE-62 toE-64. for transfer_characteristics value 16 (PQ)<br>See EquationsE-65 toE-67 for transfer_characteristics value 18 (HLG)|
|15..255|Reserved|For future use by ITU-T | ISO/IEC|






**chroma_loc_info_present_flag** equal to 1 specifies that chroma_sample_loc_type_top_field and
chroma_sample_loc_type_bottom_field are present. chroma_loc_info_present_flag equal to 0 specifies that
chroma_sample_loc_type_top_field and chroma_sample_loc_type_bottom_field are not present.


When chroma_format_idc is not equal to 1, chroma_loc_info_present_flag should be equal to 0.


**chroma_sample_loc_type_top_field** and **chroma_sample_loc_type_bottom_field** specify the location of chroma
samples as follows:


- If chroma_format_idc is equal to 1 (4:2:0 chroma format), chroma_sample_loc_type_top_field and
chroma_sample_loc_type_bottom_field specify the location of chroma samples for the top field and the bottom field,
respectively, as shown in Figure E-1.


- Otherwise (chroma_format_idc is not equal to 1), the values of the syntax elements
chroma_sample_loc_type_top_field and chroma_sample_loc_type_bottom_field shall be ignored. When
chroma_format_idc is equal to 2 (4:2:2 chroma format) or 3 (4:4:4 chroma format), the location of chroma samples
is specified in clause 6.2. When chroma_format_idc is equal to 0, there is no chroma sample array.


The value of chroma_sample_loc_type_top_field and chroma_sample_loc_type_bottom_field shall be in the range of 0
to 5, inclusive. When the chroma_sample_loc_type_top_field and chroma_sample_loc_type_bottom_field are not present,
the values of chroma_sample_loc_type_top_field and chroma_sample_loc_type_bottom_field shall be inferred to be equal
to 0.

NOTE 10 – When coding progressive-scan source material, the syntax elements chroma_sample_loc_type_top_field and
chroma_sample_loc_type_bottom_field should have the same value.





H.264(09)_FE-1


**Interpretation of symbols**
Luma sample position indications:


Luma sample top field Luma sample bottom field


Chroma sample position indications, where grey fill indicates a bottom field
sample type and no fill indicates a top field sample type:


Chroma sample type 2 Chroma sample type 3


Chroma sample type 0 Chroma sample type 1


Chroma sample type 4 Chroma sample type 5


**Figure E-** 1 **– Location of chroma samples for top and bottom fields for chroma_format_idc equal to 1 (4:2:0**
**chroma format) as a function of chroma_sample_loc_type_top_field and chroma_sample_loc_type_bottom_field**


**timing_info_present_flag** equal to 1 specifies that num_units_in_tick, time_scale and fixed_frame_rate_flag are present
in the bitstream. timing_info_present_flag equal to 0 specifies that num_units_in_tick, time_scale and
fixed_frame_rate_flag are not present in the bitstream.


**num_units_in_tick** is the number of time units of a clock operating at the frequency time_scale Hz that corresponds to
one increment (called a clock tick) of a clock tick counter. num_units_in_tick shall be greater than 0. A clock tick is the
minimum interval of time that can be represented in the coded data. For example, when the frame rate of a video signal is
30 000 ÷ 1001 Hz, time_scale may be equal to 60 000 and num_units_in_tick may be equal to 1001. See Equation C-1.


**time_scale** is the number of time units that pass in one second. For example, a time coordinate system that measures time
using a 27 MHz clock has a time_scale of 27 000 000. time_scale shall be greater than 0.


**fixed_frame_rate_flag** equal to 1 indicates that the temporal distance between the HRD output times of any two
consecutive pictures in output order is constrained as follows. fixed_frame_rate_flag equal to 0 indicates that no such
constraints apply to the temporal distance between the HRD output times of any two consecutive pictures in output order.


When fixed_frame_rate_flag is not present, it shall be inferred to be equal to 0.


For each picture n where n indicates the n-th picture (in output order) that is output and picture n is not the last picture in
the bitstream (in output order) that is output, the value of  tfi,dpb( n ) is specified by


 tfi,dpb( n ) =  to,dpb( n ) ÷ DeltaTfiDivisor (E-68)





where  to,dpb( n ) is specified in Equation C-13 and DeltaTfiDivisor is specified by Table E-6 based on the value of
pic_struct_present_flag, field_pic_flag, and pic_struct for the coded video sequence containing picture n. Entries marked
"-" in Table E-6 indicate a lack of dependence of DeltaTfiDivisor on the corresponding syntax element.


When fixed_frame_rate_flag is equal to 1 for a coded video sequence containing picture n, the value computed for
 tfi,dpb( n ) shall be equal to tc as specified in Equation C-1 (using the value of tc for the coded video sequence containing
picture n) when either or both of the following conditions are true for the following picture nn that is specified for use in
Equation C-13:


- picture nn is in the same coded video sequence as picture n.


- picture nn is in a different coded video sequence and fixed_frame_rate_flag is equal to 1 in the coded video sequence
containing picture nn and the value of num_units_in_tick ÷ time_scale is the same for both coded video sequences.


**Table E-6 – Divisor for computation of**  **tfi,dpb( n )**

|pic_struct_present_flag|field_pic_flag|pic_struct|DeltaTfiDivisor|
|---|---|---|---|
|0|1|-|1|
|1|-|1|1|
|1|-|2|1|
|0|0|-|2|
|1|-|0|2|
|1|-|3|2|
|1|-|4|2|
|1|-|5|3|
|1|-|6|3|
|1|-|7|4|
|1|-|8|6|



NOTE 11 – In order to produce a DeltaTfiDivisor other than 2 for a picture with field_pic_flag equal to 0, pic_struct_present_flag
must be equal to 1.


**nal_hrd_parameters_present_flag** equal to 1 specifies that NAL HRD parameters (pertaining to the Type II bitstream
conformance point) are present. nal_hrd_parameters_present_flag equal to 0 specifies that NAL HRD parameters are not
present.

NOTE 12 – When nal_hrd_parameters_present_flag is equal to 0, the conformance of the bitstream cannot be verified without
provision of the NAL HRD parameters and all buffering period SEI messages, and, when vcl_hrd_parameters_present_flag is also
equal to 0, all picture timing SEI messages, by some means not specified in this Recommendation | International Standard.


When nal_hrd_parameters_present_flag is equal to 1, NAL HRD parameters (clauses E.1.2 and E.2.2) immediately follow
the flag.


The variable NalHrdBpPresentFlag is derived as follows:


- If any of the following is true, the value of NalHrdBpPresentFlag shall be set equal to 1:

   - nal_hrd_parameters_present_flag is present in the bitstream and is equal to 1,

   - the need for presence of buffering periods for NAL HRD operation to be present in the bitstream in buffering
period SEI messages is determined by the application, by some means not specified in this Recommendation |
International Standard.


- Otherwise, the value of NalHrdBpPresentFlag shall be set equal to 0.


**vcl_hrd_parameters_present_flag** equal to 1 specifies that VCL HRD parameters (pertaining to the Type I bitstream
conformance point) are present. vcl_hrd_parameters_present_flag equal to 0 specifies that VCL HRD parameters are not
present.

NOTE 13 – When vcl_hrd_parameters_present_flag is equal to 0, the conformance of the bitstream cannot be verified without
provision of the VCL HRD parameters and all buffering period SEI messages and, when nal_hrd_parameters_present_flag is also
equal to 0, all picture timing SEI messages, by some means not specified in this Recommendation | International Standard.


When vcl_hrd_parameters_present_flag is equal to 1, VCL HRD parameters (clauses E.1.2 and E.2.2) immediately follow
the flag.


The variable VclHrdBpPresentFlag is derived as follows:





- If any of the following is true, the value of VclHrdBpPresentFlag shall be set equal to 1:

   - vcl_hrd_parameters_present_flag is present in the bitstream and is equal to 1,

   - the need for presence of buffering period parameters for VCL HRD operation in the bitstream in buffering period
SEI messages is determined by the application, by some means not specified in this Recommendation |
International Standard.


- Otherwise, the value of VclHrdBpPresentFlag shall be set equal to 0.


The variable CpbDpbDelaysPresentFlag is derived as follows:


- If any of the following is true, the value of CpbDpbDelaysPresentFlag shall be set equal to 1:

   - nal_hrd_parameters_present_flag is present in the bitstream and is equal to 1,

   - vcl_hrd_parameters_present_flag is present in the bitstream and is equal to 1,

   - the need for presence of CPB and DPB output delays in the bitstream in picture timing SEI messages is
determined by the application, by some means not specified in this Recommendation | International Standard.


- Otherwise, the value of CpbDpbDelaysPresentFlag shall be set equal to 0.


**low_delay_hrd_flag** specifies the HRD operational mode as specified in Annex C. When fixed_frame_rate_flag is equal
to 1, low_delay_hrd_flag shall be equal to 0. When low_delay_hrd_flag is not present, its value shall be inferred to be
equal to 1 − fixed_frame_rate_flag.

NOTE 14 – When low_delay_hrd_flag is equal to 1, "big pictures" that violate the nominal CPB removal times due to the number
of bits used by an access unit are permitted. It is expected, but not required, that such "big pictures" occur only occasionally.


**pic_struct_present_flag** equal to 1 specifies that picture timing SEI messages (clause D.2.3) are present that include the
pic_struct syntax element. pic_struct_present_flag equal to 0 specifies that the pic_struct syntax element is not present in
picture timing SEI messages. When pic_struct_present_flag is not present, its value shall be inferred to be equal to 0.


**bitstream_restriction_flag** equal to 1, specifies that the following coded video sequence bitstream restriction parameters
are present. bitstream_restriction_flag equal to 0, specifies that the following coded video sequence bitstream restriction
parameters are not present.


**motion_vectors_over_pic_boundaries_flag** equal to 0 indicates that no sample outside the picture boundaries and no
sample at a fractional sample position for which the sample value is derived using one or more samples outside the picture
boundaries is used for inter prediction of any sample. motion_vectors_over_pic_boundaries_flag equal to 1 indicates that
one or more samples outside picture boundaries may be used in inter prediction. When the
motion_vectors_over_pic_boundaries_flag syntax element is not present, motion_vectors_over_pic_boundaries_flag
value shall be inferred to be equal to 1.


**max_bytes_per_pic_denom** indicates a number of bytes not exceeded by the sum of the sizes of the VCL NAL units
associated with any coded picture in the coded video sequence.


The number of bytes that represent a picture in the NAL unit stream is specified for this purpose as the total number of
bytes of VCL NAL unit data (i.e., the total of the NumBytesInNALunit variables for the VCL NAL units) for the picture.
The value of max_bytes_per_pic_denom shall be in the range of 0 to 16, inclusive.


Depending on max_bytes_per_pic_denom the following applies:


- If max_bytes_per_pic_denom is equal to 0, no limits are indicated.


- Otherwise (max_bytes_per_pic_denom is not equal to 0), it is a requirement of bitstream conformance that no coded
picture shall be represented in the coded video sequence by more than the following number of bytes.


( PicSizeInMbs * RawMbBits ) ÷ ( 8 * max_bytes_per_pic_denom ) (E-69)


When the max_bytes_per_pic_denom syntax element is not present, the value of max_bytes_per_pic_denom shall be
inferred to be equal to 2.


**max_bits_per_mb_denom** indicates an upper bound for the number of coded bits of macroblock_layer( ) data for any
macroblock in any picture of the coded video sequence. The value of max_bits_per_mb_denom shall be in the range of 0
to 16, inclusive.


Depending on max_bits_per_mb_denom the following applies:


- If max_bits_per_mb_denom is equal to 0, no limit is specified by this syntax element.


- Otherwise (max_bits_per_mb_denom is not equal to 0), it is a requirement of bitstream conformance that no coded
macroblock_layer( ) shall be represented in the bitstream by more than the following number of bits.





( 128 + RawMbBits ) ÷ max_bits_per_mb_denom (E-70)


Depending on entropy_coding_mode_flag, the bits of macroblock_layer( ) data are counted as follows:


- If entropy_coding_mode_flag is equal to 0, the number of bits of macroblock_layer( ) data is given by the number of
bits in the macroblock_layer( ) syntax structure for a macroblock.


- Otherwise (entropy_coding_mode_flag is equal to 1), the number of bits of macroblock_layer( ) data for a macroblock
is given by the number of times read_bits( 1 ) is called in clauses 9.3.3.2.2 and 9.3.3.2.3 when parsing the
macroblock_layer( ) associated with the macroblock.


When the max_bits_per_mb_denom is not present, the value of max_bits_per_mb_denom shall be inferred to be equal
to 1.


**log2_max_mv_length_horizontal** and **log2_max_mv_length_vertical** indicate the maximum absolute value of a
decoded horizontal and vertical motion vector component, respectively, in ¼ luma sample units, for all pictures in the
coded video sequence. A value of n asserts that no value of a motion vector component shall exceed the range from −2 [n] to
2 [n] - 1, inclusive, in units of ¼ luma sample displacement. The value of log2_max_mv_length_horizontal shall be in the
range of 0 to 15, inclusive. The value of log2_max_mv_length_vertical shall be in the range of 0 to 15, inclusive. When
log2_max_mv_length_horizontal is not present, the values of log2_max_mv_length_horizontal and
log2_max_mv_length_vertical shall be inferred to be equal to 15.

NOTE 15 – The maximum absolute value of a decoded vertical or horizontal motion vector component is also constrained by profile
and level limits as specified in Annex A and clauses G.10 and H.10.


**max_num_reorder_frames** indicates an upper bound for the number of frames buffers, in the decoded picture buffer
(DPB), that are required for storing frames, complementary field pairs, and non-paired fields before output. It is a
requirement of bitstream conformance that the maximum number of frames, complementary field pairs, or non-paired
fields that precede any frame, complementary field pair, or non-paired field in the coded video sequence in decoding order
and follow it in output order shall be less than or equal to max_num_reorder_frames. The value of
max_num_reorder_frames shall be in the range of 0 to max_dec_frame_buffering, inclusive. When the
max_num_reorder_frames syntax element is not present, the value of max_num_reorder_frames value shall be inferred as
follows:


- If profile_idc is equal to 44, 86, 100, 110, 122, or 244 and constraint_set3_flag is equal to 1, the value of
max_num_reorder_frames shall be inferred to be equal to 0.


- Otherwise (profile_idc is not equal to 44, 86, 100, 110, 122, or 244 or constraint_set3_flag is equal to 0), the value of
max_num_reorder_frames shall be inferred to be equal to MaxDpbFrames.


**max_dec_frame_buffering** specifies the required size of the HRD decoded picture buffer (DPB) in units of frame buffers.
It is a requirement of bitstream conformance that the coded video sequence shall not require a decoded picture buffer with
size of more than Max( 1, max_dec_frame_buffering ) frame buffers to enable the output of decoded pictures at the output
times specified by dpb_output_delay of the picture timing SEI messages. The value of max_dec_frame_buffering shall be
greater than or equal to max_num_ref_frames. An upper bound for the value of max_dec_frame_buffering is specified by
the level limits in clauses A.3.1, A.3.2, G.10.2.1, and H.10.2.


When the max_dec_frame_buffering syntax element is not present, the value of max_dec_frame_buffering shall be inferred
as follows:


- If profile_idc is equal to 44, 86, 100, 110, 122, or 244 and constraint_set3_flag is equal to 1, the value of
max_dec_frame_buffering shall be inferred to be equal to 0.


- Otherwise (profile_idc is not equal to 44, 86, 100, 110, 122, or 244 or constraint_set3_flag is equal to 0), the value of
max_dec_frame_buffering shall be inferred to be equal to MaxDpbFrames.


**E.2.2** **HRD parameters semantics**


The syntax category of the HRD parameters syntax structure shall be inferred as follows:


- If the HRD parameters syntax structure is not part of an SEI message, the syntax category of the HRD parameters
syntax structure is inferred to be equal to 0.


- Otherwise (the HRD parameters syntax structure is part of the base layer temporal HRD SEI message as specified in
clause G.13 or the base view temporal HRD SEI message as specified in clause H.13), the syntax category of the
HRD parameters syntax structure is inferred to be equal to 5.


**cpb_cnt_minus1** plus 1 specifies the number of alternative CPB specifications in the bitstream. The value of
cpb_cnt_minus1 shall be in the range of 0 to 31, inclusive. When low_delay_hrd_flag is equal to 1, cpb_cnt_minus1 shall
be equal to 0. When cpb_cnt_minus1 is not present, it shall be inferred to be equal to 0.





**bit_rate_scale** (together with bit_rate_value_minus1[ SchedSelIdx ]) specifies the maximum input bit rate of the
SchedSelIdx-th CPB.


**cpb_size_scale** (together with cpb_size_value_minus1[ SchedSelIdx ]) specifies the CPB size of the SchedSelIdx-th CPB.


**bit_rate_value_minus1[** SchedSelIdx **]** (together with bit_rate_scale) specifies the maximum input bit rate for the
SchedSelIdx-th CPB. bit_rate_value_minus1[ SchedSelIdx ] shall be in the range of 0 to 2 [32] - 2, inclusive. For any
SchedSelIdx > 0, bit_rate_value_minus1[ SchedSelIdx ] shall be greater than bit_rate_value_minus1[ SchedSelIdx − 1 ].
The bit rate in bits per second is given by


BitRate[ SchedSelIdx ] = ( bit_rate_value_minus1[ SchedSelIdx ] + 1 ) * 2 [(6 + bit_rate_scale)] (E-71)


When the bit_rate_value_minus1[ SchedSelIdx ] syntax element is not present, the value of BitRate[ SchedSelIdx ] shall
be inferred as follows:


- If profile_idc is equal to 66, 77, or 88, BitRate[ SchedSelIdx ] shall be inferred to be equal to 1000 * MaxBR for VCL
HRD parameters and to be equal to 1200 * MaxBR for NAL HRD parameters, where MaxBR is specified in
clause A.3.1.


- Otherwise, BitRate[ SchedSelIdx ] shall be inferred to be equal to cpbBrVclFactor * MaxBR for VCL HRD
parameters and to be equal to cpbBrNalFactor * MaxBR for NAL HRD parameters, where MaxBR is specified in
clause A.3.1 and cpbBrVclFactor and cpbBrNalFactor are specified in clause A.3.3 (for profiles specified in Annex A)
or clause G.10.2.2 (for profiles specified in Annex G) or clause H.10.2 (for profiles specified in Annex H).


**cpb_size_value_minus1[** SchedSelIdx **]** is used together with cpb_size_scale to specify the SchedSelIdx-th CPB size.
cpb_size_value_minus1[ SchedSelIdx ] shall be in the range of 0 to 2 [32] - 2, inclusive. For any SchedSelIdx greater than 0,
cpb_size_value_minus1[ SchedSelIdx ] shall be less than or equal to cpb_size_value_minus1[ SchedSelIdx −1 ].


The CPB size in bits is given by


CpbSize[ SchedSelIdx ] = ( cpb_size_value_minus1[ SchedSelIdx ] + 1 ) * 2 [(4 + cpb_size_scale)] (E-72)


When the cpb_size_value_minus1[ SchedSelIdx ] syntax element is not present, the value of CpbSize[ SchedSelIdx ] shall
be inferred as follows:


- If profile_idc is equal to 66, 77, or 88, CpbSize[ SchedSelIdx ] shall be inferred to be equal to 1000 * MaxCPB for
VCL HRD parameters and to be equal to 1200 * MaxCPB for NAL HRD parameters, where MaxCPB is specified in
clause A.3.1.


- Otherwise, CpbSize[ SchedSelIdx ] shall be inferred to be equal to cpbBrVclFactor * MaxCPB for VCL HRD
parameters and to be equal to cpbBrNalFactor * MaxCPB for NAL HRD parameters, where MaxCPB is specified in
clause A.3.1 and cpbBrVclFactor and cpbBrNalFactor are specified in clause A.3.3 (for profiles specified in Annex A)
or clause G.10.2.2 (for profiles specified in Annex G) or clause H.10.2 (for profiles specified in Annex H).


**cbr_flag[** SchedSelIdx **]** equal to 0 specifies that to decode this bitstream by the HRD using the SchedSelIdx-th CPB
specification, the hypothetical stream delivery scheduler (HSS) operates in an intermittent bit rate mode.
cbr_flag[ SchedSelIdx ] equal to 1 specifies that the HSS operates in a constant bit rate (CBR) mode. When the
cbr_flag[ SchedSelIdx ] syntax element is not present, the value of cbr_flag shall be inferred to be equal to 0.


**initial_cpb_removal_delay_length_minus1** specifies the length in bits of the initial_cpb_removal_delay[ SchedSelIdx ]
and initial_cpb_removal_delay_offset[ SchedSelIdx ] syntax elements of the buffering period SEI message. The length of
initial_cpb_removal_delay[ SchedSelIdx ] and of initial_cpb_removal_delay_offset[ SchedSelIdx ] is
initial_cpb_removal_delay_length_minus1 + 1. When the initial_cpb_removal_delay_length_minus1 syntax element is
present in more than one hrd_parameters( ) syntax structure within the VUI parameters syntax structure, the value of the
initial_cpb_removal_delay_length_minus1 parameters shall be equal in both hrd_parameters( ) syntax structures. When
the initial_cpb_removal_delay_length_minus1 syntax element is not present, it shall be inferred to be equal to 23.


**cpb_removal_delay_length_minus1** specifies the length in bits of the cpb_removal_delay syntax element. The length of
the cpb_removal_delay syntax element of the picture timing SEI message is cpb_removal_delay_length_minus1 + 1.
When the cpb_removal_delay_length_minus1 syntax element is present in more than one hrd_parameters( ) syntax
structure within the VUI parameters syntax structure, the value of the cpb_removal_delay_length_minus1 parameters shall
be equal in both hrd_parameters( ) syntax structures. When the cpb_removal_delay_length_minus1 syntax element is not
present, it shall be inferred to be equal to 23.


**dpb_output_delay_length_minus1** specifies the length in bits of the dpb_output_delay syntax element. The length of the
dpb_output_delay syntax element of the picture timing SEI message is dpb_output_delay_length_minus1 + 1. When the
dpb_output_delay_length_minus1 syntax element is present in more than one hrd_parameters( ) syntax structure within
the VUI parameters syntax structure, the value of the dpb_output_delay_length_minus1 parameters shall be equal in both





hrd_parameters( ) syntax structures. When the dpb_output_delay_length_minus1 syntax element is not present, it shall be
inferred to be equal to 23.


**time_offset_length** greater than 0 specifies the length in bits of the time_offset syntax element. time_offset_length equal
to 0 specifies that the time_offset syntax element is not present. When the time_offset_length syntax element is present in
more than one hrd_parameters( ) syntax structure within the VUI parameters syntax structure, the value of the
time_offset_length parameters shall be equal in both hrd_parameters( ) syntax structures. When the time_offset_length
syntax element is not present, it shall be inferred to be equal to 24.
