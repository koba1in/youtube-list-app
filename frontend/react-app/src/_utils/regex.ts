import { Url } from "./type";

export const url_to_playlistid = ({ url }: Url): string => {
    const list = url.match(/[?&]list=([^&]+)/);
    const listid = list ? list[1] : "";
    return listid;
}

export const check_url = ({ url }: Url): boolean => {
    return /[?&]list=([^&]+)/.test(url);
}