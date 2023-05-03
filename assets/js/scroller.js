'use strict'

const transitionMilliseconds = 1000
const autoScrollDelay = 5000
const projectsSelector = '#projectListContainer .project'

let projectsScrollTimer = null
document.addEventListener('DOMContentLoaded', () => {
	updateSelectedProject(1)
	
	if(projectsScrollTimer)
		clearInterval(projectsScrollTimer)
	projectsScrollTimer = setInterval(() => updateSelectedProject(getNextProjectIndex()), autoScrollDelay)
})

function getNextProjectIndex()
{
	let target = 0
	let projects = document.querySelectorAll(projectsSelector)
	projects.forEach((el, i) => {
		if(el.className.indexOf('show') >= 0
			&& el.className.indexOf('previous') < 0
			&& el.className.indexOf('next') < 0)
		{
			target = i
		}
	})
	
	return (target + 1) % projects.length
}

function updateSelectedProject(target)
{
	let projects = document.querySelectorAll(projectsSelector)
	let [previous, current, next] = determineIndices(target, projects.length)
	
	projects.forEach((el, i) => {
		let cn = el.className
					.split(' ')
					.filter(s => ['previous', 'next', 'show'].indexOf(s) < 0)
		
		switch(i)
		{
			case previous:
				cn.push('previous')
				cn.push('show')
				el.className = cn.join(' ')
				break
			case next:
				cn.push('next')
				cn.push('show')
				el.className = cn.join(' ')
				break
			case current:
				cn.push('show')
				el.className = cn.join(' ')
				break
			default:
				if(el.className.indexOf('next') >= 0 || el.className.indexOf('previous') >= 0)
					el.className += ' offscreen'
				
				setTimeout(() => el.className = cn.join(' '), transitionMilliseconds)
				break
		}
	})
}

function determineIndices(target, count)
{
	let current = target
	let previous = target - 1
	if(previous < 0)
		previous = count + previous
	let next = (target + 1) % count
	
	return [previous, current, next]
}
