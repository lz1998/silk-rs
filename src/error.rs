use thiserror::Error;

#[derive(Error, Debug)]
pub enum SilkError {
    #[error("INVALID")]
    INVALID,
    #[error("ENC_INPUT_INVALID_NO_OF_SAMPLES")]
    ENC_INPUT_INVALID_NO_OF_SAMPLES,
    #[error("ENC_FS_NOT_SUPPORTED")]
    ENC_FS_NOT_SUPPORTED,
    #[error("ENC_PACKET_SIZE_NOT_SUPPORTED")]
    ENC_PACKET_SIZE_NOT_SUPPORTED,
    #[error("ENC_PAYLOAD_BUF_TOO_SHORT")]
    ENC_PAYLOAD_BUF_TOO_SHORT,
    #[error("ENC_INVALID_LOSS_RATE")]
    ENC_INVALID_LOSS_RATE,
    #[error("ENC_INVALID_COMPLEXITY_SETTING")]
    ENC_INVALID_COMPLEXITY_SETTING,
    #[error("ENC_INVALID_INBAND_FEC_SETTING")]
    ENC_INVALID_INBAND_FEC_SETTING,
    #[error("ENC_INVALID_DTX_SETTING")]
    ENC_INVALID_DTX_SETTING,
    #[error("ENC_INTERNAL_ERROR")]
    ENC_INTERNAL_ERROR,
    #[error("DEC_INVALID_SAMPLING_FREQUENCY")]
    DEC_INVALID_SAMPLING_FREQUENCY,
    #[error("DEC_PAYLOAD_TOO_LARGE")]
    DEC_PAYLOAD_TOO_LARGE,
    #[error("DEC_PAYLOAD_ERROR")]
    DEC_PAYLOAD_ERROR,
    #[error("OTHER {0}")]
    Other(i32),
}

impl From<i32> for SilkError {
    fn from(code: i32) -> Self {
        match code {
            -1 => Self::ENC_INPUT_INVALID_NO_OF_SAMPLES,
            -2 => Self::ENC_FS_NOT_SUPPORTED,
            -3 => Self::ENC_PACKET_SIZE_NOT_SUPPORTED,
            -4 => Self::ENC_PAYLOAD_BUF_TOO_SHORT,
            -5 => Self::ENC_INVALID_LOSS_RATE,
            -6 => Self::ENC_INVALID_COMPLEXITY_SETTING,
            -7 => Self::ENC_INVALID_INBAND_FEC_SETTING,
            -8 => Self::ENC_INVALID_DTX_SETTING,
            -9 => Self::ENC_INTERNAL_ERROR,
            -10 => Self::DEC_INVALID_SAMPLING_FREQUENCY,
            -11 => Self::DEC_PAYLOAD_TOO_LARGE,
            -12 => Self::DEC_PAYLOAD_ERROR,
            _ => Self::Other(code),
        }
    }
}
