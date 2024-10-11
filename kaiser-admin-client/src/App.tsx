import { AdminPage } from './types/enums';
import AppBar from '@mui/material/AppBar';
import { Box, List, ListItem, ListItemButton, ListItemText, Toolbar } from '@mui/material';
import { Outlet } from 'react-router-dom';
import { useNavigate } from 'react-router-dom';

function App() {
  const navigate = useNavigate();
  const swapComponent = (page: AdminPage) => {
    switch(page) {
      case AdminPage.Collections:
        navigate('/collections');
        break;
      default:
        break;
    }
  }

  return (
    <Box sx={{display: 'flex', flexDirection: 'column', height: '100%'}}>
      <AppBar position='static' sx={{ mb: 2 }}>
        <Toolbar disableGutters>
          <List>
            <ListItem key={AdminPage.Collections} disablePadding>
              <ListItemButton onClick={() => swapComponent(AdminPage.Collections)} sx={{textAlign: 'center'}}>
                <ListItemText primary={AdminPage[AdminPage.Collections]} />
              </ListItemButton>
            </ListItem>
          </List>
        </Toolbar>
      </AppBar>
      <Outlet />
    </Box>
  )
}

export default App
