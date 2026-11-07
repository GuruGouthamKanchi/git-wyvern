"""
Git-Wyvern: Dynamic GitHub Dragon Evolution Engine
Replaces git-koi with authentic Game of Thrones & House of the Dragon wyverns!
Calculates GitHub activity to evolve your wyvern companion through 3 biological stages:
- Stage 1: Dragon Egg (0 - 29 commits)
- Stage 2: Hatchling Wyvern (30 - 99 commits)
- Stage 3: Apex Soaring Dread (100+ commits)
"""

import os
import sys
import json
import shutil
import urllib.request
import re

DRAGON_METADATA = {
    "drogon": {
        "name": "Drogon",
        "title": "The Black Dread Reborn",
        "series": "Game of Thrones",
        "rider": "Daenerys Targaryen",
        "faction": "House Targaryen",
        "faction_icon": "🔴",
        "breath": "Blood-Red Dragonfire",
        "desc": "The largest and most ferocious of Daenerys's brood. Pitch-black hide with blood-red wing ribbing.",
    },
    "rhaegal": {
        "name": "Rhaegal",
        "title": "The Jade Dragon",
        "series": "Game of Thrones",
        "rider": "Jon Snow / Aegon Targaryen",
        "faction": "House Targaryen",
        "faction_icon": "🟢",
        "breath": "Emerald-Green Flame",
        "desc": "Deep jade-green scales with burnished bronze-gold wing veins and dual forked horns.",
    },
    "viserion": {
        "name": "Viserion",
        "title": "The Gold Dragon",
        "series": "Game of Thrones",
        "rider": "Daenerys Targaryen",
        "faction": "House Targaryen",
        "faction_icon": "🟡",
        "breath": "Golden-White Dragonfire",
        "desc": "Cream and ivory-white hide with golden wing webs and downward-curving ram horns.",
    },
    "wight_viserion": {
        "name": "Wight Viserion",
        "title": "The Undead Ice Dragon",
        "series": "Game of Thrones",
        "rider": "The Night King",
        "faction": "Army of the Dead",
        "faction_icon": "❄️",
        "breath": "Cobalt Plasma Laser Beam",
        "desc": "Rotting slate-grey corpse with tattered holed wings, ice-blue glowing eyes, and shattering plasma.",
    },
    "caraxes": {
        "name": "Caraxes",
        "title": "The Blood Wyrm",
        "series": "House of the Dragon",
        "rider": "Daemon Targaryen",
        "faction": "The Blacks (Team Rhaenyra)",
        "faction_icon": "⚫",
        "breath": "Blistering Crimson Fire",
        "desc": "Mutated long serpentine S-curve neck, cranial ear-frills, and webbed rudder-fins on hind ankles.",
    },
    "vhagar": {
        "name": "Vhagar",
        "title": "The Queen of All Dragons",
        "series": "House of the Dragon",
        "rider": "Aemond Targaryen",
        "faction": "The Greens (Team Aegon)",
        "faction_icon": "🟢",
        "breath": "Cataclysmic Volcanic Inferno",
        "desc": "Colossal 180-year-old behemoth. Mossy bronze-green hide, blunt dino skull, and sagging throat dewlap.",
    },
    "meleys": {
        "name": "Meleys",
        "title": "The Red Queen",
        "series": "House of the Dragon",
        "rider": "Rhaenys Targaryen",
        "faction": "The Blacks (Team Rhaenyra)",
        "faction_icon": "⚫",
        "breath": "Rapid Scarlet Plumes",
        "desc": "Fastest dragon in the realm. Scarlet-red scales with soft pink wings and a circular crown of horns.",
    },
    "sunfyre": {
        "name": "Sunfyre",
        "title": "Sunfyre the Golden",
        "series": "House of the Dragon",
        "rider": "Aegon II Targaryen",
        "faction": "The Greens (Team Aegon)",
        "faction_icon": "🟢",
        "breath": "Shimmering White-Gold Flame",
        "desc": "The most beautiful dragon ever seen. Dazzling metallic gold scales with pale rose-pink wings.",
    },
    "vermithor": {
        "name": "Vermithor",
        "title": "The Bronze Fury",
        "series": "House of the Dragon",
        "rider": "Hugh Hammer / Jaehaerys I",
        "faction": "The Blacks (Team Rhaenyra)",
        "faction_icon": "⚫",
        "breath": "Boiling Bronze Furnace Fire",
        "desc": "Colossal second-largest dragon. Mottled bronze-tan hide with distinctive chin beard of spikes.",
    },
    "moondancer": {
        "name": "Moondancer",
        "title": "The Pale Raptor",
        "series": "House of the Dragon",
        "rider": "Baela Targaryen",
        "faction": "The Blacks (Team Rhaenyra)",
        "faction_icon": "⚫",
        "breath": "High-Velocity Lime Flame",
        "desc": "Slender, hyper-agile aquamarine juvenile with an upright punk-rock mohawk quill crest.",
    },
    "tessarion": {
        "name": "Tessarion",
        "title": "The Blue Queen",
        "series": "House of the Dragon",
        "rider": "Daeron Targaryen",
        "faction": "The Greens (Team Aegon)",
        "faction_icon": "🟢",
        "breath": "Cobalt Blue Dragonfire",
        "desc": "Dark cobalt blue scales with rich beaten copper wing membranes, belly plates, and horns.",
    },
    "seasmoke": {
        "name": "Seasmoke",
        "title": "The Silver Sentinel",
        "series": "House of the Dragon",
        "rider": "Addam of Hull / Laenor Velaryon",
        "faction": "The Blacks (Team Rhaenyra)",
        "faction_icon": "⚫",
        "breath": "Golden-Yellow Flame Plumes",
        "desc": "Athletic smoky silver-grey dragon with translucent slate wings and a sharp chin goatee horn.",
    },
    "silverwing": {
        "name": "Silverwing",
        "title": "The Gentle Queen",
        "series": "House of the Dragon",
        "rider": "Ulf White / Queen Alysanne",
        "faction": "The Blacks (Team Rhaenyra)",
        "faction_icon": "⚫",
        "breath": "Iridescent Silver-White Flame",
        "desc": "Shimmering pearl-silver scales, silky white wings, smooth curved horns, and calm temperament.",
    },
    "sheepstealer": {
        "name": "Sheepstealer",
        "title": "The Wild Scavenger",
        "series": "House of the Dragon",
        "rider": "Rhaena Targaryen",
        "faction": "The Blacks (Team Rhaenyra)",
        "faction_icon": "⚫",
        "breath": "Smoky Dirt-Orange Flame",
        "desc": "Mud-brown feral wyvern with heavy protruding underbite, crooked horns, and ragged wings.",
    },
    "syrax": {
        "name": "Syrax",
        "title": "The Golden Queen",
        "series": "House of the Dragon",
        "rider": "Queen Rhaenyra Targaryen",
        "faction": "The Blacks (Team Rhaenyra)",
        "faction_icon": "⚫",
        "breath": "Golden-Yellow Stream",
        "desc": "Radiant sulfur-yellow scales, cream wings, backward crown crest, and noble equine skull.",
    },
}

