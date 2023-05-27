
export interface CanvasProperties
{
	contextName: string
	friction: Vector2
	gravity: number
	id: string
	size: Vector2
	wrapFactor: number
}

export interface Player
{
	crouchSpeed: number
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
}

export interface PlayerController
{
	down: boolean
	left: boolean
	right: boolean
	up: boolean
	handleKeyInput: Function
}

export interface Vector2
{
	x: number
	y: number
}
