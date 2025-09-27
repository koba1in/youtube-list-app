import React, { useState } from 'react';
import './App.css';
import { Header } from './_components/Header';
import { Body } from './_components/Body';
import { ThemeProvider } from '@emotion/react';
import { createTheme, CssBaseline, Grid, Box, useMediaQuery } from '@mui/material';
import { Auth, CsrfToken, Mode } from './_utils/type';
import { Height } from '@mui/icons-material';

function App() {
  const device_mode = useMediaQuery("(prefers-color-scheme: dark)") ? "dark" : "light";
  const [mode, setMode] = useState<Mode>(device_mode);
  const theme = createTheme({
    palette: {
      mode: mode,
    },
  });
  const [auth, setAuth] = useState<Auth>("logout");
  const [csrf, setCsrf] = useState<CsrfToken>(null);
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Box sx={{ height: "100vh", maxHeight: "100vh" }}>
        <Box sx={{ height: "50px" }}>
          <Header mode={mode} setMode={setMode} auth={auth} setAuth={setAuth} csrf={csrf} setCsrf={setCsrf} />
        </Box>
        <Box sx={{ height: "calc(100% - 50px)" }}>
          <Body auth={auth} />
        </Box>
      </Box>
    </ThemeProvider >
  );
}


export default App;
