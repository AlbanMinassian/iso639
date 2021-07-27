extern crate iso639_1;
use iso639_1::{Iso639_1, Iso639v1Error, Iso639v1ErrorKind};

#[test]
fn enum_0001_not_equal() {
    assert!(Iso639_1::Fr != Iso639_1::En);
}

#[test]
fn enum_0002_equal() {
    assert!(Iso639_1::Aa == Iso639_1::Aa);
    assert!(Iso639_1::Ab == Iso639_1::Ab);
    assert!(Iso639_1::Ae == Iso639_1::Ae);
    assert!(Iso639_1::Af == Iso639_1::Af);
    assert!(Iso639_1::Ak == Iso639_1::Ak);
    assert!(Iso639_1::Am == Iso639_1::Am);
    assert!(Iso639_1::An == Iso639_1::An);
    assert!(Iso639_1::Ar == Iso639_1::Ar);
    assert!(Iso639_1::As == Iso639_1::As);
    assert!(Iso639_1::Av == Iso639_1::Av);
    assert!(Iso639_1::Ay == Iso639_1::Ay);
    assert!(Iso639_1::Az == Iso639_1::Az);
    assert!(Iso639_1::Ba == Iso639_1::Ba);
    assert!(Iso639_1::Be == Iso639_1::Be);
    assert!(Iso639_1::Bg == Iso639_1::Bg);
    assert!(Iso639_1::Bh == Iso639_1::Bh);
    assert!(Iso639_1::Bi == Iso639_1::Bi);
    assert!(Iso639_1::Bm == Iso639_1::Bm);
    assert!(Iso639_1::Bn == Iso639_1::Bn);
    assert!(Iso639_1::Bo == Iso639_1::Bo);
    assert!(Iso639_1::Br == Iso639_1::Br);
    assert!(Iso639_1::Bs == Iso639_1::Bs);
    assert!(Iso639_1::Ca == Iso639_1::Ca);
    assert!(Iso639_1::Ce == Iso639_1::Ce);
    assert!(Iso639_1::Ch == Iso639_1::Ch);
    assert!(Iso639_1::Co == Iso639_1::Co);
    assert!(Iso639_1::Cr == Iso639_1::Cr);
    assert!(Iso639_1::Cs == Iso639_1::Cs);
    assert!(Iso639_1::Cu == Iso639_1::Cu);
    assert!(Iso639_1::Cv == Iso639_1::Cv);
    assert!(Iso639_1::Cy == Iso639_1::Cy);
    assert!(Iso639_1::Da == Iso639_1::Da);
    assert!(Iso639_1::De == Iso639_1::De);
    assert!(Iso639_1::Dv == Iso639_1::Dv);
    assert!(Iso639_1::Dz == Iso639_1::Dz);
    assert!(Iso639_1::Ee == Iso639_1::Ee);
    assert!(Iso639_1::El == Iso639_1::El);
    assert!(Iso639_1::En == Iso639_1::En);
    assert!(Iso639_1::Eo == Iso639_1::Eo);
    assert!(Iso639_1::Es == Iso639_1::Es);
    assert!(Iso639_1::Et == Iso639_1::Et);
    assert!(Iso639_1::Eu == Iso639_1::Eu);
    assert!(Iso639_1::Fa == Iso639_1::Fa);
    assert!(Iso639_1::Ff == Iso639_1::Ff);
    assert!(Iso639_1::Fi == Iso639_1::Fi);
    assert!(Iso639_1::Fj == Iso639_1::Fj);
    assert!(Iso639_1::Fo == Iso639_1::Fo);
    assert!(Iso639_1::Fr == Iso639_1::Fr);
    assert!(Iso639_1::Fy == Iso639_1::Fy);
    assert!(Iso639_1::Ga == Iso639_1::Ga);
    assert!(Iso639_1::Gd == Iso639_1::Gd);
    assert!(Iso639_1::Gl == Iso639_1::Gl);
    assert!(Iso639_1::Gn == Iso639_1::Gn);
    assert!(Iso639_1::Gu == Iso639_1::Gu);
    assert!(Iso639_1::Gv == Iso639_1::Gv);
    assert!(Iso639_1::Ha == Iso639_1::Ha);
    assert!(Iso639_1::He == Iso639_1::He);
    assert!(Iso639_1::Hi == Iso639_1::Hi);
    assert!(Iso639_1::Ho == Iso639_1::Ho);
    assert!(Iso639_1::Hr == Iso639_1::Hr);
    assert!(Iso639_1::Ht == Iso639_1::Ht);
    assert!(Iso639_1::Hu == Iso639_1::Hu);
    assert!(Iso639_1::Hy == Iso639_1::Hy);
    assert!(Iso639_1::Hz == Iso639_1::Hz);
    assert!(Iso639_1::Ia == Iso639_1::Ia);
    assert!(Iso639_1::Id == Iso639_1::Id);
    assert!(Iso639_1::Ie == Iso639_1::Ie);
    assert!(Iso639_1::Ig == Iso639_1::Ig);
    assert!(Iso639_1::Ii == Iso639_1::Ii);
    assert!(Iso639_1::Ik == Iso639_1::Ik);
    assert!(Iso639_1::Io == Iso639_1::Io);
    assert!(Iso639_1::Is == Iso639_1::Is);
    assert!(Iso639_1::It == Iso639_1::It);
    assert!(Iso639_1::Iu == Iso639_1::Iu);
    assert!(Iso639_1::Ja == Iso639_1::Ja);
    assert!(Iso639_1::Jv == Iso639_1::Jv);
    assert!(Iso639_1::Ka == Iso639_1::Ka);
    assert!(Iso639_1::Kg == Iso639_1::Kg);
    assert!(Iso639_1::Ki == Iso639_1::Ki);
    assert!(Iso639_1::Kj == Iso639_1::Kj);
    assert!(Iso639_1::Kk == Iso639_1::Kk);
    assert!(Iso639_1::Kl == Iso639_1::Kl);
    assert!(Iso639_1::Km == Iso639_1::Km);
    assert!(Iso639_1::Kn == Iso639_1::Kn);
    assert!(Iso639_1::Ko == Iso639_1::Ko);
    assert!(Iso639_1::Kr == Iso639_1::Kr);
    assert!(Iso639_1::Ks == Iso639_1::Ks);
    assert!(Iso639_1::Ku == Iso639_1::Ku);
    assert!(Iso639_1::Kv == Iso639_1::Kv);
    assert!(Iso639_1::Kw == Iso639_1::Kw);
    assert!(Iso639_1::Ky == Iso639_1::Ky);
    assert!(Iso639_1::La == Iso639_1::La);
    assert!(Iso639_1::Lb == Iso639_1::Lb);
    assert!(Iso639_1::Lg == Iso639_1::Lg);
    assert!(Iso639_1::Li == Iso639_1::Li);
    assert!(Iso639_1::Ln == Iso639_1::Ln);
    assert!(Iso639_1::Lo == Iso639_1::Lo);
    assert!(Iso639_1::Lt == Iso639_1::Lt);
    assert!(Iso639_1::Lu == Iso639_1::Lu);
    assert!(Iso639_1::Lv == Iso639_1::Lv);
    assert!(Iso639_1::Mg == Iso639_1::Mg);
    assert!(Iso639_1::Mh == Iso639_1::Mh);
    assert!(Iso639_1::Mi == Iso639_1::Mi);
    assert!(Iso639_1::Mk == Iso639_1::Mk);
    assert!(Iso639_1::Ml == Iso639_1::Ml);
    assert!(Iso639_1::Mn == Iso639_1::Mn);
    assert!(Iso639_1::Mr == Iso639_1::Mr);
    assert!(Iso639_1::Ms == Iso639_1::Ms);
    assert!(Iso639_1::Mt == Iso639_1::Mt);
    assert!(Iso639_1::My == Iso639_1::My);
    assert!(Iso639_1::Na == Iso639_1::Na);
    assert!(Iso639_1::Nb == Iso639_1::Nb);
    assert!(Iso639_1::Nd == Iso639_1::Nd);
    assert!(Iso639_1::Ne == Iso639_1::Ne);
    assert!(Iso639_1::Ng == Iso639_1::Ng);
    assert!(Iso639_1::Nl == Iso639_1::Nl);
    assert!(Iso639_1::Nn == Iso639_1::Nn);
    assert!(Iso639_1::No == Iso639_1::No);
    assert!(Iso639_1::Nr == Iso639_1::Nr);
    assert!(Iso639_1::Nv == Iso639_1::Nv);
    assert!(Iso639_1::Ny == Iso639_1::Ny);
    assert!(Iso639_1::Oc == Iso639_1::Oc);
    assert!(Iso639_1::Oj == Iso639_1::Oj);
    assert!(Iso639_1::Om == Iso639_1::Om);
    assert!(Iso639_1::Or == Iso639_1::Or);
    assert!(Iso639_1::Os == Iso639_1::Os);
    assert!(Iso639_1::Pa == Iso639_1::Pa);
    assert!(Iso639_1::Pi == Iso639_1::Pi);
    assert!(Iso639_1::Pl == Iso639_1::Pl);
    assert!(Iso639_1::Ps == Iso639_1::Ps);
    assert!(Iso639_1::Pt == Iso639_1::Pt);
    assert!(Iso639_1::Qu == Iso639_1::Qu);
    assert!(Iso639_1::Rm == Iso639_1::Rm);
    assert!(Iso639_1::Rn == Iso639_1::Rn);
    assert!(Iso639_1::Ro == Iso639_1::Ro);
    assert!(Iso639_1::Ru == Iso639_1::Ru);
    assert!(Iso639_1::Rw == Iso639_1::Rw);
    assert!(Iso639_1::Sa == Iso639_1::Sa);
    assert!(Iso639_1::Sc == Iso639_1::Sc);
    assert!(Iso639_1::Sd == Iso639_1::Sd);
    assert!(Iso639_1::Se == Iso639_1::Se);
    assert!(Iso639_1::Sg == Iso639_1::Sg);
    assert!(Iso639_1::Si == Iso639_1::Si);
    assert!(Iso639_1::Sk == Iso639_1::Sk);
    assert!(Iso639_1::Sl == Iso639_1::Sl);
    assert!(Iso639_1::Sm == Iso639_1::Sm);
    assert!(Iso639_1::Sn == Iso639_1::Sn);
    assert!(Iso639_1::So == Iso639_1::So);
    assert!(Iso639_1::Sq == Iso639_1::Sq);
    assert!(Iso639_1::Sr == Iso639_1::Sr);
    assert!(Iso639_1::Ss == Iso639_1::Ss);
    assert!(Iso639_1::St == Iso639_1::St);
    assert!(Iso639_1::Su == Iso639_1::Su);
    assert!(Iso639_1::Sv == Iso639_1::Sv);
    assert!(Iso639_1::Sw == Iso639_1::Sw);
    assert!(Iso639_1::Ta == Iso639_1::Ta);
    assert!(Iso639_1::Te == Iso639_1::Te);
    assert!(Iso639_1::Tg == Iso639_1::Tg);
    assert!(Iso639_1::Th == Iso639_1::Th);
    assert!(Iso639_1::Ti == Iso639_1::Ti);
    assert!(Iso639_1::Tk == Iso639_1::Tk);
    assert!(Iso639_1::Tl == Iso639_1::Tl);
    assert!(Iso639_1::Tn == Iso639_1::Tn);
    assert!(Iso639_1::To == Iso639_1::To);
    assert!(Iso639_1::Tr == Iso639_1::Tr);
    assert!(Iso639_1::Ts == Iso639_1::Ts);
    assert!(Iso639_1::Tt == Iso639_1::Tt);
    assert!(Iso639_1::Tw == Iso639_1::Tw);
    assert!(Iso639_1::Ty == Iso639_1::Ty);
    assert!(Iso639_1::Ug == Iso639_1::Ug);
    assert!(Iso639_1::Uk == Iso639_1::Uk);
    assert!(Iso639_1::Ur == Iso639_1::Ur);
    assert!(Iso639_1::Uz == Iso639_1::Uz);
    assert!(Iso639_1::Ve == Iso639_1::Ve);
    assert!(Iso639_1::Vi == Iso639_1::Vi);
    assert!(Iso639_1::Vo == Iso639_1::Vo);
    assert!(Iso639_1::Wa == Iso639_1::Wa);
    assert!(Iso639_1::Wo == Iso639_1::Wo);
    assert!(Iso639_1::Xh == Iso639_1::Xh);
    assert!(Iso639_1::Yi == Iso639_1::Yi);
    assert!(Iso639_1::Yo == Iso639_1::Yo);
    assert!(Iso639_1::Za == Iso639_1::Za);
    assert!(Iso639_1::Zh == Iso639_1::Zh);
    assert!(Iso639_1::Zu == Iso639_1::Zu);
}

#[test]
fn enum_0003_no_panic() {
    println!("{:?}", Iso639_1::En)
}

#[test]
fn enum_0004_display_error() {
    // and increase coverage :)
    assert_eq!(
        "not found frite".to_string(),
        format!(
            "{}",
            Iso639v1Error::from(Iso639v1ErrorKind::NotFoundFrom("frite".to_string()))
        )
    );
    assert_eq!(
        "not found frite".to_string(),
        format!(
            "{}",
            Iso639v1Error::from(Iso639v1ErrorKind::NotFoundTo("frite".to_string()))
        )
    );
}
