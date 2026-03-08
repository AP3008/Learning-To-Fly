var socket = new WebSocket("ws://localhost:8080/ws");
let connect = cb => {
	console.log("Attempting to connect");	
	socket.onopen = () => {
		console.log("Successfully Connected");
	};
	socket.onmessage = msg => {
		console.log(msg);
		cb(msg);
	};
	socket.onclose = event => {
		console.log("Socket closed connection: ", event);
	};
	socket.onerror = error => {
		console.log("Error occurred: ", error);
	};
};

let sendMsg = msg => {
	console.log("sending msg: ", msg);
	socket.send(msg);
};

export { connect, sendMsg };
