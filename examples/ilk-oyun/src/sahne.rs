use tgame::onsoz::{
    Donusum3B, Dunya, FizikDunyasi, FizikGovdesi, Kamera3B, MalzemeKimligi, MalzemeVerisi,
    MeshKimligi, ModelVerisi, OyunHatasi, OyunSonucu, Renk, Varlik, VarlikKimligi, Vektor3,
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
            .konum(Vektor3::yeni(0.0, 4.2, 8.0))
            .hedef(Vektor3::SIFIR)
            .kirpma(0.1, 250.0),
    );

    zemin_ekle(&mut dunya, &mut fizik);
    sutunlari_ekle(&mut dunya, &mut fizik);
    macera_isaretlerini_ekle(&mut dunya);
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

    let merkez_olcegi = Vektor3::yeni(1.7, 1.7, 1.7);
    let merkez = dunya.varlik_ekle(
        Varlik::kup("Dönen Tapınak Çekirdeği", Renk::KIRMIZI).donusum3b(
            Donusum3B::yeni()
                .konum(Vektor3::yeni(0.0, 1.35, 0.0))
                .olcek(merkez_olcegi)
                .donus(Vektor3::yeni(0.25, 0.35, 0.1)),
        ),
    );
    fizik.govde_ekle(FizikGovdesi::statik_kup(merkez, merkez_olcegi));

    Ok(SahneKurulumu {
        dunya,
        fizik,
        oyuncu,
        merkez,
        piramitler,
    })
}

fn macera_isaretlerini_ekle(dunya: &mut Dunya) {
    dunya.varlik_ekle(
        Varlik::kup("Gözcü Aras", Renk::MAVI).donusum3b(
            Donusum3B::yeni()
                .konum(Vektor3::yeni(0.0, 0.15, 2.0))
                .olcek(Vektor3::yeni(0.7, 1.6, 0.7)),
        ),
    );

    for (sira, konum) in [
        Vektor3::yeni(3.2, 0.0, 1.0),
        Vektor3::yeni(-3.2, 0.0, -1.0),
        Vektor3::yeni(2.4, 0.0, -2.4),
    ]
    .into_iter()
    .enumerate()
    {
        let renk = [Renk::YESIL, Renk::MAVI, Renk::SARI][sira];
        dunya.varlik_ekle(
            Varlik::kup("Mühür Parçası", renk).donusum3b(
                Donusum3B::yeni()
                    .konum(konum + Vektor3::YUKARI * 0.25)
                    .olcek(Vektor3::yeni(0.35, 0.35, 0.35)),
            ),
        );
    }

    dunya.varlik_ekle(
        Varlik::kup("Taş Muhafız", Renk::KIRMIZI).donusum3b(
            Donusum3B::yeni()
                .konum(Vektor3::yeni(0.0, 0.45, -4.8))
                .olcek(Vektor3::yeni(1.2, 2.2, 1.2)),
        ),
    );
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

fn zemin_ekle(dunya: &mut Dunya, fizik: &mut FizikDunyasi) {
    let koordinatlar = [-5.0, -4.0, -3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let olcek = Vektor3::yeni(0.96, 0.12, 0.96);
    let mut acik_renk = false;

    for z in koordinatlar {
        for x in koordinatlar {
            let renk = if acik_renk {
                Renk::yeni(0.18, 0.24, 0.34, 1.0)
            } else {
                Renk::yeni(0.10, 0.14, 0.22, 1.0)
            };
            let kimlik = dunya.varlik_ekle(
                Varlik::kup("Zemin", renk).donusum3b(
                    Donusum3B::yeni()
                        .konum(Vektor3::yeni(x, -0.65, z))
                        .olcek(olcek),
                ),
            );
            fizik.govde_ekle(FizikGovdesi::statik_kup(kimlik, olcek));
            acik_renk = !acik_renk;
        }
        acik_renk = !acik_renk;
    }
}

fn sutunlari_ekle(dunya: &mut Dunya, fizik: &mut FizikDunyasi) {
    let sutunlar = [
        (Vektor3::yeni(-4.0, 0.4, -4.0), Renk::MAVI, 2.0),
        (Vektor3::yeni(4.0, 0.9, -4.0), Renk::YESIL, 3.0),
        (Vektor3::yeni(-4.0, 1.4, 4.0), Renk::KIRMIZI, 4.0),
        (Vektor3::yeni(4.0, 0.65, 4.0), Renk::SARI, 2.5),
        (Vektor3::yeni(-2.5, 0.15, 0.0), Renk::YESIL, 1.5),
        (Vektor3::yeni(2.5, 0.15, 0.0), Renk::MAVI, 1.5),
    ];

    for (konum, renk, yukseklik) in sutunlar {
        let olcek = Vektor3::yeni(0.8, yukseklik, 0.8);
        let kimlik = dunya.varlik_ekle(
            Varlik::kup("Sütun", renk).donusum3b(Donusum3B::yeni().konum(konum).olcek(olcek)),
        );
        fizik.govde_ekle(FizikGovdesi::statik_kup(kimlik, olcek));
    }
}
