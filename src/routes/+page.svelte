<script lang='ts'>

	//import { io } from 'socket.io-client'

	//const socket = io()
	//socket.on('eventFromServer', (message) => {
	//	alert(message)
	//})



</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>



<div class='bg-gray-700 h-screen w-full flex items-center justify-center ' >


	<div class='bg-gray-500 border-2 border-gray-900 rounded-xl p-10 flex flex-col space-y-4'>
		<button class='p-2 bg-gray-500 rounded-xl border border-gray-900 hover:bg-gray-600' >public game</button>
		<button class='p-2 bg-gray-500 rounded-xl border border-gray-900 hover:bg-gray-600' >private game</button>
	</div>


</div>


<style>

</style>