use crate::canvas::Canvas;
use crate::palette::{DragonPalette, Rgba};
use std::f32::consts::PI;

/// Renders a top-down soaring wyvern with authentic vertebrate flight mechanics,
/// complete with all 16 canon dragonfire combustion, fluid dynamics, and flame splitting types:
/// 1. Dual sublingual gland convergence (binary hypergolic ignition node)
/// 2. Slipstream relative velocity drag & wind shear
/// 3. Kelvin-Helmholtz vortex sheet instability & billowing multi-lobed flame splitting
/// 4. Combustion chugging & traveling expanding flame packets
/// 5. Drogon: Supersonic Mach shock diamonds & concussive blast wave rings
/// 6. Balerion: Helical swirling bifurcation of carbonaceous soot & crimson core
/// 7. Rhaegal: Bifurcated V-fork split with emerald spark sheath
/// 8. Viserion: Slender laminar needle jet (< 12°) blossoming at terminal tip
/// 9. Wight Viserion: Collimated coherent magnetic pinch plasma beam & ice shrapnel
/// 10. Caraxes: Acoustic screech resonance & sinuous serpentine ribbon flame
/// 11. Vhagar: Cataclysmic rolling pyroclastic furnace cloud engulfing the arena
/// 12. Meleys: High-speed pulsed fire strafing (traveling flame bullets)
/// 13. Sunfyre: High-temperature ionization corona & glittering golden spark halo
/// 14. Vermithor: Boiling bronze napalm slag & Rayleigh-Plateau droplet breakup
/// 15. Sheepstealer: Asymmetrical sputtering & uneven napalm splatter
/// 16. Moondancer: Hyper-focused cutting torch jet with diverging micro-sparks
pub fn render_topdown_wyvern_frame(
    pal: &DragonPalette,
    phase: f32, // 0.0 to 1.0 (wing flap cycle)
    flame_progress: f32, // 0.0 to 1.0 during fire breath (-1.0 if not breathing)
    scale: f32,
    mouth_open: f32,
    cx: f32,
    cy: f32,
    angle: f32, // heading angle in radians (velocity tangent)
    turn_rate: f32, // angular turning rate (-1.0 to +1.0)
    vx: f32, // flight velocity vector X
    vy: f32, // flight velocity vector Y
    frame_idx: usize,
    altitude_factor: f32, // 0.0 = touching ground at strike, 1.0 = high cruise
    reach_talons: f32,    // 0.0 = cruise tuck, 1.0 = forward predatory grapple reach
    bolus_pos: f32,       // -1.0 = none, 0.0..7.0 = traveling cervical bolus wave
    jaw_flare: f32,       // 0.0 = normal, 1.0 = streptostylic lateral cheek expansion
    post_feed_glow: f32,  // 0.0 = none, 1.0 = digestive furnace pulse
    canvas: &mut Canvas,
) {
    let t = phase * 2.0 * PI;
    let flap = t.sin();      // +1.0 (bottom of downstroke), -1.0 (top of upstroke)
    let flap_vel = t.cos();  // > 0 downstroke, < 0 upstroke

    // Direction vectors: forward (heading) and lateral (right wing)
    let fx = angle.cos();
    let fy = angle.sin();
    let rx = -angle.sin();
    let ry = angle.cos();

    // 15 Species Flags
    let is_drogon = pal.key == "drogon";
    let is_rhaegal = pal.key == "rhaegal";
    let is_viserion = pal.key == "viserion";
    let is_wight = pal.key == "wight_viserion" || pal.is_plasma;
    let is_caraxes = pal.key == "caraxes";
    let is_vhagar = pal.key == "vhagar";
    let is_syrax = pal.key == "syrax";
    let is_meleys = pal.key == "meleys";
    let is_sunfyre = pal.key == "sunfyre";
    let is_vermithor = pal.key == "vermithor";
    let is_silverwing = pal.key == "silverwing";
    let is_seasmoke = pal.key == "seasmoke";
    let is_sheepstealer = pal.key == "sheepstealer";
    let is_balerion = pal.key == "balerion";
    let is_moondancer = pal.key == "moondancer";
    let is_tessarion = pal.key == "tessarion";

    // Forward Radiance Emissive Bloom Intensity & Elemental Radiance Color
    let flame_bloom = if flame_progress >= 0.0 && flame_progress <= 1.0 {
        1.0_f32
    } else if flame_progress > 1.0 && flame_progress <= 1.6 {
        (1.0 - (flame_progress - 1.0) / 0.6).max(0.0)
    } else if flame_progress >= -0.4 && flame_progress < 0.0 {
        ((flame_progress + 0.4) / 0.4 * 0.5).max(0.0)
    } else {
        0.0_f32
    };

    let bloom_col = if is_wight {
        Rgba::rgb(0, 220, 255)
    } else if is_sunfyre {
        Rgba::rgb(255, 235, 120)
    } else if is_caraxes {
        Rgba::rgb(255, 70, 20)
    } else if is_rhaegal {
        Rgba::rgb(118, 255, 3)
    } else if is_tessarion {
        Rgba::rgb(0, 210, 255)
    } else if is_viserion {
        Rgba::rgb(255, 215, 60)
    } else {
        pal.fire_inner.lerp(pal.fire_core, 0.5)
    };

    // -----------------------------------------------------------------
    // 1. ANATOMICAL SKELETAL AXIS (Species-Accurate Neck Lengths & Flexion)
    // -----------------------------------------------------------------
    let neck_len = if is_caraxes {
        2.00
    } else if is_moondancer || is_rhaegal {
        1.08
    } else if is_vhagar || is_balerion || is_vermithor {
        0.92
    } else {
        1.00
    };

    let fwd_landmarks = [
        36.0 * neck_len, // 0: Snout
        29.0 * neck_len, // 1: Naris / Mid-Snout
        22.0 * neck_len, // 2: Eyes / Cranium
        16.0 * neck_len, // 3: Jaw hinge / Horn bases
        9.0 * neck_len,  // 4: Neck base
        0.0,             // 5: Pectoral shoulder anchor (Rigid Thorax start)
        -9.0,            // 6: Ribcage anterior
        -18.0,           // 7: Ribcage posterior
        -27.0,           // 8: Pelvic hips (Rigid Thorax end)
        -36.0,           // 9: Tail base
        -46.0,           // 10: Tail anterior
        -56.0,           // 11: Tail mid
        -66.0,           // 12: Tail mid-posterior
        -76.0,           // 13: Tail posterior
        -86.0,           // 14: Pre-spade
        -96.0,           // 15: Tail tip
    ];

    let tail_pitch_amp = if is_caraxes {
        10.5
    } else if is_vhagar || is_balerion {
        7.0
    } else {
        8.5
    } * scale;

    let mut spine_pts: Vec<(f32, f32)> = Vec::with_capacity(16);
    let mut tail_z: Vec<f32> = Vec::with_capacity(16);

    for (i, &d_fwd) in fwd_landmarks.iter().enumerate() {
        let _frac = i as f32 / 15.0;

        // 3D Vertical Up/Down Tail Pitch Wave (Coupled to wing downstroke & traveling inertia)
        let z_elev = if i >= 9 {
            let u = (i as f32 - 8.0) / 7.0;
            tail_pitch_amp * u * (t - u * 2.2).sin()
        } else {
            0.0
        };
        tail_z.push(z_elev);

        // Pitch foreshortening along flight heading vector fx
        let pitch_slope = (z_elev / (28.0 * scale)).clamp(-0.7, 0.7);
        let foreshorten = (1.0 - pitch_slope * pitch_slope * 0.40).max(0.72);

        let dist = if i >= 9 {
            let base_dist = d_fwd * scale;
            let hip_dist = fwd_landmarks[8] * scale;
            hip_dist + (base_dist - hip_dist) * foreshorten
        } else {
            d_fwd * scale
        };

        let mut turn_lateral = if i <= 4 {
            let u = (4.0 - i as f32) / 4.0;
            turn_rate * u.powf(1.4) * 8.0 * scale
        } else if i <= 8 {
            0.0
        } else {
            let u = (i as f32 - 8.0) / 7.0;
            // Vertebrate curvilinear arc: tail conforms to circular arc trajectory (C-curve into turn)
            let arc_bend = turn_rate * u.powf(1.5) * 12.5 * scale;

            // Aeroelastic wave propagation phase lag & traveling undulation:
            // Caudal vertebrae respond with inertia and trailing wave lag behind the hips
            let wave_freq = if is_caraxes || is_moondancer { 1.5 } else if is_vhagar || is_balerion { 0.85 } else { 1.1 };
            let whip_amp = if is_caraxes {
                4.8
            } else if is_moondancer {
                3.2
            } else if is_vhagar || is_balerion {
                1.4
            } else if is_meleys {
                2.2
            } else {
                2.5
            } * scale;

            // Dynamic aeroelastic whip coupled to flight flap and turning rate
            let wave_phase = t * wave_freq - u * 2.6;
            let whip = wave_phase.sin() * (0.25 + 0.35 * turn_rate.abs()) * whip_amp * u;

            arc_bend + whip
        };

        // Caraxes Sinuous Ribbon S-Neck Wave
        if is_caraxes && i <= 4 {
            let neck_s = ((t * 2.0 - (4.0 - i as f32) * 1.2).sin()) * 5.0 * scale;
            turn_lateral += neck_s;
        }

        // Tail whip undulation for Caraxes sinuous ribbon tail
        if is_caraxes && i >= 9 {
            let u = (i as f32 - 8.0) / 7.0;
            let serp = (t * 2.2 - u * 4.0).sin() * 4.2 * scale * u;
            turn_lateral += serp;
        }

        let px = cx + dist * fx + turn_lateral * rx;
        let py = cy + dist * fy + turn_lateral * ry;
        spine_pts.push((px, py));
    }

    // Precompute smoothed vertebra tangents (anterior heading) and normals (anatomical right)
    let mut spine_tangents: Vec<(f32, f32)> = Vec::with_capacity(16);
    let mut spine_normals: Vec<(f32, f32)> = Vec::with_capacity(16);

    for k in 0..16 {
        let (tx, ty) = if k == 0 {
            let dx = spine_pts[0].0 - spine_pts[1].0;
            let dy = spine_pts[0].1 - spine_pts[1].1;
            let l = (dx * dx + dy * dy).sqrt().max(0.001);
            (dx / l, dy / l)
        } else if k == 15 {
            let dx = spine_pts[14].0 - spine_pts[15].0;
            let dy = spine_pts[14].1 - spine_pts[15].1;
            let l = (dx * dx + dy * dy).sqrt().max(0.001);
            (dx / l, dy / l)
        } else {
            let dx = spine_pts[k - 1].0 - spine_pts[k + 1].0;
            let dy = spine_pts[k - 1].1 - spine_pts[k + 1].1;
            let l = (dx * dx + dy * dy).sqrt().max(0.001);
            (dx / l, dy / l)
        };
        spine_tangents.push((tx, ty));
        spine_normals.push((-ty, tx));
    }

    // -----------------------------------------------------------------
    // 2. SPECIES-CUSTOMIZED WING PLANFORMS & 3D ROLL FORESHORTENING
    // -----------------------------------------------------------------
    let sh_pt = spine_pts[5];
    let hip_pt = spine_pts[8];
    let bank_angle = turn_rate * 0.75;
    let span_factor = 1.0 + flap * 0.22;
    let sweep_lead = (flap * 7.0 + flap_vel * 2.5) * scale;
    let billow = (flap * 3.2).max(-1.8) * scale;

    let base_span = if is_balerion {
        88.0
    } else if is_vhagar {
        82.0
    } else if is_vermithor {
        78.0
    } else if is_silverwing {
        77.0
    } else if is_drogon {
        76.0
    } else if is_meleys {
        74.0
    } else if is_rhaegal || is_viserion {
        70.0
    } else if is_syrax || is_sunfyre || is_sheepstealer {
        68.0
    } else if is_wight {
        68.0
    } else if is_seasmoke {
        66.0
    } else if is_caraxes {
        64.0
    } else if is_moondancer {
        58.0
    } else {
        72.0
    };

    struct WingLandmarks {
        side: f32,
        sh_anchor: (f32, f32),
        elbow: (f32, f32),
        wrist: (f32, f32),
        d2_knuckle: (f32, f32),
        tip_2: (f32, f32),
        d3_knuckle1: (f32, f32),
        d3_knuckle2: (f32, f32),
        tip_3: (f32, f32),
        d4_knuckle: (f32, f32),
        tip_4: (f32, f32),
        d5_knuckle: (f32, f32),
        tip_5: (f32, f32),
        flank_anchor: (f32, f32),
        sc_23: (f32, f32),
        sc_34: (f32, f32),
        sc_45: (f32, f32),
        sc_5hip: (f32, f32),
    }

    let mut wings: Vec<WingLandmarks> = Vec::with_capacity(2);

    for &side_sign in &[-1.0_f32, 1.0_f32] {
        let side = side_sign;
        let roll_inboard = side * bank_angle;
        let span_roll = if roll_inboard > 0.0 {
            (1.0 - roll_inboard * 0.32).max(0.55)
        } else {
            (1.0 + (-roll_inboard) * 0.08).min(1.10)
        };

        let w_span = base_span * span_factor * span_roll * scale;
        let roll_sweep = -roll_inboard * 3.8 * scale;

        let sh_anchor = (
            sh_pt.0 + side * 9.0 * scale * rx,
            sh_pt.1 + side * 9.0 * scale * ry,
        );

        let elbow = (
            sh_anchor.0 + (side * w_span * 0.36) * rx + (7.0 * scale + sweep_lead * 0.6 + roll_sweep * 0.4) * fx,
            sh_anchor.1 + (side * w_span * 0.36) * ry + (7.0 * scale + sweep_lead * 0.6 + roll_sweep * 0.4) * fy,
        );

        let wrist = (
            sh_anchor.0 + (side * w_span * 0.72) * rx + (3.0 * scale + sweep_lead + roll_sweep * 0.8) * fx,
            sh_anchor.1 + (side * w_span * 0.72) * ry + (3.0 * scale + sweep_lead + roll_sweep * 0.8) * fy,
        );

        let sickle_lead = if is_meleys { 4.5 * scale } else { 0.0 };

        let d2_knuckle = (
            sh_anchor.0 + (side * w_span * 0.88) * rx + (-1.0 * scale + sweep_lead * 1.1 + roll_sweep + sickle_lead * 0.5) * fx,
            sh_anchor.1 + (side * w_span * 0.88) * ry + (-1.0 * scale + sweep_lead * 1.1 + roll_sweep + sickle_lead * 0.5) * fy,
        );
        let tip_2 = (
            sh_anchor.0 + (side * w_span * 1.0) * rx + (-6.0 * scale + sweep_lead * 1.2 + roll_sweep + sickle_lead) * fx,
            sh_anchor.1 + (side * w_span * 1.0) * ry + (-6.0 * scale + sweep_lead * 1.2 + roll_sweep + sickle_lead) * fy,
        );

        let d3_spread = 1.0 + flap * 0.15;
        let d3_bow = billow * 0.7;
        let d3_knuckle1 = (
            sh_anchor.0 + (side * w_span * 0.76 * d3_spread) * rx + (-9.0 * scale + sweep_lead * 0.95 + d3_bow) * fx,
            sh_anchor.1 + (side * w_span * 0.76 * d3_spread) * ry + (-9.0 * scale + sweep_lead * 0.95 + d3_bow) * fy,
        );
        let d3_knuckle2 = (
            sh_anchor.0 + (side * w_span * 0.82 * d3_spread) * rx + (-16.0 * scale + sweep_lead * 0.85 + d3_bow * 0.8) * fx,
            sh_anchor.1 + (side * w_span * 0.82 * d3_spread) * ry + (-16.0 * scale + sweep_lead * 0.85 + d3_bow * 0.8) * fy,
        );
        let tip_3 = (
            sh_anchor.0 + (side * w_span * 0.84 * d3_spread) * rx + (-22.0 * scale + sweep_lead * 0.8 + sickle_lead * 0.7) * fx,
            sh_anchor.1 + (side * w_span * 0.84 * d3_spread) * ry + (-22.0 * scale + sweep_lead * 0.8 + sickle_lead * 0.7) * fy,
        );

        let crooked_spar = if is_sheepstealer && side < 0.0 { -4.0 * scale } else { 0.0 };

        let d4_knuckle = (
            sh_anchor.0 + (side * w_span * 0.62 * d3_spread) * rx + (-22.0 * scale + sweep_lead * 0.6 + crooked_spar) * fx,
            sh_anchor.1 + (side * w_span * 0.62 * d3_spread) * ry + (-22.0 * scale + sweep_lead * 0.6 + crooked_spar) * fy,
        );
        let tip_4 = (
            sh_anchor.0 + (side * w_span * 0.55 * d3_spread) * rx + (-32.0 * scale + sweep_lead * 0.5 + crooked_spar) * fx,
            sh_anchor.1 + (side * w_span * 0.55 * d3_spread) * ry + (-32.0 * scale + sweep_lead * 0.5 + crooked_spar) * fy,
        );

        let d5_knuckle = (
            sh_anchor.0 + (side * w_span * 0.38 * d3_spread) * rx + (-28.0 * scale + sweep_lead * 0.4) * fx,
            sh_anchor.1 + (side * w_span * 0.38 * d3_spread) * ry + (-28.0 * scale + sweep_lead * 0.4) * fy,
        );
        let tip_5 = (
            sh_anchor.0 + (side * w_span * 0.28 * d3_spread) * rx + (-36.0 * scale + sweep_lead * 0.3) * fx,
            sh_anchor.1 + (side * w_span * 0.28 * d3_spread) * ry + (-36.0 * scale + sweep_lead * 0.3) * fy,
        );

        let flank_anchor = (
            hip_pt.0 + side * 7.0 * scale * rx,
            hip_pt.1 + side * 7.0 * scale * ry,
        );

        let scallop_depth = if is_syrax {
            (1.8 - flap * 0.6) * scale
        } else {
            (4.5 - flap * 1.5) * scale
        };

        let sc_23 = (
            (tip_2.0 + tip_3.0) * 0.5 + (scallop_depth + billow * 0.4) * fx,
            (tip_2.1 + tip_3.1) * 0.5 + (scallop_depth + billow * 0.4) * fy,
        );
        let sc_34 = (
            (tip_3.0 + tip_4.0) * 0.5 + (scallop_depth + 1.0 * scale + billow * 0.5) * fx,
            (tip_3.1 + tip_4.1) * 0.5 + (scallop_depth + 1.0 * scale + billow * 0.5) * fy,
        );
        let sc_45 = (
            (tip_4.0 + tip_5.0) * 0.5 + (scallop_depth + 1.0 * scale + billow * 0.5) * fx,
            (tip_4.1 + tip_5.1) * 0.5 + (scallop_depth + 1.0 * scale + billow * 0.5) * fy,
        );
        let sc_5hip = (
            (tip_5.0 + flank_anchor.0) * 0.5 + (scallop_depth + 2.0 * scale + billow * 0.6) * fx,
            (tip_5.1 + flank_anchor.1) * 0.5 + (scallop_depth + 2.0 * scale + billow * 0.6) * fy,
        );

        wings.push(WingLandmarks {
            side,
            sh_anchor,
            elbow,
            wrist,
            d2_knuckle,
            tip_2,
            d3_knuckle1,
            d3_knuckle2,
            tip_3,
            d4_knuckle,
            tip_4,
            d5_knuckle,
            tip_5,
            flank_anchor,
            sc_23,
            sc_34,
            sc_45,
            sc_5hip,
        });
    }

    // -----------------------------------------------------------------
    // 3. PROJECTIVE DUAL-DENSITY SILHOUETTE DROP SHADOW & PENUMBRA OPTICS
    // (Active for all 16 canon wyverns: Dynamic sun azimuth, dual-density umbra/penumbra, altitude diffusion)
    // -----------------------------------------------------------------
    // Dynamic Sun Azimuth Drop Shadow: Vector S = [0.55, 0.65] * d_alt
    // Since all wyverns are flying, shadow accurately projects 3D flight altitude across the terrain surface below
    let flight_alt = (16.0 + 16.0 * altitude_factor.clamp(0.05, 1.0) + flap * 2.5) * scale;
    let d_alt = flight_alt.max(5.0 * scale);
    let sh_dx = 0.55 * d_alt;
    let sh_dy = 0.65 * d_alt;

    // Atmospheric Scattering Shadow Tint (Opaque RGB for retro stippled matrix)
    let (sh_r, sh_g, sh_b) = if is_wight {
        (22, 32, 46) // Glacial slate-blue tint
    } else if is_sunfyre {
        (38, 30, 22) // Warm amber tint
    } else if is_drogon || is_balerion {
        (28, 22, 26) // Volcanic cinder tint
    } else {
        (26, 28, 32) // Neutral ground tint
    };
    let penumbra_col = Rgba::rgb(sh_r, sh_g, sh_b);
    let umbra_col = Rgba::rgb(
        (sh_r as f32 * 0.70) as u8,
        (sh_g as f32 * 0.70) as u8,
        (sh_b as f32 * 0.70) as u8,
    );

    // Altitude Penumbra Diffusion:
    // High cruising altitudes expand and soften shadow boundaries via penumbral diffusion; low-altitude stoops sharpen and contract
    let penumbra_fringe = (((altitude_factor * 2.5 * scale).round() as i32)).max(0);

    for w in &wings {
        let sh_poly = [
            ((w.sh_anchor.0 + sh_dx).round() as i32, (w.sh_anchor.1 + sh_dy).round() as i32),
            ((w.elbow.0 + sh_dx).round() as i32, (w.elbow.1 + sh_dy).round() as i32),
            ((w.wrist.0 + sh_dx).round() as i32, (w.wrist.1 + sh_dy).round() as i32),
            ((w.tip_2.0 + sh_dx).round() as i32, (w.tip_2.1 + sh_dy).round() as i32),
            ((w.sc_23.0 + sh_dx).round() as i32, (w.sc_23.1 + sh_dy).round() as i32),
            ((w.tip_3.0 + sh_dx).round() as i32, (w.tip_3.1 + sh_dy).round() as i32),
            ((w.sc_34.0 + sh_dx).round() as i32, (w.sc_34.1 + sh_dy).round() as i32),
            ((w.tip_4.0 + sh_dx).round() as i32, (w.tip_4.1 + sh_dy).round() as i32),
            ((w.tip_5.0 + sh_dx).round() as i32, (w.tip_5.1 + sh_dy).round() as i32),
            ((w.flank_anchor.0 + sh_dx).round() as i32, (w.flank_anchor.1 + sh_dy).round() as i32),
        ];

        // Altitude Penumbra Diffusion: High cruising altitudes soften shadow boundary via 25% stipple fringe
        if penumbra_fringe > 0 {
            let pf = penumbra_fringe as f32;
            let sh_fringe_poly = [
                ((w.sh_anchor.0 + sh_dx).round() as i32, (w.sh_anchor.1 + sh_dy - pf).round() as i32),
                ((w.elbow.0 + sh_dx + pf * w.side).round() as i32, (w.elbow.1 + sh_dy - pf).round() as i32),
                ((w.wrist.0 + sh_dx + pf * 1.5 * w.side).round() as i32, (w.wrist.1 + sh_dy - pf).round() as i32),
                ((w.tip_2.0 + sh_dx + pf * 1.5 * w.side).round() as i32, (w.tip_2.1 + sh_dy).round() as i32),
                ((w.sc_23.0 + sh_dx + pf * w.side).round() as i32, (w.sc_23.1 + sh_dy + pf).round() as i32),
                ((w.tip_3.0 + sh_dx + pf * w.side).round() as i32, (w.tip_3.1 + sh_dy + pf).round() as i32),
                ((w.sc_34.0 + sh_dx + pf * 0.8 * w.side).round() as i32, (w.sc_34.1 + sh_dy + pf).round() as i32),
                ((w.tip_4.0 + sh_dx + pf * 0.8 * w.side).round() as i32, (w.tip_4.1 + sh_dy + pf).round() as i32),
                ((w.tip_5.0 + sh_dx).round() as i32, (w.tip_5.1 + sh_dy + pf).round() as i32),
                ((w.flank_anchor.0 + sh_dx).round() as i32, (w.flank_anchor.1 + sh_dy + pf).round() as i32),
            ];
            canvas.fill_polygon_stipple_25(&sh_fringe_poly, penumbra_col);
        }

        // Translucent wing membrane penumbra (50% checkerboard stipple)
        canvas.fill_polygon_stipple_50(&sh_poly, penumbra_col);

        // High-density skeletal humerus umbra
        let sh_anc_x = (w.sh_anchor.0 + sh_dx).round() as i32;
        let sh_anc_y = (w.sh_anchor.1 + sh_dy).round() as i32;
        let sh_elb_x = (w.elbow.0 + sh_dx).round() as i32;
        let sh_elb_y = (w.elbow.1 + sh_dy).round() as i32;
        let sh_wri_x = (w.wrist.0 + sh_dx).round() as i32;
        let sh_wri_y = (w.wrist.1 + sh_dy).round() as i32;
        let humerus_w = ((2.4 * scale).round() as i32).max(1);
        canvas.draw_line(sh_anc_x, sh_anc_y, sh_elb_x, sh_elb_y, umbra_col, humerus_w);
        canvas.draw_line(sh_elb_x, sh_elb_y, sh_wri_x, sh_wri_y, umbra_col, humerus_w.max(1));
    }

    // 1. Skull, Snout, & Cranium Umbra on Surface
    let cran_sh_x = (spine_pts[2].0 + sh_dx).round() as i32;
    let cran_sh_y = (spine_pts[2].1 + sh_dy).round() as i32;
    let snout_sh_x = (spine_pts[0].0 + sh_dx).round() as i32;
    let snout_sh_y = (spine_pts[0].1 + sh_dy).round() as i32;
    let cranium_rx = ((4.2 * scale).round() as i32).max(2);
    let cranium_ry = ((3.4 * scale).round() as i32).max(2);
    if penumbra_fringe > 0 {
        canvas.fill_ellipse_stipple_50(cran_sh_x, cran_sh_y, cranium_rx + penumbra_fringe, cranium_ry + penumbra_fringe, penumbra_col);
    }
    canvas.fill_ellipse(cran_sh_x, cran_sh_y, cranium_rx, cranium_ry, umbra_col);
    canvas.draw_line(cran_sh_x, cran_sh_y, snout_sh_x, snout_sh_y, umbra_col, ((3.0 * scale).round() as i32).max(1));

    // 2. Cervical Neck Shadow (Connecting cranium to shoulders across nodes 0..5)
    for i in 0..5 {
        let n_curr = spine_pts[i];
        let n_next = spine_pts[i + 1];
        let nx0 = (n_curr.0 + sh_dx).round() as i32;
        let ny0 = (n_curr.1 + sh_dy).round() as i32;
        let nx1 = (n_next.0 + sh_dx).round() as i32;
        let ny1 = (n_next.1 + sh_dy).round() as i32;
        let n_w = ((3.2 * scale).round() as i32).max(1);
        canvas.draw_line_stipple_50(nx0, ny0, nx1, ny1, penumbra_col, n_w + 2);
        canvas.draw_line(nx0, ny0, nx1, ny1, umbra_col, n_w);
    }

    // 3. Thorax & Rigid Trunk Shadow (Shoulders to Pelvis nodes 5..8)
    for i in 5..8 {
        let b_curr = spine_pts[i];
        let b_next = spine_pts[i + 1];
        let bx0 = (b_curr.0 + sh_dx).round() as i32;
        let by0 = (b_curr.1 + sh_dy).round() as i32;
        let bx1 = (b_next.0 + sh_dx).round() as i32;
        let by1 = (b_next.1 + sh_dy).round() as i32;
        let b_w = ((4.2 * scale).round() as i32).max(2);
        canvas.draw_line_stipple_50(bx0, by0, bx1, by1, penumbra_col, b_w + 3);
        canvas.draw_line(bx0, by0, bx1, by1, umbra_col, b_w);
    }

    // 4. Rear Trailing Hindlimbs / Pelvic Talons Shadow on Surface
    let hp_sh = ((spine_pts[8].0 + sh_dx).round() as i32, (spine_pts[8].1 + sh_dy).round() as i32);
    let leg_reach = 9.0 * scale;
    let leg_spread = 5.5 * scale;
    let leg_l = (hp_sh.0 as f32 - leg_spread * rx - leg_reach * fx, hp_sh.1 as f32 - leg_spread * ry - leg_reach * fy);
    let leg_r = (hp_sh.0 as f32 + leg_spread * rx - leg_reach * fx, hp_sh.1 as f32 + leg_spread * ry - leg_reach * fy);
    canvas.draw_line(hp_sh.0, hp_sh.1, leg_l.0.round() as i32, leg_l.1.round() as i32, umbra_col, ((2.0 * scale).round() as i32).max(1));
    canvas.draw_line(hp_sh.0, hp_sh.1, leg_r.0.round() as i32, leg_r.1.round() as i32, umbra_col, ((2.0 * scale).round() as i32).max(1));

    // 5. Caudal Vertebral Tail Shadow (Nodes 8..15)
    for i in 8..15 {
        let p_curr = spine_pts[i];
        let p_next = spine_pts[i + 1];
        let sh_curr_x = (p_curr.0 + sh_dx).round() as i32;
        let sh_curr_y = (p_curr.1 + sh_dy).round() as i32;
        let sh_next_x = (p_next.0 + sh_dx).round() as i32;
        let sh_next_y = (p_next.1 + sh_dy).round() as i32;
        let pen_w = (((4.2 - (i as f32 - 8.0) * 0.35) * scale).round() as i32).max(2);
        let umb_w = (((2.4 - (i as f32 - 8.0) * 0.22) * scale).round() as i32).max(1);
        canvas.draw_line_stipple_50(sh_curr_x, sh_curr_y, sh_next_x, sh_next_y, penumbra_col, pen_w);
        canvas.draw_line(sh_curr_x, sh_curr_y, sh_next_x, sh_next_y, umbra_col, umb_w);
    }

    let tip_pt = spine_pts[15];
    let sp_sh_x = tip_pt.0 + sh_dx;
    let sp_sh_y = tip_pt.1 + sh_dy;
    let (t_nx, t_ny) = spine_normals[15];
    let (t_tx, t_ty) = spine_tangents[15];
    let b_tx = -t_tx;
    let b_ty = -t_ty;
    let sp_sh_poly = [
        (sp_sh_x.round() as i32, sp_sh_y.round() as i32),
        ((sp_sh_x - 5.0 * scale * t_nx + 6.0 * scale * b_tx).round() as i32, (sp_sh_y - 5.0 * scale * t_ny + 6.0 * scale * b_ty).round() as i32),
        ((sp_sh_x + 8.0 * scale * b_tx).round() as i32, (sp_sh_y + 8.0 * scale * b_ty).round() as i32),
        ((sp_sh_x + 5.0 * scale * t_nx + 6.0 * scale * b_tx).round() as i32, (sp_sh_y + 5.0 * scale * t_ny + 6.0 * scale * b_ty).round() as i32),
    ];
    canvas.fill_polygon_stipple_50(&sp_sh_poly, penumbra_col);
    canvas.draw_line(sp_sh_x.round() as i32, sp_sh_y.round() as i32, (sp_sh_x + 8.0 * scale * b_tx).round() as i32, (sp_sh_y + 8.0 * scale * b_ty).round() as i32, umbra_col, 2);

    // -----------------------------------------------------------------
    // 4. TRANSLUCENT WING PANELS, CAMBER SHADING, SUBSURFACE SCATTERING & SPAR CONTACT SHADOWS
    // (Resolves Limitations 5, 6, 7, 8: Camber gradient, solar subsurface scattering, spar contact shadows, bank lighting)
    // -----------------------------------------------------------------
    for w in &wings {
        // Continuous smooth bilateral bank roll modulation
        let bank_factor = (-turn_rate * w.side * 5.0).clamp(-1.0, 1.0);
        let web_base1 = pal.wing_web1;
        let web_base2 = pal.wing_web2;
        let web_shadow = pal.wing_far;

        // Smooth lighting modulation based on bank roll:
        // Elevated wing catches sun specular sheen; depressed wing dips smoothly into shadow
        let lit_web1 = if bank_factor > 0.0 {
            web_base1.lerp(pal.wing_arm, bank_factor * 0.30)
        } else {
            web_base1.lerp(web_shadow, -bank_factor * 0.45)
        };
        let lit_web2 = if bank_factor > 0.0 {
            web_base2.lerp(web_base1, bank_factor * 0.30)
        } else {
            web_base2.lerp(web_shadow, -bank_factor * 0.45)
        };

        let p1 = [
            (w.wrist.0.round() as i32, w.wrist.1.round() as i32),
            (w.d2_knuckle.0.round() as i32, w.d2_knuckle.1.round() as i32),
            (w.tip_2.0.round() as i32, w.tip_2.1.round() as i32),
            (w.sc_23.0.round() as i32, w.sc_23.1.round() as i32),
            (w.tip_3.0.round() as i32, w.tip_3.1.round() as i32),
            (w.d3_knuckle2.0.round() as i32, w.d3_knuckle2.1.round() as i32),
            (w.d3_knuckle1.0.round() as i32, w.d3_knuckle1.1.round() as i32),
        ];
        let p2 = [
            (w.wrist.0.round() as i32, w.wrist.1.round() as i32),
            (w.d3_knuckle1.0.round() as i32, w.d3_knuckle1.1.round() as i32),
            (w.d3_knuckle2.0.round() as i32, w.d3_knuckle2.1.round() as i32),
            (w.tip_3.0.round() as i32, w.tip_3.1.round() as i32),
            (w.sc_34.0.round() as i32, w.sc_34.1.round() as i32),
            (w.tip_4.0.round() as i32, w.tip_4.1.round() as i32),
            (w.d4_knuckle.0.round() as i32, w.d4_knuckle.1.round() as i32),
        ];
        let p3 = [
            (w.wrist.0.round() as i32, w.wrist.1.round() as i32),
            (w.d4_knuckle.0.round() as i32, w.d4_knuckle.1.round() as i32),
            (w.tip_4.0.round() as i32, w.tip_4.1.round() as i32),
            (w.sc_45.0.round() as i32, w.sc_45.1.round() as i32),
            (w.tip_5.0.round() as i32, w.tip_5.1.round() as i32),
            (w.d5_knuckle.0.round() as i32, w.d5_knuckle.1.round() as i32),
        ];
        let p4 = [
            (w.wrist.0.round() as i32, w.wrist.1.round() as i32),
            (w.d5_knuckle.0.round() as i32, w.d5_knuckle.1.round() as i32),
            (w.tip_5.0.round() as i32, w.tip_5.1.round() as i32),
            (w.sc_5hip.0.round() as i32, w.sc_5hip.1.round() as i32),
            (w.flank_anchor.0.round() as i32, w.flank_anchor.1.round() as i32),
            (w.sh_anchor.0.round() as i32, w.sh_anchor.1.round() as i32),
            (w.elbow.0.round() as i32, w.elbow.1.round() as i32),
        ];

        // Smooth Airfoil Camber Gradient across patagium panels (from leading convex to trailing inner)
        let col_p1 = lit_web1;
        let col_p2 = lit_web1.lerp(lit_web2, 0.30);
        let col_p3 = lit_web1.lerp(lit_web2, 0.65);
        let col_p4 = lit_web2;

        canvas.fill_polygon(&p1, col_p1);
        canvas.fill_polygon(&p2, col_p2);
        canvas.fill_polygon(&p3, col_p3);
        canvas.fill_polygon(&p4, col_p4);

        let inner_web = [
            (w.sh_anchor.0.round() as i32, w.sh_anchor.1.round() as i32),
            (w.elbow.0.round() as i32, w.elbow.1.round() as i32),
            (w.wrist.0.round() as i32, w.wrist.1.round() as i32),
            (w.d3_knuckle1.0.round() as i32, w.d3_knuckle1.1.round() as i32),
            (w.d4_knuckle.0.round() as i32, w.d4_knuckle.1.round() as i32),
            (w.flank_anchor.0.round() as i32, w.flank_anchor.1.round() as i32),
        ];
        canvas.fill_polygon(&inner_web, col_p4.lerp(pal.wing_far, 0.35));

        // Subsurface Scattering (Backlit Solar Translucency):
        // Elastic patagial cells transmit radiant warm blood-vein glow when backlit
        let sss_col = Rgba(
            lit_web1.0.saturating_add(38),
            lit_web1.1.saturating_add(22),
            lit_web1.2.saturating_add(10),
            185,
        );
        let sss_pts = [w.sc_23, w.sc_34, w.sc_45, w.sc_5hip];
        for pt in &sss_pts {
            canvas.fill_circle(pt.0.round() as i32, pt.1.round() as i32, (1.8 * scale).round() as i32, sss_col);
        }

        // Spar Contact Drop Shadows:
        // Draw 1-pixel directional shadow line along the downstream trailing edge of digits II-V onto the membrane
        let spar_sh_col = Rgba(8, 10, 14, 95);
        let s_dx = (1.2 * rx).round() as i32;
        let s_dy = (1.2 * ry).round() as i32;
        let wx = w.wrist.0.round() as i32;
        let wy = w.wrist.1.round() as i32;
        canvas.draw_line(wx + s_dx, wy + s_dy, w.d3_knuckle1.0.round() as i32 + s_dx, w.d3_knuckle1.1.round() as i32 + s_dy, spar_sh_col, 1);
        canvas.draw_line(w.d3_knuckle1.0.round() as i32 + s_dx, w.d3_knuckle1.1.round() as i32 + s_dy, w.tip_3.0.round() as i32 + s_dx, w.tip_3.1.round() as i32 + s_dy, spar_sh_col, 1);
        canvas.draw_line(wx + s_dx, wy + s_dy, w.tip_4.0.round() as i32 + s_dx, w.tip_4.1.round() as i32 + s_dy, spar_sh_col, 1);
        canvas.draw_line(wx + s_dx, wy + s_dy, w.tip_5.0.round() as i32 + s_dx, w.tip_5.1.round() as i32 + s_dy, spar_sh_col, 1);

        // Wight Viserion Gaping Puncture Holes
        if is_wight {
            let hole1_cx = ((w.d3_knuckle2.0 + w.d4_knuckle.0) * 0.5).round() as i32;
            let hole1_cy = ((w.d3_knuckle2.1 + w.d4_knuckle.1) * 0.5).round() as i32;
            let r_hole = (3.5 * scale).round() as i32;
            for hx in -r_hole..=r_hole {
                for hy in -r_hole..=r_hole {
                    if hx * hx + hy * hy <= r_hole * r_hole {
                        canvas.set_pixel(hole1_cx + hx, hole1_cy + hy, Rgba::TRANSPARENT);
                    }
                }
            }
        }

        let mut trans_col = pal.wing_web1;
        trans_col.3 = 130;
        let sc_pts = [w.sc_23, w.sc_34, w.sc_45, w.sc_5hip];
        for pt in &sc_pts {
            canvas.set_pixel(pt.0.round() as i32, pt.1.round() as i32, trans_col);
        }

        let mut vein_col = pal.wing_bone;
        vein_col.3 = 160;
        canvas.draw_line(wx, wy, w.sc_23.0.round() as i32, w.sc_23.1.round() as i32, vein_col, 1);
        canvas.draw_line(wx, wy, w.sc_34.0.round() as i32, w.sc_34.1.round() as i32, vein_col, 1);
        canvas.draw_line(wx, wy, w.sc_45.0.round() as i32, w.sc_45.1.round() as i32, vein_col, 1);

        let shx = w.sh_anchor.0.round() as i32;
        let shy = w.sh_anchor.1.round() as i32;
        let elx = w.elbow.0.round() as i32;
        let ely = w.elbow.1.round() as i32;

        let arm_thick = if is_moondancer { 3 } else if is_balerion || is_vhagar { 5 } else { 4 };
        let propatagium_col = if flame_bloom > 0.0 {
            pal.wing_arm.lerp(bloom_col, flame_bloom * 0.45)
        } else {
            pal.wing_arm
        };
        canvas.draw_line(shx, shy, elx, ely, propatagium_col, arm_thick);
        canvas.draw_line(elx, ely, wx, wy, propatagium_col, (arm_thick - 1).max(2));
        if flame_bloom > 0.0 {
            let bloom_edge = bloom_col.lerp(Rgba::rgb(255, 255, 255), 0.35);
            canvas.draw_line((shx as f32 + fx).round() as i32, (shy as f32 + fy).round() as i32, (elx as f32 + fx).round() as i32, (ely as f32 + fy).round() as i32, bloom_edge, 1);
        }

        // Specular ridge glint smoothly fades in on elevated sunward wing arm
        if bank_factor > 0.0 {
            let glint_col = pal.wing_arm.lerp(pal.scale_specular, bank_factor.clamp(0.0, 1.0));
            canvas.draw_line(shx, shy, elx, ely, glint_col, 1);
            let joint_col = pal.wing_bone.lerp(pal.scale_specular, bank_factor.clamp(0.0, 1.0));
            canvas.set_pixel(elx, ely, joint_col);
            canvas.set_pixel(wx, wy, joint_col);
        }

        canvas.draw_line(wx, wy, w.d2_knuckle.0.round() as i32, w.d2_knuckle.1.round() as i32, pal.wing_bone, 2);
        canvas.draw_line(w.d2_knuckle.0.round() as i32, w.d2_knuckle.1.round() as i32, w.tip_2.0.round() as i32, w.tip_2.1.round() as i32, pal.wing_bone, 2);

        canvas.draw_line(wx, wy, w.d3_knuckle1.0.round() as i32, w.d3_knuckle1.1.round() as i32, pal.wing_bone, 2);
        canvas.draw_line(w.d3_knuckle1.0.round() as i32, w.d3_knuckle1.1.round() as i32, w.d3_knuckle2.0.round() as i32, w.d3_knuckle2.1.round() as i32, pal.wing_bone, 1);
        canvas.draw_line(w.d3_knuckle2.0.round() as i32, w.d3_knuckle2.1.round() as i32, w.tip_3.0.round() as i32, w.tip_3.1.round() as i32, pal.wing_bone, 1);

        canvas.draw_line(wx, wy, w.d4_knuckle.0.round() as i32, w.d4_knuckle.1.round() as i32, pal.wing_bone, 1);
        canvas.draw_line(w.d4_knuckle.0.round() as i32, w.d4_knuckle.1.round() as i32, w.tip_4.0.round() as i32, w.tip_4.1.round() as i32, pal.wing_bone, 1);

        canvas.draw_line(wx, wy, w.d5_knuckle.0.round() as i32, w.d5_knuckle.1.round() as i32, pal.wing_bone, 1);
        canvas.draw_line(w.d5_knuckle.0.round() as i32, w.d5_knuckle.1.round() as i32, w.tip_5.0.round() as i32, w.tip_5.1.round() as i32, pal.wing_bone, 1);

        let th_x = (w.wrist.0 - w.side * 4.0 * scale * rx + 5.0 * scale * fx).round() as i32;
        let th_y = (w.wrist.1 - w.side * 4.0 * scale * ry + 5.0 * scale * fy).round() as i32;
        canvas.draw_line(wx, wy, th_x, th_y, Rgba::rgb(245, 245, 235), 1);
    }

    // -----------------------------------------------------------------
    // 5. DYNAMIC HIND LEGS, CRUROPATAGIUM & AXILLARY/INGUINAL AMBIENT OCCLUSION
    // (Resolves Limitation 19: Axillary armpit and inguinal groin crevice shadows)
    // -----------------------------------------------------------------
    for &side_sign in &[-1.0_f32, 1.0_f32] {
        let side = side_sign;
        let leg_pitch = flap * 1.5 * scale;
        let leg_yaw = side * turn_rate * 2.2 * scale;

        // Predatory talon reach forward posture when swooping down to strike
        let reach_fwd = reach_talons * 16.0 * scale;
        let reach_spread = reach_talons * 4.0 * scale;

        let leg_hip = (hip_pt.0 + side * 7.5 * scale * rx, hip_pt.1 + side * 7.5 * scale * ry);
        let knee = (
            leg_hip.0 + side * (5.0 + leg_yaw + reach_spread) * scale * rx + (-8.0 * scale - leg_pitch + reach_fwd * 0.6) * fx,
            leg_hip.1 + side * (5.0 + leg_yaw + reach_spread) * scale * ry + (-8.0 * scale - leg_pitch + reach_fwd * 0.6) * fy,
        );
        let ankle = (
            knee.0 - side * (2.0 - leg_yaw * 0.4) * scale * rx + (-10.0 * scale - leg_pitch * 0.5 + reach_fwd) * fx,
            knee.1 - side * (2.0 - leg_yaw * 0.4) * scale * ry + (-10.0 * scale - leg_pitch * 0.5 + reach_fwd) * fy,
        );

        let flank_pt = (hip_pt.0 + side * 7.0 * scale * rx, hip_pt.1 + side * 7.0 * scale * ry);
        let tail_anchor = spine_pts[10];
        let cruro_poly = [
            (flank_pt.0.round() as i32, flank_pt.1.round() as i32),
            (knee.0.round() as i32, knee.1.round() as i32),
            (ankle.0.round() as i32, ankle.1.round() as i32),
            (tail_anchor.0.round() as i32, tail_anchor.1.round() as i32),
        ];
        let mut cruro_col = pal.wing_web1;
        cruro_col.3 = 210;
        canvas.fill_polygon(&cruro_poly, cruro_col);

        // Inguinal Joint Ambient Occlusion Shadow (Pelvis socketing crevice)
        let mut ing_ao_col = pal.scale_deep;
        ing_ao_col.3 = 195;
        canvas.draw_line(leg_hip.0.round() as i32, leg_hip.1.round() as i32, (flank_pt.0 - 2.0 * scale * fx).round() as i32, (flank_pt.1 - 2.0 * scale * fy).round() as i32, ing_ao_col, 2);

        canvas.draw_line(leg_hip.0.round() as i32, leg_hip.1.round() as i32, knee.0.round() as i32, knee.1.round() as i32, pal.scale_mid, 3);
        canvas.draw_line(knee.0.round() as i32, knee.1.round() as i32, ankle.0.round() as i32, ankle.1.round() as i32, pal.scale_dark, 2);

        // Claws splay wide in reach and snap shut on grapple
        let claw_spread = 1.0 + reach_talons * 0.8;
        let c1 = (ankle.0 + side * 2.5 * scale * claw_spread * rx - 5.0 * scale * fx, ankle.1 + side * 2.5 * scale * claw_spread * ry - 5.0 * scale * fy);
        let c2 = (ankle.0 - side * 1.5 * scale * rx - 6.0 * scale * fx, ankle.1 - side * 1.5 * scale * ry - 6.0 * scale * fy);
        canvas.draw_line(ankle.0.round() as i32, ankle.1.round() as i32, c1.0.round() as i32, c1.1.round() as i32, Rgba::rgb(240, 240, 230), 1);
        canvas.draw_line(ankle.0.round() as i32, ankle.1.round() as i32, c2.0.round() as i32, c2.1.round() as i32, Rgba::rgb(200, 200, 190), 1);

        if is_caraxes {
            let rud_tip = (
                ankle.0 + side * 10.0 * scale * rx - 6.0 * scale * fx,
                ankle.1 + side * 10.0 * scale * ry - 6.0 * scale * fy,
            );
            let rudder_poly = [
                (ankle.0.round() as i32, ankle.1.round() as i32),
                (knee.0.round() as i32, knee.1.round() as i32),
                (rud_tip.0.round() as i32, rud_tip.1.round() as i32),
            ];
            let mut rud_col = pal.wing_web1;
            rud_col.3 = 230;
            canvas.fill_polygon(&rudder_poly, rud_col);
            canvas.draw_line(ankle.0.round() as i32, ankle.1.round() as i32, rud_tip.0.round() as i32, rud_tip.1.round() as i32, pal.scale_hi, 1);
        }
    }

    // Axillary Armpit Ambient Occlusion Shadows (Where wing humerus roots into pectoral chest)
    for w in &wings {
        let mut ax_ao_col = pal.scale_deep;
        ax_ao_col.3 = 210;
        let ax_sh_p = (w.sh_anchor.0 + 3.0 * scale * fx, w.sh_anchor.1 + 3.0 * scale * fy);
        canvas.draw_line(w.sh_anchor.0.round() as i32, w.sh_anchor.1.round() as i32, ax_sh_p.0.round() as i32, ax_sh_p.1.round() as i32, ax_ao_col, 2);
    }

    // -----------------------------------------------------------------
    // 6. SPECIES-CUSTOMIZED THORAX WIDTH PROFILE, 5-TIER SCALES, CYLINDRICAL NORMALS & COUNTERSHADING
    // (Resolves Limitations 10, 17, 18, 20: 5-tier palettes, cylindrical normals, multi-tier throat glow, bank countershading)
    // -----------------------------------------------------------------
    let mut w_profile = if is_vhagar || is_balerion {
        [4.5, 7.2, 11.0, 12.5, 10.5, 16.0, 17.5, 16.0, 11.0, 8.5, 7.0, 5.5, 4.2, 3.0, 2.2, 1.4]
    } else if is_drogon || is_vermithor {
        [4.0, 6.5, 10.0, 11.5, 9.5, 14.5, 16.0, 14.5, 10.0, 8.0, 6.5, 5.2, 4.0, 2.8, 2.0, 1.3]
    } else if is_caraxes {
        [2.8, 4.2, 7.0, 7.5, 6.0, 9.0, 9.8, 9.0, 7.5, 6.5, 5.5, 4.5, 3.6, 2.8, 2.0, 1.2]
    } else if is_moondancer || is_seasmoke {
        [2.5, 4.2, 7.2, 8.0, 6.5, 10.2, 11.5, 10.2, 7.5, 6.0, 4.8, 3.8, 2.8, 2.0, 1.4, 1.0]
    } else if is_wight {
        [3.0, 4.8, 7.8, 8.5, 6.8, 11.0, 12.0, 11.0, 7.8, 6.2, 4.8, 3.6, 2.6, 1.8, 1.2, 0.9]
    } else {
        [3.0, 5.2, 8.5, 9.5, 7.8, 12.5, 14.0, 13.0, 9.0, 7.5, 6.0, 4.8, 3.8, 2.8, 2.0, 1.2]
    };

    // Peristaltic Bolus Tracking: Gaussian muscular dilation traveling down cervical vertebrae
    if bolus_pos >= 0.0 {
        let bolus_amp = if is_vhagar || is_balerion { 5.8 } else if is_caraxes { 3.8 } else { 4.6 };
        for k in 1..8 {
            let dist = (k as f32 - bolus_pos).abs();
            let dilation = bolus_amp * (-0.95 * dist * dist).exp();
            w_profile[k] += dilation;
        }
    }

    if is_vhagar {
        let p_dew0 = spine_pts[1];
        let p_dew1 = spine_pts[4];
        let dew_l = (p_dew0.0 - 9.0 * scale * rx - 2.0 * scale * fx, p_dew0.1 - 9.0 * scale * ry - 2.0 * scale * fy);
        let dew_r = (p_dew0.0 + 9.0 * scale * rx - 2.0 * scale * fx, p_dew0.1 + 9.0 * scale * ry - 2.0 * scale * fy);
        let dew_poly = [
            (dew_l.0.round() as i32, dew_l.1.round() as i32),
            (p_dew0.0.round() as i32, p_dew0.1.round() as i32),
            (dew_r.0.round() as i32, dew_r.1.round() as i32),
            (p_dew1.0.round() as i32, p_dew1.1.round() as i32),
        ];
        canvas.fill_polygon(&dew_poly, pal.ventral_deep);
    }

    for i in (1..15).rev() {
        let p0 = spine_pts[i];
        let p1 = spine_pts[i + 1];
        let z0 = tail_z[i];
        let z1 = tail_z[i + 1];

        let z_zoom0 = if i >= 9 { (1.0 + z0 / (34.0 * scale)).clamp(0.68, 1.32) } else { 1.0 };
        let z_zoom1 = if (i + 1) >= 9 { (1.0 + z1 / (34.0 * scale)).clamp(0.68, 1.32) } else { 1.0 };

        let w0 = w_profile[i] * scale * z_zoom0;
        let w1 = w_profile[i + 1] * scale * z_zoom1;

        let (n0_x, n0_y) = spine_normals[i];
        let (n1_x, n1_y) = spine_normals[i + 1];

        let left_0 = (p0.0 - w0 * n0_x, p0.1 - w0 * n0_y);
        let right_0 = (p0.0 + w0 * n0_x, p0.1 + w0 * n0_y);
        let left_1 = (p1.0 - w1 * n1_x, p1.1 - w1 * n1_y);
        let right_1 = (p1.0 + w1 * n1_x, p1.1 + w1 * n1_y);

        // 5-Tier Scale Palette & Cylindrical Normal Volumetric Curvature
        // Split segment into Left and Right Hemi-Cylinders for authentic 3D Lambertian cosine lighting
        let dot_l = (0.55 * n0_x + 0.65 * n0_y).clamp(-1.0, 1.0);
        let dot_r = -dot_l;

        let (col_left, col_right) = if i >= 4 && i <= 9 {
            // Thorax and ribcage: elevated scutes catch primary directional sunbeam
            let cl = if dot_l >= 0.0 {
                pal.scale_dark.lerp(pal.scale_mid, dot_l)
            } else {
                pal.scale_dark.lerp(pal.scale_deep, -dot_l)
            };
            let cr = if dot_r >= 0.0 {
                pal.scale_dark.lerp(pal.scale_mid, dot_r)
            } else {
                pal.scale_dark.lerp(pal.scale_deep, -dot_r)
            };
            (cl, cr)
        } else {
            // Tubular cervical neck (1..3) & tapering caudal tail (10..14):
            let cl = if dot_l >= 0.0 {
                pal.scale_dark.lerp(pal.scale_mid, dot_l * 0.7)
            } else {
                pal.scale_dark.lerp(pal.scale_deep, -dot_l * 0.8)
            };
            let cr = if dot_r >= 0.0 {
                pal.scale_dark.lerp(pal.scale_mid, dot_r * 0.7)
            } else {
                pal.scale_dark.lerp(pal.scale_deep, -dot_r * 0.8)
            };
            (cl, cr)
        };

        // Forward Radiance Emissive Bloom on Anterior Cervical Scutes (Nodes 0..=3)
        let (col_left, col_right) = if i <= 3 && flame_bloom > 0.0 {
            let bloom_falloff = (1.0 - (i as f32) / 4.0).max(0.0) * flame_bloom;
            (
                col_left.lerp(bloom_col, bloom_falloff * 0.60),
                col_right.lerp(bloom_col, bloom_falloff * 0.60),
            )
        } else {
            (col_left, col_right)
        };

        let left_poly = [
            (left_0.0.round() as i32, left_0.1.round() as i32),
            (p0.0.round() as i32, p0.1.round() as i32),
            (p1.0.round() as i32, p1.1.round() as i32),
            (left_1.0.round() as i32, left_1.1.round() as i32),
        ];
        let right_poly = [
            (p0.0.round() as i32, p0.1.round() as i32),
            (right_0.0.round() as i32, right_0.1.round() as i32),
            (right_1.0.round() as i32, right_1.1.round() as i32),
            (p1.0.round() as i32, p1.1.round() as i32),
        ];
        canvas.fill_polygon(&left_poly, col_left);
        canvas.fill_polygon(&right_poly, col_right);

        // Lateral flank occlusion crevice
        canvas.draw_line(left_0.0.round() as i32, left_0.1.round() as i32, left_1.0.round() as i32, left_1.1.round() as i32, pal.scale_deep, 1);
        canvas.draw_line(right_0.0.round() as i32, right_0.1.round() as i32, right_1.0.round() as i32, right_1.1.round() as i32, pal.scale_deep, 1);

        // Ventral Underbelly Countershading on Knife-Edge Bank Turns
        // Continuous smooth emergence without threshold pops
        let roll_amt = (turn_rate.abs() / 0.14).clamp(0.0, 1.0);
        if roll_amt > 0.01 && i >= 3 && i <= 9 {
            let roll_side = if turn_rate < 0.0 { 1.0_f32 } else { -1.0_f32 }; // high side rolls into view
            let v_thick = (w0 * 0.38 * roll_amt).max(0.5);
            let v0 = (p0.0 + roll_side * (w0 - v_thick) * n0_x, p0.1 + roll_side * (w0 - v_thick) * n0_y);
            let v1 = (p1.0 + roll_side * (w1 - v_thick) * n1_x, p1.1 + roll_side * (w1 - v_thick) * n1_y);
            let v_edge0 = if roll_side > 0.0 { right_0 } else { left_0 };
            let v_edge1 = if roll_side > 0.0 { right_1 } else { left_1 };
            let vent_poly = [
                (v0.0.round() as i32, v0.1.round() as i32),
                (v_edge0.0.round() as i32, v_edge0.1.round() as i32),
                (v_edge1.0.round() as i32, v_edge1.1.round() as i32),
                (v1.0.round() as i32, v1.1.round() as i32),
            ];
            let v_col = pal.scale_dark.lerp(pal.ventral_mid, roll_amt);
            canvas.fill_polygon(&vent_poly, v_col);
            let v_rim = pal.scale_mid.lerp(pal.ventral_hi, roll_amt);
            canvas.draw_line(v0.0.round() as i32, v0.1.round() as i32, v1.0.round() as i32, v1.1.round() as i32, v_rim, 1);
        }

        if is_sunfyre && i >= 3 && i <= 8 {
            let px_i = p0.0.round() as i32;
            let py_i = p0.1.round() as i32;
            if (px_i * 7 + py_i * 13) % 4 == 0 {
                canvas.set_pixel(px_i, py_i, pal.scale_specular);
            }
        }

        // Spinal dorsal ridge lighting: smooth continuous specular glint along vertebral spinous processes
        let spine_glint = pal.scale_mid.lerp(pal.scale_hi, (dot_l.abs() * 0.6 + 0.4).clamp(0.0, 1.0));
        canvas.draw_line(p0.0.round() as i32, p0.1.round() as i32, p1.0.round() as i32, p1.1.round() as i32, spine_glint, 1);
        if i % 2 == 0 {
            let spec_pt = pal.scale_hi.lerp(pal.scale_specular, (dot_l.abs()).clamp(0.0, 1.0));
            canvas.set_pixel(p0.0.round() as i32, p0.1.round() as i32, spec_pt);
        }

        // Esophageal Bioluminescent Throat Furnace (Multi-Tiered Combustion Transillumination)
        let throat_glow_intensity = if flame_progress >= -1.0 && flame_progress < 0.0 {
            let charge_p = flame_progress + 1.0;
            // Internal furnace ignition begins in thorax and climbs up the neck toward jaws
            let seg_lead = ((4 - i) as f32 / 3.0).clamp(0.0, 1.0);
            if charge_p >= seg_lead * 0.4 {
                ((charge_p - seg_lead * 0.4) / (1.0 - seg_lead * 0.4)) * ((frame_idx as f32 * 0.4).sin() * 0.15 + 0.85)
            } else {
                0.0
            }
        } else if flame_progress >= 0.0 && flame_progress <= 1.0 {
            1.0
        } else if flame_progress > 1.0 && flame_progress <= 1.8 {
            1.0 - (flame_progress - 1.0) / 0.8
        } else {
            0.0
        };

        let effective_throat_glow = if post_feed_glow > 0.05 && i >= 1 && i <= 5 {
            throat_glow_intensity.max(post_feed_glow * (0.85 + (frame_idx as f32 * 0.3).sin() * 0.15))
        } else {
            throat_glow_intensity
        };

        if effective_throat_glow > 0.05 && i >= 1 && i <= 5 {
            let tg_alpha = (effective_throat_glow * 235.0).clamp(0.0, 255.0) as u8;
            let tg_col = if is_wight {
                Rgba(0, 220, 255, tg_alpha)
            } else if is_sunfyre {
                Rgba(255, 235, 120, tg_alpha)
            } else if is_caraxes {
                Rgba(255, 60, 20, tg_alpha)
            } else if is_rhaegal {
                Rgba(118, 255, 3, tg_alpha)
            } else if is_viserion {
                Rgba(255, 215, 60, tg_alpha)
            } else if is_tessarion {
                Rgba(0, 210, 255, tg_alpha)
            } else {
                Rgba(255, 140, 30, tg_alpha)
            };

            // Multi-Tiered Esophageal Transillumination:
            // Tier 1: Soft lateral cervical tissue diffusion
            let diff_alpha = (tg_alpha as f32 * 0.45) as u8;
            let diff_col = Rgba(tg_col.0, tg_col.1, tg_col.2, diff_alpha);
            let diff_w = (5.2 * scale * throat_glow_intensity).max(2.0).round() as i32;
            canvas.draw_line(p0.0.round() as i32, p0.1.round() as i32, p1.0.round() as i32, p1.1.round() as i32, diff_col, diff_w);

            // Tier 2: Main combustion mantle line
            let glow_w = (2.4 * scale * throat_glow_intensity).max(1.0).round() as i32;
            canvas.draw_line(p0.0.round() as i32, p0.1.round() as i32, p1.0.round() as i32, p1.1.round() as i32, tg_col, glow_w);

            // Tier 3: Incandescent hot core filament
            canvas.draw_line(p0.0.round() as i32, p0.1.round() as i32, p1.0.round() as i32, p1.1.round() as i32, pal.fire_core, 1);
            canvas.set_pixel((p0.0 + 1.2 * scale * rx).round() as i32, (p0.1 + 1.2 * scale * ry).round() as i32, tg_col);
            canvas.set_pixel((p0.0 - 1.2 * scale * rx).round() as i32, (p0.1 - 1.2 * scale * ry).round() as i32, tg_col);
        }

        if i >= 3 && i <= 14 {
            let h_spine = if is_balerion {
                5.5 * scale
            } else if is_moondancer {
                4.8 * scale
            } else if is_drogon || is_vermithor {
                4.2 * scale
            } else {
                3.2 * scale * z_zoom0
            };

            let (t_tx, t_ty) = spine_tangents[i];
            let (t_nx, t_ny) = spine_normals[i];
            let spike_tip = (
                p0.0 - h_spine * 0.8 * t_tx + 1.2 * scale * t_nx,
                p0.1 - h_spine * 0.8 * t_ty + 1.2 * scale * t_ny,
            );

            let sp_poly = [
                (p0.0.round() as i32, p0.1.round() as i32),
                (spike_tip.0.round() as i32, spike_tip.1.round() as i32),
                ((p0.0 - h_spine * 1.2 * t_tx).round() as i32, (p0.1 - h_spine * 1.2 * t_ty).round() as i32),
            ];

            let u_pitch = (z0 / (8.0 * scale)).clamp(-1.0, 1.0);
            let sp_col = if is_moondancer {
                pal.horn_hi
            } else if is_wight {
                pal.scale_specular
            } else if i >= 9 {
                if u_pitch >= 0.0 {
                    pal.scale_mid.lerp(pal.scale_specular, u_pitch)
                } else {
                    pal.scale_mid.lerp(pal.scale_dark, -u_pitch)
                }
            } else {
                pal.scale_specular
            };

            canvas.fill_polygon(&sp_poly, sp_col);
            canvas.draw_line(p0.0.round() as i32, p0.1.round() as i32, spike_tip.0.round() as i32, spike_tip.1.round() as i32, pal.scale_hi, 1);
        }
    }

    // -----------------------------------------------------------------
    // 7. DYNAMIC Z-ORDERING: UPSTROKE ELEVATED HUMERUS & SELF-SHADOW
    // -----------------------------------------------------------------
    if flap < 0.0 {
        let u_flap = (-flap).clamp(0.0, 1.0);
        let sh_a = (u_flap * 80.0) as u8;
        let sh_cast = Rgba(10, 12, 16, sh_a);
        for w in &wings {
            let shx = w.sh_anchor.0.round() as i32;
            let shy = w.sh_anchor.1.round() as i32;
            let elx = w.elbow.0.round() as i32;
            let ely = w.elbow.1.round() as i32;
            let wx = w.wrist.0.round() as i32;
            let wy = w.wrist.1.round() as i32;

            canvas.draw_line(
                (w.sh_anchor.0 - 2.0 * scale * fx).round() as i32,
                (w.sh_anchor.1 - 2.0 * scale * fy).round() as i32,
                (w.elbow.0 - 2.0 * scale * fx).round() as i32,
                (w.elbow.1 - 2.0 * scale * fy).round() as i32,
                sh_cast,
                2,
            );

            let prop_up_col = if flame_bloom > 0.0 { pal.wing_arm.lerp(bloom_col, flame_bloom * 0.45) } else { pal.wing_arm };
            canvas.draw_line(shx, shy, elx, ely, prop_up_col, 4);
            canvas.draw_line(elx, ely, wx, wy, prop_up_col, 3);
            canvas.set_pixel(elx, ely, pal.scale_specular);
        }
    }

    // Tail Spade / Horizontal Aerodynamic Elevator with 3D Pitch Tilt
    let tip_pt = spine_pts[15];
    let z_tip = tail_z[15];
    let spade_z_zoom = (1.0 + z_tip / (30.0 * scale)).clamp(0.70, 1.30);
    let spade_tilt = (z_tip / (15.0 * scale)).clamp(-0.85, 0.85);

    // Avian Rectrix Fanning (dynamic tail spread in turns and pitch maneuvers)
    let spade_fan = 1.0 + 0.38 * turn_rate.abs() + (spade_tilt.abs() * 0.15);
    let spade_w = 10.0 * scale * spade_z_zoom * spade_fan;
    let spade_len = 13.0 * scale * spade_z_zoom * spade_tilt.cos().abs();

    // Local caudal tail tip tangent & normal alignment
    let (t_tx, t_ty) = spine_tangents[15];
    let (t_nx, t_ny) = spine_normals[15];
    let b_tx = -t_tx;
    let b_ty = -t_ty;

    if is_caraxes {
        let rud_l = (tip_pt.0 - spade_w * t_nx + spade_len * b_tx, tip_pt.1 - spade_w * t_ny + spade_len * b_ty);
        let rud_r = (tip_pt.0 + spade_w * t_nx + spade_len * b_tx, tip_pt.1 + spade_w * t_ny + spade_len * b_ty);
        let rud_rear = (tip_pt.0 + spade_len * 1.35 * b_tx, tip_pt.1 + spade_len * 1.35 * b_ty);
        let fan_poly = [
            (tip_pt.0.round() as i32, tip_pt.1.round() as i32),
            (rud_l.0.round() as i32, rud_l.1.round() as i32),
            (rud_rear.0.round() as i32, rud_rear.1.round() as i32),
            (rud_r.0.round() as i32, rud_r.1.round() as i32),
        ];
        let spade_col = if z_tip > 0.0 { pal.scale_hi } else { pal.wing_arm };
        canvas.fill_polygon(&fan_poly, spade_col);
    } else {
        let spade_l = (tip_pt.0 - spade_w * t_nx + spade_len * 0.7 * b_tx, tip_pt.1 - spade_w * t_ny + spade_len * 0.7 * b_ty);
        let spade_r = (tip_pt.0 + spade_w * t_nx + spade_len * 0.7 * b_tx, tip_pt.1 + spade_w * t_ny + spade_len * 0.7 * b_ty);
        let spade_tip = (tip_pt.0 + spade_len * b_tx, tip_pt.1 + spade_len * b_ty);

        // Lit / Shaded bank facets for 3D aerodynamic realism
        let poly_left = [
            (tip_pt.0.round() as i32, tip_pt.1.round() as i32),
            (spade_l.0.round() as i32, spade_l.1.round() as i32),
            (spade_tip.0.round() as i32, spade_tip.1.round() as i32),
        ];
        let poly_right = [
            (tip_pt.0.round() as i32, tip_pt.1.round() as i32),
            (spade_tip.0.round() as i32, spade_tip.1.round() as i32),
            (spade_r.0.round() as i32, spade_r.1.round() as i32),
        ];

        let z_blend = (z_tip / (8.0 * scale)).clamp(-1.0, 1.0);
        let base_c = if z_blend >= 0.0 {
            pal.wing_arm.lerp(pal.scale_hi, z_blend)
        } else {
            pal.wing_arm.lerp(pal.wing_far, -z_blend)
        };
        let roll_t = (turn_rate * 5.0).clamp(-1.0, 1.0);
        let col_l = if roll_t < 0.0 {
            base_c.lerp(pal.scale_specular, -roll_t)
        } else {
            base_c.lerp(pal.scale_dark, roll_t)
        };
        let col_r = if roll_t > 0.0 {
            base_c.lerp(pal.scale_specular, roll_t)
        } else {
            base_c.lerp(pal.scale_dark, -roll_t)
        };

        canvas.fill_polygon(&poly_left, col_l);
        canvas.fill_polygon(&poly_right, col_r);
        canvas.draw_line(tip_pt.0.round() as i32, tip_pt.1.round() as i32, spade_tip.0.round() as i32, spade_tip.1.round() as i32, pal.scale_hi, 1);
    }

    // -----------------------------------------------------------------
    // 8. ADVANCED CRANIAL OSTEOLOGY, 3 SKULL BREEDS & CEPHALIC TEXTURE
    // -----------------------------------------------------------------
    let snout_pt = spine_pts[0];
    let mid_head = spine_pts[2];
    let jaw_pt = spine_pts[3];

    let is_dinosaurian = is_vhagar || is_balerion || is_vermithor || is_seasmoke || pal.key == "dreamfyre";
    let is_canine = is_caraxes || is_moondancer || is_sheepstealer || is_rhaegal || is_wight;

    // 1. Biological Skull Archetype Geometries (Dinosaurian vs Canine vs Equine)
    // Streptostylic Mandibular Kinesis: Lateral jaw flaring when unhinging to swallow
    let flare_factor = 1.0 + jaw_flare * 0.38;
    let cranium_w = if is_dinosaurian {
        if is_vhagar || is_balerion { 13.0 } else if is_vermithor { 12.0 } else { 10.5 }
    } else if is_canine {
        if is_caraxes { 7.2 } else if is_moondancer { 6.8 } else { 8.2 }
    } else {
        if is_meleys { 9.8 } else { 8.8 }
    } * scale * flare_factor;

    // Continuous smooth cranial facet lighting vs celestial sun L_sun = [-0.55, -0.65]
    let sun_l = (0.55 * rx + 0.65 * ry).clamp(-1.0, 1.0);
    let sun_r = -sun_l;

    let col_facet_l = if sun_l >= 0.0 {
        pal.scale_dark.lerp(pal.scale_mid, sun_l)
    } else {
        pal.scale_dark.lerp(pal.scale_deep, -sun_l)
    };

    let col_facet_r = if sun_r >= 0.0 {
        pal.scale_dark.lerp(pal.scale_mid, sun_r)
    } else {
        pal.scale_dark.lerp(pal.scale_deep, -sun_r)
    };

    // Forward Radiance Emissive Bloom on Snout & Maxilla Facets
    let col_facet_l = if flame_bloom > 0.0 { col_facet_l.lerp(bloom_col, flame_bloom * 0.65) } else { col_facet_l };
    let col_facet_r = if flame_bloom > 0.0 { col_facet_r.lerp(bloom_col, flame_bloom * 0.65) } else { col_facet_r };
    let snout_ridge_col = if flame_bloom > 0.0 { pal.scale_hi.lerp(bloom_col, flame_bloom * 0.80) } else { pal.scale_hi };

    if is_dinosaurian {
        // DINOSAURIAN / T. REX: Heavy, boxy, squared-off pre-maxilla with bilateral facet shading
        let snout_w = if is_vhagar || is_balerion { 5.5 } else if is_vermithor { 5.0 } else { 4.0 } * scale;
        let s_mid_front = (snout_pt.0 + 1.2 * scale * fx, snout_pt.1 + 1.2 * scale * fy);
        let s_front_l = (snout_pt.0 - snout_w * rx + 1.2 * scale * fx, snout_pt.1 - snout_w * ry + 1.2 * scale * fy);
        let s_front_r = (snout_pt.0 + snout_w * rx + 1.2 * scale * fx, snout_pt.1 + snout_w * ry + 1.2 * scale * fy);
        let s_mid_nasal = (snout_pt.0 - 3.5 * scale * fx, snout_pt.1 - 3.5 * scale * fy);
        let s_nasal_l = (snout_pt.0 - (snout_w * 1.25) * rx - 3.5 * scale * fx, snout_pt.1 - (snout_w * 1.25) * ry - 3.5 * scale * fy);
        let s_nasal_r = (snout_pt.0 + (snout_w * 1.25) * rx - 3.5 * scale * fx, snout_pt.1 + (snout_w * 1.25) * ry - 3.5 * scale * fy);
        let j_base_l = (jaw_pt.0 - cranium_w * rx, jaw_pt.1 - cranium_w * ry);
        let j_base_r = (jaw_pt.0 + cranium_w * rx, jaw_pt.1 + cranium_w * ry);
        let m_mid_l = (mid_head.0 - cranium_w * 0.95 * rx, mid_head.1 - cranium_w * 0.95 * ry);
        let m_mid_r = (mid_head.0 + cranium_w * 0.95 * rx, mid_head.1 + cranium_w * 0.95 * ry);

        // Left Cranial Facet (Maxilla & Temporal Fossa)
        let dino_l = [
            (s_mid_front.0.round() as i32, s_mid_front.1.round() as i32),
            (s_front_l.0.round() as i32, s_front_l.1.round() as i32),
            (s_nasal_l.0.round() as i32, s_nasal_l.1.round() as i32),
            (m_mid_l.0.round() as i32, m_mid_l.1.round() as i32),
            (j_base_l.0.round() as i32, j_base_l.1.round() as i32),
            (jaw_pt.0.round() as i32, jaw_pt.1.round() as i32),
            (mid_head.0.round() as i32, mid_head.1.round() as i32),
            (s_mid_nasal.0.round() as i32, s_mid_nasal.1.round() as i32),
        ];
        // Right Cranial Facet (Maxilla & Temporal Fossa)
        let dino_r = [
            (s_mid_front.0.round() as i32, s_mid_front.1.round() as i32),
            (s_mid_nasal.0.round() as i32, s_mid_nasal.1.round() as i32),
            (mid_head.0.round() as i32, mid_head.1.round() as i32),
            (jaw_pt.0.round() as i32, jaw_pt.1.round() as i32),
            (j_base_r.0.round() as i32, j_base_r.1.round() as i32),
            (m_mid_r.0.round() as i32, m_mid_r.1.round() as i32),
            (s_nasal_r.0.round() as i32, s_nasal_r.1.round() as i32),
            (s_front_r.0.round() as i32, s_front_r.1.round() as i32),
        ];
        canvas.fill_polygon(&dino_l, col_facet_l);
        canvas.fill_polygon(&dino_r, col_facet_r);

        // Sagittal Median Nasal Ridge Highlight
        canvas.draw_line(s_mid_front.0.round() as i32, s_mid_front.1.round() as i32, mid_head.0.round() as i32, mid_head.1.round() as i32, snout_ridge_col, 1);
    } else if is_canine {
        // CANINE / SERPENTINE / RAPTOR: Narrow, predatory, elongated snout with bilateral facet shading
        let s_lead = if is_caraxes { 4.5 * scale } else { 0.0 };
        let c_snout = (snout_pt.0 + s_lead * fx, snout_pt.1 + s_lead * fy);
        let snout_w = if is_moondancer { 1.6 } else { 2.4 } * scale;

        let s_l = (c_snout.0 - snout_w * rx - 2.5 * scale * fx, c_snout.1 - snout_w * ry - 2.5 * scale * fy);
        let s_r = (c_snout.0 + snout_w * rx - 2.5 * scale * fx, c_snout.1 + snout_w * ry - 2.5 * scale * fy);
        let j_base_l = (jaw_pt.0 - cranium_w * rx, jaw_pt.1 - cranium_w * ry);
        let j_base_r = (jaw_pt.0 + cranium_w * rx, jaw_pt.1 + cranium_w * ry);
        let m_mid_l = (mid_head.0 - cranium_w * 0.9 * rx, mid_head.1 - cranium_w * 0.9 * ry);
        let m_mid_r = (mid_head.0 + cranium_w * 0.9 * rx, mid_head.1 + cranium_w * 0.9 * ry);

        let canine_l = [
            (c_snout.0.round() as i32, c_snout.1.round() as i32),
            (s_l.0.round() as i32, s_l.1.round() as i32),
            (m_mid_l.0.round() as i32, m_mid_l.1.round() as i32),
            (j_base_l.0.round() as i32, j_base_l.1.round() as i32),
            (jaw_pt.0.round() as i32, jaw_pt.1.round() as i32),
            (mid_head.0.round() as i32, mid_head.1.round() as i32),
        ];
        let canine_r = [
            (c_snout.0.round() as i32, c_snout.1.round() as i32),
            (mid_head.0.round() as i32, mid_head.1.round() as i32),
            (jaw_pt.0.round() as i32, jaw_pt.1.round() as i32),
            (j_base_r.0.round() as i32, j_base_r.1.round() as i32),
            (m_mid_r.0.round() as i32, m_mid_r.1.round() as i32),
            (s_r.0.round() as i32, s_r.1.round() as i32),
        ];
        canvas.fill_polygon(&canine_l, col_facet_l);
        canvas.fill_polygon(&canine_r, col_facet_r);

        // Sagittal Median Keel Highlight
        canvas.draw_line(c_snout.0.round() as i32, c_snout.1.round() as i32, mid_head.0.round() as i32, mid_head.1.round() as i32, snout_ridge_col, 1);
    } else {
        // EQUINE / HORSE: Long, slender, tapering wedge snout with bilateral facet shading
        let s_lead = 2.0 * scale;
        let e_snout = (snout_pt.0 + s_lead * fx, snout_pt.1 + s_lead * fy);
        let snout_w = 2.0 * scale;

        let s_l = (e_snout.0 - snout_w * rx - 3.5 * scale * fx, e_snout.1 - snout_w * ry - 3.5 * scale * fy);
        let s_r = (e_snout.0 + snout_w * rx - 3.5 * scale * fx, e_snout.1 + snout_w * ry - 3.5 * scale * fy);
        let j_base_l = (jaw_pt.0 - cranium_w * rx, jaw_pt.1 - cranium_w * ry);
        let j_base_r = (jaw_pt.0 + cranium_w * rx, jaw_pt.1 + cranium_w * ry);
        let m_mid_l = (mid_head.0 - cranium_w * 0.88 * rx, mid_head.1 - cranium_w * 0.88 * ry);
        let m_mid_r = (mid_head.0 + cranium_w * 0.88 * rx, mid_head.1 + cranium_w * 0.88 * ry);

        let equine_l = [
            (e_snout.0.round() as i32, e_snout.1.round() as i32),
            (s_l.0.round() as i32, s_l.1.round() as i32),
            (m_mid_l.0.round() as i32, m_mid_l.1.round() as i32),
            (j_base_l.0.round() as i32, j_base_l.1.round() as i32),
            (jaw_pt.0.round() as i32, jaw_pt.1.round() as i32),
            (mid_head.0.round() as i32, mid_head.1.round() as i32),
        ];
        let equine_r = [
            (e_snout.0.round() as i32, e_snout.1.round() as i32),
            (mid_head.0.round() as i32, mid_head.1.round() as i32),
            (jaw_pt.0.round() as i32, jaw_pt.1.round() as i32),
            (j_base_r.0.round() as i32, j_base_r.1.round() as i32),
            (m_mid_r.0.round() as i32, m_mid_r.1.round() as i32),
            (s_r.0.round() as i32, s_r.1.round() as i32),
        ];
        canvas.fill_polygon(&equine_l, col_facet_l);
        canvas.fill_polygon(&equine_r, col_facet_r);

        // Sagittal Median Keel Highlight
        canvas.draw_line(e_snout.0.round() as i32, e_snout.1.round() as i32, mid_head.0.round() as i32, mid_head.1.round() as i32, snout_ridge_col, 1);
    }

    // 2. Cephalic Epidermal Scale Texture, Scutes & Micro-Texture
    canvas.draw_line(snout_pt.0.round() as i32, snout_pt.1.round() as i32, mid_head.0.round() as i32, mid_head.1.round() as i32, snout_ridge_col, 1);
    let scute_l = (mid_head.0 - 2.5 * scale * rx, mid_head.1 - 2.5 * scale * ry);
    let scute_r = (mid_head.0 + 2.5 * scale * rx, mid_head.1 + 2.5 * scale * ry);
    canvas.set_pixel(scute_l.0.round() as i32, scute_l.1.round() as i32, pal.scale_mid);
    canvas.set_pixel(scute_r.0.round() as i32, scute_r.1.round() as i32, pal.scale_mid);

    // Vhagar Lichen Patina & Calcified Rugose Osteoderm Clusters
    if is_vhagar {
        let lichen1 = Rgba(85, 105, 80, 230);
        let lichen2 = Rgba(110, 130, 95, 220);
        canvas.set_pixel((mid_head.0 - 4.0 * scale * rx).round() as i32, (mid_head.1 - 4.0 * scale * ry).round() as i32, lichen1);
        canvas.set_pixel((mid_head.0 + 3.5 * scale * rx).round() as i32, (mid_head.1 + 3.5 * scale * ry).round() as i32, lichen2);
        canvas.set_pixel((snout_pt.0 - 2.0 * scale * rx).round() as i32, (snout_pt.1 - 2.0 * scale * ry).round() as i32, lichen1);
        canvas.set_pixel((snout_pt.0 + 2.0 * scale * rx).round() as i32, (snout_pt.1 + 2.0 * scale * ry).round() as i32, lichen2);
    }

    // Sunfyre Dazzling Photonic Gold Specular Shimmer
    if is_sunfyre {
        let shimmer_col = if frame_idx % 2 == 0 { pal.scale_specular } else { Rgba::rgb(255, 255, 220) };
        canvas.set_pixel((mid_head.0 - 2.0 * scale * rx).round() as i32, (mid_head.1 - 2.0 * scale * ry).round() as i32, shimmer_col);
        canvas.set_pixel((mid_head.0 + 2.0 * scale * rx).round() as i32, (mid_head.1 + 2.0 * scale * ry).round() as i32, shimmer_col);
        canvas.set_pixel(snout_pt.0.round() as i32, snout_pt.1.round() as i32, pal.scale_specular);
    }

    // Balerion Basalt Osteoderms & Crimson Scale Seams
    if is_balerion {
        let seam_col = Rgba(180, 20, 20, 200);
        canvas.draw_line(
            (snout_pt.0 - 2.0 * scale * rx - 2.0 * scale * fx).round() as i32,
            (snout_pt.1 - 2.0 * scale * ry - 2.0 * scale * fy).round() as i32,
            (mid_head.0 - 3.5 * scale * rx).round() as i32,
            (mid_head.1 - 3.5 * scale * ry).round() as i32,
            seam_col,
            1,
        );
        canvas.draw_line(
            (snout_pt.0 + 2.0 * scale * rx - 2.0 * scale * fx).round() as i32,
            (snout_pt.1 + 2.0 * scale * ry - 2.0 * scale * fy).round() as i32,
            (mid_head.0 + 3.5 * scale * rx).round() as i32,
            (mid_head.1 + 3.5 * scale * ry).round() as i32,
            seam_col,
            1,
        );
    }

    // Wight Viserion Necrotic Exposed Bone & Cyan Frost Crack
    if is_wight {
        let bone_poly = [
            (mid_head.0.round() as i32, mid_head.1.round() as i32),
            ((mid_head.0 - 3.2 * scale * rx).round() as i32, (mid_head.1 - 3.2 * scale * ry).round() as i32),
            ((jaw_pt.0 - 2.0 * scale * rx).round() as i32, (jaw_pt.1 - 2.0 * scale * ry).round() as i32),
            (jaw_pt.0.round() as i32, jaw_pt.1.round() as i32),
        ];
        canvas.fill_polygon(&bone_poly, Rgba::rgb(220, 230, 235));
        canvas.draw_line(
            mid_head.0.round() as i32,
            mid_head.1.round() as i32,
            (mid_head.0 + 2.5 * scale * rx - 3.0 * scale * fx).round() as i32,
            (mid_head.1 + 2.5 * scale * ry - 3.0 * scale * fy).round() as i32,
            Rgba::rgb(0, 230, 255),
            1,
        );
    }

    // Sheepstealer Asymmetric Prognathic Underbite & Healed Battle Scar
    if is_sheepstealer {
        let ub_tip = (snout_pt.0 + 3.2 * scale * fx - 1.5 * scale * rx, snout_pt.1 + 3.2 * scale * fy - 1.5 * scale * ry);
        let ub_poly = [
            (snout_pt.0.round() as i32, snout_pt.1.round() as i32),
            (ub_tip.0.round() as i32, ub_tip.1.round() as i32),
            ((ub_tip.0 + 3.0 * scale * rx).round() as i32, (ub_tip.1 + 3.0 * scale * ry).round() as i32),
        ];
        canvas.fill_polygon(&ub_poly, pal.ventral_deep);
        let f1 = (ub_tip.0 - 1.5 * scale * rx + 1.2 * scale * fx, ub_tip.1 - 1.5 * scale * ry + 1.2 * scale * fy);
        let f2 = (ub_tip.0 + 2.0 * scale * rx + 0.8 * scale * fx, ub_tip.1 + 2.0 * scale * ry + 0.8 * scale * fy);
        canvas.set_pixel(f1.0.round() as i32, f1.1.round() as i32, Rgba::rgb(245, 240, 210));
        canvas.set_pixel(f2.0.round() as i32, f2.1.round() as i32, Rgba::rgb(245, 240, 210));
        let scar_start = (mid_head.0 - 4.5 * scale * rx - 2.0 * scale * fx, mid_head.1 - 4.5 * scale * ry - 2.0 * scale * fy);
        let scar_end = (snout_pt.0 - 1.5 * scale * rx, snout_pt.1 - 1.5 * scale * ry);
        canvas.draw_line(scar_start.0.round() as i32, scar_start.1.round() as i32, scar_end.0.round() as i32, scar_end.1.round() as i32, Rgba(140, 105, 80, 200), 1);
    }

    // Drogon Blood-Red Median Sagittal Keel & Temporal Subsidiary Spines
    if is_drogon {
        let keel_col = pal.wing_arm;
        canvas.draw_line(
            (snout_pt.0 - 1.0 * scale * fx).round() as i32,
            (snout_pt.1 - 1.0 * scale * fy).round() as i32,
            (jaw_pt.0 - 1.0 * scale * fx).round() as i32,
            (jaw_pt.1 - 1.0 * scale * fy).round() as i32,
            keel_col,
            2,
        );
        for &s_sign in &[-1.0_f32, 1.0_f32] {
            let t_base = (mid_head.0 + s_sign * 5.5 * scale * rx, mid_head.1 + s_sign * 5.5 * scale * ry);
            let t_tip = (t_base.0 + s_sign * 8.0 * scale * rx - 7.0 * scale * fx, t_base.1 + s_sign * 8.0 * scale * ry - 7.0 * scale * fy);
            canvas.draw_line(t_base.0.round() as i32, t_base.1.round() as i32, t_tip.0.round() as i32, t_tip.1.round() as i32, pal.horn_mid, 1);
        }
    }

    // Rhaegal Bronze Interstitial Seams on Emerald Cranium
    if is_rhaegal {
        canvas.draw_line(
            (snout_pt.0 - 2.0 * scale * rx).round() as i32,
            (snout_pt.1 - 2.0 * scale * ry).round() as i32,
            (mid_head.0 - 3.5 * scale * rx).round() as i32,
            (mid_head.1 - 3.5 * scale * ry).round() as i32,
            pal.wing_bone,
            1,
        );
        canvas.draw_line(
            (snout_pt.0 + 2.0 * scale * rx).round() as i32,
            (snout_pt.1 + 2.0 * scale * ry).round() as i32,
            (mid_head.0 + 3.5 * scale * rx).round() as i32,
            (mid_head.1 + 3.5 * scale * ry).round() as i32,
            pal.wing_bone,
            1,
        );
    }

    // 3. 3D Supraorbital Brow Arches & Orbital Cavity Recesses
    let eye_l = ((mid_head.0 - (cranium_w * 0.7) * rx).round() as i32, (mid_head.1 - (cranium_w * 0.7) * ry).round() as i32);
    let eye_r = ((mid_head.0 + (cranium_w * 0.7) * rx).round() as i32, (mid_head.1 + (cranium_w * 0.7) * ry).round() as i32);

    let brow_col = if flame_bloom > 0.0 {
        pal.scale_hi.lerp(bloom_col, flame_bloom * 0.70)
    } else {
        pal.scale_hi
    };

    for &(ex, ey, s_sign) in &[(eye_l.0, eye_l.1, -1.0_f32), (eye_r.0, eye_r.1, 1.0_f32)] {
        let b_start = (ex as f32 + s_sign * 1.5 * rx + 2.0 * scale * fx, ey as f32 + s_sign * 1.5 * ry + 2.0 * scale * fy);
        let b_end = (ex as f32 - s_sign * 1.5 * rx - 2.0 * scale * fx, ey as f32 - s_sign * 1.5 * ry - 2.0 * scale * fy);
        canvas.draw_line(b_start.0.round() as i32, b_start.1.round() as i32, b_end.0.round() as i32, b_end.1.round() as i32, brow_col, 1);
        canvas.set_pixel(ex, ey + 1, pal.scale_deep);
    }

    if is_vhagar {
        let aged_eye = Rgba::rgb(245, 235, 140);
        canvas.set_pixel(eye_l.0, eye_l.1, aged_eye);
        canvas.set_pixel(eye_r.0, eye_r.1, aged_eye);
        canvas.set_pixel(eye_l.0, eye_l.1 - 1, Rgba::rgb(180, 170, 90));
        canvas.set_pixel(eye_r.0, eye_r.1 - 1, Rgba::rgb(180, 170, 90));
    } else {
        canvas.set_pixel(eye_l.0, eye_l.1, pal.eye_glow);
        canvas.set_pixel(eye_r.0, eye_r.1, pal.eye_glow);
        canvas.set_pixel(eye_l.0, eye_l.1 - 1, Rgba::rgb(255, 255, 255));
        canvas.set_pixel(eye_r.0, eye_r.1 - 1, Rgba::rgb(255, 255, 255));
    }

    // 4. Vhagar Sagging Guttural Throat Dewlap (Ancient Iguana Feature)
    if is_vhagar {
        let dew_base = spine_pts[3];
        let dew_tip = (dew_base.0 - 8.0 * scale * fx, dew_base.1 - 8.0 * scale * fy);
        let dew_w = 7.5 * scale;
        let dew_poly = [
            (dew_base.0.round() as i32, dew_base.1.round() as i32),
            ((dew_base.0 - dew_w * rx - 4.0 * scale * fx).round() as i32, (dew_base.1 - dew_w * ry - 4.0 * scale * fy).round() as i32),
            (dew_tip.0.round() as i32, dew_tip.1.round() as i32),
            ((dew_base.0 + dew_w * rx - 4.0 * scale * fx).round() as i32, (dew_base.1 + dew_w * ry - 4.0 * scale * fy).round() as i32),
        ];
        canvas.fill_polygon(&dew_poly, pal.ventral_deep);
        canvas.draw_line(
            (dew_base.0 - dew_w * 0.7 * rx - 3.5 * scale * fx).round() as i32,
            (dew_base.1 - dew_w * 0.7 * ry - 3.5 * scale * fy).round() as i32,
            (dew_base.0 + dew_w * 0.7 * rx - 3.5 * scale * fx).round() as i32,
            (dew_base.1 + dew_w * 0.7 * ry - 3.5 * scale * fy).round() as i32,
            pal.scale_deep,
            1,
        );
    }

    // 5. Anatomical Mandibular Unhinging & Internal Oral Cavity / Teeth (Streptostylic Depth Occlusion)
    if mouth_open > 0.5 {
        let m_gap = mouth_open * 0.45 * scale;
        let jaw_l = (snout_pt.0 - (3.5 * scale + m_gap) * rx - 1.0 * scale * fx, snout_pt.1 - (3.5 * scale + m_gap) * ry - 1.0 * scale * fy);
        let jaw_r = (snout_pt.0 + (3.5 * scale + m_gap) * rx - 1.0 * scale * fx, snout_pt.1 + (3.5 * scale + m_gap) * ry - 1.0 * scale * fy);
        let throat_in = (snout_pt.0 - 2.8 * scale * fx, snout_pt.1 - 2.8 * scale * fy);
        let snout_tip = (snout_pt.0 + 1.5 * scale * fx, snout_pt.1 + 1.5 * scale * fy);

        // Posterior Pharyngeal Cavernous Occlusion Void (Deep Shadow)
        let pharynx_poly = [
            (throat_in.0.round() as i32, throat_in.1.round() as i32),
            ((throat_in.0 - 2.0 * scale * rx).round() as i32, (throat_in.1 - 2.0 * scale * ry).round() as i32),
            ((throat_in.0 + 1.0 * scale * fx).round() as i32, (throat_in.1 + 1.0 * scale * fy).round() as i32),
            ((throat_in.0 + 2.0 * scale * rx).round() as i32, (throat_in.1 + 2.0 * scale * ry).round() as i32),
        ];
        canvas.fill_polygon(&pharynx_poly, pal.scale_deep);

        // Oral Cavity Mantle (Muscular Fleshy Floor & Fleshy Tongue)
        let oral_poly = [
            (throat_in.0.round() as i32, throat_in.1.round() as i32),
            (jaw_l.0.round() as i32, jaw_l.1.round() as i32),
            (snout_tip.0.round() as i32, snout_tip.1.round() as i32),
            (jaw_r.0.round() as i32, jaw_r.1.round() as i32),
        ];
        let oral_col = if is_wight { Rgba(0, 160, 220, 210) } else { Rgba(185, 35, 15, 230) };
        canvas.fill_polygon(&oral_poly, oral_col);

        // Fleshy Basihyal Tongue with Central Sulcus
        let tongue_tip = (snout_pt.0 + 0.5 * scale * fx, snout_pt.1 + 0.5 * scale * fy);
        let tongue_col = if is_wight { Rgba(140, 220, 255, 240) } else { Rgba(220, 60, 45, 240) };
        canvas.draw_line(throat_in.0.round() as i32, throat_in.1.round() as i32, tongue_tip.0.round() as i32, tongue_tip.1.round() as i32, tongue_col, 2);

        // Quadrate-Articular Lateral Cheek Stretch Membrane Shadows
        let cheek_l = [
            (jaw_l.0.round() as i32, jaw_l.1.round() as i32),
            ((mid_head.0 - cranium_w * 0.9 * rx).round() as i32, (mid_head.1 - cranium_w * 0.9 * ry).round() as i32),
            (throat_in.0.round() as i32, throat_in.1.round() as i32),
        ];
        let cheek_r = [
            (jaw_r.0.round() as i32, jaw_r.1.round() as i32),
            ((mid_head.0 + cranium_w * 0.9 * rx).round() as i32, (mid_head.1 + cranium_w * 0.9 * ry).round() as i32),
            (throat_in.0.round() as i32, throat_in.1.round() as i32),
        ];
        let cheek_col = Rgba(pal.scale_deep.0, pal.scale_deep.1, pal.scale_deep.2, 175);
        canvas.fill_polygon(&cheek_l, cheek_col);
        canvas.fill_polygon(&cheek_r, cheek_col);

        // Interlocking Dentition: Ivory Canine Fangs & Incisors with Specular Tips
        let tooth_ivory = Rgba(240, 238, 220, 255);
        let tooth_spec = Rgba(255, 255, 255, 255);

        // Upper Rostral Incisors
        canvas.set_pixel(snout_tip.0.round() as i32, snout_tip.1.round() as i32, tooth_spec);
        // Maxillary Canines
        canvas.set_pixel(jaw_l.0.round() as i32, jaw_l.1.round() as i32, tooth_ivory);
        canvas.set_pixel((jaw_l.0 + fx).round() as i32, (jaw_l.1 + fy).round() as i32, tooth_spec);
        canvas.set_pixel(jaw_r.0.round() as i32, jaw_r.1.round() as i32, tooth_ivory);
        canvas.set_pixel((jaw_r.0 + fx).round() as i32, (jaw_r.1 + fy).round() as i32, tooth_spec);
    }

    // Nostrils & Slipstream Exhaust
    let nos_l = (snout_pt.0 - 2.5 * scale * rx - 2.0 * scale * fx, snout_pt.1 - 2.5 * scale * ry - 2.0 * scale * fy);
    let nos_r = (snout_pt.0 + 2.5 * scale * rx - 2.0 * scale * fx, snout_pt.1 + 2.5 * scale * ry - 2.0 * scale * fy);
    canvas.set_pixel(nos_l.0.round() as i32, nos_l.1.round() as i32, Rgba::rgb(10, 12, 15));
    canvas.set_pixel(nos_r.0.round() as i32, nos_r.1.round() as i32, Rgba::rgb(10, 12, 15));

    // Post-feeding satiation nostril snort
    if post_feed_glow > 0.40 {
        let snort_len = post_feed_glow * 5.0 * scale;
        let snort_col = Rgba(90, 85, 95, (post_feed_glow * 160.0) as u8);
        canvas.draw_line(nos_l.0.round() as i32, nos_l.1.round() as i32, (nos_l.0 + snort_len * fx - 1.5 * scale * rx).round() as i32, (nos_l.1 + snort_len * fy - 1.5 * scale * ry).round() as i32, snort_col, 1);
        canvas.draw_line(nos_r.0.round() as i32, nos_r.1.round() as i32, (nos_r.0 + snort_len * fx + 1.5 * scale * rx).round() as i32, (nos_r.1 + snort_len * fy + 1.5 * scale * ry).round() as i32, snort_col, 1);
    }

    if flame_progress >= -1.0 && flame_progress < 0.0 {
        let charge_p = flame_progress + 1.0;
        if charge_p > 0.3 {
            let smk_len = 4.5 * charge_p * scale;
            let smk_col = Rgba(80, 85, 95, (charge_p * 180.0) as u8);
            canvas.draw_line(nos_l.0.round() as i32, nos_l.1.round() as i32, (nos_l.0 - smk_len * fx).round() as i32, (nos_l.1 - smk_len * fy).round() as i32, smk_col, 1);
            canvas.draw_line(nos_r.0.round() as i32, nos_r.1.round() as i32, (nos_r.0 - smk_len * fx).round() as i32, (nos_r.1 - smk_len * fy).round() as i32, smk_col, 1);
        }
        if charge_p > 0.7 {
            let spk_pt = (snout_pt.0 + 2.0 * scale * fx, snout_pt.1 + 2.0 * scale * fy);
            let spk_col = if frame_idx % 2 == 0 { Rgba::rgb(255, 255, 255) } else { pal.fire_inner };
            canvas.set_pixel(spk_pt.0.round() as i32, spk_pt.1.round() as i32, spk_col);
        }
    }

    if flame_progress > 1.0 && flame_progress <= 2.0 {
        let post_p = flame_progress - 1.0;
        if post_p < 0.7 {
            let smk_len = 6.0 * (1.0 - post_p / 0.7) * scale;
            let smk_col = Rgba(90, 95, 105, ((1.0 - post_p / 0.7) * 160.0) as u8);
            canvas.draw_line(nos_l.0.round() as i32, nos_l.1.round() as i32, (nos_l.0 - smk_len * fx).round() as i32, (nos_l.1 - smk_len * fy).round() as i32, smk_col, 1);
            canvas.draw_line(nos_r.0.round() as i32, nos_r.1.round() as i32, (nos_r.0 - smk_len * fx).round() as i32, (nos_r.1 - smk_len * fy).round() as i32, smk_col, 1);
        }
        if post_p < 0.6 {
            let smolder_a = ((1.0 - post_p / 0.6) * 220.0) as u8;
            let smolder_col = if is_wight { Rgba(0, 200, 255, smolder_a) } else { Rgba(255, 80, 20, smolder_a) };
            canvas.set_pixel(snout_pt.0.round() as i32, snout_pt.1.round() as i32, smolder_col);
            canvas.set_pixel((snout_pt.0 - 1.5 * scale * rx).round() as i32, (snout_pt.1 - 1.5 * scale * ry).round() as i32, smolder_col);
            canvas.set_pixel((snout_pt.0 + 1.5 * scale * rx).round() as i32, (snout_pt.1 + 1.5 * scale * ry).round() as i32, smolder_col);
        }
    }

    // 6. Species-Specific Cephalic Structures & Cylindrical Horn Morphology (Limitation 3)
    let eff_horn_hi = if flame_bloom > 0.0 { pal.horn_hi.lerp(bloom_col, flame_bloom * 0.45) } else { pal.horn_hi };
    if is_vermithor {
        // Spiny chin osteoderm "beard" lining lower jaw with cylindrical normal shading
        for &b_offset in &[-4.5_f32, -2.5, 0.0, 2.5, 4.5] {
            let b_base = (jaw_pt.0 + b_offset * scale * rx - 1.5 * scale * fx, jaw_pt.1 + b_offset * scale * ry - 1.5 * scale * fy);
            let b_tip = (b_base.0 + b_offset * 1.6 * scale * rx - 7.5 * scale * fx, b_base.1 + b_offset * 1.6 * scale * ry - 7.5 * scale * fy);
            let h_dx = b_tip.0 - b_base.0;
            let h_dy = b_tip.1 - b_base.1;
            let h_len = (h_dx * h_dx + h_dy * h_dy).sqrt().max(1.0);
            let (hn_x, hn_y) = (-h_dy / h_len, h_dx / h_len);
            let sun_h = (-0.55 * hn_x + -0.65 * hn_y).clamp(-1.0, 1.0);
            let col_pos = if sun_h >= 0.0 { pal.horn_mid.lerp(eff_horn_hi, sun_h) } else { pal.horn_mid.lerp(pal.horn_dark, -sun_h) };
            let col_neg = if sun_h >= 0.0 { pal.horn_mid.lerp(pal.horn_dark, sun_h) } else { pal.horn_mid.lerp(eff_horn_hi, -sun_h) };

            canvas.draw_line((b_base.0 - hn_x).round() as i32, (b_base.1 - hn_y).round() as i32, b_tip.0.round() as i32, b_tip.1.round() as i32, col_neg, 1);
            canvas.draw_line(b_base.0.round() as i32, b_base.1.round() as i32, b_tip.0.round() as i32, b_tip.1.round() as i32, pal.horn_mid, 2);
            canvas.draw_line((b_base.0 + hn_x).round() as i32, (b_base.1 + hn_y).round() as i32, b_tip.0.round() as i32, b_tip.1.round() as i32, col_pos, 1);
            canvas.set_pixel(b_tip.0.round() as i32, b_tip.1.round() as i32, pal.scale_specular);
        }
    }

    if is_caraxes {
        // Vascular cranial ear-winglets with structural ribs & cylindrical bone shading
        for &side_sign in &[-1.0_f32, 1.0_f32] {
            let side = side_sign;
            let h_base = (jaw_pt.0 + side * 6.5 * scale * rx, jaw_pt.1 + side * 6.5 * scale * ry);
            let h_tip = (h_base.0 + side * 16.0 * scale * rx - 13.0 * scale * fx, h_base.1 + side * 16.0 * scale * ry - 13.0 * scale * fy);
            let h_mid_web = (h_base.0 + side * 12.0 * scale * rx - 6.0 * scale * fx, h_base.1 + side * 12.0 * scale * ry - 6.0 * scale * fy);
            let frill_poly = [
                (h_base.0.round() as i32, h_base.1.round() as i32),
                (h_mid_web.0.round() as i32, h_mid_web.1.round() as i32),
                (h_tip.0.round() as i32, h_tip.1.round() as i32),
                ((h_base.0 - 10.0 * scale * fx).round() as i32, (h_base.1 - 10.0 * scale * fy).round() as i32),
            ];
            let mut fr_col = pal.wing_web1;
            fr_col.3 = 235;
            canvas.fill_polygon(&frill_poly, fr_col);
            canvas.draw_line((h_base.0 - fx).round() as i32, (h_base.1 - fy).round() as i32, h_tip.0.round() as i32, h_tip.1.round() as i32, pal.horn_dark, 1);
            canvas.draw_line(h_base.0.round() as i32, h_base.1.round() as i32, h_tip.0.round() as i32, h_tip.1.round() as i32, pal.horn_hi, 2);
            canvas.draw_line(h_base.0.round() as i32, h_base.1.round() as i32, h_mid_web.0.round() as i32, h_mid_web.1.round() as i32, pal.wing_bone, 1);
            canvas.set_pixel(h_tip.0.round() as i32, h_tip.1.round() as i32, pal.scale_specular);
        }
    } else if is_meleys {
        // Sovereign horn crown (tiara crest encircling posterior cranium with cylindrical highlights)
        let mut crown_pts = Vec::new();
        for &sp_angle in &[-1.0_f32, -0.7, -0.4, -0.15, 0.15, 0.4, 0.7, 1.0] {
            let sp_dir_x = (angle + PI + sp_angle * 0.75).cos();
            let sp_dir_y = (angle + PI + sp_angle * 0.75).sin();
            let h_base = (jaw_pt.0 + sp_angle * 9.0 * scale * rx, jaw_pt.1 + sp_angle * 9.0 * scale * ry);
            let h_len = if sp_angle.abs() > 0.6 { 15.0 } else { 12.0 } * scale;
            let h_tip = (h_base.0 + sp_dir_x * h_len, h_base.1 + sp_dir_y * h_len);
            let hn_x = -sp_dir_y;
            let hn_y = sp_dir_x;
            let sun_h = (-0.55 * hn_x + -0.65 * hn_y).clamp(-1.0, 1.0);
            let col_pos = if sun_h >= 0.0 { pal.horn_mid.lerp(eff_horn_hi, sun_h) } else { pal.horn_mid.lerp(pal.horn_dark, -sun_h) };
            let col_neg = if sun_h >= 0.0 { pal.horn_mid.lerp(pal.horn_dark, sun_h) } else { pal.horn_mid.lerp(eff_horn_hi, -sun_h) };

            canvas.draw_line((h_base.0 - hn_x).round() as i32, (h_base.1 - hn_y).round() as i32, h_tip.0.round() as i32, h_tip.1.round() as i32, col_neg, 1);
            canvas.draw_line(h_base.0.round() as i32, h_base.1.round() as i32, h_tip.0.round() as i32, h_tip.1.round() as i32, pal.horn_mid, 2);
            canvas.draw_line((h_base.0 + hn_x).round() as i32, (h_base.1 + hn_y).round() as i32, h_tip.0.round() as i32, h_tip.1.round() as i32, col_pos, 1);
            canvas.set_pixel(h_tip.0.round() as i32, h_tip.1.round() as i32, pal.scale_specular);
            crown_pts.push((h_base.0.round() as i32, h_base.1.round() as i32));
        }
        for k in 0..crown_pts.len() - 1 {
            canvas.draw_line(crown_pts[k].0, crown_pts[k].1, crown_pts[k + 1].0, crown_pts[k + 1].1, pal.horn_hi, 1);
        }
    } else if is_moondancer {
        // Punk-rock feathered/quill mohawk crest along sagittal ridge
        let crest_pts = [
            (snout_pt.0 - 2.0 * scale * fx, snout_pt.1 - 2.0 * scale * fy, 3.5 * scale),
            (mid_head.0 - 1.0 * scale * fx, mid_head.1 - 1.0 * scale * fy, 6.0 * scale),
            (jaw_pt.0, jaw_pt.1, 8.0 * scale),
            (spine_pts[4].0, spine_pts[4].1, 6.5 * scale),
        ];
        for &(cx, cy, q_len) in &crest_pts {
            let q_tip = (cx - q_len * fx + 1.5 * scale * rx, cy - q_len * fy + 1.5 * scale * ry);
            canvas.draw_line((cx - rx).round() as i32, (cy - ry).round() as i32, q_tip.0.round() as i32, q_tip.1.round() as i32, pal.horn_dark, 1);
            canvas.draw_line(cx.round() as i32, cy.round() as i32, q_tip.0.round() as i32, q_tip.1.round() as i32, pal.horn_hi, 2);
            canvas.set_pixel(q_tip.0.round() as i32, q_tip.1.round() as i32, pal.scale_specular);
        }
    } else if is_sheepstealer {
        // Asymmetric gnarly horns with cylindrical normal shading
        let h_base_l = (jaw_pt.0 - 6.5 * scale * rx, jaw_pt.1 - 6.5 * scale * ry);
        let h_tip_l = (h_base_l.0 - 16.0 * scale * rx - 10.0 * scale * fx, h_base_l.1 - 16.0 * scale * ry - 10.0 * scale * fy);
        let h_base_r = (jaw_pt.0 + 6.5 * scale * rx, jaw_pt.1 + 6.5 * scale * ry);
        let h_tip_r = (h_base_r.0 + 8.5 * scale * rx - 19.0 * scale * fx, h_base_r.1 + 8.5 * scale * ry - 19.0 * scale * fy);

        // Left Horn
        canvas.draw_line((h_base_l.0 - fx).round() as i32, (h_base_l.1 - fy).round() as i32, h_tip_l.0.round() as i32, h_tip_l.1.round() as i32, pal.horn_dark, 1);
        canvas.draw_line(h_base_l.0.round() as i32, h_base_l.1.round() as i32, h_tip_l.0.round() as i32, h_tip_l.1.round() as i32, pal.horn_mid, 2);
        canvas.draw_line((h_base_l.0 + fx).round() as i32, (h_base_l.1 + fy).round() as i32, h_tip_l.0.round() as i32, h_tip_l.1.round() as i32, pal.horn_hi, 1);
        canvas.set_pixel(h_tip_l.0.round() as i32, h_tip_l.1.round() as i32, pal.scale_specular);

        // Right Horn
        canvas.draw_line((h_base_r.0 - fx).round() as i32, (h_base_r.1 - fy).round() as i32, h_tip_r.0.round() as i32, h_tip_r.1.round() as i32, pal.horn_dark, 1);
        canvas.draw_line(h_base_r.0.round() as i32, h_base_r.1.round() as i32, h_tip_r.0.round() as i32, h_tip_r.1.round() as i32, pal.horn_mid, 2);
        canvas.draw_line((h_base_r.0 + fx).round() as i32, (h_base_r.1 + fy).round() as i32, h_tip_r.0.round() as i32, h_tip_r.1.round() as i32, pal.horn_hi, 1);
        canvas.set_pixel(h_tip_r.0.round() as i32, h_tip_r.1.round() as i32, pal.scale_specular);
    } else if is_drogon {
        // Swept-back apex predatory horns with 3D cylindrical lighting
        for &side_sign in &[-1.0_f32, 1.0_f32] {
            let side = side_sign;
            let h_base = (jaw_pt.0 + side * 7.5 * scale * rx, jaw_pt.1 + side * 7.5 * scale * ry);
            let h_tip = (
                h_base.0 + side * 9.0 * scale * rx - 22.0 * scale * fx,
                h_base.1 + side * 9.0 * scale * ry - 22.0 * scale * fy,
            );
            let h_dx = h_tip.0 - h_base.0;
            let h_dy = h_tip.1 - h_base.1;
            let h_len = (h_dx * h_dx + h_dy * h_dy).sqrt().max(1.0);
            let (hn_x, hn_y) = (-h_dy / h_len, h_dx / h_len);
            let sun_h = (-0.55 * hn_x + -0.65 * hn_y).clamp(-1.0, 1.0);

            let col_pos = if sun_h >= 0.0 {
                pal.horn_mid.lerp(eff_horn_hi, sun_h)
            } else {
                pal.horn_mid.lerp(pal.horn_dark, -sun_h)
            };
            let col_neg = if sun_h >= 0.0 {
                pal.horn_mid.lerp(pal.horn_dark, sun_h)
            } else {
                pal.horn_mid.lerp(eff_horn_hi, -sun_h)
            };

            canvas.draw_line((h_base.0 - hn_x).round() as i32, (h_base.1 - hn_y).round() as i32, h_tip.0.round() as i32, h_tip.1.round() as i32, col_neg, 1);
            canvas.draw_line(h_base.0.round() as i32, h_base.1.round() as i32, h_tip.0.round() as i32, h_tip.1.round() as i32, pal.horn_mid, 2);
            canvas.draw_line((h_base.0 + hn_x).round() as i32, (h_base.1 + hn_y).round() as i32, (h_tip.0 + fx).round() as i32, (h_tip.1 + fy).round() as i32, col_pos, 1);
            canvas.set_pixel(h_tip.0.round() as i32, h_tip.1.round() as i32, pal.scale_specular);
        }
    } else if is_rhaegal {
        // Dual-tiered horn architecture (upper swept, lower lateral) with cylindrical shading
        for &side_sign in &[-1.0_f32, 1.0_f32] {
            let side = side_sign;
            let h_base = (jaw_pt.0 + side * 6.5 * scale * rx, jaw_pt.1 + side * 6.5 * scale * ry);
            let h_up = (h_base.0 + side * 9.5 * scale * rx - 19.0 * scale * fx, h_base.1 + side * 9.5 * scale * ry - 19.0 * scale * fy);
            let h_low = (h_base.0 + side * 15.0 * scale * rx - 7.0 * scale * fx, h_base.1 + side * 15.0 * scale * ry - 7.0 * scale * fy);

            canvas.draw_line((h_base.0 - fx).round() as i32, (h_base.1 - fy).round() as i32, h_up.0.round() as i32, h_up.1.round() as i32, pal.horn_dark, 1);
            canvas.draw_line(h_base.0.round() as i32, h_base.1.round() as i32, h_up.0.round() as i32, h_up.1.round() as i32, pal.horn_mid, 2);
            canvas.draw_line((h_base.0 + fx).round() as i32, (h_base.1 + fy).round() as i32, h_up.0.round() as i32, h_up.1.round() as i32, pal.horn_hi, 1);

            canvas.draw_line((h_base.0 - fx).round() as i32, (h_base.1 - fy).round() as i32, h_low.0.round() as i32, h_low.1.round() as i32, pal.horn_dark, 1);
            canvas.draw_line(h_base.0.round() as i32, h_base.1.round() as i32, h_low.0.round() as i32, h_low.1.round() as i32, pal.horn_hi, 2);
            canvas.set_pixel(h_up.0.round() as i32, h_up.1.round() as i32, pal.scale_specular);
            canvas.set_pixel(h_low.0.round() as i32, h_low.1.round() as i32, pal.scale_specular);
        }
    } else if is_viserion {
        // Alpine Ram/Ibex downward jaw-curving horns with cylindrical relief
        for &side_sign in &[-1.0_f32, 1.0_f32] {
            let side = side_sign;
            let h_base = (jaw_pt.0 + side * 7.5 * scale * rx, jaw_pt.1 + side * 7.5 * scale * ry);
            let h_curve = (h_base.0 + side * 14.5 * scale * rx - 6.0 * scale * fx, h_base.1 + side * 14.5 * scale * ry - 6.0 * scale * fy);
            let h_tip = (h_curve.0 - side * 5.0 * scale * rx + 6.0 * scale * fx, h_curve.1 - side * 5.0 * scale * ry + 6.0 * scale * fy);

            canvas.draw_line((h_base.0 - fx).round() as i32, (h_base.1 - fy).round() as i32, h_curve.0.round() as i32, h_curve.1.round() as i32, pal.horn_dark, 1);
            canvas.draw_line(h_base.0.round() as i32, h_base.1.round() as i32, h_curve.0.round() as i32, h_curve.1.round() as i32, pal.horn_mid, 2);
            canvas.draw_line(h_curve.0.round() as i32, h_curve.1.round() as i32, h_tip.0.round() as i32, h_tip.1.round() as i32, pal.horn_hi, 2);
            canvas.draw_line((h_curve.0 + fx).round() as i32, (h_curve.1 + fy).round() as i32, (h_tip.0 + fx).round() as i32, (h_tip.1 + fy).round() as i32, pal.horn_hi, 1);
            canvas.set_pixel(h_tip.0.round() as i32, h_tip.1.round() as i32, pal.scale_specular);
        }
    } else if is_wight {
        // Broken stub horn + intact downward horn with frost cylindrical glints
        for &side_sign in &[-1.0_f32, 1.0_f32] {
            let side = side_sign;
            let h_base = (jaw_pt.0 + side * 7.5 * scale * rx, jaw_pt.1 + side * 7.5 * scale * ry);
            if side < 0.0 {
                let h_stub = (h_base.0 + side * 7.0 * scale * rx - 4.0 * scale * fx, h_base.1 + side * 7.0 * scale * ry - 4.0 * scale * fy);
                canvas.draw_line((h_base.0 - fx).round() as i32, (h_base.1 - fy).round() as i32, h_stub.0.round() as i32, h_stub.1.round() as i32, pal.horn_dark, 1);
                canvas.draw_line(h_base.0.round() as i32, h_base.1.round() as i32, h_stub.0.round() as i32, h_stub.1.round() as i32, pal.horn_mid, 2);
                canvas.set_pixel(h_stub.0.round() as i32, h_stub.1.round() as i32, Rgba::rgb(220, 240, 255));
            } else {
                let h_curve = (h_base.0 + side * 14.0 * scale * rx - 5.0 * scale * fx, h_base.1 + side * 14.0 * scale * ry - 5.0 * scale * fy);
                let h_tip = (h_curve.0 - side * 5.0 * scale * rx + 5.0 * scale * fx, h_curve.1 - side * 5.0 * scale * ry + 5.0 * scale * fy);
                canvas.draw_line((h_base.0 - fx).round() as i32, (h_base.1 - fy).round() as i32, h_curve.0.round() as i32, h_curve.1.round() as i32, pal.horn_dark, 1);
                canvas.draw_line(h_base.0.round() as i32, h_base.1.round() as i32, h_curve.0.round() as i32, h_curve.1.round() as i32, pal.horn_mid, 2);
                canvas.draw_line(h_curve.0.round() as i32, h_curve.1.round() as i32, h_tip.0.round() as i32, h_tip.1.round() as i32, pal.horn_hi, 1);
                canvas.set_pixel(h_tip.0.round() as i32, h_tip.1.round() as i32, Rgba::rgb(0, 240, 255));
            }
        }
    } else if is_syrax {
        // Regal golden crown crest with cylindrical specular sheen
        for &sp_angle in &[-0.8_f32, -0.4, 0.0, 0.4, 0.8] {
            let h_base = (jaw_pt.0 + sp_angle * 7.0 * scale * rx, jaw_pt.1 + sp_angle * 7.0 * scale * ry);
            let h_len = (16.0 - sp_angle.abs() * 4.0) * scale;
            let h_tip = (h_base.0 + sp_angle * 9.0 * scale * rx - h_len * fx, h_base.1 + sp_angle * 9.0 * scale * ry - h_len * fy);
            canvas.draw_line((h_base.0 - rx).round() as i32, (h_base.1 - ry).round() as i32, h_tip.0.round() as i32, h_tip.1.round() as i32, pal.horn_dark, 1);
            canvas.draw_line(h_base.0.round() as i32, h_base.1.round() as i32, h_tip.0.round() as i32, h_tip.1.round() as i32, pal.horn_hi, 2);
            canvas.set_pixel(h_tip.0.round() as i32, h_tip.1.round() as i32, pal.scale_specular);
        }
    } else if is_balerion {
        // Battering-ram titanic basalt horns with deep 3D cylindrical normal bevels
        for &side_sign in &[-1.0_f32, 1.0_f32] {
            let side = side_sign;
            let h_base = (jaw_pt.0 + side * 9.0 * scale * rx, jaw_pt.1 + side * 9.0 * scale * ry);
            let h_mid = (h_base.0 + side * 13.0 * scale * rx - 14.0 * scale * fx, h_base.1 + side * 13.0 * scale * ry - 14.0 * scale * fy);
            let h_tip = (h_mid.0 + side * 3.0 * scale * rx - 14.0 * scale * fx, h_mid.1 + side * 3.0 * scale * ry - 14.0 * scale * fy);
            canvas.draw_line((h_base.0 - fx).round() as i32, (h_base.1 - fy).round() as i32, h_mid.0.round() as i32, h_mid.1.round() as i32, pal.horn_dark, 2);
            canvas.draw_line(h_base.0.round() as i32, h_base.1.round() as i32, h_mid.0.round() as i32, h_mid.1.round() as i32, pal.horn_mid, 4);
            canvas.draw_line((h_base.0 + fx).round() as i32, (h_base.1 + fx).round() as i32, h_mid.0.round() as i32, h_mid.1.round() as i32, pal.horn_hi, 1);
            canvas.draw_line(h_mid.0.round() as i32, h_mid.1.round() as i32, h_tip.0.round() as i32, h_tip.1.round() as i32, pal.horn_hi, 2);
            canvas.set_pixel(h_tip.0.round() as i32, h_tip.1.round() as i32, Rgba::rgb(220, 20, 20));
        }
    } else if is_tessarion {
        // Metallic copper horns sweeping gracefully backward with cylindrical normal highlights
        for &side_sign in &[-1.0_f32, 1.0_f32] {
            let side = side_sign;
            let h_base = (jaw_pt.0 + side * 7.0 * scale * rx, jaw_pt.1 + side * 7.0 * scale * ry);
            let h_tip = (
                h_base.0 + side * 10.0 * scale * rx - 20.0 * scale * fx,
                h_base.1 + side * 10.0 * scale * ry - 20.0 * scale * fy,
            );
            canvas.draw_line((h_base.0 - fx).round() as i32, (h_base.1 - fy).round() as i32, h_tip.0.round() as i32, h_tip.1.round() as i32, pal.horn_dark, 1);
            canvas.draw_line(h_base.0.round() as i32, h_base.1.round() as i32, h_tip.0.round() as i32, h_tip.1.round() as i32, pal.horn_mid, 2);
            canvas.draw_line((h_base.0 + fx).round() as i32, (h_base.1 + fx).round() as i32, (h_tip.0 + fx).round() as i32, (h_tip.1 + fy).round() as i32, pal.horn_hi, 1);
            canvas.set_pixel(h_tip.0.round() as i32, h_tip.1.round() as i32, pal.scale_specular);
        }
    } else {
        // Standard streamlined horns with cylindrical highlight & underside shadow
        for &side_sign in &[-1.0_f32, 1.0_f32] {
            let side = side_sign;
            let h_base = (jaw_pt.0 + side * 6.5 * scale * rx, jaw_pt.1 + side * 6.5 * scale * ry);
            let h_tip = (h_base.0 + side * 11.0 * scale * rx - 19.0 * scale * fx, h_base.1 + side * 11.0 * scale * ry - 19.0 * scale * fy);
            canvas.draw_line((h_base.0 - fx).round() as i32, (h_base.1 - fy).round() as i32, h_tip.0.round() as i32, h_tip.1.round() as i32, pal.horn_dark, 1);
            canvas.draw_line(h_base.0.round() as i32, h_base.1.round() as i32, h_tip.0.round() as i32, h_tip.1.round() as i32, pal.horn_mid, 2);
            canvas.draw_line((h_base.0 + fx).round() as i32, (h_base.1 + fx).round() as i32, (h_tip.0 + fx).round() as i32, (h_tip.1 + fy).round() as i32, pal.horn_hi, 1);
            canvas.set_pixel(h_tip.0.round() as i32, h_tip.1.round() as i32, pal.scale_specular);
        }
    }

    // -----------------------------------------------------------------
    // 9. ADVANCED COMBUSTION PHYSICS & 16-SPECIES FLAME SPLITTING
    // -----------------------------------------------------------------
    if flame_progress >= 0.0 && flame_progress <= 1.0 {
        // Base Flame Length grows with flame timeline (bloom -> sustain -> dissipate)
        let grow_factor = if flame_progress < 0.20 {
            flame_progress / 0.20
        } else if flame_progress > 0.85 {
            (1.0 - flame_progress) / 0.15
        } else {
            1.0
        };

        let base_fl_len = if is_vhagar {
            68.0 // Colossal catastrophic furnace cloud
        } else if is_balerion {
            60.0
        } else if is_moondancer {
            36.0 // High-pressure cutting torch
        } else {
            46.0
        } * scale * grow_factor;

        // Dual Sublingual Fuel Glands (Hypergolic Convergence Node)
        let gland_l = (snout_pt.0 - 2.8 * scale * rx - 1.5 * scale * fx, snout_pt.1 - 2.8 * scale * ry - 1.5 * scale * fy);
        let gland_r = (snout_pt.0 + 2.8 * scale * rx - 1.5 * scale * fx, snout_pt.1 + 2.8 * scale * ry - 1.5 * scale * fy);
        let ign_node = (snout_pt.0 + 5.0 * scale * fx, snout_pt.1 + 5.0 * scale * fy);

        // Render converging liquid propellant jets prior to collision ignition
        let fuel_stream_col = if is_wight {
            Rgba::rgb(0, 220, 255)
        } else if is_rhaegal {
            Rgba::rgb(118, 255, 3)
        } else {
            Rgba::rgb(255, 200, 50)
        };
        canvas.draw_line(gland_l.0.round() as i32, gland_l.1.round() as i32, ign_node.0.round() as i32, ign_node.1.round() as i32, fuel_stream_col, 1);
        canvas.draw_line(gland_r.0.round() as i32, gland_r.1.round() as i32, ign_node.0.round() as i32, ign_node.1.round() as i32, fuel_stream_col, 1);
        
        // Collision Ignition Flash & Concussive Detonation Shockwave Ring
        if flame_progress < 0.15 {
            let det_p = flame_progress / 0.15;
            let shock_r = (5.0 + 16.0 * det_p) * scale;
            let shock_a = ((1.0 - det_p) * 220.0) as u8;
            let shock_col = if is_wight { Rgba(0, 220, 255, shock_a) } else { Rgba(255, 180, 50, shock_a) };
            canvas.fill_ellipse(ign_node.0.round() as i32, ign_node.1.round() as i32, shock_r.round() as i32, (shock_r * 0.7).round() as i32, shock_col);
            canvas.fill_circle(ign_node.0.round() as i32, ign_node.1.round() as i32, (3.5 * scale).round() as i32, Rgba::rgb(255, 255, 255));
        } else {
            canvas.set_pixel(ign_node.0.round() as i32, ign_node.1.round() as i32, Rgba::rgb(255, 255, 255));
        }

        // Slipstream Wind Shear Drag Vector
        let drag_x = -vx * 0.12 * scale;
        let drag_y = -vy * 0.12 * scale;

        // -------------------------------------------------------------
        // DYNAMIC DRAGONFIRE EMISSIVE RADIANCE (Forward Quadrant Illumination - Limitation 9)
        // -------------------------------------------------------------
        let rad_intensity = grow_factor;
        let em_core = if is_wight {
            Rgba(200, 245, 255, (rad_intensity * 255.0) as u8)
        } else if is_balerion {
            Rgba(255, 60, 40, (rad_intensity * 255.0) as u8)
        } else if is_rhaegal {
            Rgba(160, 255, 60, (rad_intensity * 255.0) as u8)
        } else if is_sunfyre {
            Rgba(255, 240, 120, (rad_intensity * 255.0) as u8)
        } else if is_tessarion {
            Rgba(100, 200, 255, (rad_intensity * 255.0) as u8)
        } else {
            Rgba(255, 190, 80, (rad_intensity * 255.0) as u8)
        };
        let em_mid = pal.fire_inner;
        let em_outer = pal.fire_outer;

        // 1. Snout & Pre-maxilla incandescent bloom
        canvas.fill_circle(snout_pt.0.round() as i32, snout_pt.1.round() as i32, (3.0 * scale).round() as i32, em_mid);
        canvas.set_pixel((snout_pt.0 + fx).round() as i32, (snout_pt.1 + fy).round() as i32, pal.fire_core);

        // 2. Maxilla lateral flanks lit by forward light emission
        let m_flare_l = (snout_pt.0 - 2.5 * scale * rx, snout_pt.1 - 2.5 * scale * ry);
        let m_flare_r = (snout_pt.0 + 2.5 * scale * rx, snout_pt.1 + 2.5 * scale * ry);
        canvas.set_pixel(m_flare_l.0.round() as i32, m_flare_l.1.round() as i32, em_core);
        canvas.set_pixel(m_flare_r.0.round() as i32, m_flare_r.1.round() as i32, em_core);

        // 3. Supraorbital brow arches & ocular fire reflections
        canvas.set_pixel(eye_l.0, eye_l.1, em_core);
        canvas.set_pixel(eye_r.0, eye_r.1, em_core);

        // 4. Anterior cervical neck scutes (nodes 1..3 forward specular wash)
        for &sc_idx in &[1, 2, 3] {
            let sc_pt = spine_pts[sc_idx];
            let sc_wash = (sc_pt.0 + 1.2 * scale * fx, sc_pt.1 + 1.2 * scale * fy);
            let mut sc_c = em_outer;
            sc_c.3 = (160.0 * rad_intensity) as u8;
            canvas.set_pixel(sc_wash.0.round() as i32, sc_wash.1.round() as i32, sc_c);
        }

        // 5. Leading Edge Wing Propatagium & Shoulder Anchors
        for w in &wings {
            let sh_wash = (w.sh_anchor.0 + 2.0 * scale * fx, w.sh_anchor.1 + 2.0 * scale * fy);
            let el_wash = (w.elbow.0 + 1.5 * scale * fx, w.elbow.1 + 1.5 * scale * fy);
            let mut prop_c = em_outer;
            prop_c.3 = (140.0 * rad_intensity) as u8;
            canvas.draw_line(sh_wash.0.round() as i32, sh_wash.1.round() as i32, el_wash.0.round() as i32, el_wash.1.round() as i32, prop_c, 1);
            canvas.set_pixel(el_wash.0.round() as i32, el_wash.1.round() as i32, em_mid);
        }

        // -------------------------------------------------------------
        // FLAME SPLITTING & DYNAMICS BY SPECIES
        // -------------------------------------------------------------
        if is_wight {
            // 9. Wight Viserion: Collimated Coherent Magnetic Pinch Plasma Beam & Ice Shrapnel
            let beam_tip = (ign_node.0 + base_fl_len * fx, ign_node.1 + base_fl_len * fy);
            // Non-diverging parallel magnetic beam
            canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, beam_tip.0.round() as i32, beam_tip.1.round() as i32, Rgba::rgb(0, 210, 255), 5);
            canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, beam_tip.0.round() as i32, beam_tip.1.round() as i32, Rgba::rgb(124, 58, 237), 3);
            canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, beam_tip.0.round() as i32, beam_tip.1.round() as i32, Rgba::rgb(255, 255, 255), 1);

            // Explosive Ionizing Terminal Burst & Scattering Ice Crystals
            for a in 0..8 {
                let ang = a as f32 * PI / 4.0 + (frame_idx as f32 * 0.4);
                let dist_ice = (6.0 + (frame_idx % 4) as f32 * 1.5) * scale;
                let ix = (beam_tip.0 + ang.cos() * dist_ice).round() as i32;
                let iy = (beam_tip.1 + ang.sin() * dist_ice).round() as i32;
                canvas.set_pixel(ix, iy, Rgba::rgb(220, 245, 255));
            }
        } else if is_balerion {
            // 6. Balerion: Helical Swirling Bifurcation of Carbonaceous Soot & Crimson Plasma
            let spiral_t = frame_idx as f32 * 0.4;
            // Tongue A & Tongue B form intertwined helical vortices
            let fl_tip_a = (
                ign_node.0 + base_fl_len * fx + (spiral_t.sin() * 8.0 * scale) * rx + drag_x,
                ign_node.1 + base_fl_len * fy + (spiral_t.sin() * 8.0 * scale) * ry + drag_y,
            );
            let fl_tip_b = (
                ign_node.0 + base_fl_len * fx - (spiral_t.sin() * 8.0 * scale) * rx + drag_x,
                ign_node.1 + base_fl_len * fy - (spiral_t.sin() * 8.0 * scale) * ry + drag_y,
            );

            // Pitch-Black Carbon Soot Outer Shroud
            let soot_poly = [
                (ign_node.0.round() as i32, ign_node.1.round() as i32),
                ((fl_tip_a.0 - 9.0 * scale * rx).round() as i32, (fl_tip_a.1 - 9.0 * scale * ry).round() as i32),
                (fl_tip_a.0.round() as i32, fl_tip_a.1.round() as i32),
                ((fl_tip_b.0 + 9.0 * scale * rx).round() as i32, (fl_tip_b.1 + 9.0 * scale * ry).round() as i32),
            ];
            canvas.fill_polygon(&soot_poly, Rgba::rgb(14, 10, 14));

            // Swirling Blood-Red Crimson Core
            canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, fl_tip_a.0.round() as i32, fl_tip_a.1.round() as i32, Rgba::rgb(220, 38, 38), 3);
            canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, fl_tip_b.0.round() as i32, fl_tip_b.1.round() as i32, Rgba::rgb(220, 38, 38), 3);
            canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, ((fl_tip_a.0 + fl_tip_b.0) * 0.5).round() as i32, ((fl_tip_a.1 + fl_tip_b.1) * 0.5).round() as i32, Rgba::rgb(255, 23, 68), 1);
        } else if is_rhaegal {
            // 7. Rhaegal: Bifurcated V-Fork Split with Emerald Spark Sheath
            let fork_ang = 0.24; // ~14 degrees fork angle
            let fork_dir1_x = (angle + fork_ang).cos();
            let fork_dir1_y = (angle + fork_ang).sin();
            let fork_dir2_x = (angle - fork_ang).cos();
            let fork_dir2_y = (angle - fork_ang).sin();

            let tip1 = (ign_node.0 + base_fl_len * fork_dir1_x + drag_x, ign_node.1 + base_fl_len * fork_dir1_y + drag_y);
            let tip2 = (ign_node.0 + base_fl_len * fork_dir2_x + drag_x, ign_node.1 + base_fl_len * fork_dir2_y + drag_y);

            // Emerald Green Outer Boundary Sheaths
            canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, tip1.0.round() as i32, tip1.1.round() as i32, Rgba::rgb(118, 255, 3), 4);
            canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, tip2.0.round() as i32, tip2.1.round() as i32, Rgba::rgb(118, 255, 3), 4);

            // Inner Molten Bronze-Orange Cores
            canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, tip1.0.round() as i32, tip1.1.round() as i32, pal.fire_inner, 2);
            canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, tip2.0.round() as i32, tip2.1.round() as i32, pal.fire_inner, 2);
        } else if is_meleys {
            // 12. Meleys: High-Speed Pulsed Fire Strafing (Discrete Traveling Flame Bullets)
            for bullet_idx in 0..3 {
                let bullet_phase = (frame_idx as f32 * 0.35 + bullet_idx as f32 * 0.33).fract();
                let b_dist = base_fl_len * bullet_phase;
                let b_pt = (
                    ign_node.0 + b_dist * fx + drag_x * bullet_phase * 1.5,
                    ign_node.1 + b_dist * fy + drag_y * bullet_phase * 1.5,
                );
                let b_r = (3.0 * scale * (1.0 + bullet_phase * 0.8)).round() as i32;
                canvas.fill_ellipse(b_pt.0.round() as i32, b_pt.1.round() as i32, b_r, (b_r as f32 * 0.6).round() as i32, Rgba::rgb(245, 100, 20));
                canvas.fill_ellipse(b_pt.0.round() as i32, b_pt.1.round() as i32, (b_r - 1).max(1), (b_r as f32 * 0.3).round() as i32, Rgba::rgb(255, 230, 80));
            }
        } else if is_vhagar {
            // 11. Vhagar: Cataclysmic Rolling Pyroclastic Furnace Cloud
            let roll_t = frame_idx as f32 * 0.3;
            let c_tip = (ign_node.0 + base_fl_len * fx + drag_x * 0.5, ign_node.1 + base_fl_len * fy + drag_y * 0.5);
            let w_c = 24.0 * scale;

            // Giant Billowing Multi-Lobed Pyroclastic Cloud
            let c_poly = [
                (ign_node.0.round() as i32, ign_node.1.round() as i32),
                ((ign_node.0 + (base_fl_len * 0.4) * fx - w_c * 0.6 * rx).round() as i32, (ign_node.1 + (base_fl_len * 0.4) * fy - w_c * 0.6 * ry).round() as i32),
                ((c_tip.0 - w_c * rx + (roll_t.sin() * 4.0 * scale) * rx).round() as i32, (c_tip.1 - w_c * ry + (roll_t.sin() * 4.0 * scale) * ry).round() as i32),
                (c_tip.0.round() as i32, c_tip.1.round() as i32),
                ((c_tip.0 + w_c * rx - (roll_t.sin() * 4.0 * scale) * rx).round() as i32, (c_tip.1 + w_c * ry - (roll_t.sin() * 4.0 * scale) * ry).round() as i32),
                ((ign_node.0 + (base_fl_len * 0.4) * fx + w_c * 0.6 * rx).round() as i32, (ign_node.1 + (base_fl_len * 0.4) * fy + w_c * 0.6 * ry).round() as i32),
            ];
            let mut vh_outer = pal.fire_outer;
            vh_outer.3 = 240;
            canvas.fill_polygon(&c_poly, vh_outer);

            // Boiling Inner Inferno
            let inner_tip = (ign_node.0 + (base_fl_len * 0.70) * fx, ign_node.1 + (base_fl_len * 0.70) * fy);
            let in_poly = [
                (ign_node.0.round() as i32, ign_node.1.round() as i32),
                ((inner_tip.0 - w_c * 0.5 * rx).round() as i32, (inner_tip.1 - w_c * 0.5 * ry).round() as i32),
                (inner_tip.0.round() as i32, inner_tip.1.round() as i32),
                ((inner_tip.0 + w_c * 0.5 * rx).round() as i32, (inner_tip.1 + w_c * 0.5 * ry).round() as i32),
            ];
            canvas.fill_polygon(&in_poly, pal.fire_inner);
            canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, inner_tip.0.round() as i32, inner_tip.1.round() as i32, pal.fire_core, 3);
        } else if is_caraxes {
            // 10. Caraxes: Acoustic Screech Resonance & Sinuous Serpentine Ribbon Flame
            let mut ribbon_pts: Vec<(f32, f32)> = Vec::new();
            let steps = 6;
            for s_idx in 0..=steps {
                let s = s_idx as f32 / steps as f32;
                let wave = ((s * 8.0 - frame_idx as f32 * 0.7).sin()) * (5.5 * scale * s);
                let pt = (
                    ign_node.0 + s * base_fl_len * fx + wave * rx + drag_x * s,
                    ign_node.1 + s * base_fl_len * fy + wave * ry + drag_y * s,
                );
                ribbon_pts.push(pt);
            }
            for i in 0..ribbon_pts.len() - 1 {
                let p0 = ribbon_pts[i];
                let p1 = ribbon_pts[i + 1];
                let thick = ((5.0 - i as f32 * 0.6) * scale).round() as i32;
                canvas.draw_line(p0.0.round() as i32, p0.1.round() as i32, p1.0.round() as i32, p1.1.round() as i32, pal.fire_outer, thick.max(2));
                canvas.draw_line(p0.0.round() as i32, p0.1.round() as i32, p1.0.round() as i32, p1.1.round() as i32, pal.fire_inner, (thick - 1).max(1));
            }
            // Screech Acoustic Shockwave Rings
            let ring_cx = (ign_node.0 + 10.0 * scale * fx).round() as i32;
            let ring_cy = (ign_node.1 + 10.0 * scale * fy).round() as i32;
            let ring_r = (7.0 * scale).round() as i32;
            let screech_col = Rgba(255, 230, 180, 150);
            for a in 0..12 {
                let ang = a as f32 * PI / 6.0;
                let sx = ring_cx + (ang.cos() * ring_r as f32).round() as i32;
                let sy = ring_cy + (ang.sin() * ring_r as f32).round() as i32;
                canvas.set_pixel(sx, sy, screech_col);
            }
        } else if is_moondancer {
            // 16. Moondancer: Hyper-Focused Cutting Torch Jet with Diverging Micro-Sparks
            let torch_tip = (ign_node.0 + base_fl_len * fx + drag_x, ign_node.1 + base_fl_len * fy + drag_y);
            // Needle-thin oxyacetylene torch stream
            canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, torch_tip.0.round() as i32, torch_tip.1.round() as i32, Rgba::rgb(200, 240, 50), 2);
            canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, torch_tip.0.round() as i32, torch_tip.1.round() as i32, Rgba::rgb(255, 255, 255), 1);
            // Diverging high-velocity micro-sparks
            for s_idx in 0..6 {
                let sp_ang = (s_idx as f32 - 2.5) * 0.18;
                let sp_dir_x = (angle + sp_ang).cos();
                let sp_dir_y = (angle + sp_ang).sin();
                let dist = base_fl_len * (0.8 + (s_idx % 3) as f32 * 0.15);
                let sp_x = (ign_node.0 + dist * sp_dir_x).round() as i32;
                let sp_y = (ign_node.1 + dist * sp_dir_y).round() as i32;
                canvas.set_pixel(sp_x, sp_y, Rgba::rgb(240, 255, 120));
            }
        } else if is_sheepstealer {
            // 15. Sheepstealer: Asymmetrical Sputtering & Uneven Napalm Splatter
            let feral_lean = 0.22; // Veers 13 degrees to one side
            let veer_fx = (angle - feral_lean).cos();
            let veer_fy = (angle - feral_lean).sin();
            let fl_tip = (ign_node.0 + base_fl_len * veer_fx + drag_x, ign_node.1 + base_fl_len * veer_fy + drag_y);

            let fl_l = (fl_tip.0 - 13.0 * scale * rx, fl_tip.1 - 13.0 * scale * ry);
            let fl_r = (fl_tip.0 + 7.0 * scale * rx, fl_tip.1 + 7.0 * scale * ry); // Asymmetric margins!
            let fire_poly = [
                (ign_node.0.round() as i32, ign_node.1.round() as i32),
                (fl_l.0.round() as i32, fl_l.1.round() as i32),
                (fl_tip.0.round() as i32, fl_tip.1.round() as i32),
                (fl_r.0.round() as i32, fl_r.1.round() as i32),
            ];
            canvas.fill_polygon(&fire_poly, pal.fire_outer);
            canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, fl_tip.0.round() as i32, fl_tip.1.round() as i32, pal.fire_inner, 2);

            // Sputtering unatomized feral burning oil clumps
            for clump in 0..4 {
                let c_dist = base_fl_len * (0.4 + clump as f32 * 0.2);
                let cx_clump = (ign_node.0 + c_dist * veer_fx - (clump as f32 * 2.0 * scale) * rx).round() as i32;
                let cy_clump = (ign_node.1 + c_dist * veer_fy - (clump as f32 * 2.0 * scale) * ry).round() as i32;
                canvas.set_pixel(cx_clump, cy_clump, Rgba::rgb(255, 120, 20));
            }
        } else if is_vermithor {
            // 14. Vermithor: Boiling Bronze Napalm Slag & Rayleigh-Plateau Droplet Breakup
            let fl_tip = (ign_node.0 + base_fl_len * fx + drag_x, ign_node.1 + base_fl_len * fy + drag_y);
            let fl_l = (fl_tip.0 - 12.0 * scale * rx, fl_tip.1 - 12.0 * scale * ry);
            let fl_r = (fl_tip.0 + 12.0 * scale * rx, fl_tip.1 + 12.0 * scale * ry);
            let fire_poly = [
                (ign_node.0.round() as i32, ign_node.1.round() as i32),
                (fl_l.0.round() as i32, fl_l.1.round() as i32),
                (fl_tip.0.round() as i32, fl_tip.1.round() as i32),
                (fl_r.0.round() as i32, fl_r.1.round() as i32),
            ];
            canvas.fill_polygon(&fire_poly, Rgba::rgb(180, 83, 9)); // Bronze furnace
            canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, fl_tip.0.round() as i32, fl_tip.1.round() as i32, Rgba::rgb(245, 158, 11), 3);

            // Heavy Molten Slag Droplets Falling with Gravity
            for slag in 0..5 {
                let s_dist = base_fl_len * (0.3 + slag as f32 * 0.16);
                let slag_x = (ign_node.0 + s_dist * fx + (slag as f32 * 1.5 * scale)).round() as i32;
                let slag_y = (ign_node.1 + s_dist * fy + (slag as f32 * 2.0 * scale)).round() as i32;
                canvas.set_pixel(slag_x, slag_y, Rgba::rgb(255, 230, 100));
            }
        } else if is_sunfyre {
            // 13. Sunfyre: High-Temp Ionization Corona & Glittering Golden Spark Halo
            let fl_tip = (ign_node.0 + base_fl_len * fx + drag_x, ign_node.1 + base_fl_len * fy + drag_y);
            let fl_l = (fl_tip.0 - 10.0 * scale * rx, fl_tip.1 - 10.0 * scale * ry);
            let fl_r = (fl_tip.0 + 10.0 * scale * rx, fl_tip.1 + 10.0 * scale * ry);
            let fire_poly = [
                (ign_node.0.round() as i32, ign_node.1.round() as i32),
                (fl_l.0.round() as i32, fl_l.1.round() as i32),
                (fl_tip.0.round() as i32, fl_tip.1.round() as i32),
                (fl_r.0.round() as i32, fl_r.1.round() as i32),
            ];
            canvas.fill_polygon(&fire_poly, Rgba::rgb(250, 204, 21)); // Golden corona
            canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, fl_tip.0.round() as i32, fl_tip.1.round() as i32, Rgba::rgb(255, 255, 255), 2);

            // Shimmering Golden Micro-Spark Halo
            for s in 0..10 {
                let sp_ang = s as f32 * PI / 5.0 + (frame_idx as f32 * 0.3);
                let sp_r = (base_fl_len * 0.6 + (s % 3) as f32 * 4.0) * scale;
                let sp_x = (ign_node.0 + sp_ang.cos() * sp_r * 0.4 + (base_fl_len * 0.5) * fx).round() as i32;
                let sp_y = (ign_node.1 + sp_ang.sin() * sp_r * 0.4 + (base_fl_len * 0.5) * fy).round() as i32;
                canvas.set_pixel(sp_x, sp_y, Rgba::rgb(255, 245, 160));
            }
        } else if is_viserion {
            // 8. Viserion: Slender Laminar Needle Jet (< 12°) Blossoming at Terminal Tip
            let fl_tip = (ign_node.0 + base_fl_len * fx + drag_x, ign_node.1 + base_fl_len * fy + drag_y);
            // Ultra-narrow laminar pencil jet
            let fl_l = (fl_tip.0 - 4.5 * scale * rx, fl_tip.1 - 4.5 * scale * ry);
            let fl_r = (fl_tip.0 + 4.5 * scale * rx, fl_tip.1 + 4.5 * scale * ry);
            let fire_poly = [
                (ign_node.0.round() as i32, ign_node.1.round() as i32),
                (fl_l.0.round() as i32, fl_l.1.round() as i32),
                (fl_tip.0.round() as i32, fl_tip.1.round() as i32),
                (fl_r.0.round() as i32, fl_r.1.round() as i32),
            ];
            canvas.fill_polygon(&fire_poly, Rgba::rgb(245, 158, 11));
            canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, fl_tip.0.round() as i32, fl_tip.1.round() as i32, Rgba::rgb(255, 251, 235), 2);

            // Terminal blossom flower at impact tip
            let blossom_r = (5.5 * scale).round() as i32;
            canvas.fill_ellipse(fl_tip.0.round() as i32, fl_tip.1.round() as i32, blossom_r, blossom_r, Rgba::rgb(253, 224, 71));
        } else if is_tessarion {
            // 15. Tessarion: Iridescent Dual-Sheath Plume (Cobalt Core + Metallic Copper Outer Sheath)
            let fl_tip = (ign_node.0 + base_fl_len * fx + drag_x, ign_node.1 + base_fl_len * fy + drag_y);
            let fl_l = (fl_tip.0 - 13.0 * scale * rx, fl_tip.1 - 13.0 * scale * ry);
            let fl_r = (fl_tip.0 + 13.0 * scale * rx, fl_tip.1 + 13.0 * scale * ry);

            // Outer metallic copper flame sheath
            let copper_poly = [
                (ign_node.0.round() as i32, ign_node.1.round() as i32),
                (fl_l.0.round() as i32, fl_l.1.round() as i32),
                (fl_tip.0.round() as i32, fl_tip.1.round() as i32),
                (fl_r.0.round() as i32, fl_r.1.round() as i32),
            ];
            canvas.fill_polygon(&copper_poly, Rgba::rgb(217, 119, 6));

            // Inner concentrated cobalt-blue plasma core
            let in_tip = (ign_node.0 + (base_fl_len * 0.85) * fx + drag_x, ign_node.1 + (base_fl_len * 0.85) * fy + drag_y);
            let in_l = (in_tip.0 - 6.5 * scale * rx, in_tip.1 - 6.5 * scale * ry);
            let in_r = (in_tip.0 + 6.5 * scale * rx, in_tip.1 + 6.5 * scale * ry);
            let cobalt_poly = [
                (ign_node.0.round() as i32, ign_node.1.round() as i32),
                (in_l.0.round() as i32, in_l.1.round() as i32),
                (in_tip.0.round() as i32, in_tip.1.round() as i32),
                (in_r.0.round() as i32, in_r.1.round() as i32),
            ];
            canvas.fill_polygon(&cobalt_poly, Rgba::rgb(29, 78, 216));
            canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, in_tip.0.round() as i32, in_tip.1.round() as i32, Rgba::rgb(0, 210, 255), 2);
            canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, (ign_node.0 + (base_fl_len * 0.5) * fx).round() as i32, (ign_node.1 + (base_fl_len * 0.5) * fy).round() as i32, Rgba::rgb(255, 255, 255), 1);

            // Orbiting golden-copper ionization embers
            for emb in 0..4 {
                let ang = emb as f32 * PI / 2.0 + frame_idx as f32 * 0.35;
                let edist = base_fl_len * (0.4 + (emb as f32 * 0.15));
                let ex = (ign_node.0 + edist * fx + ang.cos() * 5.0 * scale).round() as i32;
                let ey = (ign_node.1 + edist * fy + ang.sin() * 5.0 * scale).round() as i32;
                canvas.set_pixel(ex, ey, Rgba::rgb(251, 191, 36));
            }
        } else {
            // 5. Drogon: Supersonic Mach Shock Diamonds & Concussive Blast Waves
            let fl_tip = (ign_node.0 + base_fl_len * fx + drag_x, ign_node.1 + base_fl_len * fy + drag_y);

            // Kelvin-Helmholtz Multi-Lobed Expanding Boundary
            let turb_1 = (frame_idx as f32 * 0.4).sin() * 2.0 * scale;
            let turb_2 = (frame_idx as f32 * 0.4 + PI).sin() * 2.0 * scale;

            let fl_l = (fl_tip.0 - (12.0 * scale + turb_1) * rx, fl_tip.1 - (12.0 * scale + turb_1) * ry);
            let fl_r = (fl_tip.0 + (12.0 * scale + turb_2) * rx, fl_tip.1 + (12.0 * scale + turb_2) * ry);
            let fire_poly = [
                (ign_node.0.round() as i32, ign_node.1.round() as i32),
                (fl_l.0.round() as i32, fl_l.1.round() as i32),
                (fl_tip.0.round() as i32, fl_tip.1.round() as i32),
                (fl_r.0.round() as i32, fl_r.1.round() as i32),
            ];
            let mut fo = pal.fire_outer;
            fo.3 = 235;
            canvas.fill_polygon(&fire_poly, fo);

            let in_tip = (ign_node.0 + (base_fl_len * 0.75) * fx, ign_node.1 + (base_fl_len * 0.75) * fy);
            let in_l = (in_tip.0 - 7.5 * scale * rx, in_tip.1 - 7.5 * scale * ry);
            let in_r = (in_tip.0 + 7.5 * scale * rx, in_tip.1 + 7.5 * scale * ry);
            let in_poly = [
                (ign_node.0.round() as i32, ign_node.1.round() as i32),
                (in_l.0.round() as i32, in_l.1.round() as i32),
                (in_tip.0.round() as i32, in_tip.1.round() as i32),
                (in_r.0.round() as i32, in_r.1.round() as i32),
            ];
            let mut fi = pal.fire_inner;
            fi.3 = 245;
            canvas.fill_polygon(&in_poly, fi);

            // Drogon Supersonic Mach Shock Diamonds
            if is_drogon {
                for node_idx in &[0.20_f32, 0.42, 0.64] {
                    let d_center = (ign_node.0 + base_fl_len * node_idx * fx, ign_node.1 + base_fl_len * node_idx * fy);
                    let d_poly = [
                        ((d_center.0 + 3.0 * scale * fx).round() as i32, (d_center.1 + 3.0 * scale * fy).round() as i32),
                        ((d_center.0 + 2.5 * scale * rx).round() as i32, (d_center.1 + 2.5 * scale * ry).round() as i32),
                        ((d_center.0 - 3.0 * scale * fx).round() as i32, (d_center.1 - 3.0 * scale * fy).round() as i32),
                        ((d_center.0 - 2.5 * scale * rx).round() as i32, (d_center.1 - 2.5 * scale * ry).round() as i32),
                    ];
                    canvas.fill_polygon(&d_poly, Rgba::rgb(255, 255, 255)); // White-hot Mach diamond
                }
                // Concussive Shockwave Ring
                let shock_r = (8.0 * scale + (frame_idx % 8) as f32 * 1.5 * scale).round() as i32;
                canvas.fill_ellipse(ign_node.0.round() as i32, ign_node.1.round() as i32, shock_r, (shock_r as f32 * 0.7).round() as i32, Rgba(255, 120, 50, 70));
            } else {
                canvas.draw_line(ign_node.0.round() as i32, ign_node.1.round() as i32, (ign_node.0 + (base_fl_len * 0.4) * fx).round() as i32, (ign_node.1 + (base_fl_len * 0.4) * fy).round() as i32, pal.fire_core, 2);
            }
        }

        // -------------------------------------------------------------
        // TERMINAL SURFACE IMPACT BURST (Multi-Shaded Color Impact)
        // -------------------------------------------------------------
        let imp_pt = (ign_node.0 + base_fl_len * fx + drag_x, ign_node.1 + base_fl_len * fy + drag_y);
        let imp_x = imp_pt.0.round() as i32;
        let imp_y = imp_pt.1.round() as i32;
        let imp_r = (8.5 * scale).round() as i32;

        let (imp_scorch, imp_outer, imp_mid, imp_inner, imp_core) = if is_wight {
            (Rgba(16, 28, 55, 220), Rgba(0, 160, 255, 240), Rgba(34, 211, 238, 255), Rgba(165, 243, 252, 255), Rgba(255, 255, 255, 255))
        } else if is_sunfyre {
            (Rgba(75, 45, 10, 220), Rgba(217, 119, 6, 240), Rgba(250, 204, 21, 255), Rgba(254, 240, 138, 255), Rgba(255, 255, 240, 255))
        } else if is_caraxes {
            (Rgba(70, 12, 10, 220), Rgba(185, 28, 28, 240), Rgba(239, 68, 68, 255), Rgba(251, 146, 60, 255), Rgba(255, 237, 213, 255))
        } else if is_rhaegal {
            (Rgba(25, 55, 18, 220), Rgba(22, 163, 74, 240), Rgba(74, 222, 128, 255), Rgba(190, 242, 100, 255), Rgba(254, 252, 232, 255))
        } else if is_tessarion {
            (Rgba(45, 25, 70, 220), Rgba(37, 99, 235, 240), Rgba(56, 189, 248, 255), Rgba(186, 230, 253, 255), Rgba(255, 255, 255, 255))
        } else if is_viserion {
            (Rgba(70, 48, 20, 220), Rgba(217, 119, 6, 240), Rgba(245, 158, 11, 255), Rgba(253, 224, 71, 255), Rgba(255, 251, 235, 255))
        } else {
            (Rgba(55, 14, 12, 220), Rgba(220, 38, 38, 240), Rgba(249, 115, 22, 255), Rgba(253, 224, 71, 255), Rgba(255, 255, 240, 255))
        };

        // Multi-shaded concentric impact layers
        canvas.fill_ellipse_stipple_50(imp_x, imp_y, imp_r + 4, imp_r + 4, imp_scorch);
        canvas.fill_ellipse(imp_x, imp_y, imp_r, imp_r, imp_outer);
        canvas.fill_ellipse(imp_x, imp_y, ((imp_r as f32 * 0.65).round() as i32).max(2), ((imp_r as f32 * 0.65).round() as i32).max(2), imp_mid);
        canvas.fill_ellipse(imp_x, imp_y, ((imp_r as f32 * 0.38).round() as i32).max(1), ((imp_r as f32 * 0.38).round() as i32).max(1), imp_inner);
        canvas.fill_ellipse(imp_x, imp_y, ((imp_r as f32 * 0.18).round() as i32).max(1), ((imp_r as f32 * 0.18).round() as i32).max(1), imp_core);

        // Explosive radial impact splash tongues
        for sp in 0..6 {
            let ang = sp as f32 * PI / 3.0 + frame_idx as f32 * 0.4;
            let sp_len = imp_r as f32 * 1.5 + ((sp * 5 + frame_idx) % 4) as f32 * 2.0 * scale;
            let tx = (imp_pt.0 + ang.cos() * sp_len).round() as i32;
            let ty = (imp_pt.1 + ang.sin() * sp_len).round() as i32;
            canvas.draw_line(imp_x, imp_y, tx, ty, imp_inner, 2);
            canvas.set_pixel(tx, ty, imp_outer);
        }
    }

    // -----------------------------------------------------------------
    // 10. DETACHED TRAVELING PLUME & POST-COMBUSTION THERMAL DECAY
    // -----------------------------------------------------------------
    // When fuel feed cuts off at the mouth, fluid momentum carries the flame
    // column forward as a detached projectile cloud that radially expands and
    // decays across the blackbody spectrum into billowing smoke and ash.
    if flame_progress > 1.0 && flame_progress <= 2.0 {
        let post_p = flame_progress - 1.0; // 0.0 -> 1.0

        let base_len = if is_vhagar {
            68.0
        } else if is_balerion {
            60.0
        } else if is_moondancer {
            36.0
        } else {
            46.0
        } * scale;

        // Plume coasts downrange with momentum opposite to flight wind shear
        let travel_dist = (base_len * 0.65) + (42.0 * post_p * scale);
        let drag_x = -vx * 0.18 * post_p * scale;
        let drag_y = -vy * 0.18 * post_p * scale;
        let plume_cx = snout_pt.0 + travel_dist * fx + drag_x;
        let plume_cy = snout_pt.1 + travel_dist * fy + drag_y;

        // Kelvin-Helmholtz turbulent radial expansion
        let rad_x = if is_vhagar {
            18.0 + 26.0 * post_p
        } else {
            12.0 + 18.0 * post_p
        } * scale;
        let rad_y = if is_vhagar {
            14.0 + 20.0 * post_p
        } else {
            9.0 + 13.0 * post_p
        } * scale;

        // Blackbody Thermodynamic Cooling:
        // [0.00 .. 0.35]: Incandescent Hot Core (Outer + Inner + Core)
        // [0.35 .. 0.70]: Cooled Soot & Glowing Embers (Dull Crimson / Dark Amber)
        // [0.70 .. 1.00]: Extinguished Turbulent Ash Smoke (Grey billows with floating embers)
        if post_p < 0.35 {
            let cool_p = post_p / 0.35;
            let mut out_col = pal.fire_outer;
            out_col.3 = ((1.0 - cool_p * 0.3) * 220.0) as u8;
            canvas.fill_ellipse(plume_cx.round() as i32, plume_cy.round() as i32, rad_x.round() as i32, rad_y.round() as i32, out_col);

            let mut in_col = pal.fire_inner;
            in_col.3 = ((1.0 - cool_p) * 230.0) as u8;
            canvas.fill_ellipse(plume_cx.round() as i32, plume_cy.round() as i32, (rad_x * 0.6).round() as i32, (rad_y * 0.6).round() as i32, in_col);

            if cool_p < 0.5 {
                canvas.fill_circle(plume_cx.round() as i32, plume_cy.round() as i32, (3.0 * scale).round() as i32, pal.fire_core);
            }
        } else if post_p < 0.70 {
            let cool_p = (post_p - 0.35) / 0.35;
            let soot_col = if is_wight {
                Rgba(60, 150, 220, ((1.0 - cool_p * 0.4) * 190.0) as u8)
            } else if is_sunfyre {
                Rgba(230, 160, 40, ((1.0 - cool_p * 0.4) * 190.0) as u8)
            } else {
                Rgba(160, 45, 20, ((1.0 - cool_p * 0.4) * 190.0) as u8)
            };
            canvas.fill_ellipse(plume_cx.round() as i32, plume_cy.round() as i32, rad_x.round() as i32, rad_y.round() as i32, soot_col);

            // Turbulent multi-lobe vortex billows along perimeter
            for &lobe_angle in &[0.0_f32, 1.2, 2.4, 3.6, 4.8] {
                let lx = plume_cx + rad_x * 0.8 * (lobe_angle + post_p * 2.0).cos();
                let ly = plume_cy + rad_y * 0.8 * (lobe_angle + post_p * 2.0).sin();
                let mut lobe_col = soot_col;
                lobe_col.3 = (lobe_col.3 as f32 * 0.7) as u8;
                canvas.fill_circle(lx.round() as i32, ly.round() as i32, (4.5 * scale).round() as i32, lobe_col);
            }
        } else {
            // Billowing grey ash / smoke dispersal
            let cool_p = (post_p - 0.70) / 0.30;
            let smoke_a = ((1.0 - cool_p) * 140.0) as u8;
            let smoke_col = if is_vhagar {
                Rgba(45, 42, 40, smoke_a) // Deep volcanic ash
            } else if is_wight {
                Rgba(160, 200, 230, smoke_a) // Freezing cryogenic frost fog
            } else {
                Rgba(75, 80, 90, smoke_a) // Dispersing soot smoke
            };
            canvas.fill_ellipse(plume_cx.round() as i32, plume_cy.round() as i32, rad_x.round() as i32, rad_y.round() as i32, smoke_col);

            // Dispersing cloud lobes
            for &lobe_angle in &[0.5_f32, 1.8, 3.1, 4.4, 5.7] {
                let lx = plume_cx + rad_x * 0.9 * (lobe_angle + post_p * 3.0).cos();
                let ly = plume_cy + rad_y * 0.9 * (lobe_angle + post_p * 3.0).sin();
                canvas.fill_circle(lx.round() as i32, ly.round() as i32, (6.0 * scale).round() as i32, smoke_col);
            }

            // Lingering floating sparks in the smoke
            if is_sunfyre {
                for &spark_idx in &[0.3_f32, 0.7, 1.1] {
                    let sx = (plume_cx + spark_idx * 10.0 * scale * (spark_idx * 5.0).cos()).round() as i32;
                    let sy = (plume_cy + spark_idx * 10.0 * scale * (spark_idx * 5.0).sin()).round() as i32;
                    canvas.set_pixel(sx, sy, pal.scale_specular);
                }
            } else if is_wight {
                for &spark_idx in &[0.2_f32, 0.6, 1.0] {
                    let sx = (plume_cx + spark_idx * 10.0 * scale * (spark_idx * 4.0).cos()).round() as i32;
                    let sy = (plume_cy + spark_idx * 10.0 * scale * (spark_idx * 4.0).sin()).round() as i32;
                    canvas.set_pixel(sx, sy, Rgba::rgb(0, 240, 255));
                }
            }
        }
    }
}
