use crate::canvas::Canvas;
use crate::palette::{DragonPalette, Rgba};
use std::f32::consts::PI;

pub fn render_wyvern_frame(
    pal: &DragonPalette,
    phase: f32, // 0.0 to 1.0 (wing beat cycle)
    scale: f32,
    mouth_open: f32,
    canvas: &mut Canvas,
    offset_x: i32,
    offset_y: i32,
) {
    let t = phase * 2.0 * PI;
    let wing_lift = t.sin(); // -1.0 (upstroke) to +1.0 (downstroke)
    let bob_y = (wing_lift * 2.5 * scale).round() as i32;

    let cx = offset_x;
    let cy = offset_y + bob_y;

    // -----------------------------------------------------------------
    // 0. GROUND CONTACT DROP SHADOW (2.5D Depth Perspective)
    // -----------------------------------------------------------------
    let sh_y = cy + (46.0 * scale) as i32 - bob_y / 2;
    let sh_w = ((52.0 - wing_lift * 6.0) * scale).round() as i32;
    let sh_h = ((14.0 + wing_lift * 2.0) * scale).round() as i32;
    canvas.fill_ellipse(cx, sh_y, sh_w, sh_h, Rgba(10, 12, 16, 75));
    canvas.fill_ellipse(cx, sh_y, (sh_w as f32 * 0.6) as i32, (sh_h as f32 * 0.6) as i32, Rgba(10, 12, 16, 45));

    // -----------------------------------------------------------------
    // 1. FAR BAT WING (Layer 0, behind body)
    // -----------------------------------------------------------------
    let far_sh_x = cx - (8.0 * scale) as i32;
    let far_sh_y = cy - (9.0 * scale) as i32;
    let far_el_x = far_sh_x + (12.0 * scale) as i32;
    let far_el_y = far_sh_y - ((19.0 + wing_lift * 14.0) * scale) as i32;
    let far_wr_x = far_el_x + (22.0 * scale) as i32;
    let far_wr_y = far_el_y - ((12.0 + wing_lift * 10.0) * scale) as i32;

    let far_f1 = (far_wr_x + (22.0 * scale) as i32, far_wr_y - ((14.0 + wing_lift * 10.0) * scale) as i32);
    let far_f2 = (far_wr_x + (38.0 * scale) as i32, far_wr_y - ((3.0 + wing_lift * 6.0) * scale) as i32);
    let far_f3 = (far_wr_x + (30.0 * scale) as i32, far_wr_y + (15.0 * scale) as i32);

    let far_pts = [
        (far_sh_x, far_sh_y),
        (far_el_x, far_el_y),
        (far_wr_x, far_wr_y),
        far_f1,
        far_f2,
        far_f3,
        (cx - (14.0 * scale) as i32, cy),
    ];
    let mut far_col = pal.wing_far;
    far_col.3 = 230;
    canvas.fill_polygon(&far_pts, far_col);
    canvas.draw_line(far_sh_x, far_sh_y, far_el_x, far_el_y, pal.scale_deep, 2);
    canvas.draw_line(far_el_x, far_el_y, far_wr_x, far_wr_y, pal.scale_deep, 2);
    canvas.draw_line(far_wr_x, far_wr_y, far_f1.0, far_f1.1, pal.scale_deep, 1);
    canvas.draw_line(far_wr_x, far_wr_y, far_f2.0, far_f2.1, pal.scale_deep, 1);
    canvas.draw_line(far_wr_x, far_wr_y, far_f3.0, far_f3.1, pal.scale_deep, 1);

    // -----------------------------------------------------------------
    // 2. FAR HIND LEG (Layer 1, behind body in shadow)
    // -----------------------------------------------------------------
    let hip_x = cx - (22.0 * scale) as i32;
    let hip_y = cy + (7.0 * scale) as i32;
    let far_knee_x = hip_x + (5.0 * scale) as i32;
    let far_knee_y = hip_y + ((14.0 + wing_lift * 2.0) * scale) as i32;
    let far_ank_x = far_knee_x - (8.0 * scale) as i32;
    let far_ank_y = far_knee_y + (10.0 * scale) as i32;

    // Muscular thigh and shin
    canvas.draw_line(hip_x, hip_y, far_knee_x, far_knee_y, pal.scale_deep, 3);
    canvas.draw_line(far_knee_x, far_knee_y, far_ank_x, far_ank_y, pal.scale_deep, 2);
    // 3 splayed claws
    canvas.draw_line(far_ank_x, far_ank_y, far_ank_x + (6.0 * scale) as i32, far_ank_y + (2.0 * scale) as i32, pal.scale_deep, 1);
    canvas.draw_line(far_ank_x, far_ank_y, far_ank_x + (4.0 * scale) as i32, far_ank_y + (5.0 * scale) as i32, pal.scale_deep, 1);
    canvas.draw_line(far_ank_x, far_ank_y, far_ank_x - (3.0 * scale) as i32, far_ank_y + (3.0 * scale) as i32, pal.scale_deep, 1);

    // -----------------------------------------------------------------
    // 3. TAIL SPLINE (Layer 2, harmonic serpentine curve)
    // -----------------------------------------------------------------
    let mut tail_pts = Vec::new();
    let tail_segs = 12;
    for i in 0..=tail_segs {
        let frac = i as f32 / tail_segs as f32;
        let tx = hip_x - (frac * 72.0 * scale) as i32;
        let wave = (t - frac * 3.6).sin() * (frac.powf(1.6)) * 18.0 * scale;
        let ty = hip_y - (frac * 8.0 * scale) as i32 + wave.round() as i32;
        tail_pts.push((tx, ty));
    }

    for i in 0..tail_pts.len() - 1 {
        let frac = i as f32 / tail_pts.len() as f32;
        let thick = ((1.0 - frac) * 8.5 * scale).round() as i32;
        let p_a = tail_pts[i];
        let p_b = tail_pts[i + 1];
        canvas.draw_line(p_a.0, p_a.1, p_b.0, p_b.1, pal.scale_dark, thick.max(1));
        canvas.draw_line(p_a.0, p_a.1 - thick / 3, p_b.0, p_b.1 - thick / 3, pal.scale_mid, 1);

        // Alternating dorsal spikes
        if i % 2 == 0 && frac < 0.85 {
            let sp_x = p_a.0;
            let sp_y = p_a.1 - thick / 2;
            canvas.draw_line(sp_x, sp_y, sp_x - 2, sp_y - 4, pal.wing_arm, 1);
        }
    }

    // Tail spade / rudder
    if let Some(&tip) = tail_pts.last() {
        let spade = [
            tip,
            (tip.0 - (9.0 * scale) as i32, tip.1 - (6.0 * scale) as i32),
            (tip.0 - (13.0 * scale) as i32, tip.1),
            (tip.0 - (9.0 * scale) as i32, tip.1 + (6.0 * scale) as i32),
        ];
        canvas.fill_polygon(&spade, pal.wing_arm);
        canvas.draw_line(spade[0].0, spade[0].1, spade[1].0, spade[1].1, pal.scale_hi, 1);
    }

    // -----------------------------------------------------------------
    // 4. TORSO & VENTRAL BELLY SCUTES (Layer 2)
    // -----------------------------------------------------------------
    let chest_pts = [
        (cx - (26.0 * scale) as i32, cy + (5.0 * scale) as i32),
        (cx - (14.0 * scale) as i32, cy + (12.0 * scale) as i32),
        (cx + (7.0 * scale) as i32, cy + (13.0 * scale) as i32),
        (cx + (22.0 * scale) as i32, cy + (6.0 * scale) as i32),
        (cx + (28.0 * scale) as i32, cy - (3.0 * scale) as i32),
        (cx + (20.0 * scale) as i32, cy - (10.0 * scale) as i32),
        (cx + (4.0 * scale) as i32, cy - (11.0 * scale) as i32),
        (cx - (16.0 * scale) as i32, cy - (7.0 * scale) as i32),
    ];
    canvas.fill_polygon(&chest_pts, pal.scale_dark);

    // Ventral underbelly plates
    let ventral_pts = [
        (cx - (24.0 * scale) as i32, cy + (5.0 * scale) as i32),
        (cx - (13.0 * scale) as i32, cy + (11.0 * scale) as i32),
        (cx + (6.0 * scale) as i32, cy + (12.0 * scale) as i32),
        (cx + (20.0 * scale) as i32, cy + (5.0 * scale) as i32),
        (cx + (26.0 * scale) as i32, cy - (3.0 * scale) as i32),
        (cx + (21.0 * scale) as i32, cy),
        (cx + (5.0 * scale) as i32, cy + (6.0 * scale) as i32),
        (cx - (10.0 * scale) as i32, cy + (6.0 * scale) as i32),
        (cx - (21.0 * scale) as i32, cy + (3.0 * scale) as i32),
    ];
    canvas.fill_polygon(&ventral_pts, pal.ventral_mid);

    // Belly plate transverse grooves
    for vx_off in [-15.0, -5.0, 5.0, 15.0] {
        let vpx = cx + (vx_off * scale) as i32;
        canvas.draw_line(vpx, cy + (6.0 * scale) as i32, vpx - 2, cy + (11.0 * scale) as i32, pal.ventral_deep, 1);
    }

    // -----------------------------------------------------------------
    // 5. NEAR HIND LEG (Layer 3, foreground raptor leg with claws)
    // -----------------------------------------------------------------
    let near_hip_x = hip_x + (4.0 * scale) as i32;
    let near_hip_y = hip_y;
    let near_knee_x = near_hip_x + (8.0 * scale) as i32;
    let near_knee_y = near_hip_y + ((15.0 - wing_lift * 2.0) * scale) as i32;
    let near_ank_x = near_knee_x - (9.0 * scale) as i32;
    let near_ank_y = near_knee_y + (10.0 * scale) as i32;

    // Muscular thigh with ridge highlight
    canvas.draw_line(near_hip_x, near_hip_y, near_knee_x, near_knee_y, pal.scale_mid, 4);
    canvas.draw_line(near_hip_x, near_hip_y - 1, near_knee_x, near_knee_y - 1, pal.scale_hi, 1);
    // Shin
    canvas.draw_line(near_knee_x, near_knee_y, near_ank_x, near_ank_y, pal.scale_dark, 3);
    // Raptor Claws: 3 curved forward, 1 rear hallux talon
    let claw_bone = Rgba::rgb(245, 245, 235);
    let claw_shade = Rgba::rgb(205, 200, 185);
    canvas.draw_line(near_ank_x, near_ank_y, near_ank_x + (8.0 * scale) as i32, near_ank_y + (2.0 * scale) as i32, claw_bone, 1);
    canvas.draw_line(near_ank_x - 1, near_ank_y + 1, near_ank_x + (7.0 * scale) as i32, near_ank_y + (5.0 * scale) as i32, claw_bone, 1);
    canvas.draw_line(near_ank_x - 1, near_ank_y + 2, near_ank_x + (4.0 * scale) as i32, near_ank_y + (7.0 * scale) as i32, claw_shade, 1);
    canvas.draw_line(near_ank_x - 1, near_ank_y, near_ank_x - (4.0 * scale) as i32, near_ank_y + (4.0 * scale) as i32, claw_shade, 1);

    // Caraxes Ankle Rudders
    if pal.has_leg_rudders {
        let rudder = [
            (near_ank_x - 1, near_ank_y - 2),
            (near_ank_x - (8.0 * scale) as i32, near_ank_y - (4.0 * scale) as i32),
            (near_ank_x - (10.0 * scale) as i32, near_ank_y + (2.0 * scale) as i32),
            (near_ank_x, near_ank_y + 2),
        ];
        let mut rud_col = pal.wing_web1;
        rud_col.3 = 240;
        canvas.fill_polygon(&rudder, rud_col);
    }

    // -----------------------------------------------------------------
    // 6. NECK & PREDATORY SKULL (Layer 4)
    // -----------------------------------------------------------------
    let neck_len = if pal.is_serpentine { 1.35 } else { 1.0 };
    let n0 = (cx + (24.0 * scale) as i32, cy - (7.0 * scale) as i32);
    let n1 = (cx + (36.0 * scale * neck_len) as i32, cy - (15.0 * scale) as i32);
    let n2 = (cx + (48.0 * scale * neck_len) as i32, cy - (22.0 * scale) as i32);
    let n3 = (cx + (60.0 * scale * neck_len) as i32, cy - (25.0 * scale) as i32);

    let mut neck_pts = Vec::new();
    for step in 0..=8 {
        let u = step as f32 / 8.0;
        let nx = (1.0 - u).powi(3) * n0.0 as f32 + 3.0 * (1.0 - u).powi(2) * u * n1.0 as f32 + 3.0 * (1.0 - u) * u.powi(2) * n2.0 as f32 + u.powi(3) * n3.0 as f32;
        let ny = (1.0 - u).powi(3) * n0.1 as f32 + 3.0 * (1.0 - u).powi(2) * u * n1.1 as f32 + 3.0 * (1.0 - u) * u.powi(2) * n2.1 as f32 + u.powi(3) * n3.1 as f32;
        neck_pts.push((nx.round() as i32, ny.round() as i32));
    }

    for i in 0..neck_pts.len() - 1 {
        let u = i as f32 / neck_pts.len() as f32;
        let w_neck = ((1.0 - u * 0.3) * 10.5 * scale).round() as i32;
        canvas.draw_line(neck_pts[i].0, neck_pts[i].1, neck_pts[i + 1].0, neck_pts[i + 1].1, pal.scale_dark, w_neck.max(1));
        canvas.draw_line(neck_pts[i].0, neck_pts[i].1 - w_neck / 3, neck_pts[i + 1].0, neck_pts[i + 1].1 - w_neck / 3, pal.scale_hi, 1);
        if i % 2 == 0 {
            let spx = neck_pts[i].0;
            let spy = neck_pts[i].1 - w_neck / 2;
            canvas.draw_line(spx, spy, spx - 2, spy - 5, pal.wing_arm, 1);
        }
    }

    let hx = n3.0;
    let hy = n3.1;
    let l_snout = (22.0 * scale) as i32;
    let jaw_drop = (mouth_open * 1.3).round() as i32;

    // Predatory Cranium Polygon
    let cranium = [
        (hx - (5.0 * scale) as i32, hy + (5.0 * scale) as i32),
        (hx - (4.0 * scale) as i32, hy + (2.0 * scale) as i32),
        (hx - (1.0 * scale) as i32, hy - (1.0 * scale) as i32),
        (hx + (4.0 * scale) as i32, hy - (1.0 * scale) as i32),
        (hx + (8.0 * scale) as i32, hy),
        (hx + (13.0 * scale) as i32, hy + (2.0 * scale) as i32),
        (hx + (17.0 * scale) as i32, hy + (3.0 * scale) as i32),
        (hx + l_snout, hy + (5.0 * scale) as i32),
        (hx + l_snout - (1.0 * scale) as i32, hy + (7.0 * scale) as i32),
        (hx + (16.0 * scale) as i32, hy + (6.0 * scale) as i32),
        (hx + (11.0 * scale) as i32, hy + (5.0 * scale) as i32),
        (hx + (5.0 * scale) as i32, hy + (5.0 * scale) as i32),
        (hx + (1.0 * scale) as i32, hy + (5.0 * scale) as i32),
        (hx - (4.0 * scale) as i32, hy + (6.0 * scale) as i32),
    ];
    canvas.fill_polygon(&cranium, pal.scale_dark);
    // Nasal crest highlight
    canvas.draw_line(hx + (8.0 * scale) as i32, hy, hx + l_snout, hy + (5.0 * scale) as i32, pal.scale_hi, 1);
    // Nostril slit flare
    canvas.set_pixel(hx + l_snout - 2, hy + (4.0 * scale) as i32, pal.scale_deep);

    // Deep orbital eye cavity & glowing eye
    let eye_x = hx + (8.0 * scale) as i32;
    let eye_y = hy + (1.0 * scale) as i32;
    for (ox, oy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        canvas.set_pixel(eye_x + ox, eye_y + oy, pal.scale_deep);
    }
    canvas.set_pixel(eye_x, eye_y, pal.eye_glow);
    canvas.set_pixel(eye_x, eye_y - 1, Rgba::rgb(255, 255, 255)); // Specular glint

    // Upper canine fangs
    let fang_x = hx + l_snout - (2.0 * scale) as i32;
    let fang_y = hy + (6.0 * scale) as i32;
    canvas.set_pixel(fang_x, fang_y, Rgba::rgb(250, 250, 240));
    canvas.set_pixel(fang_x, fang_y + 1, Rgba::rgb(250, 250, 240));
    canvas.set_pixel(fang_x - 1, fang_y + 2, Rgba::rgb(205, 200, 180));
    canvas.set_pixel(fang_x - 3, fang_y + 1, Rgba::rgb(250, 250, 240));

    // Lower Mandible (Rotational Quadrate Jaw Hinge)
    let hinge_x = hx + (1.0 * scale) as i32;
    let hinge_y = hy + (5.0 * scale) as i32;
    let chin_x = hx + l_snout - (mouth_open * 0.3).round() as i32;
    let chin_y = hy + (5.0 * scale) as i32 + jaw_drop;

    if mouth_open > 0.0 {
        let oral = [
            (hinge_x + 2, hinge_y),
            (hx + l_snout - 3, hy + (5.0 * scale) as i32),
            (chin_x - 2, chin_y - 1),
            (hinge_x + 2, hinge_y + (mouth_open * 0.4).round() as i32),
        ];
        canvas.fill_polygon(&oral, Rgba::rgb(45, 12, 18));
        canvas.set_pixel(hinge_x + 2, hinge_y + 1, pal.fire_inner);
        canvas.set_pixel(hinge_x + 3, hinge_y + 1, pal.fire_core);
    }

    let mandible = [
        (hinge_x, hinge_y + 1),
        (chin_x, chin_y - 1),
        (chin_x + 1, chin_y),
        (chin_x, chin_y + 2),
        (chin_x - 3, chin_y + 2),
        (hinge_x + 3, hinge_y + 3),
        (hinge_x, hinge_y + 2),
    ];
    canvas.fill_polygon(&mandible, pal.scale_dark);
    canvas.draw_line(chin_x, chin_y + 2, hinge_x + 3, hinge_y + 3, pal.scale_deep, 1);
    // Lower interlocking teeth
    canvas.set_pixel(chin_x - 1, chin_y - 1, Rgba::rgb(250, 250, 240));
    canvas.set_pixel(chin_x - 3, chin_y - 2, Rgba::rgb(205, 200, 180));

    // Vhagar Throat Dewlap
    if pal.has_dewlap {
        let dewlap = [
            (hinge_x, hinge_y + 2),
            (hinge_x + 5, hinge_y + 3),
            (hinge_x + 3, hinge_y + 7),
            (hinge_x - 2, hinge_y + 6),
        ];
        canvas.fill_polygon(&dewlap, pal.scale_dark);
    }

    // Cranial Horns
    if pal.has_ear_frills {
        canvas.draw_line(hx, hy + 1, hx - 10, hy - 6, pal.horn_mid, 2);
        canvas.draw_line(hx, hy + 1, hx - 10, hy - 5, pal.horn_hi, 1);
        let frill = [(hx - 1, hy + 2), (hx - 6, hy - 2), (hx - 8, hy + 3), (hx - 2, hy + 4)];
        let mut fr_col = pal.wing_web1;
        fr_col.3 = 240;
        canvas.fill_polygon(&frill, fr_col);
    } else {
        canvas.draw_line(hx, hy + 1, hx - 16, hy - 10, pal.horn_mid, 2);
        canvas.draw_line(hx, hy, hx - 16, hy - 11, pal.horn_hi, 1);
        canvas.draw_line(hx - 1, hy + 4, hx - 6, hy + 6, pal.horn_mid, 1);
    }

    // -----------------------------------------------------------------
    // 7. NEAR BAT WING (Layer 5, foreground articulated wing)
    // -----------------------------------------------------------------
    let near_sh_x = cx + (4.0 * scale) as i32;
    let near_sh_y = cy - (10.0 * scale) as i32;
    let near_el_x = near_sh_x + (16.0 * scale) as i32;
    let near_el_y = near_sh_y - ((22.0 + wing_lift * 16.0) * scale) as i32;
    let near_wr_x = near_el_x + (26.0 * scale) as i32;
    let near_wr_y = near_el_y - ((14.0 + wing_lift * 12.0) * scale) as i32;

    let f1 = (near_wr_x + (26.0 * scale) as i32, near_wr_y - ((16.0 + wing_lift * 12.0) * scale) as i32);
    let f2 = (near_wr_x + (44.0 * scale) as i32, near_wr_y - ((4.0 + wing_lift * 8.0) * scale) as i32);
    let f3 = (near_wr_x + (40.0 * scale) as i32, near_wr_y + ((15.0 - wing_lift * 4.0) * scale) as i32);
    let f4 = (near_wr_x + (16.0 * scale) as i32, near_wr_y + (32.0 * scale) as i32);
    let flank = (cx - (9.0 * scale) as i32, cy - (4.0 * scale) as i32);

    let mut w1 = pal.wing_web1;
    w1.3 = 245;
    let mut w2 = pal.wing_web2;
    w2.3 = 245;

    let web1 = [(near_wr_x, near_wr_y), (near_el_x, near_el_y), f1];
    canvas.fill_polygon(&web1, w1);

    let web2 = [(near_wr_x, near_wr_y), f1, f2];
    canvas.fill_polygon(&web2, w1);

    let web3 = [(near_wr_x, near_wr_y), f2, f3];
    canvas.fill_polygon(&web3, w2);

    let web4 = [(near_wr_x, near_wr_y), f3, f4];
    canvas.fill_polygon(&web4, w2);

    let web5 = [(near_wr_x, near_wr_y), f4, flank, (near_sh_x, near_sh_y)];
    canvas.fill_polygon(&web5, w2);

    // Delicate translucent wing veins
    let mut vein_col = pal.wing_web1;
    vein_col.3 = 180;
    canvas.draw_line(near_wr_x, near_wr_y, (f1.0 + f2.0) / 2, (f1.1 + f2.1) / 2, vein_col, 1);
    canvas.draw_line(near_wr_x, near_wr_y, (f2.0 + f3.0) / 2, (f2.1 + f3.1) / 2, vein_col, 1);

    // Arm bones & finger spars
    canvas.draw_line(near_sh_x, near_sh_y, near_el_x, near_el_y, pal.wing_arm, 3);
    canvas.draw_line(near_el_x, near_el_y, near_wr_x, near_wr_y, pal.wing_arm, 2);
    for finger in [f1, f2, f3, f4] {
        canvas.draw_line(near_wr_x, near_wr_y, finger.0, finger.1, pal.wing_bone, 1);
    }
    // Clawed thumb at wrist joint
    canvas.set_pixel(near_wr_x + 1, near_wr_y - 2, Rgba::rgb(245, 245, 235));

    // -----------------------------------------------------------------
    // 8. ROARING DRAGONFIRE (Layer 6)
    // -----------------------------------------------------------------
    if mouth_open > 0.0 {
        let mx = chin_x + 2;
        let my = (hy + chin_y) / 2;
        let fl_len = (36.0 * scale) as i32;

        if pal.is_plasma {
            // Wight Viserion Cobalt Plasma Beam
            canvas.draw_line(mx, my, mx + fl_len, my, pal.fire_outer, 5);
            canvas.draw_line(mx, my, mx + fl_len, my, pal.fire_inner, 3);
            canvas.draw_line(mx + 1, my, mx + fl_len - 2, my, pal.fire_core, 1);
        } else {
            let fire_pts = [
                (mx, my),
                (mx + 8, my - 7),
                (mx + 20, my - 11),
                (mx + fl_len + 4, my - 4),
                (mx + fl_len + 8, my + 2),
                (mx + fl_len + 3, my + 8),
                (mx + 20, my + 11),
                (mx + 8, my + 6),
            ];
            let mut fo = pal.fire_outer;
            fo.3 = 230;
            canvas.fill_polygon(&fire_pts, fo);

            let fire_inner_pts = [
                (mx, my),
                (mx + 6, my - 4),
                (mx + 16, my - 6),
                (mx + fl_len - 2, my - 1),
                (mx + fl_len + 2, my + 2),
                (mx + 14, my + 6),
                (mx + 6, my + 3),
            ];
            let mut fi = pal.fire_inner;
            fi.3 = 245;
            canvas.fill_polygon(&fire_inner_pts, fi);

            let core_pts = [
                (mx - 1, my),
                (mx + 5, my - 2),
                (mx + 16, my),
                (mx + 5, my + 2),
            ];
            canvas.fill_polygon(&core_pts, pal.fire_core);
        }
    }
}
