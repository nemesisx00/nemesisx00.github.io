'use client'

import React, { useRef, useEffect, useState } from 'react'
import {CanvasProperties, Player, PlayerController, Vector2} from '@/types'
import {PlayerSpritePathLeft, PlayerSpritePathRight, PlayerSpriteSheet} from '@/spritesheet'

const controller = new PlayerController()
const canvasProps = new CanvasProperties('gameCanvas')
const intervalDelay: number = 1000 / 60
const player = new Player(new Vector2(200, 175))

let delta: number = 0
let sitDelta: number = 0
let sitFrameDirection: boolean = true
let moveDirection: boolean = true

export function Game()
{
	global?.window?.addEventListener('keydown', e => controller.handleKeyInput(e))
	global?.window?.addEventListener('keyup', e => controller.handleKeyInput(e))
	
	const canvasRef = useRef<HTMLCanvasElement>(null)
	const [spriteSheetLeft, setSpriteSheetLeft] = useState<PlayerSpriteSheet|null>(null)
	const [spriteSheetRight, setSpriteSheetRight] = useState<PlayerSpriteSheet|null>(null)
	
	useEffect(() => {
		if(canvasRef.current)
		{
			const context = canvasRef.current.getContext(canvasProps.contextName) as CanvasRenderingContext2D
			
			if(!spriteSheetLeft)
				setSpriteSheetLeft(new PlayerSpriteSheet(PlayerSpritePathLeft, 16, 16, false))
			if(!spriteSheetRight)
				setSpriteSheetRight(new PlayerSpriteSheet(PlayerSpritePathRight, 16, 16))
			
			if(spriteSheetLeft && spriteSheetRight)
			{
				setInterval(() => {
					delta += intervalDelay
					sitDelta += intervalDelay
					
					processGameLogic()
					render(context, spriteSheetLeft, spriteSheetRight)
				}, intervalDelay)
			}
		}
	})
	
	return (
		<canvas height={canvasProps.size.y} id={canvasProps.id} ref={canvasRef} width={canvasProps.size.x} />
	)
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

function render(context: CanvasRenderingContext2D, spriteSheetLeft: PlayerSpriteSheet, spriteSheetRight: PlayerSpriteSheet)
{
	context.fillStyle = "#003A47"
	context.fillRect(0, 0, context.canvas.width, context.canvas.height)
	
	renderGround(context)
	renderPlayer(context, spriteSheetLeft, spriteSheetRight)
}

function renderPlayer(context: CanvasRenderingContext2D, spriteSheetLeft: PlayerSpriteSheet, spriteSheetRight: PlayerSpriteSheet)
{
	let animation = getAnimation()
	let frame = getNextFrameIndex(moveDirection ? spriteSheetRight.frame : spriteSheetLeft.frame)
	
	if(!player.isMoving && !player.isJumping && !player.isCrouching && sitDelta >= 3000 && frame >= 3)
		player.isSitting = true
	if(player.isMoving)
		player.isSitting = false
	
	if(!moveDirection)
		spriteSheetLeft.drawFrame(context, animation, frame, player.position, { x: player.width, y: player.height })
	else
		spriteSheetRight.drawFrame(context, animation, frame, player.position, { x: player.width, y: player.height })
}

function getAnimation()
{
	let animation = 'idle'
	
	if(player.isJumping || player.isCrouching || player.isMoving)
	{
		sitDelta = 0
		
		if(player.isJumping)
			animation = 'jump'
		else if(player.isCrouching)
			animation = 'sneak'
		else if(player.isMoving)
			animation = 'run'
	}
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
		if(moveDirection)
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
				frame--
			else
				frame++
			
			if(player.isCrouching && !player.isMoving)
				frame = prevFrame
		}
		else
		{
			if(player.isSitting)
			{
				if(frame >= 4)
				{
					frame = 4
					sitFrameDirection = true
				}
				
				if(frame <= 1)
					sitFrameDirection = false
			}
			
			let prevFrame = frame
			if(player.isSitting && !sitFrameDirection)
				frame++
			else
				frame--
			
			if(player.isCrouching && !player.isMoving)
				frame = prevFrame
		}
		
		delta = 0
	}
	
	if(player.isJumping)
		frame = moveDirection ? 7 : 0
	
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
