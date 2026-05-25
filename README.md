Nine_Worlds_Of_The_SUN (NWOTS) BASE ENGINE
------------------------------------------

A Retro FPS Experiment in Modding, Ownership, and Cryptographic Gameplay

VISION
------

Minerva Base (Episode one of NWOTS) is an open-source retro first-person shooter framework inspired by:
- Doom
- Quake
- Hexen
- Unreal
- classic LAN shooters
- moddable PC gaming culture

The goal is NOT simply to create another FPS.

The goal is to build the most moddable FPS architecture possible.


CORE IDEAS
----------

1. EVERYTHING IS DATA-DRIVEN

Weapons, enemies, maps, HUDs, episodes, game rules, inventory systems,
teleporters, effects, sounds, and future game modes should all be editable
without recompiling the engine.

Non-coders should be able to:
- create weapons
- create enemies
- create maps
- create mods
- create total conversions
- create coop campaigns
- create entirely new genres

using simple text files and assets.


2. CRYPTOGRAPHIC OWNERSHIP

Players own their items cryptographically.

Content creators own their creations cryptographically.

Weapons, maps, mods, cosmetics, records, and achievements can be:
- signed
- verified
- transferred
- attributed
- persisted

without relying on centralized systems.

The game engine uses cryptographic signatures to validate:
- item ownership
- content authenticity
- personal records
- multiplayer inventory state
- future marketplace interactions

The long-term vision is authenticated digital worlds without predatory systems.


3. SMALL OPEN COOP WORLDS

The engine is designed around:
- small servers
- community hosting
- LAN play
- friend-group coop
- moddable campaigns

NOT massive centralized infrastructure.

Target player count:
- up to 8 players

The focus is:
- tight gameplay
- social cooperation
- replayability
- experimentation
- modding freedom


4. OPEN SOURCE FIRST

The engine is designed to be:
- inspectable
- hackable
- portable
- educational
- community-driven

This project values:
- transparency
- experimentation
- ownership
- creativity
- preservation of moddable game culture


DESIGN GOALS
------------

- Retro Quake-like rendering
- Lightweight architecture
- Highly moddable systems
- Text-defined gameplay
- Server-authoritative multiplayer
- Cryptographic identity
- Signed inventories
- Signed records and achievements
- Small executable size
- Fast iteration
- Easy asset replacement
- Open asset pipelines
- Easy dedicated server hosting


TECHNICAL PHILOSOPHY
--------------------

The engine code should provide:
- rendering
- networking
- physics
- cryptography
- asset loading
- ECS/runtime systems

The content should provide:
- gameplay
- maps
- weapons
- enemies
- UI
- progression
- mods

The engine is infrastructure.
The game is data.


LONG TERM FEATURES
------------------

Planned future systems include:
- coop campaigns
- key/door systems
- server browser
- dedicated servers
- replay validation
- deterministic records
- map editors
- scripting systems
- custom gamemodes
- community asset packs
- authenticated item trading
- mod distribution
- total conversions
- AI actors
- procedural encounters


PROJECT PHILOSOPHY
------------------

Classic FPS games became legendary because players could modify them.

This project attempts to bring that spirit back:
- small teams
- open experimentation
- creativity first
- ownership first
- moddability first

The dream is to create:
- the easiest FPS modding environment ever built
- for creators, mappers, artists, programmers, and players alike.


CURRENT STATUS
--------------

Vertical Slice Prototype

Current milestone:
- 640x480 retro renderer
- textured room
- Quake-like visuals
- future signed inventory architecture
- moddable asset structure


LICENSE
-------

Open source.

Build strange things.
