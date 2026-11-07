"""
Build Aerial Patrol Flight Mode (512x512) for Game of Thrones & House of the Dragon Wyverns.
Inspired by git-koi's spatial 512x512 trajectory, this renders a majestic wyvern patrolling
a continuous 3D-projected sky arena with yaw foreshortening, banking turns, wing flaps,
and dramatic dragonfire torrents with lingering embers.
"""

import os
import sys
import math
from PIL import Image, ImageDraw

from build_master_wyverns import render_wyvern
from build_7stage_wyverns import PALETTES, draw_realistic_head, draw_realistic_breath

TOTAL_PATROL_FRAMES = 48
CANVAS_SIZE = 512

def render_patrol_frame(pal, frame_idx, total_frames=48, stage_num=6):
    """
    Render a single 512x512 transparent frame of a wyvern in an aerial patrol flight.
    """
    t = frame_idx / float(total_frames)
    angle = t * 2 * math.pi
    
    cx, cy = 256, 256
    rx, ry = 162, 112
    
    # Smooth undulating swooping patrol trajectory
    px = cx + rx * math.cos(angle)
    py = cy + ry * math.sin(angle) - 22 * math.sin(2 * angle)
    
    # Velocity vector for aeronautical heading
    vx = -rx * math.sin(angle)
    vy = ry * math.cos(angle) - 44 * math.cos(2 * angle)
    
    # Wing cycle: 8 frames per wing beat cycle (6 full flaps per patrol lap)
    wing_phase = frame_idx % 8
    
    # Dragonfire burst during the apex swoop (frames 14 to 22)
    is_breathing = 14 <= frame_idx <= 22
    mouth_open = 5 if is_breathing else 0
    
    # Scale based on stage
    stage_scale = 1.0
    if stage_num == 4:
        stage_scale = 0.85
    elif stage_num == 5:
        stage_scale = 1.0
    elif stage_num == 6:
        stage_scale = 1.15
    elif stage_num >= 7:
        stage_scale = 1.32

    pal_inst = dict(pal)
    pal_inst["scale"] = pal.get("scale", 1.0) * stage_scale
    
    # 1. Render base wyvern body (headless to prevent double-jaw artifacts)
    sprite = render_wyvern(pal_inst, frame_idx=wing_phase, total_frames=8, include_head=False)
    
    # 2. Attach realistic articulated head
    spine_y = 66
    bob_y = math.sin((wing_phase / 8.0) * 2 * math.pi) * 2.5
    traits = pal_inst.get("special_traits", [])
    hx = 78 + (3 if "serpentine_neck" in traits else 0)
    hy = int(spine_y - 15 + bob_y - (3 if "serpentine_neck" in traits else 0))
    
    draw_realistic_head(sprite, hx, hy, pal_inst, jaw_open=mouth_open, scale=stage_scale)
    
    # 3. Roaring dragonfire if in breath phase
    if is_breathing:
        draw_realistic_breath(sprite, hx + int(24 * stage_scale), hy + int(7 * stage_scale) + mouth_open // 2, pal_inst, flame_len=int(44 * stage_scale))
        
    # 4. Upscale sprite for 512 canvas resolution
    target_scale = 1.25
    sw = int(round(sprite.width * target_scale))
    sh = int(round(sprite.height * target_scale))
    sprite = sprite.resize((sw, sh), Image.Resampling.NEAREST)
    
    # 5. 3D Yaw Foreshortening & Banking Turn:
    # As the dragon curves around the turn (vx approaches 0), foreshorten horizontally
    norm_vx = abs(vx) / float(rx)
    yaw_factor = 0.45 + 0.55 * (norm_vx ** 0.7)
    yaw_w = max(24, int(round(sw * yaw_factor)))
    sprite = sprite.resize((yaw_w, sh), Image.Resampling.BILINEAR)
    
    # 6. Directional Flip & Pitch Rotation
    flying_right = (vx >= 0)
    if not flying_right:
        sprite = sprite.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
        pitch = -math.degrees(math.atan2(vy, -vx))
    else:
        pitch = -math.degrees(math.atan2(vy, vx))
        
    # Clamp pitch angle to natural flight attitudes
    pitch = max(-26, min(26, pitch))
    rotated = sprite.rotate(pitch, resample=Image.Resampling.BILINEAR, expand=True)
    
    # 7. Place onto 512x512 transparent arena
    frame = Image.new("RGBA", (CANVAS_SIZE, CANVAS_SIZE), (0, 0, 0, 0))
    sx = int(round(px - rotated.width / 2.0))
    sy = int(round(py - rotated.height / 2.0))
    frame.paste(rotated, (sx, sy), rotated)
    
    return frame, (px, py, vx, vy, is_breathing)

def build_dragon_patrol(dragon_key, out_path, stage_num=6):
    pal = PALETTES.get(dragon_key)
    if not pal:
        print(f"Unknown dragon: {dragon_key}")
        return
        
    print(f" -> Rendering 48-Frame Aerial Patrol (512x512) for: {pal['name']}...")
    frames = []
    embers = []
    
    for f in range(TOTAL_PATROL_FRAMES):
        frame, (px, py, vx, vy, is_breathing) = render_patrol_frame(pal, f, TOTAL_PATROL_FRAMES, stage_num=stage_num)
        draw = ImageDraw.Draw(frame)
        
        # Trailing ember physics
        flying_right = (vx >= 0)
        if is_breathing:
            mouth_offset_x = 45 * (1 if flying_right else -1)
            embers.append({
                "x": px + mouth_offset_x,
                "y": py + 10,
                "vx": -vx * 0.14 + (1 if f % 2 == 0 else -1) * 2,
                "vy": -vy * 0.1 - 1.6,
                "age": 0,
                "max_age": 7,
                "col": pal["fire_inner"]
            })
            
        active_embers = []
        for emb in embers:
            emb["x"] += emb["vx"]
            emb["y"] += emb["vy"]
            emb["age"] += 1
            if emb["age"] < emb["max_age"]:
                active_embers.append(emb)
                alpha = int(255 * (1.0 - emb["age"] / float(emb["max_age"])))
                c = emb["col"]
                col_with_alpha = (c[0], c[1], c[2], alpha)
                draw.ellipse([emb["x"] - 1.5, emb["y"] - 1.5, emb["x"] + 1.5, emb["y"] + 1.5], fill=col_with_alpha)
        embers = active_embers
        
        frames.append(frame)
        
    # Save optimized 48-frame GIF
    frames[0].save(
        out_path,
        save_all=True,
        append_images=frames[1:],
        duration=75,
        loop=0,
        disposal=2
    )
    print(f"    Saved: {out_path} ({os.path.getsize(out_path):,} bytes)")

def build_all_patrols(out_dir):
    os.makedirs(out_dir, exist_ok=True)
    for key in PALETTES:
        out_file = os.path.join(out_dir, f"{key}_patrol.gif")
        build_dragon_patrol(key, out_file, stage_num=6)

if __name__ == "__main__":
    out_dir = os.path.join(os.path.dirname(__file__), "dragons")
    target = sys.argv[1].lower() if len(sys.argv) > 1 else "all"
    if target == "all":
        build_all_patrols(out_dir)
    else:
        out_file = os.path.join(out_dir, f"{target}_patrol.gif")
        build_dragon_patrol(target, out_file)

