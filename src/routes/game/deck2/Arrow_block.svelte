<script lang='ts'>
	export let  type: string;
    export let  bg: string;


SWItch statement import based on bg


	import red_block from '$lib/images/red_block.svg';
	import blue_block from '$lib/images/blue_block.svg';
	import green_block from '$lib/images/green_block.svg';
	import yellow_block from '$lib/images/yellow_block.svg';
	
    import red_arrows from '$lib/images/red_arrows.svg';
    import blue_arrows from '$lib/images/blue_arrows.svg';
    import green_arrows from '$lib/images/green_arrows.svg';
    import yellow_arrows from '$lib/images/yellow_arrows.svg';
	let block_arr = [ 
		red_block,
		blue_block,
		green_block,
		yellow_block
	 ]
	arrows_arr = [ 
		red_arrows,
		blue_arrows,
		green_arrows,
		yellow_arrows
	 ]


	//let red_block = '$lib/images/red_block.svg';
	//let blue_block = '$lib/images/blue_block.svg';
	//let yellow_block = '$lib/images/green_block.svg';
	//let green_block = '$lib/images/yellow_block.svg';
    //let red_arrows = '$lib/images/red_arrows.svg';
    //let blue_arrows = '$lib/images/blue_arrows.svg';
    //let yellow_arrows = '$lib/images/green_arrows.svg';
    //let green_arrows = '$lib/images/yellow_arrows.svg';

/**
 * 
 * 
 * 	<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
		<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center bg-red-600 '}>
			<div class="w-full flex items-start justify-start h-6 pl-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr'>
					+4
				</div>
			</div>
			<div class='bg-white w-full h-3/4  rounded-tr-[60px] rounded-tl-[160px] rounded-br-[160px] rounded-bl-[60px] flex items-center justify-center'>
				{#if type === 'pick_block'}
					<img class="w-3/4 h-3/4" src={arrows} alt="">
				{:else if type === 'change_direction'}
					<img class="w-3/4 h-3/4" src={arrows} alt="">
				{:else if type === 'pick_two'}
					<div class={'border-4 border-white w-4/5 h-full  rounded-[50%]  flex items-center justify-center  custom_rotate'}>
						<div class="absolute  w-8 h-12 ">
							<div class="absolute w-full h-full bg-white border border-black p-1 rounded-md card_icon top-2 right-2"></div>
							<div class="absolute w-full h-full bg-white border border-black p-1 rounded-md card_icon bottom-2 left-2"></div>
						</div>
					</div>
			 	{/if}
			</div>
			<div class="w-full flex items-start justify-end h-6 pr-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr rotate-180'>
					+4
				</div>
			</div>
		</div>
	</div>
 * 
*/


</script>



<div>

	<img class="w-6 h-6 mt-1" src={`red_block_import`} alt="">
	<img class="w-6 h-6 mt-1" src={blue_block_import} alt="">
	<img class="w-6 h-6 mt-1" src={red_arrows_import} alt="">
	<img class="w-6 h-6 mt-1" src={blue_arrows_import} alt="">



	<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
		<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center pointer-events-none '+bg+'_card' }>
			<div class="w-full flex items-start justify-start h-6 pl-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr'>
					{#if type === 'block'}
						<img class="w-6 h-6 mt-1" src={'$lib/images/'+bg+'_arrows.svg'} alt="">
					{:else if type === 'change_direction'}
						<img class="w-6 h-6 mt-1 -rotate-[10deg] " src={'$lib/images/'+bg+'_arrows.svg'} alt="">
			 		{:else if type === 'pick_two'}
                	    +2
                	{/if}
				</div>
			</div>
			<div class='bg-white w-full h-3/4  rounded-tr-[60px] rounded-tl-[160px] rounded-br-[160px] rounded-bl-[60px] flex items-center justify-center '>
				{#if type === 'block'}
					<img class=" h-20 w-h-20" src={'$lib/images/'+bg+'_block.svg'} alt="">
				{:else if type === 'change_direction'}
					<img class=" -rotate-[10deg] h-20 w-h-20" src={'$lib/images/'+bg+'_arrows.svg'} alt="">
			 	{:else if type === 'pick_two'}
                    <div class="absolute  w-8 h-12 ">
						<div class={"absolute w-full h-full border border-black p-1 rounded-md card_icon top-2 right-2 "+bg+'_card'}></div>
						<div class={"absolute w-full h-full border border-black p-1 rounded-md card_icon bottom-2 left-2 "+bg+'_card'}></div>
					</div>
                {/if}
			</div>
			<div class="w-full flex items-start justify-end h-6 pr-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr rotate-180'>
					{#if type === 'block'}
						<img class="w-6 h-6 mt-1" src={'$lib/images/'+bg+'_block.svg'} alt="">
					{:else if type === 'change_direction'}
						<img class="w-6 h-6 mt-1 -rotate-[10deg] " src={'$lib/images/'+bg+'_arrows.svg'} alt="">
			 		{:else if type === 'pick_two'}
                	    +2
                	{/if}
				</div>
			</div>
		</div>
	</div>


</div>