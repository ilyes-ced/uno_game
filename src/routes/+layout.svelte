<script>
	import './styles.css';









	onMount(async () => {
        getSocket();
    })

</script>

<div class="app">

	<main>
		<slot />
	</main>

	
</div>
