<script lang='ts'>

    import Number_card1 from './routes/minimal/Number_card.svelte';
    import Number_card2 from './routes/deck1/Number_card.svelte';
    import Number_card3 from './routes/deck2/Number_card.svelte';


    let show_theme_modal = false

    const toggle_theme_modal = () => {
        show_theme_modal = !show_theme_modal
    }


</script>

<div class="h-12 bg-[rgba(0,0,0,0.4)] flex items-center justify-between px-6 ">




    <div class='flex '>
        <div class=' flex items-center px-4 py-1   '>
            <a class='w-full h-full hover:underline hover:text-[#e33050] ' href="#/">home page</a>
        </div>
        <div class=' flex items-center px-4 py-1   '>
            <a class='w-full h-full hover:underline hover:text-[#e33050] ' href="#/game">game</a>
        </div>
        <div class=' flex items-center px-4 py-1   '>
            <a class='w-full h-full hover:underline hover:text-[#e33050] ' href="#/ff">settings</a>
        </div>
    </div>

    
    <div>
        <div class='flex items-center space-x-2 cursor-pointer' on:click={toggle_theme_modal}>
            <p>
                cards theme 
            </p>
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class=" mt-1 bi bi-caret-down-fill" viewBox="0 0 16 16">
                <path d="M7.247 11.14 2.451 5.658C1.885 5.013 2.345 4 3.204 4h9.592a1 1 0 0 1 .753 1.659l-4.796 5.48a1 1 0 0 1-1.506 0z"/>
            </svg>
        </div>
    </div>


    {#if show_theme_modal}
        <div class=" top-12 right-6 rounded-lg border-4 border-[#100a29]  bg-[#28233f] absolute  grid grid-cols-2 gap-4 p-4">
            <div class='bg-[#130e29] rounded-lg p-6 border-4 border-[#100a29] cursor-pointer'>
                <Number_card1  number={5} bg={'blue'} />
            </div>
            <div class='bg-[#130e29] rounded-lg p-6 border-4 border-[#100a29] cursor-pointer'>
                <Number_card2  number={5} bg={'blue'} />
            </div>
            <div class='bg-[#130e29] rounded-lg p-6 border-4 border-[#100a29] cursor-pointer'>
                <Number_card3  number={5} bg={'blue'} />
            </div>
        </div>
    {/if}
</div>