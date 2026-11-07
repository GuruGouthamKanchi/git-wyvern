use crate::canvas::Canvas;
use crate::palette::{DragonPalette, Rgba};
use std::f32::consts::PI;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScaleStyle {
    Pinecone,     // Overlapping triangular shingles with raised dorsal keels (Drogon, Balerion, Meleys)
    Diamond,      // Sleek interlocking serpentine rhomboid diamond scales (Caraxes, Rhaegal)
    Hexagon,      // Polished geometric hexagonal plates with chiseled bevel borders (Syrax, Tessarion)
    HeavyScutes,  // Thick irregular cobblestone armor scutes with deep erosion channels (Vhagar, Vermithor)
    Pearlescent,  // High-gloss opaline mother-of-pearl tiles with shifting luster (Viserion, Sunfyre, Silverwing, Moondancer)
    Crystalline,  // Jagged angular fractured ice facets with frost needle fractals (Wight Viserion)
    EarthyMottled,// Granular mineral stippling, weathered pores, and calcified sea ripples (Sheepstealer, Seasmoke)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CradleStyle {
    BasaltHearth,    // Dark obsidian volcanic crags with bubbling glowing magma fissures (Drogon, Balerion)
    MossyRuins,      // Ancient carved Valyrian ruin blocks overgrown with emerald moss and ancient roots (Rhaegal, Vhagar)
    IvoryBrazier,    // Sculpted Targaryen ivory-and-gold brazier with draconic claw feet and glowing white-gold coals (Viserion, Silverwing)
    GlacialShelf,    // Frozen permafrost shelf with translucent cyan icicles and jagged frost spires (Wight Viserion)
    SulfurFumarole,  // Cracked volcanic pumice crags with yellow-orange sulfur crusts and bubbling vents (Caraxes)
    GildedPedestal,  // Polished gold-and-sandstone dais with etched sunburst rays (Syrax, Sunfyre)
    SmeltingSlag,    // Heavy industrial furnace slag with molten copper/bronze channels and corrugated ripples (Vermithor, Meleys)
    ReefDriftwood,   // Weathered sea-driftwood, salt-bleached limestone reef, and branching coral spurs (Seasmoke)
    MountainScree,   // Rugged mountain scree nest with granite boulders, coarse gravel, and dry heather twigs (Sheepstealer)
    AmethystGeode,   // Hollow crystalline geode bowl filled with sparkling lilac quartz crystal needles (Moondancer, Tessarion)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VeinStyle {
    MagmaChasm,     // Wide jagged volcanic rifts glowing with superheated plasma (Drogon, Balerion, Meleys)
    BioCapillaries, // Fine serpentine capillary networks pulsing green/gold/carmine (Rhaegal, Caraxes)
    GoldFiligree,   // Delicate spiraling gold-leaf veins tracing scale contours (Viserion, Sunfyre, Syrax)
    FrostFracture,  // Sharp dendritic ice fracture rifts pulsing pale cyan frost fire (Wight Viserion)
    BronzeVeins,    // Metallic molten bronze/copper veins coursing through stone (Vermithor, Sheepstealer)
    TidalVortex,    // 2-arm oceanic spiral whirlpool / tidal gyre transition (Seasmoke)
    LunarAurora,    // Sweeping diagonal celestial aurora veil transition (Silverwing)
    AstralSpiral,   // Diagonal helical spiral ribbon & core bloom transition (Moondancer)
    ElectricArcs,   // High-voltage diagonal diamond-seam lightning discharge (Tessarion)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AccessoryStyle {
    None,           // Pure, clean unblemished scaled dragon egg shell (no spikes or horns)
    MossTendrils,   // Subtle emerald moss tendrils creeping up lower shell from ruin stones (Rhaegal)
    CoralSpurs,     // Calcified coral spurs clinging to lower shell (Seasmoke)
    HeatherTwigs,   // Mountain heather twigs bound around lower shell (Sheepstealer)
    BronzeHoops,    // Heavy riveted bronze reinforcing hoops clasping lower body (Vermithor)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ParticleType {
    VolcanicEmbers, // Fiery sparks drifting upward with thermal turbulence
    FrostMist,      // Pale cyan vapor wisps and sparkling ice dust motes
    SulfurVapors,   // Swirling yellow-tinted brimstone smoke plumes
    SolarGlints,    // Rotating 4-point prismatic starburst glints
    EmeraldSparks,  // Bioluminescent green and gold spore motes
    SeaSpray,       // Cyan brine droplets and sea mist swirling at the base
    BlackAsh,       // Billowing dark soot clouds with incandescent core sparks
    LilacMotes,     // Shimmering lilac and mint fairy dust motes
    ElectricSparks, // Jumping electric blue arc sparks
    ForgeSparks,    // Heavy flying molten iron forge sparks
    SilverSparks,   // Drifting pale silver-white lunar motes
    MountainDust,   // Swirling amber dust motes and dry wind eddies
}

pub struct EggArchetype {
    pub scale_style: ScaleStyle,
    pub cradle_style: CradleStyle,
    pub vein_style: VeinStyle,
    pub accessory: AccessoryStyle,
    pub particle_type: ParticleType,
    pub pulse_speed: f32,
    pub base_fullness: f32,   // Broad rounded hemisphere fullness at bottom
    pub taper_strength: f32,  // Narrow conical apex taper pointing UP at top
    pub aura_tint: Rgba,
    pub cradle_accent: Rgba,
}

