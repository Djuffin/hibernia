pub mod cavlc;
pub mod macroblock;
pub mod nal;
pub mod parser;
pub mod pps;
pub mod slice;
pub mod sps;
pub mod tables;

use num_traits::cast::FromPrimitive;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, FromPrimitive)]
pub enum Profile {
    #[default]
    Baseline = 66,
    Main = 77,
    High = 100,
    High422 = 122,
    High10 = 110,
    High444 = 244,
    Extended = 88,
    ScalableBase = 83,
    ScalableHigh = 86,
    MultiviewHigh = 118,
    StereoHigh = 128,
    MFCDepthHigh = 135,
    MultiviewDepthHigh = 138,
    EnhancedMultiviewDepthHigh = 139,
}

impl Profile {
    pub fn has_chroma_info(&self) -> bool {
        matches!(
            self,
            Profile::High
                | Profile::High10
                | Profile::High422
                | Profile::High444
                | Profile::ScalableBase
                | Profile::ScalableHigh
        )
    }
}

impl TryFrom<u32> for Profile {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or("Unknown profile.")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, FromPrimitive)]
pub enum ChromaFormat {
    Monochrome = 0,
    #[default]
    YUV420 = 1,
    YUV422 = 2,
    YUV444 = 3,
}

impl ChromaFormat {
    pub fn is_chrome_subsampled(&self) -> bool {
        matches!(self, ChromaFormat::YUV420 | ChromaFormat::YUV422)
    }
}

impl TryFrom<u32> for ChromaFormat {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or("Unknown chroma format.")
    }
}

#[derive(Clone, Debug, Default)]
pub struct DecoderContext {
    sps: Vec<sps::SequenceParameterSet>,
    pps: Vec<pps::PicParameterSet>,
}

impl DecoderContext {
    pub fn get_sps(&self, id: u8) -> Option<&sps::SequenceParameterSet> {
        self.sps.iter().find(|x| x.seq_parameter_set_id == id)
    }

    pub fn put_sps(&mut self, sps: sps::SequenceParameterSet) {
        let id = sps.seq_parameter_set_id;
        self.sps.retain(|x| x.seq_parameter_set_id != id);
        self.sps.push(sps);
    }

    pub fn get_pps(&self, id: u8) -> Option<&pps::PicParameterSet> {
        self.pps.iter().find(|x| x.seq_parameter_set_id == id)
    }

    pub fn put_pps(&mut self, pps: pps::PicParameterSet) {
        let id = pps.seq_parameter_set_id;
        self.pps.retain(|x| x.seq_parameter_set_id != id);
        self.pps.push(pps);
    }
}

#[derive(Debug, Clone)]
pub enum DecodingError {
    MisformedData(String)
}

#[derive(Debug, Default)]
pub struct Decoder {
    context: DecoderContext,
}

impl Decoder {
    pub fn new() -> Decoder {
        Decoder::default()
    }

    pub fn decode(&mut self, data: &[u8]) -> Result<(), DecodingError> {
        use nal::NalUnitType;
        let mut input = parser::BitReader::new(data);
        let parse_error_handler = |e| DecodingError::MisformedData(e);
        loop {
            println!("----------------------------------------------");
            println!(">position: {}", input.position());
            let nal = parser::parse_nal_header(&mut input).map_err(parse_error_handler)?;
            println!("NAL {:?}", nal);
            println!(">position: {}", input.position());
            match nal.nal_unit_type {
                NalUnitType::Unspecified => {

                },
                NalUnitType::SliceDataA => {

                },
                NalUnitType::SliceDataB => {

                },
                NalUnitType::SliceDataC => {

                },
                NalUnitType::NonIDRSlice |
                NalUnitType::IDRSlice => {
                    let slice = parser::parse_slice_header(&self.context, &nal, &mut input)
                                    .map_err(parse_error_handler)?;
                    println!("IDR Slice: {:?}", slice);
                },
                NalUnitType::SupplementalEnhancementInfo => {

                },
                NalUnitType::SeqParameterSet => {
                    let sps = parser::parse_sps(&mut input).map_err(parse_error_handler)?;
                    println!("SPS: {:?}", sps);
                    self.context.put_sps(sps);

                },
                NalUnitType::PicParameterSet => {
                    let pps = parser::parse_pps(&mut input).map_err(parse_error_handler)?;
                    println!("PPS: {:?}", pps);
                    self.context.put_pps(pps);

                },
                NalUnitType::AccessUnitDelimiter => {

                },
                NalUnitType::EndOfSeq => {

                },
                NalUnitType::EndOfStream => {

                },
                NalUnitType::FillerData => {

                },
                NalUnitType::SeqParameterSetExtension => {

                },
                NalUnitType::Prefix => {

                },
                NalUnitType::SubsetSeqParameterSet => {

                },
                NalUnitType::DepthParameterSet => {

                },
                NalUnitType::CodedSliceAux => {

                },
                NalUnitType::CodedSliceExtension => {

                },
                NalUnitType::CodedSliceExtensionForDepthView => {

                },
                NalUnitType::Reserved => {

                },
            }
            parser::skip_till_start_code(&mut input);
        }
        Ok(())
    }
}