def get_git_commit_count():
    """Get commit count from local git or fallback."""
    try:
        import subprocess
        res = subprocess.run(["git", "rev-list", "--count", "HEAD"], capture_output=True, text=True)
        if res.returncode == 0:
            return int(res.stdout.strip())
        return 0
    except Exception:
        return 0

def get_evolution_stage(commits):
    """Returns biological evolution stage based on commit activity (3 stages)."""
    if commits < 30:
        return {
            "stage_id": 1,
            "title": "Stage 1: Dragon Egg",
            "icon": "🥚",
            "suffix": "_stage1.gif",
            "static_suffix": "_stage1.png",
            "is_animated": True,
            "power": commits * 35,
            "progress_pct": min(100, int((commits / 30.0) * 100)),
            "next_threshold": 30,
            "desc": "Incubating in volcanic embers. Overlapping petrified archosaurian scales pulse with incandescent magma veins."
        }
    elif commits < 100:
        return {
            "stage_id": 2,
            "title": "Stage 2: Hatchling Wyvern",
            "icon": "🐣",
            "suffix": "_stage2.gif",
            "static_suffix": "_stage2.png",
            "is_animated": True,
            "power": 1050 + (commits - 30) * 55,
            "progress_pct": min(100, int(((commits - 30) / 70.0) * 100)),
            "next_threshold": 100,
            "desc": "Freshly hatched young wyvern perched on volcanic crags among broken eggshells. Flapping wings, testing talons, and puffing smoke."
        }
    else:
        return {
            "stage_id": 3,
            "title": "Stage 3: Apex Soaring Dread",
            "icon": "🐉",
            "suffix": "_stage3.gif",
            "static_suffix": "_stage3.png",
            "is_animated": True,
            "power": 4900 + (commits - 100) * 100,
            "progress_pct": 100,
            "next_threshold": None,
            "desc": "Full colossal apex predator with unclipped wingspan, predatory fangs, quadrate jaw hinge, and soaring aerial hunt with roaring dragonfire."
        }

def make_progress_bar(pct, length=16):
    filled = int(round(length * (pct / 100.0)))
    filled = max(0, min(length, filled))
    return "█" * filled + "░" * (length - filled)

