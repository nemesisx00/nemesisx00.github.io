import { Vector2 } from "@/types"

export class Player
{
	controller: PlayerController
	currentLevel: number
	frameDelta: number
	frameDeltaSit: number
	height: number
	jumpDelta: number
	jumpImpulse: number
	jumpRate: number
	position: Vector2
	velocity: Vector2
	speed: number
	speedCrouch: number
	speedSprint: number
	status: PlayerStatus
	width: number
	
	constructor(startingPosition?: Vector2)
	{
		this.controller = new PlayerController()
		this.currentLevel = 0
		this.frameDelta = 0
		this.frameDeltaSit = 0
		this.height = 16
		this.jumpDelta = 0
		this.jumpImpulse = 25
		this.jumpRate = 500
		this.position = startingPosition ? startingPosition : { x: 0, y: 0 }
		this.velocity = { x: 0, y: 0 }
		this.speed = 0.1
		this.speedCrouch = 0.05
		this.speedSprint = 0.5
		this.status = new PlayerStatus()
		this.width = 16
	}
	
	animation()
	{
		let animation = 'idle'
		
		if(this.status.isJumping || this.status.isCrouching || this.status.isSprinting || this.status.isMoving)
		{
			this.frameDeltaSit = 0
			
			if(this.status.isJumping)
				animation = 'jump'
			else if(this.status.isCrouching)
				animation = 'sneak'
			else if(this.status.isSprinting)
				animation = 'run'
			else if(this.status.isMoving)
				animation = 'walk'
		}
		else if(this.frameDeltaSit >= 3000)
		{
			if(this.status.isSitting)
				animation = 'idleSit'
			else
				animation = 'sit'
		}
		
		return animation
	}
	
	isOnFloor(y: number) { return this.position.y > y }
	
	nextFrameIndex(currentFrame: number)
	{
		let frame = currentFrame
		if(
			(
				this.status.isMoving
				&& (
					(this.status.isCrouching && !this.status.isSprinting && this.frameDelta >= 300)
					|| (!this.status.isCrouching && this.status.isSprinting && this.frameDelta >= 100)
					|| this.frameDelta >= 175
				)
			)
			|| this.frameDelta >= 300
		)
		{
			if(this.status.directionMove)
			{
				if(this.status.isSitting)
				{
					if(frame <= 3)
					{
						frame = 3
						this.status.directionSitFrame = true
					}
					
					if(frame >= 6)
						this.status.directionSitFrame = false
				}
				
				let prevFrame = frame
				if(this.status.isSitting && !this.status.directionSitFrame)
					frame--
				else
					frame++
				
				if(this.status.isCrouching && !this.status.isMoving)
					frame = prevFrame
			}
			else
			{
				if(this.status.isSitting)
				{
					if(frame >= 4)
					{
						frame = 4
						this.status.directionSitFrame = true
					}
					
					if(frame <= 1)
						this.status.directionSitFrame = false
				}
				
				let prevFrame = frame
				if(this.status.isSitting && !this.status.directionSitFrame)
					frame++
				else
					frame--
				
				if(this.status.isCrouching && !this.status.isMoving)
					frame = prevFrame
			}
			
			this.frameDelta = 0
		}
		
		if(this.status.isJumping)
			frame = this.status.directionMove ? 7 : 0
		
		if(!this.status.isMoving && !this.status.isJumping && !this.status.isCrouching && this.frameDeltaSit >= 3000 && frame >= 3)
			this.status.isSitting = true
		if(this.status.isMoving)
			this.status.isSitting = false
		
		return frame
	}
	
	standOnFloor(y: number)
	{
		if(this.isOnFloor(y))
		{
			this.status.isJumping = false
			this.position.y = y
			this.velocity.y = 0
		}
	}
	
	shadowRadius()
	{
		let radius = (this.width / 4) + this.position.y * 0.05
		if(radius < this.width / 8)
			radius = this.width / 8
		if(radius > this.width / 2)
			radius = this.width / 2
		
		return radius
	}
	
	update()
	{
		if(this.controller.up)
		{
			if(this.status.canJump && !this.status.isJumping)
			{
				this.status.canJump = false
				this.jumpDelta = 0
				this.status.isJumping = true
				this.velocity.y -= this.jumpImpulse
			}
		}
		
		if(this.controller.left)
		{
			this.velocity.x -= this.status.isCrouching
									? this.speedCrouch
									: this.status.isSprinting
										? this.speedSprint
										: this.speed
			
			if(!this.controller.right)
				this.status.directionMove = false
		}
		
		if(this.controller.right)
		{
			this.velocity.x += this.status.isCrouching
									? this.speedCrouch
									: this.status.isSprinting
										? this.speedSprint
										: this.speed
			
			if(!this.controller.left)
				this.status.directionMove = true
		}
		
		this.status.isCrouching = this.controller.down
		this.status.isMoving = this.controller.left != this.controller.right
		this.status.isSprinting = this.controller.sprint
		
		if(this.jumpDelta >= this.jumpRate)
			this.status.canJump = true
	}
}

class PlayerController
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

class PlayerStatus
{
	canJump: boolean
	directionMove: boolean
	directionSitFrame: boolean
	isCrouching: boolean
	isIdle: boolean
	isJumping: boolean
	isMoving: boolean
	isSitting: boolean
	isSprinting: boolean
	
	constructor()
	{
		this.canJump = true
		this.directionMove = true
		this.directionSitFrame = true
		this.isCrouching = false
		this.isIdle = false
		this.isJumping = true
		this.isMoving = false
		this.isSitting = false
		this.isSprinting = false
	}
}
