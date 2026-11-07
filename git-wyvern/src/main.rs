use git_wyvern::canvas::Canvas;
use git_wyvern::encoder::save_animated_gif;
use git_wyvern::palette::{get_palette, ALL_DRAGONS};
use git_wyvern::patrol::{render_patrol_frame, Ember};
use git_wyvern::stages::{get_stage_info, render_stage_frame};

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn get_git_commit_count() -> u64 {
    if let Ok(output) = Command::new("git").args(["rev-list", "--count", "HEAD"]).output() {
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Ok(count) = s.parse::<u64>() {
                return count;
            }
        }
    }
    0
}

fn make_progress_bar(pct: u32, length: usize) -> String {
    let filled = ((length as f32 * (pct as f32 / 100.0)).round() as usize).min(length);
    let mut bar = String::new();
    for _ in 0..filled {
        bar.push('█');
    }
    for _ in 0..(length - filled) {
        bar.push('░');
    }
    bar
}

fn render_patrol(dragon_key: &str, stage_id: u32, out_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let pal = get_palette(dragon_key).expect("Palette found");
    let total_frames = 144; // 144 extended cinematic frames for empty-sky dragonfire and procedural prey hunt
    let mut frames = Vec::with_capacity(total_frames);
    let mut embers: Vec<Ember> = Vec::new();

    let scale = if stage_id < 3 { 0.85 } else { 1.15 };

    for f in 0..total_frames {
        let mut canvas_640 = Canvas::new(640, 640);
        render_patrol_frame(&pal, f, total_frames, scale, &mut canvas_640, &mut embers);
        frames.push(canvas_640);
    }

    save_animated_gif(&frames, 5, out_path)?;
    Ok(())
}

