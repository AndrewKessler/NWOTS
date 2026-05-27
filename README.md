Nine_Worlds_Of_The_SUN (NWOTS) BASE ENGINE
------------------------------------------

A Retro FPS Experiment in Modding, Ownership, and Cryptographic Gameplay

VISION
------

Minerva Base (Episode one of NWOTS) is an open-source retro first-person shooter framework inspired by:
Doom, Quake, Hexen, classic LAN shooters
and strongly supporting moddable PC gaming culture

The goal is NOT simply to create another FPS. The goal is to build the most moddable FPS architecture possible.


CORE IDEAS
----------

EVERYTHING IS DATA-DRIVEN

Weapons, enemies, maps, HUDs, episodes, game rules, inventory systems,
teleporters, effects, sounds, and future game modes should all be editable
without recompiling the engine.

Non-coders should be able to:
create weapons, create enemies, create maps, create mods, create total conversions, create coop campaigns...
create entirely new genres

using simple text files and assets.


CRYPTOGRAPHIC OWNERSHIP

Players own their items cryptographically.

Content creators own their creations cryptographically.

Weapons, maps, mods, cosmetics, records, and achievements can be:
signed, verified, transferred, attributed and persisted.

without relying on centralized systems.

The long-term vision is authenticated digital worlds without predatory systems.


SMALL OPEN COOP WORLDS

The engine is designed around:
small servers, community hosting, LAN play, friend-group coop, moddable campaigns

NOT massive centralized infrastructure.

Target player count: up to 8 players

The focus is:
tight gameplay, social cooperation, replayability, experimentation, modding freedom


OPEN SOURCE FIRST

The engine is designed to be:
inspectable, hackable, portable, community-driven

This project values:
transparency, experimentation, ownership, creativity, preservation of moddable game culture

TOOLING
-------

Check out the tools subfolder. A converter tool exists in alpha that coverts ascii designs into workable map.txt files.

Usage:
cargo run -- ascii2map ../../maps/ascii/idea.txt ../../maps/episode01/map01.txt


DESIGN GOALS
------------

Retro Quake-like rendering, Lightweight architecture, Highly moddable systems, Text-defined gameplay
Server-authoritative multiplayer, Cryptographic identity, Signed inventories, Signed records and achievements
Small executable size, Fast iteration, Easy asset replacement, Open asset pipelines, Easy dedicated server hosting

CURRENT STATUS
--------------

Vertical Slice Prototype

Current milestone:
- 640x480 retro renderer
- textured room
- Quake-like visuals
- future signed inventory architecture
- moddable asset structure
- ASCII -> map file converter

LICENSE
-------

Open source.

Build strange things.