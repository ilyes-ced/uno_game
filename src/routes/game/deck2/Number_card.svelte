<script lang='ts'>
    export let  number: number;
    export let  bg: string;


	const change = (e) => {
		e.target.classList.toggle('flip_card')
	}

</script>



<div>



		<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
			<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center pointer-events-none '+bg+'_card' }>
				<div class="w-full flex items-start justify-start h-6 pl-1">
					<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr'>
						{number}
					</div>
				</div>
				<div class='bg-white w-full h-3/4  rounded-tr-[60px] rounded-tl-[160px] rounded-br-[160px] rounded-bl-[60px] flex items-center justify-center '>
					<div class={ 'text-[100px] font-extrabold the_main_number '+bg+'_text'  } >
						{number}
					</div>
				</div>
				<div class="w-full flex items-start justify-end h-6 pr-1">
					<div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr rotate-180'>
						{number}
					</div>
				</div>
			</div>
		</div>



</div>