pub fn get_egg_archetype(dragon_key: &str, pal: &DragonPalette) -> EggArchetype {
    match dragon_key.to_lowercase().as_str() {
        "drogon" => EggArchetype {
            scale_style: ScaleStyle::Pinecone, // Real overlapping archosaurian scales
            cradle_style: CradleStyle::BasaltHearth,
            vein_style: VeinStyle::MagmaChasm,
            accessory: AccessoryStyle::None, // No spikes or horns
            particle_type: ParticleType::VolcanicEmbers,
            pulse_speed: 1.0,
            base_fullness: 1.10,
            taper_strength: 0.40,
            aura_tint: Rgba(255, 60, 20, 35),
            cradle_accent: Rgba(255, 90, 25, 255),
        },
        "rhaegal" => EggArchetype {
            scale_style: ScaleStyle::Diamond, // Reference diamond scales from caraxes_stage1.gif
            cradle_style: CradleStyle::MossyRuins,
            vein_style: VeinStyle::BioCapillaries,
            accessory: AccessoryStyle::MossTendrils, // Subtle moss on stone
            particle_type: ParticleType::EmeraldSparks,
            pulse_speed: 1.05,
            base_fullness: 1.08,
            taper_strength: 0.38,
            aura_tint: Rgba(40, 220, 80, 30),
            cradle_accent: Rgba(60, 240, 110, 255),
        },
        "viserion" => EggArchetype {
            scale_style: ScaleStyle::Diamond, // Replaced with reference diamond scales from caraxes_stage1.gif
            cradle_style: CradleStyle::IvoryBrazier,
            vein_style: VeinStyle::GoldFiligree,
            accessory: AccessoryStyle::None, // No ram horns
            particle_type: ParticleType::SolarGlints,
            pulse_speed: 0.95,
            base_fullness: 1.12,
            taper_strength: 0.36,
            aura_tint: Rgba(255, 220, 120, 32),
            cradle_accent: Rgba(255, 215, 80, 255),
        },
        "wight_viserion" => EggArchetype {
            scale_style: ScaleStyle::Diamond, // Replaced with reference diamond scales from caraxes_stage1.gif
            cradle_style: CradleStyle::GlacialShelf,
            vein_style: VeinStyle::FrostFracture,
            accessory: AccessoryStyle::None, // No ice spires
            particle_type: ParticleType::FrostMist,
            pulse_speed: 0.85,
            base_fullness: 1.06,
            taper_strength: 0.42,
            aura_tint: Rgba(70, 190, 255, 40),
            cradle_accent: Rgba(120, 225, 255, 255),
        },
        "caraxes" => EggArchetype {
            scale_style: ScaleStyle::Diamond, // Reference diamond scales
            cradle_style: CradleStyle::SulfurFumarole,
            vein_style: VeinStyle::BioCapillaries,
            accessory: AccessoryStyle::None, // No spinal crest spikes
            particle_type: ParticleType::SulfurVapors,
            pulse_speed: 1.35,
            base_fullness: 1.05,
            taper_strength: 0.44,
            aura_tint: Rgba(230, 80, 30, 38),
            cradle_accent: Rgba(240, 195, 30, 255),
        },
        "vhagar" => EggArchetype {
            scale_style: ScaleStyle::Diamond, // Replaced with reference diamond scales from caraxes_stage1.gif
            cradle_style: CradleStyle::MossyRuins,
            vein_style: VeinStyle::BronzeVeins,
            accessory: AccessoryStyle::None, // No spikes or horns
            particle_type: ParticleType::VolcanicEmbers,
            pulse_speed: 0.65,
            base_fullness: 1.15,
            taper_strength: 0.35,
            aura_tint: Rgba(180, 150, 60, 28),
            cradle_accent: Rgba(160, 190, 80, 255),
        },
        "syrax" => EggArchetype {
            scale_style: ScaleStyle::Diamond, // Replaced with reference diamond scales from caraxes_stage1.gif
            cradle_style: CradleStyle::GildedPedestal,
            vein_style: VeinStyle::GoldFiligree,
            accessory: AccessoryStyle::None, // No tiara spikes
            particle_type: ParticleType::SolarGlints,
            pulse_speed: 1.0,
            base_fullness: 1.10,
            taper_strength: 0.38,
            aura_tint: Rgba(255, 205, 50, 35),
            cradle_accent: Rgba(255, 225, 90, 255),
        },
        "meleys" => EggArchetype {
            scale_style: ScaleStyle::Pinecone, // Real overlapping archosaurian scales
            cradle_style: CradleStyle::SmeltingSlag,
            vein_style: VeinStyle::MagmaChasm,
            accessory: AccessoryStyle::None, // No tiara spikes
            particle_type: ParticleType::VolcanicEmbers,
            pulse_speed: 1.25,
            base_fullness: 1.09,
            taper_strength: 0.40,
            aura_tint: Rgba(245, 50, 30, 35),
            cradle_accent: Rgba(255, 120, 40, 255),
        },
        "sunfyre" => EggArchetype {
            scale_style: ScaleStyle::Diamond, // Replaced with reference diamond scales from caraxes_stage1.gif
            cradle_style: CradleStyle::GildedPedestal,
            vein_style: VeinStyle::GoldFiligree,
            accessory: AccessoryStyle::None, // No crystal spikes
            particle_type: ParticleType::SolarGlints,
            pulse_speed: 1.10,
            base_fullness: 1.11,
            taper_strength: 0.37,
            aura_tint: Rgba(255, 230, 90, 42),
            cradle_accent: Rgba(255, 240, 140, 255),
        },
        "vermithor" => EggArchetype {
            scale_style: ScaleStyle::Diamond, // Replaced with reference diamond scales from caraxes_stage1.gif
            cradle_style: CradleStyle::SmeltingSlag,
            vein_style: VeinStyle::BronzeVeins,
            accessory: AccessoryStyle::BronzeHoops, // Metallic hoops clasping body (no spikes)
            particle_type: ParticleType::ForgeSparks,
            pulse_speed: 0.72,
            base_fullness: 1.13,
            taper_strength: 0.36,
            aura_tint: Rgba(220, 140, 50, 32),
            cradle_accent: Rgba(240, 160, 60, 255),
        },
        "silverwing" => EggArchetype {
            scale_style: ScaleStyle::Diamond, // Replaced with reference diamond scales from caraxes_stage1.gif
            cradle_style: CradleStyle::IvoryBrazier,
            vein_style: VeinStyle::LunarAurora,
            accessory: AccessoryStyle::None, // No spikes or horns
            particle_type: ParticleType::SilverSparks,
            pulse_speed: 0.90,
            base_fullness: 1.10,
            taper_strength: 0.38,
            aura_tint: Rgba(220, 230, 245, 30),
            cradle_accent: Rgba(235, 240, 255, 255),
        },
        "seasmoke" => EggArchetype {
            scale_style: ScaleStyle::Diamond, // Replaced with reference diamond scales from caraxes_stage1.gif
            cradle_style: CradleStyle::ReefDriftwood,
            vein_style: VeinStyle::TidalVortex,
            accessory: AccessoryStyle::CoralSpurs, // Coral spurs on base reef (no spikes on egg)
            particle_type: ParticleType::SeaSpray,
            pulse_speed: 0.95,
            base_fullness: 1.08,
            taper_strength: 0.39,
            aura_tint: Rgba(100, 210, 220, 32),
            cradle_accent: Rgba(130, 235, 240, 255),
        },
        "sheepstealer" => EggArchetype {
            scale_style: ScaleStyle::Diamond, // Replaced with reference diamond scales from caraxes_stage1.gif
            cradle_style: CradleStyle::MountainScree,
            vein_style: VeinStyle::BronzeVeins,
            accessory: AccessoryStyle::HeatherTwigs, // Dry twigs around base nest (no spikes on egg)
            particle_type: ParticleType::MountainDust,
            pulse_speed: 0.88,
            base_fullness: 1.07,
            taper_strength: 0.40,
            aura_tint: Rgba(180, 120, 60, 25),
            cradle_accent: Rgba(200, 140, 70, 255),
        },
        "balerion" => EggArchetype {
            scale_style: ScaleStyle::Pinecone, // Real overlapping archosaurian scales
            cradle_style: CradleStyle::BasaltHearth,
            vein_style: VeinStyle::MagmaChasm,
            accessory: AccessoryStyle::None, // No horns or spikes
            particle_type: ParticleType::BlackAsh,
            pulse_speed: 0.60,
            base_fullness: 1.15,
            taper_strength: 0.38,
            aura_tint: Rgba(220, 40, 20, 36),
            cradle_accent: Rgba(240, 70, 25, 255),
        },
        "moondancer" => EggArchetype {
            scale_style: ScaleStyle::Diamond, // Replaced with reference diamond scales from caraxes_stage1.gif
            cradle_style: CradleStyle::AmethystGeode,
            vein_style: VeinStyle::AstralSpiral,
            accessory: AccessoryStyle::None, // No mohawk quills
            particle_type: ParticleType::LilacMotes,
            pulse_speed: 1.40,
            base_fullness: 1.08,
            taper_strength: 0.41,
            aura_tint: Rgba(170, 140, 240, 35),
            cradle_accent: Rgba(195, 160, 255, 255),
        },
        "tessarion" => EggArchetype {
            scale_style: ScaleStyle::Diamond, // Replaced with reference diamond scales from caraxes_stage1.gif
            cradle_style: CradleStyle::AmethystGeode,
            vein_style: VeinStyle::ElectricArcs,
            accessory: AccessoryStyle::None, // No spikes or horns
            particle_type: ParticleType::ElectricSparks,
            pulse_speed: 1.15,
            base_fullness: 1.09,
            taper_strength: 0.39,
            aura_tint: Rgba(60, 130, 255, 38),
            cradle_accent: Rgba(90, 175, 255, 255),
        },
        _ => EggArchetype {
            scale_style: ScaleStyle::Diamond,
            cradle_style: CradleStyle::BasaltHearth,
            vein_style: VeinStyle::MagmaChasm,
            accessory: AccessoryStyle::None,
            particle_type: ParticleType::VolcanicEmbers,
            pulse_speed: 1.0,
            base_fullness: 1.10,
            taper_strength: 0.40,
            aura_tint: Rgba(pal.fire_mid.0, pal.fire_mid.1, pal.fire_mid.2, 35),
            cradle_accent: pal.fire_inner,
        },
    }
}

