import { SetStateAction, useEffect, useState } from 'react'
import Tabs from '@mui/material/Tabs'
import Tab from '@mui/material/Tab'
import { AdminPage } from './enums';
import AppBar from '@mui/material/AppBar';
import { Box, Button, Container, IconButton, List, ListItem, ListItemButton, ListItemText, MenuItem, Toolbar, Typography } from '@mui/material';
import Menu from '@mui/material/Menu';
import MenuIcon from '@mui/icons-material/Menu';

const pages = ['Products', 'Pricing', 'Blog'];

function App() {
  //const [count, setCount] = useState(0)
  const [collections, setCollections] = useState("");

  const swapComponent = (event: React.MouseEvent<HTMLDivElement, MouseEvent>, page: AdminPage) => {
    console.log(event);
    console.log(page);
  }

  useEffect(() => {
    getAllCollections().then((collections) => setCollections(collections));
  }, []);
  console.log(collections)
  return (
    <Box sx={{ flexGrow: 1 }}>
      <AppBar position='static'>
        <Toolbar disableGutters>
          <List>
            <ListItem key={AdminPage.Collections} disablePadding>
              <ListItemButton onClick={(event) => swapComponent(event, AdminPage.Collections)} sx={{textAlign: 'center'}}>
                <ListItemText primary={AdminPage[AdminPage.Collections]}/>
              </ListItemButton>
            </ListItem>
          </List>
        </Toolbar>
      </AppBar>
      <Container>
        <Typography>Hello World</Typography>
      </Container>
    </Box>
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
