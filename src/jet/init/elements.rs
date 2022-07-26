/* This file has been automatically generated. */

use crate::jet::application::Elements;
use crate::jet::type_name::TypeName;
use crate::jet::JetNode;
use crate::merkle::cmr::Cmr;
use bitcoin_hashes::sha256::Midstate;

/// Identifiers of all possible Elements jets
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum ElementsJetName {
    // Primitives
    Version,
    LockTime,
    InputIsPegin,
    InputPrevOutpoint,
    InputAsset,
    InputAmount,
    InputScriptHash,
    InputSequence,
    InputIssuanceBlinding,
    InputIssuanceContract,
    InputIssuanceEntropy,
    InputIssuanceAssetAmount,
    InputIssuanceTokenAmount,
    OutputAsset,
    OutputAmount,
    OutputNonce,
    OutputScriptHash,
    OutputNullDatum,
    ScriptCmr,
    CurrentIndex,
    CurrentIsPegin,
    CurrentPrevOutpoint,
    CurrentAsset,
    CurrentAmount,
    CurrentScriptHash,
    CurrentSequence,
    CurrentIssuanceBlinding,
    CurrentIssuanceContract,
    CurrentIssuanceEntropy,
    CurrentIssuanceAssetAmount,
    CurrentIssuanceTokenAmount,
    InputsHash,
    OutputsHash,
    NumInputs,
    NumOutputs,
    Fee,
    // Core macros
    Add32,
    FullAdd32,
    Sub32,
    FullSub32,
    Mul32,
    FullMul32,
    Eq32Verify,
    Eq256Verify,
    Lt32Verify,
    Sha256,
    Sha256Block,
    Bip0340Verify,
}

// Primitives
pub const VERSION: JetNode<Elements> = JetNode {
    name: ElementsJetName::Version,
    cmr: Cmr(Midstate([
        0x00, 0xb1, 0x45, 0x0b, 0x95, 0x57, 0xb2, 0xf6, 0xf6, 0x81, 0x85, 0x48, 0x10, 0x49, 0x9f,
        0xd1, 0xb9, 0x2f, 0x08, 0x5a, 0x61, 0xfa, 0x44, 0x63, 0x4a, 0xd8, 0xd6, 0x4e, 0x62, 0xda,
        0xf4, 0x93,
    ])),
    source_ty: TypeName(b"1"),
    target_ty: TypeName(b"i"),
};

pub const LOCK_TIME: JetNode<Elements> = JetNode {
    name: ElementsJetName::LockTime,
    cmr: Cmr(Midstate([
        0xa6, 0x9a, 0x61, 0x3b, 0xef, 0x2c, 0x1d, 0xfa, 0xb4, 0x4e, 0x8f, 0xea, 0x19, 0x6d, 0x93,
        0x5c, 0xfc, 0x41, 0x81, 0xfb, 0x5d, 0xa4, 0x02, 0x57, 0x95, 0xf5, 0x21, 0x0e, 0x3a, 0x8d,
        0x3b, 0xd7,
    ])),
    source_ty: TypeName(b"1"),
    target_ty: TypeName(b"i"),
};

pub const INPUT_IS_PEGIN: JetNode<Elements> = JetNode {
    name: ElementsJetName::InputIsPegin,
    cmr: Cmr(Midstate([
        0x21, 0x38, 0x85, 0x64, 0xaf, 0x5c, 0x26, 0x39, 0x7e, 0xd1, 0x72, 0x69, 0xa3, 0x6d, 0xfa,
        0x3e, 0xd6, 0x9c, 0x15, 0x69, 0x6d, 0x12, 0xca, 0x25, 0xbb, 0xdd, 0xb6, 0x4b, 0xeb, 0x40,
        0xad, 0xb3,
    ])),
    source_ty: TypeName(b"i"),
    target_ty: TypeName(b"+12"),
};

pub const INPUT_PREV_OUTPOINT: JetNode<Elements> = JetNode {
    name: ElementsJetName::InputPrevOutpoint,
    cmr: Cmr(Midstate([
        0x79, 0x71, 0x9d, 0x11, 0x20, 0x5f, 0x1a, 0x1b, 0xb7, 0xbe, 0x3c, 0xe5, 0xbb, 0x2c, 0x15,
        0x5b, 0xa6, 0x3a, 0x10, 0x7e, 0x63, 0x8c, 0x56, 0xf9, 0x73, 0x9d, 0x49, 0xd6, 0xb8, 0x1d,
        0x00, 0x30,
    ])),
    source_ty: TypeName(b"i"),
    target_ty: TypeName(b"+1*hi"),
};

