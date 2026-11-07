use crate::canvas::Canvas;
use crate::egg::render_egg_frame;
use crate::palette::{DragonPalette, Rgba};
use crate::wyvern::render_wyvern_frame;

pub struct StageInfo {
    pub id: u32,
    pub title: &'static str,
    pub icon: &'static str,
    pub power: u64,
    pub progress_pct: u32,
    pub next_threshold: Option<u64>,
    pub desc: &'static str,
}

pub fn get_stage_info(commits: u64) -> StageInfo {
    if commits < 30 {
        StageInfo {
            id: 1,
            title: "Stage 1: Dragon Egg",
            icon: "🥚",
            power: commits * 35,
            progress_pct: ((commits as f32 / 30.0) * 100.0).min(100.0) as u32,
            next_threshold: Some(30),
            desc: "Incubating in an elemental cradle. Petrified dragon egg rests on its broad base with pointed apex upward, pulsing with living embryonic heartbeat and species-specific veins.",
        }
    } else if commits < 100 {
        StageInfo {
            id: 2,
            title: "Stage 2: Hatchling Wyvern",
            icon: "🐣",
            power: 1050 + (commits - 30) * 55,
            progress_pct: (((commits - 30) as f32 / 70.0) * 100.0).min(100.0) as u32,
            next_threshold: Some(100),
            desc: "Freshly hatched young wyvern taking flight in agile aerial patrol, flapping wings and testing thermal breath.",
        }
    } else {
        StageInfo {
            id: 3,
            title: "Stage 3: Apex Soaring Dread",
            icon: "🐉",
            power: 4900 + (commits - 100) * 100,
            progress_pct: 100,
            next_threshold: None,
            desc: "Full colossal apex predator with unclipped wingspan, predatory fangs, quadrate jaw hinge, and soaring aerial hunt with roaring dragonfire.",
        }
    }
}

pub fn render_stage_frame(
    stage_id: u32,
    pal: &DragonPalette,
    frame_idx: usize,
    total_frames: usize,
    canvas: &mut Canvas,
) {
    let phase = frame_idx as f32 / total_frames as f32;

    match stage_id {
        1 => {
            render_egg_frame(pal, frame_idx, total_frames, canvas);
        }
        2 => {
            // Stage 2: Hatchling Wyvern perched on volcanic crag with eggshell shards
            let cx = 118;
            let cy = 136;

            // Jagged basalt crag base
            let crag = [(cx - 44, cy + 28), (cx - 18, cy + 14), (cx + 22, cy + 16), (cx + 46, cy + 30)];
            canvas.fill_polygon(&crag, Rgba::rgb(35, 38, 44));
            canvas.draw_line(cx - 18, cy + 14, cx + 22, cy + 16, Rgba::rgb(70, 75, 85), 2);

            // Broken eggshell fragments around the nest
            let shell1 = [(cx - 32, cy + 18), (cx - 18, cy + 12), (cx - 12, cy + 22), (cx - 28, cy + 26)];
            let shell2 = [(cx + 14, cy + 13), (cx + 30, cy + 19), (cx + 22, cy + 24), (cx + 10, cy + 22)];
            canvas.fill_polygon(&shell1, pal.scale_dark);
            canvas.fill_polygon(&shell2, pal.scale_dark);

            // Hatchling mouth opening with sparks/smoke
            let mouth = if (0.35..0.65).contains(&phase) { 4.0 } else { 0.0 };
            render_wyvern_frame(pal, phase, 0.56, mouth, canvas, cx, cy);
        }
        _ => {
            // Stage 3: Apex Soaring Dread (Full Colossal Adult Wyvern)
            let mouth = if (0.28..0.65).contains(&phase) { 6.0 } else { 0.0 };
            let (sc, cx) = if pal.is_serpentine { (1.02 * 0.92, 110) } else { (1.02, 118) };
            render_wyvern_frame(pal, phase, sc, mouth, canvas, cx, 134);
        }
    }
}
