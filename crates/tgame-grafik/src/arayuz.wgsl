struct PanelOrnegi {
    @location(0) konum_boyut: vec4<f32>,
    @location(1) renk: vec4<f32>,
};

struct TepeCikisi {
    @builtin(position) konum: vec4<f32>,
    @location(0) renk: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) tepe: u32, panel: PanelOrnegi) -> TepeCikisi {
    let koseler = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(1.0, 1.0),
    );
    let kose = koseler[tepe];
    let x = panel.konum_boyut.x + kose.x * panel.konum_boyut.z;
    let y = panel.konum_boyut.y - kose.y * panel.konum_boyut.w;
    var cikis: TepeCikisi;
    cikis.konum = vec4<f32>(x, y, 0.0, 1.0);
    cikis.renk = panel.renk;
    return cikis;
}

@fragment
fn fs_main(giris: TepeCikisi) -> @location(0) vec4<f32> {
    return giris.renk;
}
