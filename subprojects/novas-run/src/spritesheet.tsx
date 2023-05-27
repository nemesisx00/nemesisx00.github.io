
import {Vector2} from '@/types'

export const PlayerSpritePath: string = './assets/playersprite_0.png'

export class PlayerSpriteSheet
{
	frame: number
	frameProps: Vector2
	image: HTMLImageElement
	
	constructor(path: string, frameHeight: number, frameWidth: number)
	{
		this.frame = 0
		this.frameProps = { x: frameWidth, y: frameHeight }
		
		this.image = new Image()
		this.image.src = path
	}
	
	getTargetFrame(targetFrame: number, row: number, maxFrame: number, startFrame: number = 0)
	{
		let sX = this.frame * this.frameProps.x
		let sY = this.frameProps.y * row
		let sHeight = this.frameProps.y
		let sWidth = this.frameProps.x
		
		this.frame = targetFrame
		if(this.frame < startFrame)
			this.frame = maxFrame + this.frame
		this.frame = this.frame % maxFrame
		
		return { x: sX, y: sY, height: sHeight, width: sWidth }
	}
	
	getAnimationFrame(animation: string, frame: number)
	{
		let nextFrame: BoxData
		switch(animation)
		{
			case 'idleSit':
				nextFrame = this.idleSit(frame)
				break
			case 'run':
				nextFrame = this.run(frame)
				break
			case 'sit':
				nextFrame = this.sit(frame)
				break
			case 'sneak':
				nextFrame = this.sneak(frame)
				break
			case 'stand':
				nextFrame = this.stand(frame)
				break
			case 'walk':
				nextFrame = this.walk(frame)
				break
			case 'idle':
			default:
				nextFrame = this.idle(frame)
				break
		}
		
		return nextFrame
	}
	
	drawFrame(context: CanvasRenderingContext2D, animation: string, frame: number, position: Vector2, size: Vector2)
	{
		let nextFrame = this.getAnimationFrame(animation, frame)
		
		context.drawImage(this.image,
			nextFrame.x, nextFrame.y, nextFrame.width, nextFrame.height,
			position.x, position.y, size.x, size.y
		)
	}
	
	idle	(frame: number = 0) { return this.getTargetFrame(frame, 0, 1, 0) }
	idleSit	(frame: number = 3) { return this.getTargetFrame(frame, 3, 7, 3) }
	run		(frame: number = 0) { return this.getTargetFrame(frame, 1, 6, 0) }
	sit		(frame: number = 0) { return this.getTargetFrame(frame, 3, 5, 0) }
	sneak	(frame: number = 0) { return this.getTargetFrame(frame, 2, 8, 0) }
	stand	(frame: number = 0) { return this.getTargetFrame(frame, 3, 5, 0) }
	walk	(frame: number = 0) { return this.getTargetFrame(frame, 0, 8, 0) }
}

interface BoxData
{
	height: number
	width: number
	x: number
	y: number
}
