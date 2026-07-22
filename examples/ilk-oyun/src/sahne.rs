use tgame::onsoz::{
    AraziUreteci, Donusum3B, Dunya, FizikDunyasi, FizikGovdesi, Kamera3B, MalzemeKimligi,
    MalzemeVerisi, MeshKimligi, ModelVerisi, OyunHatasi, OyunSonucu, Renk, Varlik,
    VarlikKimligi, Vektor3, koni, kure, silindir,
};

pub(crate) const OYUNCU_OLCEGI: Vektor3 = Vektor3::yeni(0.75, 0.75, 0.75);

pub(crate) struct SahneKurulumu {
    pub(crate) dunya: Dunya,
    pub(crate) fizik: FizikDunyasi,
    pub(crate) oyuncu: VarlikKimligi,
    pub(crate) merkez: VarlikKimligi,
    pub(crate) piramitler: Vec<VarlikKimligi>,
}

pub(crate) fn sahneyi_olustur() -> OyunSonucu<SahneKurulumu> {
    let mut dunya = Dunya::yeni_3b();
    let mut fizik = FizikDunyasi::yeni();
    dunya.kamera3b_ayarla(
        Kamera3B::yeni()
            .konum(Vektor3::yeni(0.0, 4.8, 8.5))
            .hedef(Vektor3::yeni(0.0, 0.5, 0.0))
            .kirpma(0.1, 250.0),
    );

    araziyi_ekle(&mut dunya, &mut fizik)?;
    dogal_cevreyi_ekle(&mut dunya, &mut fizik)?;
    macera_isaretlerini_ekle(&mut dunya)?;

    let piramit_mesh = piramit_meshini_yukle(&mut dunya)?;
    let alternatif_malzeme =
        dunya.malzeme_ekle(MalzemeVerisi::yeni(Renk::yeni(0.08, 0.85, 1.0, 1.0)));
    let piramitler = piramitleri_ekle(&mut dunya, &mut fizik, piramit_mesh, alternatif_malzeme);
    gorunurluk_stres_sahnesi_ekle(&mut dunya, piramit_mesh);

    let oyuncu = dunya.varlik_ekle(
        Varlik::kup("Oyuncu", Renk::SARI).donusum3b(
            Donusum3B::yeni()
                .konum(Vektor3::yeni(0.0, 2.5, 3.0))
                .olcek(OYUNCU_OLCEGI),
        ),
    );
    fizik.govde_ekle(FizikGovdesi::dinamik_kup(oyuncu, OYUNCU_OLCEGI));

    let cekirdek_mesh = dunya.mesh_ekle(kure(
        20,
        12,
        1.0,
        0.08,
        MalzemeVerisi::yeni(Renk::yeni(0.85, 0.10, 0.08, 1.0)),
    )?);
    let merkez_olcegi = Vektor3::yeni(1.4, 1.4, 1.4);
    let merkez = dunya.varlik_ekle(
        Varlik::mesh("Dönen Tapınak Çekirdeği", cekirdek_mesh, Renk::BEYAZ).donusum3b(
            Donusum3B::yeni()
                .konum(Vektor3::yeni(0.0, 1.0, 0.0))
                .olcek(merkez_olcegi)
                .donus(Vektor3::yeni(0.25, 0.35, 0.1)),
        ),
    );
    fizik.govde_ekle(FizikGovdesi::statik_kup(
        merkez,
        Vektor3::yeni(1.2, 1.2, 1.2),
    ));

    Ok(SahneKurulumu {
        dunya,
        fizik,
        oyuncu,
        merkez,
        piramitler,
    })
}

fn araziyi_ekle(dunya: &mut Dunya, fizik: &mut FizikDunyasi) -> OyunSonucu {
    let arazi = AraziUreteci::fonksiyondan(41, 41, 0.65, |x, z| {
        let merkeze_uzaklik = (x * x + z * z).sqrt();
        let kenar_etkisi = ((merkeze_uzaklik - 4.5) / 8.0).clamp(0.0, 1.0);
        let dalga = (x * 0.31).sin() * 0.28 + (z * 0.24).cos() * 0.22;
        let patika = (-x.abs() * 0.08).max(-0.28);
        -0.52 + dalga * kenar_etkisi + patika * kenar_etkisi
    })?;
    let arazi_mesh = dunya.mesh_ekle(arazi.mesh_uret(MalzemeVerisi::yeni(Renk::yeni(
        0.16, 0.36, 0.18, 1.0,
    )))?);
    dunya.varlik_ekle(Varlik::mesh("Sisli Köy Arazisi", arazi_mesh, Renk::BEYAZ));

    let taban_olcegi = Vektor3::yeni(13.0, 0.18, 13.0);
    let taban = dunya.varlik_ekle(
        Varlik::yeni("Arazi Fizik Tabanı")
            .donusum3b(Donusum3B::yeni().konum(Vektor3::yeni(0.0, -0.78, 0.0))),
    );
    fizik.govde_ekle(FizikGovdesi::statik_kup(taban, taban_olcegi));
    Ok(())
}