pub const INPUT_ASSET: JetNode<Elements> = JetNode {
    name: ElementsJetName::InputAsset,
    cmr: Cmr(Midstate([
        0x5c, 0xa6, 0x34, 0xb3, 0x63, 0x7a, 0x2a, 0x18, 0xd3, 0x45, 0x43, 0xf5, 0x99, 0x7e, 0x57,
        0xd1, 0xa5, 0x9a, 0xb8, 0xe7, 0x26, 0xfc, 0x30, 0x82, 0x08, 0x44, 0x48, 0x61, 0x48, 0xf5,
        0x11, 0x3d,
    ])),
    source_ty: TypeName(b"i"),
    target_ty: TypeName(b"+1+*2hh"),
};

pub const INPUT_AMOUNT: JetNode<Elements> = JetNode {
    name: ElementsJetName::InputAmount,
    cmr: Cmr(Midstate([
        0x20, 0x2a, 0x92, 0x5f, 0x7c, 0x80, 0x42, 0xe0, 0xa8, 0x80, 0x7a, 0x06, 0x77, 0x73, 0xf1,
        0x46, 0xa2, 0x72, 0xb5, 0xe9, 0x05, 0x7d, 0xf2, 0x27, 0x95, 0xe3, 0x65, 0x4c, 0xf2, 0x69,
        0xb5, 0xef,
    ])),
    source_ty: TypeName(b"i"),
    target_ty: TypeName(b"+1+*2hl"),
};

pub const INPUT_SCRIPT_HASH: JetNode<Elements> = JetNode {
    name: ElementsJetName::InputScriptHash,
    cmr: Cmr(Midstate([
        0xd3, 0x55, 0xb3, 0xbc, 0x46, 0xa7, 0xf2, 0x54, 0x0c, 0xc3, 0xae, 0xb0, 0x4c, 0x70, 0xd5,
        0xbd, 0x97, 0xdc, 0x0a, 0xf3, 0xb8, 0xda, 0x4e, 0xc4, 0x0f, 0xc9, 0x61, 0x43, 0xea, 0xa1,
        0xb7, 0x0b,
    ])),
    source_ty: TypeName(b"i"),
    target_ty: TypeName(b"+1h"),
};

pub const INPUT_SEQUENCE: JetNode<Elements> = JetNode {
    name: ElementsJetName::InputSequence,
    cmr: Cmr(Midstate([
        0x81, 0xba, 0x00, 0xfc, 0xaf, 0xdf, 0x68, 0x65, 0xf2, 0x7b, 0x66, 0xf3, 0xe8, 0x81, 0xa0,
        0xb6, 0x5d, 0x96, 0x3c, 0x47, 0x8f, 0xc9, 0x9a, 0xb4, 0x3e, 0xb4, 0xeb, 0xcf, 0xc0, 0x3a,
        0xd7, 0x95,
    ])),
    source_ty: TypeName(b"i"),
    target_ty: TypeName(b"+1i"),
};

pub const INPUT_ISSUANCE_BLINDING: JetNode<Elements> = JetNode {
    name: ElementsJetName::InputIssuanceBlinding,
    cmr: Cmr(Midstate([
        0x1f, 0xf0, 0xa7, 0xad, 0xd4, 0xcb, 0x4c, 0xc9, 0xcc, 0x08, 0x01, 0xdc, 0xd6, 0x7f, 0xca,
        0x38, 0xe8, 0x7a, 0x46, 0x0c, 0x76, 0xb4, 0x45, 0x22, 0x94, 0x10, 0xb7, 0x00, 0xfa, 0x48,
        0x90, 0xef,
    ])),
    source_ty: TypeName(b"i"),
    target_ty: TypeName(b"+1+1h"),
};

pub const INPUT_ISSUANCE_CONTRACT: JetNode<Elements> = JetNode {
    name: ElementsJetName::InputIssuanceContract,
    cmr: Cmr(Midstate([
        0x6e, 0x95, 0x05, 0x82, 0x40, 0xf5, 0xbb, 0xb6, 0x2f, 0x41, 0xae, 0xf9, 0x4c, 0xf6, 0xc9,
        0x5e, 0x30, 0x83, 0x3e, 0x98, 0x93, 0x7a, 0xa8, 0x57, 0x1e, 0x11, 0x92, 0xed, 0xf2, 0xca,
        0xad, 0xff,
    ])),
    source_ty: TypeName(b"i"),
    target_ty: TypeName(b"+1+1h"),
};

