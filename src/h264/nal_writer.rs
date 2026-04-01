use super::nal::NalHeader;
use super::rbsp_writer::{RbspWriter, WriteResult};

pub fn add_emulation_prevention(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len() + data.len() / 10);
    let mut zeros = 0;

    for &byte in data {
        if zeros == 2 && byte <= 3 {
            result.push(0x03);
            zeros = 0;
        }

        if byte == 0 {
            zeros += 1;
        } else {
            zeros = 0;
        }

        result.push(byte);
    }

    result
}

pub fn write_nal_header(header: &NalHeader, writer: &mut RbspWriter) -> WriteResult {
    writer.f(false)?; // forbidden_zero_bit
    writer.u(2, header.nal_ref_idc as u32)?;
    writer.u(5, header.nal_unit_type as u32)?;
    Ok(())
}

pub fn create_annex_b_nal_unit(header: &NalHeader, rbsp: &[u8]) -> Vec<u8> {
    let mut writer = RbspWriter::new();
    write_nal_header(header, &mut writer).unwrap();
    let mut nal_unit_data = writer.into_inner();
    nal_unit_data.extend_from_slice(rbsp);

    let payload_with_emulation = add_emulation_prevention(&nal_unit_data);

    let mut result = Vec::with_capacity(4 + payload_with_emulation.len());
    result.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);
    result.extend_from_slice(&payload_with_emulation);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_emulation_prevention() {
        let data = [0x00, 0x00, 0x00, 0x00];
        let escaped = add_emulation_prevention(&data);
        assert_eq!(escaped, vec![0x00, 0x00, 0x03, 0x00, 0x00]);

        let data = [0x00, 0x00, 0x01];
        let escaped = add_emulation_prevention(&data);
        assert_eq!(escaped, vec![0x00, 0x00, 0x03, 0x01]);

        let data = [0x00, 0x00, 0x02];
        let escaped = add_emulation_prevention(&data);
        assert_eq!(escaped, vec![0x00, 0x00, 0x03, 0x02]);

        let data = [0x00, 0x00, 0x03];
        let escaped = add_emulation_prevention(&data);
        assert_eq!(escaped, vec![0x00, 0x00, 0x03, 0x03]);

        let data = [0x00, 0x00, 0x04];
        let escaped = add_emulation_prevention(&data);
        assert_eq!(escaped, vec![0x00, 0x00, 0x04]);
    }

    #[test]
    fn test_create_annex_b_nal_unit() {
        use crate::h264::nal::NalUnitType;

        let header = NalHeader {
            nal_ref_idc: 3,
            nal_unit_type: NalUnitType::IDRSlice,
        };

        // IDRSlice is nal_unit_type = 5
        // Header byte = (0 << 7) | (3 << 5) | 5 = 0b01100101 = 0x65
        
        let rbsp = [0x00, 0x00, 0x01, 0xFF];
        // The rbsp has an emulation prevention sequence: 0x00, 0x00, 0x01
        // Emulation prevention will transform it to: 0x00, 0x00, 0x03, 0x01, 0xFF
        
        let annex_b = create_annex_b_nal_unit(&header, &rbsp);
        
        let expected = vec![
            0x00, 0x00, 0x00, 0x01, // Annex B start code
            0x65,                   // NAL Header
            0x00, 0x00, 0x03, 0x01, 0xFF // Escaped RBSP
        ];
        
        assert_eq!(annex_b, expected);
    }
}
