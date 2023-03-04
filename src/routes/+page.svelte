<script lang='ts'>


</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>



<div class='bg-gray-700'>

	<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
		<div class='w-full h-full bg-blue-600 rounded-lg  flex flex-col items-center justify-center'>
			<div class="w-full flex items-start justify-start h-6 pl-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-center justify-cenetr'>
					5
				</div>
			</div>
			<div class='bg-white w-full h-3/4  rounded-tr-[60px] rounded-tl-[160px] rounded-br-[160px] rounded-bl-[60px] flex items-center justify-center '>
				<div class='text-[100px] text-blue-600 font-extrabold the_main_number'>
					5
				</div>
			</div>
			<div class="w-full flex items-start justify-end h-6 pr-1">
				<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-center justify-cenetr'>
					5
				</div>
			</div>
		</div>
	</div>
	
</div>


<style>

</style>