<script>
	import { onMount } from 'svelte';
	import './styles.css';





	import { io } from "socket.io-client";

	const socket = io("ws://localhost:5137");





	onMount(async () => {
		socket.emit("hello from client", 5, "6", { 7: Uint8Array.from([8]) });

		socket.on("hello from server", (...args) => {
			alert(args)
		});
    })

</script>

<div class="app">

	<main>
		<slot />
	</main>

	
</div>
