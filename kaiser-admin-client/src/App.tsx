import { useEffect, useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

function App() {
  const [count, setCount] = useState(0)
  const [collections, setCollections] = useState("");

  useEffect(() => {
    getAllCollections().then((collections) => setCollections(collections));
  }, []);

  return (
    <>
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
        {collections ? (<div>{collections}</div>) : (<div>Loading...</div>)}
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

const getAllCollections = async (): Promise<string> => {
  try{
    const response = await fetch(`http://localhost:8000/collections`, {
      method: 'GET'
    });
  
    if(response.ok)
    {
      return JSON.stringify(await response.json());
    }
    else {
      return "Bad Response";
    }
  }
  catch{
    return "Error Fetching Collections";
  }

}

export default App