pub const INPUT_ISSUANCE_ENTROPY: JetNode<Elements> = JetNode {
    name: ElementsJetName::InputIssuanceEntropy,
    cmr: Cmr(Midstate([
        0x2b, 0xcc, 0x0f, 0x48, 0x23, 0x79, 0x32, 0x9a, 0x8e, 0x39, 0x3b, 0x48, 0x5a, 0x7e, 0xcf,
        0xd5, 0xdd, 0x7d, 0x8b, 0x8d, 0x03, 0x1d, 0x94, 0xd9, 0xa3, 0x3e, 0x80, 0x08, 0x4f, 0x70,
        0x6f, 0xa1,
    ])),
    source_ty: TypeName(b"i"),
    target_ty: TypeName(b"+1+1h"),
};

pub const INPUT_ISSUANCE_ASSET_AMOUNT: JetNode<Elements> = JetNode {
    name: ElementsJetName::InputIssuanceAssetAmount,
    cmr: Cmr(Midstate([
        0x48, 0x3e, 0x09, 0xfc, 0xe0, 0x7d, 0x20, 0xa6, 0x41, 0x25, 0x97, 0x9d, 0xf3, 0x63, 0x9f,
        0x12, 0x54, 0x60, 0x1f, 0x5a, 0x00, 0x80, 0x2c, 0x1a, 0x49, 0xbd, 0x44, 0xbc, 0x95, 0xed,
        0x70, 0xd5,
    ])),
    source_ty: TypeName(b"i"),
    target_ty: TypeName(b"+1+1+*2hl"),
};

pub const INPUT_ISSUANCE_TOKEN_AMOUNT: JetNode<Elements> = JetNode {
    name: ElementsJetName::InputIssuanceTokenAmount,
    cmr: Cmr(Midstate([
        0xe9, 0x11, 0xef, 0x5a, 0xfa, 0x09, 0xc3, 0x9e, 0x0b, 0x25, 0x61, 0xb0, 0xaf, 0x91, 0xdc,
        0x4e, 0xc6, 0xee, 0x2e, 0xc4, 0x94, 0x1d, 0x76, 0x20, 0x60, 0xfa, 0x04, 0x29, 0x81, 0x14,
        0xd5, 0x52,
    ])),
    source_ty: TypeName(b"i"),
    target_ty: TypeName(b"+1+1+*2hl"),
};

pub const OUTPUT_ASSET: JetNode<Elements> = JetNode {
    name: ElementsJetName::OutputAsset,
    cmr: Cmr(Midstate([
        0xb6, 0x95, 0xfb, 0x06, 0x5e, 0xbf, 0x85, 0xb6, 0xc7, 0x6d, 0x05, 0xd1, 0xd4, 0xc7, 0x82,
        0x68, 0x5b, 0x09, 0x4e, 0x22, 0xa9, 0x4f, 0x23, 0x38, 0xdf, 0xad, 0xf6, 0xc4, 0xbc, 0x98,
        0xa9, 0xb1,
    ])),
    source_ty: TypeName(b"i"),
    target_ty: TypeName(b"+1+*2hh"),
};

pub const OUTPUT_AMOUNT: JetNode<Elements> = JetNode {
    name: ElementsJetName::OutputAmount,
    cmr: Cmr(Midstate([
        0x73, 0xad, 0xf1, 0xd9, 0x97, 0xbd, 0x82, 0x00, 0x31, 0x05, 0xaf, 0x41, 0x5a, 0x17, 0x71,
        0x90, 0x35, 0xa5, 0x77, 0x42, 0xde, 0x33, 0x93, 0xae, 0x40, 0x82, 0x22, 0x0b, 0xc5, 0x47,
        0xa5, 0x9f,
    ])),
    source_ty: TypeName(b"i"),
    target_ty: TypeName(b"+1+*2hl"),
};

pub const OUTPUT_NONCE: JetNode<Elements> = JetNode {
    name: ElementsJetName::OutputNonce,
    cmr: Cmr(Midstate([
        0x7a, 0xfb, 0xe9, 0xff, 0x18, 0x85, 0xd9, 0x37, 0x20, 0x71, 0x0a, 0x53, 0x1e, 0x8c, 0x96,
        0xab, 0xbf, 0x26, 0x6a, 0x9c, 0xef, 0x27, 0x0d, 0xe5, 0xea, 0xb8, 0xb9, 0xbc, 0xe6, 0x82,
        0x91, 0xf8,
    ])),
    source_ty: TypeName(b"i"),
    target_ty: TypeName(b"+1+*2hh"),
};