/// Renders a single photorealistic, NORMAL UPRIGHT (pointed apex up, broad rounded base down) dragon egg
/// with species-specific morphology derived from the Git-Wyvern Encyclopedia, environmental cradle, and particle FX.
pub fn render_egg_frame(
    pal: &DragonPalette,
    frame_idx: usize,
    total_frames: usize,
    canvas: &mut Canvas,
) {
    let phase = frame_idx as f32 / total_frames as f32;
    let t = phase * 2.0 * PI;
    let arch = get_egg_archetype(pal.key, pal);

    // Anatomical Egg Coordinates:
    // Normal upright orientation:
    // Pointed conical apex points UPWARD (y < cy)
    // Broad, heavy rounded hemisphere dome rests solidly on DOWNWARD base (y > cy)
    let cx = 128.0_f32;
    let cy = 120.0_f32;
    let rx = 36.0_f32;
    let ry = 48.0_f32;

    // Biological heartbeat pulse: "lub-dub" double harmonic contraction & expansion
    let pulse_arg = t * arch.pulse_speed;
    let lub = (pulse_arg.sin().max(0.0)).powi(3);
    let dub = 0.35 * ((pulse_arg + 0.6).sin().max(0.0)).powi(2);
    let pulse = (lub + dub).clamp(0.0, 1.0);

    // -------------------------------------------------------------------------
    // 1. Ambient Atmospheric Aura & Ground Hearth Radial Glow
    // -------------------------------------------------------------------------
    let aura_rad = 75.0 + pulse * 12.0;
    let aura_min_x = ((cx - aura_rad).floor() as i32).max(0);
    let aura_max_x = ((cx + aura_rad).ceil() as i32).min(canvas.width as i32 - 1);
    let aura_min_y = ((cy - aura_rad).floor() as i32).max(0);
    let aura_max_y = ((cy + aura_rad).ceil() as i32).min(canvas.height as i32 - 1);

    for ay in aura_min_y..=aura_max_y {
        let dy = ay as f32 - cy;
        for ax in aura_min_x..=aura_max_x {
            let dx = ax as f32 - cx;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < aura_rad {
                let falloff = (1.0 - dist / aura_rad).powi(2);
                let alpha = (arch.aura_tint.3 as f32 * falloff * (0.6 + 0.4 * pulse)) as u8;
                if alpha > 2 {
                    canvas.set_pixel(ax, ay, Rgba(arch.aura_tint.0, arch.aura_tint.1, arch.aura_tint.2, alpha));
                }
            }
        }
    }

    // -------------------------------------------------------------------------
    // 2. Cradle / Nest / Hearth (Rear Foundation Layer)
    // -------------------------------------------------------------------------
    render_cradle_rear(arch.cradle_style, cx, cy, ry, canvas, pal, &arch, pulse);

    // -------------------------------------------------------------------------
    // 3. Sturdy Contact Drop Shadow Beneath Broad Rounded Base
    // -------------------------------------------------------------------------
    let sh_cx = (cx + 5.0).round() as i32;
    let sh_cy = (cy + ry * 0.94).round() as i32;
    // Outer diffused penumbra
    canvas.fill_ellipse(sh_cx, sh_cy, 44, 13, Rgba(12, 14, 18, 115));
    // Inner high-density umbra
    canvas.fill_ellipse(sh_cx, sh_cy, 28, 8, Rgba(8, 9, 12, 190));

    // -------------------------------------------------------------------------
    // 4. Volumetric 3D Normal Mapping & Shading: NORMAL UPRIGHT EGG PROFILE
    // -------------------------------------------------------------------------
    let min_y = ((cy - ry).floor() as i32).max(0);
    let max_y = ((cy + ry).ceil() as i32).min(canvas.height as i32 - 1);
    let min_x = ((cx - rx * 1.35).floor() as i32).max(0);
    let max_x = ((cx + rx * 1.35).ceil() as i32).min(canvas.width as i32 - 1);

    // Sun light vector from top-left (L = [-0.55, -0.65, 0.52])
    let lx = -0.55_f32;
    let ly = -0.65_f32;
    let lz = 0.52_f32;
    let l_len = (lx * lx + ly * ly + lz * lz).sqrt();
    let (lx, ly, lz) = (lx / l_len, ly / l_len, lz / l_len);

    for yi in min_y..=max_y {
        let y_rel = (yi as f32 - cy) / ry;
        if y_rel < -1.0 || y_rel > 1.0 {
            continue;
        }

        // --- NORMAL UPRIGHT PROFILE GEOMETRY ---
        // Top (y_rel < 0): Pointed conical apex pointing upward (taper reduces width)
        // Bottom (y_rel > 0): Broad, heavy rounded hemisphere dome resting in cradle (base_fullness expands width)
        let base_circ = (1.0 - y_rel * y_rel).max(0.0).sqrt();
        let half_w = if y_rel < 0.0 {
            rx * base_circ * (1.0 + y_rel * arch.taper_strength)
        } else {
            rx * base_circ * (1.0 + y_rel * (arch.base_fullness - 1.0))
        };

        if half_w <= 0.4 {
            continue;
        }

        for xi in min_x..=max_x {
            let x_rel = (xi as f32 - cx) / half_w;
            if x_rel.abs() > 1.0 {
                continue;
            }

            // 3D Spherical Surface Normal
            let z_surf = (1.0 - x_rel * x_rel).max(0.0).sqrt() * (1.0 - y_rel * y_rel).max(0.0).sqrt();
            let nx = x_rel;
            let ny = y_rel * 0.82;
            let nz = z_surf;
            let n_len = (nx * nx + ny * ny + nz * nz).sqrt().max(0.001);
            let (nx, ny, nz) = (nx / n_len, ny / n_len, nz / n_len);

            // Diffuse Lambertian N . L
            let ndotl = (nx * lx + ny * ly + nz * lz).clamp(0.0, 1.0);

            // Specular Blinn-Phong: Half-vector H between L and V=(0,0,1)
            let hx = lx;
            let hy = ly;
            let hz = lz + 1.0;
            let h_len = (hx * hx + hy * hy + hz * hz).sqrt();
            let (hx, hy, hz) = (hx / h_len, hy / h_len, hz / h_len);
            let ndoth = (nx * hx + ny * hy + nz * hz).clamp(0.0, 1.0);
            let spec = ndoth.powi(16);

            // Fresnel rim reflection around the silhouette
            let fresnel = (1.0 - nz).powi(3);

            // -----------------------------------------------------------------
            // Dynamic Scale Morphology & Texture Pattern
            // -----------------------------------------------------------------
            let (scale_bevel, scale_ridge, _scale_core) = evaluate_scale_pattern(
                arch.scale_style,
                xi as f32 - cx,
                yi as f32 - (cy - ry),
                half_w,
                y_rel,
            );

            // -----------------------------------------------------------------
            // Embryonic Vein & Fissure System
            // -----------------------------------------------------------------
            let (is_vein, vein_intensity) = evaluate_vein_pattern(
                arch.vein_style,
                xi,
                yi,
                x_rel,
                y_rel,
                nz,
                pulse,
            );

            // -----------------------------------------------------------------
            // Chromatic Shading Composition
            // -----------------------------------------------------------------
            let pixel_color = if is_vein {
                // Living embryonic transillumination
                let vein_t = (vein_intensity * (0.65 + 0.35 * pulse)).clamp(0.0, 1.0);
                if vein_t > 0.75 {
                    pal.fire_inner.lerp(pal.fire_core, (vein_t - 0.75) * 4.0)
                } else if vein_t > 0.35 {
                    pal.fire_mid.lerp(pal.fire_inner, (vein_t - 0.35) * 2.5)
                } else {
                    pal.fire_outer.lerp(pal.fire_mid, vein_t * 2.8)
                }
            } else if scale_bevel {
                // Dark crevice / grout / scale shadow groove
                pal.scale_deep
            } else if spec > 0.55 && ndotl > 0.40 {
                // High-gloss specular highlight hot-spot
                pal.scale_specular
            } else if scale_ridge && ndotl > 0.35 {
                // Raised scale ridge / facet crest
                pal.scale_hi
            } else if fresnel > 0.65 && y_rel > -0.6 {
                // Outer fresnel rim lighting
                pal.scale_hi.lerp(pal.scale_mid, 0.4)
            } else if ndotl > 0.62 {
                if (xi + yi) % 4 == 0 { pal.scale_hi } else { pal.scale_mid }
            } else if ndotl > 0.32 {
                pal.scale_mid
            } else if ndotl > 0.16 {
                pal.scale_dark
            } else {
                pal.scale_deep
            };

            canvas.set_pixel(xi, yi, pixel_color);
        }
    }

    // -------------------------------------------------------------------------
    // 5. Unique Upright Shell Accessories (Horns, Crystals, Hoops, Moss, Tiara)
    // -------------------------------------------------------------------------
    render_accessories(arch.accessory, cx, cy, rx, ry, canvas, pal, &arch);

    // -------------------------------------------------------------------------
    // 6. Cradle / Nest / Hearth (Front Foreground Layer - Cups the Broad Base)
    // -------------------------------------------------------------------------
    render_cradle_front(arch.cradle_style, cx, cy, ry, canvas, pal, &arch, pulse);

    // -------------------------------------------------------------------------
    // 7. Dynamic Atmospheric Particle FX (Embers, Frost Mist, Sulfur Smoke, Glints)
    // -------------------------------------------------------------------------
    render_particles(arch.particle_type, cx, cy, rx, ry, canvas, pal, &arch, t);
}

