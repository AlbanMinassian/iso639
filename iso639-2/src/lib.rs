//! # iso639_2
//!
//! [![Build Status](https://travis-ci.org/AlbanMinassian/iso639.svg?branch=master)](https://travis-ci.org/AlbanMinassian/iso639)
//! [![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
//! [![iso639_2 Latest Version](https://img.shields.io/crates/v/iso639_2.svg)](https://crates.io/crates/iso639_2)
//!
//! iso639 v2 enum (for Rust) - Work in progress -
//!
//! ## Iso 639-2 ##
//!
//! ISO 639-2:1998, Codes for the representation of names of languages — Part 2: Alpha-3 code, is the second part of the ISO 639 standard, which lists codes for the representation of the names of languages. The three-letter codes given for each language in this part of the standard are referred to as "Alpha-3" codes.
//! There are 464 entries in the list of ISO 639-2 codes.  (source [https://en.wikipedia.org/wiki/ISO_639-2](https://en.wikipedia.org/wiki/ISO_639-2))
//!
//! ## Links
//!
//! github: [https://github.com/AlbanMinassian/iso639](https://github.com/AlbanMinassian/iso639)
//!
//! ## license
//!
//! MIT

/// iso639-2 enum
///
/// # Examples
/// ```rust
/// extern crate iso639_2;
/// use iso639_2::Iso639_2;
/// fn main() {
///     println!("{:?}", Iso639_2::Eng);
///     assert!(Iso639_2::Fra != Iso639_2::Eng);
/// }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum Iso639_2 {
    /// Afar - 639-1: aa
    Aar,
    /// Abkhazian - 639-1: ab
    Abk,
    /// Achinese
    Ace,
    /// Acoli
    Ach,
    /// Adangme
    Ada,
    /// Adyghe
    Ady,
    /// Afro-Asiatic languages
    Afa,
    /// Afrihili
    Afh,
    /// Afrikaans - 639-1: af
    Afr,
    /// Ainu
    Ain,
    /// Akan - 639-1: ak
    Aka,
    /// Akkadian
    Akk,
    /// Albanian - 639-1: sq
    Alb,
    /// Aleut
    Ale,
    /// Algonquian languages
    Alg,
    /// Southern Altai
    Alt,
    /// Amharic - 639-1: am
    Amh,
    /// English, Old (ca.450-1100)
    Ang,
    /// Angika
    Anp,
    /// Apache languages
    Apa,
    /// Arabic - 639-1: ar
    Ara,
    /// Official Aramaic (700-300 BCE)
    Arc,
    /// Aragonese - 639-1: an
    Arg,
    /// Armenian - 639-1: hy
    Arm,
    /// Mapudungun
    Arn,
    /// Arapaho
    Arp,
    /// Artificial languages
    Art,
    /// Arawak
    Arw,
    /// Assamese - 639-1: as
    Asm,
    /// Asturian
    Ast,
    /// Athapascan languages
    Ath,
    /// Australian languages
    Aus,
    /// Avaric - 639-1: av
    Ava,
    /// Avestan - 639-1: ae
    Ave,
    /// Awadhi
    Awa,
    /// Aymara - 639-1: ay
    Aym,
    /// Azerbaijani - 639-1: az
    Aze,
    /// Banda languages
    Bad,
    /// Bamileke languages
    Bai,
    /// Bashkir - 639-1: ba
    Bak,
    /// Baluchi
    Bal,
    /// Bambara - 639-1: bm
    Bam,
    /// Balinese
    Ban,
    /// Basque - 639-1: eu
    Baq,
    /// Basa
    Bas,
    /// Baltic languages
    Bat,
    /// Beja
    Bej,
    /// Belarusian - 639-1: be
    Bel,
    /// Bemba
    Bem,
    /// Bengali - 639-1: bn
    Ben,
    /// Berber languages
    Ber,
    /// Bhojpuri
    Bho,
    /// Bihari languages - 639-1: bh
    Bih,
    /// Bikol
    Bik,
    /// Bini
    Bin,
    /// Bislama - 639-1: bi
    Bis,
    /// Siksika
    Bla,
    /// Bantu languages
    Bnt,
    /// Tibetan - 639-1: bo
    Bod,
    /// Bosnian - 639-1: bs
    Bos,
    /// Braj
    Bra,
    /// Breton - 639-1: br
    Bre,
    /// Batak languages
    Btk,
    /// Buriat
    Bua,
    /// Buginese
    Bug,
    /// Bulgarian - 639-1: bg
    Bul,
    /// Burmese - 639-1: my
    Bur,
    /// Blin
    Byn,
    /// Caddo
    Cad,
    /// Central American Indian languages
    Cai,
    /// Galibi Carib
    Car,
    /// Catalan - 639-1: ca
    Cat,
    /// Caucasian languages
    Cau,
    /// Cebuano
    Ceb,
    /// Celtic languages
    Cel,
    /// Czech - 639-1: cs
    Ces,
    /// Chamorro - 639-1: ch
    Cha,
    /// Chibcha
    Chb,
    /// Chechen - 639-1: ce
    Che,
    /// Chagatai
    Chg,
    /// Chinese - 639-1: zh
    Chi,
    /// Chuukese
    Chk,
    /// Mari
    Chm,
    /// Chinook jargon
    Chn,
    /// Choctaw
    Cho,
    /// Chipewyan
    Chp,
    /// Cherokee
    Chr,
    /// Church Slavic - 639-1: cu
    Chu,
    /// Chuvash - 639-1: cv
    Chv,
    /// Cheyenne
    Chy,
    /// Chamic languages
    Cmc,
    /// Coptic
    Cop,
    /// Cornish - 639-1: kw
    Cor,
    /// Corsican - 639-1: co
    Cos,
    /// Creoles and pidgins, English based
    Cpe,
    /// Creoles and pidgins, French-based
    Cpf,
    /// Creoles and pidgins, Portuguese-based
    Cpp,
    /// Cree - 639-1: cr
    Cre,
    /// Crimean Tatar
    Crh,
    /// Creoles and pidgins
    Crp,
    /// Kashubian
    Csb,
    /// Cushitic languages
    Cus,
    /// Welsh - 639-1: cy
    Cym,
    /// Czech - 639-1: cs
    Cze,
    /// Dakota
    Dak,
    /// Danish - 639-1: da
    Dan,
    /// Dargwa
    Dar,
    /// Land Dayak languages
    Day,
    /// Delaware
    Del,
    /// Slave (Athapascan)
    Den,
    /// German - 639-1: de
    Deu,
    /// Dogrib
    Dgr,
    /// Dinka
    Din,
    /// Divehi - 639-1: dv
    Div,
    /// Dogri
    Doi,
    /// Dravidian languages
    Dra,
    /// Lower Sorbian
    Dsb,
    /// Duala
    Dua,
    /// Dutch, Middle (ca.1050-1350)
    Dum,
    /// Dutch - 639-1: nl
    Dut,
    /// Dyula
    Dyu,
    /// Dzongkha - 639-1: dz
    Dzo,
    /// Efik
    Efi,
    /// Egyptian (Ancient)
    Egy,
    /// Ekajuk
    Eka,
    /// Greek, Modern (1453-) - 639-1: el
    Ell,
    /// Elamite
    Elx,
    /// English - 639-1: en
    Eng,
    /// English, Middle (1100-1500)
    Enm,
    /// Esperanto - 639-1: eo
    Epo,
    /// Estonian - 639-1: et
    Est,
    /// Basque - 639-1: eu
    Eus,
    /// Ewe - 639-1: ee
    Ewe,
    /// Ewondo
    Ewo,
    /// Fang
    Fan,
    /// Faroese - 639-1: fo
    Fao,
    /// Persian - 639-1: fa
    Fas,
    /// Fanti
    Fat,
    /// Fijian - 639-1: fj
    Fij,
    /// Filipino
    Fil,
    /// Finnish - 639-1: fi
    Fin,
    /// Finno-Ugrian languages
    Fiu,
    /// Fon
    Fon,
    /// French - 639-1: fr
    Fra,
    /// French - 639-1: fr
    Fre,
    /// French, Middle (ca.1400-1600)
    Frm,
    /// French, Old (842-ca.1400)
    Fro,
    /// Northern Frisian
    Frr,
    /// Eastern Frisian
    Frs,
    /// Western Frisian - 639-1: fy
    Fry,
    /// Fulah - 639-1: ff
    Ful,
    /// Friulian
    Fur,
    /// Ga
    Gaa,
    /// Gayo
    Gay,
    /// Gbaya
    Gba,
    /// Germanic languages
    Gem,
    /// Georgian - 639-1: ka
    Geo,
    /// German - 639-1: de
    Ger,
    /// Geez
    Gez,
    /// Gilbertese
    Gil,
    /// Gaelic - 639-1: gd
    Gla,
    /// Irish - 639-1: ga
    Gle,
    /// Galician - 639-1: gl
    Glg,
    /// Manx - 639-1: gv
    Glv,
    /// German, Middle High (ca.1050-1500)
    Gmh,
    /// German, Old High (ca.750-1050)
    Goh,
    /// Gondi
    Gon,
    /// Gorontalo
    Gor,
    /// Gothic
    Got,
    /// Grebo
    Grb,
    /// Greek, Ancient (to 1453)
    Grc,
    /// Greek, Modern (1453-) - 639-1: el
    Gre,
    /// Guarani - 639-1: gn
    Grn,
    /// Swiss German
    Gsw,
    /// Gujarati - 639-1: gu
    Guj,
    /// Gwich'in
    Gwi,
    /// Haida
    Hai,
    /// Haitian - 639-1: ht
    Hat,
    /// Hausa - 639-1: ha
    Hau,
    /// Hawaiian
    Haw,
    /// Hebrew - 639-1: he
    Heb,
    /// Herero - 639-1: hz
    Her,
    /// Hiligaynon
    Hil,
    /// Himachali languages
    Him,
    /// Hindi - 639-1: hi
    Hin,
    /// Hittite
    Hit,
    /// Hmong
    Hmn,
    /// Hiri Motu - 639-1: ho
    Hmo,
    /// Croatian - 639-1: hr
    Hrv,
    /// Upper Sorbian
    Hsb,
    /// Hungarian - 639-1: hu
    Hun,
    /// Hupa
    Hup,
    /// Armenian - 639-1: hy
    Hye,
    /// Iban
    Iba,
    /// Igbo - 639-1: ig
    Ibo,
    /// Icelandic - 639-1: is
    Ice,
    /// Ido - 639-1: io
    Ido,
    /// Sichuan Yi - 639-1: ii
    Iii,
    /// Ijo languages
    Ijo,
    /// Inuktitut - 639-1: iu
    Iku,
    /// Interlingue - 639-1: ie
    Ile,
    /// Iloko
    Ilo,
    /// Interlingua (International Auxiliary Language Association) - 639-1: ia
    Ina,
    /// Indic languages
    Inc,
    /// Indonesian - 639-1: id
    Ind,
    /// Indo-European languages
    Ine,
    /// Ingush
    Inh,
    /// Inupiaq - 639-1: ik
    Ipk,
    /// Iranian languages
    Ira,
    /// Iroquoian languages
    Iro,
    /// Icelandic - 639-1: is
    Isl,
    /// Italian - 639-1: it
    Ita,
    /// Javanese - 639-1: jv
    Jav,
    /// Lojban
    Jbo,
    /// Japanese - 639-1: ja
    Jpn,
    /// Judeo-Persian
    Jpr,
    /// Judeo-Arabic
    Jrb,
    /// Kara-Kalpak
    Kaa,
    /// Kabyle
    Kab,
    /// Kachin
    Kac,
    /// Kalaallisut - 639-1: kl
    Kal,
    /// Kamba
    Kam,
    /// Kannada - 639-1: kn
    Kan,
    /// Karen languages
    Kar,
    /// Kashmiri - 639-1: ks
    Kas,
    /// Georgian - 639-1: ka
    Kat,
    /// Kanuri - 639-1: kr
    Kau,
    /// Kawi
    Kaw,
    /// Kazakh - 639-1: kk
    Kaz,
    /// Kabardian
    Kbd,
    /// Khasi
    Kha,
    /// Khoisan languages
    Khi,
    /// Central Khmer - 639-1: km
    Khm,
    /// Khotanese
    Kho,
    /// Kikuyu - 639-1: ki
    Kik,
    /// Kinyarwanda - 639-1: rw
    Kin,
    /// Kirghiz - 639-1: ky
    Kir,
    /// Kimbundu
    Kmb,
    /// Konkani
    Kok,
    /// Komi - 639-1: kv
    Kom,
    /// Kongo - 639-1: kg
    Kon,
    /// Korean - 639-1: ko
    Kor,
    /// Kosraean
    Kos,
    /// Kpelle
    Kpe,
    /// Karachay-Balkar
    Krc,
    /// Karelian
    Krl,
    /// Kru languages
    Kro,
    /// Kurukh
    Kru,
    /// Kuanyama - 639-1: kj
    Kua,
    /// Kumyk
    Kum,
    /// Kurdish - 639-1: ku
    Kur,
    /// Kutenai
    Kut,
    /// Ladino
    Lad,
    /// Lahnda
    Lah,
    /// Lamba
    Lam,
    /// Lao - 639-1: lo
    Lao,
    /// Latin - 639-1: la
    Lat,
    /// Latvian - 639-1: lv
    Lav,
    /// Lezghian
    Lez,
    /// Limburgan - 639-1: li
    Lim,
    /// Lingala - 639-1: ln
    Lin,
    /// Lithuanian - 639-1: lt
    Lit,
    /// Mongo
    Lol,
    /// Lozi
    Loz,
    /// Luxembourgish - 639-1: lb
    Ltz,
    /// Luba-Lulua
    Lua,
    /// Luba-Katanga - 639-1: lu
    Lub,
    /// Ganda - 639-1: lg
    Lug,
    /// Luiseno
    Lui,
    /// Lunda
    Lun,
    /// Luo (Kenya and Tanzania)
    Luo,
    /// Lushai
    Lus,
    /// Macedonian - 639-1: mk
    Mac,
    /// Madurese
    Mad,
    /// Magahi
    Mag,
    /// Marshallese - 639-1: mh
    Mah,
    /// Maithili
    Mai,
    /// Makasar
    Mak,
    /// Malayalam - 639-1: ml
    Mal,
    /// Mandingo
    Man,
    /// Maori - 639-1: mi
    Mao,
    /// Austronesian languages
    Map,
    /// Marathi - 639-1: mr
    Mar,
    /// Masai
    Mas,
    /// Malay - 639-1: ms
    May,
    /// Moksha
    Mdf,
    /// Mandar
    Mdr,
    /// Mende
    Men,
    /// Irish, Middle (900-1200)
    Mga,
    /// Mi'kmaq
    Mic,
    /// Minangkabau
    Min,
    /// Uncoded languages
    Mis,
    /// Macedonian - 639-1: mk
    Mkd,
    /// Mon-Khmer languages
    Mkh,
    /// Malagasy - 639-1: mg
    Mlg,
    /// Maltese - 639-1: mt
    Mlt,
    /// Manchu
    Mnc,
    /// Manipuri
    Mni,
    /// Manobo languages
    Mno,
    /// Mohawk
    Moh,
    /// Mongolian - 639-1: mn
    Mon,
    /// Mossi
    Mos,
    /// Maori - 639-1: mi
    Mri,
    /// Malay - 639-1: ms
    Msa,
    /// Multiple languages
    Mul,
    /// Munda languages
    Mun,
    /// Creek
    Mus,
    /// Mirandese
    Mwl,
    /// Marwari
    Mwr,
    /// Burmese - 639-1: my
    Mya,
    /// Mayan languages
    Myn,
    /// Erzya
    Myv,
    /// Nahuatl languages
    Nah,
    /// North American Indian languages
    Nai,
    /// Neapolitan
    Nap,
    /// Nauru - 639-1: na
    Nau,
    /// Navajo - 639-1: nv
    Nav,
    /// Ndebele, South - 639-1: nr
    Nbl,
    /// Ndebele, North - 639-1: nd
    Nde,
    /// Ndonga - 639-1: ng
    Ndo,
    /// Low German
    Nds,
    /// Nepali - 639-1: ne
    Nep,
    /// Nepal Bhasa
    New,
    /// Nias
    Nia,
    /// Niger-Kordofanian languages
    Nic,
    /// Niuean
    Niu,
    /// Dutch - 639-1: nl
    Nld,
    /// Norwegian Nynorsk - 639-1: nn
    Nno,
    /// Bokmål, Norwegian - 639-1: nb
    Nob,
    /// Nogai
    Nog,
    /// Norse, Old
    Non,
    /// Norwegian - 639-1: no
    Nor,
    /// N'Ko
    Nqo,
    /// Pedi
    Nso,
    /// Nubian languages
    Nub,
    /// Classical Newari
    Nwc,
    /// Chichewa - 639-1: ny
    Nya,
    /// Nyamwezi
    Nym,
    /// Nyankole
    Nyn,
    /// Nyoro
    Nyo,
    /// Nzima
    Nzi,
    /// Occitan (post 1500) - 639-1: oc
    Oci,
    /// Ojibwa - 639-1: oj
    Oji,
    /// Oriya - 639-1: or
    Ori,
    /// Oromo - 639-1: om
    Orm,
    /// Osage
    Osa,
    /// Ossetian - 639-1: os
    Oss,
    /// Turkish, Ottoman (1500-1928)
    Ota,
    /// Otomian languages
    Oto,
    /// Papuan languages
    Paa,
    /// Pangasinan
    Pag,
    /// Pahlavi
    Pal,
    /// Pampanga
    Pam,
    /// Panjabi - 639-1: pa
    Pan,
    /// Papiamento
    Pap,
    /// Palauan
    Pau,
    /// Persian, Old (ca.600-400 B.C.)
    Peo,
    /// Persian - 639-1: fa
    Per,
    /// Philippine languages
    Phi,
    /// Phoenician
    Phn,
    /// Pali - 639-1: pi
    Pli,
    /// Polish - 639-1: pl
    Pol,
    /// Pohnpeian
    Pon,
    /// Portuguese - 639-1: pt
    Por,
    /// Prakrit languages
    Pra,
    /// Provençal, Old (to 1500)
    Pro,
    /// Pushto - 639-1: ps
    Pus,
    /// Quechua - 639-1: qu
    Que,
    /// Rajasthani
    Raj,
    /// Rapanui
    Rap,
    /// Rarotongan
    Rar,
    /// Romance languages
    Roa,
    /// Romansh - 639-1: rm
    Roh,
    /// Romany
    Rom,
    /// Romanian - 639-1: ro
    Ron,
    /// Romanian - 639-1: ro
    Rum,
    /// Rundi - 639-1: rn
    Run,
    /// Aromanian
    Rup,
    /// Russian - 639-1: ru
    Rus,
    /// Sandawe
    Sad,
    /// Sango - 639-1: sg
    Sag,
    /// Yakut
    Sah,
    /// South American Indian languages
    Sai,
    /// Salishan languages
    Sal,
    /// Samaritan Aramaic
    Sam,
    /// Sanskrit - 639-1: sa
    San,
    /// Sasak
    Sas,
    /// Santali
    Sat,
    /// Sicilian
    Scn,
    /// Scots
    Sco,
    /// Selkup
    Sel,
    /// Semitic languages
    Sem,
    /// Irish, Old (to 900)
    Sga,
    /// Sign Languages
    Sgn,
    /// Shan
    Shn,
    /// Sidamo
    Sid,
    /// Sinhala - 639-1: si
    Sin,
    /// Siouan languages
    Sio,
    /// Sino-Tibetan languages
    Sit,
    /// Slavic languages
    Sla,
    /// Slovak - 639-1: sk
    Slk,
    /// Slovak - 639-1: sk
    Slo,
    /// Slovenian - 639-1: sl
    Slv,
    /// Southern Sami
    Sma,
    /// Northern Sami - 639-1: se
    Sme,
    /// Sami languages
    Smi,
    /// Lule Sami
    Smj,
    /// Inari Sami
    Smn,
    /// Samoan - 639-1: sm
    Smo,
    /// Skolt Sami
    Sms,
    /// Shona - 639-1: sn
    Sna,
    /// Sindhi - 639-1: sd
    Snd,
    /// Soninke
    Snk,
    /// Sogdian
    Sog,
    /// Somali - 639-1: so
    Som,
    /// Songhai languages
    Son,
    /// Sotho, Southern - 639-1: st
    Sot,
    /// Spanish - 639-1: es
    Spa,
    /// Albanian - 639-1: sq
    Sqi,
    /// Sardinian - 639-1: sc
    Srd,
    /// Sranan Tongo
    Srn,
    /// Serbian - 639-1: sr
    Srp,
    /// Serer
    Srr,
    /// Nilo-Saharan languages
    Ssa,
    /// Swati - 639-1: ss
    Ssw,
    /// Sukuma
    Suk,
    /// Sundanese - 639-1: su
    Sun,
    /// Susu
    Sus,
    /// Sumerian
    Sux,
    /// Swahili - 639-1: sw
    Swa,
    /// Swedish - 639-1: sv
    Swe,
    /// Classical Syriac
    Syc,
    /// Syriac
    Syr,
    /// Tahitian - 639-1: ty
    Tah,
    /// Tai languages
    Tai,
    /// Tamil - 639-1: ta
    Tam,
    /// Tatar - 639-1: tt
    Tat,
    /// Telugu - 639-1: te
    Tel,
    /// Timne
    Tem,
    /// Tereno
    Ter,
    /// Tetum
    Tet,
    /// Tajik - 639-1: tg
    Tgk,
    /// Tagalog - 639-1: tl
    Tgl,
    /// Thai - 639-1: th
    Tha,
    /// Tibetan - 639-1: bo
    Tib,
    /// Tigre
    Tig,
    /// Tigrinya - 639-1: ti
    Tir,
    /// Tiv
    Tiv,
    /// Tokelau
    Tkl,
    /// Klingon
    Tlh,
    /// Tlingit
    Tli,
    /// Tamashek
    Tmh,
    /// Tonga (Nyasa)
    Tog,
    /// Tonga (Tonga Islands) - 639-1: to
    Ton,
    /// Tok Pisin
    Tpi,
    /// Tsimshian
    Tsi,
    /// Tswana - 639-1: tn
    Tsn,
    /// Tsonga - 639-1: ts
    Tso,
    /// Turkmen - 639-1: tk
    Tuk,
    /// Tumbuka
    Tum,
    /// Tupi languages
    Tup,
    /// Turkish - 639-1: tr
    Tur,
    /// Altaic languages
    Tut,
    /// Tuvalu
    Tvl,
    /// Twi - 639-1: tw
    Twi,
    /// Tuvinian
    Tyv,
    /// Udmurt
    Udm,
    /// Ugaritic
    Uga,
    /// Uighur - 639-1: ug
    Uig,
    /// Ukrainian - 639-1: uk
    Ukr,
    /// Umbundu
    Umb,
    /// Undetermined
    Und,
    /// Urdu - 639-1: ur
    Urd,
    /// Uzbek - 639-1: uz
    Uzb,
    /// Vai
    Vai,
    /// Venda - 639-1: ve
    Ven,
    /// Vietnamese - 639-1: vi
    Vie,
    /// Volapük - 639-1: vo
    Vol,
    /// Votic
    Vot,
    /// Wakashan languages
    Wak,
    /// Wolaitta
    Wal,
    /// Waray
    War,
    /// Washo
    Was,
    /// Welsh - 639-1: cy
    Wel,
    /// Sorbian languages
    Wen,
    /// Walloon - 639-1: wa
    Wln,
    /// Wolof - 639-1: wo
    Wol,
    /// Kalmyk
    Xal,
    /// Xhosa - 639-1: xh
    Xho,
    /// Yao
    Yao,
    /// Yapese
    Yap,
    /// Yiddish - 639-1: yi
    Yid,
    /// Yoruba - 639-1: yo
    Yor,
    /// Yupik languages
    Ypk,
    /// Zapotec
    Zap,
    /// Blissymbols
    Zbl,
    /// Zenaga
    Zen,
    /// Standard Moroccan Tamazight
    Zgh,
    /// Zhuang - 639-1: za
    Zha,
    /// Chinese - 639-1: zh
    Zho,
    /// Zande languages
    Znd,
    /// Zulu - 639-1: zu
    Zul,
    /// Zuni
    Zun,
    /// No linguistic content
    Zxx,
    /// Zaza
    Zza,
}
