pub mod parser;
pub mod pps;
pub mod slice;
pub mod sps;

#[derive(Clone, Debug, Default)]
pub struct DecoderContext {
    sps: Vec<sps::SequenceParameterSet>,
    pps: Vec<pps::PicParameterSet>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
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
        match self {
            Profile::High
            | Profile::High10
            | Profile::High422
            | Profile::High444
            | Profile::ScalableBase
            | Profile::ScalableHigh => true,
            _ => false,
        }
    }
}

impl TryFrom<u32> for Profile {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            66 => Ok(Profile::Baseline),
            77 => Ok(Profile::Main),
            100 => Ok(Profile::High),
            122 => Ok(Profile::High422),
            110 => Ok(Profile::High10),
            244 => Ok(Profile::High444),
            88 => Ok(Profile::Extended),
            83 => Ok(Profile::ScalableBase),
            86 => Ok(Profile::ScalableHigh),
            118 => Ok(Profile::MultiviewHigh),
            128 => Ok(Profile::StereoHigh),
            135 => Ok(Profile::MFCDepthHigh),
            138 => Ok(Profile::MultiviewDepthHigh),
            139 => Ok(Profile::EnhancedMultiviewDepthHigh),
            _ => Err(()),
        }
    }
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