/// Evaluates scale patterns: returns (is_bevel_groove, is_ridge_highlight, is_core)
fn evaluate_scale_pattern(
    style: ScaleStyle,
    x_offset: f32,
    y_from_top: f32,
    _half_w: f32,
    y_rel: f32,
) -> (bool, bool, bool) {
    match style {
        ScaleStyle::Pinecone => {
            // Overlapping upward-pointing shingles with raised central vertical keels
            let sh_h = 5.2_f32;
            let sh_w = 6.2_f32;
            let row = (y_from_top / sh_h).floor() as i32;
            let row_off = if row % 2 == 0 { 0.0 } else { sh_w * 0.5 };
            let u = (((x_offset + row_off) % sh_w + sh_w) % sh_w) / sh_w - 0.5;
            let v = ((y_from_top % sh_h + sh_h) % sh_h) / sh_h - 0.5;
            let dist = (u * u * 1.4 + v * v).sqrt();
            let is_groove = dist > 0.52 || v < -0.42;
            let is_ridge = u.abs() < 0.12 && v > -0.25;
            (is_groove, is_ridge, dist < 0.3)
        }
        ScaleStyle::Diamond => {
            // Rhomboid diamond serpentine scales
            let d_size = 5.6_f32;
            let u_diag = (x_offset * 0.707 + y_from_top * 0.707) / d_size;
            let v_diag = (-x_offset * 0.707 + y_from_top * 0.707) / d_size;
            let u_frac = (u_diag - u_diag.floor() - 0.5).abs();
            let v_frac = (v_diag - v_diag.floor() - 0.5).abs();
            let is_groove = u_frac > 0.42 || v_frac > 0.42;
            let is_ridge = (u_frac - v_frac).abs() < 0.08 && u_frac < 0.32;
            (is_groove, is_ridge, u_frac < 0.25 && v_frac < 0.25)
        }
        ScaleStyle::Hexagon => {
            // Polished interlocking honeycomb plates
            let hex_r = 4.8_f32;
            let hex_h = hex_r * 1.732;
            let row = (y_from_top / (hex_h * 0.75)).floor() as i32;
            let col_off = if row % 2 == 0 { 0.0 } else { hex_r * 1.5 };
            let u = (((x_offset + col_off) % (hex_r * 3.0) + hex_r * 3.0) % (hex_r * 3.0)) - hex_r * 1.5;
            let v = (((y_from_top) % hex_h + hex_h) % hex_h) - hex_h * 0.5;
            let q2x = u.abs();
            let q2y = v.abs();
            let is_groove = (q2x * 0.5 + q2y * 0.866) > (hex_r * 0.88) || q2x > (hex_r * 0.92);
            let is_ridge = (q2x * 0.5 + q2y * 0.866) < (hex_r * 0.45);
            (is_groove, is_ridge, !is_groove && !is_ridge)
        }
        ScaleStyle::HeavyScutes => {
            // Thick irregular cobblestone armor scutes with weathered erosion cracks
            let sc_h = 7.0_f32;
            let sc_w = 8.5_f32;
            let jitter = ((x_offset * 1.7 + y_from_top * 2.3).sin() * 1.2).round();
            let row = ((y_from_top + jitter) / sc_h).floor() as i32;
            let col_off = if row % 2 == 0 { 0.0 } else { sc_w * 0.5 };
            let u = (((x_offset + col_off + jitter) % sc_w + sc_w) % sc_w) / sc_w - 0.5;
            let v = ((y_from_top % sc_h + sc_h) % sc_h) / sc_h - 0.5;
            let dist = (u * u + v * v).sqrt();
            let is_groove = dist > 0.48;
            let is_ridge = dist < 0.26 && u < 0.05 && v < 0.05;
            (is_groove, is_ridge, dist < 0.35)
        }
        ScaleStyle::Pearlescent => {
            // High-gloss opaline mother-of-pearl tiles with smooth luster bands
            let band = ((y_from_top * 0.35 + x_offset * 0.22).sin() * 0.5 + 0.5).powi(2);
            let scallop_w = 7.5_f32;
            let u = (((x_offset) % scallop_w + scallop_w) % scallop_w) / scallop_w - 0.5;
            let is_groove = u.abs() > 0.46;
            let is_ridge = band > 0.72 && u.abs() < 0.28;
            (is_groove, is_ridge, band > 0.4)
        }
        ScaleStyle::Crystalline => {
            // Angular fractured ice facets with sharp needle highlights
            let facet_u = (x_offset * 0.28 + y_from_top * 0.15).sin() * 0.5 + 0.5;
            let facet_v = (-x_offset * 0.18 + y_from_top * 0.32).cos() * 0.5 + 0.5;
            let is_groove = facet_u < 0.09 || facet_v < 0.09;
            let is_ridge = (facet_u - facet_v).abs() < 0.06 && facet_u > 0.55;
            (is_groove, is_ridge, facet_u > 0.7)
        }
        ScaleStyle::EarthyMottled => {
            // Granular sandstone with natural mineral stipples and weathered pores
            let stipple = (((x_offset * 11.3 + y_from_top * 17.7).sin() * 43758.54).fract()).abs();
            let wave = (y_from_top * 0.25 + (x_offset * 0.2).sin() * 1.5).sin() * 0.5 + 0.5;
            let is_groove = wave < 0.14 || (stipple > 0.94 && y_rel < -0.2);
            let is_ridge = wave > 0.78 && stipple < 0.65;
            (is_groove, is_ridge, wave > 0.45)
        }
    }
}

