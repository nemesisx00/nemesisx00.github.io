import { Platform, Sprite, Vector2 } from '@/types'

export class Level
{
	label: string
	platforms: Platform[]
	sprites: Sprite[]
	
	constructor(label: string = '', platforms: Platform[] = [], sprites: Sprite[] = [])
	{
		this.label = label
		this.platforms = platforms
		this.sprites = sprites
	}
}

export const Levels: Level[] = [
	new Level('Level 1', [
		new Platform(new Vector2(50, 300), new Vector2(150, 300), "#333"),
		new Platform(new Vector2(200, 250), new Vector2(250, 250), "#444"),
		new Platform(new Vector2(260, 225), new Vector2(300, 225), "#555")
	])
]
