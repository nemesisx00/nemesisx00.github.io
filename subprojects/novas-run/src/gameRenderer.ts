import { Player } from "@/types"
import { PlayerSpriteSheet } from "@/spritesheet"

export default function render(context: CanvasRenderingContext2D, spriteSheetLeft: PlayerSpriteSheet, spriteSheetRight: PlayerSpriteSheet, player: Player)
{
	context.fillStyle = "#003A47"
	context.fillRect(0, 0, context.canvas.width, context.canvas.height)
	
	renderGround(context, player)
	renderPlayer(context, spriteSheetLeft, spriteSheetRight, player)
}

function renderPlayer(context: CanvasRenderingContext2D, spriteSheetLeft: PlayerSpriteSheet, spriteSheetRight: PlayerSpriteSheet, player: Player)
{
	let animation = getAnimation(player)
	let frame = getNextFrameIndex(player.directionMove ? spriteSheetRight.frame : spriteSheetLeft.frame, player)
	
	if(!player.isMoving && !player.isJumping && !player.isCrouching && player.frameDeltaSit >= 3000 && frame >= 3)
		player.isSitting = true
	if(player.isMoving)
		player.isSitting = false
	
	if(!player.directionMove)
		spriteSheetLeft.drawFrame(context, animation, frame, player.position, { x: player.width, y: player.height })
	else
		spriteSheetRight.drawFrame(context, animation, frame, player.position, { x: player.width, y: player.height })
}

function getAnimation(player: Player)
{
	let animation = 'idle'
	
	if(player.isJumping || player.isCrouching || player.isMoving)
	{
		player.frameDeltaSit = 0
		
		if(player.isJumping)
			animation = 'jump'
		else if(player.isCrouching)
			animation = 'sneak'
		else if(player.isMoving)
			animation = 'run'
	}
	else if(player.frameDeltaSit >= 3000)
	{
		if(player.isSitting)
			animation = 'idleSit'
		else
			animation = 'sit'
	}
	
	return animation
}

function getNextFrameIndex(currentFrame: number, player: Player)
{
	let frame = currentFrame
	if((player.isMoving && !player.isCrouching && player.frameDelta >= 100) || player.frameDelta >= 300)
	{
		if(player.directionMove)
		{
			if(player.isSitting)
			{
				if(frame <= 3)
				{
					frame = 3
					player.directionSitFrame = true
				}
				
				if(frame >= 6)
					player.directionSitFrame = false
			}
			
			let prevFrame = frame
			if(player.isSitting && !player.directionSitFrame)
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
					player.directionSitFrame = true
				}
				
				if(frame <= 1)
					player.directionSitFrame = false
			}
			
			let prevFrame = frame
			if(player.isSitting && !player.directionSitFrame)
				frame++
			else
				frame--
			
			if(player.isCrouching && !player.isMoving)
				frame = prevFrame
		}
		
		player.frameDelta = 0
	}
	
	if(player.isJumping)
		frame = player.directionMove ? 7 : 0
	
	return frame
}

function renderGround(context: CanvasRenderingContext2D, player: Player)
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
