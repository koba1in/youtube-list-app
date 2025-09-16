
import { useEffect, useState } from 'react';
import { Box, Grid } from '@mui/material'
import { YoutubeForm } from './YoutubeForm';
import { Auth, YoutubeList } from '../_utils/type';
import YouTube, { YouTubeProps } from 'react-youtube';
import { VideoList } from './List';


export function Body({ auth }: { auth: Auth }) {
    const [snippets, setSnippets] = useState<YoutubeList | null>(null);
    const [loading, setLoading] = useState<boolean>(false);
    const [index, setIndex] = useState<number | null>(null);
    const [ready, setReady] = useState(false);
    const [iframeheight, setIframeheight] = useState(0);
    const [iframewidth, setIframewidth] = useState(0);

    const updateIframeSize = () => {
        const height = window.innerHeight;
        const width = window.innerWidth;
        setIframeheight(height - 120);
        setIframewidth(width * 0.65);
    }

    useEffect(() => {
        updateIframeSize();
        window.addEventListener("resize", updateIframeSize);
        return () => window.removeEventListener("resize", updateIframeSize);
    }, []);

    const onReady = () => {
        const onReady: YouTubeProps["onReady"] = (e) => {
            const player = e.target;

            player.playVideo();
        };
    }

    return (
        <Grid container direction="column" sx={{ height: "100%", display: "flex" }} >
            <Grid sx={{ height: "60px", width: "100%", display: "flex", justifyContent: "center", alignItems: "center", padding: "10px", }} >
                <YoutubeForm setSnippets={setSnippets} loading={loading} setLoading={setLoading} setIndex={setIndex} auth={auth} />
            </Grid>
            <Grid container direction="row" sx={{ flexGrow: 1, display: "flex" }}>
                <Grid sx={{
                    width: "35%",
                    height: "calc(100vh - 110px)",
                }}>
                    <VideoList youtube_list={snippets} setIndex={setIndex} />
                </Grid>
                <Grid sx={{ flexGrow: 1, backgroundColor: "black" }}>
                    {index !== null && snippets !== null && true &&
                        <YouTube
                            videoId={snippets[index].resourceId.videoId}
                            opts={{ height: iframeheight, width: iframewidth, playerVars: { autoplay: 1, rel: 0, } }}
                            onEnd={() => setIndex(index + 1 === snippets.length ? 0 : index + 1)}
                            onReady={onReady}
                        />
                    }
                </Grid>
            </Grid>
        </Grid >
    )
};