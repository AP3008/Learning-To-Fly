import { Component } from 'react';
import './App.css';
import { connect, sendMsg } from "./api"

import Header from "./components/Header/Header";
import ChatHistory from './components/ChatHistory';
import ChatInput from './components/ChatInput';

class App extends Component {
  constructor(props) {
    super(props);
	this.state = {
		chatHistory: []
	}
  }

	componentDidMount(){
		connect((msg) => {
			console.log("New Messages")
			this.setState(prevState => ({
				chatHistory: [...this.state.chatHistory, msg]	
			}))
			console.log(this.state);
		});
	}

	send(){
		console.log("hello");
		sendMsg("hello");
	}

	render(){
		return (
		  <div className="App">
			<Header />
			<ChatHistory chatHistory={this.state.chatHistory} />
			<ChatInput send={this.send} />
			<button onClick={this.send}>Hit</button>
		  </div>
		);
	}
}

export default App;
