# Introduction
Proposal by Hazel, January 3rd 2023.

## Pitch
_What our game is all about._

An arcade-style top-down dungeon crawler where players gather resources above ground, and venture into bite-sized dungeons with their gathered equipment.

## Inspiration
_What inspired us, what we can incorporate._

### Cube World
The gameplay loop of Cube World, specifically its Alpha version from 2013, consists of **world exploration for the purpose of gathering resources**, and **defeating enemies for a chance of obtaining stronger equipment**. As the player's equipment gets gradually stronger, they're able to face greater foes and challenging dungeons.

### Binding of Isaac
Levels in the Binding of Isaac consist of **screen-sized rooms** with enemies and rewards, tied together to create unique maps.

## Intended Experience
_How we want to make it fun._

In order to fit the game to both short casual sessions and long grinds, the gameplay loop between **gathering resources** and **facing dungeons with new challenges** has to be short. The growth of strength must continue to feel satisfying, while also scaling new challenges to the players' current strength. The increase in difficulty should continue to feel exciting.

## Genre
_Neat little boxes our game fits into._

Cooperative, RPG, arcade.

## Tooling and Platforms
_The right tools for the job._
- Rust, a multi-paradigm programming language with a focus on runtime stability and a rich ecosystem of development tools,
- Bevy, a data-driven game engine made for Rust that makes use of the Entity Component System model,
- Asesprite, a open source pixel art program,
- (What sound program will any of us be using?)

**Platforms:** PC - Windows, Mac OS X, Linux

# Concept

## Gameplay
_What the average session looks like._

Players control their own character and move freely through a procedurally generated overworld, rich with flora and fauna to gather necessary resources from. Gameplay starts during the daytime, where resource collection is safe. As day turns to night and players have gathered, crafted and equipped themselves, the gates are thrown wide, inviting players for a challenge through dungeons. Dungeons consist of randomly placed and sized rooms, tied together

## Thematic: Reach for the Stars (Not sure on this one yet)
_The feeling the game should invoke._

'Reach for the Stars': Players must always strife to outsmart, outmaneuver and outplay the threats the game's world will throw at them.

The gameplay revolves around the players always needing to be ahead of the curve, a curve that simultaneously moves along with them. Only with the strength of will to persevere, to continue to reach for the stars, players will steel themselves for the next fight.

## Mechanics
_The systems that make the game tick._

### Movement
Players move freely in the north, south, west and east directions. They are not snapped to the grid the worlds are built on. By holding a button, they can also sprint to move faster, at the cost of **energy**. 
Velocity mechanics?

### Combat
(We gotta work out how we want combat to work exactly. Attack style will be based on player equipment. What kind of equipment do we want? Do we want pre-determined classes?)
Likely different movement subsets, similar to terraria?
Melee/Ranged/AOE/Pets

### Quick Items
Certain items, such as food and potions, can be used quickly by putting them into hotbar slots 1 to 4.

### Energy
Players have an energy bar that depletes with intensive efforts, such as **running**, **scaling terrain** and using **strong techniques**. Energy can be restored in bulk by resting, or in small amounts by eating **gathered food**. When players are depleted of energy, they are weakened until they regain some.

### Gathering Materials
Players stand next to resource hotspots (i.e. trees, ore deposits, life-filled water...) and use either standard technique or strong technique. Standard technique spends no energy, and strong technique greatly increases the yield while expending some **energy**.

### Gathering Food from Fauna
Players can gather food from fauna by combat.

### Overworld: Heights
The overworld is separated by different heights of terrain. Players can expend some of their energy to scale the terrain, with the ability to find more valuable resources at heigher elevations.

### Overworld: At Night
Players may decide to stay the night in the overworld instead of challenging a dungeon, to catch up on lost **energy** or to continue **gathering**. At night however, the overworld is haunted by spirits, making overnight gathering outside of the **campsite** more challenging.

### Dungeons: Rooms
Dungeons are split into rooms, which may contain **hordes**, **bosses** and **hideouts**. Rooms are connected by **links**.

### Dungeons: Links
Links connect dungeon **rooms**, and may contain small amounts of enemies.

### Dungeons: Hordes
Hordes of enemies may be found in a dungeon **room**. Each of the enemies may drop pieces of equipment or rare materials.

### Dungeons: Bosses
A small amount of **rooms** in a dungeon may contain bosses. Bosses are enemies of great strength that incorporate a modular set of attacks and spells. Defeating them grants a boss treasure, which contains loot for each player exclusively.

### Dungeons: Hideouts
Hideouts are **rooms** that allow for a calm place to **rest and regain energy**.

### Dungeons: At Day
Dungeons' gates close again as the day arrives, while enemies will continue to appear within. Players will be locked into dungeons and have to wait for night arrive again to leave, massively increasing the risk of **energy depletion**.

# Audio and visuals

## Thematic Interpretation
(TODO)

## Artstyle
We have settled on wanting a pixel artstyle, mainly because part of our team is already experienced with drawing pixel art.

## Music
(TODO)

## SFX
(TODO)

# User Experience

## Interface
(Should the UI match the pixel artstyle?)

## Controls
_Keyboard/Mouse, Gamepad (Xbox layout)._

- Movement: WASD, LS
- Hotbar items: 1-4, D-pad (left, up, right, down)
- Aiming: mouse cursor, RS
- Simple technique/attack: LMB, RT
- Strong technique/attack: RMD, LT

# Development Planning
(TODO)
