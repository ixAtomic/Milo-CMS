import { SetStateAction, useEffect, useState } from 'react'
import './App.css'
import Tabs from '@mui/material/Tabs'
import Tab from '@mui/material/Tab'


function App() {
  //const [count, setCount] = useState(0)
  const [collections, setCollections] = useState("");
  const [currentTab, setTab] = useState(0);

  useEffect(() => {
    getAllCollections().then((collections) => setCollections(collections));
  }, []);

  return (
    <>
      <Header currentTab={currentTab} setTab={setTab} />
      {collections}
    </>
  )
}

// interface TabProps {
//   currentTab: number,
//   setTab: React.Dispatch<React.SetStateAction<number>>
// }
type TabMethod = React.Dispatch<SetStateAction<number>>;

const Header = ({ currentTab, setTab }: { currentTab: number, setTab: TabMethod } ) => 
{
  const handleChange = (event: React.SyntheticEvent, newValue: number) => {
    setTab(newValue);
  };

  return (
    <>
      <Tabs value={currentTab} onChange={handleChange} aria-label="basic tabs example">
        <Tab label="Item One" />
        <Tab label="Item Two" />
        <Tab label="Item Three" />
      </Tabs>
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
