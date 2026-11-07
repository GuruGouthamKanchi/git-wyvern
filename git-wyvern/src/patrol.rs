use crate::canvas::Canvas;
use crate::palette::{DragonPalette, Rgba};
use crate::prey::{get_dragon_hunt_config, get_dragon_seed, render_prey, HuntStyle, PreyState};
use crate::topdown::render_topdown_wyvern_frame;
use std::f32::consts::PI;

pub struct Ember {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub age: u32,
    pub max_age: u32,
    pub color: Rgba,
}

/// Calculate base flight trajectory on the enlarged 640x640 sky arena (doubled space).
/// Center of the arena is (320.0, 320.0).
/// Every single one of the 16 canon dragons has its own mathematically distinct flight path,
/// precisely tuned so that massive wings and elongated tails never touch or cross the boundaries.
fn get_base_trajectory_pos(key: &str, tau: f32) -> (f32, f32) {
    let cx = 320.0;
    let cy = 320.0;

    match key {
        "drogon" => {
            // Drogon (The Black Dread Reborn): Aggressive Lemniscate of Gerono (Power Figure-8)
            let x = cx + 118.0 * tau.sin();
            let y = cy + 78.0 * (2.0 * tau).sin();
            (x, y)
        }
        "rhaegal" => {
            // Rhaegal (The Jade Serpent): Sinister 3:2 Lissajous serpentine canyon weave
            let x = cx + 108.0 * tau.cos() + 25.0 * (3.0 * tau).cos();
            let y = cy + 74.0 * (2.0 * tau).sin();
            (x, y)
        }
        "viserion" => {
            // Viserion (The Golden Frost): Elegant breathing thermal spiral glide
            let r_x = 108.0 + 18.0 * tau.sin();
            let r_y = 74.0 + 12.0 * tau.sin();
            let x = cx + r_x * tau.cos();
            let y = cy + r_y * tau.sin();
            (x, y)
        }
        "wight_viserion" => {
            // Wight Viserion (The Undead Ice Dragon): Erratic, jagged stutter-loop with unnatural pivots
            let x = cx + 108.0 * tau.cos() - 25.0 * (2.0 * tau).sin();
            let y = cy + 68.0 * tau.sin() + 25.0 * (2.0 * tau).cos();
            (x, y)
        }
        "caraxes" => {
            // Caraxes (The Blood Wyrm): Sinuous elongated ribbon wave with secondary undulating ripple
            let x = cx + 124.0 * tau.sin();
            let y = cy + 74.0 * (2.0 * tau).sin() + 12.0 * (4.0 * tau).sin();
            (x, y)
        }
        "vhagar" => {
            // Vhagar (The Queen of All Dragons): Colossal wide perimeter sweep with low ground stoop
            let x = cx + 112.0 * tau.cos();
            let y = cy + 76.0 * tau.sin() - 18.0 * (2.0 * tau).sin();
            (x, y)
        }
        "syrax" => {
            // Syrax (The Golden Queen): Aristocratic regal teardrop loop
            let x = cx + 105.0 * tau.cos() * (1.0 - 0.20 * tau.sin());
            let y = cy + 74.0 * tau.sin();
            (x, y)
        }
        "meleys" => {
            // Meleys (The Red Queen): Supersonic ultra-flat oval interceptor path
            let x = cx + 120.0 * tau.cos();
            let y = cy + 54.0 * tau.sin();
            (x, y)
        }
        "sunfyre" => {
            // Sunfyre (The Golden Dragon): Trifolium 3-petal butterfly aerobatic loop
            let r = 85.0 + 24.0 * (3.0 * tau).cos();
            let x = cx + r * tau.cos();
            let y = cy + r * tau.sin() * 0.74;
            (x, y)
        }
        "vermithor" => {
            // Vermithor (The Bronze Fury): Heavy horseshoe brute patrol with deep predatory stoop
            let x = cx + 108.0 * tau.cos();
            let y = cy + 68.0 * tau.sin() + 22.0 * (2.0 * tau).cos();
            (x, y)
        }
        "silverwing" => {
            // Silverwing (The Pale Lady): Regal horizontal Lemniscate of Bernoulli (Infinity loop)
            let den = 1.0 + tau.sin() * tau.sin();
            let x = cx + 116.0 * tau.cos() / den;
            let y = cy + 76.0 * tau.sin() * tau.cos() / den;
            (x, y)
        }
        "seasmoke" => {
            // Seasmoke (The Pale Ghost): Coastal surf-skimming oval with upward thermal hook
            let x = cx + 108.0 * tau.cos() + 16.0 * (2.0 * tau).sin();
            let y = cy + 62.0 * tau.sin() - 20.0 * (1.0 - tau.cos());
            (x, y)
        }
        "sheepstealer" => {
            // Sheepstealer (The Wild Brown Dragon): Erratic moorland zigzag scavenger loop
            let x = cx + 104.0 * tau.cos() + 25.0 * (3.0 * tau).cos();
            let y = cy + 65.0 * tau.sin() + 22.0 * (3.0 * tau).sin();
            (x, y)
        }
        "balerion" => {
            // Balerion (The Black Dread): Colossal apocalyptic eclipse sweep spanning maximum sky
            let x = cx + 118.0 * tau.cos() + 16.0 * (2.0 * tau).cos();
            let y = cy + 76.0 * tau.sin() - 22.0 * (2.0 * tau).sin();
            (x, y)
        }
        "moondancer" => {
            // Moondancer (The Quick-Wing): Falcon-like 4-cusp astroid canyon darting
            let c = tau.cos();
            let s = tau.sin();
            let x = cx + 118.0 * c * (0.68 + 0.32 * (2.0 * c * c - 1.0));
            let y = cy + 78.0 * s * (0.68 - 0.32 * (2.0 * c * c - 1.0));
            (x, y)
        }
        "tessarion" => {
            // Tessarion (The Blue Queen): Swift acrobatic 4-leaf cloverleaf bank patrol
            let x = cx + 115.0 * tau.cos() + 28.0 * (2.0 * tau).sin();
            let y = cy + 74.0 * tau.sin() + 25.0 * (2.0 * tau).cos();
            (x, y)
        }
        _ => {
            let x = cx + 115.0 * tau.cos();
            let y = cy + 74.0 * tau.sin();
            (x, y)
        }
    }
}

