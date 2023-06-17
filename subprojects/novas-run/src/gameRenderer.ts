import { BoxData, Platform, Player, Sprite, Vector2 } from "@/types"
import PlayerSpriteSheet from "@/spritesheet"

export default function renderGame(context: CanvasRenderingContext2D, spriteSheetLeft: PlayerSpriteSheet, spriteSheetRight: PlayerSpriteSheet, player: Player, platforms?: Platform[], sprites?: Sprite[])
{
	context.fillStyle = "#003A47"
	context.fillRect(0, 0, context.canvas.width, context.canvas.height)
	
	renderGround(context, player, platforms, sprites)
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
	
	if(player.isJumping || player.isCrouching || player.isSprinting || player.isMoving)
	{
		player.frameDeltaSit = 0
		
		if(player.isJumping)
			animation = 'jump'
		else if(player.isCrouching)
			animation = 'sneak'
		else if(player.isSprinting)
			animation = 'run'
		else if(player.isMoving)
			animation = 'walk'
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
	if(
		(
			player.isMoving
			&& (
				(player.isCrouching && !player.isSprinting && player.frameDelta >= 300)
				|| (!player.isCrouching && player.isSprinting && player.frameDelta >= 100)
				|| player.frameDelta >= 175
			)
		)
		|| player.frameDelta >= 300
	)
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

function renderGround(context: CanvasRenderingContext2D, player: Player, platforms?: Platform[], sprites?: Sprite[])
{
	const groundLevel = context.canvas.height - (player.height / 2)
	
	// Ground
	drawLine(context, new Vector2(0, groundLevel), new Vector2(context.canvas.width, groundLevel), "#004308", player.height)
	
	// Platform 1
	platforms?.forEach(platform => drawLine(context, platform.start, platform.end, platform.color, platform.width))
	
	// Shadow
	let radius = (player.width / 4) + player.position.y * 0.05
	if(radius < player.width / 8)
		radius = player.width / 8
	if(radius > player.width / 2)
		radius = player.width / 2
	
	drawPlayerShadow(context, new Vector2(player.position.x + player.width / 2, groundLevel - player.height / 2), radius)
	
	sprites?.forEach(sprite => sprite.draw(context))
}

function drawLine(context: CanvasRenderingContext2D, start: Vector2, end: Vector2, color: string | CanvasGradient | CanvasPattern = "#000", width = 16)
{
	context.strokeStyle = color
	context.lineWidth = width
	context.beginPath()
	context.moveTo(start.x, start.y)
	context.lineTo(end.x, end.y)
	context.stroke()
}

function drawPlayerShadow(context: CanvasRenderingContext2D, origin: Vector2, radius: number, color: string | CanvasGradient | CanvasPattern = "#222")
{
	context.fillStyle = color
	context.beginPath()
	context.arc(origin.x, origin.y, radius, 0, Math.PI, false)
	context.fill()
}
