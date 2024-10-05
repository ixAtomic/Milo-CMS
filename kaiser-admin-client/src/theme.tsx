import { createTheme } from '@mui/material/styles';
import { red } from '@mui/material/colors';
import { colors } from '@mui/material';

// A custom theme for this app
const theme = createTheme({
  palette: {
    primary: {
      main: colors.grey[900],
    },
    secondary: {
      main: colors.grey[400],
    },
    error: {
      main: red.A400,
    },
    background: {
        default: colors.grey[700]
    }
  },
});

export default theme;