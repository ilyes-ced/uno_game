import { writable } from 'svelte/store';

export const count = writable(0);


let time = new Date();
let hours
let minutes
let seconds
$: hours = time.getHours();
$: minutes = time.getMinutes();
$: seconds = time.getSeconds();


const interval = setInterval(() => {
    time = new Date();
}, 1000);



let socket = new WebSocket("ws://localhost:5000")



const interval2 = setInterval(() => {
    socket.send(time)
}, 1000);


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
