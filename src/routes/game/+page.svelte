<script lang='ts'>

	import block from '$lib/images/block.svg';
    import arrows from '$lib/images/arrows.svg';
    import second_block from '$lib/images/second_block.svg';



    import Number_card from './deck1/Number_card.svelte';
	import Arrow_block from './deck1/Arrow_block.svelte';
	import Fours from './deck1/Fours.svelte';
	import Colored_card from './deck1/Colored_card.svelte';


	let cards = [
		{color: 'bg-yellow-400', text: 'text-yellow-400'},
		{color: 'bg-red-600', text: 'text-red-600'},
		{color: 'bg-blue-600', text: 'text-blue-600'},
		{color: 'bg-green-600', text: 'text-green-600'},
        
	]

    let cards_tempo = [
		{type: 'number', value:0 },
		{type: 'number', value:1 },
		{type: 'number', value:2 },
		{type: 'number', value:3 },
		{type: 'number', value:4 },
		{type: 'number', value:5 },
		{type: 'number', value:6 },
		{type: 'number', value:7 },
		{type: 'number', value:8 },
		{type: 'number', value:9 },
		{type: 'pick_two' },
		{type: 'block' },
		{type: 'change_direction' },
		{type: 'pick_four' },
		{type: 'change_color' },
    ]
	let more_types = [ "blank", "pick_four", "change_color" ]

    


</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>



<div class='bg-gray-700  grid grid-cols-5 '>

	{#each cards_tempo as card}
		{#if card.type === 'number'}
		 	<Number_card number={card.value} />
  		{:else if card.type === 'change_direction' || card.type === 'block' || card.type === 'pick_two'}
			<Arrow_block type={card.type} />
  		{:else if card.type === 'pick_four'}
		  	<Fours type={card.type} />
  		{:else if card.type === 'change_color'}
		  	<Colored_card type={card.type} />
  		{/if}
	{/each}






	
</div>


<style>

</style>