pub const OUTPUT_SCRIPT_HASH: JetNode<Elements> = JetNode {
    name: ElementsJetName::OutputScriptHash,
    cmr: Cmr(Midstate([
        0xe5, 0x0b, 0x2b, 0xc2, 0x73, 0x9f, 0xea, 0xe2, 0xea, 0xeb, 0x55, 0x2a, 0x9b, 0x34, 0xfa,
        0x7c, 0x71, 0x49, 0xe0, 0xc8, 0x0d, 0xe0, 0xf0, 0x3c, 0xed, 0xf3, 0x62, 0xbb, 0xa2, 0xa6,
        0x76, 0xf0,
    ])),
    source_ty: TypeName(b"i"),
    target_ty: TypeName(b"+1h"),
};

pub const OUTPUT_NULL_DATUM: JetNode<Elements> = JetNode {
    name: ElementsJetName::OutputNullDatum,
    cmr: Cmr(Midstate([
        0xe2, 0x1a, 0xe0, 0x15, 0x3f, 0x87, 0x87, 0x7f, 0x45, 0xf4, 0x6a, 0x9f, 0xec, 0x22, 0x84,
        0x24, 0x45, 0xd0, 0x82, 0xe2, 0xff, 0x96, 0x12, 0xbd, 0xdb, 0xed, 0xf9, 0x7d, 0x1c, 0x38,
        0x3f, 0x5d,
    ])),
    source_ty: TypeName(b"*ii"),
    target_ty: TypeName(b"+1+1+**22h+2*22"),
};

pub const SCRIPT_CMR: JetNode<Elements> = JetNode {
    name: ElementsJetName::ScriptCmr,
    cmr: Cmr(Midstate([
        0x9d, 0x16, 0x4f, 0xfd, 0x73, 0xec, 0x16, 0x87, 0xbf, 0x38, 0xa3, 0xf2, 0x2e, 0xfa, 0xad,
        0x10, 0x5e, 0x45, 0xa4, 0x93, 0xd3, 0xb7, 0xfb, 0xf4, 0x3a, 0x09, 0x4d, 0xf0, 0xf5, 0x0a,
        0x29, 0x22,
    ])),
    source_ty: TypeName(b"1"),
    target_ty: TypeName(b"h"),
};

pub const CURRENT_INDEX: JetNode<Elements> = JetNode {
    name: ElementsJetName::CurrentIndex,
    cmr: Cmr(Midstate([
        0xfb, 0xaf, 0xe4, 0xb0, 0xa5, 0x8a, 0x12, 0x76, 0xca, 0x02, 0x3c, 0x33, 0x39, 0x15, 0x1c,
        0xf3, 0x50, 0x12, 0x74, 0x72, 0xd8, 0x18, 0xe6, 0x44, 0x77, 0x47, 0x23, 0xcd, 0xf5, 0x34,
        0x3a, 0x81,
    ])),
    source_ty: TypeName(b"1"),
    target_ty: TypeName(b"i"),
};

pub const CURRENT_IS_PEGIN: JetNode<Elements> = JetNode {
    name: ElementsJetName::CurrentIsPegin,
    cmr: Cmr(Midstate([
        0xd2, 0x5d, 0xbe, 0xef, 0x34, 0xed, 0xf5, 0xbc, 0xd2, 0x37, 0x97, 0x86, 0x06, 0x66, 0xea,
        0x83, 0x96, 0xaa, 0xbc, 0xe4, 0xa4, 0x02, 0x44, 0x76, 0xbb, 0x9d, 0x63, 0x37, 0xad, 0xc1,
        0x23, 0x7d,
    ])),
    source_ty: TypeName(b"1"),
    target_ty: TypeName(b"2"),
};

pub const CURRENT_PREV_OUTPOINT: JetNode<Elements> = JetNode {
    name: ElementsJetName::CurrentPrevOutpoint,
    cmr: Cmr(Midstate([
        0x2c, 0x7e, 0x49, 0x96, 0x09, 0x14, 0xcb, 0x18, 0x82, 0x46, 0x87, 0xab, 0xc3, 0x16, 0x5c,
        0x16, 0xeb, 0x82, 0xfb, 0x1e, 0x8c, 0xdc, 0x3e, 0x15, 0xf5, 0xc9, 0x6c, 0x0d, 0x39, 0xb8,
        0x26, 0x32,
    ])),
    source_ty: TypeName(b"1"),
    target_ty: TypeName(b"*hi"),
};

pub const CURRENT_ASSET: JetNode<Elements> = JetNode {
    name: ElementsJetName::CurrentAsset,
    cmr: Cmr(Midstate([
        0x91, 0x2b, 0xf8, 0x78, 0x9d, 0xd2, 0x81, 0x8f, 0x97, 0x58, 0xca, 0xc4, 0x2d, 0xd6, 0xa1,
        0x06, 0x5b, 0x9d, 0x4e, 0x28, 0xc3, 0x84, 0x65, 0x8f, 0xc9, 0x67, 0xe4, 0x39, 0x49, 0xaa,
        0xa7, 0x9e,
    ])),
    source_ty: TypeName(b"1"),
    target_ty: TypeName(b"+*2hh"),
};

