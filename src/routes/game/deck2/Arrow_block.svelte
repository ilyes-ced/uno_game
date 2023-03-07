<script lang='ts'>
	export let  type: string;
    export let  bg: string;


	import block from '$lib/images/white_block.svg';
    import arrows from '$lib/images/white_arrows.svg';

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


	<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
		<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center pointer-events-none '+bg+'_card' }>
			<div class="w-full flex items-start justify-start h-6 pl-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr'>
					{#if type === 'block'}
						<img class="w-6 h-6 mt-1" src={block} alt="">
					{:else if type === 'change_direction'}
						<img class="w-6 h-6 mt-1 -rotate-[10deg] " src={arrows} alt="">
			 		{:else if type === 'pick_two'}
                	    +2
                	{/if}
				</div>
			</div>
			<div class='bg-white w-full h-3/4  rounded-tr-[60px] rounded-tl-[160px] rounded-br-[160px] rounded-bl-[60px] flex items-center justify-center '>
				{#if type === 'block'}
					<img class=" h-20 w-h-20" src={block} alt="">
				{:else if type === 'change_direction'}
					<img class=" -rotate-[10deg] h-20 w-h-20" src={arrows} alt="">
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
						<img class="w-6 h-6 mt-1" src={block} alt="">
					{:else if type === 'change_direction'}
						<img class="w-6 h-6 mt-1 -rotate-[10deg] " src={arrows} alt="">
			 		{:else if type === 'pick_two'}
                	    +2
                	{/if}
				</div>
			</div>
		</div>
	</div>


</div>