fn dogal_cevreyi_ekle(dunya: &mut Dunya, fizik: &mut FizikDunyasi) -> OyunSonucu {
    let govde_mesh = dunya.mesh_ekle(silindir(
        10,
        0.24,
        2.2,
        MalzemeVerisi::yeni(Renk::yeni(0.30, 0.16, 0.07, 1.0)),
    )?);
    let tac_mesh = dunya.mesh_ekle(koni(
        12,
        1.05,
        2.7,
        MalzemeVerisi::yeni(Renk::yeni(0.08, 0.32, 0.13, 1.0)),
    )?);
    let kaya_mesh = dunya.mesh_ekle(kure(
        10,
        6,
        0.8,
        0.28,
        MalzemeVerisi::yeni(Renk::yeni(0.30, 0.33, 0.36, 1.0)),
    )?);

    let agaclar = [
        (-7.5, -6.5, 1.0),
        (-5.6, -8.1, 0.8),
        (-8.4, -1.5, 1.2),
        (-7.6, 4.6, 0.9),
        (-5.8, 7.4, 1.1),
        (6.6, -7.2, 1.0),
        (8.2, -3.0, 1.2),
        (7.5, 2.8, 0.85),
        (6.2, 7.2, 1.15),
        (3.8, 8.4, 0.9),
        (-2.8, 8.7, 1.05),
        (9.0, 6.0, 1.0),
    ];
    for (sira, (x, z, olcek)) in agaclar.into_iter().enumerate() {
        let govde_konumu = Vektor3::yeni(x, 0.45 * olcek, z);
        dunya.varlik_ekle(
            Varlik::mesh(format!("Ağaç Gövdesi {}", sira + 1), govde_mesh, Renk::BEYAZ)
                .donusum3b(
                    Donusum3B::yeni()
                        .konum(govde_konumu)
                        .olcek(Vektor3::yeni(olcek, olcek, olcek)),
                ),
        );
        dunya.varlik_ekle(
            Varlik::mesh(format!("Ağaç Tacı {}", sira + 1), tac_mesh, Renk::BEYAZ)
                .donusum3b(
                    Donusum3B::yeni()
                        .konum(Vektor3::yeni(x, 2.35 * olcek, z))
                        .olcek(Vektor3::yeni(olcek, olcek, olcek)),
                ),
        );
        let engel = dunya.varlik_ekle(
            Varlik::yeni("Ağaç Çarpışması")
                .donusum3b(Donusum3B::yeni().konum(govde_konumu)),
        );
        fizik.govde_ekle(FizikGovdesi::statik_kup(
            engel,
            Vektor3::yeni(0.42 * olcek, 1.1 * olcek, 0.42 * olcek),
        ));
    }

    let kayalar = [
        (-4.7, -5.1, 1.0, 0.2),
        (-6.2, 1.8, 0.7, -0.4),
        (-4.8, 5.5, 1.2, 0.5),
        (4.5, -6.0, 0.9, -0.2),
        (6.0, -0.5, 1.3, 0.35),
        (5.4, 5.2, 0.75, -0.5),
        (0.0, -7.8, 1.1, 0.1),
        (2.8, 6.8, 0.8, 0.6),
    ];
    for (sira, (x, z, olcek, donus)) in kayalar.into_iter().enumerate() {
        let konum = Vektor3::yeni(x, -0.05, z);
        let boyut = Vektor3::yeni(olcek, olcek * 0.72, olcek * 1.18);
        let kimlik = dunya.varlik_ekle(
            Varlik::mesh(format!("Kaya {}", sira + 1), kaya_mesh, Renk::BEYAZ).donusum3b(
                Donusum3B::yeni()
                    .konum(konum)
                    .olcek(boyut)
                    .donus(Vektor3::yeni(0.0, donus, 0.12)),
            ),
        );
        fizik.govde_ekle(FizikGovdesi::statik_kup(
            kimlik,
            Vektor3::yeni(olcek * 0.75, olcek * 0.55, olcek * 0.8),
        ));
    }
    Ok(())
}

