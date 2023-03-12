import { Server } from "socket.io";

const io = new Server(5137);

io.on("connection", (socket) => {
    socket.emit("hello from server", 1, "2", { 3: Buffer.from([4]) });

    socket.on("hello from client", (...args) => {
        console.log(args)
    });
});