pub const CURRENT_AMOUNT: JetNode<Elements> = JetNode {
    name: ElementsJetName::CurrentAmount,
    cmr: Cmr(Midstate([
        0x35, 0x23, 0xb1, 0xfc, 0x22, 0xdb, 0xbc, 0x9e, 0xaa, 0xc2, 0x6b, 0x9e, 0xea, 0x23, 0xa8,
        0xcd, 0x85, 0xb3, 0xbf, 0x04, 0x56, 0x0a, 0xe0, 0x09, 0xb9, 0x77, 0x72, 0x7c, 0xc4, 0x7d,
        0x2f, 0x2c,
    ])),
    source_ty: TypeName(b"1"),
    target_ty: TypeName(b"+*2hl"),
};

pub const CURRENT_SCRIPT_HASH: JetNode<Elements> = JetNode {
    name: ElementsJetName::CurrentScriptHash,
    cmr: Cmr(Midstate([
        0x8c, 0x8e, 0xbb, 0xe2, 0x01, 0x8f, 0xf2, 0x06, 0x6a, 0x99, 0x89, 0xe8, 0xfc, 0x5b, 0x4a,
        0xd7, 0x28, 0xff, 0x0f, 0x40, 0x2a, 0x7d, 0x81, 0xa2, 0x70, 0x94, 0x60, 0xf0, 0x8f, 0xde,
        0x3d, 0x8b,
    ])),
    source_ty: TypeName(b"1"),
    target_ty: TypeName(b"h"),
};

pub const CURRENT_SEQUENCE: JetNode<Elements> = JetNode {
    name: ElementsJetName::CurrentSequence,
    cmr: Cmr(Midstate([
        0xbc, 0x4b, 0xed, 0xce, 0x1b, 0xc2, 0xd5, 0x94, 0xbc, 0xc5, 0xd8, 0x4f, 0x61, 0xda, 0xf3,
        0x0a, 0x36, 0x1e, 0x50, 0xe3, 0xb4, 0x58, 0x7c, 0x95, 0xbb, 0x88, 0x60, 0xa8, 0x8b, 0x85,
        0xd4, 0xd1,
    ])),
    source_ty: TypeName(b"1"),
    target_ty: TypeName(b"i"),
};

pub const CURRENT_ISSUANCE_BLINDING: JetNode<Elements> = JetNode {
    name: ElementsJetName::CurrentIssuanceBlinding,
    cmr: Cmr(Midstate([
        0x72, 0x70, 0xc7, 0x46, 0xce, 0x6e, 0xbd, 0xe4, 0x4c, 0x63, 0xa6, 0x08, 0x6c, 0xc6, 0x61,
        0x23, 0x75, 0xb5, 0xa9, 0xf8, 0x20, 0xf6, 0xc5, 0xb1, 0x59, 0xd2, 0xfa, 0x60, 0xec, 0xe9,
        0x71, 0xd7,
    ])),
    source_ty: TypeName(b"1"),
    target_ty: TypeName(b"+1h"),
};

pub const CURRENT_ISSUANCE_CONTRACT: JetNode<Elements> = JetNode {
    name: ElementsJetName::CurrentIssuanceContract,
    cmr: Cmr(Midstate([
        0x66, 0x75, 0x91, 0xb3, 0xdd, 0xad, 0xac, 0xa9, 0xff, 0x85, 0x96, 0xdd, 0xbe, 0x4d, 0xfd,
        0xa3, 0x78, 0xdf, 0x1a, 0xba, 0x7a, 0x9e, 0xe9, 0xf9, 0xee, 0x3d, 0xa2, 0x8d, 0x7c, 0x01,
        0x1c, 0x94,
    ])),
    source_ty: TypeName(b"1"),
    target_ty: TypeName(b"+1h"),
};

pub const CURRENT_ISSUANCE_ENTROPY: JetNode<Elements> = JetNode {
    name: ElementsJetName::CurrentIssuanceEntropy,
    cmr: Cmr(Midstate([
        0xfa, 0x0c, 0xf7, 0x68, 0xdf, 0x18, 0x4e, 0xc9, 0x6d, 0x18, 0x13, 0x37, 0x7e, 0xbb, 0x3d,
        0xdc, 0xd1, 0x3d, 0x8a, 0x63, 0xe4, 0x8d, 0x05, 0xde, 0x53, 0xd2, 0xd6, 0x91, 0x6d, 0x19,
        0xd7, 0xe1,
    ])),
    source_ty: TypeName(b"1"),
    target_ty: TypeName(b"+1h"),
};

