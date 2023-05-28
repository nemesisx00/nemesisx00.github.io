import { CanvasProperties, Player, PlayerController } from "@/types"

export default function processGameLogic(player: Player, controller: PlayerController, canvasProps: CanvasProperties, height: number, width: number)
{
	handleMovementInput(player, controller)
	processVelocity(player, canvasProps, height, width)
}

function handleMovementInput(player: Player, controller: PlayerController)
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
		player.directionMove = false
	}
	
	if(controller.right)
	{
		player.velocity.x += player.isCrouching ? player.crouchSpeed : player.speed
		player.directionMove = true
	}
}

function processVelocity(player: Player, canvasProps: CanvasProperties, height: number, width: number)
{
	player.velocity.y += canvasProps.gravity
	
	player.position.x += player.velocity.x
	player.position.y += player.velocity.y
	
	player.velocity.x *= canvasProps.friction.x
	player.velocity.y *= canvasProps.friction.y
	
	//Is on floor?
	if(player.position.y > height - (player.height * 2) + 1)
	{
		player.isJumping = false
		player.position.y = height - (player.height * 2) + 1
		player.velocity.y = 0
	}
	
	//Wrap around
	let wrapWidth = player.width * canvasProps.wrapFactor
	if(player.position.x < -wrapWidth)
		player.position.x = width - wrapWidth
	else if(player.position.x > width - wrapWidth)
		player.position.x = -wrapWidth
}
