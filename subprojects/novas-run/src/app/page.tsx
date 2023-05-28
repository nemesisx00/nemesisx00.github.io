'use client'

import styles from './page.module.css'
import Game from '@/components/game'

export default function Home()
{
	return (
		<main className={styles.main}>
			<h1>Nova's Run</h1>
			
			<p className={styles.description}>
				Nova's Run is a simple 2D platformer where you play as Nova, a curious
				and adorable little house cat who wants to explore the world. It was
				built with React/Next.js and is rendered in an HTML Canvas element.
			</p>
			
			<div className={styles.game}>
				<Game></Game>
				<div className={styles.gameControls}>
					<span><label>Walk Left:</label> A ⬅</span>
					<span><label>Jump:</label> W ⬆</span>
					<span><label>Walk Right:</label> D ➡</span>
					<span><label>Sneak:</label> S ⬇</span>
					<span><label>Run:</label> V 0</span>
				</div>
			</div>
			
			<div className={styles.attributions}>
				<h2>Third Party Resources</h2>
				<ul>
					<li>Animated Cat Sprite: <a href="https://opengameart.org/content/gudejump-art-pack">Gude!Jump Art Pack</a></li>
				</ul>
			</div>
			
			<div className={styles.footer}>© 2023 Peter Lunneberg</div>
		</main>
	)
}
