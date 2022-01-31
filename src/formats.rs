// Code encoding formats
#[derive(Copy, Clone, PartialEq)]
pub enum CodeFormat {
    AR1,
    AR2,
    ARMAX,
    CB,  		//includes user-provided BEEFC0DE CB7 encryptions
    CB7,	//Common BEEFC0DE only.
    GS3,
    GS5,
    MAXRAW,
    RAW
}

// Code "devices" to use
#[derive(Copy, Clone, PartialEq)]
pub enum CodeDevice {
    AR1,
    AR2,
    ARMAX,
    CB,
    GS3,
    STD
}

// Code with friendly name, format, and device
#[derive(Clone, PartialEq)]
pub struct CodeType {
    pub name:   &'static str,
    pub format: CodeFormat,
    pub device: CodeDevice,
}

// All supported conversion formats
pub fn get_formats() -> Vec<CodeType> {
    vec!(
        CodeType {
            name: "Raw/Unencrypted",
            format: CodeFormat::RAW,
            device: CodeDevice::STD,
        },
        CodeType {
            name: "MAXRaw/Unencrypted",
            format: CodeFormat::MAXRAW,
            device: CodeDevice::ARMAX,
        },
        CodeType {
            name: "Raw for Action Replay V1/V2",
            format: CodeFormat::RAW,
            device: CodeDevice::AR2,
        },
        CodeType {
            name: "Raw for CodeBreaker",
            format: CodeFormat::RAW,
            device: CodeDevice::CB,
        },
        CodeType {
            name: "Raw for GameShark V1/V2",
            format: CodeFormat::RAW,
            device: CodeDevice::AR2,
        },
        CodeType {
            name: "Raw for Xploder/GameShark V3",
            format: CodeFormat::RAW,
            device: CodeDevice::GS3,
        },
        CodeType {
            name: "Action Replay V1",
            format: CodeFormat::AR1,
            device: CodeDevice::AR1,
        },
        CodeType {
            name: "Action Replay V2",
            format: CodeFormat::AR2,
            device: CodeDevice::AR2,
        },
        CodeType {
            name: "Action Replay MAX",
            format: CodeFormat::ARMAX,
            device: CodeDevice::ARMAX,
        },
        CodeType {
            name: "CodeBreaker V1+ (All)",
            format: CodeFormat::CB,
            device: CodeDevice::CB,
        },
        CodeType {
            name: "CodeBreaker V7 Common",
            format: CodeFormat::CB7,
            device: CodeDevice::CB,
        },
        CodeType {
            name: "Interact GameShark V1",
            format: CodeFormat::AR1,
            device: CodeDevice::AR1,
        },
        CodeType {
            name: "Interact GameShark V2",
            format: CodeFormat::AR2,
            device: CodeDevice::AR2,
        },
        CodeType {
            name: "MadCatz GameShark V3+",
            format: CodeFormat::GS3,
            device: CodeDevice::GS3,
        },
        CodeType {
            name: "MadCatz GameShark V5+ (w/Verifier)",
            format: CodeFormat::GS5,
            device: CodeDevice::GS3,
        },
        CodeType {
            name: "Xploder V1-V3",
            format: CodeFormat::CB,
            device: CodeDevice::CB,
        },
        CodeType {
            name: "Xploder V4",
            format: CodeFormat::GS3,
            device: CodeDevice::GS3,
        },
        CodeType {
            name: "Xploder V5",
            format: CodeFormat::GS5,
            device: CodeDevice::GS3,
        },
        CodeType {
            name: "Swap Magic Coder",
            format: CodeFormat::AR1,
            device: CodeDevice::AR1,
        }
    )
}