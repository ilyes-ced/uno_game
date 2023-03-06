<script lang='ts'>

	import block from '$lib/images/block.svg';
    import arrows from '$lib/images/arrows.svg';
    import second_block from '$lib/images/second_block.svg';



    import Number_card from './deck1/Number_card.svelte';
	import Arrow_block from './deck1/Arrow_block.svelte';
	import Two_four from './deck1/Two_four.svelte';
	import Colored_card from './deck1/Colored_card.svelte';


	let cards = [
		{color: 'bg-yellow-400', text: 'text-yellow-400'},
		{color: 'bg-red-600', text: 'text-red-600'},
		{color: 'bg-blue-600', text: 'text-blue-600'},
		{color: 'bg-green-600', text: 'text-green-600'},
        
	]

    let cards_tempo = [
		//{type: 'number', value:0 },
		//{type: 'number', value:1 },
		//{type: 'number', value:2 },
		//{type: 'number', value:3 },
		//{type: 'number', value:4 },
		//{type: 'number', value:5 },
		//{type: 'number', value:6 },
		//{type: 'number', value:7 },
		//{type: 'number', value:8 },
		//{type: 'number', value:9 },
		{type: 'pick_two' },
		{type: 'pick_four' },
		{type: 'block' },
		{type: 'change_direction' },
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
  		{:else if card.type === 'change_direction' || card.type === 'block'}
			<Colored_card type={card.type} />
  		{:else}
  		  	ddue
  		{/if}
	{/each}






	<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
		<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center bg-red-600 '}>
			<div class="w-full flex items-start justify-start h-6 pl-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr'>
					+4
				</div>
			</div>
			<div class='bg-white w-full h-3/4  rounded-tr-[60px] rounded-tl-[160px] rounded-br-[160px] rounded-bl-[60px] flex items-center justify-center'>
				<div class='stop'></div>
			</div>
			<div class="w-full flex items-start justify-end h-6 pr-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr rotate-180'>
					+4
				</div>
			</div>
		</div>
	</div>





	<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
		<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center bg-red-600 '}>
			<div class="w-full flex items-start justify-start h-6 pl-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr'>
					+4
				</div>
			</div>
			<div class='bg-white w-full h-3/4  rounded-tr-[60px] rounded-tl-[160px] rounded-br-[160px] rounded-bl-[60px] flex items-center justify-center'>
				<img class="w-3/4 h-3/4" src={block} alt="">
			</div>
			<div class="w-full flex items-start justify-end h-6 pr-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr rotate-180'>
					+4
				</div>
			</div>
		</div>
	</div>




	<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
		<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center bg-red-600 '}>
			<div class="w-full flex items-start justify-start h-6 pl-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr'>
					+4
				</div>
			</div>
			<div class='bg-white w-full h-3/4  rounded-tr-[60px] rounded-tl-[160px] rounded-br-[160px] rounded-bl-[60px] flex items-center justify-center'>
				<img class="w-3/4 h-3/4" src={second_block} alt="">
			</div>
			<div class="w-full flex items-start justify-end h-6 pr-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr rotate-180'>
					+4
				</div>
			</div>
		</div>
	</div>


	<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
		<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center bg-red-600 '}>
			<div class="w-full flex items-start justify-start h-6 pl-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr'>
					+4
				</div>
			</div>
			<div class='bg-white w-full h-3/4  rounded-tr-[60px] rounded-tl-[160px] rounded-br-[160px] rounded-bl-[60px] flex items-center justify-center'>
				<img class="w-3/4 h-3/4" src={arrows} alt="">
			</div>
			<div class="w-full flex items-start justify-end h-6 pr-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr rotate-180'>
					+4
				</div>
			</div>
		</div>
	</div>





















	<!-- +4 card -->
	<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
		<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center bg-black '}>
			<div class="w-full flex items-start justify-start h-6 pl-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-end justify-cenetr'>
					<div class='border   border-white w-full h-3/4  rounded-tr-[60px] rounded-tl-[160px] rounded-br-[160px] rounded-bl-[60px] flex items-center justify-center overflow-hidden'>
						<div class="relative  w-4 h-6 bg-red-600 rotate-45 ">
							<div class="absolute w-1/2 h-1/2 top-0 right-0 bg-green-600"></div>
							<div class="absolute w-1/2 h-1/2 bottom-0 left-0 bg-red-600"></div>
							<div class="absolute w-1/2 h-1/2 bottom-0 right-0 bg-yellow-400"></div>
							<div class="absolute w-1/2 h-1/2 top-0 left-0 bg-blue-600"></div>
						</div>
					</div>
				</div>
			</div>
			<div class='border-4 border-white w-full h-3/4  rounded-tr-[60px] rounded-tl-[160px] rounded-br-[160px] rounded-bl-[60px] flex items-center justify-center overflow-hidden'>
				<div class="relative  w-48 h-48 bg-red-600 rotate-45 ">
					<div class="absolute w-1/2 h-1/2 top-0 right-0 bg-green-600"></div>
					<div class="absolute w-1/2 h-1/2 bottom-0 left-0 bg-red-600"></div>
					<div class="absolute w-1/2 h-1/2 bottom-0 right-0 bg-yellow-400"></div>
					<div class="absolute w-1/2 h-1/2 top-0 left-0 bg-blue-600"></div>
				</div>
			</div>
			<div class="w-full flex items-start justify-end h-6 pr-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-end justify-cenetr rotate-180'>
					<div class='border   border-white w-full h-3/4  rounded-tr-[60px] rounded-tl-[160px] rounded-br-[160px] rounded-bl-[60px] flex items-center justify-center overflow-hidden'>
						<div class="relative  w-4 h-6 bg-red-600 rotate-45 ">
							<div class="absolute w-1/2 h-1/2 top-0 right-0 bg-green-600"></div>
							<div class="absolute w-1/2 h-1/2 bottom-0 left-0 bg-red-600"></div>
							<div class="absolute w-1/2 h-1/2 bottom-0 right-0 bg-yellow-400"></div>
							<div class="absolute w-1/2 h-1/2 top-0 left-0 bg-blue-600"></div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>















	<!-- +4 card -->
	<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
		<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center bg-black '}>
			<div class="w-full flex items-start justify-start h-6 pl-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr'>
					+4
				</div>
			</div>
			<div class='bg-white w-full h-3/4  rounded-tr-[60px] rounded-tl-[160px] rounded-br-[160px] rounded-bl-[60px] flex items-center justify-center relative'>
				<div class="absolute  w-8 h-12 ">
					<div class="bg-green-600 absolute w-full h-full  border border-black  rounded-md card_icon top-8 right-8"></div>
					<div class="bg-red-600 absolute w-full h-full  border border-black  rounded-md card_icon bottom-8 left-8"></div>
					<div class="bg-yellow-400 absolute w-full h-full  border border-black  rounded-md card_icon bottom-4 right-2"></div>
					<div class="bg-blue-600 absolute w-full h-full  border border-black  rounded-md card_icon top-4 left-2"></div>
				</div>
			</div>
			<div class="w-full flex items-start justify-end h-6 pr-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr rotate-180'>
					+4
				</div>
			</div>
		</div>
	</div>

	<!-- +2 card -->
	<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
		<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center bg-green-600 '}>
			<div class="w-full flex items-start justify-start h-6 pl-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr'>
					+2
				</div>
			</div>
			<div class='bg-white w-full h-3/4  rounded-tr-[60px] rounded-tl-[160px] rounded-br-[160px] rounded-bl-[60px] flex items-center justify-center relative'>
				<div class="absolute  w-8 h-12 ">
					<div class="absolute w-full h-full bg-green-600 border border-black p-1 rounded-md card_icon top-4 right-4"></div>
					<div class="absolute w-full h-full bg-green-600 border border-black p-1 rounded-md card_icon bottom-4 left-4"></div>
				</div>
			</div>
			<div class="w-full flex items-start justify-end h-6 pr-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr rotate-180'>
					+2
				</div>
			</div>
		</div>
	</div>
















		{#each cards as card}
			<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
				<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center ' + card.color}>
					<div class="w-full flex items-start justify-start h-6 pl-1">
						<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr'>
							5
						</div>
					</div>
					<div class='bg-white w-full h-3/4  rounded-tr-[60px] rounded-tl-[160px] rounded-br-[160px] rounded-bl-[60px] flex items-center justify-center '>
						<div class={ 'text-[100px] font-extrabold the_main_number ' + card.text } >
							5
						</div>
					</div>
					<div class="w-full flex items-start justify-end h-6 pr-1">
						<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr rotate-180'>
							5
						</div>
					</div>
				</div>
			</div>
		{/each}
		

		





































		<!-- filled cards -->
		<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
			<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center red_card relative' }>
				<div class=' absolute top-0 left-0 text-[20px] text-white font-extrabold the_secondary_number flex items-center pl-2'>
					5
				</div>
				<div class={'border-4 border-white w-4/5 h-full  rounded-[50%]  flex items-center justify-center  custom_rotate'}>
					<div class={ 'text-[100px] font-extrabold the_main_number  text-white reverse_custom_rotate' } >
						5
					</div>
				</div>
				<div class=' absolute bottom-0 right-0 text-[20px] text-white font-extrabold the_secondary_number flex items-center pl-2 rotate-180'>
					5
				</div>
			</div>
		</div>












		<!-- filled cards -->
		<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
			<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center bg-black relative' }>
				<div class=" absolute top-1 left-1 w-10 h-10  flex items-start justify-start ">
					<div class='text-[20px] text-white font-extrabold the_secondary_number h-full flex items-end justify '>
						<div class={'border-2 border-white w-4/5 h-full  rounded-[50%]  flex items-center justify-center  custom_rotate overflow-hidden'}>
							<div class="relative  w-6 h-10 bg-blue-900   ">
								<div class="absolute w-1/2 h-1/2 top-0 right-0 bg-green-600"></div>
								<div class="absolute w-1/2 h-1/2 bottom-0 left-0 bg-red-600"></div>
								<div class="absolute w-1/2 h-1/2 bottom-0 right-0 bg-yellow-400"></div>
								<div class="absolute w-1/2 h-1/2 top-0 left-0 bg-blue-600"></div>
							</div>
						</div>
					</div>
				</div>
				<div class={'border-4 border-white w-4/5 h-full  rounded-[50%]  flex items-center justify-center  custom_rotate overflow-hidden'}>
					<div class="relative  w-60 h-48 bg-blue-900   ">
						<div class="absolute w-1/2 h-1/2 top-0 right-0 bg-green-600"></div>
						<div class="absolute w-1/2 h-1/2 bottom-0 left-0 bg-red-600"></div>
						<div class="absolute w-1/2 h-1/2 bottom-0 right-0 bg-yellow-400"></div>
						<div class="absolute w-1/2 h-1/2 top-0 left-0 bg-blue-600"></div>
					</div>
				</div>
				<div class=" absolute bottom-1 right-1 w-10 h-10  flex items-start justify-end ">
					<div class='text-[20px] text-white font-extrabold the_secondary_number h-full flex items-end justify rotate-180'>
						<div class={'border-2 border-white w-4/5 h-full  rounded-[50%]  flex items-center justify-center  custom_rotate overflow-hidden'}>
							<div class="relative  w-6 h-10 bg-blue-900   ">
								<div class="absolute w-1/2 h-1/2 top-0 right-0 bg-green-600"></div>
								<div class="absolute w-1/2 h-1/2 bottom-0 left-0 bg-red-600"></div>
								<div class="absolute w-1/2 h-1/2 bottom-0 right-0 bg-yellow-400"></div>
								<div class="absolute w-1/2 h-1/2 top-0 left-0 bg-blue-600"></div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
















		<!-- filled cards -->
		<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
			<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center bg-black relative' }>
				<div class=' absolute top-0 left-0 text-[20px] text-white font-extrabold the_secondary_number flex items-center pl-2'>
					+4
				</div>
				<div class={'border-4 border-white bg-white w-4/5 h-full  rounded-[50%]  flex items-center justify-center  custom_rotate'}>
					<div class="absolute  w-8 h-12 reverse_custom_rotate ">
						<div class="bg-green-600 absolute w-full h-full  border border-black  rounded-md card_icon top-8 right-8"></div>
						<div class="bg-red-600 absolute w-full h-full  border border-black  rounded-md card_icon bottom-8 left-8"></div>
						<div class="bg-yellow-400 absolute w-full h-full  border border-black  rounded-md card_icon bottom-4 right-2"></div>
						<div class="bg-blue-600 absolute w-full h-full  border border-black  rounded-md card_icon top-4 left-2"></div>
					</div>
				</div>
				<div class=' absolute bottom-0 right-0 text-[20px] text-white font-extrabold the_secondary_number flex items-center pl-2 rotate-180'>
					+4
				</div>
			</div>
		</div>







		<!-- filled cards -->
		<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
			<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center green_card relative' }>
				<div class=' absolute top-0 left-0 text-[20px] text-white font-extrabold the_secondary_number flex items-center pl-2'>
					+2
				</div>
				<div class={'border-4 border-white w-4/5 h-full  rounded-[50%]  flex items-center justify-center  custom_rotate'}>
					<div class="absolute  w-8 h-12 reverse_custom_rotate">
						<div class="absolute w-full h-full bg-white border border-black p-1 rounded-md card_icon top-2 right-2"></div>
						<div class="absolute w-full h-full bg-white border border-black p-1 rounded-md card_icon bottom-2 left-2"></div>
					</div>
				</div>
				<div class=' absolute bottom-0 right-0 text-[20px] text-white font-extrabold the_secondary_number flex items-center pl-2 rotate-180'>
					+2
				</div>
			</div>
		</div>

	
</div>


<style>

</style>