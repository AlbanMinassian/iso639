//! # iso639-1
//!
//! [![Build Status](https://travis-ci.org/AlbanMinassian/iso639.svg?branch=master)](https://travis-ci.org/AlbanMinassian/iso639)
//! [![codecov](https://codecov.io/gh/AlbanMinassian/iso639/branch/master/graph/badge.svg)](https://codecov.io/gh/AlbanMinassian/iso639)
//! [![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
//! [![iso639-1 Latest Version](https://img.shields.io/crates/v/iso639-1.svg)](https://crates.io/crates/iso639-1)
//!
//! iso639 v1 enum and utils (for Rust)
//!
//! ## Iso 639-1 ##
//!
//! ISO 639-1:2002, Codes for the representation of names of languages — Part 1: Alpha-2 code, is the first part of the ISO 639 series of international standards for language codes. Part 1 covers the registration of two-letter codes. There are 184 two-letter codes registered as of October 2015.
//! The registered codes cover the world's major languages. (source [https://en.wikipedia.org/wiki/ISO_639-1](https://en.wikipedia.org/wiki/ISO_639-1))
//!
//! ## Example
//!
//! ```rust
//! extern crate iso639_1;
//! use iso639_1::{Iso639_1, from_iso639_1, to_iso639_3};

//! pub fn main() {
//!     assert!(Iso639_1::Fr != Iso639_1::En);
//!     assert!(from_iso639_1("fr").unwrap() == Iso639_1::Fr);
//!     assert!(to_iso639_3("fr").unwrap() == "fra");
//! }
//! ```
//!
//! ## Links
//!
//! github: [https://github.com/AlbanMinassian/iso639](https://github.com/AlbanMinassian/iso639)
//!
//! ## license
//!
//! MIT

extern crate core;
extern crate failure;

use core::fmt::Display;
use failure::{Backtrace, Context, Fail};
use std::fmt;

// ------------------------------------------------------------------------------------
// Error
// ------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Iso639v1Error {
    pub inner: Context<Iso639v1ErrorKind>,
}

#[derive(Fail, Debug, Clone, Eq, PartialEq)]
pub enum Iso639v1ErrorKind {
    NotFoundFrom(String),
    NotFoundTo(String),
}

impl fmt::Display for Iso639v1ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Iso639v1ErrorKind::NotFoundFrom(ref param) => write!(f, "not found {}", param),
            Iso639v1ErrorKind::NotFoundTo(ref param) => write!(f, "not found {}", param),
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
impl Fail for Iso639v1Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }
    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Iso639v1Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Iso639v1Error {
    pub fn kind(&self) -> &Iso639v1ErrorKind {
        &*self.inner.get_context()
    }
}

impl From<Iso639v1ErrorKind> for Iso639v1Error {
    fn from(kind: Iso639v1ErrorKind) -> Iso639v1Error {
        Iso639v1Error {
            inner: Context::new(kind),
        }
    }
}

