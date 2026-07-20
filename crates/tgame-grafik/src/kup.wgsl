struct Kamera3B {
    gorunum_izdusum: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> kamera: Kamera3B;

struct TepeGirdisi {
    @location(0) yerel_konum: vec3<f32>,
    @location(1) yerel_normal: vec3<f32>,
    @location(2) model_0: vec4<f32>,
    @location(3) model_1: vec4<f32>,
    @location(4) model_2: vec4<f32>,
    @location(5) model_3: vec4<f32>,
    @location(6) varlik_rengi: vec4<f32>,
}

struct TepeCiktisi {
    @builtin(position) konum: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) renk: vec4<f32>,
}

@vertex
fn tepe_ana(girdi: TepeGirdisi) -> TepeCiktisi {
    let model = mat4x4<f32>(
        girdi.model_0,
        girdi.model_1,
        girdi.model_2,
        girdi.model_3,
    );
    let dunya_konumu = model * vec4<f32>(girdi.yerel_konum, 1.0);
    let dunya_normali = normalize((model * vec4<f32>(girdi.yerel_normal, 0.0)).xyz);

    var cikti: TepeCiktisi;
    cikti.konum = kamera.gorunum_izdusum * dunya_konumu;
    cikti.normal = dunya_normali;
    cikti.renk = girdi.varlik_rengi;
    return cikti;
}

@fragment
fn parca_ana(girdi: TepeCiktisi) -> @location(0) vec4<f32> {
    let isik_yonu = normalize(vec3<f32>(0.45, 0.80, 0.55));
    let yaygin = max(dot(normalize(girdi.normal), isik_yonu), 0.0);
    let parlaklik = 0.22 + yaygin * 0.78;
    return vec4<f32>(girdi.renk.rgb * parlaklik, girdi.renk.a);
}
