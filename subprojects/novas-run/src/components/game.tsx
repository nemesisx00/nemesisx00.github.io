'use client'

import React, { useRef, useEffect, useState } from 'react'
import {CanvasProperties, Player, PlayerController, Vector2} from '@/types'
import {PlayerSpritePath, PlayerSpriteSheet} from '@/spritesheet'

export function Game()
{
	global?.window?.addEventListener('keydown', e => controller.handleKeyInput(e))
	global?.window?.addEventListener('keyup', e => controller.handleKeyInput(e))
	
	const canvasRef = useRef<HTMLCanvasElement>(null)
	const [spriteSheet, setSpriteSheet] = useState<PlayerSpriteSheet|null>(null)
	
	useEffect(() => {
		if(canvasRef.current)
		{
			const context = canvasRef.current.getContext(canvasProps.contextName) as CanvasRenderingContext2D
			
			if(!spriteSheet)
				setSpriteSheet(new PlayerSpriteSheet(PlayerSpritePath, 16, 16))
			
			if(spriteSheet)
			{
				setInterval(() => {
					delta += IntervalDelay
					sitDelta += IntervalDelay
					
					processGameLogic()
					render(context, spriteSheet)
				}, IntervalDelay)
			}
		}
	})
	
	return (
		<canvas height={canvasProps.size.y} id={canvasProps.id} ref={canvasRef} width={canvasProps.size.x} />
	)
}

let delta: number = 0
let sitDelta: number = 0
let sitFrameDirection: boolean = true
let moveDirection: boolean = true

const IntervalDelay: number = 1000 / 60

const canvasProps: CanvasProperties = {
	contextName: '2d',
	gravity: 1,
	friction: { x: 0.9, y: 0.9 },
	id: 'gameCanvas',
	size: { x: 720, y: 400 },
	wrapFactor: 0.625,
}

const player: Player = {
	crouchSpeed: 0.1,
	height: 16,
	isCrouching: false,
	isIdle: false,
	isJumping: true,
	isMoving: false,
	isSitting: false,
	jumpImpulse: 25,
	position: { x: 200, y: 175 },
	velocity: { x: 0, y: 0 },
	speed: 0.5,
	width: 16,
}

const controller: PlayerController = {
	down: false,
	left: false,
	right: false,
	up: false,
	
	handleKeyInput: (event: KeyboardEvent) => {
		let pressed = event.type == 'keydown'
		switch(event.key)
		{
			case 'a':
			case 'ArrowLeft':
				controller.left = pressed
				break
			
			case 'd':
			case 'ArrowRight':
				controller.right = pressed
				break
			
			case 's':
			case 'ArrowDown':
				controller.down = pressed
				break
			
			case 'w':
			case 'ArrowUp':
				controller.up = pressed
				break
		}
	}
}

function processGameLogic()
{
	handleMovementInput()
	processVelocity()
}

function handleMovementInput()
{
	player.isCrouching = controller.down
	player.isMoving = controller.left || controller.right
	
	if(controller.up && !player.isJumping)
	{
		player.velocity.y -= player.jumpImpulse
		player.isJumping = true
	}
	
	if(controller.left)
	{
		player.velocity.x -= player.isCrouching ? player.crouchSpeed : player.speed
		moveDirection = false
	}
	
	if(controller.right)
	{
		player.velocity.x += player.isCrouching ? player.crouchSpeed : player.speed
		moveDirection = true
	}
}

function processVelocity()
{
	player.velocity.y += canvasProps.gravity
	
	player.position.x += player.velocity.x
	player.position.y += player.velocity.y
	
	player.velocity.x *= canvasProps.friction.x
	player.velocity.y *= canvasProps.friction.y
	
	//Is on floor?
	if(player.position.y > canvasProps.size.y - (player.height * 2) + 1)
	{
		player.isJumping = false
		player.position.y = canvasProps.size.y - (player.height * 2) + 1
		player.velocity.y = 0
	}
	
	//Wrap around
	let wrapWidth = player.width * canvasProps.wrapFactor
	if(player.position.x < -wrapWidth)
		player.position.x = canvasProps.size.x - wrapWidth
	else if(player.position.x > canvasProps.size.x - wrapWidth)
		player.position.x = -wrapWidth
}

function render(context: CanvasRenderingContext2D, spriteSheet: PlayerSpriteSheet)
{
	context.fillStyle = "#003A47"
	context.fillRect(0, 0, context.canvas.width, context.canvas.height)
	
	renderGround(context)
	renderPlayer(context, spriteSheet)
}

function renderPlayer(context: CanvasRenderingContext2D, spriteSheet: PlayerSpriteSheet)
{
	let animation = getAnimation()
	let frame = getNextFrameIndex(spriteSheet.frame)
	
	if(sitDelta >= 3000 && frame >= 3)
		player.isSitting = true
	if(player.isMoving)
		player.isSitting = false
	
	spriteSheet.drawFrame(context, animation, frame, player.position, { x: player.width, y: player.height })
}

function getAnimation()
{
	let animation = 'idle'
	
	if(player.isMoving)
	{
		sitDelta = 0
		
		if(player.isCrouching)
			animation = 'sneak'
		else
			animation = 'run'
	}
	else if(player.isCrouching)
		animation = 'sneak'
	else if(sitDelta >= 3000)
	{
		if(player.isSitting)
			animation = 'idleSit'
		else
			animation = 'sit'
	}
	
	return animation
}

function getNextFrameIndex(currentFrame: number)
{
	let frame = currentFrame
	if((player.isMoving && !player.isCrouching && delta >= 100) || delta >= 300)
	{
		if(player.isSitting)
		{
			if(frame <= 3)
			{
				frame = 3
				sitFrameDirection = true
			}
			
			if(frame >= 6)
				sitFrameDirection = false
		}
		
		let prevFrame = frame
		if(player.isSitting && !sitFrameDirection)
			frame -= 1
		else
			frame += 1
		
		if(player.isCrouching && !player.isMoving)
			frame = prevFrame
		
		delta = 0
	}
	
	return frame
}

function renderGround(context: CanvasRenderingContext2D)
{
	const groundLevel = context.canvas.height - (player.height / 2)
	
	context.strokeStyle = "#004308"
	context.lineWidth = player.height
	context.beginPath()
	context.moveTo(0, groundLevel)
	context.lineTo(context.canvas.width, groundLevel)
	context.stroke()
	
	let radius = (player.width / 4) + player.position.y * 0.05
	if(radius < player.width / 8)
		radius = player.width / 8
	if(radius > player.width / 2)
		radius = player.width / 2
	
	context.fillStyle = "#222222"
	context.beginPath()
	context.arc(player.position.x + player.width / 2, groundLevel - player.height / 2, radius, 0, Math.PI, false)
	context.fill()
}
