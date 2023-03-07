<script lang="ts">
    export let  bg: string;

</script>




<div class='bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
    <div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center pointer-events-none '+bg+'_card' }>
        <div class="w-full flex items-start justify-start h-6 pl-1">
            <div class='text-[20px] text-white font-extrabold the_secondary_number h-6 flex items-start justify-cenetr'>
                +4
            </div>
        </div>
        <div class='bg-white w-full h-3/4  rounded-tr-[60px] rounded-tl-[160px] rounded-br-[160px] rounded-bl-[60px] flex items-center justify-center '>
            <div class="absolute  w-8 h-12  ">
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