<script lang='ts'>

	import block from '$lib/images/block.svg';
    import arrows from '$lib/images/arrows.svg';
    import second_block from '$lib/images/second_block.svg';



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

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>



<div class='bg-gray-700  grid grid-cols-6 gap-4 '>









	<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
		<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center black_card' }>
			<div class='bg-red-600 border-white border-4 w-full h-3/4  rounded-tr-[60px] rounded-tl-[160px] rounded-br-[160px] rounded-bl-[60px] flex items-center justify-center '>
				<div class={ 'text-[50px]    font-extrabold uno_text  text-white -rotate-[20deg] text-yellow-300'  } >
					UNO
				</div>
			</div>
		</div>
	</div>


	<div class='card bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true"   >
		<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center pointer-events-none  black_card'   }>
			<div class={'border-4 border-white w-4/5 h-full  rounded-[50%] bg-red-600 flex items-center justify-center relative  custom_rotate'}>
				<div class={ 'text-[50px]    font-extrabold uno_text  text-white -rotate-[50deg] text-yellow-300' } >
					UNO
				</div>
			</div>
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


<style>

</style>