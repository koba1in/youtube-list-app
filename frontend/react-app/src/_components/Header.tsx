
import { Dispatch, SetStateAction, useState } from 'react';
import { AppBar, Box, IconButton, Toolbar } from '@mui/material'
import { DarkMode, Google, LightMode, YouTube, Logout } from '@mui/icons-material'

import { login, logout } from '../_utils/auth';
import { Auth, CsrfToken, Mode } from '../_utils/type';

const styles = {
    largeIcon: {
        width: 100,
        height: 100,
    },
};

export function Header({ mode, setMode, auth, setAuth, csrf, setCsrf }: { mode: Mode, setMode: Dispatch<SetStateAction<Mode>>, auth: Auth, setAuth: Dispatch<SetStateAction<Auth>>, csrf: CsrfToken, setCsrf: Dispatch<SetStateAction<CsrfToken>> }) {
    const toggleMode = () => setMode(prev =>
        prev === "light" ? "dark" : "light"
    );
    const [loading, setLoading] = useState<boolean>(false)
    return (
        <AppBar position="static" sx={{ height: "50px" }}>
            <Toolbar
                sx={{ minHeight: "50px", display: 'flex', justifyContent: 'space-between', marginTop: "-7px" }}>
                <IconButton
                    href="https://www.youtube.com"
                    color="secondary"
                    disableRipple={true}
                >
                    <YouTube
                        sx={{ fontSize: "40px" }}
                    />
                </IconButton>
                <Box>
                    <IconButton onClick={toggleMode} >
                        {mode === "light" &&
                            <DarkMode
                                sx={{ fontSize: "20px" }}
                            />}
                        {mode === "dark" &&
                            <LightMode
                                sx={{ fontSize: "20px" }}
                            />}
                    </IconButton>
                    <IconButton disabled={loading} sx={{ marginLeft: "10px" }}>
                        {auth === "logout" &&
                            <Google sx={{ fontSize: "25px" }} onClick={() => login({ setCsrf: setCsrf, setAuth: setAuth })} />
                        }
                        {auth === "login" && csrf !== null &&
                            <Logout sx={{ fontSize: "25px" }} onClick={() => logout({ csrf: csrf, setAuth: setAuth, setLoading: setLoading })} />
                        }
                    </IconButton>
                </Box>

            </Toolbar>
        </AppBar>
    )
};