<script lang="ts">
  import Router from 'svelte-spa-router'


    import Home from './routes/Home_page.svelte'
    import Game from './routes/Game.svelte'
    import Not_found from './routes/404.svelte'

    const routes = {
        '/': Home,
        '/game': Game,
        '*': Not_found,
    }
    
    

</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <p>
    Click on the Tauri, Vite, and Svelte logos to learn more.
  </p>


  <div class="row">
    hello

        <Router {routes}/>



        <a href="#/">fefefef</a>
        <a href="#/game">fefefef</a>
        <a href="#/fze">00</a>
    
    </div>


</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>