/// Evaluates embryonic fissures: returns (is_fissure, intensity_0_to_1)
fn evaluate_vein_pattern(
    style: VeinStyle,
    x: i32,
    y: i32,
    x_rel: f32,
    y_rel: f32,
    nz: f32,
    pulse: f32,
) -> (bool, f32) {
    if nz < 0.28 {
        return (false, 0.0);
    }

    match style {
        VeinStyle::MagmaChasm => {
            // Jagged vertical tectonic rifts branching from apex to base
            let chasm1 = ((x * 5 + y * 9) % 31).abs();
            let chasm2 = (((x - 128) * 7 - y * 4) % 29).abs();
            let is_rift = (chasm1 == 0 && x_rel.abs() < 0.55) || (chasm2 == 0 && y_rel > -0.7 && y_rel < 0.4);
            if is_rift {
                let intensity = (0.55 + 0.45 * pulse + nz * 0.35).clamp(0.0, 1.0);
                (true, intensity)
            } else {
                (false, 0.0)
            }
        }
        VeinStyle::BioCapillaries => {
            // Interconnected capillary bio-luminescent web
            let cap = ((x * 11 + y * 13 + (x * y) % 7) % 37).abs();
            let is_cap = cap <= 1 && (x_rel.abs() < 0.65);
            if is_cap {
                let intensity = (0.45 + 0.55 * pulse).clamp(0.0, 1.0);
                (true, intensity)
            } else {
                (false, 0.0)
            }
        }
        VeinStyle::GoldFiligree => {
            // Spiraling gold-leaf veins tracing scale contours
            let filigree = ((x_rel * 4.0).sin() * 3.0 + y_rel * 5.0).sin() * 0.5 + 0.5;
            let is_f = filigree > 0.88 && nz > 0.45;
            if is_f {
                let intensity = (0.60 + 0.40 * pulse).clamp(0.0, 1.0);
                (true, intensity)
            } else {
                (false, 0.0)
            }
        }
        VeinStyle::FrostFracture => {
            // Sharp dendritic frost rifts pulsing cold pale cyan
            let fr1 = ((x * 9 - y * 13) % 27).abs();
            let fr2 = ((x * 13 + y * 7) % 33).abs();
            let is_fr = (fr1 == 0 || fr2 == 0) && nz > 0.35;
            if is_fr {
                let intensity = (0.50 + 0.50 * pulse).clamp(0.0, 1.0);
                (true, intensity)
            } else {
                (false, 0.0)
            }
        }
        VeinStyle::BronzeVeins => {
            // Metallic molten bronze fissures coursing through stone
            let bz = ((x * 7 + y * 11) % 35).abs();
            let is_bz = bz == 0 && nz > 0.4;
            if is_bz {
                let intensity = (0.50 + 0.45 * pulse).clamp(0.0, 1.0);
                (true, intensity)
            } else {
                (false, 0.0)
            }
        }
        VeinStyle::TidalVortex => {
            // Two-arm logarithmic oceanic spiral / tidal gyre (Seasmoke)
            let theta = y_rel.atan2(x_rel);
            let r = (x_rel * x_rel + y_rel * y_rel).sqrt();
            let spiral = (2.0 * theta + 6.8 * r - pulse * 2.0 * PI).sin() * 0.5 + 0.5;
            let is_vortex = spiral > 0.88 && nz > 0.38;
            if is_vortex {
                let intensity = (0.55 + 0.45 * spiral * (0.8 + 0.2 * (pulse * PI).sin())).clamp(0.0, 1.0);
                (true, intensity)
            } else {
                (false, 0.0)
            }
        }
        VeinStyle::LunarAurora => {
            // Sweeping diagonal celestial aurora veil at -48 degrees (Silverwing)
            let diag_phase = 3.2 * x_rel - 3.6 * y_rel + pulse * 2.0 * PI;
            let cross_wave = (2.2 * x_rel + 2.0 * y_rel).cos();
            let aurora = (diag_phase.sin() * 0.72 + cross_wave * 0.28) * 0.5 + 0.5;
            let is_aurora = aurora > 0.87 && nz > 0.40;
            if is_aurora {
                let intensity = (0.55 + 0.45 * aurora).clamp(0.0, 1.0);
                (true, intensity)
            } else {
                (false, 0.0)
            }
        }
        VeinStyle::AstralSpiral => {
            // Diagonal helical spiral ribbon + embryonic core bloom (Moondancer)
            let helix = (4.0 * x_rel + 4.2 * y_rel + pulse * 2.0 * PI).sin() * 0.5 + 0.5;
            let r = (x_rel * x_rel + y_rel * y_rel).sqrt();
            let core_bloom = (1.0 - (r / 0.85).min(1.0)).powi(2) * (pulse * 2.0 * PI).sin().abs();
            let is_ribbon = (helix > 0.89 && nz > 0.40) || (core_bloom > 0.65 && nz > 0.60);
            if is_ribbon {
                let intensity = if core_bloom > 0.65 {
                    (0.70 + 0.30 * core_bloom).clamp(0.0, 1.0)
                } else {
                    (0.55 + 0.45 * helix).clamp(0.0, 1.0)
                };
                (true, intensity)
            } else {
                (false, 0.0)
            }
        }
        VeinStyle::ElectricArcs => {
            // High-voltage diagonal diamond-seam lightning discharge (Tessarion)
            let u_diag = 5.2 * (x_rel + y_rel);
            let v_diag = 5.2 * (-x_rel + y_rel);
            let arc_phase = pulse * 2.0 * PI;
            let arc1 = (u_diag * 1.4 + arc_phase).sin();
            let arc2 = (v_diag * 1.4 - arc_phase).cos();
            let arc_val = (arc1 * arc2).abs();
            let spark = ((x * 19 ^ y * 23) % 29).abs() == 0;
            let is_arc = (arc_val > 0.78 || (spark && arc_val > 0.45)) && nz > 0.35;
            if is_arc {
                let intensity = (0.60 + 0.40 * arc_val).clamp(0.0, 1.0);
                (true, intensity)
            } else {
                (false, 0.0)
            }
        }
    }
}