/// Retrieve position and velocity vector (dx, dy) via 4th-order central difference
fn get_base_trajectory(key: &str, tau: f32) -> (f32, f32, f32, f32) {
    let (x, y) = get_base_trajectory_pos(key, tau);
    let dt = 0.006;
    let (x_fwd, y_fwd) = get_base_trajectory_pos(key, tau + dt);
    let (x_bwd, y_bwd) = get_base_trajectory_pos(key, tau - dt);
    let vx = (x_fwd - x_bwd) / (2.0 * dt);
    let vy = (y_fwd - y_bwd) / (2.0 * dt);
    (x, y, vx, vy)
}


pub fn render_patrol_frame(
    pal: &DragonPalette,
    frame_idx: usize,
    total_frames: usize,
    scale: f32,
    canvas: &mut Canvas,
    embers: &mut Vec<Ember>,
) {
    let t = frame_idx as f32 / total_frames as f32;
    let tau = t * 2.0 * PI;

    let seed = get_dragon_seed(pal.key);
    let (prey_kind, hunt_style, feed_duration) = get_dragon_hunt_config(pal.key);
    let is_wight = pal.key == "wight_viserion" || pal.is_plasma;
    let is_caraxes = pal.key == "caraxes";

    // (Transparent sky canvas: no floating static dirt specks on GitHub profiles)

    // -----------------------------------------------------------------
    // 1. EMPTY-TIME DRAGONFIRE SPITTING (All 16 Wyverns spit fire at empty sky!)
    // -----------------------------------------------------------------
    // Phase 1 (Frames 0..48): The dragon soars across open sky with NO PREY present.
    // [0.08 .. 0.12): Pre-ignition esophageal furnace glow
    // [0.12 .. 0.26): Full sustained split dragonfire jet across empty sky
    // [0.26 .. 0.34]: Detached traveling plume and cooling smoke embers
    // -----------------------------------------------------------------
    let empty_fire = if t >= 0.08 && t < 0.12 {
        -1.0 + (t - 0.08) / 0.04
    } else if t >= 0.12 && t <= 0.26 {
        (t - 0.12) / 0.14
    } else if t > 0.26 && t <= 0.34 {
        1.0 + (t - 0.26) / 0.08
    } else {
        -999.0
    };

    // -----------------------------------------------------------------
    // 2. PROCEDURALLY RANDOMIZED PIXEL PREY (Reduced generation: Phase 2 only)
    // -----------------------------------------------------------------
    // Prey spawns at completely random pixel coordinates across the terrain [70..250]
    // -----------------------------------------------------------------
    let h1 = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    let h2 = h1.wrapping_mul(1664525).wrapping_add(1013904223);
    let target_x = 215.0 + ((h1 >> 8) % 210) as f32;
    let target_y = 215.0 + ((h2 >> 8) % 210) as f32;

    let t_spawn = 0.50;  // No prey during first half of loop!
    let t_alert = 0.60;
    let t_strike = 0.70; // Precise intercept moment
    let t_consumed = t_strike + feed_duration;

    let tau_strike = t_strike * 2.0 * PI;
    let (base_sx, base_sy, base_svx, base_svy) = get_base_trajectory(pal.key, tau_strike);

    let prey_heading = base_svy.atan2(base_svx) + 0.45;
    let prey_fx = prey_heading.cos();
    let prey_fy = prey_heading.sin();
    let prey_speed = 40.0;

    let (prey_x, prey_y) = if t < t_strike {
        let dt_strike = t_strike - t;
        (target_x - prey_fx * prey_speed * dt_strike, target_y - prey_fy * prey_speed * dt_strike)
    } else {
        (target_x, target_y)
    };

    let prey_state = if t < t_spawn {
        PreyState::Grazing
    } else if t < t_alert {
        PreyState::Grazing
    } else if t < t_strike {
        PreyState::Fleeing
    } else if t < t_strike + feed_duration * 0.45 {
        match hunt_style {
            HuntStyle::FlameRoastAndToss | HuntStyle::RaptorTalonPin => PreyState::CapturedTalons,
            HuntStyle::CavernousGulp | HuntStyle::ViperNeckStrike | HuntStyle::RoyalSwoopGulp | HuntStyle::FrostShatter => PreyState::InMaw,
        }
    } else if t < t_consumed {
        match hunt_style {
            HuntStyle::FlameRoastAndToss => {
                if t < t_strike + feed_duration * 0.70 { PreyState::CapturedTalons } else { PreyState::InMaw }
            }
            HuntStyle::RaptorTalonPin => {
                if t < t_strike + feed_duration * 0.75 { PreyState::CapturedTalons } else { PreyState::InMaw }
            }
            _ => PreyState::InMaw,
        }
    } else {
        PreyState::Swallowed
    };

    // Hunt flame burst for flame-roasters
    let hunt_fire = if (hunt_style == HuntStyle::FlameRoastAndToss || hunt_style == HuntStyle::FrostShatter) && t >= 0.60 && t <= 0.72 {
        if t < 0.62 {
            -1.0 + (t - 0.60) / 0.02
        } else if t <= 0.68 {
            (t - 0.62) / 0.06
        } else {
            1.0 + (t - 0.68) / 0.04
        }
    } else {
        -999.0
    };

    let flame_progress = empty_fire.max(hunt_fire);

    let roast_pct = if hunt_style == HuntStyle::FlameRoastAndToss || hunt_style == HuntStyle::FrostShatter {
        if t < 0.60 {
            0.0
        } else if t < 0.70 {
            (t - 0.60) / 0.10
        } else {
            1.0
        }
    } else {
        0.0
    };

    // -----------------------------------------------------------------
    // 3. PREDATORY INTERCEPTION TO RANDOM PIXEL LOCATION
    // -----------------------------------------------------------------
    let (raw_px, raw_py, raw_vx, raw_vy) = get_base_trajectory(pal.key, tau);

    let (px, py, vx, vy, altitude_factor, dive_scale_mult, reach_talons) = if t >= 0.50 && t <= 0.90 {
        let u_hunt = (t - 0.50) / (0.90 - 0.50);
        let w_hunt = (u_hunt * PI).sin().powi(2);
        let dive_depth = (u_hunt * PI).sin().powf(1.6);

        let alt_factor = (1.0 - dive_depth * 0.97).max(0.03);
        let scale_mult = 1.0 + dive_depth * 0.16;

        let u_reach = if (0.60..=0.74).contains(&t) {
            ((t - 0.60) / 0.14 * PI).sin().powf(1.4)
        } else {
            0.0
        };

        let cur_target_x = if t < t_strike { prey_x } else { target_x };
        let cur_target_y = if t < t_strike { prey_y } else { target_y };

        let mod_x = raw_px + w_hunt * (cur_target_x - base_sx);
        let mod_y = raw_py + w_hunt * (cur_target_y - base_sy);

        let dt_tan = 0.006;
        let (next_raw_x, next_raw_y) = get_base_trajectory_pos(pal.key, tau + dt_tan);
        let next_u = ((t + dt_tan / (2.0 * PI)) - 0.50) / (0.90 - 0.50);
        let next_w = if (0.0..=1.0).contains(&next_u) { (next_u * PI).sin().powi(2) } else { 0.0 };
        let next_mod_x = next_raw_x + next_w * (cur_target_x - base_sx);
        let next_mod_y = next_raw_y + next_w * (cur_target_y - base_sy);

        let mut mod_vx = (next_mod_x - mod_x) / dt_tan;
        let mut mod_vy = (next_mod_y - mod_y) / dt_tan;

        let strike_diff = (t - t_strike).abs();
        if strike_diff < 0.035 {
            let recoil_damp = 1.0 - (1.0 - strike_diff / 0.035) * 0.30;
            mod_vx *= recoil_damp;
            mod_vy *= recoil_damp;
        }

        (mod_x, mod_y, mod_vx, mod_vy, alt_factor, scale_mult, u_reach)
    } else {
        (raw_px, raw_py, raw_vx, raw_vy, 1.0, 1.0, 0.0)
    };

    let px = px.clamp(170.0, 470.0);
    let py = py.clamp(165.0, 475.0);

    let heading = vy.atan2(vx);

    let dt = 0.005;
    let tau_next = (t + dt) * 2.0 * PI;
    let (_, _, next_vx, next_vy) = get_base_trajectory(pal.key, tau_next);
    let head_next = next_vy.atan2(next_vx);
    let mut diff = head_next - heading;
    while diff > PI { diff -= 2.0 * PI; }
    while diff < -PI { diff += 2.0 * PI; }
    let turn_rate = ((diff / dt) * 0.12).clamp(-1.0, 1.0);

    let num_flaps = match pal.key {
        "caraxes" | "moondancer" | "meleys" => 6,
        "vhagar" => 2,
        "tessarion" => 5,
        _ => 4,
    };
    let wing_phase = if t > t_strike && t < t_strike + 0.12 {
        let u_rec = (t - t_strike) / 0.12;
        (u_rec * 0.50).fract()
    } else {
        (t * num_flaps as f32).fract()
    };

    // -----------------------------------------------------------------
    // 4. MOUTH GAPE FOR BOTH FIRE ROAR & EATING
    // -----------------------------------------------------------------
    let empty_fire_mouth = if empty_fire >= -1.0 && empty_fire < 0.0 {
        (empty_fire + 1.0) * 2.2
    } else if empty_fire >= 0.0 && empty_fire < 0.15 {
        2.2 + (empty_fire / 0.15) * 3.8
    } else if empty_fire >= 0.15 && empty_fire <= 0.85 {
        6.0
    } else if empty_fire > 0.85 && empty_fire <= 1.0 {
        6.0 - ((empty_fire - 0.85) / 0.15) * 4.0
    } else {
        0.0
    };

    let eat_mouth_open = if t >= 0.66 && t <= 0.74 {
        6.8
    } else if (hunt_style == HuntStyle::FlameRoastAndToss || hunt_style == HuntStyle::RaptorTalonPin) && t >= 0.76 && t <= 0.84 {
        6.0
    } else {
        0.0
    };

    let mouth_open = empty_fire_mouth.max(eat_mouth_open);
    let jaw_flare = (mouth_open / 6.8).clamp(0.0, 1.0);

    let bolus_pos = if t >= t_strike + feed_duration * 0.40 && t <= t_consumed + 0.12 {
        let t_start_b = t_strike + feed_duration * 0.40;
        let t_end_b = t_consumed + 0.12;
        let u_bolus = (t - t_start_b) / (t_end_b - t_start_b);
        1.0 + u_bolus * 5.8
    } else {
        -1.0
    };

    let post_feed_glow = if t > t_consumed && t < t_consumed + 0.16 {
        let u_sat = (t - t_consumed) / 0.16;
        (u_sat * PI).sin().powf(1.2)
    } else {
        0.0
    };

    let patrol_scale_mult = match pal.key {
        "vhagar" => 1.10,
        "balerion" => 1.12,
        "caraxes" => 1.08,
        "moondancer" => 1.06,
        _ => 1.08,
    };
    let patrol_scale = patrol_scale_mult * scale * dive_scale_mult;

    let fx = heading.cos();
    let fy = heading.sin();

    // Active dragonfire source calculation and Ground Fire Light Pool
    let fl_intensity = if flame_progress >= 0.0 && flame_progress <= 1.0 {
        1.0_f32
    } else if flame_progress > 1.0 && flame_progress <= 1.6 {
        (1.0 - (flame_progress - 1.0) / 0.6).max(0.0)
    } else if flame_progress >= -0.4 && flame_progress < 0.0 {
        ((flame_progress + 0.4) / 0.4 * 0.5).max(0.0)
    } else {
        0.0_f32
    };

    let flame_src_x = px + 34.0 * patrol_scale * fx;
    let flame_src_y = py + 34.0 * patrol_scale * fy;

    // -----------------------------------------------------------------
    // SURFACE FIRE IMPACT & MULTI-SHADED SURFACE CONCENTRATION
    // (Resolves User Request: Fire impact on the surface it hits in color with different fire shades)
    // -----------------------------------------------------------------
    if fl_intensity > 0.01 {
        // Determine exact surface impact point (ground prey during hunt, or downrange terrain during empty-sky combustion)
        let (impact_x, impact_y) = if hunt_fire > 0.0 {
            (prey_x, prey_y)
        } else {
            let pool_dist = (34.0 + 38.0 * (1.1 - altitude_factor * 0.35)) * patrol_scale;
            (px + pool_dist * fx, py + pool_dist * fy)
        };

        // 5 Distinct Species-Specific Fire Shades for Surface Impact
        let (scorch_col, outer_fire, mid_fire, inner_fire, core_fire) = if is_wight {
            (
                Rgba(16, 28, 55, 230),   // Frost-charred necrotic ground rim
                Rgba(0, 160, 255, 240),  // Glacial cobalt flame
                Rgba(34, 211, 238, 255), // Searing electric cyan
                Rgba(165, 243, 252, 255),// Superheated pale plasma
                Rgba(255, 255, 255, 255),// White-hot ionization core
            )
        } else if pal.key == "sunfyre" {
            (
                Rgba(75, 45, 10, 230),   // Dark bronze solar char
                Rgba(217, 119, 6, 240),  // Deep golden amber flame
                Rgba(250, 204, 21, 255), // Radiant molten gold
                Rgba(254, 240, 138, 255),// Canary solar mantle
                Rgba(255, 255, 240, 255),// Blinding photonic solar core
            )
        } else if is_caraxes {
            (
                Rgba(70, 12, 10, 230),   // Blackened blood-char
                Rgba(185, 28, 28, 240),  // Deep crimson flame
                Rgba(239, 68, 68, 255),  // Searing vermilion
                Rgba(251, 146, 60, 255), // Intense orange mantle
                Rgba(255, 237, 213, 255),// Incandescent core
            )
        } else if pal.key == "rhaegal" {
            (
                Rgba(25, 55, 18, 230),   // Smoldering bronze-jade char
                Rgba(22, 163, 74, 240),  // Deep emerald flame
                Rgba(74, 222, 128, 255), // Vibrant jade fire
                Rgba(190, 242, 100, 255),// Chartreuse mantle
                Rgba(254, 252, 232, 255),// Incandescent core
            )
        } else if pal.key == "tessarion" {
            (
                Rgba(45, 25, 70, 230),   // Burnt copper-indigo char
                Rgba(37, 99, 235, 240),  // Saturated cobalt flame
                Rgba(56, 189, 248, 255), // Electric azure
                Rgba(186, 230, 253, 255),// Pale ionization glow
                Rgba(255, 255, 255, 255),// Pure white plasma core
            )
        } else if pal.key == "viserion" {
            (
                Rgba(70, 48, 20, 230),   // Burnt gold-cream char
                Rgba(217, 119, 6, 240),  // Dark amber flame
                Rgba(245, 158, 11, 255), // Radiant molten gold
                Rgba(253, 224, 71, 255), // Warm yellow flame
                Rgba(255, 251, 235, 255),// Incandescent cream core
            )
        } else {
            // Drogon, Balerion, Vermithor, Vhagar, Syrax, Silverwing, Seasmoke, Sheepstealer, Moondancer
            (
                Rgba(55, 14, 12, 230),   // Volcanic cinder char
                Rgba(220, 38, 38, 240),  // Roaring blood-orange fire
                Rgba(249, 115, 22, 255), // Searing fiery orange
                Rgba(253, 224, 71, 255), // High-temp bright yellow
                Rgba(255, 255, 240, 255),// White-hot blast epicenter
            )
        };

        // 1. Expanding Elliptical Ground Light Pool & Thermal Scorch Ring on Surface
        let scorch_rx = (((28.0 + 16.0 * altitude_factor) * patrol_scale * fl_intensity).round() as i32).max(5);
        let scorch_ry = (((18.0 + 10.0 * altitude_factor) * patrol_scale * fl_intensity).round() as i32).max(4);
        canvas.fill_ellipse_stipple_50(impact_x.round() as i32, impact_y.round() as i32, scorch_rx, scorch_ry, scorch_col);

        // 2. Outer Expanding Flame Surface Layer (First fire shade)
        let out_rx = (((20.0 + 10.0 * (1.0 - altitude_factor * 0.4)) * patrol_scale * fl_intensity).round() as i32).max(4);
        let out_ry = (((13.0 + 7.0 * (1.0 - altitude_factor * 0.4)) * patrol_scale * fl_intensity).round() as i32).max(3);
        canvas.fill_ellipse_stipple_50(impact_x.round() as i32, impact_y.round() as i32, out_rx, out_ry, outer_fire);

        // 3. Mid Roaring Fire Surface Mantle (Second fire shade)
        let mid_rx = (((13.0 + 6.0 * (1.0 - altitude_factor * 0.5)) * patrol_scale * fl_intensity).round() as i32).max(3);
        let mid_ry = (((8.5 + 4.0 * (1.0 - altitude_factor * 0.5)) * patrol_scale * fl_intensity).round() as i32).max(2);
        canvas.fill_ellipse(impact_x.round() as i32, impact_y.round() as i32, mid_rx, mid_ry, mid_fire);

        // 4. Inner Superheated Fire Layer (Third fire shade)
        let in_rx = (((7.5 + 3.0 * (1.0 - altitude_factor * 0.5)) * patrol_scale * fl_intensity).round() as i32).max(2);
        let in_ry = (((5.0 + 2.0 * (1.0 - altitude_factor * 0.5)) * patrol_scale * fl_intensity).round() as i32).max(1);
        canvas.fill_ellipse(impact_x.round() as i32, impact_y.round() as i32, in_rx, in_ry, inner_fire);

        // 5. White-Hot Incandescent Thermal Blast Epicenter (Fourth fire shade)
        let core_rx = (((3.5 + 1.5 * (1.0 - altitude_factor * 0.5)) * patrol_scale * fl_intensity).round() as i32).max(1);
        let core_ry = (((2.2 + 1.0 * (1.0 - altitude_factor * 0.5)) * patrol_scale * fl_intensity).round() as i32).max(1);
        canvas.fill_ellipse(impact_x.round() as i32, impact_y.round() as i32, core_rx, core_ry, core_fire);

        // 6. Dynamic Radial Surface Fire Splash Tendrils (Splattering fire tongues across the ground in different shades)
        let num_tendrils = 8;
        for sp in 0..num_tendrils {
            let sp_ang = sp as f32 * PI * 2.0 / num_tendrils as f32 + (frame_idx as f32 * 0.35);
            let len_mod = ((sp * 7 + frame_idx * 3) % 5) as f32 * 2.0 * patrol_scale;
            let sp_len = (mid_rx as f32 * 1.5 + len_mod).max(4.0);
            
            let sp_cos = sp_ang.cos();
            let sp_sin = sp_ang.sin();
            let p_root_x = impact_x + sp_cos * (core_rx as f32);
            let p_root_y = impact_y + sp_sin * (core_ry as f32);
            let p_mid_x = impact_x + sp_cos * (sp_len * 0.6);
            let p_mid_y = impact_y + sp_sin * (sp_len * 0.6 * 0.7);
            let p_tip_x = impact_x + sp_cos * sp_len;
            let p_tip_y = impact_y + sp_sin * sp_len * 0.7;

            // Tendril root is bright inner fire; tip transitions to outer flame shade
            canvas.draw_line(p_root_x.round() as i32, p_root_y.round() as i32, p_mid_x.round() as i32, p_mid_y.round() as i32, inner_fire, 2);
            canvas.draw_line(p_mid_x.round() as i32, p_mid_y.round() as i32, p_tip_x.round() as i32, p_tip_y.round() as i32, outer_fire, 1);
            // Splattered droplet ember at splash tip
            canvas.set_pixel(p_tip_x.round() as i32, p_tip_y.round() as i32, mid_fire);
        }

        // 7. Scattering Surface Embers & Thermal Shockwave Sparks
        for emb in 0..10 {
            let e_ang = emb as f32 * PI / 5.0 + frame_idx as f32 * 0.25;
            let e_dist = out_rx as f32 * 1.1 + ((emb * 11 + frame_idx * 2) % 7) as f32 * 1.6 * patrol_scale;
            let ex = (impact_x + e_ang.cos() * e_dist).round() as i32;
            let ey = (impact_y + e_ang.sin() * e_dist * 0.7).round() as i32;
            let e_col = if emb % 2 == 0 { inner_fire } else { mid_fire };
            canvas.set_pixel(ex, ey, e_col);
        }
    }

    // Render grazing/fleeing prey on terrain before dragon swoop
    if t >= t_spawn && (prey_state == PreyState::Grazing || prey_state == PreyState::Fleeing) {
        render_prey(
            canvas,
            prey_kind,
            prey_state,
            prey_x,
            prey_y,
            prey_heading,
            t * 144.0,
            roast_pct,
            is_wight,
            Some((flame_src_x, flame_src_y, fl_intensity)),
        );
    }

    render_topdown_wyvern_frame(
        pal,
        wing_phase,
        flame_progress,
        patrol_scale,
        mouth_open,
        px,
        py,
        heading,
        turn_rate,
        vx,
        vy,
        frame_idx,
        altitude_factor,
        reach_talons,
        bolus_pos,
        jaw_flare,
        post_feed_glow,
        canvas,
    );

    // Render captured prey
    if t >= t_strike && t < t_consumed {
        let fx = heading.cos();
        let fy = heading.sin();

        let (held_x, held_y, held_heading) = match prey_state {
            PreyState::CapturedTalons => {
                (px - 10.0 * patrol_scale * fx, py - 10.0 * patrol_scale * fy, heading + PI * 0.5)
            }
            PreyState::InMaw => {
                (px + 26.0 * patrol_scale * fx, py + 26.0 * patrol_scale * fy, heading)
            }
            _ => (px, py, heading),
        };

        render_prey(
            canvas,
            prey_kind,
            prey_state,
            held_x,
            held_y,
            held_heading,
            t * 144.0,
            roast_pct,
            is_wight,
            None,
        );

        if (t - t_strike).abs() < 0.040 {
            let gore_col = if is_wight {
                Rgba(130, 230, 255, 240)
            } else if roast_pct > 0.5 {
                Rgba(255, 140, 30, 230)
            } else {
                Rgba(185, 20, 20, 235)
            };
            let bone_col = Rgba(245, 240, 220, 240);

            for g in 0..6 {
                let g_ang = heading + PI + (g as f32 - 2.5) * 0.45;
                let g_dist = (4.0 + g as f32 * 2.2) * patrol_scale;
                let gx = (held_x + g_dist * g_ang.cos()).round() as i32;
                let gy = (held_y + g_dist * g_ang.sin()).round() as i32;
                canvas.set_pixel(gx, gy, if g % 2 == 0 { gore_col } else { bone_col });
            }
        }
    }

    // Slipstream embers for active dragonfire
    if flame_progress >= 0.0 && flame_progress <= 1.8 {
        let fx = heading.cos();
        let fy = heading.sin();
        let is_detached = flame_progress > 1.0;
        let post_p = if is_detached { flame_progress - 1.0 } else { 0.0 };
        let offset = (26.0 + post_p * 35.0) * patrol_scale;

        let is_smoke = is_detached && post_p > 0.35;
        let emb_col = if is_smoke {
            Rgba(75, 80, 90, 180)
        } else if pal.key == "wight_viserion" {
            Rgba(0, 220, 255, 220)
        } else if pal.key == "sunfyre" {
            Rgba(255, 235, 100, 240)
        } else if pal.key == "rhaegal" {
            Rgba(118, 255, 3, 220)
        } else {
            pal.fire_inner
        };

        embers.push(Ember {
            x: px + offset * fx + if frame_idx % 2 == 0 { 2.0 } else { -2.0 },
            y: py + offset * fy + if frame_idx % 2 == 0 { -2.0 } else { 2.0 },
            vx: -vx * 0.12 + if frame_idx % 2 == 0 { 1.2 } else { -1.2 },
            vy: -vy * 0.10 + if frame_idx % 2 == 0 { -1.2 } else { 1.2 },
            age: 0,
            max_age: if is_smoke { 14 } else { 8 },
            color: emb_col,
        });
    }

    let mut active = Vec::new();
    for mut emb in embers.drain(..) {
        emb.x += emb.vx;
        emb.y += emb.vy;
        emb.age += 1;
        if emb.age < emb.max_age {
            let alpha = ((1.0 - emb.age as f32 / emb.max_age as f32) * emb.color.3 as f32) as u8;
            let mut col = emb.color;
            col.3 = alpha;
            canvas.set_pixel(emb.x.round() as i32, emb.y.round() as i32, col);
            active.push(emb);
        }
    }
    *embers = active;
}
