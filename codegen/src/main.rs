#![allow(non_snake_case)]

#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

use serde_xml_rs::deserialize;

// From UAX42 Revision 23 https://www.unicode.org/reports/tr42/tr42-23.html

#[derive(Debug, Deserialize)]
#[serde(rename = "ucd", rename_all = "kebab-case")]
struct UCD {
    description: String,
    repertoire: Repertoire,
    blocks: Blocks,
}

#[derive(Debug, Deserialize)]
struct Repertoire {
    #[serde(rename = "$value")]
    chars: Vec<Codepoints>,
}

// skipped name-alias

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Codepoints {
    Char {
        cp: Option<String>,

        #[serde(rename = "first-cp")]
        first_cp: Option<String>,
        #[serde(rename = "last-cp")]
        last_cp: Option<String>,

        na: String,
        na1: String,
        age: String,
        blk: String,
        gc: String,
        ccc: String,
        bc: String,
        Bidi_M: String,
        bmg: String,
        Bidi_C: String,
        bpt: String,
        bpb: String,
        dt: String,
        dm: String,
        CE: String,
        Comp_Ex: String,
        NFC_QC: String,
        NFD_QC: String,
        NFKC_QC: String,
        NFKD_QC: String,
        XO_NFC: String,
        XO_NFD: String,
        XO_NFKC: String,
        XO_NFKD: String,
        FC_NFKC: String,
        nt: String,
        nv: String,
        jt: String,
        jg: String,
        Join_C: String,
        lb: String,
        ea: String,
        Upper: String,
        Lower: String,
        OUpper: String,
        OLower: String,
        suc: String,
        slc: String,
        stc: String,
        uc: String,
        lc: String,
        tc: String,
        scf: String,
        cf: String,
        CI: String,
        Cased: String,
        CWCF: String,
        CWCM: String,
        CWL: String,
        CWKCF: String,
        CWT: String,
        CWU: String,
        NFKC_CF: String,
        sc: String,
        scx: String,
        isc: String,
        hst: String,
        JSN: String,
        InSC: String,
        InMC: Option<String>,
        InPC: String,
        IDS: String,
        OIDS: String,
        XIDS: String,
        IDC: String,
        OIDC: String,
        XIDC: String,
        Pat_Syn: String,
        Pat_WS: String,
        Dash: String,
        Hyphen: String,
        QMark: String,
        Term: String,
        STerm: String,
        Dia: String,
        Ext: String,
        PCM: String,
        SD: String,
        Alpha: String,
        OAlpha: String,
        Math: String,
        OMath: String,
        Hex: String,
        AHex: String,
        DI: String,
        ODI: String,
        LOE: String,
        WSpace: String,
        vo: String,
        RI: String,
        Gr_Base: String,
        Gr_Ext: String,
        OGr_Ext: String,
        Gr_Link: String,
        GCB: String,
        WB: String,
        SB: String,
        Ideo: String,
        UIdeo: String,
        EqUIdeo: Option<String>,
        IDSB: String,
        IDST: String,
        Radical: String,
        Dep: String,
        VS: String,
        NChar: String,
        KAccountingNumeric: Option<String>,
        KAlternateHaYu: Option<String>,
        kAlternateJEF: Option<String>,
        kAlternateKangXi: Option<String>,
        kAlternateMorohashi: Option<String>,
        kBigFive: Option<String>,
        kCCCII: Option<String>,
        kCNS1986: Option<String>,
        kCNS1992: Option<String>,
        kCangjie: Option<String>,
        kCantonese: Option<String>,
        kCheungBauer: Option<String>,
        kCheungBauerIndex: Option<String>,
        kCihaiT: Option<String>,
        kCompatibilityVariant: Option<String>,
        kCowles: Option<String>,
        kDaeJaweon: Option<String>,
        kDefinition: Option<String>,
        kEACC: Option<String>,
        kFenn: Option<String>,
        kFennIndex: Option<String>,
        kFourCornerCode: Option<String>,
        kFrequency: Option<String>,
        kGB0: Option<String>,
        kGB1: Option<String>,
        kGB3: Option<String>,
        kGB5: Option<String>,
        kGB7: Option<String>,
        kGB8: Option<String>,
        kGradeLevel: Option<String>,
        kGSR: Option<String>,
        kHangul: Option<String>,
        kHanYu: Option<String>,
        kHanyuPinlu: Option<String>,
        kHanyuPinyin: Option<String>,
        kHDZRadBreak: Option<String>,
        kHKGlyph: Option<String>,
        kHKSCS: Option<String>,
        kIBMJapan: Option<String>,
        kIICore: Option<String>,
        kIRGDaeJaweon: Option<String>,
        kIRGDaiKanwaZiten: Option<String>,
        kIRGHanyuDaZidian: Option<String>,
        kIRGKangXi: Option<String>,
        kIRG_GSource: Option<String>,
        kIRG_HSource: Option<String>,
        kIRG_JSource: Option<String>,
        kIRG_KPSource: Option<String>,
        kIRG_KSource: Option<String>,
        kIRG_MSource: Option<String>,
        kIRG_TSource: Option<String>,
        kIRG_USource: Option<String>,
        kIRG_VSource: Option<String>,
        kJa: Option<String>,
        kJHJ: Option<String>,
        kJinmeiyoKanji: Option<String>,
        kJoyoKanji: Option<String>,
        kKoreanEducationHanja: Option<String>,
        kKoreanName: Option<String>,
        kTGH: Option<String>,
        kJIS0213: Option<String>,
        kJapaneseKun: Option<String>,
        kJapaneseOn: Option<String>,
        kJis0: Option<String>,
        kJis1: Option<String>,
        kKPS0: Option<String>,
        kKPS1: Option<String>,
        kKSC0: Option<String>,
        kKSC1: Option<String>,
        kKangXi: Option<String>,
        kKarlgren: Option<String>,
        kKorean: Option<String>,
        kLau: Option<String>,
        kMainlandTelegraph: Option<String>,
        kMandarin: Option<String>,
        kMatthews: Option<String>,
        kMeyerWempe: Option<String>,
        kMorohashi: Option<String>,
        kNelson: Option<String>,
        kOtherNumeric: Option<String>,
        kPhonetic: Option<String>,
        kPrimaryNumeric: Option<String>,
        kPseudoGB1: Option<String>,
        kRSAdobe_Japan1_6: Option<String>,
        kRSJapanese: Option<String>,
        kRSKanWa: Option<String>,
        kRSKangXi: Option<String>,
        kRSKorean: Option<String>,
        kRSMerged: Option<String>,
        kRSUnicode: Option<String>,
        kSBGY: Option<String>,
        kSemanticVariant: Option<String>,
        kSimplifiedVariant: Option<String>,
        kSpecializedSemanticVariant: Option<String>,
        kTaiwanTelegraph: Option<String>,
        kTang: Option<String>,
        kTotalStrokes: Option<String>,
        kTraditionalVariant: Option<String>,
        kVietnamese: Option<String>,
        kXHC1983: Option<String>,
        kWubi: Option<String>,
        kXerox: Option<String>,
        kZVariant: Option<String>,
        kRSTUnicode: Option<String>,
        kTGT_MergedSrc: Option<String>,
        kSrc_NushuDuben: Option<String>,
        kReading: Option<String>,
    },
    #[serde(rename = "noncharacter", rename_all = "kebab-case")]
    NonCharacter {
        age: String,
        first_cp: String,
        last_cp: String,
    },
    #[serde(rename_all = "kebab-case")]
    Reserved {
        age: String,
        cp: Option<String>,
        first_cp: Option<String>,
        last_cp: Option<String>,
    },
    #[serde(rename_all = "kebab-case")]
    Surrogate {
        age: String,
        first_cp: String,
        last_cp: String,
    },
}

