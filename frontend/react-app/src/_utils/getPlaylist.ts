import { YoutubeList } from "./type";
import { BASE_URL } from "../env";

export const getPlaylist = async (playlist_id: string): Promise<YoutubeList> => {
    console.log(playlist_id);
    const request = new URL(BASE_URL + "/data");
    request.searchParams.set("playlist_id", playlist_id);
    const response = await fetch(request, {
        method: "GET",
        credentials: "include",
    });
    if (!response.ok) {
        throw new Error(`${response}`);
    }
    const data: YoutubeList = await response.json();
    return data;
};
