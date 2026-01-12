import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

const Test = ({ title }) => {
  const [hasLiked, setHasLiked] = useState(flase);

  return (
    <div className="card">
      <h2>{title}</h2>

      <button onClick={() => setHasLiked(!hasLiked)}>
        {hasLiked ? 'Liked':'Like'}
      </button>

    </div>
  )
}
const App = () => {
  return (
        <div className="container">
          <Test title="This"/>
          <Test title="is"/>
          <Test title="a"/>
          <Test title="Test!"/> 
        </div>
  )
}

export default App
