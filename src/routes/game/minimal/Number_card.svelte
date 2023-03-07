<script lang='ts'>
    export let  number: number;
    export let  bg: string;


	const change = (e) => {
		e.target.classList.toggle('flip_card')
	}

</script>



<div>

    <div class={' p-1 rounded-lg w-32 h-48 cursor-pointer '+bg+'_card'} draggable="true" >
		<div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center pointer-events-none  '}>
			<div class="w-full flex items-start justify-start h-6 pl-1">
				<div class='text-[20px] text-white  h-6 flex items-start justify-cenetr'>
					{number}
				</div>
			</div>
			<div class=' w-full h-3/4  flex items-center justify-center'>
				<div class={ 'text-[100px]  text-white '  } >
                    {number}
                </div>
			</div>
			<div class="w-full flex items-start justify-end h-6 pr-1">
				<div class='text-[20px] text-white  h-6 flex items-start justify-cenetr rotate-180'>
					{number}
				</div>
			</div>
		</div>
	</div>


</div>