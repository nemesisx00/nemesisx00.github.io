'use client'

import React, { useRef, useEffect, useState } from 'react'
import processGameLogic from '@/gameLogic'
import renderGame from '@/gameRenderer'
import { BoxData, CanvasProperties, Platform, Player, PlayerController, Sprite, Vector2 } from '@/types'
import PlayerSpriteSheet, { PlayerSpritePathLeft, PlayerSpritePathRight } from '@/spritesheet'

const controller = new PlayerController()
const canvasProps = new CanvasProperties('gameCanvas')
const intervalDelay: number = 1000 / 60
const player = new Player(new Vector2(273, 100))

const platforms = [
	new Platform(new Vector2(50, 300), new Vector2(150, 300), "#333"),
	new Platform(new Vector2(200, 250), new Vector2(250, 250), "#444"),
	new Platform(new Vector2(260, 225), new Vector2(300, 225), "#555")
]

export default function Game()
{
	const canvasRef = useRef<HTMLCanvasElement>(null)
	const canvasHeight = canvasProps.size.y
	const [canvasWidth, setCanvasWidth] = useState(canvasProps.size.x)
	const [spriteSheetLeft, setSpriteSheetLeft] = useState<PlayerSpriteSheet|null>(null)
	const [spriteSheetRight, setSpriteSheetRight] = useState<PlayerSpriteSheet|null>(null)
	
	//Detect and handle viewport resize
	useEffect(() => {
		setCanvasWidth(
			global?.window?.innerWidth < 721
				? 400
				: canvasProps.size.x
		)
	})
	
	//Start the Game Loop
	useEffect(() => {
		let gameLoopInterval: NodeJS.Timer | null = null
		if(canvasRef.current)
		{
			const context = canvasRef.current.getContext(canvasProps.contextName) as CanvasRenderingContext2D
			
			if(!spriteSheetLeft)
				setSpriteSheetLeft(new PlayerSpriteSheet(PlayerSpritePathLeft, 16, 16, false))
			if(!spriteSheetRight)
				setSpriteSheetRight(new PlayerSpriteSheet(PlayerSpritePathRight, 16, 16))
			
			if(spriteSheetLeft && spriteSheetRight)
			{
				if(!gameLoopInterval)
				{
					gameLoopInterval = setInterval(() => {
						player.frameDelta += intervalDelay
						player.frameDeltaSit += intervalDelay
						
						processGameLogic(player, controller, canvasProps, canvasHeight, canvasWidth, platforms)
						renderGame(context, spriteSheetLeft, spriteSheetRight, player, platforms)
					}, intervalDelay)
				}
			}
		}
		
		const handleKeyInput = (e: KeyboardEvent) => controller.handleKeyInput(e)
		
		global?.window?.addEventListener('keydown', handleKeyInput)
		global?.window?.addEventListener('keyup', handleKeyInput)
		
		//Clean up on component unmount
		return () => {
			if(gameLoopInterval)
				clearInterval(gameLoopInterval)
			
			global?.window?.removeEventListener('keydown', handleKeyInput)
			global?.window?.removeEventListener('keyup', handleKeyInput)
		}
	})
	
	return (
		<canvas height={canvasHeight} id={canvasProps.id} ref={canvasRef} width={canvasWidth} />
	)
}
