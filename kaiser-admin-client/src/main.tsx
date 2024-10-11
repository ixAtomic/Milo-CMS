import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';
import App from './App.tsx'
import theme from './theme.tsx';
import './assets/index.css'; // Import the CSS file
import { CssBaseline, ThemeProvider } from '@mui/material';
import { createBrowserRouter, LoaderFunction, RouterProvider } from 'react-router-dom'; 
import { Collections } from './pages/collections.tsx';
import { FieldPage } from './pages/field.tsx';
import { CreateField } from './pages/create-field.tsx'

// loaders.ts
import { LoaderFunctionArgs } from 'react-router-dom';

export const fieldPageLoader: LoaderFunction = ({ params }: LoaderFunctionArgs) => {
  const id = params.id ? parseInt(params.id, 10) : NaN;
  // Fetch additional data if needed
  return { id };
};

const router = createBrowserRouter([
  {
    path: '/',
    element: <App />,
    errorElement: <div>404 Not Found</div>, // maybe a 404 page
    children: [
      {
        path: '/collections',
        element: <Collections />,
      },
      {
        path: '/fields/:id',
        element: <FieldPage />,
        loader: fieldPageLoader,
      },
      {
        path: '/fields/create',
        element: <CreateField />,
      }
    ]
  }
]);

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <RouterProvider router={router} />
    </ThemeProvider>
  </StrictMode>,
)
