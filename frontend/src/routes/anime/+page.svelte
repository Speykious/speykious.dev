<script lang="ts">
	import '$lib/assets/scss/spey.scss';
	import '$lib/assets/scss/debug.scss';
	import '$lib/assets/scss/anime.scss';

	let selectedAnime: string;
</script>

<svelte:head>
	<title>Speykious | My Personal anime 3x3</title>

	<meta property="og:title" content="Speykious | My Personal anime 3x3" />
	<meta property="og:description" content="You can go click on the slots for a lengthy explanation!" />
	<meta property="og:image" content="https://fs.speykious.dev/spey/spey-anime-3x3.png" />
	<meta property="og:image:alt" content="My personal anime 3x3" />
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="theme-color" content="#FF4B77" data-react-helmet="true" />
</svelte:head>

<div class="center-panel centered-content">
	<section id="personal-3x3">
		<h3>My Personal Anime 3x3</h3>
		<noscript class="disclaimer">
			You unfortunately need Javascript to see the descriptions of each anime :(
		</noscript>
		<div class="content">
			<article class="anime-3x3">
				<label>
					<input type="radio" name="anime" value="tl" bind:group={selectedAnime} />
					<div class="anime tl">
						<div class="title">Beyblade</div>
					</div>
				</label>
				<label>
					<input type="radio" name="anime" value="tc" bind:group={selectedAnime} />
					<div class="anime tc">
						<div class="title">Dragon Ball</div>
					</div>
				</label>
				<label>
					<input type="radio" name="anime" value="tr" bind:group={selectedAnime} />
					<div class="anime tr">
						<div class="title">Inazuma Eleven</div>
					</div>
				</label>
				<label>
					<input type="radio" name="anime" value="cl" bind:group={selectedAnime} />
					<div class="anime cl">
						<div class="title">Takagi-san</div>
					</div>
				</label>
				<label>
					<input type="radio" name="anime" value="cc" bind:group={selectedAnime} />
					<div class="anime cc">
						<div class="title">Toradora!</div>
					</div>
				</label>
				<label>
					<input type="radio" name="anime" value="cr" bind:group={selectedAnime} />
					<div class="anime cr">
						<div class="title">Assassination Classroom</div>
					</div>
				</label>
				<label>
					<input type="radio" name="anime" value="bl" bind:group={selectedAnime} />
					<div class="anime bl">
						<div class="title">Reincarnated as a Sword</div>
					</div>
				</label>
				<label>
					<input type="radio" name="anime" value="bc" bind:group={selectedAnime} />
					<div class="anime bc">
						<div class="title">Bocchi The Rock!</div>
					</div>
				</label>
				<label>
					<input type="radio" name="anime" value="br" bind:group={selectedAnime} />
					<div class="anime br">
						<div class="title">Talentless Nana</div>
					</div>
				</label>
			</article>
			{#if selectedAnime === 'tl'}
				<article class="anime-description">
					<h3>Beyblade</h3>
					<h6>The master of taking toys way too seriously!</h6>
					<p>
						Beyblade spinning tops were a huge part of my childhood. We'd fight with them all the
						time in elementary school. IIRC there was even this trend where you'd have to give your
						spinning top to the winner if you lose.
					</p>
					<p>
						I still have a few dozens of these spinning tops today. If I had to collect anything,
						they'd be a strong contender. Back then, it was the era of Metal Fusion, the tops had
						stainless steel rings and they were quite heavy, which is not really the case nowadays.
					</p>
					<p>
						As for the anime itself, it definitely appealed to my little self, and now it's part of
						my nostalgia vibes. If anything, Gingka's character design was a bit of an inspiration
						for my OC (this website's icon), back when I used to invent and draw tons of silly
						shonen manga dragon-ball-esque characters.
					</p>
				</article>
			{:else if selectedAnime === 'tc'}
				<article class="anime-description">
					<h3>Dragon Ball</h3>
					<h6>Mainly the 1986 one</h6>
					<p>
						Dragon Ball was the main manga I'd read as a child, reading the first volumes. The huge
						ones with hundreds of pages, containing the whole story including Dragon Ball Z
						afterwards.
					</p>
					<p>
						I also played a ton of
						<a href="https://dragonball.fandom.com/wiki/Dragon_Ball:_Origins">
							Dragon Ball: Origins
						</a>
						on the DS, and then later on the PS2 I'd play a lot of
						<a href="https://dragonball.fandom.com/wiki/Dragon_Ball_Z:_Budokai_Tenkaichi">
							Dragon Ball Z: Budokai Tenkaichi
						</a>
						number 1 and 3. Had really fun times with these games :D
					</p>
					<p>
						But why did I put it on my <i>Anime</i> 3x3 then? That's because later on, around high school,
						when I started watching anime again, the first thing I did was to binge-watch the entirety
						of the 153 episodes of Dragon Ball.
					</p>
				</article>
			{:else if selectedAnime === 'tr'}
				<article class="anime-description">
					<h3>Inazuma Eleven</h3>
					<h6>What do you mean you don't like sports?</h6>
					<p>
						I didn't like sports, watching football or anything of the sort. But somehow I really
						liked Inazuma Eleven as a kid. Probably because I thought it was cool to shoot in a ball
						and suddenly it's on fire.
					</p>
					<p>Definitely a chunibyo moment :v</p>
				</article>
			{:else if selectedAnime === 'cl'}
				<article class="anime-description">
					<h3>Takagi-san</h3>
					<h6>It's so sweet</h6>
					<p>
						Takagi-san simply was my introduction to romance anime. Every moment just makes my heart
						bubble up. uwu
					</p>
					<p>
						Everything in this world's just chill, until you get chills from Nishikata returning
						some of Takagi's sweet energy back. =w=
					</p>
					<p>
						Oh and the songs are just chef's kiss. I listen to them occasionally. I like to imagine
						Takagi as a future singer where all of these are her singles.
					</p>
				</article>
			{:else if selectedAnime === 'cc'}
				<article class="anime-description">
					<h3>Toradora!</h3>
					<h6>A masterpiece of character drama.</h6>
					<p>
						One day I decided to watch Toradora on a whim, because I was seeing the title fly by
						occasionally and it seemed like a popular-ish 2010 anime that I may enjoy.
					</p>
					<p>
						As it turns out, it did way more than that. It put me to the edge of my seat like no
						anime before ever has. A very special tension the author managed to make me feel with an
						intensity so strong, that to this day it is the only anime I have decided to rewatch.
						I've done it twice already, and I'm gonna do it again every December as per the Toradora
						rewatch tradition.
					</p>
					<p>
						If you want a recommendation and you like romance, this one is definitely THE anime I'm
						gonna point to. So if you haven't watched it or are considering watching it, go ahead.
						It is, to me, by far the best romance anime I have ever watched.
					</p>
					<p>
						I'm sure not everybody will agree with me on that, though. Some people can't stand
						Toradora because they can't get past some details that ruin the show for them. And
						that's ok. But it'll stay in this special place in my heart for moving me to such a
						great extent.
					</p>
				</article>
			{:else if selectedAnime === 'cr'}
				<article class="anime-description">
					<h3>Assassination Classroom</h3>
					<h6>Oh god why am I crying</h6>
					<p>
						While it's not an anime I rewatched, at the moment, as I like to put it, it made me cry
						more than the first time someone ever broke my heart. (it was a childhood crush lol)
					</p>
				</article>
			{:else if selectedAnime === 'bl'}
				<article class="anime-description">
					<h3>Reincarnated as a Sword</h3>
					<h6>Fran best girl :D</h6>
					<p>
						I <i>had</i> to put at least one isekai in there... I've watched a lot of them, like Slime,
						Shield Hero, Re:Zero, Mushoku Tensei, etc. I enjoy them all, but I thought this one would
						fit better in my 3x3.
					</p>
					<p>
						It's a title almost as silly as the one where you get reincarnated as a vending machine,
						but the story is actually kinda wholesome and you end up rooting for the main character.
						Fran is cute and I wanna pat her head and I also enjoy watching her getting stronger :3
					</p>
					<p>
						Notably though, one thing I particularly loved from this anime is its ending:
						<a href="https://music.youtube.com/watch?v=6EkIpq-smII">more&lt;STRONGLY</a>
						by Maon Kurosaki. It captures the atmosphere of this anime so well and it hooked me every
						time. I also listen to it occasionally.
					</p>
					<p>
						Sadly though, I learned right after I finished making a
						<a href="https://lazer.ppy.sh/beatmapsets/2003475#mania/4166408">
							4k mania map of this song
						</a>
						that...
						<a
							href="https://www.animenewsnetwork.com/news/2023-02-28/singer-maon-kurosaki-passes-away/.195420"
						>
							Maon Kurosaki had died of health complications
						</a>
						in February 2023. more&lt;STRONGLY was her last track. It shocked me to say the least. Had
						to process it for a few days.
					</p>
				</article>
			{:else if selectedAnime === 'bc'}
				<article class="anime-description">
					<h3>Bocchi The Rock!</h3>
					<h6>SHE'S JUST LIKE ME FR</h6>
					<p>... <i>or is she? <strong>*VSauce music*</strong></i></p>
					<p>
						I love music, and I enjoy this anime in the same manner. It has extremely relatable
						characters. A lot of people relate to Bocchi, the socially anxious introvert doing
						things a lot of us have done ourselves, as if it was a direct callout. She
						re-skyrocketed the "she's just like me fr" meme all by herself.
					</p>
					<p>
						It's almost guaranteed that you relate to at least one of these characters. If it's not
						Bocchi, it's gonna be someone else, because they all have interesting social traits that
						feel very real. Ryo for instance is the character I relate to the most, for two traits
						in particular: being a loner introvert and flexing on your skills with confidence (see
						bass shop scene in episode 12). Picture me playing Arcaea on my phone in the metro and a
						few people seeing that and being impressed lol. (Very fun rhythm game btw, can totally
						recommend)
					</p>
					<p>
						And of course the main aspect: the music is top notch. I regularly listen to it and
						sometimes play maps of these in osu!.
					</p>
				</article>
			{:else if selectedAnime === 'br'}
				<article class="anime-description">
					<h3>Talentless Nana</h3>
					<h6>Among Us Hero Academia</h6>
					<p>
						This one is definitely something. Every single end in each episode leaves you with a
						cliffhanger bigger than the previous one.
					</p>
					<p>
						It was so strong that it had to be the first anime I watch whenever it was its airing
						day, and when it finished, so strong that I had to go see the manga. I very, very rarely
						do that, as you can see from the mangas I've listed on
						<a href="https://anilist.co/user/Speykious/mangalist">my anilist</a>.
					</p>
					<p>
						I'm usually an anime-only enjoyer, because I want the effect of the amazing animation
						coupled with the discovery of the story plots. I also care so much about spoilers that
						to me, merely reading a synopsis is a spoiler. Sometimes I'd start watching an anime
						simply by looking at its cover image / thumbnail and thinking "huh, looks cool, maybe
						I'll enjoy it".
					</p>
					<p>
						This one though, if I remember correctly, I started watching it because of a Gigguk
						"Anime [insert season] [insert year] in a nutshell" video describing it as "Among Us
						Hero Academia". That definitely got me curious lol.
					</p>
				</article>
			{/if}
		</div>
	</section>
</div>

<style lang="scss">
	.center-panel {
		align-items: stretch;

		> section {
			display: flex;
			flex-direction: column;
			align-items: center;
			gap: 20px;

			margin: 40px 0;
			padding: 30px;
			background: #ffffff10;
			border-top: 2px solid #ffffff30;
			border-bottom: 2px solid #ffffff30;

			> .content {
				display: flex;
				justify-content: center;
				gap: 20px;
			}
		}
	}
</style>
