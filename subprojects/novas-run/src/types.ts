import { Player } from "@/player"

export class BoxData
{
	height: number
	width: number
	origin: Vector2
	
	constructor(height?: number, width?: number, origin?: Vector2)
	{
		this.height = height ? height : 0
		this.width = width ? width : 0
		this.origin = origin ? origin : new Vector2(0, 0)
	}
}

export class CanvasProperties
{
	contextName: string
	gravity: number
	friction: Vector2
	id: string
	size: Vector2
	wrapFactor: number
	
	constructor(id: string, size?: Vector2)
	{
		this.contextName = '2d'
		this.gravity = 1
		this.friction = new Vector2(0.9, 0.9)
		this.id = id
		this.size = size ? size : new Vector2(720, 400)
		this.wrapFactor = 0.625
	}
}

export class Platform
{
	color: string | CanvasGradient | CanvasPattern
	end: Vector2
	start: Vector2
	width: number
	
	constructor(start?: Vector2, end?: Vector2, color: string | CanvasGradient | CanvasPattern = "#333", width: number = 16)
	{
		this.color = color
		this.end = end ? end : new Vector2()
		this.start = start ? start : new Vector2()
		this.width = width
	}
	
	draw(context: CanvasRenderingContext2D)
	{
		context.strokeStyle = this.color
		context.lineWidth = this.width
		context.beginPath()
		context.moveTo(this.start.x, this.start.y)
		context.lineTo(this.end.x, this.end.y)
		context.stroke()
	}
	
	detectPlayer(player: Player)
	{
		return player.position.x >= this.start.x - player.width // Left Bound
			&& player.position.x <= this.end.x - player.width / 4 // Right Bound
			&& player.position.y >= this.start.y - this.width - player.height / 2 // Upper Bound
			&& player.position.y <= this.end.y // Lower Bound
	}
}

export class Sprite
{
	image?: HTMLImageElement
	transform: BoxData
	
	constructor(image?: HTMLImageElement, transform?: BoxData)
	{
		this.transform = transform
					? transform
					: new BoxData()
		
		this.image = image
					? image
					: undefined
	}
	
	draw(context: CanvasRenderingContext2D)
	{
		if(this.image)
		{
			context.drawImage(
				this.image,
				0, 0, this.transform.width, this.transform.height,
				this.transform.origin.x, this.transform.origin.y, this.transform.origin.x + this.transform.width, this.transform.origin.y + this.transform.height
			)
		}
	}
}

export class Vector2
{
	x: number
	y: number
	
	constructor(x?: number, y?: number)
	{
		this.x = x ? x : 0
		this.y = y ? y : 0
	}
}
