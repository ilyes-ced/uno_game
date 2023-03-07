<script lang="ts">

</script>


<div>




	<div class='absolute '   style={"left: "+(i*cards_tempo.length*40/(cards_tempo.length+2))+"px; transform: rotatez(-"+((i*2) - (cards_tempo.length))+"deg); translate: 0px -"+(offset*3)+"px; " } >
        <div class='black_card  rounded-lg w-32 h-48 cursor-pointer relative flex items-end justify-center' draggable="true" >
            <div class='absolute w-24 h-16 rounded-[50%] border-4 border-red-600  top-1/2 -translate-y-1/2 -rotate-12  flex items-center justify-center'>
                <div class='text-[45px]  minimal_uno_text  font-semibold     text-yellow-300 '  >
                    UNO
                </div>
            </div>
            <div class="bg-white h-1 w-full mb-4 flex  ">
                <div class='h-full w-1/4 bg-red-600' ></div>
                <div class='h-full w-1/4 bg-green-600' ></div>
                <div class='h-full w-1/4 bg-blue-600' ></div>
                <div class='h-full w-1/4 bg-yellow-400' ></div>
            </div>
        </div>
    </div>	




</div>