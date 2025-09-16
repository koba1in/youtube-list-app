export type Url = {
    url: string;
};

export type GetQuery = {
    playlist_id: string;
};

export type YoutubeList = Array<Snippet>;

export type Snippet = {
    title: string;
    channelTitle: string;
    resourceId: ResourceId;
};

export type ResourceId = {
    videoId: string;
};

export type CsrfToken = string | null;

export type Mode = "light" | "dark";

export type Auth = "login" | "logout";