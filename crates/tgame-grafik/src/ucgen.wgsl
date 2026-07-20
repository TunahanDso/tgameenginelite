struct TepeCiktisi {
    @builtin(position) konum: vec4<f32>,
    @location(0) renk: vec3<f32>,
}

@vertex
fn tepe_ana(@builtin(vertex_index) tepe: u32) -> TepeCiktisi {
    let konumlar = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.65),
        vec2<f32>(-0.65, -0.55),
        vec2<f32>(0.65, -0.55),
    );
    let renkler = array<vec3<f32>, 3>(
        vec3<f32>(0.95, 0.25, 0.20),
        vec3<f32>(0.20, 0.80, 0.45),
        vec3<f32>(0.20, 0.45, 0.95),
    );

    var cikti: TepeCiktisi;
    cikti.konum = vec4<f32>(konumlar[tepe], 0.0, 1.0);
    cikti.renk = renkler[tepe];
    return cikti;
}

@fragment
fn parca_ana(girdi: TepeCiktisi) -> @location(0) vec4<f32> {
    return vec4<f32>(girdi.renk, 1.0);
}
