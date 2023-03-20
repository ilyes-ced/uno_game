<script lang='ts'>
	export let  type: string;
    export let  bg: string;


	//import block from '$lib/images/white_block.svg';
    //import arrows from '$lib/images/white_arrows.svg';

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
						<div class="absolute  w-8 h-12 reverse_custom_rotate">
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



{#if type === 'block'}
	<div class={' p-1 rounded-lg w-32 h-48 cursor-pointer '+bg+'_card pointer-events-none ' }  >
		<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center   '}>
			<div class="w-full flex items-start justify-start h-6 p-1">
				<div class=' w-6 h-6 relative flex items-center justify-center'>
					<div class='absolute w-6 h-6 rounded-[50%] border-2 border-white'></div>
					<div class='absolute h-6  border-[1px] border-white -rotate-45  bg-white '></div>
				</div>
			</div>
			<div class=' w-full h-3/4  flex items-center justify-center relative'>
	            <div class='absolute w-16 h-16 rounded-[50%] border-4 border-white'></div>
	            <div class='absolute h-16 w-1 -rotate-45 bg-white '></div>
			</div>
			<div class="w-full flex items-end justify-end h-6 p-1 ">
				<div class=' w-6 h-6 relative flex items-center justify-center'>
					<div class='absolute w-6 h-6 rounded-[50%] border-2 border-white'></div>
					<div class='absolute h-6  border-[1px] border-white -rotate-45  bg-white '></div>
				</div>
			</div>
		</div>
	</div>
{:else if type === 'change_direction'}
	<div class={'p-1 rounded-lg w-32 h-48 cursor-pointer '+bg+'_card pointer-events-none'  } >
		<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center  '}>
			<div class="w-full flex items-start justify-start h-6   ">
				<div class='w-6 h-6 mt-3'>
					<div class='absolute w-6 h-2 rounded-br-md border-b-2 border-r-2 border-white mb-2 ml-1 rotate-[135deg]  '>
						<div class='w-2 h-2 border-b-2 border-l-2  border-white  absolute top-[3px]  rotate-45'></div>
					</div>
					<div class='absolute w-6 h-2 rounded-br-md  border-b-2 border-r-2 border-white mt-2 mr-1 -rotate-45  '>
						<div class='w-2 h-2 border-b-2 border-l-2  border-white  absolute top-[3px]  rotate-45'></div>
					</div>
				</div>
			</div>
			<div class=' w-full h-3/4  flex items-center justify-center relative'>
	            <div class='absolute w-12 h-4 rounded-br-xl border-b-2 border-r-2 border-white mb-4 ml-2 rotate-[135deg]  '>
					<div class='w-4 h-4 border-b-2 border-l-2  border-white  absolute top-[7px]  rotate-45'></div>
				</div>
	            <div class='absolute w-12 h-4 rounded-br-xl  border-b-2 border-r-2 border-white mt-4 mr-2 -rotate-45  '>
					<div class='w-4 h-4 border-b-2 border-l-2  border-white  absolute top-[7px]  rotate-45'></div>
				</div>
			</div>
			<div class="w-full flex items-start justify-end h-6 pr-1">
				<div class='w-6 h-6 mb-3'>
					<div class='absolute w-6 h-2 rounded-br-md border-b-2 border-r-2 border-white mb-2 ml-1 rotate-[135deg]  '>
						<div class='w-2 h-2 border-b-2 border-l-2  border-white  absolute top-[3px]  rotate-45'></div>
					</div>
					<div class='absolute w-6 h-2 rounded-br-md  border-b-2 border-r-2 border-white mt-2 mr-1 -rotate-45  '>
						<div class='w-2 h-2 border-b-2 border-l-2  border-white  absolute top-[3px]  rotate-45'></div>
					</div>
				</div>	
			</div>
		</div>
	</div>
{:else if type === 'pick_two'}
	<div class={'p-1 rounded-lg w-32 h-48 cursor-pointer '+bg+'_card pointer-events-none' }  >
		<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center  '}>
			<div class="w-full flex items-start justify-start h-6 pl-1">
				<div class='text-[20px] text-white  h-6 flex items-start justify-cenetr'>
					+2
				</div>
			</div>
			<div class=' w-full h-3/4  flex items-center justify-center'>
				<div class="absolute  w-8 h-12 ">
	                <div class="absolute w-full h-full  border-2  p-1 rounded-md  top-4 right-2"></div>
	                <div class="absolute w-full h-full  border-2  p-1 rounded-md  bottom-4 left-2"></div>
	            </div>
			</div>
			<div class="w-full flex items-start justify-end h-6 pr-1">
				<div class='text-[20px] text-white  h-6 flex items-start justify-cenetr rotate-180'>
					+2
				</div>
			</div>
		</div>
	</div>
{/if}
