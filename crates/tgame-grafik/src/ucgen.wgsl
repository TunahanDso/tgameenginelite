struct Kamera {
    konum: vec2<f32>,
    yarim_gorus_yuksekligi: f32,
    en_boy_orani: f32,
}

@group(0) @binding(0)
var<uniform> kamera: Kamera;

struct TepeGirdisi {
    @builtin(vertex_index) tepe: u32,
    @location(0) varlik_konumu: vec2<f32>,
    @location(1) varlik_olcegi: vec2<f32>,
    @location(2) varlik_donusu: f32,
    @location(3) varlik_rengi: vec4<f32>,
}

struct TepeCiktisi {
    @builtin(position) konum: vec4<f32>,
    @location(0) renk: vec4<f32>,
}

@vertex
fn tepe_ana(girdi: TepeGirdisi) -> TepeCiktisi {
    let konumlar = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.65),
        vec2<f32>(-0.65, -0.55),
        vec2<f32>(0.65, -0.55),
    );
    let yerel = konumlar[girdi.tepe] * girdi.varlik_olcegi;
    let kosinus = cos(girdi.varlik_donusu);
    let sinus = sin(girdi.varlik_donusu);
    let donmus = vec2<f32>(
        yerel.x * kosinus - yerel.y * sinus,
        yerel.x * sinus + yerel.y * kosinus,
    );
    let dunya_konumu = donmus + girdi.varlik_konumu;
    let yarim_yukseklik = max(kamera.yarim_gorus_yuksekligi, 0.0001);
    let yarim_genislik = yarim_yukseklik * max(kamera.en_boy_orani, 0.0001);
    let kamera_goreli = dunya_konumu - kamera.konum;
    let klip_konumu = vec2<f32>(
        kamera_goreli.x / yarim_genislik,
        kamera_goreli.y / yarim_yukseklik,
    );

    var cikti: TepeCiktisi;
    cikti.konum = vec4<f32>(klip_konumu, 0.0, 1.0);
    cikti.renk = girdi.varlik_rengi;
    return cikti;
}

@fragment
fn parca_ana(girdi: TepeCiktisi) -> @location(0) vec4<f32> {
    return girdi.renk;
}
