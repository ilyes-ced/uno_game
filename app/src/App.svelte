<script lang="ts">
    import Router from 'svelte-spa-router'
    import "./tailwind.css"

    import Header from './Header.svelte'
    import Home from './routes/Home_page.svelte'
    import Game from './routes/Game.svelte'
    import Not_found from './routes/404.svelte'

    const routes = {
        '/': Home,
        '/game': Game,
        '*': Not_found,
    }
    
    
    

</script>




<div class="w-full h-full flex flex-col bg-[#130e29] text-gray-400 ">
    <Header />
    <Router {routes}/>
</div>





<style>




</style>