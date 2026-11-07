"""
Git-Wyvern: Master Pixel Art Wyvern Engine (GoT & HotD Complete Edition)
Canon Wyvern Anatomical Specifications:
- True Wyverns: 2 Wings (forelimbs with clawed digits) + 2 Hind Legs (foreground & shadow background) = 4 limbs
- Canvas: 176x128 (upscaled 2x to 352x256), 100% unclipped wings, whip tail, and oral flame eruption
- 15 Authentic Series Dragons with Unique Skull & Body Morphs:
  * Caraxes: Elongated serpentine S-curve neck, cranial ear frills, webbed leg-rudders on hind ankles
  * Vhagar: 1.28x colossal scale, hanging wrinkled throat dewlap/wattle, blunt dino skull, milky aged eye
  * Meleys: Circular crown/tiara of horns ringing cranium, scarlet body with pink/copper wings
  * Sunfyre: Metallic burnished gold body with soft blush-pink wing membranes, noble equine head
  * Vermithor: 1.22x colossal scale, bronze-ochre hide, chin beard spikes under jaw, barrel chest
  * Moondancer: Slender 0.82x raptor frame, aquamarine/mint hide, punk-rock mohawk quill crest
  * Tessarion: Cobalt blue body with beaten copper wing membranes, cobalt blue fire
  * Seasmoke: Pale silver-grey, slate wings, chin goatee spike
  * Silverwing: Shimmering pearl silver, gentle curved horns, silver-white flame
  * Sheepstealer: Mud-brown hide, ragged wings, heavy underbite with protruding lower fangs, crooked horns
  * Syrax: Sulfur-gold body, cream wings, backward crown crest, equine skull
  * Drogon: Jet black, blood-red wing ribbing/spines, swept-back horns, red eyes, massive flame
  * Rhaegal: Deep emerald jade, bronze wing accents, dual forked horns, green-edged fire
  * Viserion: Ivory-cream, golden wing webs, curved ram horns, golden fire
  * Wight Viserion: Rotting ash-grey, tattered torn wings, ice-blue glowing eyes, straight blue plasma laser beam
"""

import math
import os
import random
from PIL import Image, ImageDraw

CANVAS_W = 176
CANVAS_H = 128
UPSCALE = 2  # 176x128 -> 352x256 crisp pixel art display

