<script lang='ts'>

	

	import Number_card from './minimal/Number_card.svelte';
	import Arrow_block from './minimal/Arrow_block.svelte';
	import Fours from './minimal/Fours.svelte';
	import Colored_card from './minimal/Colored_card.svelte';
	const green_bg = 'green'
	const red_bg = 'red'
	const yellow_bg = 'yellow'
	const blue_bg = 'blue'
	const green_text = 'text-yellow-400'
	const red_text = 'text-red-600'
	const yellow_text = 'text-blue-600'
	const blue_text = 'text-green-600'
	let cards_tempo = [
		{type: 'number', value:0, color: green_bg },
		{type: 'number', value:1, color: red_bg },
		{type: 'number', value:6, color: green_bg },
		{type: 'number', value:7, color: yellow_bg },
		{type: 'number', value:9, color: yellow_bg },
		{type: 'pick_two', color: green_bg },
		{type: 'block', color: yellow_bg },
		{type: 'block', color: red_bg },
		{type: 'block', color: red_bg },
		{type: 'block', color: red_bg },
		{type: 'block', color: red_bg },
		{type: 'block', color: red_bg },
		{type: 'block', color: red_bg },
		{type: 'change_direction', color: yellow_bg },
		{type: 'pick_four', color: 'black' },
		{type: 'change_color', color: 'black', is_blank:true },
    ]

	let offset = 0


	const raise = (e) => {
		e.target.classList.add('raised')
	}
	const reset = (e) => {
		e.target.classList.remove('raised')
	}


</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>



<div class='bg-gray-700 h-screen w-full flex items-center justify-center'>








	<div id='my_deck' class=' bg-red-600 h-64 flex relative' style={ 'width: '+(cards_tempo.length*40)+'px	' } >
		{#each cards_tempo as card, i}

			{#if i < cards_tempo.length/2 }
				{offset = ( cards_tempo.length/2 - i)}
			{:else }
				{offset = ( i -  cards_tempo.length/2 +1 )}			 
			{/if}


			<div class='absolute transition-all ease-in-out duration-300' on:mouseover={raise} on:focus={raise} on:mouseleave={reset}  style={"left: "+(i*cards_tempo.length*40/(cards_tempo.length+2))+"px; transform: rotatez("+((i*2) - (cards_tempo.length))+"deg); translate: 0px "+(offset*3)+"px; " } >
				{#if card.type === 'number'}
					<Number_card number={card.value} bg={card.color} class='pointer-events-none'  />
				{:else if card.type === 'change_direction' || card.type === 'block' || card.type === 'pick_two'}
					<Arrow_block type={card.type} bg={card.color} class='pointer-events-none'  />
				{:else if card.type === 'pick_four'}
					<Fours type={card.type} bg={card.color} class='pointer-events-none'  />
				{:else if card.type === 'change_color'}
					<Colored_card type={card.type} bg={card.color} is_blank={card.is_blank} class='pointer-events-none' />
				{/if}
			</div>	
		{/each}

	</div>







	
</div>


<style>

</style>