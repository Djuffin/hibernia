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
}
