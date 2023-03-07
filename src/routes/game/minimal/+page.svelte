<script lang='ts'>


	import Number_card from './Number_card.svelte';
	import Arrow_block from './Arrow_block.svelte';
	import Fours from './Fours.svelte';
	import Colored_card from './Colored_card.svelte';
	const green_bg = 'green'
	const red_bg = 'red'
	const yellow_bg = 'yellow'
	const blue_bg = 'blue'
	const green_text = 'text-yellow-400'
	const red_text = 'text-red-600'
	const yellow_text = 'text-blue-600'
	const blue_text = 'text-green-600'

	let cards_tempo = [
		{type: 'number', value:0, color: yellow_bg },
		{type: 'number', value:0, color: red_bg },
		{type: 'number', value:0, color: blue_bg },
		{type: 'number', value:0, color: green_bg },
		{type: 'number', value:1, color: yellow_bg },
		{type: 'number', value:1, color: red_bg },
		{type: 'number', value:1, color: blue_bg },
		{type: 'number', value:1, color: green_bg },
		{type: 'number', value:2, color: yellow_bg },
		{type: 'number', value:2, color: red_bg },
		{type: 'number', value:2, color: blue_bg },
		{type: 'number', value:2, color: green_bg },
		{type: 'number', value:3, color: yellow_bg },
		{type: 'number', value:3, color: red_bg },
		{type: 'number', value:3, color: blue_bg },
		{type: 'number', value:3, color: green_bg },
		{type: 'number', value:4, color: yellow_bg },
		{type: 'number', value:4, color: red_bg },
		{type: 'number', value:4, color: blue_bg },
		{type: 'number', value:4, color: green_bg },
		{type: 'number', value:5, color: yellow_bg },
		{type: 'number', value:5, color: red_bg },
		{type: 'number', value:5, color: blue_bg },
		{type: 'number', value:5, color: green_bg },
		{type: 'number', value:6, color: yellow_bg },
		{type: 'number', value:6, color: red_bg },
		{type: 'number', value:6, color: blue_bg },
		{type: 'number', value:6, color: green_bg },
		{type: 'number', value:7, color: yellow_bg },
		{type: 'number', value:7, color: red_bg },
		{type: 'number', value:7, color: blue_bg },
		{type: 'number', value:7, color: green_bg },
		{type: 'number', value:8, color: yellow_bg },
		{type: 'number', value:8, color: red_bg },
		{type: 'number', value:8, color: blue_bg },
		{type: 'number', value:8, color: green_bg },
		{type: 'number', value:9, color: yellow_bg },
		{type: 'number', value:9, color: red_bg },
		{type: 'number', value:9, color: blue_bg },
		{type: 'number', value:9, color: green_bg },
		{type: 'pick_two', color: yellow_bg },
		{type: 'pick_two', color: red_bg },
		{type: 'pick_two', color: blue_bg },
		{type: 'pick_two', color: green_bg },
		{type: 'block', color: yellow_bg },
		{type: 'block', color: red_bg },
		{type: 'block', color: blue_bg },
		{type: 'block', color: green_bg },
		{type: 'change_direction', color: yellow_bg },
		{type: 'change_direction', color: red_bg },
		{type: 'change_direction', color: blue_bg },
		{type: 'change_direction', color: green_bg },
		{type: 'pick_four', color: yellow_bg },
		{type: 'pick_four', color: red_bg },
		{type: 'pick_four', color: blue_bg },
		{type: 'pick_four', color: green_bg },
		{type: 'pick_four', color: 'black' },
		{type: 'change_color', color: yellow_bg },
		{type: 'change_color', color: red_bg },
		{type: 'change_color', color: blue_bg },
		{type: 'change_color', color: green_bg },
		{type: 'change_color', color: 'black' },
		{type: 'change_color', color: 'black', is_blank:true },
    ]
	let more_types = [ "blank", "pick_four", "change_color" ]

    
</script>


<div class='bg-gray-700  grid grid-cols-5'>





    <div class='black_card  rounded-lg w-32 h-48 cursor-pointer relative flex items-end justify-center' draggable="true" >
		<div class='absolute w-24 h-16 rounded-[50%] border-4 border-red-600  top-1/2 -translate-y-1/2 -rotate-12  flex items-center justify-center'>
			<div class='text-[45px]  minimal_uno_text  font-semibold   text-white  text-yellow-300 '  >
				UNO
			</div>
		</div>
		<div class="bg-white h-1 w-full mb-4 flex  ">
			<div class='h-full w-1/4 bg-red-600' ></div>
			<div class='h-full w-1/4 bg-green-600' ></div>
			<div class='h-full w-1/4 bg-blue-600' ></div>
			<div class='h-full w-1/4 bg-yellow-400' ></div>
		</div>
	</div>











	{#each cards_tempo as card, i}
		{#if card.type === 'number'}
		 	<Number_card number={card.value} bg={card.color}  />
  		{:else if card.type === 'change_direction' || card.type === 'block' || card.type === 'pick_two'}
			<Arrow_block type={card.type} bg={card.color}  />
  		{:else if card.type === 'pick_four'}
		  	<Fours type={card.type} bg={card.color}  />
  		{:else if card.type === 'change_color'}
		  	<Colored_card type={card.type} bg={card.color} is_blank={card.is_blank} />
  		{/if}
	{/each}











	

</div>