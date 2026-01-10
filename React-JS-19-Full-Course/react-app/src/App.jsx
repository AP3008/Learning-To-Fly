import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

const Test = ({ title }) => {
  return (
    <div>
      <p>{title}</p>
    </div>
  )
}
const App = () => {
  return (
    <div>
    <h2>Arrow Component</h2>
      <div className="container">
        <Test title="This"/>
        <Test title="is"/>
        <Test title="a"/>
        <Test title="Test!"/> 
      </div>

    </div>
  )
}

export default App
