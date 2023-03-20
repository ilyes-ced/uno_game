<script lang='ts'>

    let show_theme_modal = false

    const toggle_theme_modal = () => {
        show_theme_modal = !show_theme_modal
    }


</script>

<div class="h-[50px] bg-[rgba(0,0,0,0.4)] flex items-center justify-between px-6 ">




    <div class='flex '>
        <div class=' flex items-center px-4 py-1  bg-white border'>
            <a class='w-full h-full bg-red-600' href="#/">1222</a>
        </div>
        <div class=' flex items-center px-4 py-1  bg-white border'>
            <a class='w-full h-full bg-red-600' href="#/game">1222</a>
        </div>
        <div class=' flex items-center px-4 py-1  bg-white border'>
            <a class='w-full h-full bg-red-600' href="#/ff">1222</a>
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
        <div class="absolute top-12 right-12 rounded-lg border-4 border-[#100a29] bg-red-600 h-12 w-12"></div>
    {/if}
</div>