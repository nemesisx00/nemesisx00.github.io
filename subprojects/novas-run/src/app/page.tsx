'use client'

import styles from './page.module.css'
import Game from '@/components/game'

export default function Home()
{
	return (
		<main className={styles.main}>
			<h1>Nova's Run</h1>
			
			<div className={styles.game}>
				<Game></Game>
			</div>
			
			<div className={styles.attributions}>
				<h2>Third Party Resources</h2>
				<ul>
					<li>Animated Cat Sprite: <a href="https://opengameart.org/content/gudejump-art-pack">Gude!Jump Art Pack</a></li>
				</ul>
			</div>
		</main>
	)
}
