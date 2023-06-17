
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
}

export class Player
{
	directionMove: boolean
	directionSitFrame: boolean
	frameDelta: number
	frameDeltaSit: number
	height: number
	isCrouching: boolean
	isIdle: boolean
	isJumping: boolean
	isMoving: boolean
	isSitting: boolean
	isSprinting: boolean
	jumpCooldown: number
	jumpImpulse: number
	position: Vector2
	velocity: Vector2
	speed: number
	speedCrouch: number
	speedSprint: number
	width: number
	
	constructor(startingPosition?: Vector2)
	{
		this.directionMove = true
		this.directionSitFrame = true
		this.frameDelta = 0
		this.frameDeltaSit = 0
		this.height = 16
		this.isCrouching = false
		this.isIdle = false
		this.isJumping = true
		this.isMoving = false
		this.isSitting = false
		this.isSprinting = false
		this.jumpCooldown = 0.25
		this.jumpImpulse = 25
		this.position = startingPosition ? startingPosition : { x: 0, y: 0 }
		this.velocity = { x: 0, y: 0 }
		this.speed = 0.1
		this.speedCrouch = 0.05
		this.speedSprint = 0.5
		this.width = 16
	}
}

export class PlayerController
{
	down: boolean = false
	left: boolean = false
	right: boolean = false
	up: boolean = false
	sprint: boolean = false
	
	constructor() {}
	
	handleKeyInput(event: KeyboardEvent)
	{
		let pressed = event.type == 'keydown'
		switch(event.key)
		{
			case 'a':
			case 'ArrowLeft':
				this.left = pressed
				break
			
			case 'd':
			case 'ArrowRight':
				this.right = pressed
				break
			
			case 's':
			case 'ArrowDown':
				this.down = pressed
				break
			
			case 'v':
			case '0':
				this.sprint = pressed
				break
			
			case 'w':
			case 'ArrowUp':
				this.up = pressed
				break
		}
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