/// Renders rear layers of the cradle that sit behind the egg base
fn render_cradle_rear(
    style: CradleStyle,
    cx: f32,
    cy: f32,
    ry: f32,
    canvas: &mut Canvas,
    pal: &DragonPalette,
    arch: &EggArchetype,
    pulse: f32,
) {
    let base_y = cy + ry * 0.74;

    match style {
        CradleStyle::BasaltHearth => {
            // Jagged obsidian basalt boulders behind egg base
            let back_rock = [
                ((cx - 56.0).round() as i32, (base_y + 14.0).round() as i32),
                ((cx - 30.0).round() as i32, (base_y - 10.0).round() as i32),
                ((cx + 32.0).round() as i32, (base_y - 8.0).round() as i32),
                ((cx + 58.0).round() as i32, (base_y + 16.0).round() as i32),
                ((cx + 20.0).round() as i32, (base_y + 30.0).round() as i32),
                ((cx - 24.0).round() as i32, (base_y + 28.0).round() as i32),
            ];
            canvas.fill_polygon(&back_rock, Rgba::rgb(20, 22, 26));
            // Back magma glow line
            let glow_col = Rgba(pal.fire_mid.0, pal.fire_mid.1, pal.fire_mid.2, (120.0 + pulse * 90.0) as u8);
            canvas.draw_line((cx - 38.0).round() as i32, (base_y + 4.0).round() as i32, (cx + 40.0).round() as i32, (base_y + 5.0).round() as i32, glow_col, 2);
        }
        CradleStyle::GlacialShelf => {
            // Translucent cyan ice shelf backdrop
            let ice_pts = [
                ((cx - 52.0).round() as i32, (base_y + 12.0).round() as i32),
                ((cx - 26.0).round() as i32, (base_y - 12.0).round() as i32),
                ((cx + 28.0).round() as i32, (base_y - 10.0).round() as i32),
                ((cx + 54.0).round() as i32, (base_y + 14.0).round() as i32),
                ((cx).round() as i32, (base_y + 26.0).round() as i32),
            ];
            canvas.fill_polygon(&ice_pts, Rgba::rgb(35, 55, 80));
            // Ice crystal spines jutting upward behind base
            canvas.draw_line((cx - 32.0).round() as i32, (base_y - 4.0).round() as i32, (cx - 44.0).round() as i32, (base_y - 24.0).round() as i32, arch.cradle_accent, 2);
            canvas.draw_line((cx + 34.0).round() as i32, (base_y - 3.0).round() as i32, (cx + 46.0).round() as i32, (base_y - 22.0).round() as i32, arch.cradle_accent, 2);
        }
        CradleStyle::IvoryBrazier => {
            // Rear rim of sculpted Targaryen royal brazier bowl
            let rim_pts = [
                ((cx - 50.0).round() as i32, (base_y + 4.0).round() as i32),
                ((cx - 26.0).round() as i32, (base_y - 8.0).round() as i32),
                ((cx + 26.0).round() as i32, (base_y - 8.0).round() as i32),
                ((cx + 50.0).round() as i32, (base_y + 4.0).round() as i32),
                ((cx).round() as i32, (base_y + 18.0).round() as i32),
            ];
            canvas.fill_polygon(&rim_pts, Rgba::rgb(140, 115, 65));
            // Glowing white-gold coals in the rear bowl
            canvas.fill_ellipse(cx.round() as i32, (base_y + 2.0).round() as i32, 36, 9, Rgba::rgb(255, 230, 150));
        }
        CradleStyle::GildedPedestal => {
            // Rear hexagonal pedestal tier
            let dais = [
                ((cx - 54.0).round() as i32, (base_y + 8.0).round() as i32),
                ((cx - 30.0).round() as i32, (base_y - 6.0).round() as i32),
                ((cx + 30.0).round() as i32, (base_y - 6.0).round() as i32),
                ((cx + 54.0).round() as i32, (base_y + 8.0).round() as i32),
                ((cx).round() as i32, (base_y + 24.0).round() as i32),
            ];
            canvas.fill_polygon(&dais, Rgba::rgb(115, 85, 45));
        }
        CradleStyle::MossyRuins => {
            // Ancient Valyrian carved masonry rear blocks
            let stone = [
                ((cx - 54.0).round() as i32, (base_y + 16.0).round() as i32),
                ((cx - 32.0).round() as i32, (base_y - 4.0).round() as i32),
                ((cx + 34.0).round() as i32, (base_y - 5.0).round() as i32),
                ((cx + 54.0).round() as i32, (base_y + 18.0).round() as i32),
                ((cx).round() as i32, (base_y + 30.0).round() as i32),
            ];
            canvas.fill_polygon(&stone, Rgba::rgb(42, 48, 44));
            // Creeping moss patches
            canvas.draw_line((cx - 38.0).round() as i32, (base_y + 2.0).round() as i32, (cx - 20.0).round() as i32, (base_y - 2.0).round() as i32, Rgba::rgb(35, 115, 55), 2);
        }
        CradleStyle::AmethystGeode => {
            // Rough rock rim of the hollow geode bowl
            let geode = [
                ((cx - 52.0).round() as i32, (base_y + 10.0).round() as i32),
                ((cx - 28.0).round() as i32, (base_y - 8.0).round() as i32),
                ((cx + 28.0).round() as i32, (base_y - 8.0).round() as i32),
                ((cx + 52.0).round() as i32, (base_y + 10.0).round() as i32),
                ((cx).round() as i32, (base_y + 22.0).round() as i32),
            ];
            canvas.fill_polygon(&geode, Rgba::rgb(45, 38, 52));
            // Purple quartz needles in rear hollow
            canvas.draw_line((cx - 28.0).round() as i32, (base_y - 2.0).round() as i32, (cx - 38.0).round() as i32, (base_y - 18.0).round() as i32, arch.cradle_accent, 2);
            canvas.draw_line((cx + 28.0).round() as i32, (base_y - 2.0).round() as i32, (cx + 40.0).round() as i32, (base_y - 20.0).round() as i32, arch.cradle_accent, 2);
        }
        _ => {
            // General dark crag rock
            let rock = [
                ((cx - 50.0).round() as i32, (base_y + 12.0).round() as i32),
                ((cx - 22.0).round() as i32, (base_y - 6.0).round() as i32),
                ((cx + 24.0).round() as i32, (base_y - 6.0).round() as i32),
                ((cx + 52.0).round() as i32, (base_y + 14.0).round() as i32),
                ((cx).round() as i32, (base_y + 26.0).round() as i32),
            ];
            canvas.fill_polygon(&rock, Rgba::rgb(35, 32, 30));
        }
    }
}

