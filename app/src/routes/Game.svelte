

<script lang='ts'>

    import Number_card from './minimal/Number_card.svelte';
    import Arrow_block from './minimal/Arrow_block.svelte';
    import Fours from './minimal/Fours.svelte';
    import Colored_card from './minimal/Colored_card.svelte';
    import Back from './minimal/Back.svelte';
    const green_bg = 'green'
    const red_bg = 'red'
    const yellow_bg = 'yellow'
    const blue_bg = 'blue'
    const green_text = 'text-yellow-400'
    const red_text = 'text-red-600'
    const yellow_text = 'text-blue-600'
    const blue_text = 'text-green-600'
    let cards_tempo = [
        {type: 'number', value:0, color: green_bg },
        {type: 'number', value:1, color: red_bg },
        {type: 'number', value:6, color: green_bg },
        {type: 'number', value:7, color: yellow_bg },
        {type: 'block', color: red_bg },
        {type: 'change_direction', color: yellow_bg },
        {type: 'pick_four', color: 'black' },
        {type: 'change_color', color: 'black', is_blank:true },
    ]

    let offset = 0


    const raise = (e) => {
        //e.target.classList.add('raised')
        //console.log(e.target.parentElement)
    }
    const reset = (e) => {
        e.target.classList.remove('raised')
    }

	import { onMount } from 'svelte';

	let time = new Date();
	let center_box 
    let center_box_cards = [
        {type: 'block', color: red_bg },
        {type: 'change_direction', color: yellow_bg },
    ]

	// these automatically update when `time`
	// changes, because of the `$:` prefix
	$: hours = time.getHours();
	$: minutes = time.getMinutes();
	$: seconds = time.getSeconds();

    let socket

	onMount(() => {
		const interval = setInterval(() => {
			time = new Date();
		}, 1000);


        socket = new WebSocket("ws:/localhost:5000/")
        socket.addEventListener("open", ()=> {
            console.log("Opened")
	    });



		return () => {
			clearInterval(interval);
		};


    })
        const test = (e) => {
            //console.log('hello')
            //console.log(center_box)
//
            //var rect = center_box.getBoundingClientRect();
            //console.log(rect.top, rect.right, rect.bottom, rect.left);
//
//console.log(e.target.style.translate)
//e.target.style.top = 0
//e.target.style.left = 0
//e.target.style.top = rect.top
//e.target.style.left = rect.left
//console.log(e.target.getBoundingClientRect())

            let new_element = e.target.cloneNode(true);
            console.log(new_element)


            var rect = e.target.getBoundingClientRect();
            console.log(rect.top, rect.right, rect.bottom, rect.left);
            
            center_box.firstChild.nextSibling.remove()
            new_element.style.backgroundColor = 'red'
            console.log('++++++++++++++++++++++++', rect.top, rect.left)
            new_element.style.position = 'absolute'
            new_element.style.top = rect.top
            new_element.style.left = rect.left
            console.log( new_element.getBoundingClientRect())
            center_box.firstChild.appendChild(new_element)



            center_box_cards = [...center_box_cards, cards_tempo[e.target.id]]
            cards_tempo.splice(e.target.id, 1)
            cards_tempo = [...cards_tempo]
            console.log(center_box_cards)
        }

        let last_card
        $: last_card = center_box_cards[center_box_cards.length-1]
  

</script>