pub const CURRENT_ISSUANCE_ASSET_AMOUNT: JetNode<Elements> = JetNode {
    name: ElementsJetName::CurrentIssuanceAssetAmount,
    cmr: Cmr(Midstate([
        0x6d, 0x52, 0x0a, 0xb0, 0xa5, 0x77, 0xb4, 0x20, 0x0c, 0xf1, 0x05, 0xff, 0x3f, 0x52, 0xc4,
        0x76, 0x8e, 0x7d, 0x00, 0xc5, 0x3b, 0x6b, 0xe4, 0xb7, 0xea, 0x42, 0xca, 0x85, 0xf5, 0x99,
        0x81, 0x96,
    ])),
    source_ty: TypeName(b"1"),
    target_ty: TypeName(b"+1+*2hl"),
};

pub const CURRENT_ISSUANCE_TOKEN_AMOUNT: JetNode<Elements> = JetNode {
    name: ElementsJetName::CurrentIssuanceTokenAmount,
    cmr: Cmr(Midstate([
        0x82, 0xc0, 0x18, 0x96, 0x69, 0xa2, 0x11, 0x56, 0x92, 0x41, 0xc0, 0x0c, 0xc4, 0x6f, 0x34,
        0x7b, 0x21, 0x9b, 0x04, 0xda, 0x71, 0x28, 0x0e, 0x54, 0x6b, 0x1d, 0x01, 0xae, 0xd5, 0xcd,
        0xa9, 0xf2,
    ])),
    source_ty: TypeName(b"1"),
    target_ty: TypeName(b"+1+*2hl"),
};

pub const INPUTS_HASH: JetNode<Elements> = JetNode {
    name: ElementsJetName::InputsHash,
    cmr: Cmr(Midstate([
        0x29, 0xbb, 0xe5, 0x6c, 0xec, 0xcf, 0x69, 0x46, 0xe0, 0xe1, 0xc7, 0x45, 0x8e, 0xbf, 0xdb,
        0x4c, 0xca, 0x34, 0x02, 0x64, 0xc4, 0x05, 0x81, 0x82, 0x6c, 0x26, 0x7e, 0x54, 0xca, 0xec,
        0x5d, 0xe2,
    ])),
    source_ty: TypeName(b"1"),
    target_ty: TypeName(b"h"),
};

pub const OUTPUTS_HASH: JetNode<Elements> = JetNode {
    name: ElementsJetName::OutputsHash,
    cmr: Cmr(Midstate([
        0xc8, 0xaa, 0xe4, 0x68, 0xd7, 0xcb, 0x1d, 0x66, 0xdf, 0x1b, 0x37, 0xa7, 0x78, 0xe6, 0x67,
        0x34, 0xd2, 0x52, 0x56, 0x70, 0xaf, 0x6f, 0xf6, 0xe5, 0x59, 0xca, 0xd4, 0xdf, 0x83, 0x03,
        0x2a, 0xe5,
    ])),
    source_ty: TypeName(b"1"),
    target_ty: TypeName(b"h"),
};

pub const NUM_INPUTS: JetNode<Elements> = JetNode {
    name: ElementsJetName::NumInputs,
    cmr: Cmr(Midstate([
        0x25, 0xe9, 0x21, 0x33, 0xe6, 0xad, 0xf2, 0xa4, 0x4c, 0xf6, 0x62, 0x2b, 0x3a, 0x79, 0x8d,
        0xf5, 0xfe, 0x73, 0xb1, 0x7c, 0xba, 0xf0, 0x75, 0x93, 0xfd, 0x49, 0x06, 0x4f, 0x4e, 0x91,
        0xbc, 0x18,
    ])),
    source_ty: TypeName(b"1"),
    target_ty: TypeName(b"i"),
};

pub const NUM_OUTPUTS: JetNode<Elements> = JetNode {
    name: ElementsJetName::NumOutputs,
    cmr: Cmr(Midstate([
        0xcd, 0x39, 0x0b, 0x66, 0x79, 0xdc, 0x17, 0x9c, 0x46, 0xb5, 0x40, 0xa7, 0x90, 0x5f, 0x9e,
        0xa7, 0xff, 0x55, 0x23, 0xf9, 0xff, 0xa0, 0x0a, 0xb9, 0x05, 0xcd, 0xe8, 0x24, 0xde, 0xf9,
        0x75, 0x00,
    ])),
    source_ty: TypeName(b"1"),
    target_ty: TypeName(b"i"),
};

