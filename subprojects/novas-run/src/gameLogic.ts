import { CanvasProperties, Platform, Player, PlayerController } from "@/types"

export default function processGameLogic(player: Player, controller: PlayerController, canvasProps: CanvasProperties, height: number, width: number, platforms?: Platform[])
{
	handleMovementInput(player, controller)
	processVelocity(player, canvasProps, height, width, platforms)
}

function handleMovementInput(player: Player, controller: PlayerController)
{
	if(controller.up && !player.isJumping)
	{
		player.velocity.y -= player.jumpImpulse
		player.isJumping = true
	}
	
	if(controller.left)
	{
		player.velocity.x -= player.isCrouching
								? player.speedCrouch
								: player.isSprinting
									? player.speedSprint
									: player.speed
		
		if(!controller.right)
			player.directionMove = false
	}
	
	if(controller.right)
	{
		player.velocity.x += player.isCrouching
								? player.speedCrouch
								: player.isSprinting
									? player.speedSprint
									: player.speed
		
		if(!controller.left)
			player.directionMove = true
	}
	
	player.isCrouching = controller.down
	player.isMoving = controller.left != controller.right
	player.isSprinting = controller.sprint
}

function processVelocity(player: Player, canvasProps: CanvasProperties, height: number, width: number, platforms?: Platform[])
{
	player.velocity.y += canvasProps.gravity
	
	player.position.x += player.velocity.x
	player.position.y += player.velocity.y
	
	player.velocity.x *= canvasProps.friction.x
	player.velocity.y *= canvasProps.friction.y
	
	let doublePlayerHeight = player.height * 2
	let halfPlayerHeight = player.height / 2
	let halfPlayerWidth = player.width / 2
	
	//Is on floor?
	if(player.position.y > height - doublePlayerHeight + 1)
	{
		player.isJumping = false
		player.position.y = height - doublePlayerHeight + 1
		player.velocity.y = 0
	}
	
	//Is on platform?
	platforms?.forEach(platform => {
		if(player.position.x >= platform.start.x - halfPlayerWidth // Left Bound
			&& player.position.x <= platform.end.x + halfPlayerWidth // Right Bound
			&& player.position.y >= platform.start.y - platform.width - halfPlayerHeight // Upper Bound
			&& player.position.y <= platform.end.y) // Lower Bound
		{
			player.isJumping = false
			player.position.y = platform.start.y - platform.width - halfPlayerHeight
			player.velocity.y = 0
		}
	})
	
	//Wrap around
	let wrapWidth = player.width * canvasProps.wrapFactor
	if(player.position.x < -wrapWidth)
		player.position.x = width - wrapWidth
	else if(player.position.x > width - wrapWidth)
		player.position.x = -wrapWidth
}
