import React, { useState } from 'react';
import './App.css';
import { Header } from './_components/Header';
import { Body } from './_components/Body';
import { ThemeProvider } from '@emotion/react';
import { createTheme, CssBaseline, Grid, useMediaQuery } from '@mui/material';
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
      <Grid container direction="column" sx={{ height: "100vh", width: "100vw", display: "flex" }}>
        <Grid sx={{ height: "50px" }}>
          <Header mode={mode} setMode={setMode} auth={auth} setAuth={setAuth} csrf={csrf} setCsrf={setCsrf} />
        </Grid>
        <Grid sx={{ flexGrow: 1, }}>
          <Body auth={auth} />
        </Grid>
      </Grid>
    </ThemeProvider>
  );
}


export default App;
