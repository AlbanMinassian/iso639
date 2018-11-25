//! # iso639-1
//!
//! iso639 v1 enum
//!
//! github: [https://github.com/AlbanMinassian/iso639](https://github.com/AlbanMinassian/iso639)
//!
//! license: MIT

/// iso639 enum
#[derive(Clone, Debug, PartialEq)]
pub enum Iso639_1 {
    Aa, // 639-2: aar, name: Afar (Afaraf)
    Ab, // 639-2: abk, name: Abkhaz (аҧсуа бызшәа, аҧсшәа)
    Ae, // 639-2: ave, name: Avestan (avesta)
    Af, // 639-2: afr, name: Afrikaans
    Ak, // 639-2: aka, name: Akan
    Am, // 639-2: amh, name: Amharic (አማርኛ)
    An, // 639-2: arg, name: Aragonese (aragonés)
    Ar, // 639-2: ara, name: Arabic (العربية)
    As, // 639-2: asm, name: Assamese (অসমীয়া)
    Av, // 639-2: ava, name: Avaric (авар мацӀ, магӀарул мацӀ)
    Ay, // 639-2: aym, name: Aymara (aymar aru)
    Az, // 639-2: aze, name: Azerbaijani (azərbaycan dili)
    Ba, // 639-2: bak, name: Bashkir (башҡорт теле)
    Be, // 639-2: bel, name: Belarusian (беларуская мова)
    Bg, // 639-2: bul, name: Bulgarian (български език)
    Bh, // 639-2: bih, name: Bihari (भोजपुरी)
    Bi, // 639-2: bis, name: Bislama
    Bm, // 639-2: bam, name: Bambara (bamanankan)
    Bn, // 639-2: ben, name: Bengali, Bangla (বাংলা)
    Bo, // 639-2: bod, name: Tibetan Standard, Tibetan, Central (བོད་ཡིག)
    Br, // 639-2: bre, name: Breton (brezhoneg)
    Bs, // 639-2: bos, name: Bosnian (bosanski jezik)
    Ca, // 639-2: cat, name: Catalan (català)
    Ce, // 639-2: che, name: Chechen (нохчийн мотт)
    Ch, // 639-2: cha, name: Chamorro (Chamoru)
    Co, // 639-2: cos, name: Corsican (corsu, lingua corsa)
    Cr, // 639-2: cre, name: Cree (ᓀᐦᐃᔭᐍᐏᐣ)
    Cs, // 639-2: ces, name: Czech (čeština, český jazyk)
    Cu, // 639-2: chu, name: Old Church Slavonic, Church Slavonic, Old Bulgarian (ѩзыкъ словѣньскъ)
    Cv, // 639-2: chv, name: Chuvash (чӑваш чӗлхи)
    Cy, // 639-2: cym, name: Welsh (Cymraeg)
    Da, // 639-2: dan, name: Danish (dansk)
    De, // 639-2: deu, name: German (Deutsch)
    Dv, // 639-2: div, name: Divehi, Dhivehi, Maldivian (ދިވެހި)
    Dz, // 639-2: dzo, name: Dzongkha (རྫོང་ཁ)
    Ee, // 639-2: ewe, name: Ewe (Eʋegbe)
    El, // 639-2: ell, name: Greek (modern) (ελληνικά)
    En, // 639-2: eng, name: English
    Eo, // 639-2: epo, name: Esperanto
    Es, // 639-2: spa, name: Spanish (Español)
    Et, // 639-2: est, name: Estonian (eesti, eesti keel)
    Eu, // 639-2: eus, name: Basque (euskara, euskera)
    Fa, // 639-2: fas, name: Persian (Farsi) (فارسی)
    Ff, // 639-2: ful, name: Fula, Fulah, Pulaar, Pular (Fulfulde, Pulaar, Pular)
    Fi, // 639-2: fin, name: Finnish (suomi, suomen kieli)
    Fj, // 639-2: fij, name: Fijian (vosa Vakaviti)
    Fo, // 639-2: fao, name: Faroese (føroyskt)
    Fr, // 639-2: fra, name: French (français, langue française)
    Fy, // 639-2: fry, name: Western Frisian (Frysk)
    Ga, // 639-2: gle, name: Irish (Gaeilge)
    Gd, // 639-2: gla, name: Scottish Gaelic, Gaelic (Gàidhlig)
    Gl, // 639-2: glg, name: Galician (galego)
    Gn, // 639-2: grn, name: Guaraní (Avañe'ẽ)
    Gu, // 639-2: guj, name: Gujarati (ગુજરાતી)
    Gv, // 639-2: glv, name: Manx (Gaelg, Gailck)
    Ha, // 639-2: hau, name: Hausa ((Hausa) هَوُسَ)
    He, // 639-2: heb, name: Hebrew (modern) (עברית)
    Hi, // 639-2: hin, name: Hindi (हिन्दी, हिंदी)
    Ho, // 639-2: hmo, name: Hiri Motu
    Hr, // 639-2: hrv, name: Croatian (hrvatski jezik)
    Ht, // 639-2: hat, name: Haitian, Haitian Creole (Kreyòl ayisyen)
    Hu, // 639-2: hun, name: Hungarian (magyar)
    Hy, // 639-2: hye, name: Armenian (Հայերեն)
    Hz, // 639-2: her, name: Herero (Otjiherero)
    Ia, // 639-2: ina, name: Interlingua
    Id, // 639-2: ind, name: Indonesian (Bahasa Indonesia)
    Ie, // 639-2: ile, name: Interlingue (Originally called Occidental; then Interlingue after WWII)
    Ig, // 639-2: ibo, name: Igbo (Asụsụ Igbo)
    Ii, // 639-2: iii, name: Nuosu (ꆈꌠ꒿ Nuosuhxop)
    Ik, // 639-2: ipk, name: Inupiaq (Iñupiaq, Iñupiatun)
    Io, // 639-2: ido, name: Ido
    Is, // 639-2: isl, name: Icelandic (Íslenska)
    It, // 639-2: ita, name: Italian (Italiano)
    Iu, // 639-2: iku, name: Inuktitut (ᐃᓄᒃᑎᑐᑦ)
    Ja, // 639-2: jpn, name: Japanese (日本語 (にほんご))
    Jv, // 639-2: jav, name: Javanese (ꦧꦱꦗꦮ, Basa Jawa)
    Ka, // 639-2: kat, name: Georgian (ქართული)
    Kg, // 639-2: kon, name: Kongo (Kikongo)
    Ki, // 639-2: kik, name: Kikuyu, Gikuyu (Gĩkũyũ)
    Kj, // 639-2: kua, name: Kwanyama, Kuanyama (Kuanyama)
    Kk, // 639-2: kaz, name: Kazakh (қазақ тілі)
    Kl, // 639-2: kal, name: Kalaallisut, Greenlandic (kalaallisut, kalaallit oqaasii)
    Km, // 639-2: khm, name: Khmer (ខ្មែរ, ខេមរភាសា, ភាសាខ្មែរ)
    Kn, // 639-2: kan, name: Kannada (ಕನ್ನಡ)
    Ko, // 639-2: kor, name: Korean (한국어)
    Kr, // 639-2: kau, name: Kanuri
    Ks, // 639-2: kas, name: Kashmiri (कश्मीरी, كشميري‎)
    Ku, // 639-2: kur, name: Kurdish (Kurdî, كوردی‎)
    Kv, // 639-2: kom, name: Komi (коми кыв)
    Kw, // 639-2: cor, name: Cornish (Kernewek)
    Ky, // 639-2: kir, name: Kyrgyz (Кыргызча, Кыргыз тили)
    La, // 639-2: lat, name: Latin (latine, lingua latina)
    Lb, // 639-2: ltz, name: Luxembourgish, Letzeburgesch (Lëtzebuergesch)
    Lg, // 639-2: lug, name: Ganda (Luganda)
    Li, // 639-2: lim, name: Limburgish, Limburgan, Limburger (Limburgs)
    Ln, // 639-2: lin, name: Lingala (Lingála)
    Lo, // 639-2: lao, name: Lao (ພາສາລາວ)
    Lt, // 639-2: lit, name: Lithuanian (lietuvių kalba)
    Lu, // 639-2: lub, name: Luba-Katanga (Tshiluba)
    Lv, // 639-2: lav, name: Latvian (latviešu valoda)
    Mg, // 639-2: mlg, name: Malagasy (fiteny malagasy)
    Mh, // 639-2: mah, name: Marshallese (Kajin M̧ajeļ)
    Mi, // 639-2: mri, name: Māori (te reo Māori)
    Mk, // 639-2: mkd, name: Macedonian (македонски јазик)
    Ml, // 639-2: mal, name: Malayalam (മലയാളം)
    Mn, // 639-2: mon, name: Mongolian (Монгол хэл)
    Mr, // 639-2: mar, name: Marathi (Marāṭhī) (मराठी)
    Ms, // 639-2: msa, name: Malay (bahasa Melayu, بهاس ملايو‎)
    Mt, // 639-2: mlt, name: Maltese (Malti)
    My, // 639-2: mya, name: Burmese (ဗမာစာ)
    Na, // 639-2: nau, name: Nauruan (Dorerin Naoero)
    Nb, // 639-2: nob, name: Norwegian Bokmål (Norsk bokmål)
    Nd, // 639-2: nde, name: Northern Ndebele (isiNdebele)
    Ne, // 639-2: nep, name: Nepali (नेपाली)
    Ng, // 639-2: ndo, name: Ndonga (Owambo)
    Nl, // 639-2: nld, name: Dutch (Nederlands, Vlaams)
    Nn, // 639-2: nno, name: Norwegian Nynorsk (Norsk nynorsk)
    No, // 639-2: nor, name: Norwegian (Norsk)
    Nr, // 639-2: nbl, name: Southern Ndebele (isiNdebele)
    Nv, // 639-2: nav, name: Navajo, Navaho (Diné bizaad)
    Ny, // 639-2: nya, name: Chichewa, Chewa, Nyanja (chiCheŵa, chinyanja)
    Oc, // 639-2: oci, name: Occitan (occitan, lenga d'òc)
    Oj, // 639-2: oji, name: Ojibwe, Ojibwa (ᐊᓂᔑᓈᐯᒧᐎᓐ)
    Om, // 639-2: orm, name: Oromo (Afaan Oromoo)
    Or, // 639-2: ori, name: Oriya (ଓଡ଼ିଆ)
    Os, // 639-2: oss, name: Ossetian, Ossetic (ирон æвзаг)
    Pa, // 639-2: pan, name: (Eastern) Punjabi (ਪੰਜਾਬੀ)
    Pi, // 639-2: pli, name: Pāli (पाऴि)
    Pl, // 639-2: pol, name: Polish (język polski, polszczyzna)
    Ps, // 639-2: pus, name: Pashto, Pushto (پښتو)
    Pt, // 639-2: por, name: Portuguese (Português)
    Qu, // 639-2: que, name: Quechua (Runa Simi, Kichwa)
    Rm, // 639-2: roh, name: Romansh (rumantsch grischun)
    Rn, // 639-2: run, name: Kirundi (Ikirundi)
    Ro, // 639-2: ron, name: Romanian (Română)
    Ru, // 639-2: rus, name: Russian (Русский)
    Rw, // 639-2: kin, name: Kinyarwanda (Ikinyarwanda)
    Sa, // 639-2: san, name: Sanskrit (Saṁskṛta) (संस्कृतम्)
    Sc, // 639-2: srd, name: Sardinian (sardu)
    Sd, // 639-2: snd, name: Sindhi (सिन्धी, سنڌي، سندھی‎)
    Se, // 639-2: sme, name: Northern Sami (Davvisámegiella)
    Sg, // 639-2: sag, name: Sango (yângâ tî sängö)
    Si, // 639-2: sin, name: Sinhalese, Sinhala (සිංහල)
    Sk, // 639-2: slk, name: Slovak (slovenčina, slovenský jazyk)
    Sl, // 639-2: slv, name: Slovene (slovenski jezik, slovenščina)
    Sm, // 639-2: smo, name: Samoan (gagana fa'a Samoa)
    Sn, // 639-2: sna, name: Shona (chiShona)
    So, // 639-2: som, name: Somali (Soomaaliga, af Soomaali)
    Sq, // 639-2: sqi, name: Albanian (Shqip)
    Sr, // 639-2: srp, name: Serbian (српски језик)
    Ss, // 639-2: ssw, name: Swati (SiSwati)
    St, // 639-2: sot, name: Southern Sotho (Sesotho)
    Su, // 639-2: sun, name: Sundanese (Basa Sunda)
    Sv, // 639-2: swe, name: Swedish (svenska)
    Sw, // 639-2: swa, name: Swahili (Kiswahili)
    Ta, // 639-2: tam, name: Tamil (தமிழ்)
    Te, // 639-2: tel, name: Telugu (తెలుగు)
    Tg, // 639-2: tgk, name: Tajik (тоҷикӣ, toçikī, تاجیکی‎)
    Th, // 639-2: tha, name: Thai (ไทย)
    Ti, // 639-2: tir, name: Tigrinya (ትግርኛ)
    Tk, // 639-2: tuk, name: Turkmen (Türkmen, Түркмен)
    Tl, // 639-2: tgl, name: Tagalog (Wikang Tagalog)
    Tn, // 639-2: tsn, name: Tswana (Setswana)
    To, // 639-2: ton, name: Tonga (Tonga Islands) (faka Tonga)
    Tr, // 639-2: tur, name: Turkish (Türkçe)
    Ts, // 639-2: tso, name: Tsonga (Xitsonga)
    Tt, // 639-2: tat, name: Tatar (татар теле, tatar tele)
    Tw, // 639-2: twi, name: Twi
    Ty, // 639-2: tah, name: Tahitian (Reo Tahiti)
    Ug, // 639-2: uig, name: Uyghur (ئۇيغۇرچە‎, Uyghurche)
    Uk, // 639-2: ukr, name: Ukrainian (Українська)
    Ur, // 639-2: urd, name: Urdu (اردو)
    Uz, // 639-2: uzb, name: Uzbek (Oʻzbek, Ўзбек, أۇزبېك‎)
    Ve, // 639-2: ven, name: Venda (Tshivenḓa)
    Vi, // 639-2: vie, name: Vietnamese (Tiếng Việt)
    Vo, // 639-2: vol, name: Volapük
    Wa, // 639-2: wln, name: Walloon (walon)
    Wo, // 639-2: wol, name: Wolof (Wollof)
    Xh, // 639-2: xho, name: Xhosa (isiXhosa)
    Yi, // 639-2: yid, name: Yiddish (ייִדיש)
    Yo, // 639-2: yor, name: Yoruba (Yorùbá)
    Za, // 639-2: zha, name: Zhuang, Chuang (Saɯ cueŋƅ, Saw cuengh)
    Zh, // 639-2: zho, name: Chinese (中文 (Zhōngwén), 汉语, 漢語)
    Zu, // 639-2: zul, name: Zulu (isiZulu)
}

