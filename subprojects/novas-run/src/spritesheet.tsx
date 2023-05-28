
import {BoxData, Vector2} from '@/types'

export const PlayerSpritePathLeft: string = './assets/playersprite_1.png'
export const PlayerSpritePathRight: string = './assets/playersprite_0.png'

export class PlayerSpriteSheet
{
	leftToRight: boolean
	frame: number
	frameProps: Vector2
	image: HTMLImageElement
	
	constructor(path: string, frameHeight: number, frameWidth: number, leftToRight: boolean = true)
	{
		this.frame = 0
		this.frameProps = { x: frameWidth, y: frameHeight }
		this.leftToRight = leftToRight
		
		this.image = new Image()
		this.image.src = path
	}
	
	drawFrame(context: CanvasRenderingContext2D, animation: string, frame: number, position: Vector2, size: Vector2)
	{
		let nextFrame = this.getAnimationFrame(animation, frame)
		
		context.drawImage(
			this.image,
			nextFrame.origin.x, nextFrame.origin.y, nextFrame.width, nextFrame.height,
			position.x, position.y, size.x, size.y
		)
	}
	
	getAnimationFrame(animation: string, frame: number)
	{
		let nextFrame: BoxData
		switch(animation)
		{
			case 'idleSit':
				nextFrame = this.idleSit(frame)
				break
			case 'jump':
				nextFrame = this.jump(frame)
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
	
	getTargetFrame(targetFrame: number, row: number, endFrame: number, startFrame: number = 0)
	{
		let height = this.frameProps.y
		let width = this.frameProps.x
		let x = this.frame * this.frameProps.x
		let y = this.frameProps.y * row
		
		this.frame = targetFrame
		
		if(this.leftToRight)
		{
			if(this.frame < startFrame)
				this.frame = endFrame + this.frame
			this.frame = this.frame % endFrame
		}
		else if(this.frame < endFrame)
			this.frame = startFrame
		
		return new BoxData(height, width, new Vector2(x, y))
	}
	
	idle(frame: number)
	{
		return this.leftToRight
			? this.getTargetFrame(frame, 0, 1, 0)
			: this.getTargetFrame(frame, 0, 7, 7)
	}
	
	idleSit(frame: number)
	{
		return this.leftToRight
			? this.getTargetFrame(frame, 3, 7, 3)
			: this.getTargetFrame(frame, 3, 1, 4)
	}
	
	jump(frame: number)
	{
		return this.leftToRight
			? this.getTargetFrame(frame, 1, 8, 7)
			: this.getTargetFrame(frame, 1, 0, 0)
	}
	
	run(frame: number)
	{
		return this.leftToRight
			? this.getTargetFrame(frame, 1, 6, 0)
			: this.getTargetFrame(frame, 1, 2, 7)
	}
	
	sit(frame: number)
	{
		return this.leftToRight
			? this.getTargetFrame(frame, 3, 5, 0)
			: this.getTargetFrame(frame, 3, 3, 7)
	}
	
	sneak(frame: number)
	{
		return this.leftToRight
			? this.getTargetFrame(frame, 2, 8, 0)
			: this.getTargetFrame(frame, 2, 0, 7)
	}
	
	stand(frame: number)
	{
		return this.leftToRight
			? this.getTargetFrame(frame, 3, 5, 0)
			: this.getTargetFrame(frame, 3, 3, 6)
	}
	
	walk(frame: number)
	{
		return this.leftToRight
			? this.getTargetFrame(frame, 0, 8, 0)
			: this.getTargetFrame(frame, 0, 0, 7)
	}
}
