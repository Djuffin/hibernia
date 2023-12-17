use num_traits::cast::FromPrimitive;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, FromPrimitive)]
pub enum NalUnitType {
    #[default]
    Unspecified = 0,
    NonIDRSlice = 1,
    SliceDataA = 2,
    SliceDataB = 3,
    SliceDataC = 4,
    IDRSlice = 5,
    SupplementalEnhancementInfo = 6,
    SeqParameterSet = 7,
    PicParameterSet = 8,
    AccessUnitDelimiter = 9,
    EndOfSeq = 10,
    EndOfStream = 11,
    FillerData = 12,
    SeqParameterSetExtension = 13,
    Prefix = 14,
    SubsetSeqParameterSet = 15,
    DepthParameterSet = 16,
    CodedSliceAux = 19,
    CodedSliceExtension = 20,
    CodedSliceExtensionForDepthView = 21,
    Reserved = 17, // 17, 18, 22, 23 are reserved
}

impl TryFrom<u32> for NalUnitType {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            17 | 18 | 22 | 23 => Ok(NalUnitType::Reserved),
            0 | 24..=31 => Ok(NalUnitType::Unspecified),
            _ => FromPrimitive::from_u32(value).ok_or("Unknown NAL unit type."),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct NalHeader {
    pub nal_ref_idc: u8,
    pub nal_unit_type: NalUnitType,
}