#[derive(Debug, Deserialize)]
struct Blocks {
    #[serde(rename = "$value")]
    blocks: Vec<Block>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Block {
    first_cp: String,
    last_cp: String,
    name: String,
}

fn main() {
    use std::time::Instant;
    let start = Instant::now();
    println!("Start!");
    let raw = include_str!("ucd.all.flat.xml");
//     let raw = r##"<?xml version="1.0" encoding="utf-8" standalone="yes"?>

// <!-- © 2018 Unicode®, Inc. -->
// <!-- For terms of use, see http://www.unicode.org/terms_of_use.html -->


// <ucd xmlns="http://www.unicode.org/ns/2003/ucd/1.0">
//     <description>Unicode 11.0.0</description>
//     <repertoire>
//         <char cp="D7FB" age="5.2" na="HANGUL JONGSEONG PHIEUPH-THIEUTH" JSN="" gc="Lo" ccc="0" dt="none" dm="#" nt="None" nv="NaN" bc="L" bpt="n" bpb="#" Bidi_M="N" bmg="" suc="#" slc="#" stc="#" uc="#" lc="#" tc="#" scf="#" cf="#" jt="U" jg="No_Joining_Group" ea="N" lb="JT" sc="Hang" scx="Hang" Dash="N" WSpace="N" Hyphen="N" QMark="N" Radical="N" Ideo="N" UIdeo="N" IDSB="N" IDST="N" hst="T" DI="N" ODI="N" Alpha="Y" OAlpha="N" Upper="N" OUpper="N" Lower="N" OLower="N" Math="N" OMath="N" Hex="N" AHex="N" NChar="N" VS="N" Bidi_C="N" Join_C="N" Gr_Base="Y" Gr_Ext="N" OGr_Ext="N" Gr_Link="N" STerm="N" Ext="N" Term="N" Dia="N" Dep="N" IDS="Y" OIDS="N" XIDS="Y" IDC="Y" OIDC="N" XIDC="Y" SD="N" LOE="N" Pat_WS="N" Pat_Syn="N" GCB="T" WB="LE" SB="LE" CE="N" Comp_Ex="N" NFC_QC="Y" NFD_QC="Y" NFKC_QC="Y" NFKD_QC="Y" XO_NFC="N" XO_NFD="N" XO_NFKC="N" XO_NFKD="N" FC_NFKC="#" CI="N" Cased="N" CWCF="N" CWCM="N" CWKCF="N" CWL="N" CWT="N" CWU="N" NFKC_CF="#" InSC="Other" InPC="NA" PCM="N" vo="U" RI="N" blk="Jamo_Ext_B" isc="" na1=""/>
//         <surrogate first-cp="D800" last-cp="DB7F" age="2.0" na="" JSN="" gc="Cs" ccc="0" dt="none" dm="#" nt="None" nv="NaN" bc="L" bpt="n" bpb="#" Bidi_M="N" bmg="" suc="#" slc="#" stc="#" uc="#" lc="#" tc="#" scf="#" cf="#" jt="U" jg="No_Joining_Group" ea="N" lb="SG" sc="Zzzz" scx="Zzzz" Dash="N" WSpace="N" Hyphen="N" QMark="N" Radical="N" Ideo="N" UIdeo="N" IDSB="N" IDST="N" hst="NA" DI="N" ODI="N" Alpha="N" OAlpha="N" Upper="N" OUpper="N" Lower="N" OLower="N" Math="N" OMath="N" Hex="N" AHex="N" NChar="N" VS="N" Bidi_C="N" Join_C="N" Gr_Base="N" Gr_Ext="N" OGr_Ext="N" Gr_Link="N" STerm="N" Ext="N" Term="N" Dia="N" Dep="N" IDS="N" OIDS="N" XIDS="N" IDC="N" OIDC="N" XIDC="N" SD="N" LOE="N" Pat_WS="N" Pat_Syn="N" GCB="CN" WB="XX" SB="XX" CE="N" Comp_Ex="N" NFC_QC="Y" NFD_QC="Y" NFKC_QC="Y" NFKD_QC="Y" XO_NFC="N" XO_NFD="N" XO_NFKC="N" XO_NFKD="N" FC_NFKC="#" CI="N" Cased="N" CWCF="N" CWCM="N" CWKCF="N" CWL="N" CWT="N" CWU="N" NFKC_CF="#" InSC="Other" InPC="NA" PCM="N" vo="R" RI="N" blk="High_Surrogates" isc="" na1=""/>
//         <char cp="D7FB" age="5.2" na="HANGUL JONGSEONG PHIEUPH-THIEUTH" JSN="" gc="Lo" ccc="0" dt="none" dm="#" nt="None" nv="NaN" bc="L" bpt="n" bpb="#" Bidi_M="N" bmg="" suc="#" slc="#" stc="#" uc="#" lc="#" tc="#" scf="#" cf="#" jt="U" jg="No_Joining_Group" ea="N" lb="JT" sc="Hang" scx="Hang" Dash="N" WSpace="N" Hyphen="N" QMark="N" Radical="N" Ideo="N" UIdeo="N" IDSB="N" IDST="N" hst="T" DI="N" ODI="N" Alpha="Y" OAlpha="N" Upper="N" OUpper="N" Lower="N" OLower="N" Math="N" OMath="N" Hex="N" AHex="N" NChar="N" VS="N" Bidi_C="N" Join_C="N" Gr_Base="Y" Gr_Ext="N" OGr_Ext="N" Gr_Link="N" STerm="N" Ext="N" Term="N" Dia="N" Dep="N" IDS="Y" OIDS="N" XIDS="Y" IDC="Y" OIDC="N" XIDC="Y" SD="N" LOE="N" Pat_WS="N" Pat_Syn="N" GCB="T" WB="LE" SB="LE" CE="N" Comp_Ex="N" NFC_QC="Y" NFD_QC="Y" NFKC_QC="Y" NFKD_QC="Y" XO_NFC="N" XO_NFD="N" XO_NFKC="N" XO_NFKD="N" FC_NFKC="#" CI="N" Cased="N" CWCF="N" CWCM="N" CWKCF="N" CWL="N" CWT="N" CWU="N" NFKC_CF="#" InSC="Other" InPC="NA" PCM="N" vo="U" RI="N" blk="Jamo_Ext_B" isc="" na1=""/>
//         <char cp="D7FB" age="5.2" na="HANGUL JONGSEONG PHIEUPH-THIEUTH" JSN="" gc="Lo" ccc="0" dt="none" dm="#" nt="None" nv="NaN" bc="L" bpt="n" bpb="#" Bidi_M="N" bmg="" suc="#" slc="#" stc="#" uc="#" lc="#" tc="#" scf="#" cf="#" jt="U" jg="No_Joining_Group" ea="N" lb="JT" sc="Hang" scx="Hang" Dash="N" WSpace="N" Hyphen="N" QMark="N" Radical="N" Ideo="N" UIdeo="N" IDSB="N" IDST="N" hst="T" DI="N" ODI="N" Alpha="Y" OAlpha="N" Upper="N" OUpper="N" Lower="N" OLower="N" Math="N" OMath="N" Hex="N" AHex="N" NChar="N" VS="N" Bidi_C="N" Join_C="N" Gr_Base="Y" Gr_Ext="N" OGr_Ext="N" Gr_Link="N" STerm="N" Ext="N" Term="N" Dia="N" Dep="N" IDS="Y" OIDS="N" XIDS="Y" IDC="Y" OIDC="N" XIDC="Y" SD="N" LOE="N" Pat_WS="N" Pat_Syn="N" GCB="T" WB="LE" SB="LE" CE="N" Comp_Ex="N" NFC_QC="Y" NFD_QC="Y" NFKC_QC="Y" NFKD_QC="Y" XO_NFC="N" XO_NFD="N" XO_NFKC="N" XO_NFKD="N" FC_NFKC="#" CI="N" Cased="N" CWCF="N" CWCM="N" CWKCF="N" CWL="N" CWT="N" CWU="N" NFKC_CF="#" InSC="Other" InPC="NA" PCM="N" vo="U" RI="N" blk="Jamo_Ext_B" isc="" na1=""/>
//         <char cp="FDC7" age="1.1" na="ARABIC LIGATURE NOON WITH JEEM WITH YEH FINAL FORM" JSN="" gc="Lo" ccc="0" dt="fin" dm="0646 062C 064A" nt="None" nv="NaN" bc="AL" bpt="n" bpb="#" Bidi_M="N" bmg="" suc="#" slc="#" stc="#" uc="#" lc="#" tc="#" scf="#" cf="#" jt="U" jg="No_Joining_Group" ea="N" lb="AL" sc="Arab" scx="Arab" Dash="N" WSpace="N" Hyphen="N" QMark="N" Radical="N" Ideo="N" UIdeo="N" IDSB="N" IDST="N" hst="NA" DI="N" ODI="N" Alpha="Y" OAlpha="N" Upper="N" OUpper="N" Lower="N" OLower="N" Math="N" OMath="N" Hex="N" AHex="N" NChar="N" VS="N" Bidi_C="N" Join_C="N" Gr_Base="Y" Gr_Ext="N" OGr_Ext="N" Gr_Link="N" STerm="N" Ext="N" Term="N" Dia="N" Dep="N" IDS="Y" OIDS="N" XIDS="Y" IDC="Y" OIDC="N" XIDC="Y" SD="N" LOE="N" Pat_WS="N" Pat_Syn="N" GCB="XX" WB="LE" SB="LE" CE="N" Comp_Ex="N" NFC_QC="Y" NFD_QC="Y" NFKC_QC="N" NFKD_QC="N" XO_NFC="N" XO_NFD="N" XO_NFKC="Y" XO_NFKD="Y" FC_NFKC="#" CI="N" Cased="N" CWCF="N" CWCM="N" CWKCF="Y" CWL="N" CWT="N" CWU="N" NFKC_CF="0646 062C 064A" InSC="Other" InPC="NA" PCM="N" vo="R" RI="N" blk="Arabic_PF_A" isc="" na1=""/>
//         <reserved first-cp="FDC8" last-cp="FDCF" age="unassigned" na="" JSN="" gc="Cn" ccc="0" dt="none" dm="#" nt="None" nv="NaN" bc="AL" bpt="n" bpb="#" Bidi_M="N" bmg="" suc="#" slc="#" stc="#" uc="#" lc="#" tc="#" scf="#" cf="#" jt="U" jg="No_Joining_Group" ea="N" lb="XX" sc="Zzzz" scx="Zzzz" Dash="N" WSpace="N" Hyphen="N" QMark="N" Radical="N" Ideo="N" UIdeo="N" IDSB="N" IDST="N" hst="NA" DI="N" ODI="N" Alpha="N" OAlpha="N" Upper="N" OUpper="N" Lower="N" OLower="N" Math="N" OMath="N" Hex="N" AHex="N" NChar="N" VS="N" Bidi_C="N" Join_C="N" Gr_Base="N" Gr_Ext="N" OGr_Ext="N" Gr_Link="N" STerm="N" Ext="N" Term="N" Dia="N" Dep="N" IDS="N" OIDS="N" XIDS="N" IDC="N" OIDC="N" XIDC="N" SD="N" LOE="N" Pat_WS="N" Pat_Syn="N" GCB="XX" WB="XX" SB="XX" CE="N" Comp_Ex="N" NFC_QC="Y" NFD_QC="Y" NFKC_QC="Y" NFKD_QC="Y" XO_NFC="N" XO_NFD="N" XO_NFKC="N" XO_NFKD="N" FC_NFKC="#" CI="N" Cased="N" CWCF="N" CWCM="N" CWKCF="N" CWL="N" CWT="N" CWU="N" NFKC_CF="#" InSC="Other" InPC="NA" PCM="N" vo="R" RI="N" blk="Arabic_PF_A" isc="" na1=""/>
//         <noncharacter first-cp="FDD0" last-cp="FDEF" age="3.1" na="" JSN="" gc="Cn" ccc="0" dt="none" dm="#" nt="None" nv="NaN" bc="BN" bpt="n" bpb="#" Bidi_M="N" bmg="" suc="#" slc="#" stc="#" uc="#" lc="#" tc="#" scf="#" cf="#" jt="U" jg="No_Joining_Group" ea="N" lb="XX" sc="Zzzz" scx="Zzzz" Dash="N" WSpace="N" Hyphen="N" QMark="N" Radical="N" Ideo="N" UIdeo="N" IDSB="N" IDST="N" hst="NA" DI="N" ODI="N" Alpha="N" OAlpha="N" Upper="N" OUpper="N" Lower="N" OLower="N" Math="N" OMath="N" Hex="N" AHex="N" NChar="Y" VS="N" Bidi_C="N" Join_C="N" Gr_Base="N" Gr_Ext="N" OGr_Ext="N" Gr_Link="N" STerm="N" Ext="N" Term="N" Dia="N" Dep="N" IDS="N" OIDS="N" XIDS="N" IDC="N" OIDC="N" XIDC="N" SD="N" LOE="N" Pat_WS="N" Pat_Syn="N" GCB="XX" WB="XX" SB="XX" CE="N" Comp_Ex="N" NFC_QC="Y" NFD_QC="Y" NFKC_QC="Y" NFKD_QC="Y" XO_NFC="N" XO_NFD="N" XO_NFKC="N" XO_NFKD="N" FC_NFKC="#" CI="N" Cased="N" CWCF="N" CWCM="N" CWKCF="N" CWL="N" CWT="N" CWU="N" NFKC_CF="#" InSC="Other" InPC="NA" PCM="N" vo="R" RI="N" blk="Arabic_PF_A" isc="" na1=""/>
//     </repertoire>
//     <blocks>
//         <block first-cp="0000" last-cp="007F" name="Basic Latin"/>
//         <block first-cp="0080" last-cp="00FF" name="Latin-1 Supplement"/>
//         <block first-cp="0100" last-cp="017F" name="Latin Extended-A"/>
//         <block first-cp="0180" last-cp="024F" name="Latin Extended-B"/>
//         <block first-cp="0250" last-cp="02AF" name="IPA Extensions"/>
//         <block first-cp="02B0" last-cp="02FF" name="Spacing Modifier Letters"/>
//         <block first-cp="0300" last-cp="036F" name="Combining Diacritical Marks"/>
//         <block first-cp="0370" last-cp="03FF" name="Greek and Coptic"/>
//         <block first-cp="0400" last-cp="04FF" name="Cyrillic"/>
//         <block first-cp="0500" last-cp="052F" name="Cyrillic Supplement"/>
//         <block first-cp="0530" last-cp="058F" name="Armenian"/>
//         <block first-cp="0590" last-cp="05FF" name="Hebrew"/>
//     </blocks>
//     <named-sequences>
//         <named-sequence name="ARABIC SEQUENCE YEH WITH HAMZA ABOVE WITH AE" cps="0626 06D5"/>
//         <named-sequence name="ARABIC SEQUENCE YEH WITH HAMZA ABOVE WITH ALEF" cps="0626 0627"/>
//         <named-sequence name="ARABIC SEQUENCE YEH WITH HAMZA ABOVE WITH ALEF MAKSURA" cps="0626 0649"/>
//         <named-sequence name="ARABIC SEQUENCE YEH WITH HAMZA ABOVE WITH E" cps="0626 06D0"/>
//         <named-sequence name="ARABIC SEQUENCE YEH WITH HAMZA ABOVE WITH OE" cps="0626 06C6"/>
//         <named-sequence name="ARABIC SEQUENCE YEH WITH HAMZA ABOVE WITH U" cps="0626 06C7"/>
//         <named-sequence name="ARABIC SEQUENCE YEH WITH HAMZA ABOVE WITH WAW" cps="0626 0648"/>
//         <named-sequence name="ARABIC SEQUENCE YEH WITH HAMZA ABOVE WITH YU" cps="0626 06C8"/>
//     </named-sequences>
//     <normalization-corrections>
//         <normalization-correction cp="F951" old="96FB" new="964B" version="3.2.0"/>
//         <normalization-correction cp="2F868" old="2136A" new="36FC" version="4.0.0"/>
//         <normalization-correction cp="2F874" old="5F33" new="5F53" version="4.0.0"/>
//         <normalization-correction cp="2F91F" old="43AB" new="243AB" version="4.0.0"/>
//         <normalization-correction cp="2F95F" old="7AAE" new="7AEE" version="4.0.0"/>
//         <normalization-correction cp="2F9BF" old="4D57" new="45D7" version="4.0.0"/>
//     </normalization-corrections>
//     <standardized-variants> 
//         <standardized-variant cps="0030 FE00" desc="short diagonal stroke form" when=""/>
//         <standardized-variant cps="2205 FE00" desc="zero with long diagonal stroke overlay form" when=""/>
//         <standardized-variant cps="2229 FE00" desc="with serifs" when=""/>
//         <standardized-variant cps="222A FE00" desc="with serifs" when=""/>
//         <standardized-variant cps="2268 FE00" desc="with vertical stroke" when=""/>
//         <standardized-variant cps="2269 FE00" desc="with vertical stroke" when=""/>
//     </standardized-variants>
// </ucd>
// "##;
    let ucd: UCD = deserialize(raw.as_bytes()).unwrap();
    println!("{:#?}", ucd);

    println!("{:?}", start.elapsed());
}
