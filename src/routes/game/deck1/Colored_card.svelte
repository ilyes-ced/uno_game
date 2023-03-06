<script lang="ts">
    export let  is_blank: Boolean;
    export let  bg: string;

</script>


<div>

	<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
		<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center  relative  '+ bg +'_card' }>
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
			{#if is_blank }
				<div class={'border-4 bg-white border-white w-4/5 h-full  rounded-[50%]  flex items-center justify-center  custom_rotate overflow-hidden'}>
					
				</div>
			{:else}
				<div class={'border-4 border-white w-4/5 h-full  rounded-[50%]  flex items-center justify-center  custom_rotate overflow-hidden'}>
					<div class="relative  w-60 h-48 bg-blue-900   ">
						<div class="absolute w-1/2 h-1/2 top-0 right-0 bg-green-600"></div>
						<div class="absolute w-1/2 h-1/2 bottom-0 left-0 bg-red-600"></div>
						<div class="absolute w-1/2 h-1/2 bottom-0 right-0 bg-yellow-400"></div>
						<div class="absolute w-1/2 h-1/2 top-0 left-0 bg-blue-600"></div>
					</div>
				</div>
			{/if}
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



</div>