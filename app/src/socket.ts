let socket = new WebSocket("ws://localhost:8080/ws/get_games")




socket.onmessage = (ev) => {
  console.log('Received: ' + ev.data, 'message')
}

socket.onopen = () => {
  console.log('Connected')
}

socket.onmessage = (ev) => {
  console.log('Received: ' + ev.data, 'message')
}

socket.onclose = () => {
  console.log('Disconnected')
}



export default socket