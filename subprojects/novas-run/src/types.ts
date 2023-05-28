
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

export class Player
{
	crouchSpeed: number
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
	jumpImpulse: number
	position: Vector2
	velocity: Vector2
	speed: number
	width: number
	
	constructor(startingPosition?: Vector2)
	{
		this.crouchSpeed = 0.1
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
		this.jumpImpulse = 25
		this.position = startingPosition ? startingPosition : { x: 0, y: 0 }
		this.velocity = { x: 0, y: 0 }
		this.speed = 0.5
		this.width = 16
	}
}

export class PlayerController
{
	down: boolean = false
	left: boolean = false
	right: boolean = false
	up: boolean = false
	
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
			
			case 'w':
			case 'ArrowUp':
				this.up = pressed
				break
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