/// return enum ``Iso639_1`` switch iso639-1 string (2 chars)
/// else panic
pub fn get_enum(language: &str) -> Iso639_1 {
    match language {
        // match iso639-1
        "aa" => Iso639_1::Aa,
        "ab" => Iso639_1::Ab,
        "ae" => Iso639_1::Ae,
        "af" => Iso639_1::Af,
        "ak" => Iso639_1::Ak,
        "am" => Iso639_1::Am,
        "an" => Iso639_1::An,
        "ar" => Iso639_1::Ar,
        "as" => Iso639_1::As,
        "av" => Iso639_1::Av,
        "ay" => Iso639_1::Ay,
        "az" => Iso639_1::Az,
        "ba" => Iso639_1::Ba,
        "be" => Iso639_1::Be,
        "bg" => Iso639_1::Bg,
        "bh" => Iso639_1::Bh,
        "bi" => Iso639_1::Bi,
        "bm" => Iso639_1::Bm,
        "bn" => Iso639_1::Bn,
        "bo" => Iso639_1::Bo,
        "br" => Iso639_1::Br,
        "bs" => Iso639_1::Bs,
        "ca" => Iso639_1::Ca,
        "ce" => Iso639_1::Ce,
        "ch" => Iso639_1::Ch,
        "co" => Iso639_1::Co,
        "cr" => Iso639_1::Cr,
        "cs" => Iso639_1::Cs,
        "cu" => Iso639_1::Cu,
        "cv" => Iso639_1::Cv,
        "cy" => Iso639_1::Cy,
        "da" => Iso639_1::Da,
        "de" => Iso639_1::De,
        "dv" => Iso639_1::Dv,
        "dz" => Iso639_1::Dz,
        "ee" => Iso639_1::Ee,
        "el" => Iso639_1::El,
        "en" => Iso639_1::En,
        "eo" => Iso639_1::Eo,
        "es" => Iso639_1::Es,
        "et" => Iso639_1::Et,
        "eu" => Iso639_1::Eu,
        "fa" => Iso639_1::Fa,
        "ff" => Iso639_1::Ff,
        "fi" => Iso639_1::Fi,
        "fj" => Iso639_1::Fj,
        "fo" => Iso639_1::Fo,
        "fr" => Iso639_1::Fr,
        "fy" => Iso639_1::Fy,
        "ga" => Iso639_1::Ga,
        "gd" => Iso639_1::Gd,
        "gl" => Iso639_1::Gl,
        "gn" => Iso639_1::Gn,
        "gu" => Iso639_1::Gu,
        "gv" => Iso639_1::Gv,
        "ha" => Iso639_1::Ha,
        "he" => Iso639_1::He,
        "hi" => Iso639_1::Hi,
        "ho" => Iso639_1::Ho,
        "hr" => Iso639_1::Hr,
        "ht" => Iso639_1::Ht,
        "hu" => Iso639_1::Hu,
        "hy" => Iso639_1::Hy,
        "hz" => Iso639_1::Hz,
        "ia" => Iso639_1::Ia,
        "id" => Iso639_1::Id,
        "ie" => Iso639_1::Ie,
        "ig" => Iso639_1::Ig,
        "ii" => Iso639_1::Ii,
        "ik" => Iso639_1::Ik,
        "io" => Iso639_1::Io,
        "is" => Iso639_1::Is,
        "it" => Iso639_1::It,
        "iu" => Iso639_1::Iu,
        "ja" => Iso639_1::Ja,
        "jv" => Iso639_1::Jv,
        "ka" => Iso639_1::Ka,
        "kg" => Iso639_1::Kg,
        "ki" => Iso639_1::Ki,
        "kj" => Iso639_1::Kj,
        "kk" => Iso639_1::Kk,
        "kl" => Iso639_1::Kl,
        "km" => Iso639_1::Km,
        "kn" => Iso639_1::Kn,
        "ko" => Iso639_1::Ko,
        "kr" => Iso639_1::Kr,
        "ks" => Iso639_1::Ks,
        "ku" => Iso639_1::Ku,
        "kv" => Iso639_1::Kv,
        "kw" => Iso639_1::Kw,
        "ky" => Iso639_1::Ky,
        "la" => Iso639_1::La,
        "lb" => Iso639_1::Lb,
        "lg" => Iso639_1::Lg,
        "li" => Iso639_1::Li,
        "ln" => Iso639_1::Ln,
        "lo" => Iso639_1::Lo,
        "lt" => Iso639_1::Lt,
        "lu" => Iso639_1::Lu,
        "lv" => Iso639_1::Lv,
        "mg" => Iso639_1::Mg,
        "mh" => Iso639_1::Mh,
        "mi" => Iso639_1::Mi,
        "mk" => Iso639_1::Mk,
        "ml" => Iso639_1::Ml,
        "mn" => Iso639_1::Mn,
        "mr" => Iso639_1::Mr,
        "ms" => Iso639_1::Ms,
        "mt" => Iso639_1::Mt,
        "my" => Iso639_1::My,
        "na" => Iso639_1::Na,
        "nb" => Iso639_1::Nb,
        "nd" => Iso639_1::Nd,
        "ne" => Iso639_1::Ne,
        "ng" => Iso639_1::Ng,
        "nl" => Iso639_1::Nl,
        "nn" => Iso639_1::Nn,
        "no" => Iso639_1::No,
        "nr" => Iso639_1::Nr,
        "nv" => Iso639_1::Nv,
        "ny" => Iso639_1::Ny,
        "oc" => Iso639_1::Oc,
        "oj" => Iso639_1::Oj,
        "om" => Iso639_1::Om,
        "or" => Iso639_1::Or,
        "os" => Iso639_1::Os,
        "pa" => Iso639_1::Pa,
        "pi" => Iso639_1::Pi,
        "pl" => Iso639_1::Pl,
        "ps" => Iso639_1::Ps,
        "pt" => Iso639_1::Pt,
        "qu" => Iso639_1::Qu,
        "rm" => Iso639_1::Rm,
        "rn" => Iso639_1::Rn,
        "ro" => Iso639_1::Ro,
        "ru" => Iso639_1::Ru,
        "rw" => Iso639_1::Rw,
        "sa" => Iso639_1::Sa,
        "sc" => Iso639_1::Sc,
        "sd" => Iso639_1::Sd,
        "se" => Iso639_1::Se,
        "sg" => Iso639_1::Sg,
        "si" => Iso639_1::Si,
        "sk" => Iso639_1::Sk,
        "sl" => Iso639_1::Sl,
        "sm" => Iso639_1::Sm,
        "sn" => Iso639_1::Sn,
        "so" => Iso639_1::So,
        "sq" => Iso639_1::Sq,
        "sr" => Iso639_1::Sr,
        "ss" => Iso639_1::Ss,
        "st" => Iso639_1::St,
        "su" => Iso639_1::Su,
        "sv" => Iso639_1::Sv,
        "sw" => Iso639_1::Sw,
        "ta" => Iso639_1::Ta,
        "te" => Iso639_1::Te,
        "tg" => Iso639_1::Tg,
        "th" => Iso639_1::Th,
        "ti" => Iso639_1::Ti,
        "tk" => Iso639_1::Tk,
        "tl" => Iso639_1::Tl,
        "tn" => Iso639_1::Tn,
        "to" => Iso639_1::To,
        "tr" => Iso639_1::Tr,
        "ts" => Iso639_1::Ts,
        "tt" => Iso639_1::Tt,
        "tw" => Iso639_1::Tw,
        "ty" => Iso639_1::Ty,
        "ug" => Iso639_1::Ug,
        "uk" => Iso639_1::Uk,
        "ur" => Iso639_1::Ur,
        "uz" => Iso639_1::Uz,
        "ve" => Iso639_1::Ve,
        "vi" => Iso639_1::Vi,
        "vo" => Iso639_1::Vo,
        "wa" => Iso639_1::Wa,
        "wo" => Iso639_1::Wo,
        "xh" => Iso639_1::Xh,
        "yi" => Iso639_1::Yi,
        "yo" => Iso639_1::Yo,
        "za" => Iso639_1::Za,
        "zh" => Iso639_1::Zh,
        "zu" => Iso639_1::Zu,
        _ => { panic!("not found") }
    }
}

