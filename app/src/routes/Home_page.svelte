<script lang='ts'>

    let selected_game = null
    let game_name
    let game_password
    let created_game_is_private = false
    let created_game_name
    let created_game_password


    let show_create_game_modal = false
    let join_public_game_modal = false
    let join_private_game_modal = false


    const create = () => {
        show_create_game_modal = true
    }
    const join_public = () => {
        join_public_game_modal = true
    }
    const join_private = () => {
        join_private_game_modal = true
    }
    const hide_modals = (e) => {
        if(e.target === e.currentTarget)
        show_create_game_modal = false
        join_public_game_modal = false 
    }



    const join_public_game = (e) => {}
    const join_private_game = (e) => {}
    const create_game = (e) => {}




    import { onMount } from 'svelte';
    //import { onMount } from 'svelte';



    onMount(() => {

    })

</script>

<div class="h-full w-fit bg-[#100a29] mx-auto max-w-[1200px] flex flex-col">
    <div>
        <img src="../images/purple_uno_img.jpg" alt="dd">
    </div>

    <div class='w-full  flex space-x-24 px-24 grow items-center justify-center'>
        <div class='rotate-[10deg] -skew-y-[10deg] w-1/3 p-8 bg-[#28233f] flex items-center justify-center flex-col space-y-4  '>
            <p>
                join pblic game
            </p>
            <button on:click={join_public} class='text-white py-2 px-4 bg-[#e33050] hover:bg-[#100a29]'>
                do
            </button>
        </div>
        <div class='rotate-[10deg] -skew-y-[10deg] w-1/3 p-8 bg-[#28233f] flex items-center justify-center flex-col space-y-4  '>
            <p>
                join private game
            </p>
            <button on:click={join_private} class='text-white py-2 px-4 bg-[#e33050] hover:bg-[#100a29]'>
                do
            </button>
        </div>
        <div class='rotate-[10deg] -skew-y-[10deg] w-1/3 p-8 bg-[#28233f] flex items-center justify-center flex-col  space-y-4 '>
            <p>
                create a game
            </p>
            <button on:click={create} class='text-white py-2 px-4 bg-[#e33050] hover:bg-[#100a29]'>
                do
            </button>
        </div>


    </div>


    {#if join_public_game_modal}
        <div class='absolute bg-[rgba(0,0,0,0.5)] h-full w-full flex items-center justify-center top-0 left-0' on:click={hide_modals} on:keyup={hide_modals} >
            <div class='p-4 bg-[#130e29] rounded-lg h-1/2 w-1/4 flex flex-col'>
                <div class='flex justify-between py-2'>
                    <h1>title</h1>
                    <div>x</div>
                </div>
                <div class='grow overflow-y-scroll divide-y-[1px] border'>
                    {#each Array(100) as _, i}
                        <div class='px-4 py-2  w-full  flex justify-between hover:bg-red-600 cursor-pointer '>
                            <div>name of game</div>
                            <div>2/3</div>
                        </div>
                    {/each}
                </div>
                <div class='flex justify-between py-2'>
                    <button class='bg-red-600 px-4 py-2 rounded-lg border'>join</button>
                </div>
            </div>
        </div>
    {/if}

    {#if join_private_game_modal}
    {game_name}
    {game_password}
        <div class='absolute bg-[rgba(0,0,0,0.5)] h-full w-full flex items-center justify-center top-0 left-0' on:click={hide_modals} on:keyup={hide_modals} >
            <div class='p-4 bg-[#130e29] rounded-lg flex flex-col space-y-4'>
                <div class='flex justify-between py-2'>
                    <h1>title</h1>
                    <div>x</div>
                </div>
                <div class='flex flex-col space-y-4'>
                    <input bind:value={game_name} class='bg-[#100a29] border-2 px-4 py-2 rounded-lg ' type="text" name='name' placeholder='game name'>
                    <input bind:value={game_password} class='bg-[#100a29] border-2 px-4 py-2 rounded-lg ' type="text" name='password' placeholder='game password'>
                    <div class='w-full flex '>
                        <button class='bg-red-600 border-2 px-4 py-2 rounded-lg'>join</button>
                    </div>
                </div>
            </div>
        </div>
    {/if}

    {#if show_create_game_modal}
        <div class='absolute bg-[rgba(0,0,0,0.5)] h-full w-full flex items-center justify-center top-0 left-0' on:click={hide_modals} on:keyup={hide_modals} >
            <div class='p-4 bg-[#130e29] rounded-lg'>
                <div class='p-4 bg-[#130e29] rounded-lg flex flex-col space-y-4'>
                    <div class='flex justify-between py-2'>
                        <h1>title</h1>
                        <div>x</div>
                    </div>
                    <div class='flex flex-col space-y-4'>
                        <input class='bg-[#100a29] border-2 px-4 py-2 rounded-lg ' type="text" name='name' placeholder='game name'>
                        <input class='' type="checkbox" name="private" id="" bind:checked={created_game_is_private}>private
                        <input class='bg-[#100a29] border-2 px-4 py-2 rounded-lg ' type="text" name='password' placeholder='game password'>
                        <div class='w-full flex '>
                            <button class='bg-red-600 border-2 px-4 py-2 rounded-lg'>join</button>
                            <div class="flex items-center justify-center">
                                <div class="inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-current border-r-transparent align-[-0.125em] motion-reduce:animate-[spin_1.5s_linear_infinite]">
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
               
            </div>
        </div>
    {/if}


</div>