/// Renders foreground layers of the cradle that cup and securely hold the broad rounded base
fn render_cradle_front(
    style: CradleStyle,
    cx: f32,
    cy: f32,
    ry: f32,
    canvas: &mut Canvas,
    pal: &DragonPalette,
    arch: &EggArchetype,
    pulse: f32,
) {
    let base_y = cy + ry * 0.90;

    match style {
        CradleStyle::BasaltHearth => {
            // Jagged obsidian basalt rocks cupping the broad base
            let rock_left = [
                ((cx - 52.0).round() as i32, (base_y + 16.0).round() as i32),
                ((cx - 32.0).round() as i32, (base_y - 4.0).round() as i32),
                ((cx - 12.0).round() as i32, (base_y + 8.0).round() as i32),
                ((cx - 20.0).round() as i32, (base_y + 26.0).round() as i32),
            ];
            let rock_right = [
                ((cx + 12.0).round() as i32, (base_y + 6.0).round() as i32),
                ((cx + 36.0).round() as i32, (base_y - 6.0).round() as i32),
                ((cx + 54.0).round() as i32, (base_y + 18.0).round() as i32),
                ((cx + 24.0).round() as i32, (base_y + 28.0).round() as i32),
            ];
            canvas.fill_polygon(&rock_left, Rgba::rgb(28, 30, 35));
            canvas.fill_polygon(&rock_right, Rgba::rgb(24, 26, 30));
            // Rock edge highlights
            canvas.draw_line((cx - 32.0).round() as i32, (base_y - 4.0).round() as i32, (cx - 12.0).round() as i32, (base_y + 8.0).round() as i32, Rgba::rgb(55, 60, 70), 1);
            canvas.draw_line((cx + 12.0).round() as i32, (base_y + 6.0).round() as i32, (cx + 36.0).round() as i32, (base_y - 6.0).round() as i32, Rgba::rgb(60, 65, 75), 1);
            // Glowing bubbling magma channel beneath the base
            let magma_alpha = (190.0 + pulse * 65.0) as u8;
            let magma_glow = Rgba(pal.fire_mid.0, pal.fire_mid.1, pal.fire_mid.2, magma_alpha);
            canvas.draw_line((cx - 18.0).round() as i32, (base_y + 14.0).round() as i32, (cx + 18.0).round() as i32, (base_y + 14.0).round() as i32, magma_glow, 2);
            canvas.draw_line((cx - 8.0).round() as i32, (base_y + 14.0).round() as i32, (cx + 8.0).round() as i32, (base_y + 14.0).round() as i32, pal.fire_inner, 1);
        }
        CradleStyle::GlacialShelf => {
            // Front frozen icicles and blue ice spires hugging the base
            let front_ice = [
                ((cx - 48.0).round() as i32, (base_y + 14.0).round() as i32),
                ((cx - 20.0).round() as i32, (base_y + 2.0).round() as i32),
                ((cx + 22.0).round() as i32, (base_y + 3.0).round() as i32),
                ((cx + 50.0).round() as i32, (base_y + 16.0).round() as i32),
                ((cx).round() as i32, (base_y + 26.0).round() as i32),
            ];
            canvas.fill_polygon(&front_ice, Rgba::rgb(50, 80, 115));
            // Crystalline highlights
            canvas.draw_line((cx - 20.0).round() as i32, (base_y + 2.0).round() as i32, (cx + 22.0).round() as i32, (base_y + 3.0).round() as i32, arch.cradle_accent, 1);
            // Hanging icicles
            for i in -2_i32..=2_i32 {
                let ix = (cx + i as f32 * 14.0).round() as i32;
                let iy = (base_y + 18.0).round() as i32;
                let len = 6 + (i.abs() * 3);
                canvas.draw_line(ix, iy, ix, iy + len, arch.cradle_accent, 1);
            }
        }
        CradleStyle::IvoryBrazier => {
            // Front golden claw feet and ornate carved rim of royal brazier
            let front_rim = [
                ((cx - 46.0).round() as i32, (base_y + 8.0).round() as i32),
                ((cx - 22.0).round() as i32, (base_y + 14.0).round() as i32),
                ((cx + 22.0).round() as i32, (base_y + 14.0).round() as i32),
                ((cx + 46.0).round() as i32, (base_y + 8.0).round() as i32),
                ((cx + 30.0).round() as i32, (base_y + 24.0).round() as i32),
                ((cx - 30.0).round() as i32, (base_y + 24.0).round() as i32),
            ];
            canvas.fill_polygon(&front_rim, Rgba::rgb(215, 185, 110));
            // Gilded filigree border
            canvas.draw_line((cx - 44.0).round() as i32, (base_y + 9.0).round() as i32, (cx + 44.0).round() as i32, (base_y + 9.0).round() as i32, arch.cradle_accent, 2);
            // Draconic claw feet
            canvas.draw_line((cx - 32.0).round() as i32, (base_y + 22.0).round() as i32, (cx - 42.0).round() as i32, (base_y + 34.0).round() as i32, Rgba::rgb(170, 135, 60), 3);
            canvas.draw_line((cx + 32.0).round() as i32, (base_y + 22.0).round() as i32, (cx + 42.0).round() as i32, (base_y + 34.0).round() as i32, Rgba::rgb(170, 135, 60), 3);
        }
        CradleStyle::GildedPedestal => {
            // Front sandstone dais tier with etched sunburst rays
            let front_dais = [
                ((cx - 52.0).round() as i32, (base_y + 6.0).round() as i32),
                ((cx - 24.0).round() as i32, (base_y + 12.0).round() as i32),
                ((cx + 24.0).round() as i32, (base_y + 12.0).round() as i32),
                ((cx + 52.0).round() as i32, (base_y + 6.0).round() as i32),
                ((cx + 36.0).round() as i32, (base_y + 24.0).round() as i32),
                ((cx - 36.0).round() as i32, (base_y + 24.0).round() as i32),
            ];
            canvas.fill_polygon(&front_dais, Rgba::rgb(180, 140, 70));
            canvas.draw_line((cx - 50.0).round() as i32, (base_y + 7.0).round() as i32, (cx + 50.0).round() as i32, (base_y + 7.0).round() as i32, arch.cradle_accent, 2);
        }
        CradleStyle::MossyRuins => {
            // Front ancient stone blocks and climbing ivy roots
            let stone_front = [
                ((cx - 50.0).round() as i32, (base_y + 14.0).round() as i32),
                ((cx - 22.0).round() as i32, (base_y + 2.0).round() as i32),
                ((cx + 22.0).round() as i32, (base_y + 4.0).round() as i32),
                ((cx + 50.0).round() as i32, (base_y + 16.0).round() as i32),
                ((cx).round() as i32, (base_y + 28.0).round() as i32),
            ];
            canvas.fill_polygon(&stone_front, Rgba::rgb(55, 62, 58));
            canvas.draw_line((cx - 22.0).round() as i32, (base_y + 2.0).round() as i32, (cx + 22.0).round() as i32, (base_y + 4.0).round() as i32, Rgba::rgb(85, 95, 90), 1);
            // Thick moss roots wrapping around stone
            canvas.draw_line((cx - 28.0).round() as i32, (base_y + 8.0).round() as i32, (cx - 12.0).round() as i32, (base_y - 2.0).round() as i32, Rgba::rgb(45, 140, 65), 2);
            canvas.draw_line((cx + 12.0).round() as i32, (base_y - 1.0).round() as i32, (cx + 30.0).round() as i32, (base_y + 10.0).round() as i32, Rgba::rgb(45, 140, 65), 2);
        }
        CradleStyle::AmethystGeode => {
            // Front cluster of purple/blue crystal points framing the base
            for c in -3_i32..=3_i32 {
                let gx = (cx + c as f32 * 13.0).round() as i32;
                let gy = (base_y + 12.0 - (c.abs() as f32 * 2.5)).round() as i32;
                let h = 8 + (3 - c.abs()) * 3;
                let crystal_pts = [(gx - 3, gy + 8), (gx, gy - h), (gx + 3, gy + 8)];
                canvas.fill_polygon(&crystal_pts, arch.cradle_accent);
                canvas.draw_line(gx, gy - h, gx, gy + 8, Rgba::rgb(255, 255, 255), 1);
            }
        }
        CradleStyle::ReefDriftwood => {
            // Front weathered sea driftwood branches and coral spurs
            canvas.draw_line((cx - 46.0).round() as i32, (base_y + 16.0).round() as i32, (cx - 14.0).round() as i32, (base_y + 2.0).round() as i32, Rgba::rgb(110, 115, 120), 3);
            canvas.draw_line((cx + 14.0).round() as i32, (base_y + 3.0).round() as i32, (cx + 48.0).round() as i32, (base_y + 18.0).round() as i32, Rgba::rgb(105, 110, 115), 3);
            // Coral branches
            canvas.draw_line((cx - 18.0).round() as i32, (base_y + 4.0).round() as i32, (cx - 24.0).round() as i32, (base_y - 8.0).round() as i32, arch.cradle_accent, 2);
            canvas.draw_line((cx + 20.0).round() as i32, (base_y + 5.0).round() as i32, (cx + 28.0).round() as i32, (base_y - 6.0).round() as i32, arch.cradle_accent, 2);
        }
        _ => {
            // Default foreground crag stones
            let crag = [
                ((cx - 44.0).round() as i32, (base_y + 12.0).round() as i32),
                ((cx - 16.0).round() as i32, (base_y + 4.0).round() as i32),
                ((cx + 16.0).round() as i32, (base_y + 4.0).round() as i32),
                ((cx + 44.0).round() as i32, (base_y + 14.0).round() as i32),
                ((cx).round() as i32, (base_y + 24.0).round() as i32),
            ];
            canvas.fill_polygon(&crag, Rgba::rgb(42, 38, 36));
            canvas.draw_line((cx - 16.0).round() as i32, (base_y + 4.0).round() as i32, (cx + 16.0).round() as i32, (base_y + 4.0).round() as i32, arch.cradle_accent, 1);
        }
    }
}

