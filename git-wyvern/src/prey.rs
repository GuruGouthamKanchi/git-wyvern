use crate::canvas::Canvas;
use crate::palette::Rgba;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PreyKind {
    Sheep,
    Pig,
    WildBoar,
    Bull,
    Goat,
    Stag,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PreyState {
    Grazing,
    Alert,
    Fleeing,
    Struck,
    CapturedTalons,
    InMaw,
    Swallowed,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HuntStyle {
    FlameRoastAndToss, // Drogon, Tessarion, Sunfyre: Roast with breath, snatch in air, toss & catch
    CavernousGulp,     // Vhagar, Balerion, Vermithor: Cavernous maw scoops whole prey off ground
    ViperNeckStrike,   // Caraxes, Rhaegal: Serpentine neck coils and strikes like a viper
    RaptorTalonPin,    // Moondancer, Meleys, Sheepstealer: High-speed talon strike, pins & tears
    FrostShatter,      // Wight Viserion: Freezes prey in ice flame, crunches into crystals
    RoyalSwoopGulp,    // Syrax, Silverwing, Viserion, Seasmoke: Graceful glide-by scoop
}

/// Retrieve the lore-accurate prey, hunting style, and feeding duration for each dragon
pub fn get_dragon_hunt_config(key: &str) -> (PreyKind, HuntStyle, f32) {
    match key {
        "vhagar" | "balerion" => (PreyKind::Bull, HuntStyle::CavernousGulp, 0.10), // Colossal instant gulp (0.10 timeline)
        "vermithor" => (PreyKind::Bull, HuntStyle::CavernousGulp, 0.12),
        "drogon" => (PreyKind::Sheep, HuntStyle::FlameRoastAndToss, 0.18),
        "sunfyre" => (PreyKind::Sheep, HuntStyle::FlameRoastAndToss, 0.18),
        "tessarion" => (PreyKind::Pig, HuntStyle::FlameRoastAndToss, 0.18),
        "caraxes" => (PreyKind::WildBoar, HuntStyle::ViperNeckStrike, 0.16),
        "rhaegal" => (PreyKind::Goat, HuntStyle::ViperNeckStrike, 0.16),
        "moondancer" => (PreyKind::Goat, HuntStyle::RaptorTalonPin, 0.22), // Multi-bite tearing feast
        "sheepstealer" => (PreyKind::Sheep, HuntStyle::RaptorTalonPin, 0.22),
        "meleys" => (PreyKind::Stag, HuntStyle::RaptorTalonPin, 0.18),
        "wight_viserion" => (PreyKind::Stag, HuntStyle::FrostShatter, 0.14),
        "syrax" => (PreyKind::Pig, HuntStyle::RoyalSwoopGulp, 0.16),
        "silverwing" => (PreyKind::Sheep, HuntStyle::RoyalSwoopGulp, 0.16),
        "viserion" => (PreyKind::Sheep, HuntStyle::RoyalSwoopGulp, 0.16),
        "seasmoke" => (PreyKind::Goat, HuntStyle::RoyalSwoopGulp, 0.16),
        _ => (PreyKind::Sheep, HuntStyle::FlameRoastAndToss, 0.18),
    }
}

/// Simple deterministic hash for procedural spawn placements
pub fn get_dragon_seed(key: &str) -> u32 {
    let mut h: u32 = 2166136261;
    for b in key.as_bytes() {
        h ^= *b as u32;
        h = h.wrapping_mul(16777619);
    }
    h
}

/// Continuous pyrolytic thermal gradient for roasted/charred meat
fn get_pyrolytic_colors(base: Rgba, roast: f32, is_wight: bool) -> (Rgba, Rgba, bool) {
    if is_wight && roast > 0.15 {
        (Rgba(130, 225, 255, 255), Rgba(0, 160, 240, 255), false) // Cryogenic blue freeze
    } else if roast <= 0.05 {
        (base, Rgba((base.0 as u16 * 82 / 100) as u8, (base.1 as u16 * 82 / 100) as u8, (base.2 as u16 * 82 / 100) as u8, 255), false)
    } else if roast < 0.35 {
        // Stage 1: Flash-ignition surface sear
        let t = roast / 0.35;
        let r = (base.0 as f32 * (1.0 - t) + 255.0 * t) as u8;
        let g = (base.1 as f32 * (1.0 - t) + 160.0 * t) as u8;
        let b = (base.2 as f32 * (1.0 - t) + 40.0 * t) as u8;
        (Rgba(r, g, b, 255), Rgba(255, 100, 20, 255), true)
    } else if roast < 0.70 {
        // Stage 2: Caramelized crispy golden roast with glowing core
        let t = (roast - 0.35) / 0.35;
        let r = (255.0 * (1.0 - t) + 155.0 * t) as u8;
        let g = (160.0 * (1.0 - t) + 80.0 * t) as u8;
        let b = (40.0 * (1.0 - t) + 25.0 * t) as u8;
        (Rgba(r, g, b, 255), Rgba(255, 140, 30, 255), true)
    } else {
        // Stage 3 & 4: Deep blackbody seared charcoal crust
        let t = ((roast - 0.70) / 0.30).min(1.0);
        let r = (155.0 * (1.0 - t) + 50.0 * t) as u8;
        let g = (80.0 * (1.0 - t) + 32.0 * t) as u8;
        let b = (25.0 * (1.0 - t) + 18.0 * t) as u8;
        (Rgba(r, g, b, 255), Rgba(35, 20, 15, 255), false)
    }
}

/// Render retro pixel-art prey on the ground or mid-air with ragdoll kinematics
pub fn render_prey(
    canvas: &mut Canvas,
    kind: PreyKind,
    state: PreyState,
    px: f32,
    py: f32,
    heading: f32,
    anim_time: f32,
    roast_pct: f32,
    is_wight: bool,
    fire_light: Option<(f32, f32, f32)>,
) {
    if state == PreyState::Swallowed {
        return;
    }

    let ix = px.round() as i32;
    let iy = py.round() as i32;

    // Ragdoll writhing & struggling rotation when clutched in talons or jaws
    let eff_heading = match state {
        PreyState::CapturedTalons => {
            heading + (anim_time * 26.0).sin() * 0.38 // Panicked flailing twist
        }
        PreyState::InMaw => {
            heading + (anim_time * 18.0).sin() * 0.22 // Head thrashing
        }
        _ => heading,
    };

    let fx = eff_heading.cos();
    let fy = eff_heading.sin();
    let rx = -fy;
    let ry = fx;

    // Dual-Source Shadow Casting on Terrain (Only when grounded):
    // Primary celestial sun azimuth shadow + elongated secondary radial cast shadow away from dragonfire
    if state != PreyState::CapturedTalons && state != PreyState::InMaw {
        // 1. Primary celestial sun shadow S = [0.55, 0.65] * d
        let sun_sh_col = Rgba(30, 26, 32, 255);
        canvas.fill_ellipse(ix + 2, iy + 3, 3, 2, sun_sh_col);

        // 2. Secondary Radial Prey Shadow (During active combustion)
        if let Some((fl_x, fl_y, fl_intensity)) = fire_light {
            if fl_intensity > 0.05 {
                let dx = px - fl_x;
                let dy = py - fl_y;
                let dist = (dx * dx + dy * dy).sqrt().max(1.0);
                let dir_x = dx / dist;
                let dir_y = dy / dist;
                let sec_len = (8.5 * fl_intensity * (1.0 - (dist / 260.0).clamp(0.0, 0.85))).max(2.0);
                let sec_tip_x = (px + dir_x * sec_len).round() as i32;
                let sec_tip_y = (py + dir_y * sec_len).round() as i32;
                let sec_alpha = (190.0 * fl_intensity) as u8;
                let sec_col = Rgba(16, 14, 20, sec_alpha);
                canvas.draw_line(ix, iy + 1, sec_tip_x, sec_tip_y, sec_col, 3);
                canvas.fill_ellipse(sec_tip_x, sec_tip_y, 2, 2, sec_col);
            }
        }
    }

    // Panic sweat drop or alert exclamation
    if state == PreyState::Alert || state == PreyState::Fleeing {
        let alert_y = iy - 8;
        if state == PreyState::Alert {
            canvas.draw_line(ix, alert_y - 3, ix, alert_y, Rgba(255, 230, 40, 240), 1);
            canvas.set_pixel(ix, alert_y + 2, Rgba(255, 230, 40, 240));
        } else {
            canvas.set_pixel(ix + 4, alert_y, Rgba(80, 200, 255, 220));
            canvas.set_pixel(ix + 5, alert_y + 1, Rgba(50, 160, 240, 200));
        }
    }

    // Dust puffs while fleeing
    if state == PreyState::Fleeing {
        let dust_col = Rgba(180, 170, 150, 140);
        let d_x = (px - fx * 5.0 + ((anim_time * 12.0).sin() * 2.0)).round() as i32;
        let d_y = (py - fy * 5.0 + 3.0).round() as i32;
        canvas.set_pixel(d_x, d_y, dust_col);
        canvas.set_pixel(d_x + 1, d_y - 1, dust_col);
    }

    // Frantic ragdoll leg kicking when captured vs trotting when running
    let (leg_step_f, leg_step_b) = if state == PreyState::CapturedTalons {
        let k1 = (anim_time * 38.0).sin() * 3.8;
        let k2 = (anim_time * 38.0 + 1.8).sin() * 3.8;
        (k1, k2)
    } else {
        let s = (anim_time * 18.0).sin() * 2.0;
        (s, -s)
    };

    // Sizzling grease smoke wisps when seared and airborne
    if roast_pct > 0.40 && (state == PreyState::CapturedTalons || state == PreyState::InMaw) {
        let smoke_col = Rgba(70, 75, 85, 140);
        let sm_x = (px - fx * 6.0 + (anim_time * 14.0).sin() * 2.5).round() as i32;
        let sm_y = (py - fy * 6.0 - 4.0).round() as i32;
        canvas.set_pixel(sm_x, sm_y, smoke_col);
        canvas.set_pixel(sm_x + 1, sm_y - 1, smoke_col);
    }

    // Draw prey species
    match kind {
        PreyKind::Sheep => {
            let base_raw = Rgba(240, 240, 235, 255);
            let (wool_col, inner_glow, is_ignited) = get_pyrolytic_colors(base_raw, roast_pct, is_wight);
            let face_col = if roast_pct > 0.5 { Rgba(35, 20, 15, 255) } else { Rgba(45, 45, 50, 255) };

            // 4 ragdoll trotting/kicking legs
            let leg_f1 = (px + 3.0 * fx - 2.5 * rx + leg_step_f * fx, py + 3.0 * fy - 2.5 * ry + leg_step_f * fy);
            let leg_f2 = (px + 3.0 * fx + 2.5 * rx + leg_step_b * fx, py + 3.0 * fy + 2.5 * ry + leg_step_b * fy);
            let leg_b1 = (px - 3.0 * fx - 2.5 * rx + leg_step_b * fx, py - 3.0 * fy - 2.5 * ry + leg_step_b * fy);
            let leg_b2 = (px - 3.0 * fx + 2.5 * rx + leg_step_f * fx, py - 3.0 * fy + 2.5 * ry + leg_step_f * fy);

            canvas.draw_line(ix, iy, leg_f1.0.round() as i32, leg_f1.1.round() as i32, face_col, 1);
            canvas.draw_line(ix, iy, leg_f2.0.round() as i32, leg_f2.1.round() as i32, face_col, 1);
            canvas.draw_line(ix, iy, leg_b1.0.round() as i32, leg_b1.1.round() as i32, face_col, 1);
            canvas.draw_line(ix, iy, leg_b2.0.round() as i32, leg_b2.1.round() as i32, face_col, 1);

            canvas.fill_ellipse(ix, iy, 4, 3, wool_col);
            canvas.fill_ellipse(ix - 1, iy, 3, 2, inner_glow);

            if is_ignited {
                canvas.set_pixel(ix + 2, iy - 1, Rgba(255, 220, 80, 230));
                canvas.set_pixel(ix - 2, iy + 1, Rgba(255, 120, 30, 230));
            }

            let hx = (px + 4.5 * fx).round() as i32;
            let hy = (py + 4.5 * fy).round() as i32;
            canvas.fill_ellipse(hx, hy, 2, 2, face_col);
            canvas.set_pixel((hx as f32 + 1.8 * rx).round() as i32, (hy as f32 + 1.8 * ry).round() as i32, face_col);
            canvas.set_pixel((hx as f32 - 1.8 * rx).round() as i32, (hy as f32 - 1.8 * ry).round() as i32, face_col);
            canvas.set_pixel(hx, hy, Rgba(240, 240, 240, 255));
        }

        PreyKind::Pig | PreyKind::WildBoar => {
            let is_boar = kind == PreyKind::WildBoar;
            let base_raw = if is_boar { Rgba(85, 60, 45, 255) } else { Rgba(245, 170, 160, 255) };
            let (body_col, inner_glow, is_ignited) = get_pyrolytic_colors(base_raw, roast_pct, is_wight);
            let snout_col = if is_boar { Rgba(55, 38, 28, 255) } else { Rgba(225, 130, 120, 255) };

            let l_col = if is_boar { Rgba(45, 30, 20, 255) } else { Rgba(210, 135, 125, 255) };
            let leg_f1 = (px + 3.0 * fx - 2.0 * rx + leg_step_f * fx, py + 3.0 * fy - 2.0 * ry + leg_step_f * fy);
            let leg_f2 = (px + 3.0 * fx + 2.0 * rx + leg_step_b * fx, py + 3.0 * fy + 2.0 * ry + leg_step_b * fy);
            let leg_b1 = (px - 3.0 * fx - 2.0 * rx + leg_step_b * fx, py - 3.0 * fy - 2.0 * ry + leg_step_b * fy);
            let leg_b2 = (px - 3.0 * fx + 2.0 * rx + leg_step_f * fx, py - 3.0 * fy + 2.0 * ry + leg_step_f * fy);

            canvas.draw_line(ix, iy, leg_f1.0.round() as i32, leg_f1.1.round() as i32, l_col, 1);
            canvas.draw_line(ix, iy, leg_f2.0.round() as i32, leg_f2.1.round() as i32, l_col, 1);
            canvas.draw_line(ix, iy, leg_b1.0.round() as i32, leg_b1.1.round() as i32, l_col, 1);
            canvas.draw_line(ix, iy, leg_b2.0.round() as i32, leg_b2.1.round() as i32, l_col, 1);

            canvas.fill_ellipse(ix, iy, 5, 3, body_col);
            canvas.fill_ellipse(ix - 1, iy, 3, 2, inner_glow);

            if is_ignited {
                canvas.set_pixel(ix + 3, iy - 1, Rgba(255, 210, 60, 230));
            }

            let sx = (px + 5.0 * fx).round() as i32;
            let sy = (py + 5.0 * fy).round() as i32;
            canvas.fill_ellipse(sx, sy, 2, 2, snout_col);

            if is_boar {
                canvas.set_pixel((sx as f32 + 2.0 * rx).round() as i32, (sy as f32 + 2.0 * ry).round() as i32, Rgba(250, 245, 220, 255));
                canvas.set_pixel((sx as f32 - 2.0 * rx).round() as i32, (sy as f32 - 2.0 * ry).round() as i32, Rgba(250, 245, 220, 255));
            } else {
                let tx = (px - 5.5 * fx).round() as i32;
                let ty = (py - 5.5 * fy).round() as i32;
                canvas.set_pixel(tx, ty, snout_col);
                canvas.set_pixel(tx + 1, ty - 1, snout_col);
            }
        }

        PreyKind::Bull => {
            let base_raw = Rgba(45, 35, 30, 255);
            let (body_col, inner_glow, is_ignited) = get_pyrolytic_colors(base_raw, roast_pct, is_wight);
            let horn_col = Rgba(235, 230, 210, 255);

            let leg_f1 = (px + 4.0 * fx - 3.0 * rx + leg_step_f * fx, py + 4.0 * fy - 3.0 * ry + leg_step_f * fy);
            let leg_f2 = (px + 4.0 * fx + 3.0 * rx + leg_step_b * fx, py + 4.0 * fy + 3.0 * ry + leg_step_b * fy);
            let leg_b1 = (px - 4.0 * fx - 3.0 * rx + leg_step_b * fx, py - 4.0 * fy - 3.0 * ry + leg_step_b * fy);
            let leg_b2 = (px - 4.0 * fx + 3.0 * rx + leg_step_f * fx, py - 4.0 * fy + 3.0 * ry + leg_step_f * fy);

            canvas.draw_line(ix, iy, leg_f1.0.round() as i32, leg_f1.1.round() as i32, body_col, 2);
            canvas.draw_line(ix, iy, leg_f2.0.round() as i32, leg_f2.1.round() as i32, body_col, 2);
            canvas.draw_line(ix, iy, leg_b1.0.round() as i32, leg_b1.1.round() as i32, body_col, 2);
            canvas.draw_line(ix, iy, leg_b2.0.round() as i32, leg_b2.1.round() as i32, body_col, 2);

            canvas.fill_ellipse(ix, iy, 6, 4, body_col);
            canvas.fill_ellipse((px + 2.0 * fx).round() as i32, (py + 2.0 * fy).round() as i32, 4, 3, inner_glow);

            if is_ignited {
                canvas.set_pixel(ix + 3, iy - 2, Rgba(255, 230, 90, 230));
            }

            let hx = (px + 6.0 * fx).round() as i32;
            let hy = (py + 6.0 * fy).round() as i32;
            canvas.fill_ellipse(hx, hy, 3, 2, body_col);

            let h_l = (hx as f32 - 4.0 * rx + 2.0 * fx, hy as f32 - 4.0 * ry + 2.0 * fy);
            let h_r = (hx as f32 + 4.0 * rx + 2.0 * fx, hy as f32 + 4.0 * ry + 2.0 * fy);
            canvas.draw_line(hx, hy, h_l.0.round() as i32, h_l.1.round() as i32, horn_col, 1);
            canvas.draw_line(hx, hy, h_r.0.round() as i32, h_r.1.round() as i32, horn_col, 1);
        }

        PreyKind::Goat => {
            let base_raw = Rgba(160, 145, 130, 255);
            let (body_col, inner_glow, is_ignited) = get_pyrolytic_colors(base_raw, roast_pct, is_wight);
            let horn_col = Rgba(120, 100, 80, 255);

            let leg_f1 = (px + 3.0 * fx - 2.0 * rx + leg_step_f * fx, py + 3.0 * fy - 2.0 * ry + leg_step_f * fy);
            let leg_f2 = (px + 3.0 * fx + 2.0 * rx + leg_step_b * fx, py + 3.0 * fy + 2.0 * ry + leg_step_b * fy);
            let leg_b1 = (px - 3.0 * fx - 2.0 * rx + leg_step_b * fx, py - 3.0 * fy - 2.0 * ry + leg_step_b * fy);
            let leg_b2 = (px - 3.0 * fx + 2.0 * rx + leg_step_f * fx, py - 3.0 * fy + 2.0 * ry + leg_step_f * fy);

            canvas.draw_line(ix, iy, leg_f1.0.round() as i32, leg_f1.1.round() as i32, body_col, 1);
            canvas.draw_line(ix, iy, leg_f2.0.round() as i32, leg_f2.1.round() as i32, body_col, 1);
            canvas.draw_line(ix, iy, leg_b1.0.round() as i32, leg_b1.1.round() as i32, body_col, 1);
            canvas.draw_line(ix, iy, leg_b2.0.round() as i32, leg_b2.1.round() as i32, body_col, 1);

            canvas.fill_ellipse(ix, iy, 4, 2, body_col);
            canvas.fill_ellipse(ix - 1, iy, 3, 2, inner_glow);

            if is_ignited {
                canvas.set_pixel(ix + 2, iy - 1, Rgba(255, 200, 50, 220));
            }

            let hx = (px + 4.5 * fx).round() as i32;
            let hy = (py + 4.5 * fy).round() as i32;
            canvas.fill_ellipse(hx, hy, 2, 2, body_col);
            canvas.draw_line(hx, hy, (hx as f32 - 3.0 * fx - 1.5 * rx).round() as i32, (hy as f32 - 3.0 * fy - 1.5 * ry).round() as i32, horn_col, 1);
            canvas.draw_line(hx, hy, (hx as f32 - 3.0 * fx + 1.5 * rx).round() as i32, (hy as f32 - 3.0 * fy + 1.5 * ry).round() as i32, horn_col, 1);
        }

        PreyKind::Stag => {
            let base_raw = Rgba(150, 95, 55, 255);
            let (body_col, inner_glow, is_ignited) = get_pyrolytic_colors(base_raw, roast_pct, is_wight);
            let antler_col = Rgba(215, 200, 175, 255);

            let leg_f1 = (px + 3.5 * fx - 2.0 * rx + leg_step_f * 1.3 * fx, py + 3.5 * fy - 2.0 * ry + leg_step_f * 1.3 * fy);
            let leg_f2 = (px + 3.5 * fx + 2.0 * rx + leg_step_b * 1.3 * fx, py + 3.5 * fy + 2.0 * ry + leg_step_b * 1.3 * fy);
            let leg_b1 = (px - 3.5 * fx - 2.0 * rx + leg_step_b * 1.3 * fx, py - 3.5 * fy - 2.0 * ry + leg_step_b * 1.3 * fy);
            let leg_b2 = (px - 3.5 * fx + 2.0 * rx + leg_step_f * 1.3 * fx, py - 3.5 * fy + 2.0 * ry + leg_step_f * 1.3 * fy);

            canvas.draw_line(ix, iy, leg_f1.0.round() as i32, leg_f1.1.round() as i32, body_col, 1);
            canvas.draw_line(ix, iy, leg_f2.0.round() as i32, leg_f2.1.round() as i32, body_col, 1);
            canvas.draw_line(ix, iy, leg_b1.0.round() as i32, leg_b1.1.round() as i32, body_col, 1);
            canvas.draw_line(ix, iy, leg_b2.0.round() as i32, leg_b2.1.round() as i32, body_col, 1);

            canvas.fill_ellipse(ix, iy, 5, 2, body_col);
            canvas.fill_ellipse(ix - 1, iy, 3, 2, inner_glow);

            if is_ignited {
                canvas.set_pixel(ix + 3, iy - 1, Rgba(255, 220, 70, 230));
            }

            let hx = (px + 5.5 * fx).round() as i32;
            let hy = (py + 5.5 * fy).round() as i32;
            canvas.fill_ellipse(hx, hy, 2, 2, body_col);

            let ant_l = (hx as f32 - 3.5 * rx + 2.0 * fx, hy as f32 - 3.5 * ry + 2.0 * fy);
            let ant_r = (hx as f32 + 3.5 * rx + 2.0 * fx, hy as f32 + 3.5 * ry + 2.0 * fy);
            canvas.draw_line(hx, hy, ant_l.0.round() as i32, ant_l.1.round() as i32, antler_col, 1);
            canvas.draw_line(hx, hy, ant_r.0.round() as i32, ant_r.1.round() as i32, antler_col, 1);
            canvas.set_pixel((ant_l.0 + 1.0 * fx).round() as i32, (ant_l.1 + 1.0 * fy).round() as i32, antler_col);
            canvas.set_pixel((ant_r.0 + 1.0 * fx).round() as i32, (ant_r.1 + 1.0 * fy).round() as i32, antler_col);
        }
    }
}
