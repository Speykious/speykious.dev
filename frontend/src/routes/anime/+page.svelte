<script lang="ts">
	import '$lib/assets/scss/spey.scss';
	import '$lib/assets/scss/debug.scss';
</script>

<div class="center-panel centered-content">

	<section>
		<h3>My Personal Anime 3x3</h3>
		<div class="content">
			<article class="anime-3x3">
				<button class="anime tl" />
				<button class="anime tc" />
				<button class="anime tr" />
				<button class="anime cl" />
				<button class="anime cc" />
				<button class="anime cr" />
				<button class="anime bl" />
				<button class="anime bc" />
				<button class="anime br" />
			</article>
		</div>
	</section>
</div>

<style lang="scss">
	.center-panel {
		align-items: stretch;
	}

	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
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

	.anime-3x3 {
		display: grid;
		gap: 10px;

		grid-template: 160px 260px 160px / 160px 260px 160px;
		grid-template-areas:
			'tl tc tr'
			'cl cc cr'
			'bl bc br';

		> .anime {
			width: 100%;
			height: 100%;
			box-sizing: border-box;
			border: 10px solid;
			background-size: calc(100% + 10px);
			background-position: center;
			background-repeat: no-repeat;
			transition: 400ms cubic-bezier(0.23, 1, 0.32, 1);

			&:hover {
				background-size: calc(110% + 10px);
				filter: brightness(70%);
			}

			&.tl {
				background-image: url('$lib/assets/images/anime-3x3/0-TL_beyblade.jpg');
				border-color: #6a81c9;
				grid-area: tl;
			}
			&.tc {
				background-image: url('$lib/assets/images/anime-3x3/1-TC_dragon-ball.jpg');
				border-color: #de665e;
				grid-area: tc;
			}
			&.tr {
				background-image: url('$lib/assets/images/anime-3x3/2-TR_inazuma-eleven.jpg');
				border-color: #e2a959;
				grid-area: tr;
			}
			&.cl {
				background-image: url('$lib/assets/images/anime-3x3/3-CL_takagi-san.jpg');
				border-color: #8ad8dd;
				grid-area: cl;
			}
			&.cc {
				background-image: url('$lib/assets/images/anime-3x3/4-CC_toradora.jpg');
				border-color: #f54b73;
				grid-area: cc;
			}
			&.cr {
				background-image: url('$lib/assets/images/anime-3x3/5-CR_assassination-classroom.jpg');
				border-color: #f0ec7e;
				grid-area: cr;
			}
			&.bl {
				background-image: url('$lib/assets/images/anime-3x3/6-BL_reincarnated-as-a-sword.jpg');
				border-color: #ecd397;
				grid-area: bl;
			}
			&.bc {
				background-image: url('$lib/assets/images/anime-3x3/7-BC_bocchi-the-rock.jpg');
				border-color: #e29cac;
				grid-area: bc;
			}
			&.br {
				background-image: url('$lib/assets/images/anime-3x3/8-BR_talentless-nana.jpg');
				border-color: #f0c5d0;
				grid-area: br;
			}
		}
	}
</style>
