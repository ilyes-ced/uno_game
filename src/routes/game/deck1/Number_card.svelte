<script lang='ts'>
    export let  number: number;
    export let  bg: string;


	const change = (e) => {
		e.target.classList.toggle('flip_card')
	}

</script>



<div class='card bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true"  on:click={change} >
	<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center pointer-events-none relative '+ bg+'_card'   }>
		<div class=' absolute top-0 left-0 text-[20px] text-white font-extrabold the_secondary_number flex items-center pl-2'>
			{number}
		</div>
		<div class={'border-4 border-white w-4/5 h-full  rounded-[50%]  flex items-center justify-center custom_rotate'}>
			<div class={ 'text-[100px] font-extrabold the_main_number  text-white reverse_custom_rotate' } >
				{number}
			</div>
		</div>
		<div class=' absolute bottom-0 right-0 text-[20px] text-white font-extrabold the_secondary_number flex items-center pl-2 rotate-180'>
			{number}
		</div>
	</div>
</div>

