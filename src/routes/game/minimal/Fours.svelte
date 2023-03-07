<script lang="ts">
    export let  bg: string;

</script>


<div>


    <div class='card bg-white p-1 rounded-lg w-32 h-48 cursor-pointer' draggable="true" >
        <div class={'w-full h-full  rounded-lg  flex flex-col items-center justify-center pointer-events-none relative '+bg+'_card' }>
            <div class=' absolute top-0 left-0 text-[20px] text-white font-extrabold the_secondary_number flex items-center pl-2'>
                +4
            </div>
            <div class={'border-4 border-white bg-white w-4/5 h-full  rounded-[50%]  flex items-center justify-center  custom_rotate'}>
                <div class="absolute  w-8 h-12 reverse_custom_rotate ">
                    <div class="bg-green-600 absolute w-full h-full  border border-black  rounded-md card_icon top-8 right-8"></div>
                    <div class="bg-red-600 absolute w-full h-full  border border-black  rounded-md card_icon bottom-8 left-8"></div>
                    <div class="bg-yellow-400 absolute w-full h-full  border border-black  rounded-md card_icon bottom-4 right-2"></div>
                    <div class="bg-blue-600 absolute w-full h-full  border border-black  rounded-md card_icon top-4 left-2"></div>
                </div>
            </div>
            <div class=' absolute bottom-0 right-0 text-[20px] text-white font-extrabold the_secondary_number flex items-center pl-2 rotate-180'>
                +4
            </div>
        </div>
    </div>




</div>