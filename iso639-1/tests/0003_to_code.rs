extern crate iso639_1;
use iso639_1::get_code_iso639_3;

#[test]
#[should_panic]
fn get_iso639v3_0001_not_match() {
    assert!(get_code_iso639_3("xxxx") == "fra");
}

#[test]
fn get_iso639v3_0002_match() {
        assert!(get_code_iso639_3("aa") == "aar");
        assert!(get_code_iso639_3("ab") == "abk");
        assert!(get_code_iso639_3("ae") == "ave");
        assert!(get_code_iso639_3("af") == "afr");
        assert!(get_code_iso639_3("ak") == "aka");
        assert!(get_code_iso639_3("am") == "amh");
        assert!(get_code_iso639_3("an") == "arg");
        assert!(get_code_iso639_3("ar") == "ara");
        assert!(get_code_iso639_3("as") == "asm");
        assert!(get_code_iso639_3("av") == "ava");
        assert!(get_code_iso639_3("ay") == "aym");
        assert!(get_code_iso639_3("az") == "aze");
        assert!(get_code_iso639_3("ba") == "bak");
        assert!(get_code_iso639_3("be") == "bel");
        assert!(get_code_iso639_3("bg") == "bul");
        assert!(get_code_iso639_3("bh") == "bih");
        assert!(get_code_iso639_3("bi") == "bis");
        assert!(get_code_iso639_3("bm") == "bam");
        assert!(get_code_iso639_3("bn") == "ben");
        assert!(get_code_iso639_3("bo") == "bod");
        assert!(get_code_iso639_3("br") == "bre");
        assert!(get_code_iso639_3("bs") == "bos");
        assert!(get_code_iso639_3("ca") == "cat");
        assert!(get_code_iso639_3("ce") == "che");
        assert!(get_code_iso639_3("ch") == "cha");
        assert!(get_code_iso639_3("co") == "cos");
        assert!(get_code_iso639_3("cr") == "cre");
        assert!(get_code_iso639_3("cs") == "ces");
        assert!(get_code_iso639_3("cu") == "chu");
        assert!(get_code_iso639_3("cv") == "chv");
        assert!(get_code_iso639_3("cy") == "cym");
        assert!(get_code_iso639_3("da") == "dan");
        assert!(get_code_iso639_3("de") == "deu");
        assert!(get_code_iso639_3("dv") == "div");
        assert!(get_code_iso639_3("dz") == "dzo");
        assert!(get_code_iso639_3("ee") == "ewe");
        assert!(get_code_iso639_3("el") == "ell");
        assert!(get_code_iso639_3("en") == "eng");
        assert!(get_code_iso639_3("eo") == "epo");
        assert!(get_code_iso639_3("es") == "spa");
        assert!(get_code_iso639_3("et") == "est");
        assert!(get_code_iso639_3("eu") == "eus");
        assert!(get_code_iso639_3("fa") == "fas");
        assert!(get_code_iso639_3("ff") == "ful");
        assert!(get_code_iso639_3("fi") == "fin");
        assert!(get_code_iso639_3("fj") == "fij");
        assert!(get_code_iso639_3("fo") == "fao");
        assert!(get_code_iso639_3("fr") == "fra");
        assert!(get_code_iso639_3("fy") == "fry");
        assert!(get_code_iso639_3("ga") == "gle");
        assert!(get_code_iso639_3("gd") == "gla");
        assert!(get_code_iso639_3("gl") == "glg");
        assert!(get_code_iso639_3("gn") == "grn");
        assert!(get_code_iso639_3("gu") == "guj");
        assert!(get_code_iso639_3("gv") == "glv");
        assert!(get_code_iso639_3("ha") == "hau");
        assert!(get_code_iso639_3("he") == "heb");
        assert!(get_code_iso639_3("hi") == "hin");
        assert!(get_code_iso639_3("ho") == "hmo");
        assert!(get_code_iso639_3("hr") == "hrv");
        assert!(get_code_iso639_3("ht") == "hat");
        assert!(get_code_iso639_3("hu") == "hun");
        assert!(get_code_iso639_3("hy") == "hye");
        assert!(get_code_iso639_3("hz") == "her");
        assert!(get_code_iso639_3("ia") == "ina");
        assert!(get_code_iso639_3("id") == "ind");
        assert!(get_code_iso639_3("ie") == "ile");
        assert!(get_code_iso639_3("ig") == "ibo");
        assert!(get_code_iso639_3("ii") == "iii");
        assert!(get_code_iso639_3("ik") == "ipk");
        assert!(get_code_iso639_3("io") == "ido");
        assert!(get_code_iso639_3("is") == "isl");
        assert!(get_code_iso639_3("it") == "ita");
        assert!(get_code_iso639_3("iu") == "iku");
        assert!(get_code_iso639_3("ja") == "jpn");
        assert!(get_code_iso639_3("jv") == "jav");
        assert!(get_code_iso639_3("ka") == "kat");
        assert!(get_code_iso639_3("kg") == "kon");
        assert!(get_code_iso639_3("ki") == "kik");
        assert!(get_code_iso639_3("kj") == "kua");
        assert!(get_code_iso639_3("kk") == "kaz");
        assert!(get_code_iso639_3("kl") == "kal");
        assert!(get_code_iso639_3("km") == "khm");
        assert!(get_code_iso639_3("kn") == "kan");
        assert!(get_code_iso639_3("ko") == "kor");
        assert!(get_code_iso639_3("kr") == "kau");
        assert!(get_code_iso639_3("ks") == "kas");
        assert!(get_code_iso639_3("ku") == "kur");
        assert!(get_code_iso639_3("kv") == "kom");
        assert!(get_code_iso639_3("kw") == "cor");
        assert!(get_code_iso639_3("ky") == "kir");
        assert!(get_code_iso639_3("la") == "lat");
        assert!(get_code_iso639_3("lb") == "ltz");
        assert!(get_code_iso639_3("lg") == "lug");
        assert!(get_code_iso639_3("li") == "lim");
        assert!(get_code_iso639_3("ln") == "lin");
        assert!(get_code_iso639_3("lo") == "lao");
        assert!(get_code_iso639_3("lt") == "lit");
        assert!(get_code_iso639_3("lu") == "lub");
        assert!(get_code_iso639_3("lv") == "lav");
        assert!(get_code_iso639_3("mg") == "mlg");
        assert!(get_code_iso639_3("mh") == "mah");
        assert!(get_code_iso639_3("mi") == "mri");
        assert!(get_code_iso639_3("mk") == "mkd");
        assert!(get_code_iso639_3("ml") == "mal");
        assert!(get_code_iso639_3("mn") == "mon");
        assert!(get_code_iso639_3("mr") == "mar");
        assert!(get_code_iso639_3("ms") == "msa");
        assert!(get_code_iso639_3("mt") == "mlt");
        assert!(get_code_iso639_3("my") == "mya");
        assert!(get_code_iso639_3("na") == "nau");
        assert!(get_code_iso639_3("nb") == "nob");
        assert!(get_code_iso639_3("nd") == "nde");
        assert!(get_code_iso639_3("ne") == "nep");
        assert!(get_code_iso639_3("ng") == "ndo");
        assert!(get_code_iso639_3("nl") == "nld");
        assert!(get_code_iso639_3("nn") == "nno");
        assert!(get_code_iso639_3("no") == "nor");
        assert!(get_code_iso639_3("nr") == "nbl");
        assert!(get_code_iso639_3("nv") == "nav");
        assert!(get_code_iso639_3("ny") == "nya");
        assert!(get_code_iso639_3("oc") == "oci");
        assert!(get_code_iso639_3("oj") == "oji");
        assert!(get_code_iso639_3("om") == "orm");
        assert!(get_code_iso639_3("or") == "ori");
        assert!(get_code_iso639_3("os") == "oss");
        assert!(get_code_iso639_3("pa") == "pan");
        assert!(get_code_iso639_3("pi") == "pli");
        assert!(get_code_iso639_3("pl") == "pol");
        assert!(get_code_iso639_3("ps") == "pus");
        assert!(get_code_iso639_3("pt") == "por");
        assert!(get_code_iso639_3("qu") == "que");
        assert!(get_code_iso639_3("rm") == "roh");
        assert!(get_code_iso639_3("rn") == "run");
        assert!(get_code_iso639_3("ro") == "ron");
        assert!(get_code_iso639_3("ru") == "rus");
        assert!(get_code_iso639_3("rw") == "kin");
        assert!(get_code_iso639_3("sa") == "san");
        assert!(get_code_iso639_3("sc") == "srd");
        assert!(get_code_iso639_3("sd") == "snd");
        assert!(get_code_iso639_3("se") == "sme");
        assert!(get_code_iso639_3("sg") == "sag");
        assert!(get_code_iso639_3("si") == "sin");
        assert!(get_code_iso639_3("sk") == "slk");
        assert!(get_code_iso639_3("sl") == "slv");
        assert!(get_code_iso639_3("sm") == "smo");
        assert!(get_code_iso639_3("sn") == "sna");
        assert!(get_code_iso639_3("so") == "som");
        assert!(get_code_iso639_3("sq") == "sqi");
        assert!(get_code_iso639_3("sr") == "srp");
        assert!(get_code_iso639_3("ss") == "ssw");
        assert!(get_code_iso639_3("st") == "sot");
        assert!(get_code_iso639_3("su") == "sun");
        assert!(get_code_iso639_3("sv") == "swe");
        assert!(get_code_iso639_3("sw") == "swa");
        assert!(get_code_iso639_3("ta") == "tam");
        assert!(get_code_iso639_3("te") == "tel");
        assert!(get_code_iso639_3("tg") == "tgk");
        assert!(get_code_iso639_3("th") == "tha");
        assert!(get_code_iso639_3("ti") == "tir");
        assert!(get_code_iso639_3("tk") == "tuk");
        assert!(get_code_iso639_3("tl") == "tgl");
        assert!(get_code_iso639_3("tn") == "tsn");
        assert!(get_code_iso639_3("to") == "ton");
        assert!(get_code_iso639_3("tr") == "tur");
        assert!(get_code_iso639_3("ts") == "tso");
        assert!(get_code_iso639_3("tt") == "tat");
        assert!(get_code_iso639_3("tw") == "twi");
        assert!(get_code_iso639_3("ty") == "tah");
        assert!(get_code_iso639_3("ug") == "uig");
        assert!(get_code_iso639_3("uk") == "ukr");
        assert!(get_code_iso639_3("ur") == "urd");
        assert!(get_code_iso639_3("uz") == "uzb");
        assert!(get_code_iso639_3("ve") == "ven");
        assert!(get_code_iso639_3("vi") == "vie");
        assert!(get_code_iso639_3("vo") == "vol");
        assert!(get_code_iso639_3("wa") == "wln");
        assert!(get_code_iso639_3("wo") == "wol");
        assert!(get_code_iso639_3("xh") == "xho");
        assert!(get_code_iso639_3("yi") == "yid");
        assert!(get_code_iso639_3("yo") == "yor");
        assert!(get_code_iso639_3("za") == "zha");
        assert!(get_code_iso639_3("zh") == "zho");
        assert!(get_code_iso639_3("zu") == "zul");
}