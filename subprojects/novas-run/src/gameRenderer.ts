import { Player } from "@/player"
import { Platform, Sprite, Vector2 } from "@/types"
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
	let animation = player.animation()
	let frame = player.nextFrameIndex(player.status.directionMove ? spriteSheetRight.frame : spriteSheetLeft.frame)
	
	if(!player.status.isMoving && !player.status.isJumping && !player.status.isCrouching && player.frameDeltaSit >= 3000 && frame >= 3)
		player.status.isSitting = true
	if(player.status.isMoving)
		player.status.isSitting = false
	
	if(!player.status.directionMove)
		spriteSheetLeft.drawFrame(context, animation, frame, player.position, { x: player.width, y: player.height })
	else
		spriteSheetRight.drawFrame(context, animation, frame, player.position, { x: player.width, y: player.height })
}

function renderGround(context: CanvasRenderingContext2D, player: Player, platforms?: Platform[], sprites?: Sprite[])
{
	const groundLevel = context.canvas.height - (player.height / 2)
	
	// Ground
	drawLine(context, new Vector2(0, groundLevel), new Vector2(context.canvas.width, groundLevel), "#004308", player.height)
	
	platforms?.forEach(platform => platform.draw(context))
	
	drawPlayerShadow(context, new Vector2(player.position.x + player.width / 2, groundLevel - player.height / 2), player.shadowRadius())
	
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
