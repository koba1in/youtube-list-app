import { YoutubeList } from "./type";

export const getPlaylist = async (playlist_id: string): Promise<YoutubeList> => {
    console.log(playlist_id);
    const request = new URL("http://localhost:8080/data");
    request.searchParams.set("playlist_id", playlist_id);
    const response = await fetch(request, {
        method: "GET",
        credentials: "include",
    });
    if (!response.ok) {
        throw new Error(`${response}`);
        // throw new Error('Failed to fetch data');
    }
    const data: YoutubeList = await response.json();
    return data;
};
