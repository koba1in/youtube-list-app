
import { useEffect, useState } from 'react';
import { Box, Grid } from '@mui/material'
import { YoutubeForm } from './YoutubeForm';
import { Auth, YoutubeList } from '../_utils/type';
import YouTube, { YouTubeProps } from 'react-youtube';
import { VideoList } from './List';
import { css } from '@emotion/react';
import { Height } from '@mui/icons-material';
import './iframe.css';



export function Body({ auth }: { auth: Auth }) {
    const [snippets, setSnippets] = useState<YoutubeList | null>(null);
    const [loading, setLoading] = useState<boolean>(false);
    const [index, setIndex] = useState<number | null>(null);
    const [ready, setReady] = useState(false);


    const onReady = () => {
        const onReady: YouTubeProps["onReady"] = (e) => {
            const player = e.target;

            player.playVideo();
        };
    }

    return (
        <Box sx={{ height: "100%" }}>
            <Box sx={{ height: "60px", width: "100%", justifyContent: "center", alignItems: "center", padding: "10px", }} >
                <YoutubeForm setSnippets={setSnippets} loading={loading} setLoading={setLoading} setIndex={setIndex} auth={auth} />
            </Box>
            <Grid container direction="row" sx={{ height: "calc(100% - 60px)" }}>
                <Grid sx={{
                    width: "35%",
                    height: "100%",
                }}>
                    <VideoList youtube_list={snippets} setIndex={setIndex} />
                </Grid>
                <Grid sx={{ width: "65%", height: "100%", backgroundColor: index === null ? "primary" : "black" }}>
                    {index !== null && snippets !== null &&
                        <YouTube
                            videoId={snippets[index].resourceId.videoId}
                            className="iframeStyle"
                            opts={{ width: "100%", height: "100%", playerVars: { autoplay: 1, rel: 0, } }}
                            onEnd={() => setIndex(index + 1 === snippets.length ? 0 : index + 1)}
                            onReady={onReady}
                        />
                    }
                </Grid>
            </Grid >
        </Box >
    )
};