/// Renders subtle base accessories (strictly NO spikes, NO horns)
fn render_accessories(
    acc: AccessoryStyle,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    canvas: &mut Canvas,
    pal: &DragonPalette,
    arch: &EggArchetype,
) {
    match acc {
        AccessoryStyle::BronzeHoops => {
            // Heavy riveted bronze reinforcing hoops (Vermithor)
            let hoop1_y = cy - ry * 0.10;
            let hoop2_y = cy + ry * 0.35;
            canvas.draw_line((cx - rx * 0.88).round() as i32, hoop1_y.round() as i32, (cx + rx * 0.88).round() as i32, hoop1_y.round() as i32, arch.cradle_accent, 2);
            canvas.draw_line((cx - rx * 0.96).round() as i32, hoop2_y.round() as i32, (cx + rx * 0.96).round() as i32, hoop2_y.round() as i32, arch.cradle_accent, 2);
            // Bronze rivets
            for r in -3_i32..=3_i32 {
                let rx_pos = (cx + r as f32 * 13.0).round() as i32;
                canvas.set_pixel(rx_pos, hoop1_y.round() as i32, pal.scale_specular);
                canvas.set_pixel(rx_pos, hoop2_y.round() as i32, pal.scale_specular);
            }
        }
        AccessoryStyle::MossTendrils => {
            // Emerald moss tendrils creeping up lower shell base from ruin stone (Rhaegal)
            canvas.draw_line((cx - 22.0).round() as i32, (cy + ry * 0.80).round() as i32, (cx - 16.0).round() as i32, (cy + ry * 0.50).round() as i32, Rgba::rgb(45, 140, 65), 2);
            canvas.draw_line((cx + 20.0).round() as i32, (cy + ry * 0.82).round() as i32, (cx + 14.0).round() as i32, (cy + ry * 0.52).round() as i32, Rgba::rgb(45, 140, 65), 2);
        }
        AccessoryStyle::CoralSpurs => {
            // Calcified coral spurs clinging to lower shell base (Seasmoke)
            canvas.draw_line((cx - 24.0).round() as i32, (cy + ry * 0.75).round() as i32, (cx - 30.0).round() as i32, (cy + ry * 0.55).round() as i32, arch.cradle_accent, 2);
            canvas.draw_line((cx + 22.0).round() as i32, (cy + ry * 0.76).round() as i32, (cx + 28.0).round() as i32, (cy + ry * 0.58).round() as i32, arch.cradle_accent, 2);
        }
        AccessoryStyle::HeatherTwigs => {
            // Dry mountain twigs around lower base (Sheepstealer)
            canvas.draw_line((cx - 26.0).round() as i32, (cy + ry * 0.85).round() as i32, (cx - 34.0).round() as i32, (cy + ry * 0.70).round() as i32, arch.cradle_accent, 1);
            canvas.draw_line((cx + 24.0).round() as i32, (cy + ry * 0.86).round() as i32, (cx + 32.0).round() as i32, (cy + ry * 0.72).round() as i32, arch.cradle_accent, 1);
        }
        AccessoryStyle::None => {}
    }
}

/// Renders dynamic atmospheric particles rising around the upright egg
fn render_particles(
    p_type: ParticleType,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    canvas: &mut Canvas,
    pal: &DragonPalette,
    arch: &EggArchetype,
    t: f32,
) {
    let count = 14;

    for i in 0..count {
        let i_f = i as f32;
        let speed = 1.0 + (i % 3) as f32 * 0.5;
        let vert_travel = (t * speed * 3.5 + i_f * 2.3) % 1.0;
        let px = cx + ((i_f * 17.0 + t * 2.0).sin() * (rx * 1.35 + 8.0));
        let py = (cy + ry * 0.9) - vert_travel * (ry * 2.1 + 25.0);

        let (p_col, size) = match p_type {
            ParticleType::VolcanicEmbers | ParticleType::BlackAsh => {
                let col = if i % 2 == 0 { pal.fire_inner } else { pal.fire_mid };
                (col, if i % 4 == 0 { 2 } else { 1 })
            }
            ParticleType::FrostMist => {
                let col = if i % 2 == 0 { arch.cradle_accent } else { Rgba::rgb(220, 245, 255) };
                (col, 1)
            }
            ParticleType::SulfurVapors => {
                let col = if i % 2 == 0 { arch.cradle_accent } else { pal.fire_mid };
                (col, 2)
            }
            ParticleType::SolarGlints => {
                let col = if i % 2 == 0 { Rgba::rgb(255, 255, 255) } else { arch.cradle_accent };
                (col, if i % 3 == 0 { 2 } else { 1 })
            }
            ParticleType::EmeraldSparks => {
                let col = if i % 2 == 0 { Rgba::rgb(80, 245, 120) } else { Rgba::rgb(240, 230, 80) };
                (col, 1)
            }
            ParticleType::SeaSpray => {
                let col = if i % 2 == 0 { Rgba::rgb(140, 235, 245) } else { Rgba::rgb(255, 255, 255) };
                (col, 1)
            }
            ParticleType::LilacMotes => {
                let col = if i % 2 == 0 { arch.cradle_accent } else { Rgba::rgb(230, 210, 255) };
                (col, 1)
            }
            ParticleType::ElectricSparks => {
                let col = if i % 2 == 0 { arch.cradle_accent } else { Rgba::rgb(255, 255, 255) };
                (col, 2)
            }
            ParticleType::ForgeSparks => {
                let col = if i % 2 == 0 { pal.fire_inner } else { Rgba::rgb(255, 200, 100) };
                (col, 2)
            }
            _ => {
                let col = if i % 2 == 0 { arch.cradle_accent } else { pal.scale_specular };
                (col, 1)
            }
        };

        if py >= 10.0 && py < (canvas.height as f32 - 10.0) && px >= 10.0 && px < (canvas.width as f32 - 10.0) {
            if size == 1 {
                canvas.set_pixel(px.round() as i32, py.round() as i32, p_col);
            } else {
                canvas.fill_circle(px.round() as i32, py.round() as i32, size, p_col);
            }
        }
    }
}