fn macera_isaretlerini_ekle(dunya: &mut Dunya) -> OyunSonucu {
    let insan_govdesi = dunya.mesh_ekle(silindir(
        12,
        0.34,
        1.45,
        MalzemeVerisi::yeni(Renk::yeni(0.12, 0.32, 0.72, 1.0)),
    )?);
    let bas_mesh = dunya.mesh_ekle(kure(
        14,
        8,
        0.36,
        0.0,
        MalzemeVerisi::yeni(Renk::yeni(0.78, 0.58, 0.38, 1.0)),
    )?);
    dunya.varlik_ekle(
        Varlik::mesh("Gözcü Aras Gövdesi", insan_govdesi, Renk::BEYAZ).donusum3b(
            Donusum3B::yeni().konum(Vektor3::yeni(0.0, 0.25, 2.0)),
        ),
    );
    dunya.varlik_ekle(
        Varlik::mesh("Gözcü Aras Başı", bas_mesh, Renk::BEYAZ)
            .donusum3b(Donusum3B::yeni().konum(Vektor3::yeni(0.0, 1.3, 2.0))),
    );

    let muhur_mesh = dunya.mesh_ekle(kure(
        14,
        8,
        0.34,
        0.06,
        MalzemeVerisi::new(Renk::BEYAZ),
    )?);
    let muhur_malzemeleri = [
        dunya.malzeme_ekle(MalzemeVerisi::yeni(Renk::YESIL)),
        dunya.malzeme_ekle(MalzemeVerisi::yeni(Renk::MAVI)),
        dunya.malzeme_ekle(MalzemeVerisi::yeni(Renk::SARI)),
    ];
    for (sira, konum) in [
        Vektor3::yeni(3.2, 0.1, 1.0),
        Vektor3::yeni(-3.2, 0.1, -1.0),
        Vektor3::yeni(2.4, 0.1, -2.4),
    ]
    .into_iter()
    .enumerate()
    {
        dunya.varlik_ekle(
            Varlik::mesh_malzemeli(
                format!("Mühür Parçası {}", sira + 1),
                muhur_mesh,
                muhur_malzemeleri[sira],
                Renk::BEYAZ,
            )
            .donusum3b(Donusum3B::yeni().konum(konum)),
        );
    }

    let muhafiz_mesh = dunya.mesh_ekle(kure(
        12,
        7,
        1.0,
        0.32,
        MalzemeVerisi::yeni(Renk::yeni(0.42, 0.12, 0.10, 1.0)),
    )?);
    dunya.varlik_ekle(
        Varlik::mesh("Taş Muhafız", muhafiz_mesh, Renk::BEYAZ).donusum3b(
            Donusum3B::yeni()
                .konum(Vektor3::yeni(0.0, 0.45, -4.8))
                .olcek(Vektor3::yeni(1.0, 1.75, 1.0)),
        ),
    );
    Ok(())
}

fn piramit_meshini_yukle(dunya: &mut Dunya) -> OyunSonucu<MeshKimligi> {
    let model = ModelVerisi::gltf_yukle("assets/piramit.gltf")?;
    dunya
        .model_ekle(model)
        .first()
        .copied()
        .ok_or_else(|| OyunHatasi::yeni("Piramit modeli kayıtlı mesh üretmedi."))
}

fn piramitleri_ekle(
    dunya: &mut Dunya,
    fizik: &mut FizikDunyasi,
    mesh: MeshKimligi,
    alternatif_malzeme: MalzemeKimligi,
) -> Vec<VarlikKimligi> {
    let piramitler = [
        (Vektor3::yeni(-3.2, -0.58, -2.8), Renk::MAVI),
        (Vektor3::yeni(-1.6, -0.58, -4.2), Renk::YESIL),
        (Vektor3::yeni(1.6, -0.58, -4.2), Renk::KIRMIZI),
        (Vektor3::yeni(3.2, -0.58, -2.8), Renk::SARI),
        (Vektor3::yeni(-3.2, -0.58, 2.8), Renk::KIRMIZI),
        (Vektor3::yeni(-1.6, -0.58, 4.2), Renk::SARI),
        (Vektor3::yeni(1.6, -0.58, 4.2), Renk::MAVI),
        (Vektor3::yeni(3.2, -0.58, 2.8), Renk::YESIL),
    ];
    let goruntu_olcegi = Vektor3::yeni(0.65, 0.65, 0.65);
    let carpisma_olcegi = Vektor3::yeni(1.3, 1.17, 1.3);
    let mut kimlikler = Vec::with_capacity(piramitler.len());

    for (sira, (konum, renk)) in piramitler.into_iter().enumerate() {
        let varlik = if sira.is_multiple_of(3) {
            Varlik::mesh_malzemeli(
                "Alternatif Malzemeli Piramit",
                mesh,
                alternatif_malzeme,
                renk,
            )
        } else {
            Varlik::mesh("Dokulu glTF Piramit", mesh, renk)
        };
        let kimlik = dunya
            .varlik_ekle(varlik.donusum3b(Donusum3B::yeni().konum(konum).olcek(goruntu_olcegi)));
        kimlikler.push(kimlik);

        let engel = dunya.varlik_ekle(
            Varlik::yeni("Piramit Çarpışması")
                .donusum3b(Donusum3B::yeni().konum(konum + Vektor3::YUKARI * 0.585)),
        );
        fizik.govde_ekle(FizikGovdesi::statik_kup(engel, carpisma_olcegi));
    }

    kimlikler
}

fn gorunurluk_stres_sahnesi_ekle(dunya: &mut Dunya, mesh: MeshKimligi) {
    let olcek = Vektor3::yeni(0.35, 0.35, 0.35);
    for sira in 0_u16..256 {
        let sutun = f32::from(sira % 16);
        let satir = f32::from(sira / 16);
        dunya.varlik_ekle(
            Varlik::mesh("Frustum Dışı Piramit", mesh, Renk::BEYAZ).donusum3b(
                Donusum3B::yeni()
                    .konum(Vektor3::yeni(400.0 + sutun * 2.0, -0.58, -satir * 2.0))
                    .olcek(olcek),
            ),
        );
    }
}