fn render_stage(dragon_key: &str, stage_id: u32, out_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let pal = get_palette(dragon_key).expect("Palette found");
    let total_frames = 16;
    let mut frames_upscaled = Vec::with_capacity(total_frames);

    for f in 0..total_frames {
        let mut canvas_256 = Canvas::new(256, 256);
        render_stage_frame(stage_id, &pal, f, total_frames, &mut canvas_256);
        let canvas_512 = canvas_256.upscale_2x();
        frames_upscaled.push(canvas_512);
    }

    save_animated_gif(&frames_upscaled, 7, out_path)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    // Auto-detect whether invoked from repository root or inside git-wyvern/
    let dragons_dir = if Path::new("dragons").is_dir() || Path::new("README.md").is_file() {
        PathBuf::from("dragons")
    } else if Path::new("../dragons").is_dir() || Path::new("../README.md").is_file() {
        PathBuf::from("../dragons")
    } else {
        PathBuf::from("dragons")
    };
    fs::create_dir_all(&dragons_dir)?;

    let readme_path = if Path::new("README.md").is_file() {
        PathBuf::from("README.md")
    } else if Path::new("../README.md").is_file() {
        PathBuf::from("../README.md")
    } else {
        PathBuf::from("README.md")
    };

    // Handle --batch-all flag
    if args.iter().any(|a| a == "--batch-all") {
        println!("🐉 Git-Wyvern Batch Generation (15 Canon Dragons, Aerial Patrol)...");
        for &d in ALL_DRAGONS {
            let pal = get_palette(d).unwrap();
            println!("   -> Compiling {} ({})", pal.name, pal.title);
            let patrol_path = dragons_dir.join(format!("{}_patrol.gif", d));
            render_patrol(d, 3, &patrol_path)?;
            let stage2_patrol_path = dragons_dir.join(format!("{}_stage2_patrol.gif", d));
            render_patrol(d, 2, &stage2_patrol_path)?;
            let stage1_path = dragons_dir.join(format!("{}_stage1.gif", d));
            render_stage(d, 1, &stage1_path)?;

            // Overwrite any legacy stage2/stage3/stage files with aerial patrol
            let stage2_dest = dragons_dir.join(format!("{}_stage2.gif", d));
            fs::copy(&stage2_patrol_path, &stage2_dest)?;
            let stage3_dest = dragons_dir.join(format!("{}_stage3.gif", d));
            fs::copy(&patrol_path, &stage3_dest)?;
            let stage_dest = dragons_dir.join(format!("{}_stage.gif", d));
            fs::copy(&patrol_path, &stage_dest)?;
        }
        println!("✨ Successfully compiled all 15 canon dragon aerial patrol assets in pure Rust!");
        return Ok(());
    }

    // 1. Dragon selection
    let mut dragon_key = env::var("WYVERN_DRAGON").unwrap_or_else(|_| "drogon".to_string());
    if args.len() > 1 && !args[1].starts_with('-') {
        dragon_key = args[1].clone();
    }

    // 2. Commit count
    let mut commits = if let Ok(c_str) = env::var("WYVERN_COMMITS") {
        c_str.parse::<u64>().unwrap_or_else(|_| get_git_commit_count())
    } else {
        get_git_commit_count()
    };
    if args.len() > 2 && !args[2].starts_with('-') {
        if let Ok(c) = args[2].parse::<u64>() {
            if c > 0 {
                commits = c;
            }
        }
    }

    // 3. Flight mode: All wyverns operate exclusively in 640x640 Aerial Patrol mode!

    let pal = get_palette(&dragon_key).expect("Palette found");
    let stage = get_stage_info(commits);

    println!("🐉 Git-Wyvern (Rust Edition) v0.1.0");
    println!("   Companion : {} ({})", pal.name, pal.title);
    println!("   Commits   : {} (Stage: {})", commits, stage.title);
    println!("   Mode      : Aerial Patrol");

    let is_patrol = stage.id >= 2;
    let companion_path = dragons_dir.join("companion.gif");

    if is_patrol {
        println!(" -> Rendering 144-Frame Silky Aerial Hunt & Patrol in pure Rust (320x320 upscaled to 640x640)...");
        render_patrol(&dragon_key, stage.id, &companion_path)?;
        let patrol_dest = if stage.id == 2 {
            dragons_dir.join(format!("{}_stage2_patrol.gif", pal.key))
        } else {
            dragons_dir.join(format!("{}_patrol.gif", pal.key))
        };
        fs::copy(&companion_path, &patrol_dest)?;
        let stage_dest = dragons_dir.join(format!("{}_stage{}.gif", pal.key, stage.id));
        fs::copy(&companion_path, &stage_dest)?;
        println!("    Saved companion patrol to: {}", companion_path.display());
    } else {
        println!(" -> Rendering Stage 1 Dragon Egg on Volcanic Hearth in pure Rust...");
        render_stage(&dragon_key, 1, &companion_path)?;
        let stage_dest = dragons_dir.join(format!("{}_stage1.gif", pal.key));
        fs::copy(&companion_path, &stage_dest)?;
        println!("    Saved companion egg to: {}", companion_path.display());
    }

    // Update README.md
    if readme_path.exists() {
        let content = fs::read_to_string(&readme_path)?;
        let bar = make_progress_bar(stage.progress_pct, 16);
        let next_info = if let Some(_next) = stage.next_threshold {
            format!("({}% to Stage {})", stage.progress_pct, stage.id + 1)
        } else {
            "(Max Apex Reached!)".to_string()
        };

        let flight_title = if is_patrol {
            format!("{} (Aerial Patrol)", stage.title)
        } else {
            stage.title.to_string()
        };

        let wyvern_block = format!(
            r#"<!-- START_SECTION:wyvern -->
<div align="center">

### {} Active Companion: **{}** — *{}*

<a href="https://github.com/topics/game-of-thrones">
  <img src="dragons/companion.gif" alt="{} Wyvern" width="{}"/>
</a>

```text
┌────────────────────────────────────────────────────────────────────────┐
│  🐲 Companion   : {:<14} Series  : {:<26}│
│  👑 Bonded Rider: {:<14} Faction : {:<26}│
│  🔥 Breath Type : {:<48}│
│  ✨ Growth Stage: {:<48}│
│  ⚡ Power Level : {:<5} XP        Commits : {:<5} ({:>3}% to Next Level) │
│  📈 Growth Bar  : [{}] {:<27}│
└────────────────────────────────────────────────────────────────────────┘
```
> *"{}"*

</div>
<!-- END_SECTION:wyvern -->"#,
            pal.faction_icon,
            pal.name,
            pal.title,
            pal.name,
            if is_patrol { "420" } else { "352" },
            pal.name,
            pal.series,
            pal.rider,
            pal.faction,
            pal.breath_name,
            flight_title,
            stage.power,
            commits,
            stage.progress_pct,
            bar,
            next_info,
            pal.desc
        );

        let new_content = if content.contains("<!-- START_SECTION:wyvern -->") && content.contains("<!-- END_SECTION:wyvern -->") {
            let start = content.find("<!-- START_SECTION:wyvern -->").unwrap();
            let end_tag = "<!-- END_SECTION:wyvern -->";
            let end = content.find(end_tag).unwrap() + end_tag.len();
            format!("{}{}{}", &content[..start], wyvern_block, &content[end..])
        } else {
            format!("{}\n\n{}", wyvern_block, content)
        };

        fs::write(readme_path, new_content)?;
        println!("    Updated README.md successfully!");
    }

    Ok(())
}