<div class='  w-full flex items-center justify-between grow'>
    <div class="absolute ">
        {hours + ':'+minutes+':'+seconds}
        
    </div>

    <div id='game_board_back' class='absolute bottom-0 left-0  w-full h-full'></div>
    <div class="w-full h-full flex  " id='game_board'>  
        <!--  LEFT  -->
        <div class='w-1/4 h-full  flex items-center justify-center ' >
            <div class='w-2/3 h-2/3  flex items-center justify-center'>
                <div class='w-2/3 h-full  flex items-center justify-center '>
                    <div id='' class='flex relative bg-green-600 w-full'  style={ 'height: '+(cards_tempo.length*40)+'px	' } >
                        {#each cards_tempo as card, i}
                            <div class='absolute transition-all ease-in-out duration-300 -rotate-90  left-1/2 -translate-x-1/2 '   style={"top: "+(i*cards_tempo.length*40/(cards_tempo.length+3))+"px;"} >
                                <Back />
                            </div>	
                        {/each}
                    </div>
                </div>
            </div>
        </div>
        <div class='w-1/2 h-full  flex flex-col' >
            <!--  TOP  -->
            <div class='w-full h-2/3  flex items-center justify-center     ' >
                <div class='w-full h-full   flex items-start justify-center'>
                    <div id='' class='flex relative h-full   ' style={ 'width: '+(cards_tempo.length*40)+'px	' } >
                        {#each cards_tempo as card, i}
                            <div class='absolute transition-all ease-in-out duration-300 top-1/2 -translate-y-1/2  '   style={"left: "+(i*cards_tempo.length*40/(cards_tempo.length+3))+"px;"} >
                                <Back />
                            </div>	
                        {/each}
                    </div>
                </div>
            </div>
            <!--  CENTER  -->
            <div class='w-full h-full  flex items-center justify-center' >
                <div class='w-64 h-48 bg-green-500   rounded-lg flex ' bind:this={center_box}>
                    <Back />
                    {#if last_card.type === 'number'}
                        <Number_card number={last_card.value} bg={last_card.color}   />
                    {:else if last_card.type === 'change_direction' || last_card.type === 'block' || last_card.type === 'pick_two'}
                        <Arrow_block type={last_card.type} bg={last_card.color}   />
                    {:else if last_card.type === 'pick_four'}
                        <Fours type={last_card.type} bg={last_card.color}   />
                    {:else if last_card.type === 'change_color'}
                        <Colored_card type={last_card.type} bg={last_card.color} is_blank={last_card.is_blank}  />
                    {/if}
                    
                </div>
            </div>
            <!--  BOTTOM  -->
            <div class='w-full h-2/3  flex items-center justify-center     ' >
                <div class='w-full h-full   flex items-start justify-center'>
                    <div id='' class='flex relative h-full bg-green-600  ' style={ 'width: '+(cards_tempo.length*40)+'px	' } >
                        {#each cards_tempo as card, i}
                            <div id={i+''} class='absolute transition-all ease-in-out duration-300  top-1/2 -translate-y-1/2 cursor-pointer' style={"left: "+(i*cards_tempo.length*40/(cards_tempo.length+3))+"px;"} on:click={test} on:keydown={()=>{console.log('idk why')}} >
                                {#if card.type === 'number'}
                                    <Number_card number={card.value} bg={card.color}   />
                                {:else if card.type === 'change_direction' || card.type === 'block' || card.type === 'pick_two'}
                                    <Arrow_block type={card.type} bg={card.color}   />
                                {:else if card.type === 'pick_four'}
                                    <Fours type={card.type} bg={card.color}   />
                                {:else if card.type === 'change_color'}
                                    <Colored_card type={card.type} bg={card.color} is_blank={card.is_blank}  />
                                {/if}
                            </div>	
                        {/each}
                    </div>
                </div>
            </div>
        </div>
        <!--  RIGHT  -->
        <div class='w-1/4 h-full  flex items-center justify-center ' >
            <div class='w-2/3 h-2/3  flex items-center justify-center'>
                <div class='w-2/3 h-full  flex items-center justify-center '>
                    <div id='' class='flex relative bg-green-600 w-full '  style={ 'height: '+(cards_tempo.length*40)+'px	' } >
                        {#each cards_tempo as card, i}
                            <div class='absolute transition-all ease-in-out duration-300 rotate-90  left-1/2 -translate-x-1/2 '   style={"top: "+(i*cards_tempo.length*40/(cards_tempo.length+3))+"px;"} >
                                <Back />
                            </div>	
                        {/each}
                    </div>
                </div>
            </div>
        </div>
    </div>



<!--
    
    <div class='w-full  border h-64 '>
        <div id='' class='   flex relative' style={ 'width: '+(cards_tempo.length*40)+'px	' } >
            {#each cards_tempo as card, i}
            
            {#if i < cards_tempo.length/2 }
            {offset = ( cards_tempo.length/2 - i)}
            {:else }
            {offset = ( i -  cards_tempo.length/2 +1 )}			 
            {/if}
            
            
            <div class='absolute transition-all ease-in-out duration-300' on:mouseover={raise} on:focus={raise} on:mouseleave={reset}  style={"left: "+(i*cards_tempo.length*40/(cards_tempo.length+2))+"px;"} >
                    {#if card.type === 'number'}
                    <Number_card number={card.value} bg={card.color}   />
                    {:else if card.type === 'change_direction' || card.type === 'block' || card.type === 'pick_two'}
                    <Arrow_block type={card.type} bg={card.color}   />
                    {:else if card.type === 'pick_four'}
                    <Fours type={card.type} bg={card.color}   />
                    {:else if card.type === 'change_color'}
                    <Colored_card type={card.type} bg={card.color} is_blank={card.is_blank}  />
                    {/if}
                </div>	
                {/each}
                
            </div>
        </div>	
        
    -->


    
</div>


<style>

</style>