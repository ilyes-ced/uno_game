<script lang="ts">
    export let  is_blank: Boolean;
    export let  bg: string;

</script>




<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
	<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center   pointer-events-none   '+ bg +'_card'}>
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
			{#if is_blank }
				<div class={' bg-white w-full h-full'}>
				</div>
			{:else}
				<div class="relative  w-48 h-48 bg-red-600 rotate-45 ">
					<div class="absolute w-1/2 h-1/2 top-0 right-0 bg-green-600"></div>
					<div class="absolute w-1/2 h-1/2 bottom-0 left-0 bg-red-600"></div>
					<div class="absolute w-1/2 h-1/2 bottom-0 right-0 bg-yellow-400"></div>
					<div class="absolute w-1/2 h-1/2 top-0 left-0 bg-blue-600"></div>
				</div>
			{/if}
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


	