// ------------------------------------------------------------------------------------
// Iso639_1 (enum)
// ------------------------------------------------------------------------------------
/// iso639-1 enum
///
/// # Examples
/// ```rust
/// extern crate iso639_1;
/// use iso639_1::Iso639_1;
/// fn main() {
///     println!("{:?}", Iso639_1::En);
///     assert!(Iso639_1::Fr != Iso639_1::En);
/// }
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Iso639_1 {
    /// 639-2: aar, name: Afar (Afaraf)
    Aa,
    /// 639-2: abk, name: Abkhaz (аҧсуа бызшәа, аҧсшәа)
    Ab,
    /// 639-2: ave, name: Avestan (avesta)
    Ae,
    /// 639-2: afr, name: Afrikaans
    Af,
    /// 639-2: aka, name: Akan
    Ak,
    /// 639-2: amh, name: Amharic (አማርኛ)
    Am,
    /// 639-2: arg, name: Aragonese (aragonés)
    An,
    /// 639-2: ara, name: Arabic (العربية)
    Ar,
    /// 639-2: asm, name: Assamese (অসমীয়া)
    As,
    /// 639-2: ava, name: Avaric (авар мацӀ, магӀарул мацӀ)
    Av,
    /// 639-2: aym, name: Aymara (aymar aru)
    Ay,
    /// 639-2: aze, name: Azerbaijani (azərbaycan dili)
    Az,
    /// 639-2: bak, name: Bashkir (башҡорт теле)
    Ba,
    /// 639-2: bel, name: Belarusian (беларуская мова)
    Be,
    /// 639-2: bul, name: Bulgarian (български език)
    Bg,
    /// 639-2: bih, name: Bihari (भोजपुरी)
    Bh,
    /// 639-2: bis, name: Bislama
    Bi,
    /// 639-2: bam, name: Bambara (bamanankan)
    Bm,
    /// 639-2: ben, name: Bengali, Bangla (বাংলা)
    Bn,
    /// 639-2: bod, name: Tibetan Standard, Tibetan, Central (བོད་ཡིག)
    Bo,
    /// 639-2: bre, name: Breton (brezhoneg)
    Br,
    /// 639-2: bos, name: Bosnian (bosanski jezik)
    Bs,
    /// 639-2: cat, name: Catalan (català)
    Ca,
    /// 639-2: che, name: Chechen (нохчийн мотт)
    Ce,
    /// 639-2: cha, name: Chamorro (Chamoru)
    Ch,
    /// 639-2: cos, name: Corsican (corsu, lingua corsa)
    Co,
    /// 639-2: cre, name: Cree (ᓀᐦᐃᔭᐍᐏᐣ)
    Cr,
    /// 639-2: ces, name: Czech (čeština, český jazyk)
    Cs,
    /// 639-2: chu, name: Old Church Slavonic, Church Slavonic, Old Bulgarian (ѩзыкъ словѣньскъ)
    Cu,
    /// 639-2: chv, name: Chuvash (чӑваш чӗлхи)
    Cv,
    /// 639-2: cym, name: Welsh (Cymraeg)
    Cy,
    /// 639-2: dan, name: Danish (dansk)
    Da,
    /// 639-2: deu, name: German (Deutsch)
    De,
    /// 639-2: div, name: Divehi, Dhivehi, Maldivian (ދިވެހި)
    Dv,
    /// 639-2: dzo, name: Dzongkha (རྫོང་ཁ)
    Dz,
    /// 639-2: ewe, name: Ewe (Eʋegbe)
    Ee,
    /// 639-2: ell, name: Greek (modern) (ελληνικά)
    El,
    /// 639-2: eng, name: English
    En,
    /// 639-2: epo, name: Esperanto
    Eo,
    /// 639-2: spa, name: Spanish (Español)
    Es,
    /// 639-2: est, name: Estonian (eesti, eesti keel)
    Et,
    /// 639-2: eus, name: Basque (euskara, euskera)
    Eu,
    /// 639-2: fas, name: Persian (Farsi) (فارسی)
    Fa,
    /// 639-2: ful, name: Fula, Fulah, Pulaar, Pular (Fulfulde, Pulaar, Pular)
    Ff,
    /// 639-2: fin, name: Finnish (suomi, suomen kieli)
    Fi,
    /// 639-2: fij, name: Fijian (vosa Vakaviti)
    Fj,
    /// 639-2: fao, name: Faroese (føroyskt)
    Fo,
    /// 639-2: fra, name: French (français, langue française)
    Fr,
    /// 639-2: fry, name: Western Frisian (Frysk)
    Fy,
    /// 639-2: gle, name: Irish (Gaeilge)
    Ga,
    /// 639-2: gla, name: Scottish Gaelic, Gaelic (Gàidhlig)
    Gd,
    /// 639-2: glg, name: Galician (galego)
    Gl,
    /// 639-2: grn, name: Guaraní (Avañe'ẽ)
    Gn,
    /// 639-2: guj, name: Gujarati (ગુજરાતી)
    Gu,
    /// 639-2: glv, name: Manx (Gaelg, Gailck)
    Gv,
    /// 639-2: hau, name: Hausa ((Hausa) هَوُسَ)
    Ha,
    /// 639-2: heb, name: Hebrew (modern) (עברית)
    He,
    /// 639-2: hin, name: Hindi (हिन्दी, हिंदी)
    Hi,
    /// 639-2: hmo, name: Hiri Motu
    Ho,
    /// 639-2: hrv, name: Croatian (hrvatski jezik)
    Hr,
    /// 639-2: hat, name: Haitian, Haitian Creole (Kreyòl ayisyen)
    Ht,
    /// 639-2: hun, name: Hungarian (magyar)
    Hu,
    /// 639-2: hye, name: Armenian (Հայերեն)
    Hy,
    /// 639-2: her, name: Herero (Otjiherero)
    Hz,
    /// 639-2: ina, name: Interlingua
    Ia,
    /// 639-2: ind, name: Indonesian (Bahasa Indonesia)
    Id,
    /// 639-2: ile, name: Interlingue (Originally called Occidental; then Interlingue after WWII)
    Ie,
    /// 639-2: ibo, name: Igbo (Asụsụ Igbo)
    Ig,
    /// 639-2: iii, name: Nuosu (ꆈꌠ꒿ Nuosuhxop)
    Ii,
    /// 639-2: ipk, name: Inupiaq (Iñupiaq, Iñupiatun)
    Ik,
    /// 639-2: ido, name: Ido
    Io,
    /// 639-2: isl, name: Icelandic (Íslenska)
    Is,
    /// 639-2: ita, name: Italian (Italiano)
    It,
    /// 639-2: iku, name: Inuktitut (ᐃᓄᒃᑎᑐᑦ)
    Iu,
    /// 639-2: jpn, name: Japanese (日本語 (にほんご))
    Ja,
    /// 639-2: jav, name: Javanese (ꦧꦱꦗꦮ, Basa Jawa)
    Jv,
    /// 639-2: kat, name: Georgian (ქართული)
    Ka,
    /// 639-2: kon, name: Kongo (Kikongo)
    Kg,
    /// 639-2: kik, name: Kikuyu, Gikuyu (Gĩkũyũ)
    Ki,
    /// 639-2: kua, name: Kwanyama, Kuanyama (Kuanyama)
    Kj,
    /// 639-2: kaz, name: Kazakh (қазақ тілі)
    Kk,
    /// 639-2: kal, name: Kalaallisut, Greenlandic (kalaallisut, kalaallit oqaasii)
    Kl,
    /// 639-2: khm, name: Khmer (ខ្មែរ, ខេមរភាសា, ភាសាខ្មែរ)
    Km,
    /// 639-2: kan, name: Kannada (ಕನ್ನಡ)
    Kn,
    /// 639-2: kor, name: Korean (한국어)
    Ko,
    /// 639-2: kau, name: Kanuri
    Kr,
    /// 639-2: kas, name: Kashmiri (कश्मीरी, كشميري‎)
    Ks,
    /// 639-2: kur, name: Kurdish (Kurdî, كوردی‎)
    Ku,
    /// 639-2: kom, name: Komi (коми кыв)
    Kv,
    /// 639-2: cor, name: Cornish (Kernewek)
    Kw,
    /// 639-2: kir, name: Kyrgyz (Кыргызча, Кыргыз тили)
    Ky,
    /// 639-2: lat, name: Latin (latine, lingua latina)
    La,
    /// 639-2: ltz, name: Luxembourgish, Letzeburgesch (Lëtzebuergesch)
    Lb,
    /// 639-2: lug, name: Ganda (Luganda)
    Lg,
    /// 639-2: lim, name: Limburgish, Limburgan, Limburger (Limburgs)
    Li,
    /// 639-2: lin, name: Lingala (Lingála)
    Ln,
    /// 639-2: lao, name: Lao (ພາສາລາວ)
    Lo,
    /// 639-2: lit, name: Lithuanian (lietuvių kalba)
    Lt,
    /// 639-2: lub, name: Luba-Katanga (Tshiluba)
    Lu,
    /// 639-2: lav, name: Latvian (latviešu valoda)
    Lv,
    /// 639-2: mlg, name: Malagasy (fiteny malagasy)
    Mg,
    /// 639-2: mah, name: Marshallese (Kajin M̧ajeļ)
    Mh,
    /// 639-2: mri, name: Māori (te reo Māori)
    Mi,
    /// 639-2: mkd, name: Macedonian (македонски јазик)
    Mk,
    /// 639-2: mal, name: Malayalam (മലയാളം)
    Ml,
    /// 639-2: mon, name: Mongolian (Монгол хэл)
    Mn,
    /// 639-2: mar, name: Marathi (Marāṭhī) (मराठी)
    Mr,
    /// 639-2: msa, name: Malay (bahasa Melayu, بهاس ملايو‎)
    Ms,
    /// 639-2: mlt, name: Maltese (Malti)
    Mt,
    /// 639-2: mya, name: Burmese (ဗမာစာ)
    My,
    /// 639-2: nau, name: Nauruan (Dorerin Naoero)
    Na,
    /// 639-2: nob, name: Norwegian Bokmål (Norsk bokmål)
    Nb,
    /// 639-2: nde, name: Northern Ndebele (isiNdebele)
    Nd,
    /// 639-2: nep, name: Nepali (नेपाली)
    Ne,
    /// 639-2: ndo, name: Ndonga (Owambo)
    Ng,
    /// 639-2: nld, name: Dutch (Nederlands, Vlaams)
    Nl,
    /// 639-2: nno, name: Norwegian Nynorsk (Norsk nynorsk)
    Nn,
    /// 639-2: nor, name: Norwegian (Norsk)
    No,
    /// 639-2: nbl, name: Southern Ndebele (isiNdebele)
    Nr,
    /// 639-2: nav, name: Navajo, Navaho (Diné bizaad)
    Nv,
    /// 639-2: nya, name: Chichewa, Chewa, Nyanja (chiCheŵa, chinyanja)
    Ny,
    /// 639-2: oci, name: Occitan (occitan, lenga d'òc)
    Oc,
    /// 639-2: oji, name: Ojibwe, Ojibwa (ᐊᓂᔑᓈᐯᒧᐎᓐ)
    Oj,
    /// 639-2: orm, name: Oromo (Afaan Oromoo)
    Om,
    /// 639-2: ori, name: Oriya (ଓଡ଼ିଆ)
    Or,
    /// 639-2: oss, name: Ossetian, Ossetic (ирон æвзаг)
    Os,
    /// 639-2: pan, name: (Eastern) Punjabi (ਪੰਜਾਬੀ)
    Pa,
    /// 639-2: pli, name: Pāli (पाऴि)
    Pi,
    /// 639-2: pol, name: Polish (język polski, polszczyzna)
    Pl,
    /// 639-2: pus, name: Pashto, Pushto (پښتو)
    Ps,
    /// 639-2: por, name: Portuguese (Português)
    Pt,
    /// 639-2: que, name: Quechua (Runa Simi, Kichwa)
    Qu,
    /// 639-2: roh, name: Romansh (rumantsch grischun)
    Rm,
    /// 639-2: run, name: Kirundi (Ikirundi)
    Rn,
    /// 639-2: ron, name: Romanian (Română)
    Ro,
    /// 639-2: rus, name: Russian (Русский)
    Ru,
    /// 639-2: kin, name: Kinyarwanda (Ikinyarwanda)
    Rw,
    /// 639-2: san, name: Sanskrit (Saṁskṛta) (संस्कृतम्)
    Sa,
    /// 639-2: srd, name: Sardinian (sardu)
    Sc,
    /// 639-2: snd, name: Sindhi (सिन्धी, سنڌي، سندھی‎)
    Sd,
    /// 639-2: sme, name: Northern Sami (Davvisámegiella)
    Se,
    /// 639-2: sag, name: Sango (yângâ tî sängö)
    Sg,
    /// 639-2: sin, name: Sinhalese, Sinhala (සිංහල)
    Si,
    /// 639-2: slk, name: Slovak (slovenčina, slovenský jazyk)
    Sk,
    /// 639-2: slv, name: Slovene (slovenski jezik, slovenščina)
    Sl,
    /// 639-2: smo, name: Samoan (gagana fa'a Samoa)
    Sm,
    /// 639-2: sna, name: Shona (chiShona)
    Sn,
    /// 639-2: som, name: Somali (Soomaaliga, af Soomaali)
    So,
    /// 639-2: sqi, name: Albanian (Shqip)
    Sq,
    /// 639-2: srp, name: Serbian (српски језик)
    Sr,
    /// 639-2: ssw, name: Swati (SiSwati)
    Ss,
    /// 639-2: sot, name: Southern Sotho (Sesotho)
    St,
    /// 639-2: sun, name: Sundanese (Basa Sunda)
    Su,
    /// 639-2: swe, name: Swedish (svenska)
    Sv,
    /// 639-2: swa, name: Swahili (Kiswahili)
    Sw,
    /// 639-2: tam, name: Tamil (தமிழ்)
    Ta,
    /// 639-2: tel, name: Telugu (తెలుగు)
    Te,
    /// 639-2: tgk, name: Tajik (тоҷикӣ, toçikī, تاجیکی‎)
    Tg,
    /// 639-2: tha, name: Thai (ไทย)
    Th,
    /// 639-2: tir, name: Tigrinya (ትግርኛ)
    Ti,
    /// 639-2: tuk, name: Turkmen (Türkmen, Түркмен)
    Tk,
    /// 639-2: tgl, name: Tagalog (Wikang Tagalog)
    Tl,
    /// 639-2: tsn, name: Tswana (Setswana)
    Tn,
    /// 639-2: ton, name: Tonga (Tonga Islands) (faka Tonga)
    To,
    /// 639-2: tur, name: Turkish (Türkçe)
    Tr,
    /// 639-2: tso, name: Tsonga (Xitsonga)
    Ts,
    /// 639-2: tat, name: Tatar (татар теле, tatar tele)
    Tt,
    /// 639-2: twi, name: Twi
    Tw,
    /// 639-2: tah, name: Tahitian (Reo Tahiti)
    Ty,
    /// 639-2: uig, name: Uyghur (ئۇيغۇرچە‎, Uyghurche)
    Ug,
    /// 639-2: ukr, name: Ukrainian (Українська)
    Uk,
    /// 639-2: urd, name: Urdu (اردو)
    Ur,
    /// 639-2: uzb, name: Uzbek (Oʻzbek, Ўзбек, أۇزبېك‎)
    Uz,
    /// 639-2: ven, name: Venda (Tshivenḓa)
    Ve,
    /// 639-2: vie, name: Vietnamese (Tiếng Việt)
    Vi,
    /// 639-2: vol, name: Volapük
    Vo,
    /// 639-2: wln, name: Walloon (walon)
    Wa,
    /// 639-2: wol, name: Wolof (Wollof)
    Wo,
    /// 639-2: xho, name: Xhosa (isiXhosa)
    Xh,
    /// 639-2: yid, name: Yiddish (ייִדיש)
    Yi,
    /// 639-2: yor, name: Yoruba (Yorùbá)
    Yo,
    /// 639-2: zha, name: Zhuang, Chuang (Saɯ cueŋƅ, Saw cuengh)
    Za,
    /// 639-2: zho, name: Chinese (中文 (Zhōngwén), 汉语, 漢語)
    Zh,
    /// 639-2: zul, name: Zulu (isiZulu)
    Zu,
}

