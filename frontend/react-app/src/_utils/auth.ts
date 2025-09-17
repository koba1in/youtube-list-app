import { Dispatch, SetStateAction } from "react";
import { Auth, CsrfToken } from "./type";
import { BASE_URL } from "../env";


export const login = ({ setCsrf, setAuth }: { setCsrf: Dispatch<SetStateAction<CsrfToken>>, setAuth: Dispatch<SetStateAction<Auth>> }) => {
    const width = 500;
    const height = 600;
    const left = window.screenX + (window.outerWidth - width) / 2;
    const top = window.screenY + (window.outerHeight - height) / 2;

    const authWindow = window.open(
        BASE_URL + "/auth/login",
        "OAuthLogin",
        `width=${width},height=${height},left=${left},top=${top}`
    );

    const listener = (event: MessageEvent) => {
        if (event.origin !== BASE_URL) return;
        setCsrf(event.data.csrf_token);
        setAuth("login");
        authWindow?.close();
        window.removeEventListener("message", listener);
    };
    window.addEventListener("message", listener);

};


export const logout = async ({ csrf, setAuth, setLoading }: { csrf: CsrfToken, setAuth: Dispatch<SetStateAction<Auth>>, setLoading: Dispatch<SetStateAction<boolean>> }) => {
    setLoading(true);
    const request = new URL(BASE_URL + "/auth/logout");
    const response = await fetch(
        request, {
        method: "POST",
        credentials: "include",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ "csrf_token": csrf }),
    }
    )
    setLoading(false);
    if (!response.ok) {
        throw new Error(`${response}`);
    }
    setAuth("logout");
}