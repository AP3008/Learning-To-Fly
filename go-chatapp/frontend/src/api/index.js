var socket = new WebSocket("ws://localhost:8080/ws");
let connect = () => {
	console.log("Attempting to connect");	
	socket.onopen = () => {
		console.log("Successfully Connected");
	};
	socket.onmessage = msg => {
		console.log(msg);
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
	socket.sendMsg(msg);
};

export { connect, sendMsg };