def update_readme(repo_dir, chosen_dragon_key, commits, mode="patrol"):
    meta = DRAGON_METADATA.get(chosen_dragon_key, DRAGON_METADATA["drogon"])
    stage = get_evolution_stage(commits)
    
    dragons_dir = os.path.join(repo_dir, "dragons")
    
    # Wyverns always operate in 640x640 Aerial Patrol mode!
    if stage["stage_id"] >= 2:
        src_file = os.path.join(dragons_dir, f"{chosen_dragon_key}_patrol.gif") if stage["stage_id"] == 3 else os.path.join(dragons_dir, f"{chosen_dragon_key}_stage2_patrol.gif")
        if not os.path.exists(src_file):
            src_file = os.path.join(dragons_dir, f"{chosen_dragon_key}_patrol.gif")
        img_width = "420"
        flight_title = f"{stage['title']} (Aerial Patrol)"
    else:
        src_file = os.path.join(dragons_dir, f"{chosen_dragon_key}_stage1.gif")
        img_width = "352"
        flight_title = stage['title']
    
    if not os.path.exists(src_file):
        src_file = os.path.join(dragons_dir, f"{chosen_dragon_key}.gif")
    
    companion_file = os.path.join(dragons_dir, "companion.gif")
    if os.path.exists(src_file):
        shutil.copyfile(src_file, companion_file)
        rel_companion_path = "dragons/companion.gif"
    else:
        rel_companion_path = f"dragons/{chosen_dragon_key}_patrol.gif" if stage["stage_id"] >= 2 else f"dragons/{chosen_dragon_key}_stage1.gif"

    bar = make_progress_bar(stage["progress_pct"])
    next_info = f"({stage['progress_pct']}% to Stage {stage['stage_id'] + 1})" if stage["next_threshold"] else "(Max Apex Reached!)"

    wyvern_block = f"""<!-- START_SECTION:wyvern -->
<div align="center">

### {meta['faction_icon']} Active Companion: **{meta['name']}** — *{meta['title']}*

<a href="https://github.com/topics/game-of-thrones">
  <img src="{rel_companion_path}" alt="{meta['name']} Wyvern" width="{img_width}"/>
</a>

```text
┌────────────────────────────────────────────────────────────────────────┐
│  🐲 Companion   : {meta['name']:<14} Series  : {meta['series']:<26}│
│  👑 Bonded Rider: {meta['rider']:<14} Faction : {meta['faction']:<26}│
│  🔥 Breath Type : {meta['breath']:<48}│
│  ✨ Growth Stage: {flight_title:<48}│
│  ⚡ Power Level : {stage['power']:<5} XP        Commits : {commits:<5} ({stage['progress_pct']:>3}% to Next Level) │
│  📈 Growth Bar  : [{bar}] {next_info:<27}│
└────────────────────────────────────────────────────────────────────────┘
```
> *"{meta['desc']}"*

</div>
<!-- END_SECTION:wyvern -->"""

    readme_path = os.path.join(repo_dir, "README.md")
    if os.path.exists(readme_path):
        with open(readme_path, "r", encoding="utf-8") as f:
            content = f.read()

        if "<!-- START_SECTION:wyvern -->" in content and "<!-- END_SECTION:wyvern -->" in content:
            new_content = re.sub(
                r"<!-- START_SECTION:wyvern -->.*?<!-- END_SECTION:wyvern -->",
                wyvern_block,
                content,
                flags=re.DOTALL
            )
        else:
            new_content = wyvern_block + "\n\n" + content

        with open(readme_path, "w", encoding="utf-8") as f:
            f.write(new_content)
        print(f"Updated README.md with {meta['name']} ({flight_title})!")
    else:
        with open(readme_path, "w", encoding="utf-8") as f:
            f.write(wyvern_block)
        print(f"Created README.md with {meta['name']} ({flight_title})!")

def main():
    repo_dir = os.path.abspath(os.path.join(os.path.dirname(__file__)))
    
    # Selected dragon via env or argument (default: drogon)
    dragon_key = os.environ.get("WYVERN_DRAGON", "drogon").lower()
    if len(sys.argv) > 1:
        dragon_key = sys.argv[1].lower()

    if dragon_key not in DRAGON_METADATA:
        dragon_key = "drogon"

    # Flight mode (patrol vs hover, default: patrol)
    mode = os.environ.get("WYVERN_MODE", "patrol").lower()
    if len(sys.argv) > 2:
        mode = sys.argv[2].lower()

    # Commit count
    commits_env = os.environ.get("WYVERN_COMMITS")
    if commits_env and commits_env.isdigit():
        commits = int(commits_env)
    else:
        commits = get_git_commit_count()

    print(f"🐉 Git-Wyvern Engine running for: {dragon_key} [Mode: {mode}] with {commits} commits")
    update_readme(repo_dir, dragon_key, commits, mode=mode)

if __name__ == "__main__":
    main()