pub const FEE: JetNode<Elements> = JetNode {
    name: ElementsJetName::Fee,
    cmr: Cmr(Midstate([
        0xe2, 0x25, 0xdb, 0x38, 0x12, 0xe8, 0xd5, 0xd3, 0xe6, 0x07, 0xa2, 0x17, 0x79, 0xb9, 0x32,
        0xc1, 0x40, 0x52, 0x98, 0xa3, 0x12, 0xa3, 0x47, 0x01, 0xe3, 0xb0, 0xf7, 0x1c, 0xd4, 0xc9,
        0x8c, 0xbc,
    ])),
    source_ty: TypeName(b"h"),
    target_ty: TypeName(b"l"),
};

// Core macros
pub const ADD32: JetNode<Elements> = JetNode {
    name: ElementsJetName::Add32,
    cmr: Cmr(Midstate([
        0x5d, 0x5c, 0x8f, 0xf3, 0x86, 0xd5, 0xa0, 0x14, 0x08, 0xe9, 0xe0, 0x79, 0xed, 0x95, 0x2c,
        0xb9, 0xc1, 0xdc, 0x86, 0x14, 0xfc, 0x1f, 0x3e, 0x54, 0x61, 0xab, 0x1c, 0x30, 0x24, 0xdc,
        0xea, 0x54,
    ])),
    source_ty: TypeName(b"l"),
    target_ty: TypeName(b"*2i"),
};

pub const FULL_ADD32: JetNode<Elements> = JetNode {
    name: ElementsJetName::FullAdd32,
    cmr: Cmr(Midstate([
        0xf0, 0x95, 0x9d, 0x3c, 0xb9, 0x2c, 0x72, 0x8c, 0xd0, 0x86, 0x26, 0x81, 0x71, 0xaa, 0x1f,
        0xdd, 0x5c, 0x97, 0x4c, 0xbe, 0x3f, 0xf6, 0x4a, 0x09, 0x94, 0x13, 0x28, 0x76, 0x6d, 0x24,
        0xbf, 0xf1,
    ])),
    source_ty: TypeName(b"*l2"),
    target_ty: TypeName(b"*2i"),
};

pub const SUB32: JetNode<Elements> = JetNode {
    name: ElementsJetName::Sub32,
    cmr: Cmr(Midstate([
        0x01, 0x6d, 0x32, 0x48, 0xee, 0x72, 0x7e, 0xb7, 0x27, 0xc3, 0x3a, 0xa6, 0xf2, 0xcf, 0xb8,
        0xb8, 0x7e, 0x7d, 0x07, 0x46, 0x55, 0x40, 0xdc, 0x3f, 0x9a, 0xb3, 0x22, 0x93, 0x78, 0x85,
        0x2a, 0xc7,
    ])),
    source_ty: TypeName(b"l"),
    target_ty: TypeName(b"*2i"),
};

pub const FULL_SUB32: JetNode<Elements> = JetNode {
    name: ElementsJetName::FullSub32,
    cmr: Cmr(Midstate([
        0x34, 0x73, 0xfa, 0x10, 0xe0, 0xe7, 0xd9, 0x80, 0x2d, 0x53, 0x3b, 0x13, 0x01, 0xb2, 0x09,
        0x83, 0x85, 0x92, 0xb3, 0x1a, 0xf9, 0xd9, 0x14, 0xb0, 0xe7, 0x46, 0x11, 0x32, 0xf5, 0x3d,
        0x79, 0x7a,
    ])),
    source_ty: TypeName(b"*l2"),
    target_ty: TypeName(b"*2i"),
};

pub const MUL32: JetNode<Elements> = JetNode {
    name: ElementsJetName::Mul32,
    cmr: Cmr(Midstate([
        0x02, 0x44, 0x52, 0xa5, 0x7a, 0xc5, 0x8c, 0xd0, 0xa1, 0x97, 0x57, 0xbb, 0xf1, 0x68, 0xa3,
        0xa8, 0xcb, 0x6a, 0x02, 0x38, 0xa8, 0x0f, 0x61, 0x81, 0x3e, 0xf7, 0x9c, 0x92, 0x6c, 0x8f,
        0x08, 0x9e,
    ])),
    source_ty: TypeName(b"l"),
    target_ty: TypeName(b"l"),
};

pub const FULL_MUL32: JetNode<Elements> = JetNode {
    name: ElementsJetName::FullMul32,
    cmr: Cmr(Midstate([
        0x47, 0xe0, 0xca, 0x35, 0x3a, 0x6f, 0x93, 0x4b, 0xd9, 0x97, 0x5d, 0xfe, 0x04, 0x27, 0x62,
        0x96, 0x42, 0x94, 0xf7, 0x51, 0xd1, 0xd4, 0x6d, 0x39, 0xcf, 0xa5, 0xee, 0x5f, 0x3a, 0x37,
        0x8b, 0xfd,
    ])),
    source_ty: TypeName(b"*ll"),
    target_ty: TypeName(b"l"),
};