PALETTES = {
    # -------------------------------------------------------------------------
    # GAME OF THRONES WYVERNS
    # -------------------------------------------------------------------------
    "drogon": {
        "name": "Drogon (The Black Dread Reborn)",
        "series": "GoT",
        "scale": 1.06,
        "neck_type": "standard",
        "leg_type": "standard",
        "skull_type": "canine",
        "horn_type": "swept_back",
        "wing_style": "solid",
        "breath_type": "fire_red",
        "scale_hi": (76, 82, 98, 255),
        "scale_mid": (36, 38, 48, 255),
        "scale_dark": (20, 21, 28, 255),
        "scale_deep": (10, 11, 14, 255),
        "belly_hi": (65, 60, 68, 255),
        "belly_mid": (44, 40, 48, 255),
        "belly_dark": (26, 24, 28, 255),
        "belly_line": (12, 10, 14, 255),
        "horn_hi": (245, 60, 75, 255),
        "horn_mid": (185, 22, 38, 255),
        "horn_dark": (115, 12, 24, 255),
        "web_glow": (240, 50, 65, 255),
        "web_hi": (200, 26, 40, 255),
        "web_mid": (140, 14, 26, 255),
        "web_dark": (75, 8, 16, 255),
        "web_shadow": (35, 4, 8, 255),
        "wing_bone_hi": (70, 75, 90, 255),
        "wing_bone_mid": (36, 38, 48, 255),
        "wing_bone_dark": (18, 19, 24, 255),
        "eye_glow": (255, 30, 30, 255),
        "eye_core": (255, 220, 50, 255),
        "fire_core": (255, 255, 250, 255),
        "fire_inner": (255, 225, 40, 255),
        "fire_mid": (255, 120, 10, 255),
        "fire_outer": (215, 25, 10, 255),
        "fire_ember": (255, 210, 70, 255),
    },
    "rhaegal": {
        "name": "Rhaegal (The Jade Dragon)",
        "series": "GoT",
        "scale": 0.98,
        "neck_type": "standard",
        "leg_type": "standard",
        "skull_type": "canine",
        "horn_type": "dual_forked",
        "wing_style": "solid",
        "breath_type": "fire_green",
        "scale_hi": (68, 150, 110, 255),
        "scale_mid": (34, 96, 66, 255),
        "scale_dark": (18, 58, 40, 255),
        "scale_deep": (10, 32, 22, 255),
        "belly_hi": (158, 140, 88, 255),
        "belly_mid": (112, 98, 56, 255),
        "belly_dark": (72, 60, 34, 255),
        "belly_line": (38, 30, 16, 255),
        "horn_hi": (238, 192, 82, 255),
        "horn_mid": (188, 140, 44, 255),
        "horn_dark": (122, 84, 24, 255),
        "web_glow": (248, 208, 92, 255),
        "web_hi": (208, 158, 48, 255),
        "web_mid": (158, 110, 30, 255),
        "web_dark": (98, 64, 18, 255),
        "web_shadow": (55, 34, 10, 255),
        "wing_bone_hi": (60, 125, 90, 255),
        "wing_bone_mid": (30, 75, 50, 255),
        "wing_bone_dark": (16, 42, 28, 255),
        "eye_glow": (255, 200, 30, 255),
        "eye_core": (255, 245, 140, 255),
        "fire_core": (255, 255, 245, 255),
        "fire_inner": (255, 230, 60, 255),
        "fire_mid": (255, 135, 15, 255),
        "fire_outer": (110, 235, 30, 255),
        "fire_ember": (140, 255, 60, 255),
    },
    "viserion": {
        "name": "Viserion (The Gold Dragon)",
        "series": "GoT",
        "scale": 0.98,
        "neck_type": "standard",
        "leg_type": "standard",
        "skull_type": "equine",
        "horn_type": "curved_down",
        "wing_style": "solid",
        "breath_type": "fire_gold",
        "scale_hi": (255, 255, 252, 255),
        "scale_mid": (245, 239, 220, 255),
        "scale_dark": (210, 202, 176, 255),
        "scale_deep": (160, 152, 128, 255),
        "belly_hi": (255, 245, 215, 255),
        "belly_mid": (232, 212, 162, 255),
        "belly_dark": (188, 162, 112, 255),
        "belly_line": (132, 112, 72, 255),
        "horn_hi": (255, 230, 88, 255),
        "horn_mid": (230, 170, 35, 255),
        "horn_dark": (165, 115, 18, 255),
        "web_glow": (255, 236, 122, 255),
        "web_hi": (252, 192, 48, 255),
        "web_mid": (214, 140, 24, 255),
        "web_dark": (154, 94, 16, 255),
        "web_shadow": (95, 55, 8, 255),
        "wing_bone_hi": (255, 252, 240, 255),
        "wing_bone_mid": (220, 210, 184, 255),
        "wing_bone_dark": (164, 154, 130, 255),
        "eye_glow": (255, 185, 25, 255),
        "eye_core": (255, 242, 110, 255),
        "fire_core": (255, 255, 255, 255),
        "fire_inner": (255, 245, 150, 255),
        "fire_mid": (255, 195, 45, 255),
        "fire_outer": (245, 135, 18, 255),
        "fire_ember": (255, 230, 90, 255),
    },
    "wight_viserion": {
        "name": "Wight Viserion (The Undead Ice Dragon)",
        "series": "GoT",
        "scale": 0.98,
        "neck_type": "standard",
        "leg_type": "standard",
        "skull_type": "equine",
        "horn_type": "broken_ram",
        "wing_style": "tattered",
        "breath_type": "plasma_beam",
        "scale_hi": (172, 184, 202, 255),
        "scale_mid": (88, 100, 118, 255),
        "scale_dark": (50, 58, 70, 255),
        "scale_deep": (26, 30, 38, 255),
        "belly_hi": (138, 148, 160, 255),
        "belly_mid": (92, 102, 114, 255),
        "belly_dark": (54, 62, 74, 255),
        "belly_line": (24, 28, 34, 255),
        "horn_hi": (218, 232, 248, 255),
        "horn_mid": (152, 168, 184, 255),
        "horn_dark": (98, 112, 128, 255),
        "web_glow": (122, 138, 162, 255),
        "web_hi": (82, 94, 118, 255),
        "web_mid": (54, 64, 82, 255),
        "web_dark": (32, 38, 50, 255),
        "web_shadow": (18, 22, 30, 255),
        "wing_bone_hi": (148, 160, 178, 255),
        "wing_bone_mid": (78, 88, 102, 255),
        "wing_bone_dark": (40, 46, 56, 255),
        "eye_glow": (0, 245, 255, 255),
        "eye_core": (190, 252, 255, 255),
        "fire_core": (235, 252, 255, 255),
        "fire_inner": (90, 225, 255, 255),
        "fire_mid": (0, 145, 255, 255),
        "fire_outer": (135, 50, 245, 255),
        "fire_ember": (0, 220, 255, 255),
    },

    # -------------------------------------------------------------------------
    # HOUSE OF THE DRAGON WYVERNS
    # -------------------------------------------------------------------------
    "caraxes": {
        "name": "Caraxes (The Blood Wyrm)",
        "series": "HotD",
        "scale": 1.04,
        "neck_type": "serpentine_long",
        "leg_type": "rudder_fins",
        "skull_type": "serpentine",
        "horn_type": "caraxes_frills",
        "wing_style": "solid",
        "breath_type": "fire_blood",
        "scale_hi": (220, 40, 40, 255),
        "scale_mid": (155, 20, 25, 255),
        "scale_dark": (95, 10, 15, 255),
        "scale_deep": (50, 5, 8, 255),
        "belly_hi": (195, 65, 45, 255),
        "belly_mid": (135, 38, 26, 255),
        "belly_dark": (85, 20, 15, 255),
        "belly_line": (40, 8, 8, 255),
        "horn_hi": (130, 15, 20, 255),
        "horn_mid": (75, 8, 12, 255),
        "horn_dark": (40, 4, 6, 255),
        "web_glow": (245, 55, 45, 255),
        "web_hi": (190, 25, 30, 255),
        "web_mid": (120, 12, 20, 255),
        "web_dark": (65, 8, 14, 255),
        "web_shadow": (35, 4, 8, 255),
        "wing_bone_hi": (210, 45, 45, 255),
        "wing_bone_mid": (135, 20, 25, 255),
        "wing_bone_dark": (70, 10, 15, 255),
        "eye_glow": (255, 140, 0, 255),
        "eye_core": (255, 225, 60, 255),
        "fire_core": (255, 255, 240, 255),
        "fire_inner": (255, 195, 30, 255),
        "fire_mid": (255, 75, 15, 255),
        "fire_outer": (185, 15, 25, 255),
        "fire_ember": (255, 130, 40, 255),
    },
    "vhagar": {
        "name": "Vhagar (The Queen of All Dragons)",
        "series": "HotD",
        "scale": 1.28,
        "neck_type": "thick_dewlap",
        "leg_type": "standard",
        "skull_type": "dino_heavy",
        "horn_type": "vhagar_ancient",
        "wing_style": "solid",
        "breath_type": "fire_volcano",
        "scale_hi": (112, 132, 102, 255),
        "scale_mid": (72, 88, 66, 255),
        "scale_dark": (45, 56, 42, 255),
        "scale_deep": (26, 34, 24, 255),
        "belly_hi": (156, 150, 122, 255),
        "belly_mid": (118, 112, 88, 255),
        "belly_dark": (80, 74, 56, 255),
        "belly_line": (48, 44, 32, 255),
        "horn_hi": (158, 142, 108, 255),
        "horn_mid": (112, 98, 74, 255),
        "horn_dark": (68, 58, 42, 255),
        "web_glow": (132, 148, 118, 255),
        "web_hi": (98, 114, 88, 255),
        "web_mid": (68, 80, 60, 255),
        "web_dark": (42, 52, 38, 255),
        "web_shadow": (24, 30, 22, 255),
        "wing_bone_hi": (108, 126, 98, 255),
        "wing_bone_mid": (68, 82, 62, 255),
        "wing_bone_dark": (36, 46, 32, 255),
        "eye_glow": (255, 235, 160, 255),
        "eye_core": (240, 245, 220, 255),  # Cloudy pale aged eye
        "fire_core": (255, 255, 250, 255),
        "fire_inner": (255, 220, 50, 255),
        "fire_mid": (255, 110, 15, 255),
        "fire_outer": (220, 45, 10, 255),
        "fire_ember": (255, 190, 40, 255),
    },
    "meleys": {
        "name": "Meleys (The Red Queen)",
        "series": "HotD",
        "scale": 1.08,
        "neck_type": "standard",
        "leg_type": "standard",
        "skull_type": "equine",
        "horn_type": "meleys_crown",
        "wing_style": "solid",
        "breath_type": "fire_red",
        "scale_hi": (245, 65, 65, 255),
        "scale_mid": (195, 25, 35, 255),
        "scale_dark": (125, 15, 22, 255),
        "scale_deep": (65, 8, 12, 255),
        "belly_hi": (245, 152, 120, 255),
        "belly_mid": (205, 108, 80, 255),
        "belly_dark": (145, 68, 48, 255),
        "belly_line": (75, 30, 20, 255),
        "horn_hi": (245, 165, 55, 255),   # Crown tiara gold/copper
        "horn_mid": (205, 115, 30, 255),
        "horn_dark": (135, 65, 15, 255),
        "web_glow": (255, 175, 195, 255),  # Distinctive soft pink & copper membranes
        "web_hi": (235, 115, 145, 255),
        "web_mid": (175, 65, 95, 255),
        "web_dark": (110, 35, 60, 255),
        "web_shadow": (55, 16, 30, 255),
        "wing_bone_hi": (225, 60, 60, 255),
        "wing_bone_mid": (155, 25, 30, 255),
        "wing_bone_dark": (85, 12, 18, 255),
        "eye_glow": (255, 150, 20, 255),
        "eye_core": (255, 220, 90, 255),
        "fire_core": (255, 255, 245, 255),
        "fire_inner": (255, 225, 50, 255),
        "fire_mid": (255, 90, 20, 255),
        "fire_outer": (225, 25, 20, 255),
        "fire_ember": (255, 160, 60, 255),
    },
    "sunfyre": {
        "name": "Sunfyre (The Golden)",
        "series": "HotD",
        "scale": 0.98,
        "neck_type": "standard",
        "leg_type": "standard",
        "skull_type": "equine",
        "horn_type": "sunfyre_equine",
        "wing_style": "solid",
        "breath_type": "fire_gold",
        "scale_hi": (255, 238, 110, 255),  # Dazzling metallic burnished gold
        "scale_mid": (238, 192, 45, 255),
        "scale_dark": (185, 138, 22, 255),
        "scale_deep": (115, 82, 10, 255),
        "belly_hi": (255, 245, 175, 255),
        "belly_mid": (245, 215, 125, 255),
        "belly_dark": (195, 162, 85, 255),
        "belly_line": (125, 98, 42, 255),
        "horn_hi": (255, 250, 180, 255),
        "horn_mid": (242, 205, 75, 255),
        "horn_dark": (175, 135, 28, 255),
        "web_glow": (255, 198, 215, 255),  # Striking pale rose / light pink membranes
        "web_hi": (252, 158, 185, 255),
        "web_mid": (218, 105, 138, 255),
        "web_dark": (155, 62, 92, 255),
        "web_shadow": (95, 32, 55, 255),
        "wing_bone_hi": (255, 230, 95, 255),
        "wing_bone_mid": (215, 170, 35, 255),
        "wing_bone_dark": (145, 105, 18, 255),
        "eye_glow": (255, 175, 25, 255),
        "eye_core": (255, 240, 120, 255),
        "fire_core": (255, 255, 255, 255),
        "fire_inner": (255, 245, 140, 255),
        "fire_mid": (255, 190, 40, 255),
        "fire_outer": (245, 130, 20, 255),
        "fire_ember": (255, 235, 90, 255),
    },
    "vermithor": {
        "name": "Vermithor (The Bronze Fury)",
        "series": "HotD",
        "scale": 1.22,
        "neck_type": "standard",
        "leg_type": "standard",
        "skull_type": "dino_heavy",
        "horn_type": "vermithor_beard",
        "wing_style": "solid",
        "breath_type": "fire_volcano",
        "scale_hi": (175, 115, 52, 255),  # Deep mottled bronze / dark ochre
        "scale_mid": (128, 76, 28, 255),
        "scale_dark": (84, 46, 16, 255),
        "scale_deep": (48, 24, 8, 255),
        "belly_hi": (195, 145, 88, 255),
        "belly_mid": (148, 102, 54, 255),
        "belly_dark": (102, 66, 32, 255),
        "belly_line": (58, 34, 14, 255),
        "horn_hi": (145, 95, 45, 255),
        "horn_mid": (98, 58, 24, 255),
        "horn_dark": (58, 32, 12, 255),
        "web_glow": (185, 122, 62, 255),
        "web_hi": (142, 86, 38, 255),
        "web_mid": (102, 58, 24, 255),
        "web_dark": (68, 36, 14, 255),
        "web_shadow": (38, 18, 8, 255),
        "wing_bone_hi": (165, 108, 48, 255),
        "wing_bone_mid": (118, 68, 24, 255),
        "wing_bone_dark": (68, 36, 12, 255),
        "eye_glow": (255, 145, 20, 255),
        "eye_core": (255, 215, 60, 255),
        "fire_core": (255, 255, 245, 255),
        "fire_inner": (255, 210, 45, 255),
        "fire_mid": (255, 115, 18, 255),
        "fire_outer": (215, 48, 12, 255),
        "fire_ember": (255, 180, 50, 255),
    },
    "moondancer": {
        "name": "Moondancer",
        "series": "HotD",
        "scale": 0.82,
        "neck_type": "mohawk_crest",
        "leg_type": "standard",
        "skull_type": "raptor",
        "horn_type": "moondancer_crest",
        "wing_style": "solid",
        "breath_type": "fire_green",
        "scale_hi": (145, 245, 200, 255),  # Pale aquamarine / mint green
        "scale_mid": (85, 205, 155, 255),
        "scale_dark": (45, 145, 105, 255),
        "scale_deep": (22, 85, 58, 255),
        "belly_hi": (225, 255, 242, 255),
        "belly_mid": (175, 235, 205, 255),
        "belly_dark": (120, 185, 150, 255),
        "belly_line": (65, 120, 92, 255),
        "horn_hi": (40, 220, 140, 255),
        "horn_mid": (20, 165, 98, 255),
        "horn_dark": (12, 105, 60, 255),
        "web_glow": (185, 252, 225, 255),
        "web_hi": (135, 232, 192, 255),
        "web_mid": (85, 185, 142, 255),
        "web_dark": (48, 128, 92, 255),
        "web_shadow": (26, 75, 52, 255),
        "wing_bone_hi": (125, 228, 182, 255),
        "wing_bone_mid": (72, 178, 135, 255),
        "wing_bone_dark": (36, 112, 80, 255),
        "eye_glow": (10, 220, 110, 255),
        "eye_core": (180, 255, 210, 255),
        "fire_core": (255, 255, 245, 255),
        "fire_inner": (210, 255, 80, 255),
        "fire_mid": (140, 235, 40, 255),
        "fire_outer": (70, 195, 45, 255),
        "fire_ember": (160, 255, 110, 255),
    },
    "tessarion": {
        "name": "Tessarion (The Blue Queen)",
        "series": "HotD",
        "scale": 0.98,
        "neck_type": "standard",
        "leg_type": "standard",
        "skull_type": "equine",
        "horn_type": "tessarion_crest",
        "wing_style": "solid",
        "breath_type": "fire_blue",
        "scale_hi": (65, 120, 235, 255),   # Dark cobalt blue
        "scale_mid": (32, 75, 185, 255),
        "scale_dark": (18, 45, 125, 255),
        "scale_deep": (10, 24, 75, 255),
        "belly_hi": (245, 165, 75, 255),   # Beaten copper belly
        "belly_mid": (205, 115, 35, 255),
        "belly_dark": (145, 72, 18, 255),
        "belly_line": (85, 38, 8, 255),
        "horn_hi": (245, 155, 65, 255),    # Copper horns
        "horn_mid": (195, 105, 30, 255),
        "horn_dark": (125, 60, 15, 255),
        "web_glow": (245, 155, 75, 255),   # Rich metallic beaten copper wing membranes
        "web_hi": (205, 110, 38, 255),
        "web_mid": (155, 72, 22, 255),
        "web_dark": (102, 42, 12, 255),
        "web_shadow": (55, 20, 6, 255),
        "wing_bone_hi": (55, 105, 215, 255),
        "wing_bone_mid": (28, 65, 160, 255),
        "wing_bone_dark": (14, 38, 105, 255),
        "eye_glow": (255, 185, 30, 255),
        "eye_core": (255, 235, 110, 255),
        "fire_core": (245, 255, 255, 255),
        "fire_inner": (120, 220, 255, 255),
        "fire_mid": (35, 115, 255, 255),
        "fire_outer": (15, 55, 220, 255),
        "fire_ember": (110, 210, 255, 255),
    },
    "seasmoke": {
        "name": "Seasmoke",
        "series": "HotD",
        "scale": 1.02,
        "neck_type": "standard",
        "leg_type": "standard",
        "skull_type": "canine",
        "horn_type": "seasmoke_goatee",
        "wing_style": "solid",
        "breath_type": "fire_gold",
        "scale_hi": (212, 222, 232, 255),  # Pale silvery-grey / smoky pewter
        "scale_mid": (158, 172, 186, 255),
        "scale_dark": (108, 122, 138, 255),
        "scale_deep": (62, 74, 88, 255),
        "belly_hi": (235, 242, 250, 255),
        "belly_mid": (195, 206, 218, 255),
        "belly_dark": (148, 160, 174, 255),
        "belly_line": (95, 108, 122, 255),
        "horn_hi": (142, 155, 170, 255),
        "horn_mid": (102, 114, 128, 255),
        "horn_dark": (64, 74, 86, 255),
        "web_glow": (198, 212, 228, 255),  # Translucent slate-grey membranes
        "web_hi": (152, 168, 185, 255),
        "web_mid": (112, 128, 145, 255),
        "web_dark": (76, 90, 106, 255),
        "web_shadow": (45, 55, 68, 255),
        "wing_bone_hi": (195, 208, 222, 255),
        "wing_bone_mid": (142, 156, 172, 255),
        "wing_bone_dark": (92, 106, 120, 255),
        "eye_glow": (110, 225, 245, 255),
        "eye_core": (215, 250, 255, 255),
        "fire_core": (255, 255, 250, 255),
        "fire_inner": (255, 235, 90, 255),
        "fire_mid": (255, 165, 30, 255),
        "fire_outer": (245, 110, 15, 255),
        "fire_ember": (255, 225, 80, 255),
    },
    "silverwing": {
        "name": "Silverwing",
        "series": "HotD",
        "scale": 1.18,
        "neck_type": "standard",
        "leg_type": "standard",
        "skull_type": "canine",
        "horn_type": "silverwing_curved",
        "wing_style": "solid",
        "breath_type": "fire_silver",
        "scale_hi": (255, 255, 255, 255),  # Shimmering pearl-silver
        "scale_mid": (228, 235, 242, 255),
        "scale_dark": (185, 195, 206, 255),
        "scale_deep": (132, 142, 155, 255),
        "belly_hi": (255, 255, 255, 255),
        "belly_mid": (238, 242, 248, 255),
        "belly_dark": (198, 208, 218, 255),
        "belly_line": (148, 158, 170, 255),
        "horn_hi": (220, 228, 238, 255),
        "horn_mid": (172, 184, 196, 255),
        "horn_dark": (120, 132, 145, 255),
        "web_glow": (255, 255, 255, 255),  # Silky pale silver-white membranes
        "web_hi": (242, 246, 252, 255),
        "web_mid": (205, 215, 228, 255),
        "web_dark": (158, 170, 184, 255),
        "web_shadow": (108, 120, 134, 255),
        "wing_bone_hi": (255, 255, 255, 255),
        "wing_bone_mid": (218, 226, 236, 255),
        "wing_bone_dark": (168, 178, 190, 255),
        "eye_glow": (255, 235, 140, 255),
        "eye_core": (255, 250, 210, 255),
        "fire_core": (255, 255, 255, 255),
        "fire_inner": (255, 248, 195, 255),
        "fire_mid": (238, 215, 130, 255),
        "fire_outer": (210, 175, 95, 255),
        "fire_ember": (255, 245, 170, 255),
    },
    "sheepstealer": {
        "name": "Sheepstealer (Wild Dragon)",
        "series": "HotD",
        "scale": 1.10,
        "neck_type": "standard",
        "leg_type": "standard",
        "skull_type": "underbite_feral",
        "horn_type": "sheepstealer_crooked",
        "wing_style": "tattered",
        "breath_type": "fire_red",
        "scale_hi": (138, 102, 68, 255),  # Mud-brown / dark dirt ochre
        "scale_mid": (98, 68, 42, 255),
        "scale_dark": (64, 42, 24, 255),
        "scale_deep": (36, 22, 12, 255),
        "belly_hi": (162, 126, 90, 255),
        "belly_mid": (120, 88, 58, 255),
        "belly_dark": (82, 56, 34, 255),
        "belly_line": (46, 30, 16, 255),
        "horn_hi": (118, 88, 56, 255),
        "horn_mid": (78, 54, 32, 255),
        "horn_dark": (46, 30, 16, 255),
        "web_glow": (148, 112, 78, 255),
        "web_hi": (112, 80, 52, 255),
        "web_mid": (80, 54, 32, 255),
        "web_dark": (52, 34, 18, 255),
        "web_shadow": (28, 18, 8, 255),
        "wing_bone_hi": (126, 92, 60, 255),
        "wing_bone_mid": (88, 60, 36, 255),
        "wing_bone_dark": (52, 34, 18, 255),
        "eye_glow": (245, 140, 25, 255),
        "eye_core": (255, 210, 80, 255),
        "fire_core": (255, 255, 240, 255),
        "fire_inner": (255, 205, 50, 255),
        "fire_mid": (245, 105, 20, 255),
        "fire_outer": (195, 45, 12, 255),
        "fire_ember": (255, 165, 45, 255),
    },
    "syrax": {
        "name": "Syrax",
        "series": "HotD",
        "scale": 1.00,
        "neck_type": "standard",
        "leg_type": "standard",
        "skull_type": "equine",
        "horn_type": "sunfyre_equine",
        "wing_style": "solid",
        "breath_type": "fire_gold",
        "scale_hi": (255, 228, 85, 255),  # Sulfur-yellow / warm gold
        "scale_mid": (235, 180, 35, 255),
        "scale_dark": (185, 130, 20, 255),
        "scale_deep": (115, 75, 10, 255),
        "belly_hi": (255, 248, 195, 255),
        "belly_mid": (248, 225, 145, 255),
        "belly_dark": (195, 172, 98, 255),
        "belly_line": (125, 105, 52, 255),
        "horn_hi": (245, 210, 75, 255),
        "horn_mid": (205, 155, 30, 255),
        "horn_dark": (145, 100, 18, 255),
        "web_glow": (255, 250, 190, 255),  # Pale cream-yellow membranes
        "web_hi": (254, 238, 145, 255),
        "web_mid": (222, 195, 95, 255),
        "web_dark": (165, 140, 58, 255),
        "web_shadow": (102, 84, 30, 255),
        "wing_bone_hi": (255, 222, 75, 255),
        "wing_bone_mid": (215, 165, 30, 255),
        "wing_bone_dark": (145, 102, 16, 255),
        "eye_glow": (225, 120, 20, 255),
        "eye_core": (255, 195, 50, 255),
        "fire_core": (255, 255, 250, 255),
        "fire_inner": (255, 235, 80, 255),
        "fire_mid": (255, 160, 25, 255),
        "fire_outer": (235, 95, 15, 255),
        "fire_ember": (255, 215, 70, 255),
    },
}