// ------------------------------------------------------------------------------------
// from_iso639_1(language: &str) -> Iso639_1
// ------------------------------------------------------------------------------------
/// return enum ``Iso639_1`` switch iso639-1 string (2 chars) - or error ``Iso639v1Error``
///
/// # Examples
/// ```rust
/// extern crate iso639_1;
/// use iso639_1::from_iso639_1;
/// use iso639_1::Iso639_1;
/// fn main() {
///     let lang = from_iso639_1("fr").unwrap();
///     assert!(lang == Iso639_1::Fr);
/// }
/// ```
pub fn from_iso639_1(language: &str) -> Result<Iso639_1, Iso639v1Error> {
    match language {
        // match iso639-1
        "aa" => Ok(Iso639_1::Aa),
        "ab" => Ok(Iso639_1::Ab),
        "ae" => Ok(Iso639_1::Ae),
        "af" => Ok(Iso639_1::Af),
        "ak" => Ok(Iso639_1::Ak),
        "am" => Ok(Iso639_1::Am),
        "an" => Ok(Iso639_1::An),
        "ar" => Ok(Iso639_1::Ar),
        "as" => Ok(Iso639_1::As),
        "av" => Ok(Iso639_1::Av),
        "ay" => Ok(Iso639_1::Ay),
        "az" => Ok(Iso639_1::Az),
        "ba" => Ok(Iso639_1::Ba),
        "be" => Ok(Iso639_1::Be),
        "bg" => Ok(Iso639_1::Bg),
        "bh" => Ok(Iso639_1::Bh),
        "bi" => Ok(Iso639_1::Bi),
        "bm" => Ok(Iso639_1::Bm),
        "bn" => Ok(Iso639_1::Bn),
        "bo" => Ok(Iso639_1::Bo),
        "br" => Ok(Iso639_1::Br),
        "bs" => Ok(Iso639_1::Bs),
        "ca" => Ok(Iso639_1::Ca),
        "ce" => Ok(Iso639_1::Ce),
        "ch" => Ok(Iso639_1::Ch),
        "co" => Ok(Iso639_1::Co),
        "cr" => Ok(Iso639_1::Cr),
        "cs" => Ok(Iso639_1::Cs),
        "cu" => Ok(Iso639_1::Cu),
        "cv" => Ok(Iso639_1::Cv),
        "cy" => Ok(Iso639_1::Cy),
        "da" => Ok(Iso639_1::Da),
        "de" => Ok(Iso639_1::De),
        "dv" => Ok(Iso639_1::Dv),
        "dz" => Ok(Iso639_1::Dz),
        "ee" => Ok(Iso639_1::Ee),
        "el" => Ok(Iso639_1::El),
        "en" => Ok(Iso639_1::En),
        "eo" => Ok(Iso639_1::Eo),
        "es" => Ok(Iso639_1::Es),
        "et" => Ok(Iso639_1::Et),
        "eu" => Ok(Iso639_1::Eu),
        "fa" => Ok(Iso639_1::Fa),
        "ff" => Ok(Iso639_1::Ff),
        "fi" => Ok(Iso639_1::Fi),
        "fj" => Ok(Iso639_1::Fj),
        "fo" => Ok(Iso639_1::Fo),
        "fr" => Ok(Iso639_1::Fr),
        "fy" => Ok(Iso639_1::Fy),
        "ga" => Ok(Iso639_1::Ga),
        "gd" => Ok(Iso639_1::Gd),
        "gl" => Ok(Iso639_1::Gl),
        "gn" => Ok(Iso639_1::Gn),
        "gu" => Ok(Iso639_1::Gu),
        "gv" => Ok(Iso639_1::Gv),
        "ha" => Ok(Iso639_1::Ha),
        "he" => Ok(Iso639_1::He),
        "hi" => Ok(Iso639_1::Hi),
        "ho" => Ok(Iso639_1::Ho),
        "hr" => Ok(Iso639_1::Hr),
        "ht" => Ok(Iso639_1::Ht),
        "hu" => Ok(Iso639_1::Hu),
        "hy" => Ok(Iso639_1::Hy),
        "hz" => Ok(Iso639_1::Hz),
        "ia" => Ok(Iso639_1::Ia),
        "id" => Ok(Iso639_1::Id),
        "ie" => Ok(Iso639_1::Ie),
        "ig" => Ok(Iso639_1::Ig),
        "ii" => Ok(Iso639_1::Ii),
        "ik" => Ok(Iso639_1::Ik),
        "io" => Ok(Iso639_1::Io),
        "is" => Ok(Iso639_1::Is),
        "it" => Ok(Iso639_1::It),
        "iu" => Ok(Iso639_1::Iu),
        "ja" => Ok(Iso639_1::Ja),
        "jv" => Ok(Iso639_1::Jv),
        "ka" => Ok(Iso639_1::Ka),
        "kg" => Ok(Iso639_1::Kg),
        "ki" => Ok(Iso639_1::Ki),
        "kj" => Ok(Iso639_1::Kj),
        "kk" => Ok(Iso639_1::Kk),
        "kl" => Ok(Iso639_1::Kl),
        "km" => Ok(Iso639_1::Km),
        "kn" => Ok(Iso639_1::Kn),
        "ko" => Ok(Iso639_1::Ko),
        "kr" => Ok(Iso639_1::Kr),
        "ks" => Ok(Iso639_1::Ks),
        "ku" => Ok(Iso639_1::Ku),
        "kv" => Ok(Iso639_1::Kv),
        "kw" => Ok(Iso639_1::Kw),
        "ky" => Ok(Iso639_1::Ky),
        "la" => Ok(Iso639_1::La),
        "lb" => Ok(Iso639_1::Lb),
        "lg" => Ok(Iso639_1::Lg),
        "li" => Ok(Iso639_1::Li),
        "ln" => Ok(Iso639_1::Ln),
        "lo" => Ok(Iso639_1::Lo),
        "lt" => Ok(Iso639_1::Lt),
        "lu" => Ok(Iso639_1::Lu),
        "lv" => Ok(Iso639_1::Lv),
        "mg" => Ok(Iso639_1::Mg),
        "mh" => Ok(Iso639_1::Mh),
        "mi" => Ok(Iso639_1::Mi),
        "mk" => Ok(Iso639_1::Mk),
        "ml" => Ok(Iso639_1::Ml),
        "mn" => Ok(Iso639_1::Mn),
        "mr" => Ok(Iso639_1::Mr),
        "ms" => Ok(Iso639_1::Ms),
        "mt" => Ok(Iso639_1::Mt),
        "my" => Ok(Iso639_1::My),
        "na" => Ok(Iso639_1::Na),
        "nb" => Ok(Iso639_1::Nb),
        "nd" => Ok(Iso639_1::Nd),
        "ne" => Ok(Iso639_1::Ne),
        "ng" => Ok(Iso639_1::Ng),
        "nl" => Ok(Iso639_1::Nl),
        "nn" => Ok(Iso639_1::Nn),
        "no" => Ok(Iso639_1::No),
        "nr" => Ok(Iso639_1::Nr),
        "nv" => Ok(Iso639_1::Nv),
        "ny" => Ok(Iso639_1::Ny),
        "oc" => Ok(Iso639_1::Oc),
        "oj" => Ok(Iso639_1::Oj),
        "om" => Ok(Iso639_1::Om),
        "or" => Ok(Iso639_1::Or),
        "os" => Ok(Iso639_1::Os),
        "pa" => Ok(Iso639_1::Pa),
        "pi" => Ok(Iso639_1::Pi),
        "pl" => Ok(Iso639_1::Pl),
        "ps" => Ok(Iso639_1::Ps),
        "pt" => Ok(Iso639_1::Pt),
        "qu" => Ok(Iso639_1::Qu),
        "rm" => Ok(Iso639_1::Rm),
        "rn" => Ok(Iso639_1::Rn),
        "ro" => Ok(Iso639_1::Ro),
        "ru" => Ok(Iso639_1::Ru),
        "rw" => Ok(Iso639_1::Rw),
        "sa" => Ok(Iso639_1::Sa),
        "sc" => Ok(Iso639_1::Sc),
        "sd" => Ok(Iso639_1::Sd),
        "se" => Ok(Iso639_1::Se),
        "sg" => Ok(Iso639_1::Sg),
        "si" => Ok(Iso639_1::Si),
        "sk" => Ok(Iso639_1::Sk),
        "sl" => Ok(Iso639_1::Sl),
        "sm" => Ok(Iso639_1::Sm),
        "sn" => Ok(Iso639_1::Sn),
        "so" => Ok(Iso639_1::So),
        "sq" => Ok(Iso639_1::Sq),
        "sr" => Ok(Iso639_1::Sr),
        "ss" => Ok(Iso639_1::Ss),
        "st" => Ok(Iso639_1::St),
        "su" => Ok(Iso639_1::Su),
        "sv" => Ok(Iso639_1::Sv),
        "sw" => Ok(Iso639_1::Sw),
        "ta" => Ok(Iso639_1::Ta),
        "te" => Ok(Iso639_1::Te),
        "tg" => Ok(Iso639_1::Tg),
        "th" => Ok(Iso639_1::Th),
        "ti" => Ok(Iso639_1::Ti),
        "tk" => Ok(Iso639_1::Tk),
        "tl" => Ok(Iso639_1::Tl),
        "tn" => Ok(Iso639_1::Tn),
        "to" => Ok(Iso639_1::To),
        "tr" => Ok(Iso639_1::Tr),
        "ts" => Ok(Iso639_1::Ts),
        "tt" => Ok(Iso639_1::Tt),
        "tw" => Ok(Iso639_1::Tw),
        "ty" => Ok(Iso639_1::Ty),
        "ug" => Ok(Iso639_1::Ug),
        "uk" => Ok(Iso639_1::Uk),
        "ur" => Ok(Iso639_1::Ur),
        "uz" => Ok(Iso639_1::Uz),
        "ve" => Ok(Iso639_1::Ve),
        "vi" => Ok(Iso639_1::Vi),
        "vo" => Ok(Iso639_1::Vo),
        "wa" => Ok(Iso639_1::Wa),
        "wo" => Ok(Iso639_1::Wo),
        "xh" => Ok(Iso639_1::Xh),
        "yi" => Ok(Iso639_1::Yi),
        "yo" => Ok(Iso639_1::Yo),
        "za" => Ok(Iso639_1::Za),
        "zh" => Ok(Iso639_1::Zh),
        "zu" => Ok(Iso639_1::Zu),
        _ => {
            return Err(Iso639v1Error::from(Iso639v1ErrorKind::NotFoundFrom(
                language.to_string(),
            )))
        }
    }
}

