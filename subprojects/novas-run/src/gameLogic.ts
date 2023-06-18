import { Player } from "@/player"
import { CanvasProperties, Platform } from "@/types"

export default function processGameLogic(player: Player, canvasProps: CanvasProperties, height: number, width: number, platforms?: Platform[])
{
	player.update()
	processVelocity(player, canvasProps, height, width, platforms)
}

function processVelocity(player: Player, canvasProps: CanvasProperties, height: number, width: number, platforms?: Platform[])
{
	let halfPlayerHeight = player.height / 2
	let groundY = height - (player.height * 2) + 1;
	
	player.velocity.y += canvasProps.gravity
	
	player.position.x += player.velocity.x
	player.position.y += player.velocity.y
	
	player.velocity.x *= canvasProps.friction.x
	player.velocity.y *= canvasProps.friction.y
	
	player.standOnFloor(groundY)
	
	platforms?.filter(platform => platform.detectPlayer(player))
		.forEach(platform => player.standOnFloor(platform.start.y - platform.width - halfPlayerHeight))
	
	//Wrap around
	let wrapWidth = player.width * canvasProps.wrapFactor
	if(player.position.x < -wrapWidth)
		player.position.x = width - wrapWidth
	else if(player.position.x > width - wrapWidth)
		player.position.x = -wrapWidth
}
