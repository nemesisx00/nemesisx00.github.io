import { Platform, Sprite, Vector2 } from '@/types'

export class Level
{
	label: string
	platforms: Platform[]
	platformsShortMax: number
	sprites: Sprite[]
	
	constructor(label: string = '', platforms: Platform[] = [], sprites: Sprite[] = [], platformsShortMax?: number)
	{
		this.label = label
		this.platforms = platforms
		this.platformsShortMax = platformsShortMax ? platformsShortMax : platforms.length
		this.sprites = sprites
	}
}

export const Levels: Level[] = [
	new Level(
		'Level 1',
		[
			new Platform(new Vector2(75, 300), new Vector2(175, 300), "#333"),
			new Platform(new Vector2(225, 300), new Vector2(325, 300), "#333"),
			new Platform(new Vector2(150, 250), new Vector2(250, 250), "#444"),
			
			new Platform(new Vector2(375, 300), new Vector2(475, 300), "#333"),
			new Platform(new Vector2(525, 300), new Vector2(625, 300), "#333"),
			new Platform(new Vector2(450, 250), new Vector2(550, 250), "#444")
		],
		[],
		3
	),
	
	new Level('Level 2', [
		new Platform(new Vector2(50, 300), new Vector2(150, 300), "#333"),
		new Platform(new Vector2(200, 250), new Vector2(250, 250), "#444"),
		new Platform(new Vector2(260, 225), new Vector2(300, 225), "#555")
	])
]