COL_FANG_HI = (250, 250, 245, 255)
COL_FANG_MID = (185, 185, 178, 255)
COL_FANG_DARK = (95, 95, 90, 255)

def render_wyvern(pal, frame_idx=0, total_frames=8, include_head=True):
    master = Image.new("RGBA", (CANVAS_W, CANVAS_H), (0, 0, 0, 0))
    
    t = (frame_idx % total_frames) / float(total_frames)
    bob_y = int(math.sin(t * 2 * math.pi) * 2.2)
    wing_phase = frame_idx % 8
    scale = pal["scale"]
    
    # Anchor coordinates comfortably centered with generous margins
    # Vhagar slightly shifted left/down to balance colossal size
    if pal["scale"] > 1.2:
        cx = 62
        cy = 71 + bob_y
    elif pal["scale"] < 0.9:
        cx = 66
        cy = 73 + bob_y
    else:
        cx = 64
        cy = 72 + bob_y

    # Head anchor coordinates
    if pal.get("neck_type") == "serpentine_long":
        hx = cx + 24
        hy = cy - 23
    else:
        hx = cx + 22
        hy = cy - 20

    jaw_open = 4 if wing_phase in (3, 4, 5) else 0

    # =========================================================================
    # LAYER 1: FAR WING & FAR HIND LEG (Behind Dragon Body)
    # =========================================================================
    far_layer = Image.new("RGBA", (CANVAS_W, CANVAS_H), (0, 0, 0, 0))
    far_draw = ImageDraw.Draw(far_layer)
    far_pix = far_layer.load()

    def set_far_p(x, y, col):
        ix, iy = int(round(x)), int(round(y))
        if 0 <= ix < CANVAS_W and 0 <= iy < CANVAS_H:
            far_pix[ix, iy] = col

    # 1A. Far Wing
    far_offsets = [
        (-4, -16), (-2, -10), (0, -2), (3, 6), (5, 10), (3, 5), (0, -3), (-2, -11)
    ]
    fwx, fwy = far_offsets[wing_phase]
    
    far_sh = (cx - 2, cy - 6)
    far_el = (cx - 8 + fwx // 2, cy - 22 + fwy)
    far_wr = (cx - 14 + fwx, cy - 32 + fwy)
    
    far_f1 = (far_wr[0] + 16, far_wr[1] - 8)
    far_f2 = (far_wr[0] - 6, far_wr[1] - 8)
    far_f3 = (far_wr[0] - 22, far_wr[1] + 4)
    far_f4 = (far_wr[0] - 26, far_wr[1] + 18)
    
    # Complete Far Wing Membranes (including propatagium between shoulder, elbow, wrist and f1)
    far_draw.polygon([far_sh, far_el, far_wr, far_f1], fill=pal["web_dark"])
    far_draw.polygon([far_sh, far_el, far_wr, far_f1, far_f2, far_f3, far_f4, (cx - 18, cy + 2)], fill=pal["web_dark"])
    far_draw.line([far_sh, far_el, far_wr], fill=pal["scale_deep"], width=2)
    for fx, fy in [far_f1, far_f2, far_f3, far_f4]:
        far_draw.line([far_wr, (fx, fy)], fill=pal["scale_deep"], width=1)

    # 1B. FAR HIND LEG (2nd Leg, visible in shadow with offset flight stride)
    far_hip_x = cx - 11
    far_hip_y = cy + 2
    far_draw.polygon([
        (far_hip_x - 1, far_hip_y),
        (far_hip_x + 5, far_hip_y),
        (far_hip_x + 3, far_hip_y + 7),
        (far_hip_x - 3, far_hip_y + 6)
    ], fill=pal["scale_deep"])
    
    far_hock_x = far_hip_x - 1
    far_hock_y = far_hip_y + 7
    far_foot_x = far_hock_x + 3
    far_foot_y = far_hock_y + 6
    far_draw.line([(far_hock_x, far_hock_y), (far_hock_x - 2, far_hock_y + 3), (far_foot_x, far_foot_y)], fill=pal["scale_deep"], width=2)
    
    set_far_p(far_foot_x + 2, far_foot_y + 1, COL_FANG_DARK)
    set_far_p(far_foot_x + 3, far_foot_y + 2, COL_FANG_MID)
    set_far_p(far_foot_x + 1, far_foot_y + 2, COL_FANG_DARK)
    set_far_p(far_foot_x + 2, far_foot_y + 3, COL_FANG_MID)
    set_far_p(far_foot_x - 1, far_foot_y + 1, pal["scale_deep"])

    # Caraxes Leg Rudder Fin (Far leg)
    if pal.get("leg_type") == "rudder_fins":
        far_draw.polygon([
            (far_hock_x - 1, far_hock_y + 1),
            (far_hock_x - 7, far_hock_y - 2),
            (far_hock_x - 8, far_hock_y + 2),
            (far_hock_x - 2, far_hock_y + 4)
        ], fill=pal["web_dark"])
        far_draw.line([(far_hock_x - 1, far_hock_y + 1), (far_hock_x - 7, far_hock_y - 2)], fill=pal["scale_deep"], width=1)
        far_draw.line([(far_hock_x - 1, far_hock_y + 2), (far_hock_x - 8, far_hock_y + 2)], fill=pal["scale_deep"], width=1)

    master.paste(far_layer, (0, 0), far_layer)

    # =========================================================================
    # LAYER 2: NEAR WING LAYER (Behind Body, before Head/Horns)
    # =========================================================================
    near_layer = Image.new("RGBA", (CANVAS_W, CANVAS_H), (0, 0, 0, 0))
    near_draw = ImageDraw.Draw(near_layer)
    near_pix = near_layer.load()

    def set_np(x, y, col):
        ix, iy = int(round(x)), int(round(y))
        if 0 <= ix < CANVAS_W and 0 <= iy < CANVAS_H:
            near_pix[ix, iy] = col

    near_offsets = [
        (-4, -16), (-2, -10), (0, -2), (3, 6), (5, 10), (3, 5), (0, -3), (-2, -11)
    ]
    nwx, nwy = near_offsets[wing_phase]

    sh_x, sh_y = cx + 2, cy - 4
    el_x, el_y = sh_x - 4 + (nwx // 2), sh_y - 18 + nwy
    wr_x, wr_y = el_x - 6 + nwx, el_y - 12 + (nwy // 2)
    
    f1_x, f1_y = wr_x + 18, wr_y - 10
    f2_x, f2_y = wr_x - 4, wr_y - 14
    f3_x, f3_y = wr_x - 22, wr_y - 4
    f4_x, f4_y = wr_x - 28, wr_y + 12
    
    near_draw.polygon([(wr_x, wr_y), (f1_x, f1_y), (f2_x, f2_y)], fill=pal["web_mid"])
    near_draw.polygon([(wr_x, wr_y), (f2_x, f2_y), (f3_x, f3_y)], fill=pal["web_dark"])
    near_draw.polygon([(wr_x, wr_y), (f3_x, f3_y), (f4_x, f4_y)], fill=pal["web_mid"])
    near_draw.polygon([(sh_x, sh_y), (wr_x, wr_y), (f4_x, f4_y), (cx - 14, cy + 2)], fill=pal.get("web_shadow", pal["web_dark"]))
    # Complete Near Wing Propatagium (membrane connecting shoulder, elbow, wrist and finger 1)
    near_draw.polygon([(sh_x, sh_y), (el_x, el_y), (wr_x, wr_y), (f1_x, f1_y)], fill=pal["web_mid"])
    
    def draw_scallop(p1, p2, edge_col):
        mid_x = (p1[0] + p2[0]) // 2 + 1
        mid_y = (p1[1] + p2[1]) // 2 + 2
        near_draw.line([p1, (mid_x, mid_y), p2], fill=edge_col, width=1)
        
    draw_scallop((f1_x, f1_y), (f2_x, f2_y), pal["web_glow"])
    draw_scallop((f2_x, f2_y), (f3_x, f3_y), pal["web_glow"])
    draw_scallop((f3_x, f3_y), (f4_x, f4_y), pal["web_glow"])
    draw_scallop((f4_x, f4_y), (cx - 14, cy + 2), pal["web_dark"])
    
    for fx, fy in [(f1_x, f1_y), (f2_x, f2_y), (f3_x, f3_y), (f4_x, f4_y)]:
        near_draw.line([(wr_x, wr_y), (fx, fy)], fill=pal["wing_bone_dark"], width=2)
        near_draw.line([(wr_x, wr_y), (fx, fy)], fill=pal["wing_bone_hi"], width=1)
        set_np(fx, fy, COL_FANG_HI)
        set_np(fx + 1, fy, pal["scale_deep"])

    near_draw.line([(sh_x, sh_y), (el_x, el_y)], fill=pal["wing_bone_mid"], width=3)
    near_draw.line([(sh_x, sh_y), (el_x, el_y)], fill=pal["wing_bone_hi"], width=1)
    near_draw.line([(el_x, el_y), (wr_x, wr_y)], fill=pal["wing_bone_mid"], width=3)
    near_draw.line([(el_x, el_y), (wr_x, wr_y)], fill=pal["wing_bone_hi"], width=1)
    
    set_np(el_x + 1, el_y - 1, pal["horn_hi"])
    set_np(el_x + 2, el_y - 2, pal["horn_mid"])
    
    set_np(wr_x, wr_y - 1, COL_FANG_HI)
    set_np(wr_x + 1, wr_y - 2, COL_FANG_HI)
    set_np(wr_x + 2, wr_y - 3, COL_FANG_DARK)

    # Tattered wings (Wight Viserion, Sheepstealer)
    if pal["wing_style"] == "tattered":
        holes = [
            ((wr_x + f1_x) // 2 + 1, (wr_y + f1_y) // 2 + 1, 3, 3),
            ((wr_x + f2_x) // 2 - 1, (wr_y + f2_y) // 2 + 1, 4, 3),
            ((wr_x + f3_x) // 2 - 1, (wr_y + f3_y) // 2 + 1, 3, 2),
            ((sh_x + f4_x) // 2 + 1, (sh_y + f4_y) // 2 + 1, 4, 2),
        ]
        for hx_h, hy_h, hw_h, hh_h in holes:
            for dx in range(hw_h):
                for dy in range(hh_h):
                    set_np(hx_h + dx, hy_h + dy, (0, 0, 0, 0))
            near_draw.rectangle([hx_h - 1, hy_h - 1, hx_h + hw_h, hy_h + hh_h], outline=pal["scale_deep"])

    master.paste(near_layer, (0, 0), near_layer)

    # =========================================================================
    # LAYER 3: BODY & TAIL LAYER (Continuous Spline Extrusion)
    # =========================================================================
    body_layer = Image.new("RGBA", (CANVAS_W, CANVAS_H), (0, 0, 0, 0))
    body_draw = ImageDraw.Draw(body_layer)
    body_pix = body_layer.load()

    def set_bp(x, y, col):
        ix, iy = int(round(x)), int(round(y))
        if 0 <= ix < CANVAS_W and 0 <= iy < CANVAS_H:
            body_pix[ix, iy] = col

    # Configure spine based on neck & body morph
    if pal.get("neck_type") == "serpentine_long":
        # Caraxes: Authentic serpentine S-curve cobra neck!
        spine = [
            (hx - 1, hy + 7),                                           # 0: Neck top (anchored in skull)
            (cx + 19, cy - 18),                                         # 1: High arch S
            (cx + 15, cy - 12),                                         # 2: Mid neck S curve
            (cx + 17, cy - 5),                                          # 3: Forward dip
            (cx + 9, cy - 1),                                           # 4: Neck base
            (cx + 2, cy),                                               # 5: Chest
            (cx - 7, cy + 2),                                           # 6: Waist
            (cx - 15, cy + 3),                                          # 7: Hips
            (cx - 24, cy + 3),                                          # 8: Tail base
            (cx - 33, cy + 2 + int(math.sin(t*2*math.pi)*2.4)),         # 9: Tail mid
            (cx - 41, cy + int(math.sin(t*2*math.pi - 0.8)*3.4)),       # 10: Tail curve
            (cx - 48, cy - 3 + int(math.sin(t*2*math.pi - 1.6)*4.4)),   # 11: Tail whip
            (cx - 53, cy - 8 + int(math.sin(t*2*math.pi - 2.4)*5.2)),   # 12: Tail tip
        ]
        girth_t = [3.2 * scale, 3.4 * scale, 3.4 * scale, 3.8 * scale, 4.0 * scale, 4.4 * scale, 4.0 * scale, 3.4 * scale, 2.5 * scale, 1.8 * scale, 1.2 * scale, 0.8 * scale, 0.5]
        girth_b = [3.8 * scale, 4.0 * scale, 4.2 * scale, 4.6 * scale, 5.0 * scale, 6.2 * scale, 5.0 * scale, 4.2 * scale, 3.0 * scale, 2.0 * scale, 1.4 * scale, 0.8 * scale, 0.5]

    elif pal.get("neck_type") == "thick_dewlap":
        # Vhagar: Colossal neck with hanging wrinkled throat dewlap/wattle!
        spine = [
            (hx - 1, hy + 7),                                           # 0: Anchored in skull
            (cx + 14, cy - 7),
            (cx + 6, cy - 2),
            (cx, cy + 1),
            (cx - 8, cy + 2),
            (cx - 16, cy + 3),
            (cx - 24, cy + 3),
            (cx - 32, cy + 2 + int(math.sin(t*2*math.pi)*2.2)),
            (cx - 40, cy + int(math.sin(t*2*math.pi - 0.8)*3.2)),
            (cx - 47, cy - 3 + int(math.sin(t*2*math.pi - 1.6)*4.0)),
            (cx - 52, cy - 8 + int(math.sin(t*2*math.pi - 2.4)*4.8)),
        ]
        girth_t = [4.8 * scale, 4.6 * scale, 5.0 * scale, 6.2 * scale, 5.5 * scale, 4.4 * scale, 3.2 * scale, 2.4 * scale, 1.6 * scale, 1.0 * scale, 0.5]
        # Massive girth_b on neck and chest representing the hanging throat dewlap
        girth_b = [7.5 * scale, 8.2 * scale, 9.4 * scale, 11.0 * scale, 8.5 * scale, 5.8 * scale, 4.0 * scale, 2.8 * scale, 1.8 * scale, 1.1 * scale, 0.5]

    else:
        # Standard balanced wyvern spine
        spine = [
            (hx - 1, hy + 7),                                           # 0: Anchored in skull
            (cx + 14, cy - 8),                                          # 1: Mid neck
            (cx + 6, cy - 3),                                           # 2: Neck base
            (cx, cy + 1),                                               # 3: Chest peak
            (cx - 8, cy + 2),                                           # 4: Waist
            (cx - 16, cy + 3),                                          # 5: Hips
            (cx - 24, cy + 3),                                          # 6: Tail base
            (cx - 32, cy + 2 + int(math.sin(t*2*math.pi)*2.4)),         # 7: Tail mid
            (cx - 40, cy + int(math.sin(t*2*math.pi - 0.8)*3.4)),       # 8: Tail curve
            (cx - 47, cy - 3 + int(math.sin(t*2*math.pi - 1.6)*4.4)),   # 9: Tail whip
            (cx - 52, cy - 8 + int(math.sin(t*2*math.pi - 2.4)*5.2)),   # 10: Tail tip
        ]
        girth_t = [4.0 * scale, 3.8 * scale, 4.2 * scale, 5.5 * scale, 4.8 * scale, 3.8 * scale, 2.8 * scale, 2.0 * scale, 1.4 * scale, 0.9 * scale, 0.5]
        girth_b = [4.8 * scale, 4.5 * scale, 5.5 * scale, 8.5 * scale, 6.5 * scale, 4.8 * scale, 3.4 * scale, 2.4 * scale, 1.6 * scale, 1.0 * scale, 0.5]

    dense_top = []
    dense_bot = []
    
    for seg in range(len(spine) - 1):
        pA, pB = spine[seg], spine[seg + 1]
        gtA, gtB = girth_t[seg], girth_t[seg + 1]
        gbA, gbB = girth_b[seg], girth_b[seg + 1]
        
        for s in range(6):
            u = s / 6.0
            u_smooth = u * u * (3.0 - 2.0 * u)
            mx = pA[0] + (pB[0] - pA[0]) * u_smooth
            my = pA[1] + (pB[1] - pA[1]) * u_smooth
            dense_top.append((mx, my - (gtA + (gtB - gtA) * u_smooth)))
            dense_bot.append((mx, my + (gbA + (gbB - gbA) * u_smooth)))
            
    dense_top.append((spine[-1][0], spine[-1][1] - girth_t[-1]))
    dense_bot.append((spine[-1][0], spine[-1][1] + girth_b[-1]))

    hull_pts = [(int(round(pt[0])), int(round(pt[1]))) for pt in (dense_top + list(reversed(dense_bot)))]
    body_draw.polygon(hull_pts, fill=pal["scale_dark"])
    
    min_x = min(p[0] for p in hull_pts)
    max_x = max(p[0] for p in hull_pts)
    
    for scan_x in range(min_x, max_x + 1):
        y_inside = [sy for sy in range(CANVAS_H) if body_layer.getpixel((scan_x, sy))[3] > 0]
        if not y_inside:
            continue
        y_min, y_max = y_inside[0], y_inside[-1]
        span = max(1, y_max - y_min)
        
        for py in range(y_min, y_max + 1):
            rel = (py - y_min) / float(span)
            if rel < 0.22:
                col = pal["scale_hi"] if scan_x % 2 == 0 else pal["scale_mid"]
            elif rel < 0.58:
                col = pal["scale_mid"] if (scan_x + py) % 2 == 0 else pal["scale_dark"]
            elif rel < 0.72:
                col = pal["scale_dark"]
            else:
                if scan_x % 4 == 0:
                    col = pal["belly_line"]
                elif rel < 0.86:
                    col = pal["belly_hi"]
                else:
                    col = pal["belly_mid"]
            set_bp(scan_x, py, col)
            
        set_bp(scan_x, y_min, pal["scale_deep"])
        
        # Dorsal Spines along back
        if scan_x % 3 == 0 and cx - 46 < scan_x < cx + 18:
            set_bp(scan_x, y_min - 1, pal["horn_hi"])
            set_bp(scan_x, y_min - 2, pal["horn_mid"])
            set_bp(scan_x - 1, y_min - 1, pal["horn_dark"])
            
        set_bp(scan_x, y_max, pal["scale_deep"])

    # Moondancer Punk-Rock Mohawk Quill Crest down back of neck
    if pal.get("neck_type") == "mohawk_crest":
        for qx in range(cx + 8, cx + 22, 2):
            y_in = [sy for sy in range(CANVAS_H) if body_layer.getpixel((qx, sy))[3] > 0]
            if y_in:
                top_y = y_in[0]
                set_bp(qx, top_y - 1, pal["horn_hi"])
                set_bp(qx, top_y - 2, pal["horn_hi"])
                set_bp(qx, top_y - 3, pal["horn_mid"])
                set_bp(qx - 1, top_y - 1, pal["horn_dark"])

    # Tail Arrowhead Spade Tip
    tip_x, tip_y = int(round(spine[-1][0])), int(round(spine[-1][1]))
    for sx, sy, sc in [
        (tip_x - 1, tip_y, pal["horn_hi"]),
        (tip_x - 2, tip_y - 1, pal["horn_hi"]),
        (tip_x - 3, tip_y - 2, pal["horn_mid"]),
        (tip_x - 4, tip_y - 3, pal["horn_dark"]),
        (tip_x - 2, tip_y + 1, pal["horn_dark"]),
        (tip_x - 3, tip_y + 2, pal["horn_dark"]),
        (tip_x, tip_y - 1, pal["horn_mid"]),
        (tip_x, tip_y + 1, pal["horn_dark"]),
    ]:
        set_bp(sx, sy, sc)

    # 3B. NEAR HIND LEG (1st Leg, in foreground with sharp talons)
    hip_x, hip_y = cx - 17, cy + 4
    body_draw.polygon([
        (hip_x - 2, hip_y), (hip_x + 5, hip_y - 1), (hip_x + 4, hip_y + 8), (hip_x - 3, hip_y + 7)
    ], fill=pal["scale_dark"])
    set_bp(hip_x + 1, hip_y + 1, pal["scale_hi"])
    
    hock_x, hock_y = hip_x, hip_y + 8
    foot_x, foot_y = hock_x + 3, hock_y + 6
    body_draw.line([(hock_x, hock_y), (hock_x - 2, hock_y + 3), (foot_x, foot_y)], fill=pal["scale_dark"], width=2)
    set_bp(hock_x - 2, hock_y + 3, pal["scale_hi"])
    
    # Talons
    set_bp(foot_x + 2, foot_y + 1, COL_FANG_MID)
    set_bp(foot_x + 3, foot_y + 2, COL_FANG_HI)
    set_bp(foot_x + 4, foot_y + 3, COL_FANG_HI)
    set_bp(foot_x + 1, foot_y + 2, COL_FANG_MID)
    set_bp(foot_x + 2, foot_y + 3, COL_FANG_HI)
    set_bp(foot_x - 1, foot_y + 1, COL_FANG_DARK)
    set_bp(foot_x - 2, foot_y + 2, COL_FANG_HI)

    # Caraxes Leg Rudder Fin (Near leg)
    if pal.get("leg_type") == "rudder_fins":
        body_draw.polygon([
            (hock_x - 1, hock_y + 1),
            (hock_x - 8, hock_y - 2),
            (hock_x - 9, hock_y + 2),
            (hock_x - 3, hock_y + 4)
        ], fill=pal["web_hi"])
        body_draw.line([(hock_x - 1, hock_y + 1), (hock_x - 8, hock_y - 2)], fill=pal["wing_bone_hi"], width=1)
        body_draw.line([(hock_x - 1, hock_y + 2), (hock_x - 9, hock_y + 2)], fill=pal["wing_bone_hi"], width=1)
        set_bp(hock_x - 8, hock_y - 2, COL_FANG_HI)
        set_bp(hock_x - 9, hock_y + 2, COL_FANG_HI)

    master.paste(body_layer, (0, 0), body_layer)

    if not include_head:
        return master

    # =========================================================================
    # LAYER 4: HEAD, JAWS, HORNS, & THROAT LAYER (IN FRONT OF WINGS!)
    # =========================================================================
    head_layer = Image.new("RGBA", (CANVAS_W, CANVAS_H), (0, 0, 0, 0))
    head_draw = ImageDraw.Draw(head_layer)
    head_pix = head_layer.load()

    def set_hp(x, y, col):
        ix, iy = int(round(x)), int(round(y))
        if 0 <= ix < CANVAS_W and 0 <= iy < CANVAS_H:
            head_pix[ix, iy] = col

    # 4A. Upper Cranium Morphologies
    skull_type = pal.get("skull_type", "canine")
    
    if skull_type == "dino_heavy":
        # Vhagar, Vermithor: Colossal heavy boxy T. Rex armored cranium
        if pal["scale"] > 1.25:
            # Vhagar: Largest apex prehistoric skull
            head_draw.polygon([
                (hx - 3, hy + 8),
                (hx - 1, hy),
                (hx + 8, hy - 1),
                (hx + 17, hy - 1),
                (hx + 21, hy + 1),
                (hx + 22, hy + 6),
                (hx + 18, hy + 9),
                (hx + 4, hy + 9),
                (hx - 3, hy + 9)
            ], fill=pal["scale_mid"])
            for bx in range(hx + 2, hx + 19):
                set_hp(bx, hy, pal["scale_hi"])
                set_hp(bx, hy + 1, pal["scale_mid"])
            # Nostril
            set_hp(hx + 19, hy + 3, pal["scale_deep"])
            set_hp(hx + 20, hy + 4, pal["scale_dark"])
            # Heavy dinosaur brow
            set_hp(hx + 8, hy, pal["scale_dark"])
            set_hp(hx + 9, hy, pal["scale_deep"])
        else:
            # Vermithor: Armored boxy skull
            head_draw.polygon([
                (hx - 2, hy + 7),
                (hx, hy + 1),
                (hx + 7, hy),
                (hx + 15, hy),
                (hx + 18, hy + 2),
                (hx + 19, hy + 6),
                (hx + 16, hy + 8),
                (hx + 4, hy + 8),
                (hx - 2, hy + 8)
            ], fill=pal["scale_mid"])
            for bx in range(hx + 2, hx + 16):
                set_hp(bx, hy + 1, pal["scale_hi"])
                set_hp(bx, hy + 2, pal["scale_mid"])
            set_hp(hx + 16, hy + 3, pal["scale_deep"])
            set_hp(hx + 17, hy + 4, pal["scale_dark"])
            set_hp(hx + 7, hy + 1, pal["scale_dark"])
            set_hp(hx + 8, hy + 1, pal["scale_deep"])

    elif skull_type == "equine":
        # Syrax, Sunfyre, Meleys, Tessarion: Long, sleek, aristocratic equine skull
        head_draw.polygon([
            (hx - 2, hy + 5),
            (hx + 2, hy),
            (hx + 9, hy),
            (hx + 16, hy + 2),
            (hx + 20, hy + 4),
            (hx + 18, hy + 7),
            (hx + 5, hy + 7),
            (hx - 2, hy + 6)
        ], fill=pal["scale_mid"])
        for bx in range(hx + 3, hx + 13):
            set_hp(bx, hy + 1, pal["scale_hi"])
        set_hp(hx + 14, hy + 2, pal["scale_hi"])
        set_hp(hx + 15, hy + 3, pal["scale_hi"])
        # Nostril
        set_hp(hx + 18, hy + 4, pal["scale_deep"])

    elif skull_type == "underbite_feral":
        # Sheepstealer: Crooked feral snout with heavy underbite
        head_draw.polygon([
            (hx - 2, hy + 6),
            (hx + 1, hy + 1),
            (hx + 7, hy + 1),
            (hx + 13, hy + 2),
            (hx + 16, hy + 4),
            (hx + 15, hy + 6),
            (hx + 4, hy + 7),
            (hx - 2, hy + 7)
        ], fill=pal["scale_mid"])
        for bx in range(hx + 2, hx + 10):
            set_hp(bx, hy + 1, pal["scale_hi"])
        set_hp(hx + 14, hy + 3, pal["scale_deep"])

    elif skull_type == "raptor":
        # Moondancer: Compact sharp raptor cranium
        head_draw.polygon([
            (hx - 1, hy + 5),
            (hx + 2, hy + 1),
            (hx + 8, hy + 1),
            (hx + 14, hy + 2),
            (hx + 16, hy + 4),
            (hx + 14, hy + 6),
            (hx + 4, hy + 6),
            (hx - 1, hy + 6)
        ], fill=pal["scale_mid"])
        for bx in range(hx + 2, hx + 9):
            set_hp(bx, hy + 1, pal["scale_hi"])
        set_hp(hx + 13, hy + 3, pal["scale_deep"])

    else:
        # Standard Canine / Predatory Snout (Drogon, Rhaegal, Seasmoke, Silverwing)
        head_draw.polygon([
            (hx - 2, hy + 6),
            (hx + 1, hy + 1),
            (hx + 8, hy),
            (hx + 14, hy + 1),
            (hx + 17, hy + 2),
            (hx + 19, hy + 5),
            (hx + 16, hy + 7),
            (hx + 4, hy + 8),
            (hx - 2, hy + 7)
        ], fill=pal["scale_mid"])
        for bx in range(hx + 2, hx + 10):
            set_hp(bx, hy + 1, pal["scale_hi"])
        set_hp(hx + 11, hy + 2, pal["scale_hi"])
        set_hp(hx + 12, hy + 2, pal["scale_hi"])
        set_hp(hx + 16, hy + 3, pal["scale_deep"])
        set_hp(hx + 15, hy + 3, pal["scale_dark"])

    # Eye slit
    eye_x = hx + (7 if skull_type == "raptor" else 9)
    eye_y = hy + 3
    set_hp(eye_x, eye_y, pal["eye_glow"])
    set_hp(eye_x + 1, eye_y, pal["eye_core"])
    set_hp(eye_x + 1, eye_y + 1, (10, 0, 0, 255))
    set_hp(eye_x, eye_y + 1, pal["horn_dark"])
    
    # Frost aura on eye for Wight Viserion
    if pal.get("horn_type") == "broken_ram":
        set_hp(eye_x - 1, eye_y, (0, 200, 255, 180))
        set_hp(eye_x + 2, eye_y, (0, 200, 255, 180))
        set_hp(eye_x + 1, eye_y - 1, (0, 200, 255, 200))

    # Upper Fangs
    if skull_type == "dino_heavy" and pal["scale"] > 1.25:
        snout_front_x = hx + 21
    elif skull_type in ("dino_heavy", "equine", "canine"):
        snout_front_x = hx + 18
    else:
        snout_front_x = hx + 15

    set_hp(snout_front_x, hy + 6, COL_FANG_HI)
    set_hp(snout_front_x - 2, hy + 7, COL_FANG_HI)
    set_hp(snout_front_x - 4, hy + 7, COL_FANG_MID)
    if skull_type == "dino_heavy" and pal["scale"] > 1.25:
        set_hp(snout_front_x - 6, hy + 8, COL_FANG_MID)

    # 4B. Lower Jaw Morphologies
    jaw_hinge_x, jaw_hinge_y = hx + 4, hy + 8
    
    if skull_type == "underbite_feral":
        # Sheepstealer Underbite: lower jaw protrudes 2px PAST upper snout with fangs pointing UP!
        jaw_tip_x = hx + 19
        jaw_tip_y = hy + 8 + jaw_open
        head_draw.polygon([
            (jaw_hinge_x, jaw_hinge_y),
            (jaw_tip_x, jaw_tip_y),
            (jaw_tip_x - 1, jaw_tip_y + 3),
            (jaw_hinge_x - 1, jaw_hinge_y + 3)
        ], fill=pal["scale_dark"])
        # Protruding lower fangs pointing up
        set_hp(jaw_tip_x, jaw_tip_y - 1, COL_FANG_HI)
        set_hp(jaw_tip_x, jaw_tip_y - 2, COL_FANG_HI)
        set_hp(jaw_tip_x - 2, jaw_tip_y - 1, COL_FANG_MID)
        set_hp(jaw_tip_x - 4, jaw_tip_y - 1, COL_FANG_HI)

    else:
        jaw_tip_x = snout_front_x
        jaw_tip_y = hy + 8 + jaw_open
        head_draw.polygon([
            (jaw_hinge_x, jaw_hinge_y),
            (jaw_tip_x, jaw_tip_y),
            (jaw_tip_x - 1, jaw_tip_y + 3),
            (jaw_hinge_x - 1, jaw_hinge_y + 3)
        ], fill=pal["scale_dark"])
        set_hp(jaw_tip_x - 1, jaw_tip_y - 1, COL_FANG_HI)
        set_hp(jaw_tip_x - 3, jaw_tip_y - 1, COL_FANG_HI)
        set_hp(jaw_tip_x - 5, jaw_tip_y - 1, COL_FANG_MID)

    # Oral Cavity
    if jaw_open > 0:
        head_draw.polygon([
            (jaw_hinge_x + 1, jaw_hinge_y + 1),
            (jaw_hinge_x + 10, jaw_hinge_y + 2),
            (jaw_hinge_x + 10, jaw_hinge_y + jaw_open),
            (jaw_hinge_x + 1, jaw_hinge_y + jaw_open)
        ], fill=(90, 10, 20, 255))

    # -------------------------------------------------------------------------
    # 4C. SIGNATURE HORN MORPHOLOGIES PER DRAGON
    # -------------------------------------------------------------------------
    horn_type = pal.get("horn_type", "swept_back")

    if horn_type == "swept_back":
        # Drogon: Swept-back aggressive ramjet horns
        for i, (px, py) in enumerate([
            (hx, hy), (hx - 1, hy - 1), (hx - 2, hy - 1), (hx - 3, hy - 2),
            (hx - 4, hy - 2), (hx - 5, hy - 3), (hx - 6, hy - 3), (hx - 7, hy - 4),
            (hx - 8, hy - 4), (hx - 9, hy - 5), (hx - 10, hy - 5), (hx - 11, hy - 6)
        ]):
            prog = i / 12.0
            col = pal["horn_hi"] if prog < 0.4 else (pal["horn_mid"] if prog < 0.8 else pal["horn_dark"])
            set_hp(px, py, col)
            set_hp(px, py + 1, pal["horn_dark"])
        for px, py in [(hx - 1, hy + 3), (hx - 2, hy + 3), (hx - 3, hy + 3), (hx - 4, hy + 4), (hx - 5, hy + 4)]:
            set_hp(px, py, pal["horn_mid"])

    elif horn_type == "dual_forked":
        # Rhaegal: Dual forked horns
        for px, py, pc in [(hx, hy - 1, pal["horn_hi"]), (hx - 2, hy - 2, pal["horn_hi"]), (hx - 4, hy - 3, pal["horn_mid"]), (hx - 6, hy - 5, pal["horn_dark"]), (hx - 8, hy - 6, pal["horn_hi"])]:
            set_hp(px, py, pc)
            set_hp(px, py + 1, pal["horn_dark"])
        for px, py, pc in [(hx - 1, hy + 2, pal["horn_mid"]), (hx - 3, hy + 2, pal["horn_mid"]), (hx - 5, hy + 3, pal["horn_dark"]), (hx - 7, hy + 4, pal["horn_hi"])]:
            set_hp(px, py, pc)

    elif horn_type == "curved_down":
        # Viserion: Downward-curving ibex ram horns
        for px, py, pc in [
            (hx + 1, hy, pal["horn_hi"]), (hx - 1, hy + 1, pal["horn_hi"]), (hx - 3, hy + 1, pal["horn_mid"]),
            (hx - 4, hy + 3, pal["horn_mid"]), (hx - 5, hy + 5, pal["horn_dark"]), (hx - 6, hy + 7, pal["horn_hi"])
        ]:
            set_hp(px, py, pc)
            set_hp(px + 1, py, pal["horn_dark"])

    elif horn_type == "broken_ram":
        # Wight Viserion: Frost-fractured chipped horn
        for px, py, pc in [(hx + 1, hy, pal["horn_hi"]), (hx - 1, hy + 1, pal["horn_mid"]), (hx - 2, hy + 1, pal["horn_dark"]), (hx - 3, hy + 2, pal["scale_deep"])]:
            set_hp(px, py, pc)

    elif horn_type == "caraxes_frills":
        # Caraxes: Swept horns + cranial ear-fins/frills on skull!
        for px, py in [(hx, hy), (hx - 2, hy - 1), (hx - 4, hy - 2), (hx - 6, hy - 3), (hx - 8, hy - 4), (hx - 10, hy - 5)]:
            set_hp(px, py, pal["horn_hi"])
            set_hp(px, py + 1, pal["horn_dark"])
        # Ear Frills / Winglets flanking skull
        head_draw.polygon([
            (hx - 1, hy + 1),
            (hx - 5, hy - 2),
            (hx - 7, hy + 2),
            (hx - 3, hy + 4)
        ], fill=pal["web_hi"])
        head_draw.line([(hx - 1, hy + 1), (hx - 5, hy - 2)], fill=pal["scale_deep"], width=1)
        head_draw.line([(hx - 1, hy + 2), (hx - 7, hy + 2)], fill=pal["scale_deep"], width=1)

    elif horn_type == "meleys_crown":
        # Meleys: Circular crown / tiara of horns encircling back of cranium
        crown_spikes = [
            (hx - 3, hy - 4), (hx - 1, hy - 5), (hx + 2, hy - 5), (hx + 5, hy - 4),
            (hx + 8, hy - 2), (hx - 4, hy - 1), (hx - 5, hy + 2)
        ]
        for sx, sy in crown_spikes:
            set_hp(sx, sy, pal["horn_hi"])
            set_hp(sx, sy + 1, pal["horn_mid"])
            set_hp(sx - 1, sy + 1, pal["horn_dark"])

    elif horn_type == "sunfyre_equine":
        # Sunfyre, Syrax: Graceful crown of golden horns
        for px, py in [(hx + 1, hy - 1), (hx - 1, hy - 2), (hx - 3, hy - 3), (hx - 5, hy - 4), (hx - 7, hy - 5), (hx - 9, hy - 6)]:
            set_hp(px, py, pal["horn_hi"])
            set_hp(px, py + 1, pal["horn_dark"])
        # Nasal spikelet
        set_hp(hx + 13, hy, pal["horn_hi"])

    elif horn_type == "vermithor_beard":
        # Vermithor: Heavy horns PLUS chin beard spikes under jaw!
        for px, py in [(hx, hy - 1), (hx - 2, hy - 2), (hx - 4, hy - 3), (hx - 6, hy - 4), (hx - 8, hy - 5)]:
            set_hp(px, py, pal["horn_hi"])
            set_hp(px, py + 1, pal["horn_dark"])
        # Distinctive Chin Beard of Spikes under lower jaw
        beard_y = jaw_hinge_y + 3 + (jaw_open // 2)
        for bx in range(jaw_hinge_x + 1, jaw_hinge_x + 10, 2):
            set_hp(bx, beard_y, pal["horn_hi"])
            set_hp(bx, beard_y + 1, pal["horn_mid"])
            set_hp(bx, beard_y + 2, pal["horn_dark"])

    elif horn_type == "vhagar_ancient":
        # Vhagar: Ancient worn calcified blunt horns
        for px, py in [(hx, hy - 1), (hx - 1, hy - 2), (hx - 2, hy - 2), (hx - 4, hy - 3), (hx - 6, hy - 3), (hx - 7, hy - 4)]:
            set_hp(px, py, pal["horn_hi"])
            set_hp(px, py + 1, pal["horn_dark"])
            set_hp(px - 1, py, pal["horn_mid"])

    elif horn_type == "moondancer_crest":
        # Moondancer: Punk-rock mohawk quill crest
        for px, py in [(hx + 3, hy - 3), (hx, hy - 4), (hx - 3, hy - 4), (hx - 6, hy - 3)]:
            set_hp(px, py, pal["horn_hi"])
            set_hp(px, py + 1, pal["horn_mid"])

    elif horn_type == "tessarion_crest":
        # Tessarion: Cobalt & copper royal crest
        for px, py in [(hx + 1, hy - 1), (hx - 1, hy - 2), (hx - 3, hy - 3), (hx - 6, hy - 4), (hx - 8, hy - 5)]:
            set_hp(px, py, pal["horn_hi"])
            set_hp(px, py + 1, pal["horn_dark"])

    elif horn_type == "seasmoke_goatee":
        # Seasmoke: Slate horns with goatee spike under chin
        for px, py in [(hx, hy - 1), (hx - 2, hy - 2), (hx - 4, hy - 3), (hx - 7, hy - 4)]:
            set_hp(px, py, pal["horn_hi"])
            set_hp(px, py + 1, pal["horn_dark"])
        # Chin goatee spike
        set_hp(jaw_hinge_x + 8, jaw_hinge_y + 3 + jaw_open, pal["horn_hi"])
        set_hp(jaw_hinge_x + 8, jaw_hinge_y + 4 + jaw_open, pal["horn_mid"])

    elif horn_type == "silverwing_curved":
        # Silverwing: Smooth elegant curved silver horns
        for px, py in [(hx + 1, hy - 1), (hx - 1, hy - 2), (hx - 3, hy - 2), (hx - 5, hy - 1), (hx - 7, hy)]:
            set_hp(px, py, pal["horn_hi"])
            set_hp(px, py + 1, pal["horn_mid"])

    elif horn_type == "sheepstealer_crooked":
        # Sheepstealer: Crooked, broken, asymmetric feral horns
        for px, py in [(hx, hy - 1), (hx - 1, hy - 3), (hx - 3, hy - 3), (hx - 4, hy - 5), (hx - 5, hy - 4)]:
            set_hp(px, py, pal["horn_hi"])
            set_hp(px, py + 1, pal["horn_dark"])

    master.paste(head_layer, (0, 0), head_layer)

    # =========================================================================
    # LAYER 5: DRAGONFIRE / PLASMA BEAM LAYER (Frames 3, 4, 5)
    # =========================================================================
    if wing_phase in (3, 4, 5):
        fire_layer = Image.new("RGBA", (CANVAS_W, CANVAS_H), (0, 0, 0, 0))
        fire_draw = ImageDraw.Draw(fire_layer)
        fire_pix = fire_layer.load()

        def set_fp(x, y, col):
            ix, iy = int(round(x)), int(round(y))
            if 0 <= ix < CANVAS_W and 0 <= iy < CANVAS_H:
                fire_pix[ix, iy] = col

        mouth_x = hx + 18 if (pal.get("skull_type") == "dino_heavy" and pal["scale"] > 1.25) else hx + 14
        mouth_y = hy + 8 + (jaw_open // 2)
        flame_len = 46  # Fully unclipped!
        breath_type = pal.get("breath_type", "fire_red")
        
        if breath_type == "plasma_beam":
            # Wight Viserion: Piercing laser beam
            beam_len = 48
            fire_draw.line([(mouth_x, mouth_y), (mouth_x + beam_len, mouth_y)], fill=pal["fire_outer"], width=5)
            fire_draw.line([(mouth_x, mouth_y), (mouth_x + beam_len, mouth_y)], fill=pal["fire_inner"], width=3)
            fire_draw.line([(mouth_x + 1, mouth_y), (mouth_x + beam_len - 2, mouth_y)], fill=pal["fire_core"], width=1)
            for sx, sy in [
                (mouth_x + beam_len + 1, mouth_y - 2), (mouth_x + beam_len + 2, mouth_y),
                (mouth_x + beam_len + 1, mouth_y + 2), (mouth_x + beam_len + 3, mouth_y - 1)
            ]:
                set_fp(sx, sy, pal["fire_inner"])
                set_fp(sx + 1, sy, pal["fire_core"])

        elif breath_type == "fire_volcano":
            # Vhagar, Vermithor: Massive billowing apocalyptic inferno
            v_len = 50
            fire_draw.polygon([
                (mouth_x, mouth_y),
                (mouth_x + 10, mouth_y - 11),
                (mouth_x + 24, mouth_y - 16),
                (mouth_x + v_len + 4, mouth_y - 8),
                (mouth_x + v_len + 8, mouth_y + 3),
                (mouth_x + v_len + 3, mouth_y + 12),
                (mouth_x + 24, mouth_y + 16),
                (mouth_x + 10, mouth_y + 9),
            ], fill=pal["fire_outer"])
            
            fire_draw.polygon([
                (mouth_x, mouth_y),
                (mouth_x + 8, mouth_y - 8),
                (mouth_x + 20, mouth_y - 11),
                (mouth_x + v_len - 2, mouth_y - 3),
                (mouth_x + v_len + 2, mouth_y + 3),
                (mouth_x + 18, mouth_y + 10),
                (mouth_x + 8, mouth_y + 6),
            ], fill=pal["fire_mid"])
            
            fire_draw.polygon([
                (mouth_x, mouth_y),
                (mouth_x + 8, mouth_y - 4),
                (mouth_x + 18, mouth_y - 5),
                (mouth_x + 28, mouth_y),
                (mouth_x + 16, mouth_y + 5),
                (mouth_x + 7, mouth_y + 3),
            ], fill=pal["fire_inner"])
            
            # Radiant diamond core
            fire_draw.polygon([
                (mouth_x - 1, mouth_y),
                (mouth_x + 6, mouth_y - 2),
                (mouth_x + 18, mouth_y),
                (mouth_x + 6, mouth_y + 2)
            ], fill=pal["fire_core"])
            for ex, ey in [
                (mouth_x + v_len + 8, mouth_y - 5), (mouth_x + v_len + 14, mouth_y + 2),
                (mouth_x + v_len + 10, mouth_y + 8), (mouth_x + 30, mouth_y - 17), (mouth_x + 32, mouth_y + 17)
            ]:
                set_fp(ex, ey, pal["fire_ember"])

        else:
            # Standard & specialized color flamethrower plumes
            fire_draw.polygon([
                (mouth_x, mouth_y),
                (mouth_x + 9, mouth_y - 8),
                (mouth_x + 22, mouth_y - 13),
                (mouth_x + flame_len + 4, mouth_y - 5),
                (mouth_x + flame_len + 8, mouth_y + 2),
                (mouth_x + flame_len + 3, mouth_y + 9),
                (mouth_x + 22, mouth_y + 13),
                (mouth_x + 9, mouth_y + 7),
            ], fill=pal["fire_outer"])
            
            fire_draw.polygon([
                (mouth_x, mouth_y),
                (mouth_x + 7, mouth_y - 6),
                (mouth_x + 18, mouth_y - 9),
                (mouth_x + flame_len - 2, mouth_y - 2),
                (mouth_x + flame_len + 2, mouth_y + 2),
                (mouth_x + 16, mouth_y + 8),
                (mouth_x + 7, mouth_y + 5),
            ], fill=pal["fire_mid"])
            
            fire_draw.polygon([
                (mouth_x, mouth_y),
                (mouth_x + 7, mouth_y - 3),
                (mouth_x + 16, mouth_y - 4),
                (mouth_x + 24, mouth_y),
                (mouth_x + 14, mouth_y + 4),
                (mouth_x + 6, mouth_y + 2),
            ], fill=pal["fire_inner"])
            
            # Incandescent core
            fire_draw.polygon([
                (mouth_x - 1, mouth_y),
                (mouth_x + 5, mouth_y - 2),
                (mouth_x + 15, mouth_y),
                (mouth_x + 5, mouth_y + 2)
            ], fill=pal["fire_core"])
            
            for ex, ey in [
                (mouth_x + flame_len + 8, mouth_y - 4),
                (mouth_x + flame_len + 14, mouth_y + 1),
                (mouth_x + flame_len + 9, mouth_y + 7),
                (mouth_x + 26, mouth_y - 14),
                (mouth_x + 30, mouth_y + 14)
            ]:
                set_fp(ex, ey, pal["fire_ember"])
                set_fp(ex + 1, ey, pal["fire_inner"])

        master.paste(fire_layer, (0, 0), fire_layer)

    # Ambient particles for undead/wild
    if pal.get("wing_style") == "tattered":
        rng = random.Random(frame_idx * 37)
        for _ in range(8):
            px = rng.randint(cx - 24, cx + 25)
            py = rng.randint(cy - 12, cy + 22)
            ix, iy = int(px), int(py)
            if 0 <= ix < CANVAS_W and 0 <= iy < CANVAS_H and master.getpixel((ix, iy))[3] == 0:
                p_col = (0, 220, 255, rng.randint(80, 180)) if pal.get("horn_type") == "broken_ram" else (140, 100, 60, rng.randint(60, 140))
                master.putpixel((ix, iy), p_col)

    return master

def render_egg(pal):
    """Render a master 16-bit pixel art dragon egg customized with the dragon's colors."""
    egg_w, egg_h = 48, 48
    egg = Image.new("RGBA", (egg_w, egg_h), (0, 0, 0, 0))
    draw = ImageDraw.Draw(egg)
    pix = egg.load()

    cx, cy = 24, 24
    rx, ry = 11, 15

    # Draw egg oval
    draw.ellipse([cx - rx, cy - ry, cx + rx, cy + ry], fill=pal["scale_dark"], outline=pal["scale_deep"])
    
    # Scale texture and highlights
    for y in range(cy - ry, cy + ry + 1):
        for x in range(cx - rx, cx + rx + 1):
            if egg.getpixel((x, y))[3] > 0:
                # Relative distance from top-left light source
                dx = (x - (cx - 4)) / float(rx)
                dy = (y - (cy - 6)) / float(ry)
                dist = math.sqrt(dx * dx + dy * dy)
                
                # Diamond scale pattern
                pattern = (x * 2 + y * 3) % 7
                if dist < 0.45:
                    col = pal["scale_hi"] if pattern in (0, 1) else pal["scale_mid"]
                elif dist < 0.85:
                    col = pal["scale_mid"] if pattern in (0, 1, 2) else pal["scale_dark"]
                else:
                    col = pal["scale_dark"] if pattern == 0 else pal["scale_deep"]
                
                # Vein / accent glow
                if (x + y) % 9 == 0 and dist < 0.9:
                    col = pal.get("web_glow", pal["horn_hi"])
                
                pix[x, y] = col

    # Ambient glowing embers around base
    for ex, ey, ec in [
        (cx - 8, cy + ry - 2, pal["fire_ember"]),
        (cx + 7, cy + ry - 3, pal["fire_ember"]),
        (cx - 12, cy + ry, pal["fire_mid"]),
        (cx + 11, cy + ry, pal["fire_mid"]),
    ]:
        if 0 <= ex < egg_w and 0 <= ey < egg_h:
            pix[ex, ey] = ec

    return egg.resize((egg_w * 2, egg_h * 2), Image.Resampling.NEAREST)

def render_hatchling(pal, frame_idx=0, total_frames=4):
    """Render an animated cute baby wyvern / hatchling stage."""
    hw, hh = 64, 52
    hatch = Image.new("RGBA", (hw, hh), (0, 0, 0, 0))
    draw = ImageDraw.Draw(hatch)
    pix = hatch.load()

    def set_hp(x, y, col):
        ix, iy = int(round(x)), int(round(y))
        if 0 <= ix < hw and 0 <= iy < hh:
            pix[ix, iy] = col

    bob = 1 if frame_idx in (1, 3) else 0
    mouth_open = 1 if frame_idx == 2 else 0
    wing_up = frame_idx in (1, 2)

    bx = 28
    by = 32 + bob

    # 1. Tail curled on floor
    draw.line([(bx - 8, by + 4), (bx - 14, by + 4), (bx - 18, by + 1), (bx - 20, by - 2)], fill=pal["scale_dark"], width=2)
    set_hp(bx - 21, by - 3, pal["horn_hi"])
    set_hp(bx - 20, by - 4, pal["horn_mid"])

    # 2. Far Hind Leg & Far Wing
    set_hp(bx - 4, by + 6, pal["scale_deep"])
    set_hp(bx - 3, by + 7, pal["scale_deep"])
    set_hp(bx - 2, by + 7, COL_FANG_MID)

    # Far Wing
    far_wy = by - 12 if wing_up else by - 6
    draw.polygon([(bx - 2, by - 2), (bx - 8, far_wy), (bx - 2, far_wy - 4)], fill=pal["web_dark"])
    draw.line([(bx - 2, by - 2), (bx - 8, far_wy), (bx - 2, far_wy - 4)], fill=pal["scale_deep"], width=1)

    # 3. Chubby Baby Torso & Segmented Belly
    draw.ellipse([bx - 6, by - 5, bx + 6, by + 6], fill=pal["scale_mid"], outline=pal["scale_deep"])
    for px in range(bx - 3, bx + 4):
        set_hp(px, by - 4, pal["scale_hi"])
    for px in range(bx, bx + 5):
        set_hp(px, by + 2, pal["belly_hi"])
        set_hp(px, by + 3, pal["belly_mid"])
        set_hp(px, by + 4, pal["belly_dark"])

    # 4. Near Hind Leg & Talons
    draw.line([(bx - 1, by + 3), (bx - 1, by + 7), (bx + 2, by + 7)], fill=pal["scale_dark"], width=2)
    set_hp(bx + 2, by + 6, COL_FANG_HI)
    set_hp(bx + 3, by + 7, COL_FANG_HI)

    # 5. Near Wing (flapping baby wing)
    near_wy = by - 15 if wing_up else by - 8
    draw.polygon([(bx + 1, by - 3), (bx - 6, near_wy), (bx + 2, near_wy - 6), (bx + 8, by - 1)], fill=pal["web_mid"])
    draw.line([(bx + 1, by - 3), (bx - 6, near_wy), (bx + 2, near_wy - 6)], fill=pal["wing_bone_hi"], width=1)
    draw.line([(bx - 6, near_wy), (bx + 8, by - 1)], fill=pal["wing_bone_mid"], width=1)
    set_hp(bx - 6, near_wy, COL_FANG_HI)

    # 6. Baby Head with Big Curious Eye
    hx, hy = bx + 5, by - 10
    draw.polygon([
        (hx - 2, hy + 4),
        (hx, hy),
        (hx + 6, hy),
        (hx + 10, hy + 2),
        (hx + 10, hy + 6),
        (hx + 4, hy + 7),
        (hx - 2, hy + 6)
    ], fill=pal["scale_mid"])
    for px in range(hx + 1, hx + 7):
        set_hp(px, hy, pal["scale_hi"])

    # Hornlets
    set_hp(hx, hy - 1, pal["horn_hi"])
    set_hp(hx - 1, hy - 2, pal["horn_mid"])
    set_hp(hx - 2, hy - 2, pal["horn_dark"])

    # Big Expressive Baby Slit Eye
    set_hp(hx + 4, hy + 2, pal["eye_glow"])
    set_hp(hx + 5, hy + 2, pal["eye_core"])
    set_hp(hx + 5, hy + 3, (10, 0, 0, 255))
    set_hp(hx + 4, hy + 3, pal["eye_glow"])

    # Cute tiny fangs & open chirping mouth
    set_hp(hx + 9, hy + 4, COL_FANG_HI)
    if mouth_open:
        set_hp(hx + 8, hy + 6, (120, 15, 20, 255))
        set_hp(hx + 9, hy + 7, pal["scale_dark"])
        set_hp(hx + 10, hy + 6, COL_FANG_HI)
        # Tiny cute baby smoke / spark puff
        spark_x = hx + 12
        spark_y = hy + 4
        set_hp(spark_x, spark_y, pal["fire_core"])
        set_hp(spark_x + 1, spark_y - 1, pal["fire_inner"])
        set_hp(spark_x + 2, spark_y, pal["fire_mid"])
        set_hp(spark_x + 3, spark_y - 2, pal["fire_ember"])
        set_hp(spark_x + 1, spark_y + 1, pal["fire_outer"])
    else:
        set_hp(hx + 8, hy + 6, pal["scale_dark"])
        set_hp(hx + 9, hy + 5, COL_FANG_HI)

    return hatch.resize((hw * 2, hh * 2), Image.Resampling.NEAREST)

def build_all():
    out_dir = os.path.join(os.path.dirname(__file__), "dragons")
    os.makedirs(out_dir, exist_ok=True)
    print(f"Building Complete Master Pixel Art Wyvern System (15 Canon Series Dragons) in: {out_dir}")

    for key, pal in PALETTES.items():
        print(f" -> Rendering master sprite [{pal['series']}]: {pal['name']}...")
        frames = []
        for f in range(8):
            f_img = render_wyvern(pal, f, 8)
            upscaled = f_img.resize((CANVAS_W * UPSCALE, CANVAS_H * UPSCALE), Image.Resampling.NEAREST)
            frames.append(upscaled)
            
        gif_path = os.path.join(out_dir, f"{key}.gif")
        frames[0].save(
            gif_path,
            save_all=True,
            append_images=frames[1:],
            duration=125,
            loop=0,
            disposal=2
        )

        png_path = os.path.join(out_dir, f"{key}.png")
        frames[4].save(png_path)  # Frame 4 has full fire eruption

        # Render Egg
        egg_img = render_egg(pal)
        egg_path = os.path.join(out_dir, f"{key}_egg.png")
        egg_img.save(egg_path)

        # Render Hatchling / Baby Wyvern
        hatch_frames = []
        for hf in range(4):
            h_img = render_hatchling(pal, hf, 4)
            hatch_frames.append(h_img)
        hatch_gif_path = os.path.join(out_dir, f"{key}_hatchling.gif")
        hatch_frames[0].save(
            hatch_gif_path,
            save_all=True,
            append_images=hatch_frames[1:],
            duration=160,
            loop=0,
            disposal=2
        )
        hatch_png_path = os.path.join(out_dir, f"{key}_hatchling.png")
        hatch_frames[2].save(hatch_png_path)  # Frame 2 has cute spark puff
        
        print(f"    Saved: {key}.gif ({os.path.getsize(gif_path):,} B) | {key}_egg.png | {key}_hatchling.gif")

    print(f"\nSuccessfully built all 15 master wyverns + eggs across Game of Thrones & House of the Dragon!")

if __name__ == "__main__":
    build_all()