pub const EQ32_VERIFY: JetNode<Elements> = JetNode {
    name: ElementsJetName::Eq32Verify,
    cmr: Cmr(Midstate([
        0x32, 0x61, 0x8d, 0x01, 0xfb, 0xfe, 0x81, 0x9f, 0x29, 0x69, 0xb7, 0x1c, 0xda, 0xbf, 0x40,
        0x5d, 0xde, 0x3d, 0xa1, 0x7c, 0x04, 0x45, 0xe8, 0xd0, 0x53, 0x47, 0x65, 0x7c, 0x5b, 0x53,
        0x2f, 0x72,
    ])),
    source_ty: TypeName(b"l"),
    target_ty: TypeName(b"1"),
};

pub const EQ256_VERIFY: JetNode<Elements> = JetNode {
    name: ElementsJetName::Eq256Verify,
    cmr: Cmr(Midstate([
        0x7c, 0x1d, 0x68, 0x82, 0xe5, 0x38, 0x22, 0xe8, 0x0c, 0x5d, 0x7d, 0x36, 0xf8, 0x59, 0xc1,
        0xc4, 0x02, 0xfe, 0x29, 0x10, 0xcf, 0xbc, 0xa2, 0x32, 0xc0, 0x67, 0x97, 0x25, 0x6b, 0xe3,
        0xdb, 0x07,
    ])),
    source_ty: TypeName(b"*hh"),
    target_ty: TypeName(b"1"),
};

pub const LT32_VERIFY: JetNode<Elements> = JetNode {
    name: ElementsJetName::Lt32Verify,
    cmr: Cmr(Midstate([
        0xa1, 0xfa, 0x71, 0x96, 0xbe, 0x58, 0x35, 0x47, 0xcf, 0xb5, 0x15, 0x25, 0xc7, 0x65, 0x2f,
        0xc2, 0x14, 0x0c, 0x70, 0x46, 0xab, 0xab, 0x4a, 0x8c, 0x3a, 0x25, 0x1f, 0x1e, 0xa3, 0xc3,
        0x94, 0xc1,
    ])),
    source_ty: TypeName(b"l"),
    target_ty: TypeName(b"1"),
};

pub const SHA256: JetNode<Elements> = JetNode {
    name: ElementsJetName::Sha256,
    cmr: Cmr(Midstate([
        0xd6, 0x49, 0xd3, 0x03, 0xd1, 0x96, 0xcd, 0x53, 0xfe, 0x29, 0x86, 0xfc, 0x6b, 0x81, 0x25,
        0x08, 0xb5, 0x5d, 0x23, 0xaa, 0xa3, 0x92, 0xf4, 0xf3, 0xa7, 0xd0, 0x8c, 0x6c, 0xad, 0xb2,
        0xf8, 0xac,
    ])),
    source_ty: TypeName(b"*hh"),
    target_ty: TypeName(b"h"),
};

pub const SHA256_BLOCK: JetNode<Elements> = JetNode {
    name: ElementsJetName::Sha256Block,
    cmr: Cmr(Midstate([
        0xd5, 0xb6, 0xf8, 0x48, 0x44, 0x17, 0x32, 0x12, 0xe2, 0x69, 0x9e, 0x99, 0xa8, 0x9b, 0xcd,
        0x3e, 0xb7, 0xf8, 0xe9, 0x6c, 0x0c, 0xc8, 0x46, 0x7f, 0x1c, 0xf2, 0xc2, 0x50, 0x14, 0x74,
        0x59, 0x48,
    ])),
    source_ty: TypeName(b"*h*hh"),
    target_ty: TypeName(b"h"),
};

pub const BIP_0340_VERIFY: JetNode<Elements> = JetNode {
    name: ElementsJetName::Bip0340Verify,
    cmr: Cmr(Midstate([
        0xd6, 0xdf, 0x54, 0x94, 0x7e, 0xb9, 0x27, 0x86, 0xb3, 0x79, 0xd3, 0xec, 0x81, 0x93, 0x99,
        0x4a, 0x0f, 0xfe, 0xdd, 0xc2, 0x86, 0x46, 0xa9, 0x21, 0x4e, 0xea, 0xc8, 0xf9, 0x34, 0x1f,
        0x56, 0x4e,
    ])),
    source_ty: TypeName(b"*h*hh"),
    target_ty: TypeName(b"1"),
};