// ------------------------------------------------------------------------------------
// to_iso639_3(language: &str) -> &str
// ------------------------------------------------------------------------------------
/// get iso639-3 string (3 chars) switch iso639-1 string (2 chars) - or error ``Iso639v1Error``
///
/// # Examples
/// ```rust
/// extern crate iso639_1;
/// use iso639_1::to_iso639_3;
/// fn main() {
///     let lang = to_iso639_3("fr").unwrap();
///     assert!(lang == "fra");
/// }
/// ```
pub fn to_iso639_3(language: &str) -> Result<&str, Iso639v1Error> {
    match language {
        "aa" => Ok("aar"),
        "ab" => Ok("abk"),
        "ae" => Ok("ave"),
        "af" => Ok("afr"),
        "ak" => Ok("aka"),
        "am" => Ok("amh"),
        "an" => Ok("arg"),
        "ar" => Ok("ara"),
        "as" => Ok("asm"),
        "av" => Ok("ava"),
        "ay" => Ok("aym"),
        "az" => Ok("aze"),
        "ba" => Ok("bak"),
        "be" => Ok("bel"),
        "bg" => Ok("bul"),
        "bh" => Ok("bih"),
        "bi" => Ok("bis"),
        "bm" => Ok("bam"),
        "bn" => Ok("ben"),
        "bo" => Ok("bod"),
        "br" => Ok("bre"),
        "bs" => Ok("bos"),
        "ca" => Ok("cat"),
        "ce" => Ok("che"),
        "ch" => Ok("cha"),
        "co" => Ok("cos"),
        "cr" => Ok("cre"),
        "cs" => Ok("ces"),
        "cu" => Ok("chu"),
        "cv" => Ok("chv"),
        "cy" => Ok("cym"),
        "da" => Ok("dan"),
        "de" => Ok("deu"),
        "dv" => Ok("div"),
        "dz" => Ok("dzo"),
        "ee" => Ok("ewe"),
        "el" => Ok("ell"),
        "en" => Ok("eng"),
        "eo" => Ok("epo"),
        "es" => Ok("spa"),
        "et" => Ok("est"),
        "eu" => Ok("eus"),
        "fa" => Ok("fas"),
        "ff" => Ok("ful"),
        "fi" => Ok("fin"),
        "fj" => Ok("fij"),
        "fo" => Ok("fao"),
        "fr" => Ok("fra"),
        "fy" => Ok("fry"),
        "ga" => Ok("gle"),
        "gd" => Ok("gla"),
        "gl" => Ok("glg"),
        "gn" => Ok("grn"),
        "gu" => Ok("guj"),
        "gv" => Ok("glv"),
        "ha" => Ok("hau"),
        "he" => Ok("heb"),
        "hi" => Ok("hin"),
        "ho" => Ok("hmo"),
        "hr" => Ok("hrv"),
        "ht" => Ok("hat"),
        "hu" => Ok("hun"),
        "hy" => Ok("hye"),
        "hz" => Ok("her"),
        "ia" => Ok("ina"),
        "id" => Ok("ind"),
        "ie" => Ok("ile"),
        "ig" => Ok("ibo"),
        "ii" => Ok("iii"),
        "ik" => Ok("ipk"),
        "io" => Ok("ido"),
        "is" => Ok("isl"),
        "it" => Ok("ita"),
        "iu" => Ok("iku"),
        "ja" => Ok("jpn"),
        "jv" => Ok("jav"),
        "ka" => Ok("kat"),
        "kg" => Ok("kon"),
        "ki" => Ok("kik"),
        "kj" => Ok("kua"),
        "kk" => Ok("kaz"),
        "kl" => Ok("kal"),
        "km" => Ok("khm"),
        "kn" => Ok("kan"),
        "ko" => Ok("kor"),
        "kr" => Ok("kau"),
        "ks" => Ok("kas"),
        "ku" => Ok("kur"),
        "kv" => Ok("kom"),
        "kw" => Ok("cor"),
        "ky" => Ok("kir"),
        "la" => Ok("lat"),
        "lb" => Ok("ltz"),
        "lg" => Ok("lug"),
        "li" => Ok("lim"),
        "ln" => Ok("lin"),
        "lo" => Ok("lao"),
        "lt" => Ok("lit"),
        "lu" => Ok("lub"),
        "lv" => Ok("lav"),
        "mg" => Ok("mlg"),
        "mh" => Ok("mah"),
        "mi" => Ok("mri"),
        "mk" => Ok("mkd"),
        "ml" => Ok("mal"),
        "mn" => Ok("mon"),
        "mr" => Ok("mar"),
        "ms" => Ok("msa"),
        "mt" => Ok("mlt"),
        "my" => Ok("mya"),
        "na" => Ok("nau"),
        "nb" => Ok("nob"),
        "nd" => Ok("nde"),
        "ne" => Ok("nep"),
        "ng" => Ok("ndo"),
        "nl" => Ok("nld"),
        "nn" => Ok("nno"),
        "no" => Ok("nor"),
        "nr" => Ok("nbl"),
        "nv" => Ok("nav"),
        "ny" => Ok("nya"),
        "oc" => Ok("oci"),
        "oj" => Ok("oji"),
        "om" => Ok("orm"),
        "or" => Ok("ori"),
        "os" => Ok("oss"),
        "pa" => Ok("pan"),
        "pi" => Ok("pli"),
        "pl" => Ok("pol"),
        "ps" => Ok("pus"),
        "pt" => Ok("por"),
        "qu" => Ok("que"),
        "rm" => Ok("roh"),
        "rn" => Ok("run"),
        "ro" => Ok("ron"),
        "ru" => Ok("rus"),
        "rw" => Ok("kin"),
        "sa" => Ok("san"),
        "sc" => Ok("srd"),
        "sd" => Ok("snd"),
        "se" => Ok("sme"),
        "sg" => Ok("sag"),
        "si" => Ok("sin"),
        "sk" => Ok("slk"),
        "sl" => Ok("slv"),
        "sm" => Ok("smo"),
        "sn" => Ok("sna"),
        "so" => Ok("som"),
        "sq" => Ok("sqi"),
        "sr" => Ok("srp"),
        "ss" => Ok("ssw"),
        "st" => Ok("sot"),
        "su" => Ok("sun"),
        "sv" => Ok("swe"),
        "sw" => Ok("swa"),
        "ta" => Ok("tam"),
        "te" => Ok("tel"),
        "tg" => Ok("tgk"),
        "th" => Ok("tha"),
        "ti" => Ok("tir"),
        "tk" => Ok("tuk"),
        "tl" => Ok("tgl"),
        "tn" => Ok("tsn"),
        "to" => Ok("ton"),
        "tr" => Ok("tur"),
        "ts" => Ok("tso"),
        "tt" => Ok("tat"),
        "tw" => Ok("twi"),
        "ty" => Ok("tah"),
        "ug" => Ok("uig"),
        "uk" => Ok("ukr"),
        "ur" => Ok("urd"),
        "uz" => Ok("uzb"),
        "ve" => Ok("ven"),
        "vi" => Ok("vie"),
        "vo" => Ok("vol"),
        "wa" => Ok("wln"),
        "wo" => Ok("wol"),
        "xh" => Ok("xho"),
        "yi" => Ok("yid"),
        "yo" => Ok("yor"),
        "za" => Ok("zha"),
        "zh" => Ok("zho"),
        "zu" => Ok("zul"),
        _ => {
            return Err(Iso639v1Error::from(Iso639v1ErrorKind::NotFoundTo(
                language.to_string(),
            )))
        }
    }
}