/// get iso639-3 string (3 chars) switch iso639-1 string (2 chars)
/// else panic
pub fn get_code_iso639_3(language: &str) -> &str {
    match language {
        "aa" => "aar",
        "ab" => "abk",
        "ae" => "ave",
        "af" => "afr",
        "ak" => "aka",
        "am" => "amh",
        "an" => "arg",
        "ar" => "ara",
        "as" => "asm",
        "av" => "ava",
        "ay" => "aym",
        "az" => "aze",
        "ba" => "bak",
        "be" => "bel",
        "bg" => "bul",
        "bh" => "bih",
        "bi" => "bis",
        "bm" => "bam",
        "bn" => "ben",
        "bo" => "bod",
        "br" => "bre",
        "bs" => "bos",
        "ca" => "cat",
        "ce" => "che",
        "ch" => "cha",
        "co" => "cos",
        "cr" => "cre",
        "cs" => "ces",
        "cu" => "chu",
        "cv" => "chv",
        "cy" => "cym",
        "da" => "dan",
        "de" => "deu",
        "dv" => "div",
        "dz" => "dzo",
        "ee" => "ewe",
        "el" => "ell",
        "en" => "eng",
        "eo" => "epo",
        "es" => "spa",
        "et" => "est",
        "eu" => "eus",
        "fa" => "fas",
        "ff" => "ful",
        "fi" => "fin",
        "fj" => "fij",
        "fo" => "fao",
        "fr" => "fra",
        "fy" => "fry",
        "ga" => "gle",
        "gd" => "gla",
        "gl" => "glg",
        "gn" => "grn",
        "gu" => "guj",
        "gv" => "glv",
        "ha" => "hau",
        "he" => "heb",
        "hi" => "hin",
        "ho" => "hmo",
        "hr" => "hrv",
        "ht" => "hat",
        "hu" => "hun",
        "hy" => "hye",
        "hz" => "her",
        "ia" => "ina",
        "id" => "ind",
        "ie" => "ile",
        "ig" => "ibo",
        "ii" => "iii",
        "ik" => "ipk",
        "io" => "ido",
        "is" => "isl",
        "it" => "ita",
        "iu" => "iku",
        "ja" => "jpn",
        "jv" => "jav",
        "ka" => "kat",
        "kg" => "kon",
        "ki" => "kik",
        "kj" => "kua",
        "kk" => "kaz",
        "kl" => "kal",
        "km" => "khm",
        "kn" => "kan",
        "ko" => "kor",
        "kr" => "kau",
        "ks" => "kas",
        "ku" => "kur",
        "kv" => "kom",
        "kw" => "cor",
        "ky" => "kir",
        "la" => "lat",
        "lb" => "ltz",
        "lg" => "lug",
        "li" => "lim",
        "ln" => "lin",
        "lo" => "lao",
        "lt" => "lit",
        "lu" => "lub",
        "lv" => "lav",
        "mg" => "mlg",
        "mh" => "mah",
        "mi" => "mri",
        "mk" => "mkd",
        "ml" => "mal",
        "mn" => "mon",
        "mr" => "mar",
        "ms" => "msa",
        "mt" => "mlt",
        "my" => "mya",
        "na" => "nau",
        "nb" => "nob",
        "nd" => "nde",
        "ne" => "nep",
        "ng" => "ndo",
        "nl" => "nld",
        "nn" => "nno",
        "no" => "nor",
        "nr" => "nbl",
        "nv" => "nav",
        "ny" => "nya",
        "oc" => "oci",
        "oj" => "oji",
        "om" => "orm",
        "or" => "ori",
        "os" => "oss",
        "pa" => "pan",
        "pi" => "pli",
        "pl" => "pol",
        "ps" => "pus",
        "pt" => "por",
        "qu" => "que",
        "rm" => "roh",
        "rn" => "run",
        "ro" => "ron",
        "ru" => "rus",
        "rw" => "kin",
        "sa" => "san",
        "sc" => "srd",
        "sd" => "snd",
        "se" => "sme",
        "sg" => "sag",
        "si" => "sin",
        "sk" => "slk",
        "sl" => "slv",
        "sm" => "smo",
        "sn" => "sna",
        "so" => "som",
        "sq" => "sqi",
        "sr" => "srp",
        "ss" => "ssw",
        "st" => "sot",
        "su" => "sun",
        "sv" => "swe",
        "sw" => "swa",
        "ta" => "tam",
        "te" => "tel",
        "tg" => "tgk",
        "th" => "tha",
        "ti" => "tir",
        "tk" => "tuk",
        "tl" => "tgl",
        "tn" => "tsn",
        "to" => "ton",
        "tr" => "tur",
        "ts" => "tso",
        "tt" => "tat",
        "tw" => "twi",
        "ty" => "tah",
        "ug" => "uig",
        "uk" => "ukr",
        "ur" => "urd",
        "uz" => "uzb",
        "ve" => "ven",
        "vi" => "vie",
        "vo" => "vol",
        "wa" => "wln",
        "wo" => "wol",
        "xh" => "xho",
        "yi" => "yid",
        "yo" => "yor",
        "za" => "zha",
        "zh" => "zho",
        "zu" => "zul",
        _ => { panic!("not found") }
    }
}
